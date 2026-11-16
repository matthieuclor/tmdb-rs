use crate::{Client, Error, Media, MovieSummary, Page, PersonSummary, TvSummary, client::NONE};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeWindow {
    Day,
    Week,
}

impl TimeWindow {
    fn as_str(self) -> &'static str {
        match self {
            Self::Day => "day",
            Self::Week => "week",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TrendingApi<'a> {
    client: &'a Client,
    window: TimeWindow,
}

impl Client {
    pub fn trending(&self, window: TimeWindow) -> TrendingApi<'_> {
        TrendingApi {
            client: self,
            window,
        }
    }
}

impl TrendingApi<'_> {
    pub async fn all(&self) -> Result<Page<Media>, Error> {
        self.client.get(&self.path("all"), NONE).await
    }

    pub async fn movies(&self) -> Result<Page<MovieSummary>, Error> {
        self.client.get(&self.path("movie"), NONE).await
    }

    pub async fn tv(&self) -> Result<Page<TvSummary>, Error> {
        self.client.get(&self.path("tv"), NONE).await
    }

    pub async fn people(&self) -> Result<Page<PersonSummary>, Error> {
        self.client.get(&self.path("person"), NONE).await
    }

    fn path(&self, media: &str) -> String {
        format!("/trending/{media}/{}", self.window.as_str())
    }
}
