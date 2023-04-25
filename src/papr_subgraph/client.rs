use crate::graphql;
use graphql_client::QueryBody;
use once_cell::sync::Lazy;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::env;

static SUBGRAPH_URL: Lazy<String> =
    Lazy::new(|| env::var("PAPR_SUBGRAPH_URL").expect("PAPR_SUBGRAPH_URL not set"));
pub struct Client {
    client: graphql::Client,
}

impl Default for Client {
    fn default() -> Self {
        Self {
            client: graphql::Client::new(SUBGRAPH_URL.clone()),
        }
    }
}

impl Client {
    pub async fn query<V: Serialize, D: DeserializeOwned>(
        &self,
        query: QueryBody<V>,
    ) -> Result<D, eyre::Error> {
        Ok(self.client.query::<V, D>(query).await?)
    }
}
