use std::collections::{HashMap, HashSet};

use anyhow::{bail, Result};
use rocket_db_pools::sqlx::{self, Row};
use rocket_db_pools::Connection;
use crate::{modules::dmodel::model::model::{Column, Model, Relation, Table, TypeAlias}, Db};



pub struct ModelGetter<'a> {
    pub db: &'a mut Connection<Db>,
}


impl<'a> ModelGetter<'a> {
    pub(crate) fn new(db: &'a mut Connection<Db>) -> Self {
        ModelGetter { db }
    }
    pub async fn retrieve(&mut self, model_id: &str) -> Result<Model> {
        let model = self.get_model_by_id(model_id).await?;
        if let Some(mut model) = model {
            model.tables = self.get_tables_by_model_id(&model.id).await?;
            self.set_model_relations(&mut model).await?;
            return Ok(model);
        }
        bail!("Model not found: {}", model_id)
    }

    async fn get_model_by_id(&mut self, model_id: &str) -> Result<Option<Model>> {
        let query = "SELECT * FROM models WHERE id = ?";
        let model = sqlx::query(query)
            .bind(model_id)
            .map(|row| Model {
                id: row.get(0),
                datasource_id: row.get(1),
                relations: HashMap::new(),
                tables: HashMap::new(),
                relation_count: 0,
            })
            .fetch_one(&mut ***self.db)
            .await?;
        Ok(Some(model))
    }

    async fn get_tables_by_model_id(&mut self, model_id: &str) -> Result<HashMap<usize, Table>> {
        let query = "
        SELECT
            id,table_name,display_name,model_id
        FROM tables WHERE model_id = ?";

        let mut tables = HashMap::new();

        let result = sqlx::query(query)
            .bind(model_id)
            .map(|row| Table {
                table_id: row.get::<u32, _>(0) as usize,
                name: row.get(1),
                display_name: row.get(2),
                columns: HashMap::new(),
                column_count: 0,
                relations: HashSet::new(),
            })
            .fetch_all(&mut ***self.db)
            .await?;

        for table in result {
            tables.insert(table.table_id, table);
        }

        let query = "
        SELECT 
        c.id, c.column_name, c.display_name, c.type, c.type_alias, c.table_id 
          FROM columns c 
        INNER JOIN tables t on c.table_id = t.id 
        WHERE t.model_id = ?";

        let cols_result = sqlx::query(query)
            .bind(model_id)
            .map(|row| {
                let type_alias: String = row.get(4);
                let table_id = row.get::<u32, _>(5) as usize;
                (
                    Column {
                        column_id: row.get::<u32, _>(0) as usize,
                        name: row.get(1),
                        display_name: row.get(2),
                        type_alias: TypeAlias::from_string(&type_alias),
                        actual_type: row.get(3),
                    },
                    table_id,
                )
            })
            .fetch_all(&mut ***self.db)
            .await?;
        for col in cols_result {
            let table = tables.get_mut(&col.1).unwrap();
            table.columns.insert(col.0.column_id, col.0);
            table.column_count += 1;
        }

        Ok(tables)
    }

    async fn set_model_relations(&mut self, model: &mut Model) -> Result<()> {
        let query = "SELECT r.* FROM relations r 
        inner join tables t on r.pk_table = t.id  or r.fk_table = t.id
        WHERE t.model_id = ?";

        let result = sqlx::query(query)
            .bind(&model.id)
            .map(|row| Relation {
                id: row.get::<u32, _>(0) as usize,
                pk_table: row.get::<u32, _>(1) as usize,
                fk_table: row.get::<u32, _>(2) as usize,
                pk_column: row.get::<u32, _>(3) as usize,
                fk_column: row.get::<u32, _>(4) as usize,
                active: true,
            })
            .fetch_all(&mut ***self.db)
            .await?;
        for r in result {
            model.relation_count += 1;
            let pk = model.tables.get_mut(&r.pk_table).unwrap();
            pk.relations.insert(r.id);
            let fk = model.tables.get_mut(&r.fk_table).unwrap();
            fk.relations.insert(r.id);
            model.relations.insert(r.id.clone(), r);
        }
        Ok(())
    }
}

