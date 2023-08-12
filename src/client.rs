use reqwest::{
    header::{self, HeaderMap, HeaderValue},
    Body, Client as ReqwestClient, Method,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{SPEndpoint, SPError};

#[derive(Deserialize)]
pub struct ApiResponse<C> {
    pub exists: Option<bool>,
    pub id: Option<String>,
    pub content: Option<C>,
}

#[derive(Clone)]
pub struct Client {
    endpoint: SPEndpoint,
    reqwest_client: ReqwestClient,
}

impl Client {
    pub fn new(auth_key: impl AsRef<str>, endpoint: SPEndpoint) -> Result<Self, SPError> {
        let mut headers = HeaderMap::new();

        headers.insert(
            header::AUTHORIZATION,
            HeaderValue::from_str(auth_key.as_ref()).unwrap(),
        );

        let reqwest_client = ReqwestClient::builder().default_headers(headers).build()?;

        Ok(Self {
            reqwest_client,
            endpoint,
        })
    }

    pub(crate) async fn send<C>(
        &self,
        path: impl AsRef<str>,
        method: Method,
        body: Option<impl Into<Body>>,
        query: &Option<impl Serialize>,
    ) -> Result<ApiResponse<C>, SPError>
    where
        C: DeserializeOwned,
    {
        let url = self.get_url(path);
        let mut builder = self.reqwest_client.request(method, url);

        if let Some(body) = body {
            builder = builder.body(body);
        }

        if let Some(query) = query {
            builder = builder.query(query);
        }

        let res = builder.send().await?;

        Ok(res.json::<ApiResponse<C>>().await?)
    }

    pub(crate) fn get_url(&self, path: impl AsRef<str>) -> String {
        format!("{}{}", self.endpoint.to_string(), path.as_ref())
    }
}
