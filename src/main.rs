use reqwest;
use std::error::Error;
use std::time::Duration;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let places_json = client.get("https://api.meteo.lt/v1/places")
        .header("Accept", "application/json")
        .timeout(Duration::from_secs(3))
        .send()
        .await?
        .text()
        .await?;

    let places: Vec<Place> = serde_json::from_str(&places_json)?;

    for x in places {
    println!("{:?}", x);
    }
    Ok(())
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
struct Place {
    code: String,
    name: String,
    administrativeDivision: String,
    countryCode: String
}

