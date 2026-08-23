//! Streams live tickers and trades over the WebSocket API.
//!
//! Run with `cargo run --example ws_ticker`.

use bitvavo_api_client::ws::{WsClient, WsEvent};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (client, mut events) = WsClient::connect_public().await?;

    client.subscribe_ticker(&["BTC-EUR", "ETH-EUR"])?;
    client.subscribe_trades(&["BTC-EUR"])?;

    let mut received = 0;
    while let Some(event) = events.recv().await {
        match event {
            WsEvent::Ticker(ticker) => {
                println!(
                    "{}: bid {} ask {}",
                    ticker.market,
                    ticker.best_bid.unwrap_or_default(),
                    ticker.best_ask.unwrap_or_default()
                );
            }
            WsEvent::Trade(trade) => {
                println!(
                    "trade {}: {:?} {} @ {}",
                    trade.market, trade.side, trade.amount, trade.price
                );
            }
            WsEvent::Subscribed(subscriptions) => {
                println!("subscribed: {subscriptions}");
            }
            WsEvent::Error { code, message } => {
                eprintln!("error {code}: {message}");
            }
            WsEvent::Closed => {
                println!("connection closed");
                break;
            }
            _ => {}
        }
        received += 1;
        if received >= 50 {
            break;
        }
    }

    client.close();
    Ok(())
}
