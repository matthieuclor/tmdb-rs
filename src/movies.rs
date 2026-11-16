use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::{
    AlternativeTitle, Certifications, ChangedId, Changes, ChangesQuery, Client, CollectionSummary,
    Company, Country, Credits, Error, ExternalIds, Genre, Genres, Images, Keyword, MovieSummary,
    Page, Review, SpokenLanguage, Translations, UserList, Videos, WatchProviderList,
    WatchProviders, client::NONE, serde_helpers::comma_list,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Movie {
    pub id: u64,
    pub title: String,
    pub imdb_id: Option<String>,
    #[serde(default)]
    pub original_title: String,
    #[serde(default)]
    pub original_language: String,
    #[serde(default)]
    pub overview: String,
    pub tagline: Option<String>,
    #[serde(default)]
    pub status: String,
    #[serde(default, deserialize_with = "crate::serde_helpers::date")]
    pub release_date: Option<NaiveDate>,
    pub runtime: Option<u32>,
    #[serde(default)]
    pub budget: u64,
    #[serde(default)]
    pub revenue: u64,
    pub homepage: Option<String>,
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
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub belongs_to_collection: Option<CollectionSummary>,
    #[serde(default)]
    pub genres: Vec<Genre>,
    #[serde(default)]
    pub origin_country: Vec<String>,
    #[serde(default)]
    pub production_companies: Vec<Company>,
    #[serde(default)]
    pub production_countries: Vec<Country>,
    #[serde(default)]
    pub spoken_languages: Vec<SpokenLanguage>,
    pub alternative_titles: Option<MovieAlternativeTitles>,
    pub credits: Option<Credits>,
    pub external_ids: Option<ExternalIds>,
    pub images: Option<Images>,
    pub keywords: Option<MovieKeywords>,
    pub lists: Option<Page<UserList>>,
    pub recommendations: Option<Page<MovieSummary>>,
    pub release_dates: Option<ReleaseDates>,
    pub reviews: Option<Page<Review>>,
    pub similar: Option<Page<MovieSummary>>,
    pub translations: Option<Translations<MovieTranslation>>,
    pub videos: Option<Videos>,
    #[serde(rename = "watch/providers")]
    pub watch_providers: Option<WatchProviders>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MovieAppend {
    AlternativeTitles,
    Credits,
    ExternalIds,
    Images,
    Keywords,
    Lists,
    Recommendations,
    ReleaseDates,
    Reviews,
    Similar,
    Translations,
    Videos,
    #[serde(rename = "watch/providers")]
    WatchProviders,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MovieAlternativeTitles {
    #[serde(default)]
    pub id: u64,
    #[serde(default)]
    pub titles: Vec<AlternativeTitle>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MovieKeywords {
    #[serde(default)]
    pub id: u64,
    #[serde(default)]
    pub keywords: Vec<Keyword>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MovieTranslation {
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub overview: String,
    #[serde(default)]
    pub tagline: String,
    #[serde(default)]
    pub homepage: String,
    #[serde(default)]
    pub runtime: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReleaseDate {
    #[serde(rename = "type")]
    pub kind: u8,
    pub release_date: String,
    #[serde(default)]
    pub certification: String,
    #[serde(default)]
    pub descriptors: Vec<String>,
    #[serde(default)]
    pub iso_639_1: String,
    #[serde(default)]
    pub note: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CountryReleaseDates {
    pub iso_3166_1: String,
    #[serde(default)]
    pub release_dates: Vec<ReleaseDate>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReleaseDates {
    #[serde(default)]
    pub id: u64,
    #[serde(default)]
    pub results: Vec<CountryReleaseDates>,
}

#[derive(Debug, Clone, Copy)]
pub struct MovieApi<'a> {
    client: &'a Client,
    id: u64,
}

#[derive(Debug, Clone, Copy)]
pub struct MoviesApi<'a> {
    client: &'a Client,
}

impl Client {
    pub fn movie(&self, id: u64) -> MovieApi<'_> {
        MovieApi { client: self, id }
    }

    pub fn movies(&self) -> MoviesApi<'_> {
        MoviesApi { client: self }
    }
}

impl MovieApi<'_> {
    pub async fn details(&self) -> Result<Movie, Error> {
        self.client.get(&self.path(""), NONE).await
    }

    pub async fn details_with(&self, append: &[MovieAppend]) -> Result<Movie, Error> {
        self.client
            .get(
                &self.path(""),
                &[("append_to_response", comma_list(append))],
            )
            .await
    }

    pub async fn alternative_titles(&self) -> Result<MovieAlternativeTitles, Error> {
        self.client
            .get(&self.path("/alternative_titles"), NONE)
            .await
    }

    pub async fn changes(&self, query: &ChangesQuery) -> Result<Changes, Error> {
        self.client.get(&self.path("/changes"), query).await
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

    pub async fn keywords(&self) -> Result<MovieKeywords, Error> {
        self.client.get(&self.path("/keywords"), NONE).await
    }

    pub async fn lists(&self, page: u32) -> Result<Page<UserList>, Error> {
        self.client
            .get(&self.path("/lists"), &[("page", page)])
            .await
    }

    pub async fn recommendations(&self, page: u32) -> Result<Page<MovieSummary>, Error> {
        self.client
            .get(&self.path("/recommendations"), &[("page", page)])
            .await
    }

    pub async fn release_dates(&self) -> Result<ReleaseDates, Error> {
        self.client.get(&self.path("/release_dates"), NONE).await
    }

    pub async fn reviews(&self, page: u32) -> Result<Page<Review>, Error> {
        self.client
            .get(&self.path("/reviews"), &[("page", page)])
            .await
    }

    pub async fn similar(&self, page: u32) -> Result<Page<MovieSummary>, Error> {
        self.client
            .get(&self.path("/similar"), &[("page", page)])
            .await
    }

    pub async fn translations(&self) -> Result<Translations<MovieTranslation>, Error> {
        self.client.get(&self.path("/translations"), NONE).await
    }

    pub async fn videos(&self) -> Result<Videos, Error> {
        self.client.get(&self.path("/videos"), NONE).await
    }

    pub async fn watch_providers(&self) -> Result<WatchProviders, Error> {
        self.client.get(&self.path("/watch/providers"), NONE).await
    }

    fn path(&self, suffix: &str) -> String {
        format!("/movie/{}{suffix}", self.id)
    }
}

impl MoviesApi<'_> {
    pub async fn latest(&self) -> Result<Movie, Error> {
        self.client.get("/movie/latest", NONE).await
    }

    pub async fn now_playing(&self, page: u32) -> Result<Page<MovieSummary>, Error> {
        self.client
            .get("/movie/now_playing", &[("page", page)])
            .await
    }

    pub async fn popular(&self, page: u32) -> Result<Page<MovieSummary>, Error> {
        self.client.get("/movie/popular", &[("page", page)]).await
    }

    pub async fn top_rated(&self, page: u32) -> Result<Page<MovieSummary>, Error> {
        self.client.get("/movie/top_rated", &[("page", page)]).await
    }

    pub async fn upcoming(&self, page: u32) -> Result<Page<MovieSummary>, Error> {
        self.client.get("/movie/upcoming", &[("page", page)]).await
    }

    pub async fn changes(&self, query: &ChangesQuery) -> Result<Page<ChangedId>, Error> {
        self.client.get("/movie/changes", query).await
    }

    pub async fn certifications(&self) -> Result<Certifications, Error> {
        self.client.get("/certification/movie/list", NONE).await
    }

    pub async fn genres(&self) -> Result<Genres, Error> {
        self.client.get("/genre/movie/list", NONE).await
    }

    pub async fn providers(&self, watch_region: Option<&str>) -> Result<WatchProviderList, Error> {
        self.client
            .get("/watch/providers/movie", &[("watch_region", watch_region)])
            .await
    }
}
