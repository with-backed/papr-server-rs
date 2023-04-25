use graphql_client::{QueryBody, Response};
use serde::de::DeserializeOwned;
use serde::Serialize;

pub struct Client {
    client: reqwest::Client,
    url: String,
}

impl Client {
    pub fn new(url: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            url: url,
        }
    }
}

impl Client {
    pub async fn query<V: Serialize, D: DeserializeOwned>(
        &self,
        query: QueryBody<V>,
    ) -> Result<D, eyre::Error> {
        let response = self
            .client
            .post(self.url.clone())
            .json(&query)
            .send()
            .await?;
        let body: Response<D> = response.json().await?;
        body.data
            .ok_or(eyre::eyre!("missing response data for query"))
    }
}
