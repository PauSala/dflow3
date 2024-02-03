use std::collections::HashMap;

use anyhow::Result;
use rocket_db_pools::sqlx::{self, Acquire};
use rocket_db_pools::Connection;

use crate::modules::dmodel::model::model::{Column, Model, TModelSaver, Relation, Table};
use crate::Db;

pub struct ModelStorer<'a> {
    db: &'a mut Connection<Db>,
}

impl<'a> ModelStorer<'a> {
    pub fn new(db: &'a mut Connection<Db>) -> Self {
        Self { db }
    }

    async fn insert_model(&mut self, id: &str, datasource_id: &str) -> Result<()> {
        sqlx::query("INSERT INTO models (id, datasource_id) VALUES (?, ?)")
            .bind(id)
            .bind(datasource_id)
            .execute(&mut ***self.db)
            .await?;
        Ok(())
    }

    async fn insert_tables(
        &mut self,
        tables: &HashMap<usize, Table>,
        model_id: &str,
    ) -> Result<()> {
        let mut tx = self.db.begin().await?;

        for (_, table) in tables {
            sqlx::query(
                "INSERT INTO tables (id, table_name, display_name, model_id) VALUES (?, ?, ?, ?)",
            )
            .bind(table.table_id as u32)
            .bind(table.name.clone())
            .bind(table.display_name.clone())
            .bind(model_id)
            .execute(&mut *tx)
            .await?;
        }
        tx.commit().await?;
        Ok(())
    }

    async fn insert_columns(
        &mut self,
        columns: &mut Vec<(&usize, &Column)>,
        table_id: &usize,
        model_id: &str,
    ) -> Result<()> {
        let mut tx = self.db.begin().await?;
        for (_, column) in columns {
            let column_type = column.type_alias.to_string();
            sqlx::query(
                "INSERT INTO columns 
                    (id, column_name, display_name, type, type_alias, table_id, model_id, is_array) 
                    VALUES (?, ?, ?, ?, ?, ?, ?, ?)
                ",
            )
            .bind(column.column_id as u32)
            .bind(column.name.clone())
            .bind(column.display_name.clone())
            .bind(column.actual_type.clone())
            .bind(column_type.clone())
            .bind(*table_id as u32)
            .bind(model_id)
            .bind(column.is_array)
            .execute(&mut *tx)
            .await?;
        }
        tx.commit().await?;
        Ok(())
    }

    async fn insert_relations(
        &mut self,
        relations: &HashMap<usize, Relation>,
        model_id: &str,
    ) -> Result<()> {
        let mut tx = self.db.begin().await?;
        for (id, r) in relations.iter() {
            sqlx::query(
                "INSERT INTO relations (id, pk_table, fk_table, pk_column, fk_column, model_id) VALUES (?, ?, ?, ?, ?, ?)"
            )
            .bind(*id as u32)
            .bind(r.pk_table as u32)
            .bind(r.fk_table as u32)
            .bind(r.pk_column as u32)
            .bind(r.fk_column as u32)
            .bind(model_id)
            .execute(&mut *tx).await?;
        }
        tx.commit().await?;
        Ok(())
    }
}

impl TModelSaver for ModelStorer<'_> {
    async fn persist(&mut self, model: &Model) -> Result<()> {
        self.insert_model(&model.id, &model.datasource_id).await?;
        self.insert_tables(&model.tables, &model.id).await?;
        for (_, table) in &model.tables {
            let mut cols: Vec<(&usize, &Column)> = table.columns.iter().collect();
            self.insert_columns(&mut cols, &table.table_id, &model.id)
                .await?;
        }
        self.insert_relations(&model.relations, &model.id).await?;
        Ok(())
    }
}
