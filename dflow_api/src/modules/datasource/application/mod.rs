
use anyhow::{Ok, Result};

use super::{
    infrastructure::persistence::datasource_saver::DataSourceSaver,
    model::{configurations::configurations::ConfigSaver, datasource_type::DataSource},
};
pub struct ConfigSaverService;

impl ConfigSaverService {
    pub(crate) fn new() -> Self {
        ConfigSaverService {}
    }

    pub(crate) async fn run<'a, Storer: ConfigSaver>(
        &mut self,
        mut config_storer: Storer,
    ) -> Result<()> {
        config_storer.store().await?;
        Ok(())
    }
}

pub struct DatasourceSaverService {}

impl DatasourceSaverService {
    pub(crate) fn new() -> Self {
        DatasourceSaverService {}
    }
    pub(crate) async fn run<'a>(
        &mut self,
        datasource: DataSource,
        datastorer: &'a mut DataSourceSaver<'a>,
    ) -> Result<()> {
        datastorer.persist(&datasource).await
    }
}
