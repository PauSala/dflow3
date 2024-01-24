use rocket::http::Status;
use rocket::tokio::sync::RwLock;
use rocket::{post, serde::json::Json, State};
use rocket_db_pools::Connection;
use serde::Deserialize;

use crate::modules::datasource::infrastructure::factory::configuration_factory::configuration_factory;
use crate::modules::dmodel::infrastructure::persistence::model_getter::ModelGetter;
use crate::modules::query::application::user_query_executor::user_query_executor;
use crate::modules::query::infrastructure::factory::query_runner_factory::query_runner_factory;
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

/// Handles user queries and executes them against the database.
///
/// # Example
///
/// ```json
/// {
///     "datasource_id": "test",
///     "query": {
///         "columns": [
///             {
///                 "table_id": 0,
///                 "column_id": 1,
///                 "table_name": "categories",
///                 "column_name": "categoryname",
///                 "aggregation": null,
///                 "format": null,
///                 "order": "Asc",
///                 "data_type": "Text"
///             },
///             {
///                 "table_id": 5,
///                 "column_id": 3,
///                 "table_name": "products",
///                 "column_name": "productname",
///                 "aggregation": null,
///                 "format": null,
///                 "order": "Asc",
///                 "data_type": "Text"
///             }
///         ],
///         "joins": [
///             {
///                 "main_table_id": 5,
///                 "join_table": 0,
///                 "main_field": 0,
///                 "join_field": 0
///             }
///         ],
///         "model_id": "test",
///                "filters": [
///                     {
///                         "column_name": "productname",
///                         "table_name": "products",
///                         "column_id": 3,
///                         "table_id": 5,
///                         "operator": "NotEq",
///                         "value": { "UniValue": {"Text":"Spegesild"}},
///                         "data_type": "Text"
///                     }
///                ]
///     }
/// }
/// ```
///
/// # Arguments
///
/// - `db`: A mutable reference to the database connection.
/// - `state`: A reference to the shared state containing user's database connections.
/// - `user_query`: A JSON payload representing the user query.
///
/// # Returns
///
/// Returns a `Result` containing either the JSON result of the query or an HTTP 500 error.

#[post("/", data = "<user_query>")]
pub(crate) async fn user_query_handler(
    mut db: Connection<Db>,
    state: &State<RwLock<SharedConnections>>,
    user_query: Json<UserQueryRequest<'_>>,
) -> Result<Json<QueryResult>, (Status, Error500Template)> {
    let model_configuration = configuration_factory(user_query.datasource_id, &mut db)
        .await
        .map_err(|e| http500(e))?;
    let model_retriever = ModelGetter::new(&mut db);

    let (builder, executor) = query_runner_factory(
        model_configuration,
        state,
        model_retriever,
        &user_query.query.model_id,
    )
    .await
    .map_err(|e| http500(e))?;

    let result = user_query_executor(builder, executor, &user_query.query).await;
    result.map(|e| Json(e)).map_err(|e| http500(e))
}
