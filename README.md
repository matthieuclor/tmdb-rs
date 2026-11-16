# tmdb-rs

Async Rust client for [The Movie Database (TMDB)](https://www.themoviedb.org/) API v3, with typed responses and a single error type.

## Features

- Every public read endpoint of the API v3 (116 endpoints): movies, TV series, seasons, episodes, people, collections, companies, networks, search, discover, trending, configuration, certifications, genres, watch providers, keywords, lists, reviews, credits, find, changes
- Resource-oriented API: `client.movie(11).credits()`, `client.tv(1399).season(1).episode(1).images()`
- Typed `append_to_response` on movies, series, seasons, episodes and people
- Dates as `chrono::NaiveDate`, genders as an enum
- Optional response language (`fr-FR`, `en-US`...), applied to every request
- Configurable timeout, dedicated rate limit error with `Retry-After`
- `reqwest` with `rustls`, no OpenSSL, and no `reqwest` type in the public API

Endpoints that need a user session (account, authentication, guest sessions, ratings, list editing, account states) are not covered.

## Installation

The crate is not published on crates.io yet. Add it from Git:

```toml
[dependencies]
tmdb-rs = { git = "https://github.com/<owner>/tmdb-rs" }
```

or from a local checkout:

```bash
cargo add --path ../tmdb-rs
```

The client is async and needs a Tokio runtime.

## Authentication

Create an account on TMDB, then copy the **API Read Access Token** from your [API settings](https://www.themoviedb.org/settings/api). The client sends it as `Authorization: Bearer <token>`, which keeps it out of URLs and logs.

Never commit this token. Load it from the environment instead.

## Usage

```rust,no_run
use std::time::Duration;

use tmdb_rs::{Client, MovieAppend, SearchQuery, image_url};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(std::env::var("TMDB_API_TOKEN")?)
        .language("fr-FR")
        .timeout(Duration::from_secs(10));

    let page = client.search().movies(&SearchQuery::new("fight club")).await?;
    println!("{} results", page.total_results);

    for movie in &page.results {
        println!("{} ({:?})", movie.title, movie.release_date);
    }

    let movie = client
        .movie(550)
        .details_with(&[MovieAppend::Credits, MovieAppend::Videos])
        .await?;
    println!("{} min, {:?}", movie.runtime.unwrap_or_default(), movie.genres);

    if let Some(credits) = &movie.credits {
        println!("starring {}", credits.cast[0].name);
    }

    if let Some(poster) = &movie.poster_path {
        println!("{}", image_url(poster, "w500"));
    }

    let episode = client.tv(1399).season(1).episode(1).details().await?;
    println!("{} aired on {:?}", episode.name, episode.air_date);

    Ok(())
}
```

## API

Each resource is reached through a lightweight handle borrowed from the `Client`:

| Handle | Methods |
| --- | --- |
| `client.movie(id)` | `details`, `details_with`, `alternative_titles`, `changes`, `credits`, `external_ids`, `images`, `keywords`, `lists`, `recommendations`, `release_dates`, `reviews`, `similar`, `translations`, `videos`, `watch_providers` |
| `client.movies()` | `latest`, `now_playing`, `popular`, `top_rated`, `upcoming`, `changes`, `certifications`, `genres`, `providers` |
| `client.tv(id)` | `details`, `details_with`, `aggregate_credits`, `alternative_titles`, `changes`, `content_ratings`, `credits`, `episode_groups`, `external_ids`, `images`, `keywords`, `lists`, `recommendations`, `reviews`, `screened_theatrically`, `similar`, `translations`, `videos`, `watch_providers`, `season` |
| `client.tv(id).season(n)` | `details`, `details_with`, `aggregate_credits`, `credits`, `external_ids`, `images`, `translations`, `videos`, `watch_providers`, `episode` |
| `client.tv(id).season(n).episode(n)` | `details`, `details_with`, `credits`, `external_ids`, `images`, `translations`, `videos` |
| `client.tv_shows()` | `latest`, `airing_today`, `on_the_air`, `popular`, `top_rated`, `changes`, `season_changes`, `episode_changes`, `episode_group`, `certifications`, `genres`, `providers` |
| `client.person(id)` | `details`, `details_with`, `changes`, `combined_credits`, `movie_credits`, `tv_credits`, `external_ids`, `images`, `tagged_images`, `translations` |
| `client.people()` | `latest`, `popular`, `changes` |
| `client.collection(id)` | `details`, `images`, `translations` |
| `client.company(id)`, `client.network(id)` | `details`, `alternative_names`, `images` |
| `client.keyword(id)` | `details`, `movies` |
| `client.list(id)` | `details`, `item_status` |
| `client.credit(id)`, `client.review(id)` | `details` |
| `client.external_id(source, id)` | `find` |
| `client.search()` | `collections`, `companies`, `keywords`, `movies`, `multi`, `people`, `tv` |
| `client.discover()` | `movies`, `tv` |
| `client.trending(window)` | `all`, `movies`, `tv`, `people` |
| `client.configuration()` | `details`, `countries`, `jobs`, `languages`, `primary_translations`, `timezones`, `watch_regions` |

Paginated endpoints take a `page: u32` and return `Page<T>`. Endpoints with many filters take a query struct whose fields are all optional:

```rust,no_run
# async fn example(client: tmdb_rs::Client) -> Result<(), tmdb_rs::Error> {
use chrono::NaiveDate;
use tmdb_rs::{MovieDiscover, SearchQuery, TimeWindow};

let query = SearchQuery { year: Some(1999), ..SearchQuery::new("matrix") };
let movies = client.search().movies(&query).await?;

let discover = MovieDiscover {
    with_genres: Some("28|12".to_string()),
    primary_release_date_gte: NaiveDate::from_ymd_opt(2020, 1, 1),
    sort_by: Some("popularity.desc".to_string()),
    ..MovieDiscover::default()
};
let page = client.discover().movies(&discover).await?;

let trending = client.trending(TimeWindow::Week).all().await?;
# Ok(())
# }
```

Multi-type results (`search().multi`, `trending().all`, `known_for`, list items, combined credits) are returned as the `Media` enum: `Media::Movie`, `Media::Tv`, `Media::Person`, or `Media::Unknown` for any other type.

When a language is set, image endpoints also send `include_image_language=<code>,null`, so images without text are still returned.

Common image sizes for `image_url(path, size)`: `w92`, `w185`, `w342`, `w500`, `w780`, `original`.

## Errors

Every call returns `Result<T, tmdb_rs::Error>`:

```rust,no_run
# async fn example(client: tmdb_rs::Client) {
use tmdb_rs::Error;

match client.movie(0).details().await {
    Ok(movie) => println!("{}", movie.title),
    Err(Error::Api { status: 404, .. }) => println!("not found"),
    Err(Error::RateLimited { retry_after }) => println!("retry in {retry_after:?}"),
    Err(Error::Timeout) => println!("too slow"),
    Err(error) => println!("{error}"),
}
# }
```

| Variant | When |
| --- | --- |
| `Error::Api { status, message }` | TMDB answered with an error status; `message` is its `status_message`, empty when the body is not JSON |
| `Error::RateLimited { retry_after }` | TMDB answered 429; `retry_after` comes from the `Retry-After` header |
| `Error::Timeout` | No answer within the timeout (30 seconds by default) |
| `Error::Network(source)` | Connection or TLS failure |
| `Error::Decode(source)` | The body is not the expected JSON, a required field (`id`, `title`, `name`...) is missing, or a date is malformed |

`Error` implements `std::error::Error` and is `#[non_exhaustive]`, so match it with a fallback arm.

## Development

```bash
cargo test                                 # integration tests and README examples
cargo cov                                  # tests + coverage, fails under 100% lines
cargo clippy --all-targets -- -D warnings
cargo fmt
```

Tests never call the real API: they run against a local [wiremock](https://crates.io/crates/wiremock) server. Each endpoint is tested with the response example from the official OpenAPI specification (`tests/fixtures/`), and the typed result is serialized back to check that no field from the example was lost. `cargo cov` requires [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov).

## Attribution

This product uses the TMDB API but is not endorsed or certified by TMDB.
