// http.rs - Utility functions for making web requests.
// These helpers are used to download files and fetch data from APIs.

use anyhow::{Context, Result};
use reqwest::Client;

/// Create a new HTTP client with default settings.
/// This client is used for all web requests in the project.
///
/// # Returns
/// A configured HTTP client.
pub fn create_client() -> Result<Client> {
    let client = Client::builder()
        .user_agent("juv/0.1.0")
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .context("Failed to create HTTP client")?;
    Ok(client)
}

/// Fetch JSON data from a URL and return it as a string.
///
/// # Arguments
/// * `url` - The URL to fetch.
///
/// # Returns
/// The response body as a string.
pub async fn fetch_json(url: &str) -> Result<String> {
    let client = create_client()?;
    let response = client
        .get(url)
        .send()
        .await
        .with_context(|| format!("Failed to fetch '{}'", url))?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!(
            "Server returned error: {}",
            response.status()
        ));
    }

    let body = response
        .text()
        .await
        .context("Failed to read response body")?;

    Ok(body)
}
