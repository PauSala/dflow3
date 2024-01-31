use rocket::{get, http::Status, serde::json::Json};
use rocket_db_pools::Connection;

use crate::{
    modules::dashboard::{
        infrastructure::persistence::panel_getter::PanelGetter, model::panel::PanelDto,
    },
    template_dir::{http500, Error500Template},
    Db,
};

#[get("/panel/<panel_id>")]
pub(crate) async fn get_panel_handler(
    mut db: Connection<Db>,
    panel_id: &str,
) -> Result<Json<PanelDto>, (Status, Error500Template)> {
    let mut panel_getter = PanelGetter::new(&mut db);
    panel_getter
        .get_panel(&panel_id)
        .await
        .map(|panel| Json(panel))
        .map_err(|e| http500(e))
}
