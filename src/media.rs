use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::Gender;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MovieSummary {
    pub id: u64,
    pub title: String,
    #[serde(default)]
    pub original_title: String,
    #[serde(default)]
    pub original_language: String,
    #[serde(default)]
    pub overview: String,
    #[serde(default, deserialize_with = "crate::serde_helpers::date")]
    pub release_date: Option<NaiveDate>,
    #[serde(default)]
    pub genre_ids: Vec<u64>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    #[serde(default)]
    pub adult: bool,
    #[serde(default)]
    pub video: bool,
    #[serde(default)]
    pub popularity: f64,
    #[serde(default)]
    pub vote_average: f64,
    #[serde(default)]
    pub vote_count: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TvSummary {
    pub id: u64,
    pub name: String,
    #[serde(default)]
    pub original_name: String,
    #[serde(default)]
    pub original_language: String,
    #[serde(default)]
    pub overview: String,
    #[serde(default, deserialize_with = "crate::serde_helpers::date")]
    pub first_air_date: Option<NaiveDate>,
    #[serde(default)]
    pub origin_country: Vec<String>,
    #[serde(default)]
    pub genre_ids: Vec<u64>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    #[serde(default)]
    pub adult: bool,
    #[serde(default)]
    pub popularity: f64,
    #[serde(default)]
    pub vote_average: f64,
    #[serde(default)]
    pub vote_count: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PersonSummary {
    pub id: u64,
    pub name: String,
    #[serde(default)]
    pub original_name: String,
    #[serde(default)]
    pub gender: Gender,
    pub known_for_department: Option<String>,
    pub profile_path: Option<String>,
    #[serde(default)]
    pub adult: bool,
    #[serde(default)]
    pub popularity: f64,
    #[serde(default)]
    pub known_for: Vec<Media>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CollectionSummary {
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
    pub adult: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "media_type", rename_all = "lowercase")]
pub enum Media {
    Movie(MovieSummary),
    Tv(TvSummary),
    Person(PersonSummary),
    #[serde(other)]
    Unknown,
}
