use crate::modules::datasource::model::configurations::configurations::{
    ConfigGetter, DatasourceConfiguration,
};
use crate::modules::datasource::model::configurations::sql_configuration::SqlConfig;
use crate::modules::datasource::model::sql_dialect::SqlDialect;
use crate::modules::shared::security::decrypt_service;
use crate::Db;
use anyhow::Result;
use rocket_db_pools::sqlx::{self, Row};
use rocket_db_pools::Connection;



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
                &decrypt_service(&base64_pwd),
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
