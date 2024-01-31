use crate::{
    modules::dashboard::{
        infrastructure::persistence::panel_saver::PanelSaver, model::panel::PanelDto,
    },
    template_dir::{http500, Error500Template},
    Db,
};
use rocket::{http::Status, post, serde::json::Json};
use rocket_db_pools::Connection;

///
/// Example query
/// ```text
/// {
///    "model_id": "test",
///    "panel_id": "abcdefgh",
///    "user_id": "test",
///    "panel": {
///      "layout": {
///        "x": 0,
///        "y": 0,
///        "w": 0,
///        "h": 0,
///        "minH": 0,
///        "minW": 0
///      },
///      "props": {
///        "name": "foo",
///        "content_type": {
///             "Chart": {
///                 "chart_type": "bar"
///             }
///        }
///      },
///        "query": null
///    }
///  }
///```
#[post("/panel/save", data = "<panel_dto>")]
pub(crate) async fn post_panel_handler(
    mut db: Connection<Db>,
    panel_dto: Json<PanelDto>,
) -> Result<(), (Status, Error500Template)> {
    let mut panel_saver = PanelSaver::new(&mut db);
    panel_saver
        .insert_panel(&panel_dto)
        .await
        .map_err(|e| http500(e))
}
