use anyhow::{Ok, Result};

use crate::modules::{
    datasource::model::configurations::sql_configuration::SqlConfig,
    shared::shared_state::shared_connections::MssqlClient,
};

use super::{ColumnQueryResult, RelationsResult};

pub struct MssqlModelBuilder {
    client: MssqlClient,
    config: SqlConfig,
}

impl MssqlModelBuilder {
    pub fn new(client: MssqlClient, config: SqlConfig) -> Self {
        MssqlModelBuilder { client, config }
    }

    pub(crate) async fn query_columns(&mut self) -> Result<Vec<ColumnQueryResult>> {
        let query = format!(
            "
            SELECT DISTINCT t.table_name, 
                    column_name, 
                    data_type AS column_type,
                    CASE
                        WHEN COLUMNPROPERTY(object_id(t.table_name), column_name, 'IsIdentity') = 1 THEN 1
                        ELSE 0
                    END AS auto_incremental
            FROM (
                SELECT table_name 
                FROM information_schema.tables 
                WHERE table_type = 'BASE TABLE' AND table_schema = '{schema}'
                UNION ALL
                SELECT table_name 
                FROM information_schema.views 
                WHERE table_schema = '{schema}'
            ) t
            INNER JOIN INFORMATION_SCHEMA.columns c ON c.table_name = t.table_name;
            ",
            schema = self.config
            .schema
            .clone()
            .expect("Schema should be defined"));
        let rows = self
            .client
            .query(query, &[])
            .await?
            .into_first_result()
            .await?;
        let value: Vec<ColumnQueryResult> = rows
            .iter()
            .map(|r| {
                let identity: i32 = r.get(3).expect("This is tested (no)");
                let is_identity;
                match identity {
                    0 => {
                        is_identity = false;
                    }
                    _ => {
                        is_identity = true;
                    }
                }
                ColumnQueryResult::new(
                    r.get::<&str, _>(0)
                        .expect("This is tested (no)")
                        .to_string(),
                    r.get::<&str, _>(1)
                        .expect("This is tested (no)")
                        .to_string(),
                    r.get::<&str, _>(2)
                        .expect("This is tested (no)")
                        .to_string(),
                    is_identity,
                )
            })
            .collect();
        Ok(value)
    }

    pub(crate) async fn query_relations(&mut self) -> Result<Vec<RelationsResult>> {
        let query = format!(
            "
            SELECT 
                rel_kcu.table_name AS pk_table,
                kcu.table_name AS fk_table, 
                rel_kcu.column_name AS pk_column,
                kcu.column_name AS fk_column
            FROM information_schema.table_constraints tco
            JOIN information_schema.key_column_usage kcu
               ON tco.constraint_schema = kcu.constraint_schema
               AND tco.constraint_name = kcu.constraint_name
            JOIN information_schema.referential_constraints rco
               ON tco.constraint_schema = rco.constraint_schema
               AND tco.constraint_name = rco.constraint_name
            JOIN information_schema.key_column_usage rel_kcu
               ON rco.unique_constraint_schema = rel_kcu.constraint_schema
               AND rco.unique_constraint_name = rel_kcu.constraint_name
               AND kcu.ordinal_position = rel_kcu.ordinal_position
            WHERE tco.constraint_type = 'FOREIGN KEY'
            AND tco.table_schema = '{schema}'
            ORDER BY kcu.table_schema, kcu.table_name, kcu.ordinal_position;

            ",
            schema = self
                .config
                .schema
                .clone()
                .expect("Schema should be defined")
        );

        let rows = self
            .client
            .query(query, &[])
            .await?
            .into_first_result()
            .await?;
        let value: Vec<RelationsResult> = rows
            .iter()
            .map(|r| {
                RelationsResult::new(
                    r.get::<&str, _>(0)
                        .expect("This is tested (no)")
                        .to_string(),
                    r.get::<&str, _>(1)
                        .expect("This is tested (no)")
                        .to_string(),
                    r.get::<&str, _>(2)
                        .expect("This is tested (no)")
                        .to_string(),
                    r.get::<&str, _>(3)
                        .expect("This is tested (no)")
                        .to_string(),
                )
            })
            .collect();
        Ok(value)
    }
}
