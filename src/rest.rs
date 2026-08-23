//! REST API client.
//!
//! Endpoint reference: <https://docs.bitvavo.com/>.

use crate::ClientConfig;
use crate::auth;
use crate::error::{Error, Result};
use crate::types::*;
use reqwest::Method;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicI64, Ordering};

/// Error payload returned by the API.
#[derive(Debug, Deserialize)]
struct ApiErrorBody {
    #[serde(rename = "errorCode")]
    error_code: i64,
    error: String,
}

/// Parameters for trade listing endpoints.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TradesParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_id_from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_id_to: Option<String>,
}

impl TradesParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn start(mut self, start_ms: u64) -> Self {
        self.start = Some(start_ms);
        self
    }

    pub fn end(mut self, end_ms: u64) -> Self {
        self.end = Some(end_ms);
        self
    }

    pub fn trade_id_from(mut self, id: impl Into<String>) -> Self {
        self.trade_id_from = Some(id.into());
        self
    }

    pub fn trade_id_to(mut self, id: impl Into<String>) -> Self {
        self.trade_id_to = Some(id.into());
        self
    }
}

/// Parameters for `GET /{market}/candles`.
#[derive(Debug, Clone, Default, Serialize)]
pub struct CandlesParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<u64>,
}

impl CandlesParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn start(mut self, start_ms: u64) -> Self {
        self.start = Some(start_ms);
        self
    }

    pub fn end(mut self, end_ms: u64) -> Self {
        self.end = Some(end_ms);
        self
    }
}

/// Parameters for `GET /orders`.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrdersParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id_from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id_to: Option<String>,
}

impl OrdersParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn start(mut self, start_ms: u64) -> Self {
        self.start = Some(start_ms);
        self
    }

    pub fn end(mut self, end_ms: u64) -> Self {
        self.end = Some(end_ms);
        self
    }

    pub fn order_id_from(mut self, id: impl Into<String>) -> Self {
        self.order_id_from = Some(id.into());
        self
    }

    pub fn order_id_to(mut self, id: impl Into<String>) -> Self {
        self.order_id_to = Some(id.into());
        self
    }
}

/// Parameters for deposit and withdrawal history endpoints.
#[derive(Debug, Clone, Default, Serialize)]
pub struct HistoryParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<u64>,
}

impl HistoryParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = Some(symbol.into());
        self
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn start(mut self, start_ms: u64) -> Self {
        self.start = Some(start_ms);
        self
    }

    pub fn end(mut self, end_ms: u64) -> Self {
        self.end = Some(end_ms);
        self
    }
}

/// Request body for `POST /order`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderRequest {
    pub market: String,
    pub side: Side,
    pub order_type: OrderType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_quote: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_trade_prevention: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_market_protection: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_reference: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator_id: Option<i64>,
}

impl OrderRequest {
    /// Creates an order request with only the required fields set.
    pub fn new(market: impl Into<String>, side: Side, order_type: OrderType) -> Self {
        Self {
            market: market.into(),
            side,
            order_type,
            amount: None,
            amount_quote: None,
            price: None,
            time_in_force: None,
            self_trade_prevention: None,
            post_only: None,
            disable_market_protection: None,
            response_required: None,
            trigger_type: None,
            trigger_reference: None,
            trigger_amount: None,
            client_order_id: None,
            operator_id: None,
        }
    }

    /// Creates a limit order request.
    pub fn limit(
        market: impl Into<String>,
        side: Side,
        amount: impl Into<String>,
        price: impl Into<String>,
    ) -> Self {
        Self::new(market, side, OrderType::Limit)
            .amount(amount)
            .price(price)
    }

    /// Creates a market order for an amount of the base asset.
    pub fn market_amount(market: impl Into<String>, side: Side, amount: impl Into<String>) -> Self {
        Self::new(market, side, OrderType::Market).amount(amount)
    }

    /// Creates a market order for an amount of the quote asset.
    pub fn market_amount_quote(
        market: impl Into<String>,
        side: Side,
        amount_quote: impl Into<String>,
    ) -> Self {
        Self::new(market, side, OrderType::Market).amount_quote(amount_quote)
    }

    pub fn amount(mut self, amount: impl Into<String>) -> Self {
        self.amount = Some(amount.into());
        self
    }

