use dotenv_codegen::dotenv;
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use crate::modules::datasource::model::configurations::configurations::{
    ConfigGetter, DatasourceConfiguration,
};
use crate::modules::datasource::model::configurations::sql_configuration::SqlConfig;
use crate::modules::datasource::model::sql_dialect::SqlDialect;
use crate::Db;
use anyhow::Result;
use rocket_db_pools::sqlx::{self, Row};
use rocket_db_pools::Connection;

static SECRET_KEY: &'static str = dotenv!("SECRET_KEY");

pub struct SqlConfigurationGetter<'a> {
    pub db: &'a mut Connection<Db>,
}

impl<'a> SqlConfigurationGetter<'a> {
    pub(crate) fn new(db: &'a mut Connection<Db>) -> Self {
        SqlConfigurationGetter { db }
    }
}

impl<'a> ConfigGetter for SqlConfigurationGetter<'a> {
    async fn retrieve(&mut self, datasource_id: &str) -> Result<DatasourceConfiguration> {
        let mc = new_magic_crypt!(SECRET_KEY, 256);
        let result = sqlx::query(
            "SELECT 
             datasource_id, host, port, user ,password, db_name, schema, dialect
             FROM sql_configurations 
             WHERE datasource_id = ? ",
        )
        .bind(datasource_id)
        .map(|row| {
            let dialect: String = row.get(7);
            let base64_pwd: String = row.get(4);
            SqlConfig::new(
                row.get(0),
                row.get(1),
                row.get(2),
                row.get(3),
                &mc.decrypt_base64_to_string(base64_pwd).expect("msg"),
                row.get(5),
                row.get(6),
                SqlDialect::from_string(&dialect),
            )
        })
        .fetch_one(&mut ***self.db)
        .await?;
        Ok(DatasourceConfiguration::Sql(result))
    }
}
