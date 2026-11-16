use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::{
    Cast, Client, Company, Credits, Crew, EpisodeSummary, Error, ExternalIds, Images, Translations,
    TvTranslation, Videos, client::NONE, serde_helpers::comma_list,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Episode {
    pub id: u64,
    pub season_number: u32,
    pub episode_number: u32,
    #[serde(default)]
    pub show_id: u64,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub overview: String,
    pub episode_type: Option<String>,
    #[serde(default, deserialize_with = "crate::serde_helpers::date")]
    pub air_date: Option<NaiveDate>,
    #[serde(default)]
    pub production_code: String,
    pub runtime: Option<u32>,
    pub still_path: Option<String>,
    #[serde(default)]
    pub vote_average: f64,
    #[serde(default)]
    pub vote_count: u32,
    #[serde(default)]
    pub crew: Vec<Crew>,
    #[serde(default)]
    pub guest_stars: Vec<Cast>,
    pub credits: Option<Credits>,
    pub external_ids: Option<ExternalIds>,
    pub images: Option<Images>,
    pub translations: Option<Translations<TvTranslation>>,
    pub videos: Option<Videos>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EpisodeAppend {
    Credits,
    ExternalIds,
    Images,
    Translations,
    Videos,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EpisodeGroupEntry {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub order: u32,
    #[serde(default)]
    pub locked: bool,
    #[serde(default)]
    pub episodes: Vec<EpisodeSummary>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EpisodeGroup {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub episode_count: u32,
    #[serde(default)]
    pub group_count: u32,
    #[serde(rename = "type", default)]
    pub kind: u8,
    pub network: Option<Company>,
    #[serde(default)]
    pub groups: Vec<EpisodeGroupEntry>,
}

#[derive(Debug, Clone, Copy)]
pub struct EpisodeApi<'a> {
    client: &'a Client,
    series_id: u64,
    season: u32,
    number: u32,
}

impl<'a> EpisodeApi<'a> {
    pub(crate) fn new(client: &'a Client, series_id: u64, season: u32, number: u32) -> Self {
        Self {
            client,
            series_id,
            season,
            number,
        }
    }

    pub async fn details(&self) -> Result<Episode, Error> {
        self.client.get(&self.path(""), NONE).await
    }

    pub async fn details_with(&self, append: &[EpisodeAppend]) -> Result<Episode, Error> {
        self.client
            .get(
                &self.path(""),
                &[("append_to_response", comma_list(append))],
            )
            .await
    }

    pub async fn credits(&self) -> Result<Credits, Error> {
        self.client.get(&self.path("/credits"), NONE).await
    }

    pub async fn external_ids(&self) -> Result<ExternalIds, Error> {
        self.client.get(&self.path("/external_ids"), NONE).await
    }

    pub async fn images(&self) -> Result<Images, Error> {
        self.client.get_images(&self.path("/images")).await
    }

    pub async fn translations(&self) -> Result<Translations<TvTranslation>, Error> {
        self.client.get(&self.path("/translations"), NONE).await
    }

    pub async fn videos(&self) -> Result<Videos, Error> {
        self.client.get(&self.path("/videos"), NONE).await
    }

    fn path(&self, suffix: &str) -> String {
        format!(
            "/tv/{}/season/{}/episode/{}{suffix}",
            self.series_id, self.season, self.number
        )
    }
}
