use std::collections::HashMap;

use anyhow::{Ok, Result};
use deadpool_tiberius::tiberius::numeric::Numeric;

use crate::modules::{
    dmodel::model::model::TypeAlias,
    query::model::{
        query_builder::abstract_query::AbstractQuery, query_executor::ColumnReturnDataType,
    },
    shared::shared_state::shared_connections::MssqlClient,
};

pub struct MssqlExecutor {
    client: MssqlClient,
}

impl MssqlExecutor {
    pub fn new(client: MssqlClient) -> Self {
        MssqlExecutor { client }
    }

    pub(crate) async fn execute(
        &mut self,
        query: &str,
        abstract_query: &AbstractQuery<'_>,
    ) -> Result<Vec<Vec<ColumnReturnDataType>>> {
        let columns = abstract_query.get_columns();
        let mut column_map = HashMap::new();
        for col in columns {
            column_map.insert(col.column_name.clone(), col);
        }
        let rows = self
            .client
            .query(query, &[])
            .await?
            .into_first_result()
            .await?;
        let mut data: Vec<Vec<ColumnReturnDataType>> = Vec::new();
        for row in rows.iter() {
            let mut row_data = Vec::new();
            for (col_index, col) in row.columns().iter().enumerate() {
                let column_id = col.name();
                let m = column_map.get(column_id).expect("Column id should exist");
                match m.data_type {
                    TypeAlias::Integer | TypeAlias::Float => {
                        let v: Option<Numeric> = row.get(col_index);
                        row_data.push(ColumnReturnDataType::Number(v.map(|value| value.into())));
                    }
                    TypeAlias::Bool => {
                        let v: Option<bool> = row.get(col_index);
                        row_data.push(ColumnReturnDataType::Bool(v.map(|value| value.into())));
                    }
                    TypeAlias::Text => {
                        let v: Option<&str> = row.get(col_index);
                        row_data.push(ColumnReturnDataType::Text(v.map(|value| value.into())));
                    }
                    TypeAlias::Date => {
                        let v: Option<&str> = row.get(col_index);
                        row_data.push(ColumnReturnDataType::Date(v.map(|value| value.into())));
                    }
                }
            }
            data.push(row_data);
        }
        Ok(data)
    }
}
