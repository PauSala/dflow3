use crate::{modules::dashboard::model::dashboard::DashboardDto, Db};
use anyhow::Result;
use rocket_db_pools::{sqlx, Connection};

pub struct DashboardSaver<'a> {
    pub db: &'a mut Connection<Db>,
}

impl<'a> DashboardSaver<'a> {
    pub(crate) fn new(db: &'a mut Connection<Db>) -> Self {
        DashboardSaver { db }
    }

    pub(crate) async fn insert_dashboard(&mut self, dashbaord: &DashboardDto) -> Result<()> {
        sqlx::query(
            "INSERT OR REPLACE INTO dashboards 
            (id, user_id, model_id, name, config) 
            VALUES (?, ?, ?, ?, ?)",
        )
        .bind(dashbaord.id.to_owned())
        .bind(dashbaord.user_id.to_owned())
        .bind(dashbaord.model_id.to_owned())
        .bind(dashbaord.name.to_owned())
        .bind(serde_json::to_string(&dashbaord.config).unwrap())
        .execute(&mut ***self.db)
        .await?;
        Ok(())
    }
}
