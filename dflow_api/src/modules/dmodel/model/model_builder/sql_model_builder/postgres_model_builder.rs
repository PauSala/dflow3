use anyhow::{Ok, Result};
use deadpool_postgres::Client;

use crate::modules::datasource::model::configurations::sql_configuration::SqlConfig;

use super::{ColumnQueryResult, RelationsResult};

pub struct PosgtresModelBuilder {
    client: Client,
    config: SqlConfig,
}

impl PosgtresModelBuilder {
    pub fn new(client: Client, config: SqlConfig) -> Self {
        PosgtresModelBuilder { client, config }
    }

    pub(crate) async fn query_columns(&self) -> Result<Vec<ColumnQueryResult>> {
        let query = format!(
            "
            SELECT distinct t.table_name, column_name, udt_name AS column_type,
            CASE
                WHEN c.column_default LIKE 'nextval%' THEN true
                ELSE false
            END AS auto_incremental
            FROM 
               ( 
                   SELECT table_name 
                   FROM information_schema.tables 
                   WHERE table_type = 'BASE TABLE' AND table_schema = '{schema}'
                   UNION ALL
                   SELECT table_name 
                   FROM  information_schema.views 
               WHERE table_schema = '{schema}'
           ) t
           INNER JOIN  INFORMATION_SCHEMA.columns c on c.table_name = t.table_name;
        ",
            schema = self
                .config
                .schema
                .clone()
                .expect("Schema should be defined")
        );
        let statement = self.client.prepare(&query).await?;
        let rows = self.client.query(&statement, &[]).await?;

        // Assuming you expect a single row with a single column
        let value: Vec<ColumnQueryResult> = rows
            .iter()
            .map(|r| ColumnQueryResult::new(r.get(0), r.get(1), r.get(2), r.get(3)))
            .collect();
        Ok(value)
    }

    pub(crate) async fn query_relations(&self) -> Result<Vec<RelationsResult>> {
        let query = format!(
            "
            SELECT 
                rel_kcu.table_name as pk_table,
                kcu.table_name as fk_table, 
                rel_kcu.column_name as pk_column,
                kcu.column_name as fk_column
            FROM information_schema.table_constraints tco
            JOIN information_schema.key_column_usage kcu
               on tco.constraint_schema = kcu.constraint_schema
               and tco.constraint_name = kcu.constraint_name
            JOIN information_schema.referential_constraints rco
               on tco.constraint_schema = rco.constraint_schema
               and tco.constraint_name = rco.constraint_name
            JOIN information_schema.key_column_usage rel_kcu
               on rco.unique_constraint_schema = rel_kcu.constraint_schema
               and rco.unique_constraint_name = rel_kcu.constraint_name
               and kcu.ordinal_position = rel_kcu.ordinal_position
            WHERE tco.constraint_type = 'FOREIGN KEY'
            and tco.table_schema = '{}'
            ORDER BY kcu.table_schema,  kcu.table_name, kcu.ordinal_position;
        ",
            self.config
                .schema
                .clone()
                .expect("Schema should be defined")
        );
        let statement = self.client.prepare(&query).await?;
        let rows = self.client.query(&statement, &[]).await?;

        let value: Vec<RelationsResult> = rows
            .iter()
            .map(|r| RelationsResult::new(r.get(0), r.get(1), r.get(2), r.get(3)))
            .collect();
        Ok(value)
    }
}
