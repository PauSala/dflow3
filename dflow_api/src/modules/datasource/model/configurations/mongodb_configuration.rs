use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct MongoDbConfiguration{
    pub datasource_id: String,
    pub conn_string: String,
    pub db_name: String
}
