mod http;
mod place;

use crate::place::Place;
use serde_json::Value;
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let places_uri = String::from("https://api.meteo.lt/v1/places");
    let places_json = http::get(places_uri).await?;
    let places: Vec<Place> = serde_json::from_str(&places_json)?;

    for x in places {
        let sleep_duration = Duration::from_millis(400);
        //TODO: Remove this
        sleep(sleep_duration);
        println!("Calling {}", x.code);
        let res = call_place(x.code).await?;
        println!("{}", res);
    }
    Ok(())
}

async fn call_place(code: String) -> Result<Value, Box<dyn Error>> {
    let place_uri = String::from("https://api.meteo.lt/v1/places/") + &code;
    let place_json = http::get(place_uri).await?;
    let place: Value = serde_json::from_str(&place_json)?;
    Ok(place)
}
