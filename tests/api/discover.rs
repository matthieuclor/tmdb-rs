use chrono::NaiveDate;
use tmdb_rs::{MovieDiscover, TvDiscover};
use wiremock::{
    Mock, ResponseTemplate,
    matchers::{path, query_param, query_param_is_missing},
};

use crate::common::mock_client;

endpoint!(movies: "/discover/movie", "discover-movie", client => client.discover().movies(&MovieDiscover::default()));
endpoint!(tv: "/discover/tv", "discover-tv", client => client.discover().tv(&TvDiscover::default()));

#[tokio::test]
async fn uses_the_dotted_parameter_names() {
    let (server, client) = mock_client().await;
    Mock::given(path("/discover/movie"))
        .and(query_param("primary_release_date.gte", "2020-01-01"))
        .and(query_param("with_genres", "28|12"))
        .and(query_param("vote_average.gte", "7.5"))
        .and(query_param_is_missing("sort_by"))
        .respond_with(ResponseTemplate::new(200).set_body_string("{}"))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(path("/discover/tv"))
        .and(query_param("first_air_date.lte", "2010-12-31"))
        .and(query_param("with_networks", "49"))
        .respond_with(ResponseTemplate::new(200).set_body_string("{}"))
        .expect(1)
        .mount(&server)
        .await;

    let movies = MovieDiscover {
        primary_release_date_gte: NaiveDate::from_ymd_opt(2020, 1, 1),
        with_genres: Some("28|12".to_string()),
        vote_average_gte: Some(7.5),
        ..MovieDiscover::default()
    };
    let tv = TvDiscover {
        first_air_date_lte: NaiveDate::from_ymd_opt(2010, 12, 31),
        with_networks: Some(49),
        ..TvDiscover::default()
    };
    client.discover().movies(&movies).await.unwrap();
    client.discover().tv(&tv).await.unwrap();
}
