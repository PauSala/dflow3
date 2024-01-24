use anyhow::Result;
use rocket_db_pools::sqlx::{self, Row};
use rocket_db_pools::Connection;

use crate::modules::datasource::model::datasource_repository::TDataSourceGetter;
use crate::modules::datasource::model::datasource_type::DataSourceType;
use crate::{modules::datasource::model::datasource_type::DataSource, Db};

pub(crate) struct DataSourceGetter<'a> {
    db: &'a mut Connection<Db>,
}

impl<'a> TDataSourceGetter for DataSourceGetter<'a> {

    async fn get_datasource_by_id(&mut self, datasource_id: &str) -> Result<DataSource> {
        let query = "SELECT id, name, type FROM datasources where id = ?";

        let result = sqlx::query(query)
            .bind(datasource_id)
            .map(|row| {
                let datasource_type: String = row.get(2);
                DataSource {
                    id: row.get(0),
                    name: row.get(1),
                    datasource_type: DataSourceType::from_string(&datasource_type),
                }
            })
            .fetch_one(&mut ***self.db)
            .await?;
        Ok(result)
    }
}

impl<'a> DataSourceGetter<'a> {
    pub fn new(db: &'a mut Connection<Db>) -> Self {
        Self { db }
    }
}
