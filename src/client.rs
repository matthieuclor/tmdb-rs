use serde::de::DeserializeOwned;

use crate::{Error, Movie, MovieSummary, Page, error::ApiError};

const BASE_URL: &str = "https://api.themoviedb.org/3";

#[derive(Debug, Clone)]
pub struct Client {
    http: reqwest::Client,
    base_url: String,
    token: String,
    language: Option<String>,
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
        }
    }

    pub fn language(mut self, language: impl Into<String>) -> Self {
        self.language = Some(language.into());
        self
    }

    pub async fn search_movies(&self, query: &str, page: u32) -> Result<Page<MovieSummary>, Error> {
        self.get(
            "/search/movie",
            &[("query", query), ("page", &page.to_string())],
        )
        .await
    }

    pub async fn movie(&self, id: u64) -> Result<Movie, Error> {
        self.get(&format!("/movie/{id}"), &[]).await
    }

    async fn get<T: DeserializeOwned>(
        &self,
        path: &str,
        query: &[(&str, &str)],
    ) -> Result<T, Error> {
        let mut request = self
            .http
            .get(format!("{}{path}", self.base_url))
            .bearer_auth(&self.token)
            .query(query);

        if let Some(language) = &self.language {
            request = request.query(&[("language", language)]);
        }

        let response = request.send().await?;
        let status = response.status();

        if !status.is_success() {
            let message = response
                .json::<ApiError>()
                .await
                .map(|error| error.status_message)
                .unwrap_or_default();
            return Err(Error::Api {
                status: status.as_u16(),
                message,
            });
        }

        Ok(response.json().await?)
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
    }
}
