use std::fmt;

use serde::Deserialize;

#[derive(Debug)]
pub enum Error {
    Http(reqwest::Error),
    Api { status: u16, message: String },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Http(error) => write!(f, "TMDB request failed: {error}"),
            Self::Api { status, message } => write!(f, "TMDB returned {status}: {message}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Http(error) => Some(error),
            Self::Api { .. } => None,
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Self::Http(error)
    }
}

#[derive(Deserialize)]
pub(crate) struct ApiError {
    pub(crate) status_message: String,
}
