use console::{style, StyledObject};
use serde::de::{self, Deserializer, Unexpected};
use serde::Deserialize;

pub type WeatherResult = Result<WeatherResponse, anyhow::Error>;

#[derive(Debug, Deserialize)]
pub struct Condition {
    pub text: String,
    pub icon: String,
}

#[derive(Debug, Deserialize)]
pub struct AirQuality {
    #[serde(rename(deserialize = "gb-defra-index"))]
    pub gb_defra_index: i32,
}

impl AirQuality {
    pub fn description(&self) -> StyledObject<&str> {
        match &self.gb_defra_index {
            1..=3 => style("Low").green(),
            4..=6 => style("Moderate").yellow(),
            7..=9 => style("High").red(),
            10.. => style("Very high").bright().red(),
            _ => style("Unknown").white(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Location {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct CurrentWeather {
    pub temp_c: f32,
    #[serde(deserialize_with = "bool_from_int")]
    pub is_day: bool,
    pub feelslike_c: f32,
    pub condition: Condition,
    pub air_quality: AirQuality,
    pub last_updated: String,
}

#[derive(Debug, Deserialize)]
pub struct WeatherResponse {
    pub location: Location,
    pub current: CurrentWeather,
}

fn bool_from_int<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match u8::deserialize(deserializer)? {
        0 => Ok(false),
        1 => Ok(true),
        other => Err(de::Error::invalid_value(
            Unexpected::Unsigned(other as u64),
            &"zero or one",
        )),
    }
}
