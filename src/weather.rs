use reqwest::Client;
use serde::{
    Deserialize, Deserializer, Serialize,
    de::{self, Unexpected},
};
use serde_json::from_str;
use thiserror::Error;

const WEATHER_API_URI: &str = "https://api.weatherapi.com/v1/forecast.json";
const QUERY: &str = "days=1&aqi=yes&alerts=yes";

#[derive(Debug, Error)]
pub(crate) enum WeatherError {
    #[error("Error fetching weather details")]
    NetworkError(#[from] reqwest::Error),
    #[error("Error deserializing weather details")]
    ParsingError(#[from] serde_json::Error),
}

#[derive(Debug, Default, Serialize)]
pub(crate) struct ShortWeather {
    max_temp: f32,
    min_temp: f32,
    chance_of_rain: f32,
    chance_of_snow: f32,
    humidity: f32,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Weather {
    location: Location,
    current: Current,
    forecast: Forecast,
    alerts: Alerts,
}

#[derive(Debug, Deserialize)]
struct Location {
    name: String,
    region: String,
    country: String,
    lat: f64,
    lon: f64,
    tz_id: String,
    localtime_epoch: u64,
    localtime: String,
}

#[derive(Debug, Deserialize)]
struct Current {
    last_updated_epoch: u64,
    last_updated: String,
    temp_c: f32,
    temp_f: f32,
    #[serde(deserialize_with = "int_to_bool")]
    is_day: bool,
    condition: Condition,
    wind_mph: f32,
    wind_kph: f32,
    wind_degree: u16,
    wind_dir: String,
    pressure_mb: f32,
    pressure_in: f32,
    precip_mm: f32,
    precip_in: f32,
    humidity: u8,
    cloud: i32,
    feelslike_c: f32,
    feelslike_f: f32,
    windchill_c: f32,
    windchill_f: f32,
    heatindex_c: f32,
    heatindex_f: f32,
    dewpoint_c: f32,
    dewpoint_f: f32,
    vis_km: f32,
    vis_miles: f32,
    uv: f32,
    gust_mph: f32,
    gust_kph: f32,
    #[serde(deserialize_with = "int_to_bool")]
    will_it_rain: bool,
    chance_of_rain: f32,
    #[serde(deserialize_with = "int_to_bool")]
    will_it_snow: bool,
    chance_of_snow: f32,
    wetbulb_c: f32,
    wetbulb_f: f32,
    air_quality: AirQuality,
}

#[derive(Debug, Deserialize)]
struct Condition {
    text: String,
    icon: String,
    code: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "air_quality")]
struct AirQuality {
    co: f32,
    no2: f32,
    o3: f32,
    so2: f32,
    pm2_5: f32,
    pm10: f32,
    #[serde(rename(deserialize = "us-epa-index"))]
    us_epa_index: i32,
    #[serde(rename(deserialize = "gb-defra-index"))]
    gb_defra_index: i32,
}

#[derive(Debug, Deserialize)]
struct Forecast {
    forecastday: Vec<ForecastDay>,
}

#[derive(Debug, Deserialize)]
struct ForecastDay {
    date: String,
    date_epoch: u64,
    day: Day,
    astro: Astro,
    hour: Vec<Hour>,
}

#[derive(Debug, Deserialize)]
struct Day {
    maxtemp_c: f32,
    maxtemp_f: f32,
    mintemp_c: f32,
    mintemp_f: f32,
    avgtemp_c: f32,
    avgtemp_f: f32,
    maxwind_mph: f32,
    maxwind_kph: f32,
    totalprecip_mm: f32,
    totalprecip_in: f32,
    totalsnow_cm: f32,
    avgvis_km: f32,
    avgvis_miles: f32,
    avghumidity: f32,
    #[serde(deserialize_with = "int_to_bool")]
    daily_will_it_rain: bool,
    daily_chance_of_rain: f32,
    #[serde(deserialize_with = "int_to_bool")]
    daily_will_it_snow: bool,
    daily_chance_of_snow: f32,
    condition: Condition,
    uv: f32,
    avgwetbulb_c: f32,
    avgwetbulb_f: f32,
    maxwetbulb_c: f32,
    maxwetbulb_f: f32,
}

#[derive(Debug, Deserialize)]
struct Astro {
    sunrise: String,
    sunset: String,
    moonrise: String,
    moonset: String,
    moon_phase: String,
    moon_illumination: u8,
    #[serde(deserialize_with = "int_to_bool")]
    is_moon_up: bool,
    #[serde(deserialize_with = "int_to_bool")]
    is_sun_up: bool,
}

#[derive(Debug, Deserialize)]
struct Hour {
    time_epoch: u64,
    time: String,
    temp_c: f32,
    temp_f: f32,
    #[serde(deserialize_with = "int_to_bool")]
    is_day: bool,
    condition: Condition,
    wind_mph: f32,
    wind_kph: f32,
    wind_degree: u16,
    wind_dir: String,
    pressure_mb: f32,
    pressure_in: f32,
    precip_mm: f32,
    precip_in: f32,
    snow_cm: f32,
    humidity: u8,
    cloud: i32,
    feelslike_c: f32,
    feelslike_f: f32,
    windchill_c: f32,
    windchill_f: f32,
    heatindex_c: f32,
    heatindex_f: f32,
    dewpoint_c: f32,
    dewpoint_f: f32,
    will_it_rain: f32,
    chance_of_rain: f32,
    will_it_snow: f32,
    chance_of_snow: f32,
    vis_km: f32,
    vis_miles: f32,
    gust_mph: f32,
    gust_kph: f32,
    uv: f32,
    wetbulb_c: f32,
    wetbulb_f: f32,
}

#[derive(Debug, Deserialize)]
struct Alerts {
    alert: Vec<Alert>,
}

#[derive(Debug, Deserialize)]
struct Alert {
    headline: String,
    msgtype: String,
    severity: String,
    urgency: String,
    areas: String,
    category: String,
    certainty: String,
    event: String,
    note: String,
    effective: String,
    expires: String,
    desc: String,
    instruction: String,
}

pub(crate) async fn get_weather(
    client: &Client,
    config: &crate::Config,
) -> Result<ShortWeather, WeatherError> {
    let url = format!(
        "{WEATHER_API_URI}?key={}&q={}&{QUERY}",
        config.weather_api, config.weather_location
    );

    let result: String = client.get(url).send().await?.text().await?;

    let weather: Weather = from_str(&result)?;

    Ok(ShortWeather {
        max_temp: weather.forecast.forecastday[0].day.maxtemp_f,
        min_temp: weather.forecast.forecastday[0].day.mintemp_f,
        chance_of_rain: weather.forecast.forecastday[0].day.daily_chance_of_rain,
        chance_of_snow: weather.forecast.forecastday[0].day.daily_chance_of_snow,
        humidity: weather.forecast.forecastday[0].day.avghumidity,
    })
}

fn int_to_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match u8::deserialize(deserializer)? {
        0 => Ok(false),
        1 => Ok(true),
        other => Err(de::Error::invalid_value(
            Unexpected::Signed(other as i64),
            &"zero or one",
        )),
    }
}
