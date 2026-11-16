use tmdb_rs::ExternalSource;
use wiremock::{
    Mock, ResponseTemplate,
    matchers::{path, query_param},
};

use crate::common::mock_client;

endpoint!(by_external_id: "/find/tt0076759", "find-by-id", client => client.external_id(ExternalSource::ImdbId, "tt0076759").find());

#[tokio::test]
async fn sends_the_external_source() {
    let (server, client) = mock_client().await;
    Mock::given(path("/find/tt0137523"))
        .and(query_param("external_source", "imdb_id"))
        .respond_with(ResponseTemplate::new(200).set_body_string("{}"))
        .expect(1)
        .mount(&server)
        .await;

    client
        .external_id(ExternalSource::ImdbId, "tt0137523")
        .find()
        .await
        .unwrap();
}
