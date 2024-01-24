pub mod sql_executor;

use self::sql_executor::postgres_executor::PostgresExecutor;

use super::query_builder::abstract_query::AbstractQuery;
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

pub(crate) trait QueryExecutor {
    async fn run(&mut self, query: &str, abstract_query: &AbstractQuery) -> Result<QueryResult>;
}

pub(crate) enum Executor {
    Pg(PostgresExecutor),
}

impl QueryExecutor for Executor {
    async fn run(&mut self, query: &str, abstract_query: &AbstractQuery<'_>) -> Result<QueryResult> {
        let data: Vec<Vec<ColumnReturnDataType>>;
        match self {
            Executor::Pg(executor) => data = executor.execute(query, abstract_query).await?,
        }

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
