use std::time::Duration;

use reqwest::{StatusCode, header::RETRY_AFTER};
use serde::{Serialize, de::DeserializeOwned};

use crate::{Error, error::ApiError};

const BASE_URL: &str = "https://api.themoviedb.org/3";
const TIMEOUT: Duration = Duration::from_secs(30);

pub(crate) const NONE: &[(&str, &str)] = &[];

#[derive(Debug, Clone)]
pub struct Client {
    http: reqwest::Client,
    base_url: String,
    token: String,
    language: Option<String>,
    timeout: Duration,
}

impl Client {
    pub fn new(token: impl Into<String>) -> Self {
        Self::with_base_url(token, BASE_URL)
    }

    pub fn with_base_url(token: impl Into<String>, base_url: impl Into<String>) -> Self {
        Self {
            http: reqwest::Client::new(),
            base_url: base_url.into(),
            token: token.into(),
            language: None,
            timeout: TIMEOUT,
        }
    }

    pub fn language(mut self, language: impl Into<String>) -> Self {
        self.language = Some(language.into());
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub(crate) async fn get<T: DeserializeOwned>(
        &self,
        path: &str,
        query: &(impl Serialize + ?Sized),
    ) -> Result<T, Error> {
        let response = self
            .http
            .get(format!("{}{path}", self.base_url))
            .bearer_auth(&self.token)
            .timeout(self.timeout)
            .query(query)
            .query(&[("language", &self.language)])
            .send()
            .await?;
        let status = response.status();

        if status == StatusCode::TOO_MANY_REQUESTS {
            let retry_after = response
                .headers()
                .get(RETRY_AFTER)
                .and_then(|value| value.to_str().ok())
                .and_then(|value| value.parse().ok())
                .map(Duration::from_secs);
            return Err(Error::RateLimited { retry_after });
        }

        let body = response.bytes().await?;

        if !status.is_success() {
            let message = match serde_json::from_slice::<ApiError>(&body) {
                Ok(error) => error.status_message,
                Err(_) => String::new(),
            };
            return Err(Error::Api {
                status: status.as_u16(),
                message,
            });
        }

        serde_json::from_slice(&body).map_err(Error::Decode)
    }

    pub(crate) async fn get_images<T: DeserializeOwned>(&self, path: &str) -> Result<T, Error> {
        let include = self.language.as_deref().map(|language| {
            let code = language.split_once('-').map_or(language, |(code, _)| code);
            format!("{code},null")
        });
        self.get(path, &[("include_image_language", include)]).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_targets_the_tmdb_api() {
        let client = Client::new("token");

        assert_eq!(client.base_url, BASE_URL);
        assert_eq!(client.language, None);
        assert_eq!(client.timeout, TIMEOUT);
    }
}
