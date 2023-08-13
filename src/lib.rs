pub(crate) mod client;
pub mod error;
pub mod models;

use std::sync::Arc;

use crate::{client::Client, error::*, models::user::User};

#[derive(Clone, Debug)]
pub enum SPEndpoint {
    Production,
    Pretesting,
    Custom(String),
}

impl ToString for SPEndpoint {
    fn to_string(&self) -> String {
        match self {
            SPEndpoint::Production => "https://api.apparyllis.com".to_string(),
            SPEndpoint::Pretesting => "https://devapi.apparyllis.com".to_string(),
            SPEndpoint::Custom(endpoint) => endpoint.clone(),
        }
    }
}

#[derive(Clone)]
pub struct SPApi {
    client: Arc<Client>,
}

impl SPApi {
    pub fn new(auth_key: impl AsRef<str>, endpoint: SPEndpoint) -> Result<Self, SPError> {
        Ok(Self {
            client: Arc::new(Client::new(auth_key, endpoint)?),
        })
    }

    pub async fn get_user(&self, user_id: Option<String>) -> Result<User, SPError> {
        User::new(user_id, Arc::clone(&self.client)).await
    }
}
