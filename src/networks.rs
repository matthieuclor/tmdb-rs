use serde::{Deserialize, Serialize};

use crate::{AlternativeNames, Client, Error, Logos, client::NONE};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Network {
    pub id: u64,
    pub name: String,
    #[serde(default)]
    pub headquarters: String,
    #[serde(default)]
    pub homepage: String,
    pub logo_path: Option<String>,
    #[serde(default)]
    pub origin_country: String,
}

#[derive(Debug, Clone, Copy)]
pub struct NetworkApi<'a> {
    client: &'a Client,
    id: u64,
}

impl Client {
    pub fn network(&self, id: u64) -> NetworkApi<'_> {
        NetworkApi { client: self, id }
    }
}

impl NetworkApi<'_> {
    pub async fn details(&self) -> Result<Network, Error> {
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
        format!("/network/{}{suffix}", self.id)
    }
}
