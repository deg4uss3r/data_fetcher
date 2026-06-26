use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use thiserror::Error;

use crate::Config;

const PGH_ST_URL: &str = "https://pgh.st/locate/";

/// Response from pgh.st contain information for
/// trash and recycling pickup
#[derive(Clone, Debug, Deserialize)]
pub(crate) struct TrashInfo {
    division: String,
    next_yard_date: String,
    next_recycling_date_long: String,
    regular_trash_pickup_day: i32,
    zip: i32,
    next_yard_date_long: String,
    division_sched: i32,
    number: String,
    next_pickup_date: String,
    holiday_cancellation: bool,
    street: String,
    hood: String,
    other_cancellation: bool,
    next_pickup_date_long: String,
    next_recycling_date: String,
    next_recycling: String,
}

#[derive(Debug, Default, Serialize)]
pub(crate) struct ShortTrashInfo {
    next_yard_date_long: String,
    next_recycling_date_long: String,
    next_pickup_date_long: String,
    holiday_cancellation: bool,
    other_cancellation: bool,
}

impl From<TrashInfo> for ShortTrashInfo {
    fn from(value: TrashInfo) -> Self {
        ShortTrashInfo {
            next_yard_date_long: value.next_yard_date_long,
            next_recycling_date_long: value.next_recycling_date_long,
            next_pickup_date_long: value.next_pickup_date_long,
            holiday_cancellation: value.holiday_cancellation,
            other_cancellation: value.other_cancellation,
        }
    }
}

#[derive(Debug, Error)]
pub(crate) enum TrashError {
    #[error("Error fetching trash data")]
    NetworkError(#[from] reqwest::Error),
    #[error("Error parsing trash data")]
    ParsingError(#[from] serde_json::Error),
    #[error("Error unexpected response from service {0}")]
    ResponseError(usize),
}

pub(crate) async fn get_trash(
    client: &Client,
    config: &Config,
) -> Result<ShortTrashInfo, TrashError> {
    let result: String = client
        .get(format!("{PGH_ST_URL}{}", config.trash_location))
        .send()
        .await?
        .text()
        .await?;

    let full_trash: Vec<TrashInfo> = from_str(&result)?;

    match full_trash.len() {
        1 => Ok(full_trash[0].clone().into()),
        other => Err(TrashError::ResponseError(other)),
    }
}
