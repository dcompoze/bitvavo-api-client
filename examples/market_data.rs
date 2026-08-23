//! Fetches public market data over REST.
//!
//! Run with `cargo run --example market_data`.

use bitvavo_client::rest::{CandlesParams, RestClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RestClient::public()?;

    let time = client.time().await?;
    println!("Server time: {}", time.time);

    let markets = client.markets().await?;
    println!("Number of markets: {}", markets.len());

    let ticker = client.ticker_price("BTC-EUR").await?;
    println!("BTC-EUR price: {}", ticker.price.unwrap_or_default());

    let book = client.order_book("BTC-EUR", Some(3)).await?;
    println!("Top of book:");
    if let (Some(bid), Some(ask)) = (book.bids.first(), book.asks.first()) {
        println!("  bid {} @ {}", bid[1], bid[0]);
        println!("  ask {} @ {}", ask[1], ask[0]);
    }

    let candles = client
        .candles("BTC-EUR", "1h", &CandlesParams::new().limit(3))
        .await?;
    println!("Last {} hourly candles:", candles.len());
    for candle in &candles {
        println!(
            "  {} open {} close {} volume {}",
            candle.timestamp, candle.open, candle.close, candle.volume
        );
    }

    println!("Rate limit remaining: {}", client.rate_limit_remaining());
    Ok(())
}
