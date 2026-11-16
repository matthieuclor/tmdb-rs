use std::collections::HashMap;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::{Genre, WatchProvider};

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct ChangesQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChangedId {
    pub id: u64,
    pub adult: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Certification {
    pub certification: String,
    #[serde(default)]
    pub meaning: String,
    #[serde(default)]
    pub order: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Certifications {
    #[serde(default)]
    pub certifications: HashMap<String, Vec<Certification>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Genres {
    #[serde(default)]
    pub genres: Vec<Genre>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WatchProviderList {
    #[serde(default)]
    pub results: Vec<WatchProvider>,
}
