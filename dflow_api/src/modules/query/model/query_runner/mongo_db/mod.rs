use anyhow::Result;
use mongodb::{bson::{self, Document}, Client, Collection};
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
    ) -> Result<Vec<Vec<ColumnReturnDataType>>> {
        let database = &self.config.db_name;
        dbg!(&query.pipeline);
        let btt: Collection<Document> = self
            .client
            .database(database)
            .collection(&query.main_document);
        let mut results = btt.aggregate(query.pipeline, None).await?;
        while let Some(result) = results.next().await {
            let doc: Document = bson::from_document(result?)?;
            println!("* {}", doc);
        }
        Ok(vec![])
    }
}
