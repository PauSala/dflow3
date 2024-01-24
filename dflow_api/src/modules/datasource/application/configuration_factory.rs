use crate::{
    modules::datasource::{
        infrastructure::persistence::{
            datasource_getter::DataSourceGetter, sql_config_getter::SqlConfigurationGetter,
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

///
/// Utility factory to narrow configuration type for given datasource
/// Returns narrowed datasource configuration
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

    let mut config_retriever;

    match datasource.datasource_type {
        DataSourceType::Sql(_) => config_retriever = SqlConfigurationGetter::new(db),
    }

    let model_configuration = config_retriever.retrieve(datasource_id).await.unwrap();
    Ok(model_configuration)
}
