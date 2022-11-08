mod http;
mod place;

use crate::place::{to_dynamodb, to_dynamodb_num, Place, PlaceDetailed};
use aws_sdk_dynamodb::Client;
use std::env::var;
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let places_uri = String::from("https://api.meteo.lt/v1/places");
    let places_json = http::get(places_uri).await?;
    let places: Vec<Place> = serde_json::from_str(&places_json)?;
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);
    let table_name = var("METEO_TABLE").unwrap_or("none".to_string());
    println!("Updating table {}", table_name);

    let lt_places = places
        .into_iter()
        .filter(|place| String::from(&place.countryCode).eq_ignore_ascii_case("LT"));

    for place in lt_places {
        let sleep_duration = Duration::from_millis(400);
        //TODO: Remove this
        sleep(sleep_duration);
        println!("Calling {}", place.code);
        let req = client.put_item().table_name(String::from(&table_name));
        let detailed_place = call_place(place.code).await?;
        req.item("code", to_dynamodb(detailed_place.code))
            .item("name", to_dynamodb(detailed_place.name))
            .item("country", to_dynamodb(detailed_place.country))
            .item("countryCode", to_dynamodb(detailed_place.countryCode))
            .item(
                "administrativeDivision",
                to_dynamodb(detailed_place.administrativeDivision),
            )
            .item(
                "coord_lat",
                to_dynamodb_num(detailed_place.coordinates.latitude),
            )
            .item(
                "coord_lon",
                to_dynamodb_num(detailed_place.coordinates.longitude),
            )
            .send()
            .await?;
    }
    Ok(())
}

async fn call_place(code: String) -> Result<PlaceDetailed, Box<dyn Error>> {
    let place_uri = String::from("https://api.meteo.lt/v1/places/") + &code;
    let place_json = http::get(place_uri).await?;
    let place: PlaceDetailed = serde_json::from_str(&place_json).unwrap();
    Ok(place)
}
