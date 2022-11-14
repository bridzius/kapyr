use crate::place::{to_dynamodb, to_dynamodb_num, PlaceDetailed};
use aws_sdk_dynamodb::Client;
use std::env::var;
use std::error::Error;

pub async fn save_place(detailed_place: PlaceDetailed) -> Result<(), Box<dyn Error>> {
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);
    let table_name = var("METEO_TABLE").unwrap_or("none".to_string());
    let req = client.put_item().table_name(String::from(&table_name));
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

    Ok(())
}
