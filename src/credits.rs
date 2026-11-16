use serde::{Deserialize, Serialize};

use crate::{Client, EpisodeSummary, Error, Media, PersonSummary, SeasonSummary, client::NONE};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreditMedia {
    #[serde(flatten)]
    pub media: Media,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub character: Option<String>,
    #[serde(default)]
    pub episodes: Vec<EpisodeSummary>,
    #[serde(default)]
    pub seasons: Vec<SeasonSummary>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreditDetails {
    pub id: String,
    pub media: CreditMedia,
    pub person: PersonSummary,
    #[serde(default)]
    pub credit_type: String,
    #[serde(default)]
    pub department: String,
    #[serde(default)]
    pub job: String,
    #[serde(default)]
    pub media_type: String,
}

#[derive(Debug, Clone, Copy)]
pub struct CreditApi<'a> {
    client: &'a Client,
    id: &'a str,
}

impl Client {
    pub fn credit<'a>(&'a self, id: &'a str) -> CreditApi<'a> {
        CreditApi { client: self, id }
    }
}

impl CreditApi<'_> {
    pub async fn details(&self) -> Result<CreditDetails, Error> {
        self.client.get(&format!("/credit/{}", self.id), NONE).await
    }
}
