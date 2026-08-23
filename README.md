# bitvavo-client

A Rust client library for the [Bitvavo API](https://docs.bitvavo.com/):

- Full REST API coverage: market data, trading, account, balances, deposits, and withdrawals
- WebSocket support for real-time public and private channels
- HMAC-SHA256 authentication for REST and WebSocket
- Async/await support with Tokio
- Strongly typed request/response structures
- Rate limit visibility from response headers
- Amounts and prices kept as strings to preserve exact decimal values

## Library

Public REST API client:

```rust
use bitvavo_client::rest::{CandlesParams, RestClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RestClient::public()?;

    let ticker = client.ticker_price("BTC-EUR").await?;
    println!("BTC-EUR: {}", ticker.price.unwrap_or_default());

    let candles = client
        .candles("BTC-EUR", "1h", &CandlesParams::new().limit(10))
        .await?;
    for candle in &candles {
        println!("{}: close {}", candle.timestamp, candle.close);
    }

    Ok(())
}
```

Authenticated client:

```rust
use bitvavo_client::rest::{OrderRequest, RestClient};
use bitvavo_client::{ClientConfig, Side};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RestClient::new(ClientConfig::with_credentials(
        "your_api_key",
        "your_api_secret",
    ))?;

    let balances = client.balances().await?;
    for balance in &balances {
        println!("{}: {}", balance.symbol, balance.available);
    }

    let order = OrderRequest::limit("BTC-EUR", Side::Buy, "0.001", "50000");
    let result = client.place_order(&order).await?;
    println!("Order ID: {}", result.order_id);

    Ok(())
}
```

WebSocket public channels:

```rust
use bitvavo_client::ws::{WsClient, WsEvent};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (client, mut events) = WsClient::connect_public().await?;

    client.subscribe_ticker(&["BTC-EUR", "ETH-EUR"])?;
    client.subscribe_trades(&["BTC-EUR"])?;

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
                println!("trade: {} {} @ {}", trade.market, trade.amount, trade.price);
            }
            WsEvent::Closed => break,
            _ => {}
        }
    }

    Ok(())
}
```

WebSocket private channel:

```rust
use bitvavo_client::ws::{WsClient, WsEvent};
use bitvavo_client::ClientConfig;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClientConfig::with_credentials("your_api_key", "your_api_secret");
    let (client, mut events) = WsClient::connect(config).await?;

    client.authenticate()?;

    while let Some(event) = events.recv().await {
        match event {
            WsEvent::Authenticated => {
                client.subscribe_account(&["BTC-EUR"])?;
            }
            WsEvent::Order(order) => println!("order update: {order:?}"),
            WsEvent::Fill(fill) => println!("fill: {fill:?}"),
            WsEvent::Closed => break,
            _ => {}
        }
    }

    Ok(())
}
```

## API coverage

REST endpoints:

| Method | Endpoint | Authentication |
|--------|----------|----------------|
| `time()` | `GET /time` | Public |
| `markets()`, `market()` | `GET /markets` | Public |
| `assets()`, `asset()` | `GET /assets` | Public |
| `order_book()` | `GET /{market}/book` | Public |
| `public_trades()` | `GET /{market}/trades` | Public |
| `candles()` | `GET /{market}/candles` | Public |
| `ticker_price()`, `ticker_prices()` | `GET /ticker/price` | Public |
| `ticker_book()`, `ticker_books()` | `GET /ticker/book` | Public |
| `ticker_24h()`, `tickers_24h()` | `GET /ticker/24h` | Public |
| `place_order()` | `POST /order` | Required |
| `get_order()` | `GET /order` | Required |
| `update_order()` | `PUT /order` | Required |
| `cancel_order()` | `DELETE /order` | Required |
| `get_orders()` | `GET /orders` | Required |
| `cancel_orders()` | `DELETE /orders` | Required |
| `open_orders()` | `GET /ordersOpen` | Required |
| `trades()` | `GET /trades` | Required |
| `account()` | `GET /account` | Required |
| `fees()` | `GET /account/fees` | Required |
| `balances()`, `balance()` | `GET /balance` | Required |
| `deposit_assets()` | `GET /depositAssets` | Required |
| `withdraw_assets()` | `POST /withdrawal` | Required |
| `deposit_history()` | `GET /depositHistory` | Required |
| `withdrawal_history()` | `GET /withdrawalHistory` | Required |

WebSocket channels:

- `ticker` - Best bid, best ask, and last price updates
- `ticker24h` - 24 hour statistics
- `trades` - Public trades
- `candles` - Candlestick updates per interval
- `book` - Order book deltas, with `get_book()` for snapshots
- `account` - Order and fill updates for your account (requires authentication)

The WebSocket client does not reconnect on its own.
A `WsEvent::Closed` event signals that the connection ended.
Reconnect with `WsClient::connect` and resubscribe when this happens.

## Configuration

```rust
use bitvavo_client::ClientConfig;
use std::time::Duration;

let config = ClientConfig::with_credentials("api_key", "api_secret")
    .access_window_ms(10_000)                  // Signed request validity window
    .timeout(Duration::from_secs(30))          // HTTP request timeout
    .rest_url("https://api.bitvavo.com/v2")    // REST base URL override
    .ws_url("wss://ws.bitvavo.com/v2/");       // WebSocket URL override
```

## Environment variables

You can set credentials via environment variables:

```bash
export BITVAVO_API_KEY=""
export BITVAVO_API_SECRET=""
```

`ClientConfig::from_env()` and `RestClient::from_env()` read these variables.
The examples and integration tests also load them from a `.env` file via `dotenvy`.

## Examples

```bash
cargo run --example market_data   # Public REST market data
cargo run --example account      # Private REST account data
cargo run --example ws_ticker    # Live tickers and trades over WebSocket
```

## Tests

```bash
cargo test --lib                 # Unit tests, no network access needed
cargo test --test rest --test ws # Integration tests against the live API
```

Private integration tests run only when API credentials are present.
They are skipped otherwise.
