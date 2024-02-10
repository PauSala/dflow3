pub mod mongo_db;
pub mod sql;

use std::collections::HashMap;

use self::{
    mongo_db::MongoDbRunner,
    sql::{mssql_runner::MssqlRunner, postgres_runner::PostgresRunner},
};

use super::query_builder::{
    abstract_query::{AbstractQuery, QueryColumn},
    mongodb_builder::MongoDbQuery,
};
use anyhow::Result;
use serde::{Serialize, Serializer};

#[derive(Debug)]
pub enum ReturnDataType {
    Number(Option<f64>),
    Bool(Option<bool>),
    Text(Option<String>),
    Date(Option<String>),
}

impl Serialize for ReturnDataType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ReturnDataType::Number(inner) => inner.serialize(serializer),
            ReturnDataType::Bool(inner) => inner.serialize(serializer),
            ReturnDataType::Text(inner) => inner.serialize(serializer),
            ReturnDataType::Date(inner) => inner.serialize(serializer),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct ShapedField {
    field: QueryColumn,
    label: String,
    values: Vec<ReturnDataType>,
}

impl ShapedField {
    pub fn new(field: QueryColumn, label: String) -> Self {
        Self {
            field,
            label,
            values: vec![],
        }
    }
}

#[derive(Serialize, Debug)]
pub struct ShapedResult {
    pub numerical_fields: Vec<ShapedField>,
    pub categorical_fields: Vec<ShapedField>,
    pub count_numerical: usize,
    pub count_categorical: usize,
}

impl ShapedResult {
    fn new() -> Self {
        Self {
            numerical_fields: Vec::new(),
            categorical_fields: Vec::new(),
            count_numerical: 0,
            count_categorical: 0,
        }
    }
    fn add_field(&mut self, field: ShapedField) {
        if field.field.aggregation.is_some() {
            self.add_numerical(field);
            return;
        }
        self.add_categorical(field);
    }
    fn add_numerical(&mut self, field: ShapedField) {
        self.numerical_fields.push(field);
        self.count_numerical += 1;
    }
    fn add_categorical(&mut self, field: ShapedField) {
        self.categorical_fields.push(field);
        self.count_categorical += 1;
    }
}

#[derive(Serialize, Debug)]
pub struct QueryResult {
    pub columns: Vec<String>,
    pub data: Vec<Vec<ReturnDataType>>,
}

impl QueryResult {
    pub fn serialize(r: QueryResult) -> String {
        serde_json::to_string(&r).expect("QueryResult should accept serialization")
    }
}

pub(crate) trait QueryRunner {
    type Input;
    async fn run(
        &mut self,
        query: Self::Input,
        abstract_query: &AbstractQuery,
    ) -> Result<ShapedResult>;
}

impl QueryRunner for PostgresRunner {
    type Input = String;
    async fn run(
        &mut self,
        query: Self::Input,
        abstract_query: &AbstractQuery<'_>,
    ) -> Result<ShapedResult> {
        let data: Vec<Vec<ReturnDataType>> = self.run_query(&query, abstract_query).await?;
        let result = QueryResult {
            columns: abstract_query
                .columns
                .iter()
                .map(|c| c.column_name.clone())
                .collect(),
            data,
        };
        Ok(ShapedResult::new())
    }
}

impl QueryRunner for MssqlRunner {
    type Input = String;
    async fn run(
        &mut self,
        query: Self::Input,
        abstract_query: &AbstractQuery<'_>,
    ) -> Result<ShapedResult> {
        let data: Vec<Vec<ReturnDataType>> = self.run_query(&query, abstract_query).await?;
        let result = QueryResult {
            columns: abstract_query
                .columns
                .iter()
                .map(|c| c.column_name.clone())
                .collect(),
            data,
        };
        Ok(ShapedResult::new())
    }
}

impl QueryRunner for MongoDbRunner {
    type Input = MongoDbQuery;
    async fn run(
        &mut self,
        query: Self::Input,
        abstract_query: &AbstractQuery<'_>,
    ) -> Result<ShapedResult> {
        let data: HashMap<String, ShapedField> = self.run_query(query, abstract_query).await?;
        let mut result = ShapedResult::new();
        for (_, field ) in data {
            result.add_field(field);
        }
        Ok(result)
    }
}
