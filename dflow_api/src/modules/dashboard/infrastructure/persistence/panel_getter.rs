use crate::{modules::dashboard::model::panel::PanelDto, Db};
use anyhow::Result;
use rocket_db_pools::sqlx::Row;
use rocket_db_pools::{sqlx, Connection};

pub struct PanelGetter<'a> {
    pub db: &'a mut Connection<Db>,
}

impl<'a> PanelGetter<'a> {
    pub(crate) fn new(db: &'a mut Connection<Db>) -> Self {
        PanelGetter { db }
    }

    pub(crate) async fn get_panel(&mut self, panel_id: &str) -> Result<PanelDto> {
        let query = "SELECT panel_id, user_id, model_id, panel FROM panels where panel_id = ?";
        let result = sqlx::query(query)
            .bind(panel_id)
            .map(|row| {
                let panel: String = row.get(3);
                let panel_dto = PanelDto{
                    panel_id: row.get(0),
                    user_id: row.get(1),
                    model_id: row.get(2),
                    panel: serde_json::from_str(&panel).expect("Json should have been saved withoud issues"),
                };
                panel_dto
            })
            .fetch_one(&mut ***self.db)
            .await?;
        Ok(result)
    }
}
