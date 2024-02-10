use std::collections::HashMap;

use anyhow::Result;
use mongodb::{
    bson::{self, Document},
    Client, Collection,
};
use rocket::futures::StreamExt;

use crate::modules::{
    datasource::model::configurations::mongodb_configuration::MongoDbConfig,
    query::model::{
        query_builder::{abstract_query::AbstractQuery, mongodb_builder::MongoDbQuery},
        query_runner::ShapedField,
    },
};

use super::ReturnDataType;

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
    ) -> Result<HashMap<String, ShapedField>> {
        let mut shaped_fields: HashMap<String, ShapedField> = HashMap::new();
        for (label, col) in query.columns {
            shaped_fields.insert(label.clone(), ShapedField::new(col, label));
        }
        let database = &self.config.db_name;
        let btt: Collection<Document> = self
            .client
            .database(database)
            .collection(&query.main_document);
        let mut results = btt.aggregate(query.pipeline, None).await?;
        let mut rows: Vec<Vec<ReturnDataType>> = vec![];
        while let Some(result) = results.next().await {
            let mut row: Vec<ReturnDataType> = vec![];
            let doc: Document = bson::from_document(result?)?;
            for (label, value) in doc {
                let entry = shaped_fields.get_mut(&label).unwrap();
                let inner_value: ReturnDataType;
                match value {
                    bson::Bson::Double(v) => {
                        inner_value = ReturnDataType::Number(Some(v));
                    }
                    bson::Bson::String(v) => {
                        inner_value = ReturnDataType::Text(Some(v));
                    }
                    bson::Bson::Boolean(v) => inner_value = ReturnDataType::Bool(Some(v)),
                    bson::Bson::Null => {
                        inner_value = ReturnDataType::Text(None);
                    }
                    bson::Bson::Int32(v) => {
                        inner_value = ReturnDataType::Number(Some(v.into()));
                    }
                    //TO DO!
                    bson::Bson::Int64(v) => {
                        inner_value = ReturnDataType::Text(Some(v.to_string()));
                    }
                    //TO DO!
                    bson::Bson::Decimal128(v) => {
                        inner_value = ReturnDataType::Text(Some(v.to_string()));
                    }
                    //TO DO!
                    bson::Bson::Binary(v) => {
                        inner_value = ReturnDataType::Text(Some(v.to_string()));
                    }
                    bson::Bson::Timestamp(v) => {
                        inner_value = ReturnDataType::Date(Some(v.to_string()));
                    }
                    bson::Bson::DateTime(v) => {
                        inner_value = ReturnDataType::Date(Some(v.to_string()));
                        row.push(ReturnDataType::Date(Some(v.to_string())))
                    }
                    bson::Bson::ObjectId(v) => {
                        inner_value = ReturnDataType::Text(Some(v.to_string()));
                    }
                    bson::Bson::Symbol(v) => {
                        inner_value = ReturnDataType::Text(Some(v.to_string()));
                    }
                    bson::Bson::Array(_) => inner_value = ReturnDataType::Text(None),
                    bson::Bson::Document(_) => inner_value = ReturnDataType::Text(None),
                    bson::Bson::Undefined => inner_value = ReturnDataType::Text(None),

                    _ => inner_value = ReturnDataType::Text(None),
                }
                entry.values.push(inner_value);
            }
            rows.push(row);
        }
        Ok(shaped_fields)
    }
}
