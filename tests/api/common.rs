use serde::Serialize;
use serde_json::Value;
use tmdb_rs::Client;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path},
};

macro_rules! endpoint {
    ($name:ident: $route:literal, $fixture:literal, $client:ident => $call:expr) => {
        #[tokio::test]
        async fn $name() {
            let (_server, $client) = crate::common::serve($route, $fixture).await;
            let response = $call.await.unwrap();
            crate::common::assert_fixture($fixture, &response);
        }
    };
}

pub async fn mock_client() -> (MockServer, Client) {
    let server = MockServer::start().await;
    let client = Client::with_base_url("token", server.uri());
    (server, client)
}

pub async fn serve(route: &str, name: &str) -> (MockServer, Client) {
    let (server, client) = mock_client().await;
    Mock::given(method("GET"))
        .and(path(route))
        .respond_with(ResponseTemplate::new(200).set_body_raw(fixture(name), "application/json"))
        .expect(1)
        .mount(&server)
        .await;
    (server, client)
}

pub fn assert_fixture<T: Serialize>(name: &str, value: &T) {
    let expected: Value = serde_json::from_str(&fixture(name)).unwrap();
    let actual = serde_json::to_value(value).unwrap();
    assert_covers(&expected, &actual, name);
}

fn fixture(name: &str) -> String {
    std::fs::read_to_string(format!(
        "{}/tests/fixtures/{name}.json",
        env!("CARGO_MANIFEST_DIR")
    ))
    .unwrap()
}

fn assert_covers(expected: &Value, actual: &Value, at: &str) {
    match (expected, actual) {
        (Value::Null, _) => {}
        (Value::String(expected), Value::Null) if expected.is_empty() => {}
        (Value::Object(expected), Value::Object(actual)) => {
            for (key, value) in expected {
                let at = format!("{at}.{key}");
                let found = actual
                    .get(key)
                    .unwrap_or_else(|| panic!("missing field {at}"));
                assert_covers(value, found, &at);
            }
        }
        (Value::Array(expected), Value::Array(actual)) => {
            assert_eq!(expected.len(), actual.len(), "length of {at}");
            for (index, (expected, actual)) in expected.iter().zip(actual).enumerate() {
                assert_covers(expected, actual, &format!("{at}[{index}]"));
            }
        }
        (Value::String(expected), Value::String(actual)) => assert_eq!(expected, actual, "{at}"),
        (Value::Number(_), Value::Number(_) | Value::String(_)) => {}
        (Value::Bool(_), Value::Bool(_)) => {}
        _ => panic!("type mismatch at {at}: expected {expected}, got {actual}"),
    }
}
