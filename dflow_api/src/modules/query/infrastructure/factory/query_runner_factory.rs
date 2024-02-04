use rocket::{tokio::sync::RwLock, State};

use crate::modules::dmodel::infrastructure::persistence::model_getter::ModelGetter;
use crate::modules::query::model::mssql_query_handler;
use crate::modules::query::model::postgres_query_handler;
use crate::modules::query::model::query_builder::abstract_query::AbstractQuery;
use crate::modules::query::model::query_executor::QueryResult;
use crate::modules::{
    datasource::model::{
        configurations::configurations::DatasourceConfiguration, sql_dialect::SqlDialect,
    },
    shared::shared_state::shared_connections::SharedConnections,
};
use anyhow::Result;

pub(crate) async fn handle_query(
    config: DatasourceConfiguration,
    state: &State<RwLock<SharedConnections>>,
    mut model_retriever: ModelGetter<'_>,
    model_id: &str,
    query: &AbstractQuery<'_>,
) -> Result<QueryResult> {
    let model = model_retriever.retrieve(model_id).await?;
    let result;
    match config {
        DatasourceConfiguration::Sql(config) => match config.dialect {
            SqlDialect::Postgresql => {
                let mut handler = postgres_query_handler(state, model, config).await?;
                result = handler.handle(query).await?;
            }
            SqlDialect::Mssql => {
                let mut handler = mssql_query_handler(state, model, config).await?;
                result = handler.handle(query).await?;
            }
        },
        DatasourceConfiguration::MongoDb(_) => todo!(),
    };
    return Ok(result);
}
