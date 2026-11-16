use std::error::Error as _;

use serde_json::json;
use tmdb_rs::{Client, Error};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{header, method, path, query_param},
};

async fn client() -> (MockServer, Client) {
    let server = MockServer::start().await;
    let client = Client::with_base_url("token", server.uri());
    (server, client)
}

#[tokio::test]
async fn search_movies_returns_a_page_of_summaries() {
    let (server, client) = client().await;
    Mock::given(method("GET"))
        .and(path("/search/movie"))
        .and(header("authorization", "Bearer token"))
        .and(query_param("query", "fight club"))
        .and(query_param("page", "2"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "page": 2,
            "results": [{
                "id": 550,
                "title": "Fight Club",
                "original_title": "Fight Club",
                "overview": "An insomniac office worker...",
                "release_date": "1999-10-15",
                "poster_path": "/poster.jpg",
                "backdrop_path": null,
                "vote_average": 8.4
            }],
            "total_pages": 3,
            "total_results": 41
        })))
        .expect(1)
        .mount(&server)
        .await;

    let page = client.search_movies("fight club", 2).await.unwrap();

    assert_eq!(page.page, 2);
    assert_eq!(page.total_results, 41);
    assert_eq!(page.results[0].id, 550);
    assert_eq!(page.results[0].poster_path.as_deref(), Some("/poster.jpg"));
    assert_eq!(page.results[0].backdrop_path, None);
}

#[tokio::test]
async fn movie_returns_the_details() {
    let (server, client) = client().await;
    Mock::given(method("GET"))
        .and(path("/movie/550"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": 550,
            "imdb_id": "tt0137523",
            "title": "Fight Club",
            "original_title": "Fight Club",
            "overview": "An insomniac office worker...",
            "tagline": "Mischief. Mayhem. Soap.",
            "release_date": "1999-10-15",
            "runtime": 139,
            "genres": [{ "id": 18, "name": "Drama" }],
            "poster_path": "/poster.jpg",
            "backdrop_path": "/backdrop.jpg",
            "vote_average": 8.4
        })))
        .mount(&server)
        .await;

    let movie = client.movie(550).await.unwrap();

    assert_eq!(movie.title, "Fight Club");
    assert_eq!(movie.runtime, Some(139));
    assert_eq!(movie.genres[0].name, "Drama");
}

#[tokio::test]
async fn language_is_sent_as_query_parameter() {
    let (server, client) = client().await;
    let client = client.language("fr-FR");
    Mock::given(method("GET"))
        .and(path("/movie/550"))
        .and(query_param("language", "fr-FR"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": 550,
            "imdb_id": null,
            "title": "Fight Club",
            "original_title": "Fight Club",
            "tagline": null,
            "runtime": null,
            "poster_path": null,
            "backdrop_path": null
        })))
        .expect(1)
        .mount(&server)
        .await;

    let movie = client.movie(550).await.unwrap();

    assert_eq!(movie.overview, "");
    assert!(movie.genres.is_empty());
}

#[tokio::test]
async fn api_error_carries_status_and_message() {
    let (server, client) = client().await;
    Mock::given(method("GET"))
        .and(path("/movie/0"))
        .respond_with(ResponseTemplate::new(404).set_body_json(json!({
            "success": false,
            "status_code": 34,
            "status_message": "The resource you requested could not be found."
        })))
        .mount(&server)
        .await;

    let error = client.movie(0).await.unwrap_err();

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
    let (server, client) = client().await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(503).set_body_string("Service Unavailable"))
        .mount(&server)
        .await;

    let error = client.movie(550).await.unwrap_err();

    assert!(matches!(error, Error::Api { status: 503, ref message } if message.is_empty()));
}

#[tokio::test]
async fn invalid_body_is_an_http_error() {
    let (server, client) = client().await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_string("not json"))
        .mount(&server)
        .await;

    let error = client.movie(550).await.unwrap_err();

    assert!(matches!(error, Error::Http(_)));
    assert!(error.to_string().starts_with("TMDB request failed: "));
    assert!(error.source().is_some());
}
