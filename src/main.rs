use reqwest;
use std::error::Error;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use serde_json::{Value};
use std::thread::sleep;

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
        let one_sec = Duration::from_secs(1);
        //TODO: Remove this
        sleep(one_sec);
        println!("Calling {}", x.code);
        let res = call_place(x.code).await?;
        println!("{}", res);
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

async fn call_place(code: String) -> Result<Value, Box<dyn Error>> {
    let client = reqwest::Client::new();

    let place_uri = String::from("https://api.meteo.lt/v1/places/") + &code; 

    let place_json = client.get(place_uri)
        .header("Accept", "application/json")
        .send().await?.text().await?;

    let place: Value = serde_json::from_str(&place_json)?;
    Ok(place)
}

