use crate::forecast::Forecast;
use crate::http::get;
use crate::place::{Place, PlaceDetailed};
use std::error::Error;
use std::fmt::Write;

pub async fn get_all_places() -> Result<Vec<Place>, Box<dyn Error>> {
    println!("Calling all places");
    let places_uri = String::from("https://api.meteo.lt/v1/places");
    let places_json = get(places_uri).await?;
    let places: Vec<Place> = serde_json::from_str(&places_json)?;
    println!("PLaces are ok");
    Ok(places)
}

pub async fn get_detailed_place(code: String) -> Result<PlaceDetailed, Box<dyn Error>> {
    let place_uri = String::from("https://api.meteo.lt/v1/places/") + &code;
    let place_json = get(place_uri).await?;
    let place: PlaceDetailed = serde_json::from_str(&place_json).unwrap();
    Ok(place)
}

pub async fn get_place_forecast(
    code: String,
    forecast: String,
) -> Result<Forecast, Box<dyn Error>> {
    let mut forecast_uri = String::new();
    write!(
        &mut forecast_uri,
        "https://api.meteo.lt/v1/places/{}/forecasts/{}",
        code, forecast
    );
    let forecast_json = get(forecast_uri).await?;
    let forecast: Forecast = serde_json::from_str(&forecast_json).unwrap();
    Ok(forecast)
}
