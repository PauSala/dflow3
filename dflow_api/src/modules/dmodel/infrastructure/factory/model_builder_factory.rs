use crate::{
    modules::{
        datasource::{
            infrastructure::factory::configuration_factory::configuration_factory,
            model::{
                configurations::{
                    configurations::DatasourceConfiguration,
                    mongodb_configuration::MongoDbConfiguration,
                },
                sql_dialect::SqlDialect,
            },
        },
        dmodel::model::model_builder::{
            mongodb_model_builder::MongoDbBuilder,
            sql_model_builder::{
                mssql_model_builder::MssqlModelBuilder,
                postgres_model_builder::PosgtresModelBuilder, SqlBuilderDialect, SqlModelBuilder,
            },
            ModelBuilder,
        },
        shared::shared_state::shared_connections::SharedConnections,
    },
    Db,
};
use anyhow::Result;
use mongodb::{options::ClientOptions, Client};
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
        DatasourceConfiguration::MongoDb(_) => {
            let mut client_options = ClientOptions::parse("mongodb://localhost:27017")
                .await
                .unwrap();

            // Manually set an option.
            client_options.app_name = Some("DFLOW".to_string());

            // Get a handle to the deployment.
            let client = Client::with_options(client_options).unwrap();
            let b = MongoDbBuilder::new(
                MongoDbConfiguration {
                    datasource_id: "".to_owned(),
                    conn_string: "mongodb://localhost:27017".to_owned(),
                    db_name: "DFLOW".to_owned(),
                },
                client,
            );
            Ok(ModelBuilder::MongoDb(b))
        }
    }
}
