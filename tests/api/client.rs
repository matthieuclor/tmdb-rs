use std::{error::Error as _, time::Duration};

use chrono::NaiveDate;
use serde_json::json;
use tmdb_rs::{
    ChangesQuery, Client, EpisodeAppend, Error, Media, MovieAppend, PersonAppend, SearchQuery,
    SeasonAppend, TvAppend,
};
use wiremock::{
    Mock, ResponseTemplate,
    matchers::{header, method, path, query_param, query_param_is_missing},
};

use crate::common::mock_client;

fn ok() -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_json(
        json!({ "id": 1, "title": "t", "name": "n", "season_number": 1, "episode_number": 1 }),
    )
}

#[tokio::test]
async fn sends_the_token_as_bearer_without_language() {
    let (server, client) = mock_client().await;
    Mock::given(header("authorization", "Bearer token"))
        .and(query_param_is_missing("language"))
        .respond_with(ok())
        .expect(1)
        .mount(&server)
        .await;

    client.movie(11).details().await.unwrap();
}

#[tokio::test]
async fn sends_the_language_on_every_request() {
    let (server, client) = mock_client().await;
    let client = client.language("fr-FR");
    Mock::given(query_param("language", "fr-FR"))
        .respond_with(ok())
        .expect(1)
        .mount(&server)
        .await;

    client.movie(11).details().await.unwrap();
}

#[tokio::test]
async fn images_include_untagged_images_for_the_language() {
    let (server, client) = mock_client().await;
    let client = client.language("fr-FR");
    Mock::given(path("/movie/11/images"))
        .and(query_param("include_image_language", "fr,null"))
        .respond_with(ok())
        .expect(1)
        .mount(&server)
        .await;

    client.movie(11).images().await.unwrap();
}

#[tokio::test]
async fn images_without_language_do_not_filter() {
    let (server, client) = mock_client().await;
    Mock::given(path("/movie/11/images"))
        .and(query_param_is_missing("include_image_language"))
        .respond_with(ok())
        .expect(1)
        .mount(&server)
        .await;

    client.movie(11).images().await.unwrap();
}

#[tokio::test]
async fn movie_append_to_response_is_typed() {
    let (server, client) = mock_client().await;
    Mock::given(path("/movie/11"))
        .and(query_param("append_to_response", "credits,watch/providers"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": 11,
            "title": "Star Wars",
            "credits": { "cast": [{ "id": 2, "name": "Mark Hamill" }] },
            "watch/providers": { "results": { "FR": { "link": "https://tmdb.org" } } }
        })))
        .expect(1)
        .mount(&server)
        .await;

    let movie = client
        .movie(11)
        .details_with(&[MovieAppend::Credits, MovieAppend::WatchProviders])
        .await
        .unwrap();

    assert_eq!(movie.credits.unwrap().cast[0].name, "Mark Hamill");
    assert_eq!(
        movie.watch_providers.unwrap().results["FR"].link,
        "https://tmdb.org"
    );
}

#[tokio::test]
async fn tv_season_episode_and_person_accept_append_to_response() {
    let (server, client) = mock_client().await;
    Mock::given(query_param("append_to_response", "external_ids,images"))
        .respond_with(ok())
        .expect(4)
        .mount(&server)
        .await;

    client
        .tv(1399)
        .details_with(&[TvAppend::ExternalIds, TvAppend::Images])
        .await
        .unwrap();
    client
        .tv(1399)
        .season(1)
        .details_with(&[SeasonAppend::ExternalIds, SeasonAppend::Images])
        .await
        .unwrap();
    client
        .tv(1399)
        .season(1)
        .episode(1)
        .details_with(&[EpisodeAppend::ExternalIds, EpisodeAppend::Images])
        .await
        .unwrap();
    client
        .person(31)
        .details_with(&[PersonAppend::ExternalIds, PersonAppend::Images])
        .await
        .unwrap();
}

