use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::{
    AggregateCredits, Client, Company, Credits, Episode, EpisodeApi, Error, ExternalIds, Images,
    Translations, TvTranslation, Videos, WatchProviders, client::NONE, serde_helpers::comma_list,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Season {
    pub id: u64,
    pub season_number: u32,
    #[serde(rename = "_id", default)]
    pub internal_id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub overview: String,
    #[serde(default, deserialize_with = "crate::serde_helpers::date")]
    pub air_date: Option<NaiveDate>,
    pub poster_path: Option<String>,
    #[serde(default)]
    pub vote_average: f64,
    #[serde(default)]
    pub episodes: Vec<Episode>,
    #[serde(default)]
    pub networks: Vec<Company>,
    pub aggregate_credits: Option<AggregateCredits>,
    pub credits: Option<Credits>,
    pub external_ids: Option<ExternalIds>,
    pub images: Option<Images>,
    pub translations: Option<Translations<TvTranslation>>,
    pub videos: Option<Videos>,
    #[serde(rename = "watch/providers")]
    pub watch_providers: Option<WatchProviders>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SeasonAppend {
    AggregateCredits,
    Credits,
    ExternalIds,
    Images,
    Translations,
    Videos,
    #[serde(rename = "watch/providers")]
    WatchProviders,
}

#[derive(Debug, Clone, Copy)]
pub struct SeasonApi<'a> {
    client: &'a Client,
    series_id: u64,
    number: u32,
}

impl<'a> SeasonApi<'a> {
    pub(crate) fn new(client: &'a Client, series_id: u64, number: u32) -> Self {
        Self {
            client,
            series_id,
            number,
        }
    }

    pub fn episode(&self, number: u32) -> EpisodeApi<'a> {
        EpisodeApi::new(self.client, self.series_id, self.number, number)
    }

    pub async fn details(&self) -> Result<Season, Error> {
        self.client.get(&self.path(""), NONE).await
    }

    pub async fn details_with(&self, append: &[SeasonAppend]) -> Result<Season, Error> {
        self.client
            .get(
                &self.path(""),
                &[("append_to_response", comma_list(append))],
            )
            .await
    }

    pub async fn aggregate_credits(&self) -> Result<AggregateCredits, Error> {
        self.client
            .get(&self.path("/aggregate_credits"), NONE)
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

    pub async fn watch_providers(&self) -> Result<WatchProviders, Error> {
        self.client.get(&self.path("/watch/providers"), NONE).await
    }

    fn path(&self, suffix: &str) -> String {
        format!("/tv/{}/season/{}{suffix}", self.series_id, self.number)
    }
}
