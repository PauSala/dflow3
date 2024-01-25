use anyhow::Result;
use rocket_db_pools::{sqlx, Connection};
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use dotenv_codegen::dotenv;
use crate::{modules::datasource::model::configurations::{configurations::ConfigSaver, sql_configuration::SqlConfig}, Db};


pub struct SqlConfigurationSaver<'a> {
    db: &'a mut Connection<Db>,
    config: &'a SqlConfig,
}

impl<'a> ConfigSaver for SqlConfigurationSaver<'a> {
    async fn store(&mut self) -> Result<()> {
        let mc = new_magic_crypt!(dotenv!("SECRET_KEY"), 256);
        let password = mc.encrypt_str_to_base64(&self.config.password);
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
