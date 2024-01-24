use rocket::http::Status;
use rocket::tokio::sync::RwLock;
use rocket::{post, serde::json::Json, State};
use rocket_db_pools::Connection;
use serde::Deserialize;

use crate::modules::dmodel::application::model_saver::ModelSaverService;
use crate::modules::dmodel::infrastructure::factory::model_builder_factory::model_builder_factory;
use crate::modules::dmodel::infrastructure::persistence::model_saver::ModelStorer;
use crate::modules::dmodel::model::model::Model;

use crate::modules::shared::shared_state::shared_connections::SharedConnections;
use crate::template_dir::{http500, Error500Template};
use crate::Db;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize)]
pub(crate) struct ModelRequest<'a> {
    pub(crate) datasource_id: &'a str,
    pub(crate) model_id: &'a str,
}

/// generate model from datasource (datasource and associated config must exist)  
/// ```
/// route: http://127.0.0.1:8000/model/save
///
/// {
///     "datasource_id": "test",
///     "model_id": "RustTest",
/// }
/// ```
#[post("/save", data = "<model_req>")]
pub(crate) async fn post_model_handler(
    mut db: Connection<Db>,
    shared_cns: &State<RwLock<SharedConnections>>,
    model_req: Json<ModelRequest<'_>>,
) -> Result<Json<Model>, (Status, Error500Template)> {
    let mut model_builder = model_builder_factory(&model_req.datasource_id, &mut db, shared_cns)
        .await
        .map_err(|e| http500(e))?;

    let mut model_saver = ModelStorer::new(&mut db);
    let mut saver_service = ModelSaverService::new();
    saver_service
        .run(
            &model_req.datasource_id,
            &model_req.model_id,
            &mut model_builder,
            &mut model_saver,
        )
        .await
        .map(|model| Json(model))
        .map_err(|e| http500(e))
}
