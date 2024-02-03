use serde::Deserialize;

use super::sql_dialect::SqlDialect;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub enum DataSourceType {
    Sql(SqlDialect),
    MongoDb
}

impl DataSourceType {
    pub fn to_string(&self) -> String {
        match self {
            DataSourceType::Sql(dialect) => dialect.to_string(),
            DataSourceType::MongoDb => "mongodb".to_owned(),
        }
    }
    pub fn from_string(v: &str) -> Self {
        match v {
            "postgresql" => DataSourceType::Sql(SqlDialect::Postgresql),
            "mssql" => DataSourceType::Sql(SqlDialect::Mssql),
            "mongodb" => DataSourceType::MongoDb,
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
