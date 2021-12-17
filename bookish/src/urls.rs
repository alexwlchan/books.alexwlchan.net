use url::Url;

pub fn is_url(s: &str) -> bool {
    Url::parse(s).is_ok()
}

#[cfg(test)]
mod tests {
    use crate::urls::is_url;

    #[test]
    fn is_url_matches_valid_urls() {
        assert!(is_url("https://example.net/"));
        assert!(is_url("https://example.net/picture.jpg"));
        assert!(is_url("https://example.net/picture.jpg?query=cat"));

    }

    #[test]
    fn is_url_does_not_match_invalid_urls() {
        assert!(!is_url("XXX"));
        assert!(!is_url(""));
    }
}
