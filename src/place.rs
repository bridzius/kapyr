use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
pub struct Place {
    pub code: String,
    name: String,
    administrativeDivision: String,
    countryCode: String,
}
