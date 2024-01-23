use anyhow::Result;
use rocket_db_pools::sqlx::{self};
use rocket_db_pools::Connection;

use crate::{modules::datasource::model::datasource_type::DataSource, Db};

pub struct DataSourceSaver<'a> {
    db: &'a mut Connection<Db>,
}

impl<'a> DataSourceSaver<'a> {
    pub fn new(db: &'a mut Connection<Db>) -> Self {
        Self { db }
    }

    pub async fn persist(&mut self, datasource: &DataSource) -> Result<()> {
        sqlx::query("INSERT INTO datasources (id, name, type) VALUES (?, ?, ?)")
            .bind(datasource.id.clone())
            .bind(datasource.name.clone())
            .bind(datasource.datasource_type.to_string())
            .execute(&mut ***self.db)
            .await?;
        Ok(())
    }
}
