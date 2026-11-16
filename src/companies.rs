use serde::{Deserialize, Serialize};

use crate::{AlternativeNames, Client, Company, Error, Logos, client::NONE};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompanyDetails {
    pub id: u64,
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub headquarters: String,
    #[serde(default)]
    pub homepage: String,
    pub logo_path: Option<String>,
    #[serde(default)]
    pub origin_country: String,
    pub parent_company: Option<Company>,
}

#[derive(Debug, Clone, Copy)]
pub struct CompanyApi<'a> {
    client: &'a Client,
    id: u64,
}

impl Client {
    pub fn company(&self, id: u64) -> CompanyApi<'_> {
        CompanyApi { client: self, id }
    }
}

impl CompanyApi<'_> {
    pub async fn details(&self) -> Result<CompanyDetails, Error> {
        self.client.get(&self.path(""), NONE).await
    }

    pub async fn alternative_names(&self) -> Result<AlternativeNames, Error> {
        self.client
            .get(&self.path("/alternative_names"), NONE)
            .await
    }

    pub async fn images(&self) -> Result<Logos, Error> {
        self.client.get(&self.path("/images"), NONE).await
    }

    fn path(&self, suffix: &str) -> String {
        format!("/company/{}{suffix}", self.id)
    }
}