    pub fn amount_quote(mut self, amount_quote: impl Into<String>) -> Self {
        self.amount_quote = Some(amount_quote.into());
        self
    }

    pub fn price(mut self, price: impl Into<String>) -> Self {
        self.price = Some(price.into());
        self
    }

    pub fn time_in_force(mut self, tif: impl Into<String>) -> Self {
        self.time_in_force = Some(tif.into());
        self
    }

    pub fn self_trade_prevention(mut self, stp: impl Into<String>) -> Self {
        self.self_trade_prevention = Some(stp.into());
        self
    }

    pub fn post_only(mut self, post_only: bool) -> Self {
        self.post_only = Some(post_only);
        self
    }

    pub fn disable_market_protection(mut self, disable: bool) -> Self {
        self.disable_market_protection = Some(disable);
        self
    }

    pub fn trigger(
        mut self,
        trigger_type: impl Into<String>,
        reference: impl Into<String>,
        amount: impl Into<String>,
    ) -> Self {
        self.trigger_type = Some(trigger_type.into());
        self.trigger_reference = Some(reference.into());
        self.trigger_amount = Some(amount.into());
        self
    }

    pub fn client_order_id(mut self, id: impl Into<String>) -> Self {
        self.client_order_id = Some(id.into());
        self
    }

    pub fn operator_id(mut self, id: i64) -> Self {
        self.operator_id = Some(id);
        self
    }
}

/// Request body for `PUT /order`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateOrderRequest {
    pub market: String,
    pub order_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_remaining: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_trade_prevention: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator_id: Option<i64>,
}

impl UpdateOrderRequest {
    pub fn new(market: impl Into<String>, order_id: impl Into<String>) -> Self {
        Self {
            market: market.into(),
            order_id: order_id.into(),
            amount: None,
            amount_remaining: None,
            price: None,
            trigger_amount: None,
            time_in_force: None,
            self_trade_prevention: None,
            post_only: None,
            response_required: None,
            operator_id: None,
        }
    }

    pub fn amount(mut self, amount: impl Into<String>) -> Self {
        self.amount = Some(amount.into());
        self
    }

    pub fn amount_remaining(mut self, amount: impl Into<String>) -> Self {
        self.amount_remaining = Some(amount.into());
        self
    }

    pub fn price(mut self, price: impl Into<String>) -> Self {
        self.price = Some(price.into());
        self
    }

    pub fn trigger_amount(mut self, amount: impl Into<String>) -> Self {
        self.trigger_amount = Some(amount.into());
        self
    }

    pub fn operator_id(mut self, id: i64) -> Self {
        self.operator_id = Some(id);
        self
    }
}

/// Request body for `POST /withdrawal`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawalRequest {
    pub symbol: String,
    pub amount: String,
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_withdrawal_fee: Option<bool>,
}

impl WithdrawalRequest {
    pub fn new(
        symbol: impl Into<String>,
        amount: impl Into<String>,
        address: impl Into<String>,
    ) -> Self {
        Self {
            symbol: symbol.into(),
            amount: amount.into(),
            address: address.into(),
            payment_id: None,
            internal: None,
            add_withdrawal_fee: None,
        }
    }

    pub fn payment_id(mut self, id: impl Into<String>) -> Self {
        self.payment_id = Some(id.into());
        self
    }

    pub fn internal(mut self, internal: bool) -> Self {
        self.internal = Some(internal);
        self
    }

    pub fn add_withdrawal_fee(mut self, add: bool) -> Self {
        self.add_withdrawal_fee = Some(add);
        self
    }
}

/// Query string pairs for an endpoint.
type Query = Vec<(String, String)>;

/// Serializes a params struct into query string pairs.
fn to_query<P: Serialize>(params: &P) -> Result<Query> {
    let value = serde_json::to_value(params)?;
    let mut query = Vec::new();
    if let serde_json::Value::Object(map) = value {
        for (key, val) in map {
            let text = match val {
                serde_json::Value::String(s) => s,
                serde_json::Value::Null => continue,
                other => other.to_string(),
            };
            query.push((key, text));
        }
    }
    Ok(query)
}

/// Async client for the Bitvavo REST API.
pub struct RestClient {
    http: reqwest::Client,
    config: ClientConfig,
    rate_limit_remaining: AtomicI64,
    rate_limit_reset_at: AtomicI64,
}

