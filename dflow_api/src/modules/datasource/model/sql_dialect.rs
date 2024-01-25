use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub enum SqlDialect {
    Postgresql,
    Mssql
}

impl SqlDialect {
    pub fn to_string(&self) -> String {
        match self {
            SqlDialect::Postgresql => "postgresql".to_string(),
            SqlDialect::Mssql => "mssql".to_string(),
        }
    }
    pub fn from_string(v: &str) -> Self {
        match v {
            "postgresql" => SqlDialect::Postgresql,
         /*    "mssql" => SqlDialect::Mssql, */
            _ => panic!("Unknown sqlDialect"),
        }
    }
}
