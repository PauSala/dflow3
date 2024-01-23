use anyhow::Result;
use rocket_db_pools::{sqlx, Connection};

use crate::{modules::datasource::model::configurations::{configurations::ConfigSaver, sql_configuration::SqlConfig}, Db};

pub struct SqlConfigurationSaver<'a> {
    db: &'a mut Connection<Db>,
    config: SqlConfig,
}

impl<'a> ConfigSaver for SqlConfigurationSaver<'a> {
    async fn store(&mut self) -> Result<()> {
        sqlx::query(
                "INSERT INTO sql_configurations (datasource_id, host, port, user, password, db_name, schema, dialect)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)"
            )
            .bind(self.config.datasource_id.clone())
            .bind(self.config.host.clone())
            .bind(self.config.port)
            .bind(self.config.user.clone())
            .bind(self.config.password.clone())
            .bind(self.config.db_name.clone())
            .bind(self.config.schema.clone())
            .bind(self.config.dialect.to_string())
            .execute(&mut ***self.db)
            .await?;
        Ok(())
    }
}

impl<'a> SqlConfigurationSaver<'a> {
    pub fn new(db: &'a mut Connection<Db>, config: SqlConfig) -> Self {
        Self { db, config }
    }
}
