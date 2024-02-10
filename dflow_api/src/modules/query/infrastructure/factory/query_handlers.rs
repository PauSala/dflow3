use mongodb::options::ClientOptions;
use rocket::{tokio::sync::RwLock, State};

use crate::modules::datasource::model::configurations::mongodb_configuration::MongoDbConfig;
use crate::modules::datasource::model::configurations::sql_configuration::SqlConfig;
use crate::modules::dmodel::infrastructure::persistence::model_getter::ModelGetter;
use crate::modules::dmodel::model::model::Model;
use crate::modules::query::application::QueryHandler;
use crate::modules::query::model::query_builder::abstract_query::AbstractQuery;
use crate::modules::query::model::query_builder::mongodb_builder::MongoDbBuilder;
use crate::modules::query::model::query_builder::sql_builder::mssql_builder::MssqlDialect;
use crate::modules::query::model::query_builder::sql_builder::postgres_builder::PostgresDialect;
use crate::modules::query::model::query_builder::sql_builder::SqlQueryBuilder;
use crate::modules::query::model::query_runner::mongo_db::MongoDbRunner;
use crate::modules::query::model::query_runner::sql::mssql_runner::MssqlRunner;
use crate::modules::query::model::query_runner::sql::postgres_runner::PostgresRunner;
use crate::modules::query::model::query_runner::ShapedResult;

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
    query: &AbstractQuery<'_>,
) -> Result<ShapedResult> {
    let model = model_retriever.retrieve(query.model_id).await?;
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
        DatasourceConfiguration::MongoDb(config) => {
            let mut handler = mongo_query_handler(model, config).await?;
            result = handler.handle(query).await?;
        }
    };
    return Ok(result);
}

pub(crate) async fn mongo_query_handler(
    model: Model,
    config: MongoDbConfig,
) -> Result<QueryHandler<MongoDbBuilder, MongoDbRunner>> {
    use mongodb::Client;
    let mut client_options = ClientOptions::parse(&config.conn_string).await?;
    client_options.app_name = Some("DFLOW".to_string());
    let client = Client::with_options(client_options)?;

    let builder = MongoDbBuilder::new(model);
    let runner = MongoDbRunner::new(client, config);
    Ok(QueryHandler::new(builder, runner))
}

pub(crate) async fn postgres_query_handler(
    state: &State<RwLock<SharedConnections>>,
    model: Model,
    config: SqlConfig,
) -> Result<QueryHandler<SqlQueryBuilder<PostgresDialect>, PostgresRunner>> {
    let client = SharedConnections::get_pg_client(state, &config).await?;
    let builder = SqlQueryBuilder {
        dialect: PostgresDialect {
            model,
            schema: config
                .schema
                .expect("Schema should be defined for posgres datamodels"),
        },
    };
    let executor = PostgresRunner::new(client);
    Ok(QueryHandler::new(builder, executor))
}

pub(crate) async fn mssql_query_handler(
    state: &State<RwLock<SharedConnections>>,
    model: Model,
    config: SqlConfig,
) -> Result<QueryHandler<SqlQueryBuilder<MssqlDialect>, MssqlRunner>> {
    let client = SharedConnections::get_mssql_client(state, &config).await?;
    let builder = SqlQueryBuilder {
        dialect: MssqlDialect {
            model,
            schema: config
                .schema
                .expect("Schema should be defined for mssql datamodels"),
        },
    };
    let executor = MssqlRunner::new(client);
    Ok(QueryHandler::new(builder, executor))
}
