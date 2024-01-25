use anyhow::Result;
use rocket_db_pools::{sqlx, Connection};

use crate::{
    modules::{
        datasource::model::configurations::{
            configurations::ConfigSaver, sql_configuration::SqlConfig,
        },
        shared::security::encrypt_service,
    },
    Db,
};

pub struct SqlConfigurationSaver<'a> {
    db: &'a mut Connection<Db>,
    config: &'a SqlConfig,
}

impl<'a> ConfigSaver for SqlConfigurationSaver<'a> {
    async fn store(&mut self) -> Result<()> {
        let password = encrypt_service(&self.config.password);
        sqlx::query(
                "INSERT INTO sql_configurations (datasource_id, host, port, user, password, db_name, schema, dialect)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)"
            )
            .bind(&self.config.datasource_id)
            .bind(&self.config.host)
            .bind(&self.config.port)
            .bind(&self.config.user)
            .bind(password)
            .bind(&self.config.db_name)
            .bind(&self.config.schema)
            .bind(&self.config.dialect.to_string())
            .execute(&mut ***self.db)
            .await?;
        Ok(())
    }
}

impl<'a> SqlConfigurationSaver<'a> {
    pub fn new(db: &'a mut Connection<Db>, config: &'a SqlConfig) -> Self {
        Self { db, config }
    }
}
