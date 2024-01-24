use rocket::{tokio::sync::RwLock, State};

use crate::modules::dmodel::infrastructure::persistence::model_getter::ModelRetriever;
use crate::modules::query::model::query_builder::Builder;
use crate::modules::{
    datasource::model::{
        configurations::{configurations::DatasourceConfiguration, sql_configuration::SqlConfig},
        sql_dialect::SqlDialect,
    },
    dmodel::model::model::Model,
    query::model::{
        query_builder::sql_builder::{postgres_builder::PostgresDialect, SqlQueryBuilder},
        query_executor::{sql_executor::postgres_executor::PostgresExecutor, Executor},
    },
    shared::shared_state::shared_connections::SharedConnections,
};
use anyhow::{bail, Result};

pub(crate) async fn postgres_runner_factory(
    config: SqlConfig,
    state: &State<RwLock<SharedConnections>>,
    model: Model,
) -> Result<(SqlQueryBuilder<PostgresDialect>, Executor)> {
    let client = SharedConnections::get_pg_client(state, &config).await?;
    let builder = SqlQueryBuilder {
        dialect: PostgresDialect {
            model,
            schema: config
                .schema
                .expect("Schema should be defined for posgres datamodels"),
        },
    };
    let query_executor = PostgresExecutor::new(client);
    let executor = Executor::Pg(query_executor);

    Ok((builder, executor))
}

/* pub(crate) async fn mssql_runner_factory(
    config: SqlConfig,
    state: &State<RwLock<SharedConnections>>,
    model: Model,
) -> Result<(SqlQueryBuilder<MssqlDialect>, Executor)> {
    let client = SharedData::get_mssql_client(state, &config).await?;
    let builder = SqlQueryBuilder {
        dialect: MssqlDialect {
            model,
            schema: config
                .schema
                .expect("Schema should be defined for posgres datamodels"),
        },
    };
    let query_executor = MssqlExecutor::new(client);
    let executor = Executor::Mssql(query_executor);
    Ok((builder, executor))
} */

pub(crate) async fn query_runner_factory(
    config: DatasourceConfiguration,
    state: &State<RwLock<SharedConnections>>,
    mut model_retriever: ModelRetriever<'_>,
    model_id: &str,
) -> Result<(Builder, Executor)> {
    let model = model_retriever.retrieve(model_id).await?;

    match config {
        DatasourceConfiguration::Sql(config) => match config.dialect {
            SqlDialect::Postgresql => {
                let pg = postgres_runner_factory(config, state, model).await;
                match pg {
                    Ok((_builder, _executor)) => {
                        let builder = _builder;
                        let executor = _executor;
                        return Ok((Builder::PgBuilder(builder), executor));
                    }
                    Err(e) => {
                        bail!(e)
                    }
                }
            } /*             SqlDialect::Mssql => {
                  let mssql = mssql_runner_factory(config, state, model).await;
                  match mssql {
                      Ok((_builder, _executor)) => {
                          let builder = _builder;
                          let executor = _executor;
                          return Ok((Builder::MssqlBuilder(builder), executor));
                      }
                      Err(e) => {
                          bail!(e)
                      }
                  }
              } */
        },
    };
}
