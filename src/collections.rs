use serde::{Deserialize, Serialize};

use crate::{Client, Error, Images, MovieSummary, Translations, client::NONE};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Collection {
    pub id: u64,
    pub name: String,
    #[serde(default)]
    pub original_name: String,
    #[serde(default)]
    pub original_language: String,
    #[serde(default)]
    pub overview: String,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    #[serde(default)]
    pub parts: Vec<MovieSummary>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CollectionTranslation {
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub overview: String,
    #[serde(default)]
    pub homepage: String,
}

#[derive(Debug, Clone, Copy)]
pub struct CollectionApi<'a> {
    client: &'a Client,
    id: u64,
}

impl Client {
    pub fn collection(&self, id: u64) -> CollectionApi<'_> {
        CollectionApi { client: self, id }
    }
}

impl CollectionApi<'_> {
    pub async fn details(&self) -> Result<Collection, Error> {
        self.client.get(&self.path(""), NONE).await
    }

    pub async fn images(&self) -> Result<Images, Error> {
        self.client.get_images(&self.path("/images")).await
    }

    pub async fn translations(&self) -> Result<Translations<CollectionTranslation>, Error> {
        self.client.get(&self.path("/translations"), NONE).await
    }

    fn path(&self, suffix: &str) -> String {
        format!("/collection/{}{suffix}", self.id)
    }
}
