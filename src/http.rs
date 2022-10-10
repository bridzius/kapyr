use reqwest::Client;
use std::error::Error;
use std::time::Duration;

pub async fn get(uri: String) -> Result<String, Box<dyn Error>> {
    let client = Client::new();

    let response_json = client
        .get(uri)
        .header("Accept", "application/json")
        .timeout(Duration::from_secs(3))
        .send()
        .await?
        .text()
        .await?;

    Ok(response_json)
}
