use crate::papr_subgraph::client;
use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/papr_subgraph/graphql/paprSchema.graphql",
    query_path = "src/papr_subgraph/graphql/controller_by_id.graphql"
)]
pub struct ControllerById;

impl client::Client {
    pub async fn controller_by_id(
        &self,
        controller: String,
    ) -> Result<Option<controller_by_id::ControllerByIdPaprController>, eyre::Error> {
        use controller_by_id::*;
        let variables = Variables { id: controller };
        let query = ControllerById::build_query(variables);
        Ok(self.query::<_, ResponseData>(query).await?.papr_controller)
    }
}
