use crate::{Client, Error, Review, client::NONE};

#[derive(Debug, Clone, Copy)]
pub struct ReviewApi<'a> {
    client: &'a Client,
    id: &'a str,
}

impl Client {
    pub fn review<'a>(&'a self, id: &'a str) -> ReviewApi<'a> {
        ReviewApi { client: self, id }
    }
}

impl ReviewApi<'_> {
    pub async fn details(&self) -> Result<Review, Error> {
        self.client.get(&format!("/review/{}", self.id), NONE).await
    }
}
