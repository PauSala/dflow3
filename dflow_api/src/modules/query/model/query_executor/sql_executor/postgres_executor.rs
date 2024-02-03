use std::collections::HashMap;

use anyhow::{Ok, Result};
use deadpool_postgres::Object as Client;

use crate::modules::{dmodel::model::model::TypeAlias, query::model::{query_builder::abstract_query::AbstractQuery, query_executor::ColumnReturnDataType}};


pub struct PostgresExecutor {
    client: Client,
}

impl PostgresExecutor {
    pub fn new(client: Client) -> Self {
        PostgresExecutor { client }
    }

    pub(crate) async fn execute(
        &self,
        query: &str,
        abstract_query: &AbstractQuery<'_>,
    ) -> Result<Vec<Vec<ColumnReturnDataType>>> {
        let columns = abstract_query.get_columns();
        let mut column_map = HashMap::new();
        for col in columns {
            column_map.insert(col.column_name.clone(), col);
        }
        let statement = self.client.prepare(&query).await?;
        let rows = self.client.query(&statement, &[]).await?;
        let mut data: Vec<Vec<ColumnReturnDataType>> = Vec::new();
        for row in rows.iter() {
            let mut row_data = Vec::new();
            for (col_index, col) in row.columns().iter().enumerate() {
                let column_id = col.name();
                let m = column_map.get(column_id).expect("Column id should exist");
                match m.data_type {
                    TypeAlias::Integer | TypeAlias::Float => {
                        let v: Option<f64> = row.get(col_index);
                        row_data.push(ColumnReturnDataType::Number(v));
                    }
                    TypeAlias::Bool => {
                        let v: Option<bool> = row.get(col_index);
                        row_data.push(ColumnReturnDataType::Bool(v));
                    }
                    TypeAlias::Text => {
                        let v: Option<String> = row.get(col_index);
                        row_data.push(ColumnReturnDataType::Text(v));
                    }
                    TypeAlias::Date => {
                        let v: Option<String> = row.get(col_index);
                        row_data.push(ColumnReturnDataType::Date(v));
                    }
                    TypeAlias::Array(_) => {}
                }
            }
            data.push(row_data);
        }
        Ok(data)
    }
}
