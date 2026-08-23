//! Integration tests for the WebSocket client.
//!
//! These tests run against the live WebSocket API and need network access.

use bitvavo_api_client::ws::{WsClient, WsEvent};
use std::time::Duration;
use tokio::time::timeout;

const WAIT: Duration = Duration::from_secs(30);

#[tokio::test]
async fn subscribe_ticker_receives_events() {
    let (client, mut events) = WsClient::connect_public().await.unwrap();
    client.subscribe_ticker(&["BTC-EUR"]).unwrap();

    let result = timeout(WAIT, async {
        while let Some(event) = events.recv().await {
            match event {
                WsEvent::Ticker(ticker) => {
                    assert_eq!(ticker.market, "BTC-EUR");
                    return true;
                }
                WsEvent::Error { code, message } => {
                    panic!("websocket error {code}: {message}");
                }
                WsEvent::Closed => return false,
                _ => {}
            }
        }
        false
    })
    .await;

    assert!(result.unwrap_or(false), "no ticker event received in time");
    client.close();
}

#[tokio::test]
async fn get_book_returns_snapshot() {
    let (client, mut events) = WsClient::connect_public().await.unwrap();
    client.get_book("BTC-EUR").unwrap();

    let result = timeout(WAIT, async {
        while let Some(event) = events.recv().await {
            match event {
                WsEvent::Response { action, response } => {
                    assert_eq!(action, "getBook");
                    assert_eq!(response["market"], "BTC-EUR");
                    return true;
                }
                WsEvent::Error { code, message } => {
                    panic!("websocket error {code}: {message}");
                }
                WsEvent::Closed => return false,
                _ => {}
            }
        }
        false
    })
    .await;

    assert!(result.unwrap_or(false), "no book snapshot received in time");
    client.close();
}
