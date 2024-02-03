use anyhow::{bail, Ok, Result};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(thiserror::Error, Debug, Clone)]
pub enum ModelError {
    #[error("Table not found in model")]
    TableNotFound(usize),
    #[error("Column not found in model")]
    ColumnNotFound(usize),
}

/// Represents a type alias for a column type in a database.
///
/// This type alias is used to abstract and generalize the specific data types
/// associated with columns in a database.
///
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum TypeAlias {
    Integer,
    Float,
    Bool,
    Text,
    Date,
    Array(Box<TypeAlias>)
}

impl TypeAlias {
    pub fn to_string(&self) -> String {
        match self {
            TypeAlias::Integer => "integer".to_owned(),
            TypeAlias::Float => "float".to_owned(),
            TypeAlias::Bool => "bool".to_owned(),
            TypeAlias::Text => "text".to_owned(),
            TypeAlias::Date => "date".to_owned(),
            TypeAlias::Array(_) => "array".to_owned(),
        }
    }
    pub fn from_string(value: &str) -> Self {
        match value {
            "integer" => TypeAlias::Integer,
            "float" => TypeAlias::Float,
            "bool" => TypeAlias::Bool,
            "text" => TypeAlias::Text,
            "date" => TypeAlias::Date,
            _ => TypeAlias::Text,
        }
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Column {
    pub column_id: usize,
    pub name: String,
    pub display_name: String,
    pub type_alias: TypeAlias,
    pub actual_type: String,
    pub is_array: bool
}

impl Column {
    pub fn new(column_id: usize, name: &str, type_alias: TypeAlias, actual_type: &str, is_array: bool) -> Self {
        Column {
            column_id,
            name: name.to_string(),
            display_name: name.to_string(),
            type_alias,
            actual_type: actual_type.to_string(),
            is_array
        }
    }
}

#[derive(Serialize, Debug, PartialEq, Eq, Clone, Hash)]
pub struct Relation {
    pub id: usize,
    pub pk_table: usize,
    pub fk_table: usize,
    pub pk_column: usize,
    pub fk_column: usize,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Eq, PartialEq)]
pub struct Table {
    pub table_id: usize,
    pub name: String,
    pub display_name: String,
    pub columns: HashMap<usize, Column>,
    pub column_count: usize,
    pub relations: HashSet<usize>,
}

impl Table {
    pub fn new(table_id: usize, name: &str) -> Self {
        Self {
            table_id,
            name: name.to_string(),
            display_name: name.to_string(),
            columns: HashMap::new(),
            column_count: 0,
            relations: HashSet::new(),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct Model {
    pub datasource_id: String,
    pub id: String,
    pub tables: HashMap<usize, Table>,
    pub relations: HashMap<usize, Relation>,
    pub relation_count: usize,
}

impl Model {
    pub fn new(datasource_id: &str, model_id: &str) -> Self {
        Self {
            datasource_id: datasource_id.to_string(),
            id: model_id.to_string(),
            tables: HashMap::new(),
            relations: HashMap::new(),
            relation_count: 0,
        }
    }

    pub fn find_table_by_name(&self, table_name: &str) -> Option<&Table> {
        self.tables
            .iter()
            .map(|(_, table)| table)
            .find(|t| t.name == table_name)
    }

    pub fn find_column_by_name(&self, table_id: usize, column_name: &str) -> Option<&Column> {
        let table = self.tables.get(&table_id);
        match table {
            None => None,
            Some(table) => table
                .columns
                .iter()
                .map(|(_, c)| c)
                .find(|c| c.name == column_name),
        }
    }

    pub fn add_table(&mut self, name: &str, table_id: usize) -> &mut Self {
        self.tables.insert(table_id, Table::new(table_id, name));
        self
    }

    pub fn add_column(
        &mut self,
        table_id: usize,
        name: &str,
        type_alias: TypeAlias,
        actual_type: &str,
        is_array: bool
    ) -> Result<&mut Self> {
        if let Some(table) = self.tables.get_mut(&table_id) {
            let id = table.column_count;
            let column = Column::new(id, name, type_alias, actual_type, is_array);
            table.columns.insert(column.column_id, column);
            table.column_count += 1;
            return Ok(self);
        }
        bail!(ModelError::TableNotFound(table_id))
    }
    pub fn add_relation(
        &mut self,
        pk_table: usize,
        pk_column: usize,
        fk_table: usize,
        fk_column: usize,
    ) -> Result<&mut Self> {
        if !self.tables.contains_key(&pk_table) {
            bail!(ModelError::TableNotFound(pk_table));
        }
        if !self.tables.contains_key(&fk_table) {
            bail!(ModelError::TableNotFound(fk_table));
        }
        if !self
            .tables
            .get(&pk_table)
            .unwrap()
            .columns
            .contains_key(&pk_column)
        {
            dbg!(self);
            bail!(ModelError::ColumnNotFound(pk_column));
        }

        if !self
            .tables
            .get(&fk_table)
            .unwrap()
            .columns
            .contains_key(&fk_column)
        {
            dbg!(self);
            bail!(ModelError::ColumnNotFound(pk_column));
        }

        let relation_id = self.relation_count + 1;
        let relation = Relation {
            id: relation_id,
            pk_table,
            fk_table,
            pk_column,
            fk_column,
            active: true,
        };

        self.relations.insert(relation_id, relation);
        self.relation_count += 1;

        let pk_table = self
            .tables
            .get_mut(&pk_table)
            .expect("If this happen bail is not working!");
        pk_table.relations.insert(relation_id);
        let fk_table = self
            .tables
            .get_mut(&fk_table)
            .expect("If this happen bail is not working!");
        fk_table.relations.insert(relation_id);
        Ok(self)
    }
}

pub(crate) trait TModelSaver {
    async fn persist(&mut self, model: &Model) -> Result<()>;
}

#[cfg(test)]
pub mod test {

    use super::Model;
    use anyhow::{Ok, Result};

    #[test]
    fn should_add_tables() {
        let mut model = Model::new("test", "test");
        model.add_table("T1", 1).add_table("T2", 2);
        let table = model.tables.get(&1).expect("Table should exist");
        assert_eq!(table.table_id, 1);
        let table = model.tables.get(&2).expect("Table should exist");
        assert_eq!(table.table_id, 2);
    }

    #[test]
    fn should_add_columns() -> Result<()> {
        let mut model = Model::new("test", "test");
        let table_name = "T1";
        let table_id = 1;
        let column_name = "C1";
        model.add_table(table_name, table_id);
        model.add_column(table_id, column_name, super::TypeAlias::Integer, "int4", false)?;
        let table = model
            .tables
            .get(&table_id)
            .expect("This table should exist");
        let column = table
            .columns
            .get(&0)
            .expect("This column should exist and it should have id = 0");
        assert_eq!(column.name, column_name);
        Ok(())
    }

    #[test]
    fn should_error_on_add_column_if_table_does_not_exist() -> Result<()> {
        let mut model = Model::new("test", "test");
        let err = model.add_column(0, "C1", super::TypeAlias::Integer, "int4", false);
        assert!(err.is_err());
        Ok(())
    }

    #[test]
    fn it_should_add_relations() -> Result<()> {
        let mut model = Model::new("test", "test");
        model.add_table("T1", 1).add_table("T2", 2);
        model.add_column(1, "C1", super::TypeAlias::Integer, "int4", false)?;
        model.add_column(2, "C1", super::TypeAlias::Integer, "int4", false)?;
        model.add_column(2, "C2", super::TypeAlias::Integer, "int4", false)?;
        model.add_relation(1, 0, 2, 1)?;
        assert_eq!(model.relations.len(), 1);
        let table1 = model.tables.get(&1).expect("Table should exist");
        let table2 = model.tables.get(&2).expect("table should exist");
        assert_eq!(table1.relations.len(), 1);
        assert_eq!(table2.relations.len(), 1);

        Ok(())
    }

    #[test]
    fn it_should_throw_creating_relations_for_non_existing_tables() -> Result<()> {
        let mut model = Model::new("test", "test");
        let err = model.add_relation(0, 1, 0, 2);
        assert!(err.is_err());
        Ok(())
    }

    #[test]
    fn it_should_throw_creating_relations_for_non_existing_columns() -> Result<()> {
        let mut model = Model::new("test", "test");
        model.add_table("T1", 1).add_table("T2", 2);
        let err = model.add_relation(0, 1, 2, 2);
        assert!(err.is_err());
        Ok(())
    }
}
