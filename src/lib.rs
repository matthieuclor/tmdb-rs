mod client;
mod error;
mod image;
mod models;

pub use client::Client;
pub use error::Error;
pub use image::image_url;
pub use models::{Genre, Movie, MovieSummary, Page};
