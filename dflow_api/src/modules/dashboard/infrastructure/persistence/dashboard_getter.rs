use crate::modules::dashboard::model::dashboard::DashboardDto;
use crate::Db;
use anyhow::Result;
use rocket_db_pools::sqlx::Row;
use rocket_db_pools::{sqlx, Connection};

pub struct DashboardGetter<'a> {
    pub db: &'a mut Connection<Db>,
}

impl<'a> DashboardGetter<'a> {
    pub(crate) fn new(db: &'a mut Connection<Db>) -> Self {
        DashboardGetter { db }
    }

    pub(crate) async fn get_dashboards_by_user(
        &mut self,
        user_id: &str,
    ) -> Result<Vec<DashboardDto>> {
        let query = "SELECT id, user_id, model_id, name, config FROM dashboards where user_id = ?";
        let result = sqlx::query(query)
            .bind(user_id)
            .map(|row| {
                let config: String = row.get(4);
                let panel_dto = DashboardDto {
                    id: row.get(0),
                    user_id: row.get(1),
                    model_id: row.get(2),
                    name: row.get(3),
                    config: serde_json::from_str(&config)
                        .expect("Json should have been saved withoud issues"),
                };
                panel_dto
            })
            .fetch_all(&mut ***self.db)
            .await?;
        Ok(result)
    }
}
