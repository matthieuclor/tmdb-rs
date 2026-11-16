use chrono::NaiveDate;
use serde::Serialize;

use crate::{Client, Error, MovieSummary, Page, TvSummary};

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct MovieDiscover {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_adult: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_video: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certification: Option<String>,
    #[serde(rename = "certification.gte", skip_serializing_if = "Option::is_none")]
    pub certification_gte: Option<String>,
    #[serde(rename = "certification.lte", skip_serializing_if = "Option::is_none")]
    pub certification_lte: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certification_country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_release_year: Option<u32>,
    #[serde(
        rename = "primary_release_date.gte",
        skip_serializing_if = "Option::is_none"
    )]
    pub primary_release_date_gte: Option<NaiveDate>,
    #[serde(
        rename = "primary_release_date.lte",
        skip_serializing_if = "Option::is_none"
    )]
    pub primary_release_date_lte: Option<NaiveDate>,
    #[serde(rename = "release_date.gte", skip_serializing_if = "Option::is_none")]
    pub release_date_gte: Option<NaiveDate>,
    #[serde(rename = "release_date.lte", skip_serializing_if = "Option::is_none")]
    pub release_date_lte: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_release_type: Option<String>,
    #[serde(rename = "vote_average.gte", skip_serializing_if = "Option::is_none")]
    pub vote_average_gte: Option<f64>,
    #[serde(rename = "vote_average.lte", skip_serializing_if = "Option::is_none")]
    pub vote_average_lte: Option<f64>,
    #[serde(rename = "vote_count.gte", skip_serializing_if = "Option::is_none")]
    pub vote_count_gte: Option<u32>,
    #[serde(rename = "vote_count.lte", skip_serializing_if = "Option::is_none")]
    pub vote_count_lte: Option<u32>,
    #[serde(rename = "with_runtime.gte", skip_serializing_if = "Option::is_none")]
    pub with_runtime_gte: Option<u32>,
    #[serde(rename = "with_runtime.lte", skip_serializing_if = "Option::is_none")]
    pub with_runtime_lte: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_cast: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_crew: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_people: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_companies: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub without_companies: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_genres: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub without_genres: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_keywords: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub without_keywords: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_origin_country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_original_language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watch_region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_watch_providers: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub without_watch_providers: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_watch_monetization_types: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct TvDiscover {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_adult: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_null_first_air_dates: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screened_theatrically: Option<bool>,
    #[serde(rename = "air_date.gte", skip_serializing_if = "Option::is_none")]
    pub air_date_gte: Option<NaiveDate>,
    #[serde(rename = "air_date.lte", skip_serializing_if = "Option::is_none")]
    pub air_date_lte: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_air_date_year: Option<u32>,
    #[serde(rename = "first_air_date.gte", skip_serializing_if = "Option::is_none")]
    pub first_air_date_gte: Option<NaiveDate>,
    #[serde(rename = "first_air_date.lte", skip_serializing_if = "Option::is_none")]
    pub first_air_date_lte: Option<NaiveDate>,
    #[serde(rename = "vote_average.gte", skip_serializing_if = "Option::is_none")]
    pub vote_average_gte: Option<f64>,
    #[serde(rename = "vote_average.lte", skip_serializing_if = "Option::is_none")]
    pub vote_average_lte: Option<f64>,
    #[serde(rename = "vote_count.gte", skip_serializing_if = "Option::is_none")]
    pub vote_count_gte: Option<u32>,
    #[serde(rename = "vote_count.lte", skip_serializing_if = "Option::is_none")]
    pub vote_count_lte: Option<u32>,
    #[serde(rename = "with_runtime.gte", skip_serializing_if = "Option::is_none")]
    pub with_runtime_gte: Option<u32>,
    #[serde(rename = "with_runtime.lte", skip_serializing_if = "Option::is_none")]
    pub with_runtime_lte: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_companies: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub without_companies: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_genres: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub without_genres: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_keywords: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub without_keywords: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_networks: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_origin_country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_original_language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watch_region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_watch_providers: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub without_watch_providers: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_watch_monetization_types: Option<String>,
}

#[derive(Debug, Clone, Copy)]
pub struct DiscoverApi<'a> {
    client: &'a Client,
}

impl Client {
    pub fn discover(&self) -> DiscoverApi<'_> {
        DiscoverApi { client: self }
    }
}

impl DiscoverApi<'_> {
    pub async fn movies(&self, query: &MovieDiscover) -> Result<Page<MovieSummary>, Error> {
        self.client.get("/discover/movie", query).await
    }

    pub async fn tv(&self, query: &TvDiscover) -> Result<Page<TvSummary>, Error> {
        self.client.get("/discover/tv", query).await
    }
}
