//! Data types shared by the REST and WebSocket clients.
//!
//! Monetary amounts and prices are kept as strings to preserve the exact
//! decimal representation returned by the exchange.

use serde::de::{self, Deserializer, SeqAccess, Visitor};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Order side.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Side {
    Buy,
    Sell,
}

/// Order type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OrderType {
    Limit,
    Market,
    StopLoss,
    StopLossLimit,
    TakeProfit,
    TakeProfitLimit,
}

/// Response of `GET /time`.
#[derive(Debug, Clone, Deserialize)]
pub struct ServerTime {
    pub time: u64,
}

/// A tradable market.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Market {
    pub market: String,
    pub status: String,
    pub base: String,
    pub quote: String,
    #[serde(default)]
    pub price_precision: Option<u32>,
    #[serde(default)]
    pub min_order_in_base_asset: Option<String>,
    #[serde(default)]
    pub min_order_in_quote_asset: Option<String>,
    #[serde(default)]
    pub max_order_in_base_asset: Option<String>,
    #[serde(default)]
    pub max_order_in_quote_asset: Option<String>,
    #[serde(default)]
    pub order_types: Vec<String>,
}

/// A supported asset.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    pub symbol: String,
    pub name: String,
    #[serde(default)]
    pub decimals: Option<u32>,
    #[serde(default)]
    pub deposit_fee: Option<String>,
    #[serde(default)]
    pub deposit_confirmations: Option<u32>,
    #[serde(default)]
    pub deposit_status: Option<String>,
    #[serde(default)]
    pub withdrawal_fee: Option<String>,
    #[serde(default)]
    pub withdrawal_min_amount: Option<String>,
    #[serde(default)]
    pub withdrawal_status: Option<String>,
    #[serde(default)]
    pub networks: Vec<String>,
    #[serde(default)]
    pub message: Option<String>,
}

/// A single price level as `[price, size]`.
pub type PriceLevel = [String; 2];

/// Order book snapshot.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderBook {
    pub market: String,
    pub nonce: u64,
    pub bids: Vec<PriceLevel>,
    pub asks: Vec<PriceLevel>,
}

/// A public trade.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicTrade {
    pub id: String,
    pub timestamp: u64,
    pub amount: String,
    pub price: String,
    pub side: Side,
}

/// A single OHLCV candle.
/// The API returns candles as arrays of the form
/// `[timestamp, open, high, low, close, volume]`.
#[derive(Debug, Clone)]
pub struct Candle {
    pub timestamp: u64,
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
    pub volume: String,
}

impl<'de> Deserialize<'de> for Candle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CandleVisitor;

        impl<'de> Visitor<'de> for CandleVisitor {
            type Value = Candle;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a candle array [timestamp, open, high, low, close, volume]")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Candle, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let timestamp = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let open = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let high = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(2, &self))?;
                let low = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(3, &self))?;
                let close = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(4, &self))?;
                let volume = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(5, &self))?;
                Ok(Candle {
                    timestamp,
                    open,
                    high,
                    low,
                    close,
                    volume,
                })
            }
        }

        deserializer.deserialize_seq(CandleVisitor)
    }
}

/// Latest trade price for a market.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TickerPrice {
    pub market: String,
    #[serde(default)]
    pub price: Option<String>,
}

/// Best bid and ask for a market.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TickerBook {
    pub market: String,
    #[serde(default)]
    pub bid: Option<String>,
    #[serde(default)]
    pub ask: Option<String>,
    #[serde(default)]
    pub bid_size: Option<String>,
    #[serde(default)]
    pub ask_size: Option<String>,
}

/// 24 hour ticker statistics for a market.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ticker24h {
    pub market: String,
    #[serde(default)]
    pub open: Option<String>,
    #[serde(default)]
    pub high: Option<String>,
    #[serde(default)]
    pub low: Option<String>,
    #[serde(default)]
    pub last: Option<String>,
    #[serde(default)]
    pub volume: Option<String>,
    #[serde(default)]
    pub volume_quote: Option<String>,
    #[serde(default)]
    pub bid: Option<String>,
    #[serde(default)]
    pub bid_size: Option<String>,
    #[serde(default)]
    pub ask: Option<String>,
    #[serde(default)]
    pub ask_size: Option<String>,
    #[serde(default)]
    pub timestamp: Option<u64>,
    #[serde(default)]
    pub start_timestamp: Option<u64>,
    #[serde(default)]
    pub open_timestamp: Option<u64>,
    #[serde(default)]
    pub close_timestamp: Option<u64>,
}

