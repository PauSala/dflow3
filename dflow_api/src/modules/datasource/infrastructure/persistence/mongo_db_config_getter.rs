use crate::{
    modules::{datasource::model::configurations::{
        configurations::{ConfigGetter, DatasourceConfiguration},
        mongodb_configuration::MongoDbConfig,
    }, shared::security::decrypt_service},
    Db,
};
use anyhow::Result;
use rocket_db_pools::sqlx::Row;
use rocket_db_pools::{sqlx, Connection};

pub struct MongoDbConfigurationGetter<'a> {
    pub db: &'a mut Connection<Db>,
}

impl<'a> MongoDbConfigurationGetter<'a> {
    pub(crate) fn new(db: &'a mut Connection<Db>) -> Self {
        MongoDbConfigurationGetter { db }
    }
}

impl<'a> ConfigGetter for MongoDbConfigurationGetter<'a> {
    async fn retrieve(&mut self, datasource_id: &str) -> Result<DatasourceConfiguration> {
        let result = sqlx::query(
            "SELECT 
             datasource_id, conn_string, db_name
             FROM mongodb_configurations 
             WHERE datasource_id = ? ",
        )
        .bind(datasource_id)
        .map(|row| {
            let conn_string: String = row.get(1);
            MongoDbConfig::new(row.get(0), decrypt_service(&conn_string), row.get(2))
        })
        .fetch_one(&mut ***self.db)
        .await?;
        Ok(DatasourceConfiguration::MongoDb(result))
    }
}
