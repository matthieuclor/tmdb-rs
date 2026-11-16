use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Page<T> {
    pub page: u32,
    pub results: Vec<T>,
    pub total_pages: u32,
    pub total_results: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MovieSummary {
    pub id: u64,
    pub title: String,
    pub original_title: String,
    #[serde(default)]
    pub overview: String,
    #[serde(default)]
    pub release_date: String,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    #[serde(default)]
    pub vote_average: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Movie {
    pub id: u64,
    pub imdb_id: Option<String>,
    pub title: String,
    pub original_title: String,
    #[serde(default)]
    pub overview: String,
    pub tagline: Option<String>,
    #[serde(default)]
    pub release_date: String,
    pub runtime: Option<u32>,
    #[serde(default)]
    pub genres: Vec<Genre>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    #[serde(default)]
    pub vote_average: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Genre {
    pub id: u64,
    pub name: String,
}
