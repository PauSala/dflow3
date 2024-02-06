use crate::{
    modules::{
        datasource::model::configurations::{
            configurations::ConfigSaver, mongodb_configuration::MongoDbConfig,
        },
        shared::security::encrypt_service,
    },
    Db,
};
use anyhow::Result;
use rocket_db_pools::{sqlx, Connection};

pub struct MongoDbConfigurationSaver<'a> {
    db: &'a mut Connection<Db>,
    config: &'a MongoDbConfig,
}

impl<'a> MongoDbConfigurationSaver<'a> {
    pub fn new(db: &'a mut Connection<Db>, config: &'a MongoDbConfig) -> Self {
        Self { db, config }
    }
}

impl<'a> ConfigSaver for MongoDbConfigurationSaver<'a> {
    async fn store(&mut self) -> Result<()> {
        let conn_string = encrypt_service(&self.config.conn_string);
        sqlx::query(
            "INSERT INTO mongodb_configurations (datasource_id, conn_string, db_name)
                 VALUES (?1, ?2, ?3)",
        )
        .bind(&self.config.datasource_id)
        .bind(conn_string)
        .bind(&self.config.db_name)
        .execute(&mut ***self.db)
        .await?;
        Ok(())
    }
}
