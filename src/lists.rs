use serde::{Deserialize, Serialize};

use crate::{Client, Error, Media};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct List {
    #[serde(deserialize_with = "crate::serde_helpers::string_or_number")]
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub created_by: String,
    #[serde(default)]
    pub favorite_count: u32,
    #[serde(default)]
    pub item_count: u32,
    #[serde(default)]
    pub iso_639_1: String,
    pub poster_path: Option<String>,
    #[serde(default)]
    pub items: Vec<Media>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItemStatus {
    #[serde(deserialize_with = "crate::serde_helpers::string_or_number")]
    pub id: String,
    pub item_present: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct ListApi<'a> {
    client: &'a Client,
    id: &'a str,
}

impl Client {
    pub fn list<'a>(&'a self, id: &'a str) -> ListApi<'a> {
        ListApi { client: self, id }
    }
}

impl ListApi<'_> {
    pub async fn details(&self, page: u32) -> Result<List, Error> {
        self.client.get(&self.path(""), &[("page", page)]).await
    }

    pub async fn item_status(&self, movie_id: u64) -> Result<ItemStatus, Error> {
        self.client
            .get(&self.path("/item_status"), &[("movie_id", movie_id)])
            .await
    }

    fn path(&self, suffix: &str) -> String {
        format!("/list/{}{suffix}", self.id)
    }
}
