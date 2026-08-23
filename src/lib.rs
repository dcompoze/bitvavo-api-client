//! Async Rust client for the [Bitvavo](https://docs.bitvavo.com/) exchange API.
//!
//! The library exposes two entry points:
//! - [`rest::RestClient`] for the REST API.
//! - [`ws::WsClient`] for the WebSocket API.
//!
//! Public endpoints work without credentials.
//! Private endpoints require an API key and secret, either passed directly
//! or read from the `BITVAVO_API_KEY` and `BITVAVO_API_SECRET` environment variables.

mod auth;
#[cfg(test)]
mod tests;

pub mod error;
pub mod rest;
pub mod types;
pub mod ws;

pub use error::{Error, Result};
pub use types::{OrderType, Side};

use std::time::Duration;

/// Default REST API base URL.
pub const DEFAULT_REST_URL: &str = "https://api.bitvavo.com/v2";
/// Default WebSocket URL.
pub const DEFAULT_WS_URL: &str = "wss://ws.bitvavo.com/v2/";
/// Default access window in milliseconds.
pub const DEFAULT_ACCESS_WINDOW_MS: u64 = 10_000;

/// Configuration shared by the REST and WebSocket clients.
///
/// The `Debug` implementation redacts the API key and secret.
#[derive(Clone)]
pub struct ClientConfig {
    pub api_key: Option<String>,
    pub api_secret: Option<String>,
    pub rest_url: String,
    pub ws_url: String,
    /// Time in milliseconds during which a signed request stays valid.
    pub access_window_ms: u64,
    /// HTTP request timeout.
    pub timeout: Duration,
}

impl std::fmt::Debug for ClientConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClientConfig")
            .field("api_key", &self.api_key.as_ref().map(|_| "<redacted>"))
            .field(
                "api_secret",
                &self.api_secret.as_ref().map(|_| "<redacted>"),
            )
            .field("rest_url", &self.rest_url)
            .field("ws_url", &self.ws_url)
            .field("access_window_ms", &self.access_window_ms)
            .field("timeout", &self.timeout)
            .finish()
    }
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            api_key: None,
            api_secret: None,
            rest_url: DEFAULT_REST_URL.to_string(),
            ws_url: DEFAULT_WS_URL.to_string(),
            access_window_ms: DEFAULT_ACCESS_WINDOW_MS,
            timeout: Duration::from_secs(30),
        }
    }
}

impl ClientConfig {
    /// Creates a configuration without credentials for public endpoints.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a configuration with API credentials.
    pub fn with_credentials(api_key: impl Into<String>, api_secret: impl Into<String>) -> Self {
        Self {
            api_key: Some(api_key.into()),
            api_secret: Some(api_secret.into()),
            ..Self::default()
        }
    }

    /// Reads credentials from the `BITVAVO_API_KEY` and `BITVAVO_API_SECRET`
    /// environment variables.
    /// Returns a public-only configuration when the variables are not set.
    pub fn from_env() -> Self {
        let api_key = std::env::var("BITVAVO_API_KEY")
            .ok()
            .filter(|v| !v.is_empty());
        let api_secret = std::env::var("BITVAVO_API_SECRET")
            .ok()
            .filter(|v| !v.is_empty());
        Self {
            api_key,
            api_secret,
            ..Self::default()
        }
    }

    /// Overrides the REST base URL.
    pub fn rest_url(mut self, url: impl Into<String>) -> Self {
        self.rest_url = url.into();
        self
    }

    /// Overrides the WebSocket URL.
    pub fn ws_url(mut self, url: impl Into<String>) -> Self {
        self.ws_url = url.into();
        self
    }

    /// Sets the access window in milliseconds.
    pub fn access_window_ms(mut self, window: u64) -> Self {
        self.access_window_ms = window;
        self
    }

    /// Sets the HTTP request timeout.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Returns `true` when both API key and secret are set.
    pub fn has_credentials(&self) -> bool {
        self.api_key.is_some() && self.api_secret.is_some()
    }

    pub(crate) fn credentials(&self) -> Result<(&str, &str)> {
        match (self.api_key.as_deref(), self.api_secret.as_deref()) {
            (Some(key), Some(secret)) => Ok((key, secret)),
            _ => Err(Error::MissingCredentials),
        }
    }
}
