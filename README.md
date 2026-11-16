# tmdb-rs

Small async Rust client for [The Movie Database (TMDB)](https://www.themoviedb.org/) API v3.

It wraps only what is needed to search movies and fetch their details, with typed responses and a single error type.

## Features

- Movie search with pagination
- Movie details (runtime, genres, tagline, IMDb id...)
- Optional response language (`fr-FR`, `en-US`...)
- Image URL builder for posters and backdrops
- Bearer token authentication
- `reqwest` with `rustls`, no OpenSSL

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

Never commit this token. Load it from the environment instead:

```rust
let token = std::env::var("TMDB_API_TOKEN")?;
```

## Usage

```rust
use tmdb_rs::{Client, image_url};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(std::env::var("TMDB_API_TOKEN")?).language("fr-FR");

    let page = client.search_movies("fight club", 1).await?;
    println!("{} results", page.total_results);

    for movie in &page.results {
        println!("{} ({})", movie.title, movie.release_date);
    }

    let movie = client.movie(550).await?;
    println!("{} min, {:?}", movie.runtime.unwrap_or_default(), movie.genres);

    if let Some(poster) = &movie.poster_path {
        println!("{}", image_url(poster, "w500"));
    }

    Ok(())
}
```

## API

| Method | TMDB endpoint | Returns |
| --- | --- | --- |
| `Client::new(token)` | | `Client` |
| `client.language(code)` | adds `language=<code>` to every request | `Client` |
| `client.search_movies(query, page)` | `GET /search/movie` | `Page<MovieSummary>` |
| `client.movie(id)` | `GET /movie/{id}` | `Movie` |
| `image_url(path, size)` | `https://image.tmdb.org/t/p/{size}{path}` | `String` |

Common image sizes: `w92`, `w185`, `w342`, `w500`, `w780`, `original`.

## Errors

Every call returns `Result<T, tmdb_rs::Error>`:

```rust
use tmdb_rs::Error;

match client.movie(0).await {
    Ok(movie) => println!("{}", movie.title),
    Err(Error::Api { status: 404, .. }) => println!("not found"),
    Err(Error::Api { status, message }) => println!("TMDB error {status}: {message}"),
    Err(Error::Http(error)) => println!("network or decoding error: {error}"),
}
```

- `Error::Api` holds the HTTP status and the `status_message` returned by TMDB (empty when the body is not JSON).
- `Error::Http` wraps the underlying `reqwest::Error` (connection failure, timeout, invalid JSON).

`Error` implements `std::error::Error`, so it works with `?`, `anyhow` and `Box<dyn Error>`.

## Development

```bash
cargo test                                 # unit and integration tests
cargo cov                                  # tests + coverage, fails under 100% lines
cargo clippy --all-targets -- -D warnings
cargo fmt
```

Tests never call the real API: they run against a local [wiremock](https://crates.io/crates/wiremock) server. `cargo cov` requires [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov).

## Attribution

This product uses the TMDB API but is not endorsed or certified by TMDB.
