use serde::Serialize;

use crate::{
    Client, CollectionSummary, Company, Error, Keyword, Media, MovieSummary, Page, PersonSummary,
    TvSummary,
};

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct SearchQuery {
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_adult: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_release_year: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_air_date_year: Option<u32>,
}

impl SearchQuery {
    pub fn new(query: impl Into<String>) -> Self {
        Self {
            query: query.into(),
            ..Self::default()
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SearchApi<'a> {
    client: &'a Client,
}

impl Client {
    pub fn search(&self) -> SearchApi<'_> {
        SearchApi { client: self }
    }
}

impl SearchApi<'_> {
    pub async fn collections(&self, query: &SearchQuery) -> Result<Page<CollectionSummary>, Error> {
        self.client.get("/search/collection", query).await
    }

    pub async fn companies(&self, query: &SearchQuery) -> Result<Page<Company>, Error> {
        self.client.get("/search/company", query).await
    }

    pub async fn keywords(&self, query: &SearchQuery) -> Result<Page<Keyword>, Error> {
        self.client.get("/search/keyword", query).await
    }

    pub async fn movies(&self, query: &SearchQuery) -> Result<Page<MovieSummary>, Error> {
        self.client.get("/search/movie", query).await
    }

    pub async fn multi(&self, query: &SearchQuery) -> Result<Page<Media>, Error> {
        self.client.get("/search/multi", query).await
    }

    pub async fn people(&self, query: &SearchQuery) -> Result<Page<PersonSummary>, Error> {
        self.client.get("/search/person", query).await
    }

    pub async fn tv(&self, query: &SearchQuery) -> Result<Page<TvSummary>, Error> {
        self.client.get("/search/tv", query).await
    }
}
