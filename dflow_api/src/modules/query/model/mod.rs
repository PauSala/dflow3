pub mod query_builder;
pub mod query_executor;
use crate::modules::{
    datasource::model::configurations::sql_configuration::SqlConfig, dmodel::model::model::Model,
    shared::shared_state::shared_connections::SharedConnections,
};

use self::{
    query_builder::{
        abstract_query::AbstractQuery,
        sql_builder::{
            mssql_builder::MssqlDialect, postgres_builder::PostgresDialect, SqlQueryBuilder,
        },
        TQueryBuilder,
    },
    query_executor::{
        sql_executor::{mssql_executor::MssqlExecutor, postgres_executor::PostgresExecutor},
        QueryResult, TQueryExecutor,
    },
};
use anyhow::Result;
use rocket::{tokio::sync::RwLock, State};

pub(crate) struct QueryHandler<B, E>
where
    B: TQueryBuilder,
    E: TQueryExecutor<Input = B::Output>,
{
    builder: B,
    executor: E,
}

impl<B, E> QueryHandler<B, E>
where
    B: TQueryBuilder,
    E: TQueryExecutor<Input = B::Output>,
{
    pub fn new(builder: B, executor: E) -> Self {
        QueryHandler { builder, executor }
    }
    pub async fn handle(&mut self, query: &AbstractQuery<'_>) -> Result<QueryResult> {
        let q = self.builder.build(query);
        let result = self.executor.run(q, query).await?;
        Ok(result)
    }
}

pub(crate)  async fn postgres_query_handler(
    state: &State<RwLock<SharedConnections>>,
    model: Model,
    config: SqlConfig,
) -> Result<QueryHandler<SqlQueryBuilder<PostgresDialect>, PostgresExecutor>> {
    let client = SharedConnections::get_pg_client(state, &config).await?;
    let builder = SqlQueryBuilder {
        dialect: PostgresDialect {
            model,
            schema: config
                .schema
                .expect("Schema should be defined for posgres datamodels"),
        },
    };
    let executor = PostgresExecutor::new(client);
    Ok(QueryHandler::new(builder, executor))
}

pub(crate) async fn mssql_query_handler(
    state: &State<RwLock<SharedConnections>>,
    model: Model,
    config: SqlConfig,
) -> Result<QueryHandler<SqlQueryBuilder<MssqlDialect>, MssqlExecutor>> {
    let client = SharedConnections::get_mssql_client(state, &config).await?;
    let builder = SqlQueryBuilder {
        dialect: MssqlDialect {
            model,
            schema: config
                .schema
                .expect("Schema should be defined for posgres datamodels"),
        },
    };
    let executor = MssqlExecutor::new(client);
    Ok(QueryHandler::new(builder, executor))
}
