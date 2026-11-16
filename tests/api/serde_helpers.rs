use chrono::NaiveDate;
use serde_json::json;
use tmdb_rs::{Error, Gender};
use wiremock::{Mock, ResponseTemplate, matchers::method};

use crate::common::mock_client;

#[tokio::test]
async fn dates_are_parsed_and_empty_dates_are_none() {
    let (server, client) = mock_client().await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "page": 1,
            "results": [
                { "id": 1, "title": "Dated", "release_date": "1999-10-15" },
                { "id": 2, "title": "Empty", "release_date": "" },
                { "id": 3, "title": "Null", "release_date": null },
                { "id": 4, "title": "Missing" }
            ]
        })))
        .mount(&server)
        .await;

    let page = client.movies().popular(1).await.unwrap();
    let dates: Vec<_> = page
        .results
        .iter()
        .map(|movie| movie.release_date)
        .collect();

    assert_eq!(
        dates,
        vec![NaiveDate::from_ymd_opt(1999, 10, 15), None, None, None]
    );
}

#[tokio::test]
async fn invalid_date_is_a_decode_error() {
    let (server, client) = mock_client().await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": 1,
            "title": "Broken",
            "release_date": "15/10/1999"
        })))
        .mount(&server)
        .await;

    let error = client.movie(1).details().await.unwrap_err();

    assert!(matches!(error, Error::Decode(_)));
}

#[tokio::test]
async fn genders_map_the_tmdb_codes() {
    let (server, client) = mock_client().await;
    let people: Vec<_> = [0, 1, 2, 3, 9]
        .map(|gender| json!({ "id": gender, "name": "n", "gender": gender }))
        .to_vec();
    Mock::given(method("GET"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({ "page": 1, "results": people })),
        )
        .mount(&server)
        .await;

    let page = client.people().popular(1).await.unwrap();
    let genders: Vec<_> = page.results.iter().map(|person| person.gender).collect();

    assert_eq!(
        genders,
        vec![
            Gender::NotSpecified,
            Gender::Female,
            Gender::Male,
            Gender::NonBinary,
            Gender::NotSpecified
        ]
    );
    assert_eq!(serde_json::to_value(Gender::NonBinary).unwrap(), json!(3));
}
