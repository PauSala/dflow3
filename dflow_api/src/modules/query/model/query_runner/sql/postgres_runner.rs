use std::collections::HashMap;

use anyhow::{Ok, Result};
use deadpool_postgres::Object as Client;

use crate::modules::{
    dmodel::model::model::TypeAlias,
    query::model::{
        query_builder::abstract_query::AbstractQuery, query_runner::ReturnDataType,
    },
};

pub struct PostgresRunner {
    client: Client,
}

impl PostgresRunner {
    pub fn new(client: Client) -> Self {
        PostgresRunner { client }
    }

    pub(crate) async fn run_query(
        &self,
        query: &str,
        abstract_query: &AbstractQuery<'_>,
    ) -> Result<Vec<Vec<ReturnDataType>>> {
        let columns = abstract_query.get_columns();
        let mut column_map = HashMap::new();
        for col in columns {
            column_map.insert(col.column_name.clone(), col);
        }
        let statement = self.client.prepare(&query).await?;
        let rows = self.client.query(&statement, &[]).await?;
        let mut data: Vec<Vec<ReturnDataType>> = Vec::new();
        for row in rows.iter() {
            let mut row_data = Vec::new();
            for (col_index, col) in row.columns().iter().enumerate() {
                let column_id = col.name();
                let m = column_map.get(column_id).expect("Column id should exist");
                match m.data_type {
                    TypeAlias::Integer | TypeAlias::Float => {
                        let v: Option<f64> = row.get(col_index);
                        row_data.push(ReturnDataType::Number(v));
                    }
                    TypeAlias::Bool => {
                        let v: Option<bool> = row.get(col_index);
                        row_data.push(ReturnDataType::Bool(v));
                    }
                    TypeAlias::Text => {
                        let v: Option<String> = row.get(col_index);
                        row_data.push(ReturnDataType::Text(v));
                    }
                    TypeAlias::Date => {
                        let v: Option<String> = row.get(col_index);
                        row_data.push(ReturnDataType::Date(v));
                    }
                    TypeAlias::Array(_) => {}
                }
            }
            data.push(row_data);
        }
        Ok(data)
    }
}
