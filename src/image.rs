const IMAGE_BASE_URL: &str = "https://image.tmdb.org/t/p";

pub fn image_url(path: &str, size: &str) -> String {
    format!("{IMAGE_BASE_URL}/{size}{path}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_a_sized_image_url() {
        assert_eq!(
            image_url("/poster.jpg", "w500"),
            "https://image.tmdb.org/t/p/w500/poster.jpg"
        );
    }
}
