use std::io::Cursor;

use url::Url;

pub fn is_url(s: &str) -> bool {
    Url::parse(s).is_ok()
}

pub fn download_url(url: &Url, download_path: &str) -> Result<(), String> {
    let resp = reqwest::blocking::get(url.as_str()).unwrap();

    let mut file = std::fs::File::create(download_path).unwrap();
    let mut content =  Cursor::new(resp.bytes().unwrap());
    std::io::copy(&mut content, &mut file).unwrap();

    Ok(())
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
