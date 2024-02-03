
use anyhow::Result;

use super::{mongodb_configuration::MongoDbConfiguration, sql_configuration::SqlConfig};
pub(crate) trait ConfigSaver {
    async fn store(&mut self) -> Result<()>;
}

pub (crate) trait ConfigGetter {
    async fn retrieve(&mut self, datasource_id: &str) -> Result<DatasourceConfiguration>;
}

#[derive(Debug)]
pub enum DatasourceConfiguration {
    Sql(SqlConfig),
    MongoDb(MongoDbConfiguration)
}
