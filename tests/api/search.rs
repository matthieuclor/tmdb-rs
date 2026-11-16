use tmdb_rs::SearchQuery;
use wiremock::{
    Mock, ResponseTemplate,
    matchers::{path, query_param, query_param_is_missing},
};

use crate::common::mock_client;

endpoint!(collections: "/search/collection", "search-collection", client => client.search().collections(&SearchQuery::new("star wars")));
endpoint!(companies: "/search/company", "search-company", client => client.search().companies(&SearchQuery::new("lucasfilm")));
endpoint!(keywords: "/search/keyword", "search-keyword", client => client.search().keywords(&SearchQuery::new("space")));
endpoint!(movies: "/search/movie", "search-movie", client => client.search().movies(&SearchQuery::new("fight club")));
endpoint!(multi: "/search/multi", "search-multi", client => client.search().multi(&SearchQuery::new("star wars")));
endpoint!(people: "/search/person", "search-person", client => client.search().people(&SearchQuery::new("tom hanks")));
endpoint!(tv: "/search/tv", "search-tv", client => client.search().tv(&SearchQuery::new("game of thrones")));

#[tokio::test]
async fn sends_only_the_given_parameters() {
    let (server, client) = mock_client().await;
    Mock::given(path("/search/movie"))
        .and(query_param("query", "fight club"))
        .and(query_param("page", "2"))
        .and(query_param("year", "1999"))
        .and(query_param_is_missing("region"))
        .respond_with(ResponseTemplate::new(200).set_body_string("{}"))
        .expect(1)
        .mount(&server)
        .await;

    let query = SearchQuery {
        page: Some(2),
        year: Some(1999),
        ..SearchQuery::new("fight club")
    };
    client.search().movies(&query).await.unwrap();
}
