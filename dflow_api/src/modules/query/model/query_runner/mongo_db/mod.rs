use anyhow::Result;
use mongodb::{
    bson::{self, Document},
    Client, Collection,
};
use rocket::futures::StreamExt;

use crate::modules::{
    datasource::model::configurations::mongodb_configuration::MongoDbConfig,
    query::model::query_builder::{abstract_query::AbstractQuery, mongodb_builder::MongoDbQuery},
};

use super::ColumnReturnDataType;

pub struct MongoDbRunner {
    client: Client,
    config: MongoDbConfig,
}

impl MongoDbRunner {
    pub fn new(client: Client, config: MongoDbConfig) -> Self {
        MongoDbRunner { client, config }
    }

    pub async fn run_query(
        &self,
        query: MongoDbQuery,
        _abstract_query: &AbstractQuery<'_>,
    ) -> Result<(Vec<String>, Vec<Vec<ColumnReturnDataType>>)> {
        let database = &self.config.db_name;
        dbg!(&query.pipeline);
        let btt: Collection<Document> = self
            .client
            .database(database)
            .collection(&query.main_document);
        let mut results = btt.aggregate(query.pipeline, None).await?;
        let mut rows: Vec<Vec<ColumnReturnDataType>> = vec![];
        while let Some(result) = results.next().await {
            let mut row: Vec<ColumnReturnDataType> = vec![];
            let doc: Document = bson::from_document(result?)?;
            for (_, value) in doc {

                match value {
                    bson::Bson::Double(v) => row.push(ColumnReturnDataType::Number(Some(v))),
                    bson::Bson::String(v) => row.push(ColumnReturnDataType::Text(Some(v))),
                    bson::Bson::Boolean(v) => row.push(ColumnReturnDataType::Bool(Some(v))),
                    bson::Bson::Null => row.push(ColumnReturnDataType::Text(None)),
                    bson::Bson::Int32(v) => row.push(ColumnReturnDataType::Number(Some(v.into()))),
                    bson::Bson::Int64(v) => {
                        row.push(ColumnReturnDataType::Text(Some(v.to_string())))
                    } //TO DO!
                    bson::Bson::Decimal128(v) => {
                        row.push(ColumnReturnDataType::Text(Some(v.to_string())))
                    } //TO DO!
                    bson::Bson::Binary(v) => {
                        row.push(ColumnReturnDataType::Text(Some(v.to_string())))
                    } //TO DO!
                    bson::Bson::Timestamp(v) => {
                        row.push(ColumnReturnDataType::Date(Some(v.to_string())))
                    }
                    bson::Bson::DateTime(v) => {
                        row.push(ColumnReturnDataType::Date(Some(v.to_string())))
                    }
                    bson::Bson::ObjectId(v) => {
                        row.push(ColumnReturnDataType::Text(Some(v.to_string())))
                    }
                    bson::Bson::Symbol(v) => {
                        row.push(ColumnReturnDataType::Text(Some(v.to_string())))
                    }
                    bson::Bson::Array(_) => row.push(ColumnReturnDataType::Text(None)),
                    bson::Bson::Document(_) => row.push(ColumnReturnDataType::Text(None)),
                    bson::Bson::Undefined => row.push(ColumnReturnDataType::Text(None)),
                    _ => row.push(ColumnReturnDataType::Text(None))
                }
            }
            rows.push(row);
        }
        Ok((query.columns, rows))
    }
}
