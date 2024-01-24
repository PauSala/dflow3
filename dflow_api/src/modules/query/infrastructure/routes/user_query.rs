use rocket::http::Status;
use rocket::tokio::sync::RwLock;
use rocket::{post, serde::json::Json, State};
use rocket_db_pools::Connection;
use serde::Deserialize;

use crate::modules::datasource::application::configuration_factory::configuration_factory;
use crate::modules::dmodel::infrastructure::persistence::model_getter::ModelGetter;
use crate::modules::query::application::query_runner_factory::query_runner_factory;
use crate::modules::query::application::user_query_executor::user_query_executor;
use crate::modules::query::model::query_builder::abstract_query::AbstractQuery;
use crate::modules::query::model::query_executor::QueryResult;
use crate::modules::shared::shared_state::shared_connections::SharedConnections;
use crate::template_dir::{http500, Error500Template};
use crate::Db;

#[derive(Debug, Deserialize)]
pub struct UserQueryRequest<'a> {
    query: AbstractQuery<'a>,
    datasource_id: &'a str,
}

#[post("/", data = "<user_query>")]
pub(crate) async fn user_query_handler(
    mut db: Connection<Db>,
    state: &State<RwLock<SharedConnections>>,
    user_query: Json<UserQueryRequest<'_>>,
) -> Result<Json<QueryResult>, (Status, Error500Template)> {
    let model_configuration = configuration_factory(user_query.datasource_id, &mut db).await;
    let model_retriever = ModelGetter::new(&mut db);

    let res = query_runner_factory(
        model_configuration.map_err(|e| http500(e))?,
        state,
        model_retriever,
        &user_query.query.model_id,
    )
    .await;

    let (builder, executor) = res.map_err(|e| http500(e))?;
    let result = user_query_executor(builder, executor, &user_query.query).await;
    result.map(|e| Json(e)).map_err(|e| http500(e))
}
