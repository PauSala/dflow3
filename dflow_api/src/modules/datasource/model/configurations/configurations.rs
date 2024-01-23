
use anyhow::Result;

use super::sql_configuration::SqlConfig;
pub(crate) trait ConfigSaver {
    async fn store(&mut self) -> Result<()>;
}

pub (crate) trait ConfigGetter {
    async fn retrieve(&mut self, datasource_id: &str) -> Result<DatasourceConfiguration>;
}

#[derive(Debug)]
pub enum DatasourceConfiguration {
    Sql(SqlConfig)
}
