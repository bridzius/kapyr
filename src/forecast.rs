use crate::place::PlaceDetailed;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
pub struct Forecast {
    pub place: PlaceDetailed,
    pub forecastType: String,
    pub forecastCreationTimeUtc: String,
    pub forecastTimestamps: Vec<ForecastTimestamp>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
pub struct ForecastTimestamp {
    forecastTimeUtc: String,
    airTemperature: f32,
    feelsLikeTemperature: f32,
    windSpeed: i32,
    windGust: i32,
    windDirection: i32,
    cloudCover: i32,
    seaLevelPressure: i32,
    relativeHumidity: i32,
    totalPrecipitation: f32,
    conditionCode: String,
}