impl RestClient {
    /// Creates a client from a configuration.
    pub fn new(config: ClientConfig) -> Result<Self> {
        let http = reqwest::Client::builder().timeout(config.timeout).build()?;
        Ok(Self {
            http,
            config,
            rate_limit_remaining: AtomicI64::new(1000),
            rate_limit_reset_at: AtomicI64::new(0),
        })
    }

    /// Creates a client without credentials for public endpoints.
    pub fn public() -> Result<Self> {
        Self::new(ClientConfig::new())
    }

    /// Creates a client from the `BITVAVO_API_KEY` and `BITVAVO_API_SECRET`
    /// environment variables.
    pub fn from_env() -> Result<Self> {
        Self::new(ClientConfig::from_env())
    }

    /// Returns the number of rate limit points left in the current window.
    pub fn rate_limit_remaining(&self) -> i64 {
        self.rate_limit_remaining.load(Ordering::Relaxed)
    }

    /// Returns the Unix time in milliseconds at which the rate limit resets.
    pub fn rate_limit_reset_at(&self) -> i64 {
        self.rate_limit_reset_at.load(Ordering::Relaxed)
    }

    fn update_rate_limit(&self, headers: &reqwest::header::HeaderMap) {
        if let Some(remaining) = headers
            .get("bitvavo-ratelimit-remaining")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse().ok())
        {
            self.rate_limit_remaining
                .store(remaining, Ordering::Relaxed);
        }
        if let Some(reset) = headers
            .get("bitvavo-ratelimit-resetat")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse().ok())
        {
            self.rate_limit_reset_at.store(reset, Ordering::Relaxed);
        }
    }

    /// Sends a request and decodes the response.
    /// Requests are signed whenever credentials are configured.
    async fn request<T: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        query: Query,
        body: Option<serde_json::Value>,
    ) -> Result<T> {
        let mut path_with_query = path.to_string();
        if !query.is_empty() {
            let joined: Vec<String> = query.iter().map(|(k, v)| format!("{k}={v}")).collect();
            path_with_query.push('?');
            path_with_query.push_str(&joined.join("&"));
        }
        let url = format!("{}{}", self.config.rest_url, path_with_query);
        let body_text = match &body {
            Some(value) => serde_json::to_string(value)?,
            None => String::new(),
        };

        let mut request = self.http.request(method.clone(), &url);
        if let Ok((key, secret)) = self.config.credentials() {
            let timestamp = auth::timestamp_ms();
            let signature = auth::sign(
                secret,
                timestamp,
                method.as_str(),
                &path_with_query,
                &body_text,
            );
            request = request
                .header("bitvavo-access-key", key)
                .header("bitvavo-access-signature", signature)
                .header("bitvavo-access-timestamp", timestamp.to_string())
                .header(
                    "bitvavo-access-window",
                    self.config.access_window_ms.to_string(),
                );
        }
        if body.is_some() {
            request = request
                .header("content-type", "application/json")
                .body(body_text);
        }

        let response = request.send().await?;
        self.update_rate_limit(response.headers());
        let status = response.status();
        let text = response.text().await?;

        if let Ok(err) = serde_json::from_str::<ApiErrorBody>(&text) {
            return Err(Error::Api {
                code: err.error_code,
                message: err.error,
            });
        }
        if !status.is_success() {
            return Err(Error::Api {
                code: status.as_u16() as i64,
                message: text,
            });
        }
        Ok(serde_json::from_str(&text)?)
    }

    async fn get<T: DeserializeOwned>(&self, path: &str, query: Query) -> Result<T> {
        self.request(Method::GET, path, query, None).await
    }

    // Public endpoints.

    /// `GET /time`. Returns the server time.
    pub async fn time(&self) -> Result<ServerTime> {
        self.get("/time", Vec::new()).await
    }

    /// `GET /markets`. Returns all markets.
    pub async fn markets(&self) -> Result<Vec<Market>> {
        self.get("/markets", Vec::new()).await
    }

    /// `GET /markets?market={market}`. Returns a single market.
    pub async fn market(&self, market: &str) -> Result<Market> {
        self.get("/markets", vec![("market".into(), market.into())])
            .await
    }

    /// `GET /assets`. Returns all assets.
    pub async fn assets(&self) -> Result<Vec<Asset>> {
        self.get("/assets", Vec::new()).await
    }

    /// `GET /assets?symbol={symbol}`. Returns a single asset.
    pub async fn asset(&self, symbol: &str) -> Result<Asset> {
        self.get("/assets", vec![("symbol".into(), symbol.into())])
            .await
    }

    /// `GET /{market}/book`. Returns the order book for a market.
    pub async fn order_book(&self, market: &str, depth: Option<u32>) -> Result<OrderBook> {
        let mut query = Vec::new();
        if let Some(depth) = depth {
            query.push(("depth".into(), depth.to_string()));
        }
        self.get(&format!("/{market}/book"), query).await
    }

    /// `GET /{market}/trades`. Returns public trades for a market.
    pub async fn public_trades(
        &self,
        market: &str,
        params: &TradesParams,
    ) -> Result<Vec<PublicTrade>> {
        self.get(&format!("/{market}/trades"), to_query(params)?)
            .await
    }

    /// `GET /{market}/candles`. Returns OHLCV candles for a market.
    /// Supported intervals: `1m`, `5m`, `15m`, `30m`, `1h`, `2h`, `4h`, `6h`, `8h`, `12h`, `1d`.
    pub async fn candles(
        &self,
        market: &str,
        interval: &str,
        params: &CandlesParams,
    ) -> Result<Vec<Candle>> {
        let mut query = vec![("interval".to_string(), interval.to_string())];
        query.extend(to_query(params)?);
        self.get(&format!("/{market}/candles"), query).await
    }

    /// `GET /ticker/price`. Returns the latest price for all markets.
    pub async fn ticker_prices(&self) -> Result<Vec<TickerPrice>> {
        self.get("/ticker/price", Vec::new()).await
    }

    /// `GET /ticker/price?market={market}`. Returns the latest price for one market.
    pub async fn ticker_price(&self, market: &str) -> Result<TickerPrice> {
        self.get("/ticker/price", vec![("market".into(), market.into())])
            .await
    }

    /// `GET /ticker/book`. Returns the best bid and ask for all markets.
    pub async fn ticker_books(&self) -> Result<Vec<TickerBook>> {
        self.get("/ticker/book", Vec::new()).await
    }

    /// `GET /ticker/book?market={market}`. Returns the best bid and ask for one market.
    pub async fn ticker_book(&self, market: &str) -> Result<TickerBook> {
        self.get("/ticker/book", vec![("market".into(), market.into())])
            .await
    }

    /// `GET /ticker/24h`. Returns 24 hour statistics for all markets.
    pub async fn tickers_24h(&self) -> Result<Vec<Ticker24h>> {
        self.get("/ticker/24h", Vec::new()).await
    }

    /// `GET /ticker/24h?market={market}`. Returns 24 hour statistics for one market.
    pub async fn ticker_24h(&self, market: &str) -> Result<Ticker24h> {
        self.get("/ticker/24h", vec![("market".into(), market.into())])
            .await
    }

    // Private endpoints.

    /// `POST /order`. Places a new order.
    pub async fn place_order(&self, order: &OrderRequest) -> Result<Order> {
        self.config.credentials()?;
        self.request(
            Method::POST,
            "/order",
            Vec::new(),
            Some(serde_json::to_value(order)?),
        )
        .await
    }

    /// `GET /order`. Returns a single order.
    pub async fn get_order(&self, market: &str, order_id: &str) -> Result<Order> {
        self.config.credentials()?;
        let query = vec![
            ("market".into(), market.into()),
            ("orderId".into(), order_id.into()),
        ];
        self.get("/order", query).await
    }

    /// `PUT /order`. Updates an open order.
    pub async fn update_order(&self, update: &UpdateOrderRequest) -> Result<Order> {
        self.config.credentials()?;
        self.request(
            Method::PUT,
            "/order",
            Vec::new(),
            Some(serde_json::to_value(update)?),
        )
        .await
    }

    /// `DELETE /order`. Cancels a single order.
    pub async fn cancel_order(
        &self,
        market: &str,
        order_id: &str,
        operator_id: Option<i64>,
    ) -> Result<CanceledOrder> {
        self.config.credentials()?;
        let mut query = vec![
            ("market".to_string(), market.to_string()),
            ("orderId".to_string(), order_id.to_string()),
        ];
        if let Some(id) = operator_id {
            query.push(("operatorId".into(), id.to_string()));
        }
        self.request(Method::DELETE, "/order", query, None).await
    }

    /// `GET /orders`. Returns orders for a market.
    pub async fn get_orders(&self, market: &str, params: &OrdersParams) -> Result<Vec<Order>> {
        self.config.credentials()?;
        let mut query = vec![("market".to_string(), market.to_string())];
        query.extend(to_query(params)?);
        self.get("/orders", query).await
    }

    /// `DELETE /orders`. Cancels all open orders, optionally for one market.
    pub async fn cancel_orders(&self, market: Option<&str>) -> Result<Vec<CanceledOrder>> {
        self.config.credentials()?;
        let mut query = Vec::new();
        if let Some(market) = market {
            query.push(("market".to_string(), market.to_string()));
        }
        self.request(Method::DELETE, "/orders", query, None).await
    }

    /// `GET /ordersOpen`. Returns all open orders, optionally for one market.
    pub async fn open_orders(&self, market: Option<&str>) -> Result<Vec<Order>> {
        self.config.credentials()?;
        let mut query = Vec::new();
        if let Some(market) = market {
            query.push(("market".to_string(), market.to_string()));
        }
        self.get("/ordersOpen", query).await
    }

    /// `GET /trades`. Returns your trades for a market.
    pub async fn trades(&self, market: &str, params: &TradesParams) -> Result<Vec<Trade>> {
        self.config.credentials()?;
        let mut query = vec![("market".to_string(), market.to_string())];
        query.extend(to_query(params)?);
        self.get("/trades", query).await
    }

    /// `GET /account`. Returns account information including fee rates.
    pub async fn account(&self) -> Result<Account> {
        self.config.credentials()?;
        self.get("/account", Vec::new()).await
    }

    /// `GET /account/fees`. Returns fee rates, optionally for one market.
    pub async fn fees(&self, market: Option<&str>) -> Result<FeeRates> {
        self.config.credentials()?;
        let mut query = Vec::new();
        if let Some(market) = market {
            query.push(("market".to_string(), market.to_string()));
        }
        self.get("/account/fees", query).await
    }

    /// `GET /balance`. Returns balances for all assets.
    pub async fn balances(&self) -> Result<Vec<Balance>> {
        self.config.credentials()?;
        self.get("/balance", Vec::new()).await
    }

    /// `GET /balance?symbol={symbol}`. Returns the balance for one asset.
    pub async fn balance(&self, symbol: &str) -> Result<Vec<Balance>> {
        self.config.credentials()?;
        self.get("/balance", vec![("symbol".into(), symbol.into())])
            .await
    }

    /// `GET /depositAssets`. Returns deposit details for an asset.
    pub async fn deposit_assets(&self, symbol: &str) -> Result<DepositInfo> {
        self.config.credentials()?;
        self.get("/depositAssets", vec![("symbol".into(), symbol.into())])
            .await
    }

    /// `POST /withdrawal`. Requests a withdrawal.
    pub async fn withdraw_assets(&self, request: &WithdrawalRequest) -> Result<WithdrawalResponse> {
        self.config.credentials()?;
        self.request(
            Method::POST,
            "/withdrawal",
            Vec::new(),
            Some(serde_json::to_value(request)?),
        )
        .await
    }

    /// `GET /depositHistory`. Returns the deposit history.
    pub async fn deposit_history(&self, params: &HistoryParams) -> Result<Vec<DepositEntry>> {
        self.config.credentials()?;
        self.get("/depositHistory", to_query(params)?).await
    }

    /// `GET /withdrawalHistory`. Returns the withdrawal history.
    pub async fn withdrawal_history(&self, params: &HistoryParams) -> Result<Vec<WithdrawalEntry>> {
        self.config.credentials()?;
        self.get("/withdrawalHistory", to_query(params)?).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_query_skips_none_fields() {
        let params = TradesParams::new().limit(10);
        let query = to_query(&params).unwrap();
        assert_eq!(query, vec![("limit".to_string(), "10".to_string())]);
    }

    #[test]
    fn order_request_serializes_required_fields() {
        let order = OrderRequest::limit("BTC-EUR", Side::Buy, "0.1", "20000");
        let value = serde_json::to_value(&order).unwrap();
        assert_eq!(value["market"], "BTC-EUR");
        assert_eq!(value["side"], "buy");
        assert_eq!(value["orderType"], "limit");
        assert_eq!(value["amount"], "0.1");
        assert_eq!(value["price"], "20000");
        assert!(value.get("postOnly").is_none());
    }
}
