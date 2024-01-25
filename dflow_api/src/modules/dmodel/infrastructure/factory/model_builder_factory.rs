use crate::{
    modules::{
        datasource::{
            infrastructure::factory::configuration_factory::configuration_factory,
            model::{
                configurations::configurations::DatasourceConfiguration, sql_dialect::SqlDialect,
            },
        },
        dmodel::model::model_builder::{
            sql_model_builder::{
                mssql_model_builder::MssqlModelBuilder, postgres_model_builder::PosgtresModelBuilder, SqlBuilderDialect, SqlModelBuilder
            },
            ModelBuilder,
        },
        shared::shared_state::shared_connections::SharedConnections,
    },
    Db,
};
use anyhow::Result;
use rocket::{tokio::sync::RwLock, State};
use rocket_db_pools::Connection;

pub(crate) async fn model_builder_factory(
    datasource_id: &str,
    db: &mut Connection<Db>,
    shared_cns: &State<RwLock<SharedConnections>>,
) -> Result<ModelBuilder> {
    let model_configuration = configuration_factory(datasource_id, db).await?;

    let model_builder;
    match model_configuration {
        DatasourceConfiguration::Sql(config) => match config.dialect {
            SqlDialect::Postgresql => {
                let client = SharedConnections::get_pg_client(shared_cns, &config).await?;
                let cn = SqlBuilderDialect::Postgresql(PosgtresModelBuilder::new(client, config));
                model_builder = SqlModelBuilder::new(cn);
                Ok(ModelBuilder::Sql(model_builder))
            }
            SqlDialect::Mssql => {
                let client = SharedConnections::get_mssql_client(shared_cns, &config).await?;
                let cn = SqlBuilderDialect::Mssql(MssqlModelBuilder::new(client, config));
                model_builder = SqlModelBuilder::new(cn);
                Ok(ModelBuilder::Sql(model_builder))
            }
        },
    }
}
