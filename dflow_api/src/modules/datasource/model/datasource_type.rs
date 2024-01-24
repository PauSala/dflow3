use serde::Deserialize;

use super::sql_dialect::SqlDialect;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub enum DataSourceType {
    Sql(SqlDialect),
}

impl DataSourceType {
    pub fn to_string(&self) -> String {
        match self {
            DataSourceType::Sql(dialect) => dialect.to_string(),
        }
    }
    pub fn from_string(v: &str) -> Self {
        match v {
            "postgresql" => DataSourceType::Sql(SqlDialect::Postgresql),
            /* "mssql" => DataSourceType::Sql(SqlDialect::Mssql), */
            _ => panic!("{}", format!("Unknown datasource type: {}", v))
        }
    }
}
pub struct DataSource {
    pub id: String,
    pub name: String,
    pub datasource_type: DataSourceType,
}

impl DataSource {
    pub fn new(id: String, name: String, datasource_type: DataSourceType) -> Self {
        Self { id, name, datasource_type }
    }
}
