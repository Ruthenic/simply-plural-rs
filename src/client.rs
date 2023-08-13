use reqwest::{
    header::{self, HeaderMap, HeaderValue},
    Client as ReqwestClient, Method, Response,
};
use serde::{de::DeserializeOwned, Serialize};

use crate::{SPEndpoint, SPError};

#[derive(Clone, Debug)]
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

    pub(crate) async fn request_json<C>(
        &self,
        path: impl AsRef<str>,
        method: Method,
        body: &Option<impl Serialize>,
        query: &Option<impl Serialize>,
    ) -> Result<C, SPError>
    where
        C: DeserializeOwned,
    {
        let res = self.request_raw(path, method, body, query).await?;

        Ok(res.json::<C>().await?)
    }

    pub(crate) async fn request_text(
        &self,
        path: impl AsRef<str>,
        method: Method,
        body: &Option<impl Serialize>,
        query: &Option<impl Serialize>,
    ) -> Result<String, SPError> {
        let res = self.request_raw(path, method, body, query).await?;

        Ok(res.text().await?)
    }

    async fn request_raw(
        &self,
        path: impl AsRef<str>,
        method: Method,
        body: &Option<impl Serialize>,
        query: &Option<impl Serialize>,
    ) -> Result<Response, SPError> {
        let url = self.get_url(path);
        let mut builder = self.reqwest_client.request(method, url);

        if let Some(body) = body {
            builder = builder.json(body);
        }

        if let Some(query) = query {
            builder = builder.query(query);
        }

        let res = builder.send().await?;

        Ok(res)
    }

    pub(crate) fn get_url(&self, path: impl AsRef<str>) -> String {
        format!("{}{}", self.endpoint.to_string(), path.as_ref())
    }
}