#[tokio::test]
async fn changes_send_the_date_range() {
    let (server, client) = mock_client().await;
    Mock::given(path("/movie/changes"))
        .and(query_param("start_date", "2026-01-01"))
        .and(query_param("end_date", "2026-01-14"))
        .and(query_param_is_missing("page"))
        .respond_with(ok())
        .expect(1)
        .mount(&server)
        .await;

    let query = ChangesQuery {
        start_date: NaiveDate::from_ymd_opt(2026, 1, 1),
        end_date: NaiveDate::from_ymd_opt(2026, 1, 14),
        page: None,
    };
    client.movies().changes(&query).await.unwrap();
}

#[tokio::test]
async fn watch_providers_send_the_region_only_when_given() {
    let (server, client) = mock_client().await;
    Mock::given(path("/watch/providers/movie"))
        .and(query_param_is_missing("watch_region"))
        .respond_with(ok())
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(path("/watch/providers/tv"))
        .and(query_param("watch_region", "FR"))
        .respond_with(ok())
        .expect(1)
        .mount(&server)
        .await;

    client.movies().providers(None).await.unwrap();
    client.tv_shows().providers(Some("FR")).await.unwrap();
}

#[tokio::test]
async fn unknown_media_types_do_not_fail() {
    let (server, client) = mock_client().await;
    Mock::given(path("/search/multi"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "page": 1,
            "results": [{ "media_type": "collection", "id": 10 }]
        })))
        .mount(&server)
        .await;

    let page = client
        .search()
        .multi(&SearchQuery::new("star wars"))
        .await
        .unwrap();

    assert_eq!(page.results, vec![Media::Unknown]);
}

#[tokio::test]
async fn missing_required_field_is_a_decode_error() {
    let (server, client) = mock_client().await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "title": "No id" })))
        .mount(&server)
        .await;

    let error = client.movie(11).details().await.unwrap_err();

    assert!(matches!(error, Error::Decode(_)));
    assert!(
        error
            .to_string()
            .starts_with("TMDB response could not be decoded: missing field `id`")
    );
    assert!(error.source().is_some());
}

#[tokio::test]
async fn api_error_carries_status_and_message() {
    let (server, client) = mock_client().await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(404).set_body_json(json!({
            "success": false,
            "status_code": 34,
            "status_message": "The resource you requested could not be found."
        })))
        .mount(&server)
        .await;

    let error = client.movie(0).details().await.unwrap_err();

    assert!(matches!(
        error,
        Error::Api { status: 404, ref message } if message == "The resource you requested could not be found."
    ));
    assert_eq!(
        error.to_string(),
        "TMDB returned 404: The resource you requested could not be found."
    );
    assert!(error.source().is_none());
}

#[tokio::test]
async fn api_error_without_json_body_has_an_empty_message() {
    let (server, client) = mock_client().await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(503).set_body_string("Service Unavailable"))
        .mount(&server)
        .await;

    let error = client.movie(11).details().await.unwrap_err();

    assert!(matches!(error, Error::Api { status: 503, ref message } if message.is_empty()));
}

#[tokio::test]
async fn rate_limit_carries_retry_after() {
    let (server, client) = mock_client().await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(429).insert_header("retry-after", "3"))
        .mount(&server)
        .await;

    let error = client.movie(11).details().await.unwrap_err();

    assert!(matches!(
        error,
        Error::RateLimited { retry_after: Some(delay) } if delay == Duration::from_secs(3)
    ));
    assert_eq!(error.to_string(), "TMDB rate limit reached");
}

#[tokio::test]
async fn slow_response_is_a_timeout() {
    let (server, client) = mock_client().await;
    let client = client.timeout(Duration::from_millis(50));
    Mock::given(method("GET"))
        .respond_with(ok().set_delay(Duration::from_millis(500)))
        .mount(&server)
        .await;

    let error = client.movie(11).details().await.unwrap_err();

    assert!(matches!(error, Error::Timeout));
    assert_eq!(error.to_string(), "TMDB request timed out");
}

#[tokio::test]
async fn unreachable_server_is_a_network_error() {
    let client = Client::with_base_url("token", "http://127.0.0.1:9");

    let error = client.movie(11).details().await.unwrap_err();

    assert!(matches!(error, Error::Network(_)));
    assert!(error.to_string().starts_with("TMDB request failed: "));
    assert!(error.source().is_some());
}
