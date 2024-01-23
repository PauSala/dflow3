use serde::Deserialize;

use crate::modules::datasource::model::sql_dialect::SqlDialect;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct SqlConfig {
    pub datasource_id: String,
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub db_name: Option<String>,
    pub schema: Option<String>,
    pub dialect: SqlDialect,
}

impl SqlConfig {
    pub fn new(
        datasource_id: &str,
        host: &str,
        port: u16,
        user: &str,
        password: &str,
        db_name: Option<String>,
        schema: Option<String>,
        dialect: SqlDialect,
    ) -> Self {
        SqlConfig {
            datasource_id: datasource_id.to_string(),
            host: host.to_string(),
            port,
            user: user.to_string(),
            password: password.to_string(),
            db_name,
            schema,
            dialect,
        }
    }
}
