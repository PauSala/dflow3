pub mod sql;
pub mod mongo_db;

use self::{mongo_db::MongoDbRunner, sql::{mssql_runner::MssqlRunner, postgres_runner::PostgresRunner}};

use super::query_builder::{abstract_query::AbstractQuery, mongodb_builder::MongoDbQuery};
use anyhow::Result;
use serde::{Serialize, Serializer};

#[derive(Debug)]
pub enum ColumnReturnDataType {
    Number(Option<f64>),
    Bool(Option<bool>),
    Text(Option<String>),
    Date(Option<String>),
}

impl Serialize for ColumnReturnDataType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ColumnReturnDataType::Number(inner) => inner.serialize(serializer),
            ColumnReturnDataType::Bool(inner) => inner.serialize(serializer),
            ColumnReturnDataType::Text(inner) => inner.serialize(serializer),
            ColumnReturnDataType::Date(inner) => inner.serialize(serializer),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct QueryResult {
    pub columns: Vec<String>,
    pub data: Vec<Vec<ColumnReturnDataType>>,
}

impl QueryResult {
    pub fn serialize(r: QueryResult) -> String {
        serde_json::to_string(&r).expect("QueryResult should accept serialization")
    }
}

pub(crate) trait QueryRunner {
    type Input;
    async fn run(&mut self, query: Self::Input, abstract_query: &AbstractQuery) -> Result<QueryResult>;
}

impl QueryRunner for PostgresRunner {
    type Input = String;
    async fn run(&mut self, query: Self::Input, abstract_query: &AbstractQuery<'_>) -> Result<QueryResult>{
        let data: Vec<Vec<ColumnReturnDataType>> = self.run_query(&query, abstract_query).await?;
        let result = QueryResult {
            columns: abstract_query
                .columns
                .iter()
                .map(|c| c.column_name.clone())
                .collect(),
            data,
        };
        Ok(result)
    }
}

impl QueryRunner for MssqlRunner {
    type Input = String;
    async fn run(&mut self, query: Self::Input, abstract_query: &AbstractQuery<'_>) -> Result<QueryResult>{
        let data: Vec<Vec<ColumnReturnDataType>> = self.run_query(&query, abstract_query).await?;
        let result = QueryResult {
            columns: abstract_query
                .columns
                .iter()
                .map(|c| c.column_name.clone())
                .collect(),
            data,
        };
        Ok(result)
    }
}

impl QueryRunner for MongoDbRunner {
    type Input = MongoDbQuery;
    async fn run(&mut self, query: Self::Input, abstract_query: &AbstractQuery<'_>) -> Result<QueryResult>{
        let data: (Vec<String>, Vec<Vec<ColumnReturnDataType>>) = self.run_query(query, abstract_query).await?;
        let result = QueryResult {
            columns: data.0,
            data: data.1,
        };
        Ok(result)
    }
}
