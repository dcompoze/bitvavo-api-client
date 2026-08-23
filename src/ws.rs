//! WebSocket API client.
//!
//! The client pushes market data and account events into a channel.
//! Request-response calls (for example order placement) are covered by the
//! REST client in [`crate::rest`].

use crate::ClientConfig;
use crate::auth;
use crate::error::{Error, Result};
use crate::types::*;
use futures::{SinkExt, StreamExt};
use serde_json::json;
use tokio::sync::mpsc;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;

/// Event received from the WebSocket connection.
#[derive(Debug, Clone)]
pub enum WsEvent {
    /// Authentication succeeded.
    Authenticated,
    /// Subscription confirmation with the active subscriptions.
    Subscribed(serde_json::Value),
    /// Unsubscription confirmation with the remaining subscriptions.
    Unsubscribed(serde_json::Value),
    /// Update on the `ticker` channel.
    Ticker(TickerEvent),
    /// Update on the `ticker24h` channel.
    Ticker24h(Vec<Ticker24h>),
    /// Update on the `candles` channel.
    Candle(CandleEvent),
    /// Update on the `trades` channel.
    Trade(TradeEvent),
    /// Order book delta on the `book` channel.
    Book(BookEvent),
    /// Order update on the `account` channel.
    Order(OrderEvent),
    /// Fill on the `account` channel.
    Fill(FillEvent),
    /// Response to a request sent over the socket, for example `getBook`.
    Response {
        action: String,
        response: serde_json::Value,
    },
    /// Error message from the server.
    Error { code: i64, message: String },
    /// The connection was closed and no further events will arrive.
    Closed,
    /// A message that did not match any known shape.
    Unknown(serde_json::Value),
}

enum Command {
    Send(String),
    Pong(Vec<u8>),
    Close,
}

/// Handle to an open WebSocket connection.
///
/// Dropping the handle closes the connection.
/// The connection does not reconnect on its own.
/// Call [`WsClient::connect`] again and resubscribe when [`WsEvent::Closed`]
/// arrives.
pub struct WsClient {
    cmd_tx: mpsc::UnboundedSender<Command>,
    config: ClientConfig,
}

impl WsClient {
    /// Opens a connection and returns the client handle and the event stream.
    pub async fn connect(config: ClientConfig) -> Result<(Self, mpsc::UnboundedReceiver<WsEvent>)> {
        let (stream, _) = connect_async(&config.ws_url).await?;
        let (mut write, mut read) = stream.split();
        let (cmd_tx, mut cmd_rx) = mpsc::unbounded_channel::<Command>();
        let (event_tx, event_rx) = mpsc::unbounded_channel::<WsEvent>();

        tokio::spawn(async move {
            while let Some(cmd) = cmd_rx.recv().await {
                let result = match cmd {
                    Command::Send(text) => write.send(Message::Text(text.into())).await,
                    Command::Pong(payload) => write.send(Message::Pong(payload.into())).await,
                    Command::Close => {
                        let _ = write.send(Message::Close(None)).await;
                        break;
                    }
                };
                if result.is_err() {
                    break;
                }
            }
        });

        let pong_tx = cmd_tx.clone();
        tokio::spawn(async move {
            while let Some(message) = read.next().await {
                match message {
                    Ok(Message::Text(text)) => {
                        let event = parse_event(&text);
                        if event_tx.send(event).is_err() {
                            break;
                        }
                    }
                    Ok(Message::Ping(payload)) => {
                        let _ = pong_tx.send(Command::Pong(payload.to_vec()));
                    }
                    Ok(Message::Close(_)) | Err(_) => break,
                    Ok(_) => {}
                }
            }
            let _ = event_tx.send(WsEvent::Closed);
        });

        Ok((Self { cmd_tx, config }, event_rx))
    }

    /// Opens a connection with the default configuration for public channels.
    pub async fn connect_public() -> Result<(Self, mpsc::UnboundedReceiver<WsEvent>)> {
        Self::connect(ClientConfig::new()).await
    }

    fn send_json(&self, value: serde_json::Value) -> Result<()> {
        self.cmd_tx
            .send(Command::Send(value.to_string()))
            .map_err(|_| Error::ConnectionClosed)
    }

    /// Authenticates the connection.
    /// Required before subscribing to the `account` channel.
    /// The server confirms with [`WsEvent::Authenticated`].
    pub fn authenticate(&self) -> Result<()> {
        let (key, secret) = self.config.credentials()?;
        let timestamp = auth::timestamp_ms();
        let signature = auth::sign(secret, timestamp, "GET", "/websocket", "");
        self.send_json(json!({
            "action": "authenticate",
            "key": key,
            "signature": signature,
            "timestamp": timestamp,
            "window": self.config.access_window_ms.to_string(),
        }))
    }

    fn subscription(&self, action: &str, channel: serde_json::Value) -> Result<()> {
        self.send_json(json!({ "action": action, "channels": [channel] }))
    }

    /// Subscribes to the `ticker` channel for the given markets.
    pub fn subscribe_ticker(&self, markets: &[&str]) -> Result<()> {
        self.subscription("subscribe", json!({ "name": "ticker", "markets": markets }))
    }

    /// Subscribes to the `ticker24h` channel for the given markets.
    pub fn subscribe_ticker_24h(&self, markets: &[&str]) -> Result<()> {
        self.subscription(
            "subscribe",
            json!({ "name": "ticker24h", "markets": markets }),
        )
    }

    /// Subscribes to the `trades` channel for the given markets.
    pub fn subscribe_trades(&self, markets: &[&str]) -> Result<()> {
        self.subscription("subscribe", json!({ "name": "trades", "markets": markets }))
    }

