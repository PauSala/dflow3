pub mod modules;
pub mod template_dir;

use modules::dashboard::infrastructure::routes::dashboard_routes;
use modules::datasource::infrastructure::routes::datasource_routes;
use modules::dmodel::infrastructure::routes::dmodel_routes;
use modules::query::infrastructure::routes::user_query_routes;
use modules::shared::persistence::SqliteConnection;
use modules::shared::shared_state::shared_connections::SharedConnections;
use rocket::{get, launch, routes};
use rocket::http::{Method, Status};
use rocket::tokio::sync::RwLock;
use rocket_cors::{AllowedOrigins, CorsOptions};
use rocket_dyn_templates::Template;

// Database state share stuff
use rocket_db_pools::sqlx::{self};
use rocket_db_pools::Database;

use crate::modules::shared::auth::jwt::UserClaim;
//sqlite database
#[derive(Database)]
#[database("models")]
pub struct Db(sqlx::SqlitePool);

// test some auth routes -------------------------------------------

#[get("/")]
async fn index() -> String {
    let user_claim = UserClaim {
        id: format!("hello_rocket_jwt"),
    };
    let token = UserClaim::sign(user_claim);
    println!("{:#?}", UserClaim::decode(token.clone()));
    token
}

#[get("/user_id")]
fn get_user_id_from_jwt(_user: UserClaim) -> Result<String, (Status, String)> {
    Ok(String::from("hello!"))
}

//END test some auth routes -------------------------------------------

#[launch]
fn rocket() -> _ {
    // Init sqlite DB
    let sqlite = SqliteConnection::new();
    sqlite
        .create_db_if_not_exists()
        .expect("Sqlite should be available");

    // Set cors
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![
                Method::Get,
                Method::Post,
                Method::Patch,
                Method::Options,
                Method::Delete,
            ]
            .into_iter()
            .map(From::from)
            .collect(),
        )
        .allow_credentials(true);

    rocket::build()
        .attach(cors.to_cors().unwrap())
        .attach(Db::init())
        .attach(Template::fairing())
        .manage(RwLock::new(SharedConnections {
            postgres_pool: None,
            mssql_pool: None,
        }))
        .mount("/datasource", datasource_routes())
        .mount("/model", dmodel_routes())
        .mount("/query", user_query_routes())
        .mount("/dashboard", dashboard_routes())
        .mount("/", routes![index, get_user_id_from_jwt])
}