/// An order as returned by the trading endpoints.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub order_id: String,
    pub market: String,
    #[serde(default)]
    pub client_order_id: Option<String>,
    #[serde(default)]
    pub created: Option<u64>,
    #[serde(default)]
    pub updated: Option<u64>,
    pub status: String,
    pub side: Side,
    pub order_type: OrderType,
    #[serde(default)]
    pub amount: Option<String>,
    #[serde(default)]
    pub amount_remaining: Option<String>,
    #[serde(default)]
    pub amount_quote: Option<String>,
    #[serde(default)]
    pub amount_quote_remaining: Option<String>,
    #[serde(default)]
    pub price: Option<String>,
    #[serde(default)]
    pub on_hold: Option<String>,
    #[serde(default)]
    pub on_hold_currency: Option<String>,
    #[serde(default)]
    pub filled_amount: Option<String>,
    #[serde(default)]
    pub filled_amount_quote: Option<String>,
    #[serde(default)]
    pub fee_paid: Option<String>,
    #[serde(default)]
    pub fee_currency: Option<String>,
    #[serde(default)]
    pub fills: Vec<OrderFill>,
    #[serde(default)]
    pub trigger_price: Option<String>,
    #[serde(default)]
    pub trigger_amount: Option<String>,
    #[serde(default)]
    pub trigger_type: Option<String>,
    #[serde(default)]
    pub trigger_reference: Option<String>,
    #[serde(default)]
    pub time_in_force: Option<String>,
    #[serde(default)]
    pub post_only: Option<bool>,
    #[serde(default)]
    pub self_trade_prevention: Option<String>,
    #[serde(default)]
    pub visible: Option<bool>,
    #[serde(default)]
    pub disable_market_protection: Option<bool>,
}

/// A fill embedded in an order response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderFill {
    pub id: String,
    pub timestamp: u64,
    pub amount: String,
    pub price: String,
    #[serde(default)]
    pub taker: Option<bool>,
    #[serde(default)]
    pub fee: Option<String>,
    #[serde(default)]
    pub fee_currency: Option<String>,
    #[serde(default)]
    pub settled: Option<bool>,
}

/// Response of a cancel order request.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CanceledOrder {
    pub order_id: String,
    #[serde(default)]
    pub client_order_id: Option<String>,
}

/// A private trade (an execution of one of your orders).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    pub id: String,
    pub order_id: String,
    #[serde(default)]
    pub client_order_id: Option<String>,
    pub timestamp: u64,
    pub market: String,
    pub side: Side,
    pub amount: String,
    pub price: String,
    #[serde(default)]
    pub taker: Option<bool>,
    #[serde(default)]
    pub fee: Option<String>,
    #[serde(default)]
    pub fee_currency: Option<String>,
    #[serde(default)]
    pub settled: Option<bool>,
}

/// Fee rates that apply to the account.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeeRates {
    #[serde(default)]
    pub tier: Option<serde_json::Value>,
    #[serde(default)]
    pub taker: Option<String>,
    #[serde(default)]
    pub maker: Option<String>,
    #[serde(default)]
    pub volume: Option<String>,
}

/// Account information.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub fees: FeeRates,
    #[serde(default)]
    pub capabilities: Vec<String>,
}

/// Balance of one asset.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    pub symbol: String,
    pub available: String,
    pub in_order: String,
}

/// Deposit address or bank details for an asset.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepositInfo {
    #[serde(default)]
    pub address: Option<String>,
    #[serde(default)]
    pub payment_id: Option<String>,
    #[serde(default)]
    pub iban: Option<String>,
    #[serde(default)]
    pub bic: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

/// Response of a withdrawal request.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawalResponse {
    pub success: bool,
    pub symbol: String,
    pub amount: String,
}

