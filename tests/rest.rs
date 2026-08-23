//! Integration tests for the REST client.
//!
//! Public tests run against the live API and need network access.
//! Private tests run only when `BITVAVO_API_KEY` and `BITVAVO_API_SECRET`
//! are set in the environment or in a `.env` file.

use bitvavo_api_client::ClientConfig;
use bitvavo_api_client::rest::{CandlesParams, RestClient, TradesParams};

fn public_client() -> RestClient {
    RestClient::public().expect("client construction")
}

fn private_client() -> Option<RestClient> {
    dotenvy::dotenv().ok();
    let config = ClientConfig::from_env();
    if !config.has_credentials() {
        eprintln!("skipping private test: BITVAVO_API_KEY or BITVAVO_API_SECRET not set");
        return None;
    }
    Some(RestClient::new(config).expect("client construction"))
}

#[tokio::test]
async fn get_time() {
    let time = public_client().time().await.unwrap();
    assert!(time.time > 1_600_000_000_000);
}

#[tokio::test]
async fn get_markets() {
    let markets = public_client().markets().await.unwrap();
    assert!(!markets.is_empty());
    assert!(markets.iter().any(|m| m.market == "BTC-EUR"));
}

#[tokio::test]
async fn get_single_market() {
    let market = public_client().market("BTC-EUR").await.unwrap();
    assert_eq!(market.market, "BTC-EUR");
    assert_eq!(market.base, "BTC");
    assert_eq!(market.quote, "EUR");
}

#[tokio::test]
async fn get_assets() {
    let assets = public_client().assets().await.unwrap();
    assert!(assets.iter().any(|a| a.symbol == "BTC"));
}

#[tokio::test]
async fn get_order_book() {
    let book = public_client()
        .order_book("BTC-EUR", Some(5))
        .await
        .unwrap();
    assert_eq!(book.market, "BTC-EUR");
    assert!(book.bids.len() <= 5);
    assert!(book.asks.len() <= 5);
}

#[tokio::test]
async fn get_public_trades() {
    let params = TradesParams::new().limit(10);
    let trades = public_client()
        .public_trades("BTC-EUR", &params)
        .await
        .unwrap();
    assert!(trades.len() <= 10);
}

#[tokio::test]
async fn get_candles() {
    let params = CandlesParams::new().limit(5);
    let candles = public_client()
        .candles("BTC-EUR", "1h", &params)
        .await
        .unwrap();
    assert!(!candles.is_empty());
    assert!(candles.len() <= 5);
    assert!(candles[0].timestamp > 0);
}

#[tokio::test]
async fn get_ticker_price() {
    let ticker = public_client().ticker_price("BTC-EUR").await.unwrap();
    assert_eq!(ticker.market, "BTC-EUR");
    assert!(ticker.price.is_some());
}

#[tokio::test]
async fn get_ticker_book() {
    let ticker = public_client().ticker_book("BTC-EUR").await.unwrap();
    assert_eq!(ticker.market, "BTC-EUR");
}

#[tokio::test]
async fn get_ticker_24h() {
    let ticker = public_client().ticker_24h("BTC-EUR").await.unwrap();
    assert_eq!(ticker.market, "BTC-EUR");
}

#[tokio::test]
async fn unknown_market_returns_api_error() {
    let result = public_client().ticker_price("NOPE-NOPE").await;
    match result {
        Err(bitvavo_api_client::Error::Api { code, .. }) => assert!(code > 0),
        other => panic!("expected api error, got {other:?}"),
    }
}

#[tokio::test]
async fn rate_limit_is_tracked() {
    let client = public_client();
    client.time().await.unwrap();
    assert!(client.rate_limit_remaining() < 1000);
}

#[tokio::test]
async fn private_get_account() {
    let Some(client) = private_client() else {
        return;
    };
    let account = client.account().await.unwrap();
    assert!(account.fees.taker.is_some() || account.fees.maker.is_some());
}

#[tokio::test]
async fn private_get_balances() {
    let Some(client) = private_client() else {
        return;
    };
    client.balances().await.unwrap();
}

#[tokio::test]
async fn private_get_open_orders() {
    let Some(client) = private_client() else {
        return;
    };
    client.open_orders(Some("BTC-EUR")).await.unwrap();
}
