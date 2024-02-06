use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct MongoDbConfig {
    pub datasource_id: String,
    pub conn_string: String,
    pub db_name: String,
}

impl MongoDbConfig {
    pub fn new(datasource_id: String, conn_string: String, db_name: String) -> Self {
        Self {
            datasource_id,
            conn_string,
            db_name,
        }
    }
}
