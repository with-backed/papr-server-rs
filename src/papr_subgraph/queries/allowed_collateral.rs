use crate::papr_subgraph::client;
use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/papr_subgraph/graphql/paprSchema.graphql",
    query_path = "src/papr_subgraph/graphql/allowed_collateral.graphql"
)]
pub struct AllowedCollateralForControllers;

impl client::Client {
    pub async fn allowed_collateral_for_controllers(
        &self,
        controllers: Vec<String>,
    ) -> Result<
        Vec<allowed_collateral_for_controllers::AllowedCollateralForControllersAllowedCollaterals>,
        eyre::Error,
    > {
        use allowed_collateral_for_controllers::*;
        let variables = Variables {
            controllers: Some(controllers),
        };
        let query = AllowedCollateralForControllers::build_query(variables);
        Ok(self
            .query::<_, ResponseData>(query)
            .await?
            .allowed_collaterals)
    }
}
