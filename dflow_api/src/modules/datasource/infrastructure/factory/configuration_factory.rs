use crate::{
    modules::datasource::{
        infrastructure::persistence::{
            datasource_getter::DataSourceGetter,
            mongo_db_config_getter::MongoDbConfigurationGetter,
            sql_config_getter::SqlConfigurationGetter,
        },
        model::{
            configurations::configurations::{ConfigGetter, DatasourceConfiguration},
            datasource_repository::TDataSourceGetter,
            datasource_type::DataSourceType,
        },
    },
    Db,
};

use anyhow::Result;
use rocket_db_pools::Connection;

/// Utility factory to retrieve and narrow the configuration type for a given datasource.
/// Returns the narrowed datasource configuration.
///
/// # Arguments
///
/// - `datasource_id`: The identifier of the datasource for which configuration is to be retrieved.
/// - `db`: A mutable reference to the database connection.
///
/// # Returns
///
/// Returns a `Result` containing the narrowed `DatasourceConfiguration` or an error if retrieval fails.
///
/// # Errors
///
/// This function may return an error if there are issues retrieving or narrowing the configuration.
///
pub(crate) async fn configuration_factory(
    datasource_id: &str,
    db: &mut Connection<Db>,
) -> Result<DatasourceConfiguration> {
    let mut datasource_getter = DataSourceGetter::new(db);
    let datasource = datasource_getter
        .get_datasource_by_id(datasource_id)
        .await
        .unwrap();

    let model_configuration;

    match datasource.datasource_type {
        DataSourceType::Sql(_) => {
            let mut config_retriever = SqlConfigurationGetter::new(db);
            model_configuration = config_retriever.retrieve(datasource_id).await.unwrap();
        }
        DataSourceType::MongoDb => {
            let mut config_retriever = MongoDbConfigurationGetter::new(db);
            model_configuration = config_retriever.retrieve(datasource_id).await.unwrap();
        }
    }
    Ok(model_configuration)
}
