use rocket::{get, http::Status, serde::json::Json};
use rocket_db_pools::Connection;

use crate::{
    modules::dmodel::{
        infrastructure::persistence::model_getter::ModelGetter, model::model::Model,
    },
    template_dir::{http404, Error404Template},
    Db,
};

#[get("/<model_id>")]
pub(crate) async fn get_model_handler(
    mut db: Connection<Db>,
    model_id: &str,
) -> Result<Json<Model>, (Status, Error404Template)> {
    let mut model_retriever = ModelGetter { db: &mut db };
    model_retriever
        .retrieve(model_id)
        .await
        .map(|model| Json(model))
        .map_err(|_| http404())
}
