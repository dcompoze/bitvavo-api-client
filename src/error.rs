//! Error types returned by the client.

use thiserror::Error;

/// Errors produced by the REST and WebSocket clients.
#[derive(Debug, Error)]
pub enum Error {
    /// Transport-level HTTP failure.
    #[error("http request failed: {0}")]
    Http(#[from] reqwest::Error),

    /// Transport-level WebSocket failure.
    /// The error is boxed to keep the size of `Error` small.
    #[error("websocket error: {0}")]
    WebSocket(Box<tokio_tungstenite::tungstenite::Error>),

    /// Failure to serialize or deserialize a payload.
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),

    /// Error response from the Bitvavo API.
    /// Error codes are documented at <https://docs.bitvavo.com/>.
    #[error("bitvavo api error {code}: {message}")]
    Api { code: i64, message: String },

    /// A private endpoint was called without API credentials.
    #[error("missing api credentials")]
    MissingCredentials,

    /// The WebSocket connection is no longer usable.
    #[error("websocket connection closed")]
    ConnectionClosed,
}

impl From<tokio_tungstenite::tungstenite::Error> for Error {
    fn from(err: tokio_tungstenite::tungstenite::Error) -> Self {
        Self::WebSocket(Box::new(err))
    }
}

pub type Result<T> = std::result::Result<T, Error>;
