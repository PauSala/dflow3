use crate::{
    modules::dashboard::{
        infrastructure::persistence::dashboard_saver::DashboardSaver,
        model::dashboard::DashboardDto,
    },
    template_dir::{http500, Error500Template},
    Db,
};
use rocket::{http::Status, post, serde::json::Json};
use rocket_db_pools::Connection;

#[post("/save", data = "<dashboard_dto>")]
pub(crate) async fn post_dashboard_handler(
    mut db: Connection<Db>,
    dashboard_dto: Json<DashboardDto>,
) -> Result<(), (Status, Error500Template)> {
    let mut dashboard_saver = DashboardSaver::new(&mut db);
    dashboard_saver
        .insert_dashboard(&dashboard_dto)
        .await
        .map_err(|e| http500(e))
}
