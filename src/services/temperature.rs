use std::error::Error;
use serde::Deserialize;

const URL: &str = "https://app.open-meteo.com/v1/forecast?latitude=51.1315&longitude=-114.0105&current_weather=true";

#[derive(Debug, Deserialize)]
struct CurrentWeather {
    temperature: f32,
}

#[derive(Debug, Deserialize)]
struct WeatherRequest {
    current_weather: CurrentWeather
}

pub async fn get_current_temperature() -> Result<f32, Box<dyn Error>> {
    let response = reqwest::get(URL).await?.json::<WeatherRequest>().await?;
    Ok(response.current_weather.temperature)
}