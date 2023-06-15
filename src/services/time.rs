use std::error::Error;
use actix_web::cookie::time::OffsetDateTime;
use serde::Deserialize;

const URL: &str = "http://worldtimeapi.org/api/timezone/America/Edmonton";

#[derive(Deserialize)]
pub struct TimeRequest {
    #[serde(with = "time::serde::rfc3339")]
    pub datetime: OffsetDateTime,
}

pub async fn get_current_time() -> Result<OffsetDateTime, Box<dyn Error>> {
    let response = reqwest::get(URL).await?.json::<TimeRequest>().await?;
    Ok(response.datetime)
}