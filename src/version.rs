use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct GitHubApiResponse {
    tag_name: String,
}

/// Returns the latest release of vfd on GitHub.
///
/// This is used to prompt me to update if I'm running an outdated version.
///
pub async fn get_latest_version() -> Result<Option<String>, reqwest::Error> {
    // Use the GitHub releases API to find the latest release of vfd
    // https://docs.github.com/en/rest/reference/releases#get-the-latest-release
    let url = "https://api.github.com/repos/alexwlchan/books.alexwlchan.net/releases/latest";

    let client = reqwest::Client::new();

    // Return an error if the GET request completely fails, e.g. if we can't
    // connect to the network at all.
    let resp = futures::executor::block_on(
        client
            .get(url)
            .header("Accept", "application/vnd.github.v3+json")
            .header("User-Agent", "alexwlchan (via https://github.com/alexwlchan/books.alexwlchan.net)")
            .send()
    ).unwrap();

    // Return an error if we don't get a 200 OK status code.
    let resp = resp.error_for_status()?;

    let text = resp.text().await?;
    let api_response = serde_json::from_str::<GitHubApiResponse>(&text);

    match api_response {
        Ok(resp) => Ok(Some(resp.tag_name)),
        Err(_)   => Ok(None),
    }
}