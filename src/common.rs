use std::collections::HashMap;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Page<T> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dates: Option<DateRange>,
    #[serde(default)]
    pub page: u32,
    #[serde(default = "Vec::new")]
    pub results: Vec<T>,
    #[serde(default)]
    pub total_pages: u32,
    #[serde(default)]
    pub total_results: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DateRange {
    #[serde(default, deserialize_with = "crate::serde_helpers::date")]
    pub maximum: Option<NaiveDate>,
    #[serde(default, deserialize_with = "crate::serde_helpers::date")]
    pub minimum: Option<NaiveDate>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(from = "u8", into = "u8")]
#[repr(u8)]
pub enum Gender {
    #[default]
    NotSpecified = 0,
    Female = 1,
    Male = 2,
    NonBinary = 3,
}

impl From<u8> for Gender {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::Female,
            2 => Self::Male,
            3 => Self::NonBinary,
            _ => Self::NotSpecified,
        }
    }
}

impl From<Gender> for u8 {
    fn from(gender: Gender) -> Self {
        gender as u8
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Genre {
    pub id: u64,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Keyword {
    pub id: u64,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Company {
    pub id: u64,
    pub name: String,
    pub logo_path: Option<String>,
    #[serde(default)]
    pub origin_country: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Country {
    pub iso_3166_1: String,
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpokenLanguage {
    pub iso_639_1: String,
    #[serde(default)]
    pub english_name: String,
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Image {
    pub file_path: String,
    pub id: Option<String>,
    pub file_type: Option<String>,
    pub iso_639_1: Option<String>,
    #[serde(default)]
    pub aspect_ratio: f64,
    #[serde(default)]
    pub height: u32,
    #[serde(default)]
    pub width: u32,
    #[serde(default)]
    pub vote_average: f64,
    #[serde(default)]
    pub vote_count: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Images {
    #[serde(default)]
    pub id: u64,
    #[serde(default)]
    pub backdrops: Vec<Image>,
    #[serde(default)]
    pub logos: Vec<Image>,
    #[serde(default)]
    pub posters: Vec<Image>,
    #[serde(default)]
    pub profiles: Vec<Image>,
    #[serde(default)]
    pub stills: Vec<Image>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Logos {
    pub id: u64,
    #[serde(default)]
    pub logos: Vec<Image>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Video {
    pub id: String,
    pub key: String,
    pub site: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "type", default)]
    pub kind: String,
    #[serde(default)]
    pub official: bool,
    #[serde(default)]
    pub size: u32,
    #[serde(default)]
    pub iso_639_1: String,
    #[serde(default)]
    pub iso_3166_1: String,
    #[serde(default)]
    pub published_at: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Videos {
    #[serde(default)]
    pub id: u64,
    #[serde(default)]
    pub results: Vec<Video>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AlternativeTitle {
    pub title: String,
    #[serde(default)]
    pub iso_3166_1: String,
    #[serde(rename = "type", default)]
    pub kind: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AlternativeName {
    pub name: String,
    #[serde(rename = "type", default)]
    pub kind: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AlternativeNames {
    pub id: u64,
    #[serde(default)]
    pub results: Vec<AlternativeName>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Translation<T> {
    pub iso_639_1: String,
    #[serde(default)]
    pub iso_3166_1: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub english_name: String,
    pub data: T,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Translations<T> {
    #[serde(default)]
    pub id: u64,
    #[serde(default = "Vec::new")]
    pub translations: Vec<Translation<T>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExternalIds {
    #[serde(default)]
    pub id: u64,
    pub imdb_id: Option<String>,
    pub freebase_mid: Option<String>,
    pub freebase_id: Option<String>,
    pub tvdb_id: Option<u64>,
    pub tvrage_id: Option<u64>,
    pub wikidata_id: Option<String>,
    pub facebook_id: Option<String>,
    pub instagram_id: Option<String>,
    pub tiktok_id: Option<String>,
    pub twitter_id: Option<String>,
    pub youtube_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cast {
    pub id: u64,
    pub name: String,
    #[serde(default)]
    pub original_name: String,
    #[serde(default)]
    pub character: String,
    #[serde(default)]
    pub credit_id: String,
    #[serde(default)]
    pub order: u32,
    pub cast_id: Option<u64>,
    #[serde(default)]
    pub gender: Gender,
    pub known_for_department: Option<String>,
    pub profile_path: Option<String>,
    #[serde(default)]
    pub adult: bool,
    #[serde(default)]
    pub popularity: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Crew {
    pub id: u64,
    pub name: String,
    #[serde(default)]
    pub original_name: String,
    #[serde(default)]
    pub department: String,
    #[serde(default)]
    pub job: String,
    #[serde(default)]
    pub credit_id: String,
    #[serde(default)]
    pub gender: Gender,
    pub known_for_department: Option<String>,
    pub profile_path: Option<String>,
    #[serde(default)]
    pub adult: bool,
    #[serde(default)]
    pub popularity: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Credits {
    #[serde(default)]
    pub id: u64,
    #[serde(default)]
    pub cast: Vec<Cast>,
    #[serde(default)]
    pub crew: Vec<Crew>,
    #[serde(default)]
    pub guest_stars: Vec<Cast>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Role {
    pub credit_id: String,
    #[serde(default)]
    pub character: String,
    #[serde(default)]
    pub episode_count: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Job {
    pub credit_id: String,
    #[serde(default)]
    pub job: String,
    #[serde(default)]
    pub episode_count: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AggregateCast {
    pub id: u64,
    pub name: String,
    #[serde(default)]
    pub original_name: String,
    #[serde(default)]
    pub roles: Vec<Role>,
    #[serde(default)]
    pub total_episode_count: u32,
    #[serde(default)]
    pub order: u32,
    #[serde(default)]
    pub gender: Gender,
    pub known_for_department: Option<String>,
    pub profile_path: Option<String>,
    #[serde(default)]
    pub adult: bool,
    #[serde(default)]
    pub popularity: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AggregateCrew {
    pub id: u64,
    pub name: String,
    #[serde(default)]
    pub original_name: String,
    #[serde(default)]
    pub department: String,
    #[serde(default)]
    pub jobs: Vec<Job>,
    #[serde(default)]
    pub total_episode_count: u32,
    #[serde(default)]
    pub gender: Gender,
    pub known_for_department: Option<String>,
    pub profile_path: Option<String>,
    #[serde(default)]
    pub adult: bool,
    #[serde(default)]
    pub popularity: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AggregateCredits {
    #[serde(default)]
    pub id: u64,
    #[serde(default)]
    pub cast: Vec<AggregateCast>,
    #[serde(default)]
    pub crew: Vec<AggregateCrew>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AuthorDetails {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub username: String,
    pub avatar_path: Option<String>,
    pub rating: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Review {
    pub id: String,
    #[serde(default)]
    pub author: String,
    #[serde(default)]
    pub author_details: AuthorDetails,
    #[serde(default)]
    pub content: String,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub updated_at: String,
    #[serde(default)]
    pub url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iso_639_1: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub media_id: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub media_title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WatchProvider {
    pub provider_id: u64,
    pub provider_name: String,
    pub logo_path: Option<String>,
    #[serde(default)]
    pub display_priority: i32,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub display_priorities: HashMap<String, i32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegionWatchProviders {
    #[serde(default)]
    pub link: String,
    #[serde(default)]
    pub flatrate: Vec<WatchProvider>,
    #[serde(default)]
    pub rent: Vec<WatchProvider>,
    #[serde(default)]
    pub buy: Vec<WatchProvider>,
    #[serde(default)]
    pub free: Vec<WatchProvider>,
    #[serde(default)]
    pub ads: Vec<WatchProvider>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WatchProviders {
    #[serde(default)]
    pub id: u64,
    #[serde(default)]
    pub results: HashMap<String, RegionWatchProviders>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChangeItem {
    pub id: String,
    #[serde(default)]
    pub action: String,
    #[serde(default)]
    pub time: String,
    pub iso_639_1: Option<String>,
    pub iso_3166_1: Option<String>,
    pub value: Option<Value>,
    pub original_value: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Change {
    pub key: String,
    #[serde(default)]
    pub items: Vec<ChangeItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Changes {
    #[serde(default)]
    pub changes: Vec<Change>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserList {
    pub id: u64,
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub favorite_count: u32,
    #[serde(default)]
    pub item_count: u32,
    #[serde(default)]
    pub iso_639_1: String,
    pub iso_3166_1: Option<String>,
    pub list_type: Option<String>,
    pub poster_path: Option<String>,
}
