mod dynamo;
mod http;
mod place;

use crate::dynamo::save_place;
use crate::place::{Place, PlaceDetailed};
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let places_uri = String::from("https://api.meteo.lt/v1/places");
    let places_json = http::get(places_uri).await?;
    let places: Vec<Place> = serde_json::from_str(&places_json)?;

    let lt_places = places
        .into_iter()
        .filter(|place| String::from(&place.countryCode).eq_ignore_ascii_case("LT"));

    for place in lt_places {
        let sleep_duration = Duration::from_millis(400);
        //TODO: Remove this
        sleep(sleep_duration);
        let detailed_place = call_place(place.code).await?;
        save_place(detailed_place).await?;
    }
    Ok(())
}

async fn call_place(code: String) -> Result<PlaceDetailed, Box<dyn Error>> {
    let place_uri = String::from("https://api.meteo.lt/v1/places/") + &code;
    let place_json = http::get(place_uri).await?;
    let place: PlaceDetailed = serde_json::from_str(&place_json).unwrap();
    Ok(place)
}
