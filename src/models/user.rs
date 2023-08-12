use std::{collections::HashMap, sync::Arc};

use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{client::Client, error::SPError};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Field {
    pub name: String,
    pub order: i64,
    pub private: bool,
    pub prevent_trusted: bool,
    #[serde(rename = "type")]
    pub field_type: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub uid: String,
    pub fields: HashMap<String, Field>,
    pub username: String,
    pub is_asystem: bool,
    pub last_operation_time: i64,
}

impl User {
    pub async fn new(user_id: Option<String>, client: Arc<Client>) -> Result<Self, SPError> {
        let res;

        if let Some(user_id) = user_id {
            res = client
                .send(
                    format!("/v1/user/{}", user_id),
                    Method::GET,
                    None::<String>,
                    &None::<String>,
                )
                .await?;
        } else {
            res = client
                .send::<Self>("/v1/me", Method::GET, None::<String>, &None::<String>)
                .await?;
        }

        if let Some(user) = res.content {
            Ok(user)
        } else {
            Err(SPError::ContentMissingError)
        }
    }
}
