use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::{
    ChangedId, Changes, ChangesQuery, Client, Error, ExternalIds, Gender, Image, Images, Media,
    MovieSummary, Page, PersonSummary, Translations, TvSummary, client::NONE,
    serde_helpers::comma_list,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Person {
    pub id: u64,
    pub name: String,
    pub imdb_id: Option<String>,
    #[serde(default)]
    pub also_known_as: Vec<String>,
    #[serde(default)]
    pub biography: String,
    #[serde(default, deserialize_with = "crate::serde_helpers::date")]
    pub birthday: Option<NaiveDate>,
    #[serde(default, deserialize_with = "crate::serde_helpers::date")]
    pub deathday: Option<NaiveDate>,
    pub place_of_birth: Option<String>,
    #[serde(default)]
    pub gender: Gender,
    pub known_for_department: Option<String>,
    pub homepage: Option<String>,
    pub profile_path: Option<String>,
    #[serde(default)]
    pub adult: bool,
    #[serde(default)]
    pub popularity: f64,
    pub combined_credits: Option<PersonCredits<Media>>,
    pub movie_credits: Option<PersonCredits<MovieSummary>>,
    pub tv_credits: Option<PersonCredits<TvSummary>>,
    pub external_ids: Option<ExternalIds>,
    pub images: Option<Images>,
    pub tagged_images: Option<Page<TaggedImage>>,
    pub translations: Option<Translations<PersonTranslation>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PersonAppend {
    CombinedCredits,
    MovieCredits,
    TvCredits,
    ExternalIds,
    Images,
    TaggedImages,
    Translations,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PersonCredit<M> {
    #[serde(flatten)]
    pub media: M,
    pub credit_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub character: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub episode_count: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PersonCredits<M> {
    #[serde(default)]
    pub id: u64,
    #[serde(default = "Vec::new")]
    pub cast: Vec<PersonCredit<M>>,
    #[serde(default = "Vec::new")]
    pub crew: Vec<PersonCredit<M>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaggedImage {
    #[serde(flatten)]
    pub image: Image,
    #[serde(default)]
    pub image_type: String,
    #[serde(default)]
    pub media_type: String,
    pub media: Media,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PersonTranslation {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub biography: String,
}

#[derive(Debug, Clone, Copy)]
pub struct PersonApi<'a> {
    client: &'a Client,
    id: u64,
}

#[derive(Debug, Clone, Copy)]
pub struct PeopleApi<'a> {
    client: &'a Client,
}

impl Client {
    pub fn person(&self, id: u64) -> PersonApi<'_> {
        PersonApi { client: self, id }
    }

    pub fn people(&self) -> PeopleApi<'_> {
        PeopleApi { client: self }
    }
}

impl PersonApi<'_> {
    pub async fn details(&self) -> Result<Person, Error> {
        self.client.get(&self.path(""), NONE).await
    }

    pub async fn details_with(&self, append: &[PersonAppend]) -> Result<Person, Error> {
        self.client
            .get(
                &self.path(""),
                &[("append_to_response", comma_list(append))],
            )
            .await
    }

    pub async fn changes(&self, query: &ChangesQuery) -> Result<Changes, Error> {
        self.client.get(&self.path("/changes"), query).await
    }

    pub async fn combined_credits(&self) -> Result<PersonCredits<Media>, Error> {
        self.client.get(&self.path("/combined_credits"), NONE).await
    }

    pub async fn movie_credits(&self) -> Result<PersonCredits<MovieSummary>, Error> {
        self.client.get(&self.path("/movie_credits"), NONE).await
    }

    pub async fn tv_credits(&self) -> Result<PersonCredits<TvSummary>, Error> {
        self.client.get(&self.path("/tv_credits"), NONE).await
    }

    pub async fn external_ids(&self) -> Result<ExternalIds, Error> {
        self.client.get(&self.path("/external_ids"), NONE).await
    }

    pub async fn images(&self) -> Result<Images, Error> {
        self.client.get(&self.path("/images"), NONE).await
    }

    pub async fn tagged_images(&self, page: u32) -> Result<Page<TaggedImage>, Error> {
        self.client
            .get(&self.path("/tagged_images"), &[("page", page)])
            .await
    }

    pub async fn translations(&self) -> Result<Translations<PersonTranslation>, Error> {
        self.client.get(&self.path("/translations"), NONE).await
    }

    fn path(&self, suffix: &str) -> String {
        format!("/person/{}{suffix}", self.id)
    }
}

impl PeopleApi<'_> {
    pub async fn latest(&self) -> Result<Person, Error> {
        self.client.get("/person/latest", NONE).await
    }

    pub async fn popular(&self, page: u32) -> Result<Page<PersonSummary>, Error> {
        self.client.get("/person/popular", &[("page", page)]).await
    }

    pub async fn changes(&self, query: &ChangesQuery) -> Result<Page<ChangedId>, Error> {
        self.client.get("/person/changes", query).await
    }
}
