use std::{fmt, time::Duration};

use serde::Deserialize;

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    Network(Box<dyn std::error::Error + Send + Sync>),
    Timeout,
    Decode(serde_json::Error),
    RateLimited { retry_after: Option<Duration> },
    Api { status: u16, message: String },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Network(error) => write!(f, "TMDB request failed: {error}"),
            Self::Timeout => write!(f, "TMDB request timed out"),
            Self::Decode(error) => write!(f, "TMDB response could not be decoded: {error}"),
            Self::RateLimited { .. } => write!(f, "TMDB rate limit reached"),
            Self::Api { status, message } => write!(f, "TMDB returned {status}: {message}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Network(error) => Some(error.as_ref()),
            Self::Decode(error) => Some(error),
            _ => None,
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        if error.is_timeout() {
            Self::Timeout
        } else {
            Self::Network(Box::new(error))
        }
    }
}

#[derive(Deserialize)]
pub(crate) struct ApiError {
    pub(crate) status_message: String,
}
