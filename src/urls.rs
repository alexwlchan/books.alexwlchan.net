use std::io::Cursor;
use std::path::PathBuf;

use reqwest::header::CONTENT_TYPE;
use url::Url;

pub fn is_url(s: &str) -> bool {
    Url::parse(s).is_ok()
}

/// Download the contents of a URL to a local path.
///
/// This function assumes the HTTP request may fail, but everyone on the local
/// filesystem will be fine.  In particular, it returns an Err<reqwest::Error>
/// if the HTTP request fails, but it panics if something goes wrong writing
/// successfully fetched bytes to a file.
///
/// It will panic if `download_path` is not a valid path or its parent directory
/// doesn't exist.
///
/// This will append the appropriate file extension to the provided path,
/// based on the file which is downloaded.
///
/// Returns the path where the file was actually downloaded.
///
pub fn download_url(url: &Url, download_path: PathBuf) -> Result<PathBuf, reqwest::Error> {

    // Return an error if the GET request completely fails, e.g. if we can't
    // connect to the network at all.
    let resp = futures::executor::block_on(reqwest::get(url.as_str())).unwrap();

    // Return an error if we don't get a 200 OK status code.
    let resp = resp.error_for_status()?;

    // Decide what we're going to use as the actual download path.
    //
    // Pick the file extension based on the content-type header; this is
    // significantly more reliable than inspecting the URL path (which may
    // have a wrong extension or no extension at all).
    //
    // This mapping will likely have to be extended over time.
    let extension = match resp.headers().get(CONTENT_TYPE).and_then(|value| value.to_str().ok()) {
        Some("image/jpeg") => "jpg",
        Some("image/png")  => "png",
        _ => "",
    };

    let download_path = download_path.with_extension(extension);

    // Assuming we made a successful request, write the bytes of the response
    // to a file.
    //
    // Ideally I wouldn't be using quite so much unwrap() here, but as we're
    // doing operations on the local filesystem and within a known-safe directory
    // (i.e. the "covers" folder of this repository), it's probably fine.
    let mut file = std::fs::File::create(&download_path).unwrap();
    let mut content = Cursor::new(futures::executor::block_on(resp.bytes()).unwrap());
    std::io::copy(&mut content, &mut file).unwrap();

    Ok(download_path)
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
