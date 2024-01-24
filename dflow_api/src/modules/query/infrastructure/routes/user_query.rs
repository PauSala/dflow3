
use rocket::tokio::sync::RwLock;
use rocket::{post, serde::json::Json, State};
use rocket_db_pools::Connection;
use serde::Deserialize;

use crate::modules::datasource::infrastructure::persistence::sql_config_getter::SqlConfigurationGetter;
use crate::modules::datasource::model::configurations::configurations::ConfigGetter;
use crate::modules::dmodel::infrastructure::persistence::model_getter::ModelRetriever;
use crate::modules::query::application::query_runner_factory::query_runner_factory;
use crate::modules::query::application::user_query_executor::user_query_executor;
use crate::modules::query::model::query_builder::abstract_query::AbstractQuery;
use crate::modules::query::model::query_executor::QueryResult;
use crate::modules::shared::shared_state::shared_connections::SharedConnections;
use crate::Db;


#[derive(Debug, Deserialize)]
pub struct UserQueryRequest<'a> {
    query: AbstractQuery<'a>,
    datasource_id: &'a str,
}

#[post("/", data = "<user_query>")]
pub async fn user_query_handler(
    mut db: Connection<Db>,
    state: &State<RwLock<SharedConnections>>,
    user_query: Json<UserQueryRequest<'_>>,
) -> Result<Json<QueryResult>, String> {

    let mut config_retriever = SqlConfigurationGetter::new(&mut db);
    
    let model_configuration = config_retriever
        .retrieve(&user_query.datasource_id)
        .await
        .unwrap();
    let model_retriever = ModelRetriever::new(&mut db);

    let res = query_runner_factory(
        model_configuration,
        state,
        model_retriever,
        &user_query.query.model_id,
    )
    .await;

    match res {
        Ok((builder, executor)) => {
            let result = user_query_executor(builder, executor, &user_query.query).await;
            match result {
                Ok(data) => Ok(Json(data)),
                Err(e) => Err(e.to_string()),
            }
        }
        Err(e) => Err(e.to_string()),
    }
}
