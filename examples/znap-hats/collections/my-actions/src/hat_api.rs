use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use znap::prelude::*;

use crate::error::ActionError;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hat {
    pub id: String,
    pub image_url: String,
    pub title: String,
    pub price: u64,
    pub details_url: String,
    pub share_image_url: String,
}

pub async fn fetch_hat(hat_id: &String) -> Result<Hat> {
    let response = reqwest::Client::new()
        .get(format!(
            "http://localhost:5020/api/hats/{}",
            hat_id
        ))
        .send()
        .await
        .or_else(|_| Err(Error::from(ActionError::InternalServerError)))?;

    if response.status() == StatusCode::NOT_FOUND {
        return Err(Error::from(ActionError::HatNotFound));
    }

    if response.status() == StatusCode::OK {
        return response
            .json::<Hat>()
            .await
            .or_else(|_| Err(Error::from(ActionError::InvalidResponseBody)));
    }

    return Err(Error::from(ActionError::UnknownServerError));
}