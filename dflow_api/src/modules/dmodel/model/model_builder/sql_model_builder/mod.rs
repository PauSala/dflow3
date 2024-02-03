pub mod mssql_model_builder;
pub mod postgres_model_builder;

use self::{mssql_model_builder::MssqlModelBuilder, postgres_model_builder::PosgtresModelBuilder};
use crate::modules::dmodel::model::model::{Model, TypeAlias};

use anyhow::{bail, Result};

pub struct SqlModelBuilder {
    pub connection: SqlBuilderDialect,
}

impl SqlModelBuilder {
    pub fn new(connection: SqlBuilderDialect) -> Self {
        SqlModelBuilder { connection }
    }
    async fn get_columns(&mut self) -> Result<Vec<ColumnQueryResult>> {
        match &mut self.connection {
            SqlBuilderDialect::Postgresql(cn) => {
                let tables = cn.query_columns().await?;
                Ok(tables)
            }
            SqlBuilderDialect::Mssql(cn) => {
                let tables = cn.query_columns().await?;
                Ok(tables)
            }
        }
    }

    async fn get_relations(&mut self) -> Result<Vec<RelationsResult>> {
        match &mut self.connection {
            SqlBuilderDialect::Postgresql(cn) => {
                let relations = cn.query_relations().await?;
                Ok(relations)
            }
            SqlBuilderDialect::Mssql(cn) => {
                let relations = cn.query_relations().await?;
                Ok(relations)
            }
        }
    }

    fn normalize_type(&self, type_str: &str, autoincremental: &bool) -> TypeAlias {
        if *autoincremental {
            return TypeAlias::Integer;
        }

        let clean_type = type_str
            .replace(|c: char| c == '(' || c == ')', "")
            .to_uppercase();

        match clean_type.as_str() {
            "NUMERIC" | "FIXED" | "NUMBER" | "SERIAL" | "DECIMAL" | "DEC" | "VARBINARY"
            | "DOUBLE" | "DOUBLE PRECISSION" | "FLOAT" | "FLOAT8" | "FLOAT16" | "FLOAT64"
            | "REAL" | "LONG" => TypeAlias::Float,

            "INT4" | "INT8" | "INT" | "INT64" | "INTEGER" | "TINYINT" | "SMALLINT" | "BIT"
            | "MEDIUMINT" | "BIGINT" => TypeAlias::Integer,

            "DATE" | "TIMESTAMP" | "TIME" | "DATETIME" | "TIMESTAMPTZ" => TypeAlias::Date,

            "TEXT" | "TINYTEXT" | "MEDIUMTEXT" | "LONGTEXT" | "VARCHAR" | "VARCHAR2"
            | "NVARCHAR" | "CHAR" | "NCHAR" => TypeAlias::Text,

            "BOOL" | "BOOLEAN" => TypeAlias::Bool,

            _ => TypeAlias::Text,
        }
    }
    pub async fn build_model(&mut self, data_source_id: &str, model_id: &str) -> Result<Model> {
        let mut model = Model::new(data_source_id, model_id);
        let columns = self.get_columns().await?;
        let mut table_index = 0;
        for column in columns {
            let table = model.find_table_by_name(&column.table_name);
            match table {
                None => {
                    model.add_table(&column.table_name, table_index);
                    model.add_column(
                        table_index,
                        &column.column_name,
                        self.normalize_type(&column.column_type, &column.auto_incremental),
                        &column.column_type,
                        false
                    )?;
                    table_index += 1;
                }
                Some(table) => {
                    model.add_column(
                        table.table_id,
                        &column.column_name,
                        self.normalize_type(&column.column_type, &column.auto_incremental),
                        &column.column_type,
                        false
                    )?;
                }
            }
        }
        let relations = self.get_relations().await?;
        for relation in relations {
            let pk_table = model.find_table_by_name(&relation.pk_table);
            let fk_table = model.find_table_by_name(&relation.fk_table);

            if pk_table.is_none() {
                bail!(ModelError::TableNotFound(relation.pk_table))
            }
            if fk_table.is_none() {
                bail!(ModelError::TableNotFound(relation.fk_table))
            }
            let pk_column =
                model.find_column_by_name(pk_table.unwrap().table_id, &relation.pk_column);
            let fk_column =
                model.find_column_by_name(fk_table.unwrap().table_id, &relation.fk_column);

            if pk_column.is_none() {
                bail!(ModelError::ColumnNotFound(relation.pk_column))
            }
            if fk_column.is_none() {
                bail!(ModelError::ColumnNotFound(relation.fk_column))
            }

            model.add_relation(
                pk_table.unwrap().table_id,
                pk_column.unwrap().column_id,
                fk_table.unwrap().table_id,
                fk_column.unwrap().column_id,
            )?;
        }

        Ok(model)
    }

}

#[derive(thiserror::Error, Debug, Clone)]
pub enum ModelError {
    #[error("Table not found in model")]
    TableNotFound(String),
    #[error("Column not found in model")]
    ColumnNotFound(String),
}

pub enum SqlBuilderDialect {
    Postgresql(PosgtresModelBuilder),
    Mssql(MssqlModelBuilder),
}

pub(crate) struct ColumnQueryResult {
    table_name: String,
    column_name: String,
    column_type: String,
    auto_incremental: bool,
}

impl ColumnQueryResult {
    pub(crate) fn new(
        table_name: String,
        column_name: String,
        column_type: String,
        auto_incremental: bool,
    ) -> Self {
        ColumnQueryResult {
            table_name,
            column_name,
            column_type,
            auto_incremental,
        }
    }
}

pub(crate) struct RelationsResult {
    pk_table: String,
    fk_table: String,
    pk_column: String,
    fk_column: String,
}

impl RelationsResult {
    pub(crate) fn new(
        pk_table: String,
        fk_table: String,
        pk_column: String,
        fk_column: String,
    ) -> Self {
        RelationsResult {
            pk_table,
            fk_table,
            pk_column,
            fk_column,
        }
    }
}
