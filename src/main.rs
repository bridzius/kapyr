mod dynamo;
mod forecast;
mod http;
mod place;
mod place_http;

use crate::dynamo::save_place;
use crate::forecast::Forecast;
use crate::place_http::{get_all_places, get_detailed_place, get_place_forecast};
use std::env::var;
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let places = get_all_places().await?;

    let update_or_forecast = var("KAPYR").unwrap_or("update".to_string());

    let lt_places = places
        .into_iter()
        .filter(|place| String::from(&place.countryCode).eq_ignore_ascii_case("LT"));

    for place in lt_places {
        let sleep_duration = Duration::from_millis(400);
        //TODO: Remove this
        sleep(sleep_duration);
        if update_or_forecast.eq_ignore_ascii_case("update") {
            println!("[Updating Place] {}", place.code);
            let detailed_place = get_detailed_place(place.code).await?;
            save_place(detailed_place).await?;
        } else {
            println!("[Fetching Forecast] {}", place.code);
            let place_forecast: Forecast =
                get_place_forecast(place.code, String::from("long-term")).await?;
            println!("[Forecast] {:?}", place_forecast.forecastTimestamps)
        }
    }
    Ok(())
}
