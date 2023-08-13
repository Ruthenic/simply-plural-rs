use std::{collections::HashMap, sync::Arc};

use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    client::Client,
    error::{bail, SPError},
};

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

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub uid: String,
    fields: HashMap<String, Field>,
    pub username: String,

    #[serde(rename = "isAsystem")]
    pub is_a_system: bool,
    pub last_operation_time: i64,

    #[serde(skip)]
    client: Option<Arc<Client>>,
}

#[derive(Deserialize)]
pub struct GetUserResponse<C> {
    pub exists: Option<bool>,
    pub id: Option<String>,
    pub content: Option<C>,
}

impl User {
    pub async fn new(user_id: Option<String>, client: Arc<Client>) -> Result<Self, SPError> {
        let res;

        if let Some(user_id) = user_id {
            res = client
                .request_json(
                    format!("/v1/user/{}", user_id),
                    Method::GET,
                    &None::<String>,
                    &None::<String>,
                )
                .await?;
        } else {
            res = client
                .request_json::<GetUserResponse<Self>>(
                    "/v1/me",
                    Method::GET,
                    &None::<String>,
                    &None::<String>,
                )
                .await?;
        }

        if let Some(mut user) = res.content {
            user.client = Some(client);

            Ok(user)
        } else {
            Err(SPError::ContentMissingError)
        }
    }

    pub async fn set_username(&mut self, new_username: impl AsRef<str>) -> Result<(), SPError> {
        let client = self.client.as_ref().unwrap();

        client
            .request_json::<Value>(
                format!("/v1/user/username/{}", self.uid),
                Method::PATCH,
                &Some(json!({
                    "username": new_username.as_ref()
                })),
                &None::<String>,
            )
            .await?;

        self.username = new_username.as_ref().to_string();

        Ok(())
    }

    pub fn get_fields(&self) -> Vec<(String, Field)> {
        let mut res: Vec<(String, Field)> = vec![];

        for field in self.fields.values() {
            res.push((field.name.clone(), field.clone()))
        }

        res
    }

    /// Note until I write better docs: `name` needs to be the original name, not a new one (that belongs in field)
    pub fn set_field(&mut self, name: String, new_field: Field) {
        let field = self
            .fields
            .iter()
            .filter(|(_, field)| field.name == name)
            .next();

        if let Some((key, _)) = field {
            self.fields.insert(key.clone(), new_field);
        }
    }

    pub async fn update(&self) -> Result<(), SPError> {
        let client = self.client.as_ref().unwrap();

        client
            .request_text(
                format!("/v1/user/{}", self.uid),
                Method::PATCH,
                &Some(json!({
                    "fields": self.fields,
                    "isAsystem": self.is_a_system
                })),
                &None::<String>,
            )
            .await?;

        Ok(())
    }
}
