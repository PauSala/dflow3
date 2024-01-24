use anyhow::Result;

use super::datasource_type::DataSource;
pub(crate) trait TDataSourceGetter {
    async fn get_datasource_by_id(&mut self, datasource_id: &str) -> Result<DataSource>;
}
