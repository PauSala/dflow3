use rocket::{get, http::Status, serde::json::Json};
use rocket_db_pools::Connection;

use crate::{
    modules::dashboard::{
        infrastructure::persistence::dashboard_getter::DashboardGetter,
        model::dashboard::DashboardDto,
    },
    template_dir::{http500, Error500Template},
    Db,
};

#[get("/<user_id>")]
pub(crate) async fn get_dashboards_by_user_handler(
    mut db: Connection<Db>,
    user_id: &str,
) -> Result<Json<Vec<DashboardDto>>, (Status, Error500Template)> {
    let mut dashboard_getter = DashboardGetter::new(&mut db);
    dashboard_getter
        .get_dashboards_by_user(&user_id)
        .await
        .map(|panel| Json(panel))
        .map_err(|e| http500(e))
}