    /// Subscribes to the `candles` channel for the given markets and interval.
    pub fn subscribe_candles(&self, markets: &[&str], interval: &str) -> Result<()> {
        self.subscription(
            "subscribe",
            json!({ "name": "candles", "interval": [interval], "markets": markets }),
        )
    }

    /// Subscribes to the `book` channel for the given markets.
    pub fn subscribe_book(&self, markets: &[&str]) -> Result<()> {
        self.subscription("subscribe", json!({ "name": "book", "markets": markets }))
    }

    /// Subscribes to the `account` channel for the given markets.
    /// Call [`WsClient::authenticate`] first and wait for
    /// [`WsEvent::Authenticated`].
    pub fn subscribe_account(&self, markets: &[&str]) -> Result<()> {
        self.subscription(
            "subscribe",
            json!({ "name": "account", "markets": markets }),
        )
    }

    /// Unsubscribes from a channel for the given markets.
    pub fn unsubscribe(&self, channel: &str, markets: &[&str]) -> Result<()> {
        self.subscription(
            "unsubscribe",
            json!({ "name": channel, "markets": markets }),
        )
    }

    /// Requests an order book snapshot over the socket.
    /// The server answers with [`WsEvent::Response`] where the action is `getBook`.
    pub fn get_book(&self, market: &str) -> Result<()> {
        self.send_json(json!({ "action": "getBook", "market": market }))
    }

    /// Closes the connection.
    pub fn close(&self) {
        let _ = self.cmd_tx.send(Command::Close);
    }
}

/// Parses a raw text message into a typed event.
fn parse_event(text: &str) -> WsEvent {
    let value: serde_json::Value = match serde_json::from_str(text) {
        Ok(value) => value,
        Err(err) => {
            return WsEvent::Error {
                code: -1,
                message: format!("invalid json: {err}"),
            };
        }
    };

    if let Some(code) = value.get("errorCode").and_then(|c| c.as_i64()) {
        let message = value
            .get("error")
            .and_then(|e| e.as_str())
            .unwrap_or_default()
            .to_string();
        return WsEvent::Error { code, message };
    }

    if let Some(event) = value.get("event").and_then(|e| e.as_str()) {
        let typed = match event {
            "authenticate" => Some(WsEvent::Authenticated),
            "subscribed" => Some(WsEvent::Subscribed(
                value.get("subscriptions").cloned().unwrap_or_default(),
            )),
            "unsubscribed" => Some(WsEvent::Unsubscribed(
                value.get("subscriptions").cloned().unwrap_or_default(),
            )),
            "ticker" => serde_json::from_value(value.clone())
                .ok()
                .map(WsEvent::Ticker),
            "ticker24h" => value
                .get("data")
                .cloned()
                .and_then(|d| serde_json::from_value(d).ok())
                .map(WsEvent::Ticker24h),
            "candle" => serde_json::from_value(value.clone())
                .ok()
                .map(WsEvent::Candle),
            "trade" => serde_json::from_value(value.clone())
                .ok()
                .map(WsEvent::Trade),
            "book" => serde_json::from_value(value.clone())
                .ok()
                .map(WsEvent::Book),
            "order" => serde_json::from_value(value.clone())
                .ok()
                .map(WsEvent::Order),
            "fill" => serde_json::from_value(value.clone())
                .ok()
                .map(WsEvent::Fill),
            _ => None,
        };
        return typed.unwrap_or(WsEvent::Unknown(value));
    }

    if let Some(action) = value.get("action").and_then(|a| a.as_str()) {
        return WsEvent::Response {
            action: action.to_string(),
            response: value.get("response").cloned().unwrap_or_default(),
        };
    }

    WsEvent::Unknown(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ticker_event() {
        let text = r#"{"event":"ticker","market":"BTC-EUR","bestBid":"9156.8","bestAsk":"9157.9","lastPrice":"9157.3"}"#;
        match parse_event(text) {
            WsEvent::Ticker(ticker) => {
                assert_eq!(ticker.market, "BTC-EUR");
                assert_eq!(ticker.best_bid.as_deref(), Some("9156.8"));
            }
            other => panic!("unexpected event: {other:?}"),
        }
    }

    #[test]
    fn parse_error_event() {
        let text = r#"{"errorCode":105,"error":"rate limit exceeded"}"#;
        match parse_event(text) {
            WsEvent::Error { code, message } => {
                assert_eq!(code, 105);
                assert_eq!(message, "rate limit exceeded");
            }
            other => panic!("unexpected event: {other:?}"),
        }
    }

    #[test]
    fn parse_book_response() {
        let text =
            r#"{"action":"getBook","response":{"market":"BTC-EUR","nonce":1,"bids":[],"asks":[]}}"#;
        match parse_event(text) {
            WsEvent::Response { action, response } => {
                assert_eq!(action, "getBook");
                assert_eq!(response["market"], "BTC-EUR");
            }
            other => panic!("unexpected event: {other:?}"),
        }
    }

    #[test]
    fn parse_candle_event() {
        let text = r#"{"event":"candle","market":"BTC-EUR","interval":"1h","candle":[[1538784000000,"4999","5012","4999","5012","0.45"]]}"#;
        match parse_event(text) {
            WsEvent::Candle(candle) => {
                assert_eq!(candle.market, "BTC-EUR");
                assert_eq!(candle.candle.len(), 1);
                assert_eq!(candle.candle[0].close, "5012");
            }
            other => panic!("unexpected event: {other:?}"),
        }
    }
}
