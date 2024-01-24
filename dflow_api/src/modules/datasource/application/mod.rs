use anyhow::{Ok, Result};

use super::{
    infrastructure::persistence::datasource_saver::TDataSourceSaver,
    model::{
        configurations::configurations::ConfigSaver,
        datasource_type::{DataSource, DataSourceType},
    },
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

pub struct DatasourceSaverService;

impl DatasourceSaverService {
    pub(crate) fn new() -> Self {
        DatasourceSaverService {}
    }
    pub(crate) async fn run<'a, Saver: TDataSourceSaver>(
        &mut self,
        datasource_id: &'a str,
        datasource_name: &'a str,
        datasource_type: DataSourceType,
        datastorer: &mut Saver,
    ) -> Result<()> {
        let datasource = DataSource::new(
            datasource_id.to_owned(),
            datasource_name.to_owned(),
            datasource_type.clone(),
        );
        datastorer.persist(&datasource).await
    }
}
