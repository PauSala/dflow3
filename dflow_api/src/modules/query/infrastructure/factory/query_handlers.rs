use mongodb::bson::{self, Document};
use mongodb::options::ClientOptions;
use mongodb::Collection;
use rocket::futures::StreamExt;
use rocket::{tokio::sync::RwLock, State};

use crate::modules::datasource::model::configurations::sql_configuration::SqlConfig;
use crate::modules::dmodel::infrastructure::persistence::model_getter::ModelGetter;
use crate::modules::dmodel::model::model::Model;
use crate::modules::query::application::QueryHandler;
use crate::modules::query::model::query_builder::abstract_query::AbstractQuery;
use crate::modules::query::model::query_builder::mongodb_builder::MongoDbBuilder;
use crate::modules::query::model::query_builder::sql_builder::mssql_builder::MssqlDialect;
use crate::modules::query::model::query_builder::sql_builder::postgres_builder::PostgresDialect;
use crate::modules::query::model::query_builder::sql_builder::SqlQueryBuilder;
use crate::modules::query::model::query_builder::QueryBuilder;
use crate::modules::query::model::query_executor::sql_executor::mssql_executor::MssqlRunner;
use crate::modules::query::model::query_executor::sql_executor::postgres_executor::PostgresRunner;
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
    query: &AbstractQuery<'_>,
) -> Result<QueryResult> {
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
        DatasourceConfiguration::MongoDb(_) => {
            let builder = MongoDbBuilder::new(model);
            let pipeline = builder.build(query);
            dbg!(&pipeline);
            let mut client_options = ClientOptions::parse("mongodb://localhost:27017")
                .await
                .unwrap();
            client_options.app_name = Some("DFLOW".to_string());
            use mongodb::Client;
            let client = Client::with_options(client_options).unwrap();
            let btt: Collection<Document> = client.database("DFLOW").collection("customers");
            let mut results = btt.aggregate(pipeline, None).await?;
            while let Some(result) = results.next().await {
                // Use serde to deserialize into the MovieSummary struct:
                let doc: Document = bson::from_document(result?)?;
                println!("* {}", doc);
             }
            result = QueryResult {
                columns: vec![],
                data: vec![],
            }
        }
    };
    return Ok(result);
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
