use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::{
    AggregateCredits, AlternativeTitle, Certifications, ChangedId, Changes, ChangesQuery, Client,
    Company, Country, Credits, EpisodeGroup, Error, ExternalIds, Gender, Genre, Genres, Images,
    Keyword, Page, Review, SeasonApi, SpokenLanguage, Translations, TvSummary, UserList, Videos,
    WatchProviderList, WatchProviders, client::NONE, serde_helpers::comma_list,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TvSeries {
    pub id: u64,
    pub name: String,
    #[serde(default)]
    pub original_name: String,
    #[serde(default)]
    pub original_language: String,
    #[serde(default)]
    pub overview: String,
    pub tagline: Option<String>,
    #[serde(default)]
    pub status: String,
    #[serde(rename = "type", default)]
    pub kind: String,
    #[serde(default, deserialize_with = "crate::serde_helpers::date")]
    pub first_air_date: Option<NaiveDate>,
    #[serde(default, deserialize_with = "crate::serde_helpers::date")]
    pub last_air_date: Option<NaiveDate>,
    #[serde(default)]
    pub in_production: bool,
    #[serde(default)]
    pub number_of_episodes: u32,
    #[serde(default)]
    pub number_of_seasons: u32,
    #[serde(default)]
    pub episode_run_time: Vec<u32>,
    pub homepage: Option<String>,
    #[serde(default)]
    pub adult: bool,
    #[serde(default)]
    pub popularity: f64,
    #[serde(default)]
    pub vote_average: f64,
    #[serde(default)]
    pub vote_count: u32,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    #[serde(default)]
    pub created_by: Vec<Creator>,
    #[serde(default)]
    pub genres: Vec<Genre>,
    #[serde(default)]
    pub languages: Vec<String>,
    #[serde(default)]
    pub origin_country: Vec<String>,
    #[serde(default)]
    pub networks: Vec<Company>,
    #[serde(default)]
    pub production_companies: Vec<Company>,
    #[serde(default)]
    pub production_countries: Vec<Country>,
    #[serde(default)]
    pub spoken_languages: Vec<SpokenLanguage>,
    #[serde(default)]
    pub seasons: Vec<SeasonSummary>,
    pub last_episode_to_air: Option<EpisodeSummary>,
    pub next_episode_to_air: Option<EpisodeSummary>,
    pub aggregate_credits: Option<AggregateCredits>,
    pub alternative_titles: Option<TvAlternativeTitles>,
    pub content_ratings: Option<ContentRatings>,
    pub credits: Option<Credits>,
    pub episode_groups: Option<EpisodeGroups>,
    pub external_ids: Option<ExternalIds>,
    pub images: Option<Images>,
    pub keywords: Option<TvKeywords>,
    pub lists: Option<Page<UserList>>,
    pub recommendations: Option<Page<TvSummary>>,
    pub reviews: Option<Page<Review>>,
    pub screened_theatrically: Option<ScreenedTheatrically>,
    pub similar: Option<Page<TvSummary>>,
    pub translations: Option<Translations<TvTranslation>>,
    pub videos: Option<Videos>,
    #[serde(rename = "watch/providers")]
    pub watch_providers: Option<WatchProviders>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TvAppend {
    AggregateCredits,
    AlternativeTitles,
    ContentRatings,
    Credits,
    EpisodeGroups,
    ExternalIds,
    Images,
    Keywords,
    Lists,
    Recommendations,
    Reviews,
    ScreenedTheatrically,
    Similar,
    Translations,
    Videos,
    #[serde(rename = "watch/providers")]
    WatchProviders,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Creator {
    pub id: u64,
    pub name: String,
    #[serde(default)]
    pub credit_id: String,
    #[serde(default)]
    pub gender: Gender,
    pub profile_path: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SeasonSummary {
    pub id: u64,
    pub season_number: u32,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub overview: String,
    #[serde(default)]
    pub episode_count: u32,
    #[serde(default, deserialize_with = "crate::serde_helpers::date")]
    pub air_date: Option<NaiveDate>,
    pub poster_path: Option<String>,
    #[serde(default)]
    pub vote_average: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show_id: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EpisodeSummary {
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TvTranslation {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub overview: String,
    #[serde(default)]
    pub tagline: String,
    #[serde(default)]
    pub homepage: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TvAlternativeTitles {
    #[serde(default)]
    pub id: u64,
    #[serde(default)]
    pub results: Vec<AlternativeTitle>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TvKeywords {
    #[serde(default)]
    pub id: u64,
    #[serde(default)]
    pub results: Vec<Keyword>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContentRating {
    pub iso_3166_1: String,
    pub rating: String,
    #[serde(default)]
    pub descriptors: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContentRatings {
    #[serde(default)]
    pub id: u64,
    #[serde(default)]
    pub results: Vec<ContentRating>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EpisodeGroupSummary {
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
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EpisodeGroups {
    #[serde(default)]
    pub id: u64,
    #[serde(default)]
    pub results: Vec<EpisodeGroupSummary>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScreenedEpisode {
    pub id: u64,
    pub season_number: u32,
    pub episode_number: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScreenedTheatrically {
    #[serde(default)]
    pub id: u64,
    #[serde(default)]
    pub results: Vec<ScreenedEpisode>,
}

#[derive(Debug, Clone, Copy)]
pub struct TvApi<'a> {
    client: &'a Client,
    id: u64,
}

#[derive(Debug, Clone, Copy)]
pub struct TvShowsApi<'a> {
    client: &'a Client,
}

impl Client {
    pub fn tv(&self, id: u64) -> TvApi<'_> {
        TvApi { client: self, id }
    }

    pub fn tv_shows(&self) -> TvShowsApi<'_> {
        TvShowsApi { client: self }
    }
}

impl<'a> TvApi<'a> {
    pub fn season(&self, number: u32) -> SeasonApi<'a> {
        SeasonApi::new(self.client, self.id, number)
    }

    pub async fn details(&self) -> Result<TvSeries, Error> {
        self.client.get(&self.path(""), NONE).await
    }

    pub async fn details_with(&self, append: &[TvAppend]) -> Result<TvSeries, Error> {
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

    pub async fn alternative_titles(&self) -> Result<TvAlternativeTitles, Error> {
        self.client
            .get(&self.path("/alternative_titles"), NONE)
            .await
    }

    pub async fn changes(&self, query: &ChangesQuery) -> Result<Changes, Error> {
        self.client.get(&self.path("/changes"), query).await
    }

    pub async fn content_ratings(&self) -> Result<ContentRatings, Error> {
        self.client.get(&self.path("/content_ratings"), NONE).await
    }

    pub async fn credits(&self) -> Result<Credits, Error> {
        self.client.get(&self.path("/credits"), NONE).await
    }

    pub async fn episode_groups(&self) -> Result<EpisodeGroups, Error> {
        self.client.get(&self.path("/episode_groups"), NONE).await
    }

    pub async fn external_ids(&self) -> Result<ExternalIds, Error> {
        self.client.get(&self.path("/external_ids"), NONE).await
    }

    pub async fn images(&self) -> Result<Images, Error> {
        self.client.get_images(&self.path("/images")).await
    }

    pub async fn keywords(&self) -> Result<TvKeywords, Error> {
        self.client.get(&self.path("/keywords"), NONE).await
    }

    pub async fn lists(&self, page: u32) -> Result<Page<UserList>, Error> {
        self.client
            .get(&self.path("/lists"), &[("page", page)])
            .await
    }

    pub async fn recommendations(&self, page: u32) -> Result<Page<TvSummary>, Error> {
        self.client
            .get(&self.path("/recommendations"), &[("page", page)])
            .await
    }

    pub async fn reviews(&self, page: u32) -> Result<Page<Review>, Error> {
        self.client
            .get(&self.path("/reviews"), &[("page", page)])
            .await
    }

    pub async fn screened_theatrically(&self) -> Result<ScreenedTheatrically, Error> {
        self.client
            .get(&self.path("/screened_theatrically"), NONE)
            .await
    }

    pub async fn similar(&self, page: u32) -> Result<Page<TvSummary>, Error> {
        self.client
            .get(&self.path("/similar"), &[("page", page)])
            .await
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
        format!("/tv/{}{suffix}", self.id)
    }
}

impl TvShowsApi<'_> {
    pub async fn latest(&self) -> Result<TvSeries, Error> {
        self.client.get("/tv/latest", NONE).await
    }

    pub async fn airing_today(&self, page: u32) -> Result<Page<TvSummary>, Error> {
        self.client.get("/tv/airing_today", &[("page", page)]).await
    }

    pub async fn on_the_air(&self, page: u32) -> Result<Page<TvSummary>, Error> {
        self.client.get("/tv/on_the_air", &[("page", page)]).await
    }

    pub async fn popular(&self, page: u32) -> Result<Page<TvSummary>, Error> {
        self.client.get("/tv/popular", &[("page", page)]).await
    }

    pub async fn top_rated(&self, page: u32) -> Result<Page<TvSummary>, Error> {
        self.client.get("/tv/top_rated", &[("page", page)]).await
    }

    pub async fn changes(&self, query: &ChangesQuery) -> Result<Page<ChangedId>, Error> {
        self.client.get("/tv/changes", query).await
    }

    pub async fn season_changes(
        &self,
        season_id: u64,
        query: &ChangesQuery,
    ) -> Result<Changes, Error> {
        self.client
            .get(&format!("/tv/season/{season_id}/changes"), query)
            .await
    }

    pub async fn episode_changes(&self, episode_id: u64) -> Result<Changes, Error> {
        self.client
            .get(&format!("/tv/episode/{episode_id}/changes"), NONE)
            .await
    }

    pub async fn episode_group(&self, id: &str) -> Result<EpisodeGroup, Error> {
        self.client
            .get(&format!("/tv/episode_group/{id}"), NONE)
            .await
    }

    pub async fn certifications(&self) -> Result<Certifications, Error> {
        self.client.get("/certification/tv/list", NONE).await
    }

    pub async fn genres(&self) -> Result<Genres, Error> {
        self.client.get("/genre/tv/list", NONE).await
    }

    pub async fn providers(&self, watch_region: Option<&str>) -> Result<WatchProviderList, Error> {
        self.client
            .get("/watch/providers/tv", &[("watch_region", watch_region)])
            .await
    }
}
