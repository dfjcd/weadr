use crate::dtos::{WeatherResponse, WeatherResult};

pub struct WeadrClient {
    api_key: String,
}

impl WeadrClient {
    pub fn new(key: String) -> Self {
        WeadrClient { api_key: key }
    }

    pub async fn get_data(&self, city: &str) -> WeatherResult {
        let url = format!("https://api.weatherapi.com/v1/current.json?key={}&q={}&aqi=yes", &self.api_key, city);
        let result = reqwest::get(url).await?;

        let json = result.json::<WeatherResponse>().await?;

        Ok(json)
    }
}
