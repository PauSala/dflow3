use crate::{
    modules::{
        datasource::{infrastructure::persistence::sql_config_getter::SqlConfigurationGetter, model::{
            configurations::configurations::{ConfigGetter, DatasourceConfiguration}, datasource_type::DataSourceType, sql_dialect::SqlDialect
        }},
        dmodel::model::model_builder::{sql_model_builder::{postgres_model_builder::PosgtresModelBuilder, SqlBuilderDialect, SqlModelBuilder}, ModelBuilder},
        shared::shared_state::shared_connections::SharedConnections,
    },
    Db,
};
use anyhow::Result;
use rocket::{tokio::sync::RwLock, State};
use rocket_db_pools::Connection;

pub(crate) async fn model_builder_factory(
    datasource_type: DataSourceType,
    datasource_id: &str,
    db: &mut Connection<Db>,
    shared_cns: &State<RwLock<SharedConnections>>,
) -> Result<ModelBuilder> {
    let model_configuration: DatasourceConfiguration;
    match datasource_type {
        DataSourceType::Sql(_) => {
            let mut config_getter = SqlConfigurationGetter::new(db);
            model_configuration = config_getter.retrieve(datasource_id).await?;
        }
    }

    let model_builder;
    match model_configuration {
        DatasourceConfiguration::Sql(config) => match config.dialect {
            SqlDialect::Postgresql => {
                let client = SharedConnections::get_pg_client(shared_cns, &config)
                    .await
                    .unwrap();
                let cn = SqlBuilderDialect::Postgresql(PosgtresModelBuilder::new(client, config));
                model_builder = SqlModelBuilder::new(cn);
                Ok(ModelBuilder::Sql(model_builder))
            } /*             SqlDialect::Mssql => {
                  let client = SharedConnections::get_mssql_client(shared_cns, &config)
                      .await
                      .unwrap();
                  let cn = SqlBuilderDialect::Mssql(MssqlModelBuilder::new(client, config));
                  model_builder = SqlModelBuilder::new(cn);
                  Ok(ModelBuilder::Sql(model_builder))
              } */
        },
    }
}