use anyhow::Result;
use reqwest::{header, Client};

pub fn new_api_client(token: String) -> Result<Client> {
    let mut headers = header::HeaderMap::new();
    headers.insert("Authorization", format!("token {}", token).parse()?);
    headers.insert(
        header::ACCEPT,
        header::HeaderValue::from_static("application/vnd.github.v3+json"),
    );
    let client = reqwest::Client::builder()
        .user_agent("reqwest")
        .default_headers(headers)
        .build()?;
    Ok(client)
}
