use crate::{modules::dashboard::model::panel::PanelDto, Db};
use anyhow::Result;
use rocket_db_pools::{sqlx, Connection};

pub struct PanelSaver<'a> {
    pub db: &'a mut Connection<Db>,
}

impl<'a> PanelSaver<'a> {
    pub(crate) fn new(db: &'a mut Connection<Db>) -> Self {
        PanelSaver { db }
    }

    pub(crate) async fn insert_panel(&mut self, panel: &PanelDto) -> Result<()> {
        sqlx::query(
            "INSERT OR REPLACE INTO panels 
            (panel_id, user_id, model_id, panel) 
            VALUES (?, ?, ?, ?)",
        )
        .bind(panel.panel_id.to_owned())
        .bind(panel.user_id.to_owned())
        .bind(panel.model_id.to_owned())
        .bind(serde_json::to_string(&panel.panel).unwrap())
        .execute(&mut ***self.db)
        .await?;
        Ok(())
    }
}