/// A historical deposit.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepositEntry {
    pub timestamp: u64,
    pub symbol: String,
    pub amount: String,
    #[serde(default)]
    pub address: Option<String>,
    #[serde(default)]
    pub payment_id: Option<String>,
    #[serde(default)]
    pub tx_id: Option<String>,
    #[serde(default)]
    pub fee: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
}

/// A historical withdrawal.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawalEntry {
    pub timestamp: u64,
    pub symbol: String,
    pub amount: String,
    #[serde(default)]
    pub address: Option<String>,
    #[serde(default)]
    pub payment_id: Option<String>,
    #[serde(default)]
    pub tx_id: Option<String>,
    #[serde(default)]
    pub fee: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
}

/// Ticker update pushed on the `ticker` WebSocket channel.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TickerEvent {
    pub market: String,
    #[serde(default)]
    pub best_bid: Option<String>,
    #[serde(default)]
    pub best_bid_size: Option<String>,
    #[serde(default)]
    pub best_ask: Option<String>,
    #[serde(default)]
    pub best_ask_size: Option<String>,
    #[serde(default)]
    pub last_price: Option<String>,
}

/// Trade pushed on the `trades` WebSocket channel.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeEvent {
    pub id: String,
    pub timestamp: u64,
    pub market: String,
    pub amount: String,
    pub price: String,
    pub side: Side,
}

/// Candle update pushed on the `candles` WebSocket channel.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CandleEvent {
    pub market: String,
    pub interval: String,
    pub candle: Vec<Candle>,
}

/// Order book delta pushed on the `book` WebSocket channel.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BookEvent {
    pub market: String,
    pub nonce: u64,
    #[serde(default)]
    pub bids: Vec<PriceLevel>,
    #[serde(default)]
    pub asks: Vec<PriceLevel>,
}

/// Order update pushed on the `account` WebSocket channel.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderEvent {
    pub order_id: String,
    pub market: String,
    #[serde(default)]
    pub client_order_id: Option<String>,
    #[serde(default)]
    pub created: Option<u64>,
    #[serde(default)]
    pub updated: Option<u64>,
    pub status: String,
    pub side: Side,
    #[serde(default)]
    pub order_type: Option<OrderType>,
    #[serde(default)]
    pub amount: Option<String>,
    #[serde(default)]
    pub amount_remaining: Option<String>,
    #[serde(default)]
    pub amount_quote: Option<String>,
    #[serde(default)]
    pub amount_quote_remaining: Option<String>,
    #[serde(default)]
    pub price: Option<String>,
    #[serde(default)]
    pub on_hold: Option<String>,
    #[serde(default)]
    pub on_hold_currency: Option<String>,
    #[serde(default)]
    pub time_in_force: Option<String>,
    #[serde(default)]
    pub post_only: Option<bool>,
    #[serde(default)]
    pub self_trade_prevention: Option<String>,
    #[serde(default)]
    pub visible: Option<bool>,
}

/// Fill pushed on the `account` WebSocket channel.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FillEvent {
    pub market: String,
    pub order_id: String,
    pub fill_id: String,
    pub timestamp: u64,
    pub side: Side,
    pub amount: String,
    pub price: String,
    #[serde(default)]
    pub taker: Option<bool>,
    #[serde(default)]
    pub fee: Option<String>,
    #[serde(default)]
    pub fee_currency: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn candle_deserializes_from_array() {
        let json = r#"[1548051540000, "3416.3", "3416.3", "3396.1", "3396.1", "4.14"]"#;
        let candle: Candle = serde_json::from_str(json).unwrap();
        assert_eq!(candle.timestamp, 1548051540000);
        assert_eq!(candle.open, "3416.3");
        assert_eq!(candle.volume, "4.14");
    }

    #[test]
    fn side_serializes_lowercase() {
        assert_eq!(serde_json::to_string(&Side::Buy).unwrap(), "\"buy\"");
        assert_eq!(serde_json::to_string(&Side::Sell).unwrap(), "\"sell\"");
    }

    #[test]
    fn order_type_serializes_camel_case() {
        assert_eq!(
            serde_json::to_string(&OrderType::StopLossLimit).unwrap(),
            "\"stopLossLimit\""
        );
    }
}
