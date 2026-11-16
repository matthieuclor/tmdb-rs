use crate::{Client, Error, Keyword, MovieSummary, Page, client::NONE};

#[derive(Debug, Clone, Copy)]
pub struct KeywordApi<'a> {
    client: &'a Client,
    id: u64,
}

impl Client {
    pub fn keyword(&self, id: u64) -> KeywordApi<'_> {
        KeywordApi { client: self, id }
    }
}

impl KeywordApi<'_> {
    pub async fn details(&self) -> Result<Keyword, Error> {
        self.client.get(&self.path(""), NONE).await
    }

    pub async fn movies(&self, page: u32) -> Result<Page<MovieSummary>, Error> {
        self.client
            .get(&self.path("/movies"), &[("page", page)])
            .await
    }

    fn path(&self, suffix: &str) -> String {
        format!("/keyword/{}{suffix}", self.id)
    }
}
