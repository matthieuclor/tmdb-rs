use serde::{Deserialize, Serialize};

use crate::{Client, Error, client::NONE};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Configuration {
    pub images: ImagesConfiguration,
    #[serde(default)]
    pub change_keys: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImagesConfiguration {
    pub base_url: String,
    pub secure_base_url: String,
    #[serde(default)]
    pub backdrop_sizes: Vec<String>,
    #[serde(default)]
    pub logo_sizes: Vec<String>,
    #[serde(default)]
    pub poster_sizes: Vec<String>,
    #[serde(default)]
    pub profile_sizes: Vec<String>,
    #[serde(default)]
    pub still_sizes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CountryInfo {
    pub iso_3166_1: String,
    #[serde(default)]
    pub english_name: String,
    #[serde(default)]
    pub native_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LanguageInfo {
    pub iso_639_1: String,
    #[serde(default)]
    pub english_name: String,
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Department {
    pub department: String,
    #[serde(default)]
    pub jobs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Timezone {
    pub iso_3166_1: String,
    #[serde(default)]
    pub zones: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WatchRegions {
    #[serde(default)]
    pub results: Vec<CountryInfo>,
}

#[derive(Debug, Clone, Copy)]
pub struct ConfigurationApi<'a> {
    client: &'a Client,
}

impl Client {
    pub fn configuration(&self) -> ConfigurationApi<'_> {
        ConfigurationApi { client: self }
    }
}

impl ConfigurationApi<'_> {
    pub async fn details(&self) -> Result<Configuration, Error> {
        self.client.get("/configuration", NONE).await
    }

    pub async fn countries(&self) -> Result<Vec<CountryInfo>, Error> {
        self.client.get("/configuration/countries", NONE).await
    }

    pub async fn jobs(&self) -> Result<Vec<Department>, Error> {
        self.client.get("/configuration/jobs", NONE).await
    }

    pub async fn languages(&self) -> Result<Vec<LanguageInfo>, Error> {
        self.client.get("/configuration/languages", NONE).await
    }

    pub async fn primary_translations(&self) -> Result<Vec<String>, Error> {
        self.client
            .get("/configuration/primary_translations", NONE)
            .await
    }

    pub async fn timezones(&self) -> Result<Vec<Timezone>, Error> {
        self.client.get("/configuration/timezones", NONE).await
    }

    pub async fn watch_regions(&self) -> Result<WatchRegions, Error> {
        self.client.get("/watch/providers/regions", NONE).await
    }
}
