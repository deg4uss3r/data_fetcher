mod italian;
mod trash;
mod weather;

use axum::{
    Router,
    body::Body,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
};
use jiff::{Unit, Zoned};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use std::{env::var, sync::Arc};

/// Config for sensitive information
/// trash_location should be a String that conforms to the the end of a
///  https://pgh.st url that contains an address already in URL approved formatting
/// (e.g. space is %20)
///
/// weather_api is a free API Key to api.weatherapi.com
///
/// weather_location is the location you'd like to look up, see weatherapi for available
/// values (city strings, zip codes, etc)
///
/// these are also checked in the environment if used via the default implementation through
/// `DATA_FETCHER_TRASH_LOCATION`
/// `DATA_FETCHER_WEATHER_API_KEY`
/// `DATA_FETCHER_WEATHER_LOCATION`
#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct Config {
    trash_location: String,
    weather_api: String,
    weather_location: String,
}

/// `Default` here means to check for the envvars first
impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            trash_location: var("DATA_FETCHER_TRASH_LOCATION").unwrap_or_default(),
            weather_api: var("DATA_FETCHER_WEATHER_API_KEY").unwrap_or_default(),
            weather_location: var("DATA_FETCHER_WEATHER_LOCATION").unwrap_or_default(),
        }
    }
}

#[derive(Debug, Error)]
enum Error {
    #[error("Error starting server:")]
    StartupError(#[from] std::io::Error),
    #[error("Error with Trash:")]
    TrashError(#[from] trash::TrashError),
    #[error("Error with Italian:")]
    ItalianError(#[from] italian::ItalianError),
    #[error("Error with Weather:")]
    WeatherError(#[from] weather::WeatherError),
    #[error("Error loading config:")]
    ConfigError(#[from] confy::ConfyError),
}

#[derive(Debug, Default, Serialize)]

struct GeneralInfo {
    updated_at: String,
}

#[derive(Debug, Default, Serialize)]
struct DisplayBoard {
    general: GeneralInfo,
    trash: trash::ShortTrashInfo,
    italian: italian::ItalianWord,
    weather: weather::ShortWeather,
}

async fn get_info(config: &Config) -> Result<DisplayBoard, Error> {
    let req_client = Client::new();

    let trash = trash::get_trash(&req_client, config).await?;
    println!("Trash: {trash:?}\n");

    let italian = italian::italian_word(&req_client).await?;
    println!("Italian: {italian:?}\n");

    let weather = weather::get_weather(&req_client, config).await?;
    println!("Weather: {weather:#?}\n");

    let now = Zoned::now().round(Unit::Second).unwrap();
    let general: GeneralInfo = GeneralInfo {
        updated_at: now.strftime("%Y-%m-%d %H:%M:%S").to_string(),
    };

    Ok(DisplayBoard {
        general,
        trash,
        italian,
        weather,
    })
}

async fn display_board(State(config): State<Arc<Config>>) -> impl IntoResponse {
    match get_info(&config).await {
        Ok(d) => Response::builder()
            .status(StatusCode::OK)
            .body(Body::from(serde_json::to_string(&d).unwrap()))
            .unwrap(),
        Err(_e) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from(""))
            .unwrap(),
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config: Config = confy::load("data_fetcher", Some("default"))?;

    let app = Router::new()
        .route("/", get(display_board))
        .with_state(Arc::new(config));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:9999").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
