use aws_sdk_dynamodb::model::AttributeValue;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
pub struct Place {
    pub code: String,
    pub name: String,
    pub administrativeDivision: String,
    pub countryCode: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
pub struct PlaceDetailed {
    pub code: String,
    pub name: String,
    pub administrativeDivision: String,
    pub country: String,
    pub countryCode: String,
    pub coordinates: Coordinates,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Coordinates {
    pub latitude: f32,
    pub longitude: f32,
}

//TODO: Remove this
pub fn to_dynamodb(value: String) -> AttributeValue {
    AttributeValue::S(value)
}

//TODO: Remove this
pub fn to_dynamodb_num(value: f32) -> AttributeValue {
    AttributeValue::N(value.to_string())
}
