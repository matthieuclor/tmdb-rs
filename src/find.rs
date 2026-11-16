use serde::{Deserialize, Serialize};

use crate::{Client, EpisodeSummary, Error, MovieSummary, PersonSummary, SeasonSummary, TvSummary};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ExternalSource {
    ImdbId,
    FacebookId,
    InstagramId,
    TiktokId,
    TvdbId,
    TwitterId,
    WikidataId,
    YoutubeId,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FindResults {
    #[serde(default)]
    pub movie_results: Vec<MovieSummary>,
    #[serde(default)]
    pub person_results: Vec<PersonSummary>,
    #[serde(default)]
    pub tv_results: Vec<TvSummary>,
    #[serde(default)]
    pub tv_season_results: Vec<SeasonSummary>,
    #[serde(default)]
    pub tv_episode_results: Vec<EpisodeSummary>,
}

#[derive(Debug, Clone, Copy)]
pub struct ExternalIdApi<'a> {
    client: &'a Client,
    source: ExternalSource,
    id: &'a str,
}

impl Client {
    pub fn external_id<'a>(&'a self, source: ExternalSource, id: &'a str) -> ExternalIdApi<'a> {
        ExternalIdApi {
            client: self,
            source,
            id,
        }
    }
}

impl ExternalIdApi<'_> {
    pub async fn find(&self) -> Result<FindResults, Error> {
        self.client
            .get(
                &format!("/find/{}", self.id),
                &[("external_source", self.source)],
            )
            .await
    }
}
