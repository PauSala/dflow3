pub mod modules;
pub mod template_dir;

use modules::datasource::infrastructure::routes::datasource_routes;
use modules::dmodel::infrastructure::routes::dmodel_routes;
use modules::query::infrastructure::routes::user_query_routes;
use modules::shared::persistence::SqliteConnection;
use modules::shared::shared_state::shared_connections::SharedConnections;
use rocket::response::Redirect;
use rocket::{get, launch, routes};
//use rocket::tokio::sync::RwLock;
use rocket::http::Method;
use rocket::tokio::sync::RwLock;
use rocket::uri;
use rocket_cors::{AllowedOrigins, CorsOptions};
use rocket_dyn_templates::Template;

// Database state share stuff
use rocket_db_pools::sqlx::{self};
use rocket_db_pools::Database;
use template_dir::Error401Template;

use crate::modules::shared::auth::jwt::UserClaim;

//sqlite database
#[derive(Database)]
#[database("models")]
pub struct Db(sqlx::SqlitePool);

// test some auth routes -------------------------------------------

#[get("/")]
fn index() -> String {
    let user_claim = UserClaim {
        id: format!("hello_rocket_jwt"),
    };
    let token = UserClaim::sign(user_claim);
    println!("{:#?}", UserClaim::decode(token.clone()));
    token
}

#[get("/user_id")]
fn get_uer_id_from_jwt(user: Option<UserClaim>) -> Result<String, Redirect> {
    let user = user.ok_or_else(|| Redirect::to(uri!(not_authorized)));
    match user {
        Ok(user) => Ok(format!("user id is {}", user.id.clone())),
        Err(_) => Err(Redirect::to(uri!(not_authorized))),
    }
}

#[get("/not-authorized")]
fn not_authorized() -> Error401Template {
    Error401Template {}
}

//END test some auth routes -------------------------------------------

#[launch]
fn rocket() -> _ {
    // Init main DB
    let sqlite = SqliteConnection::new();
    sqlite
        .create_db_if_not_exists()
        .expect("Sqlite should be available");

    // Set cors
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Patch]
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
        .mount("/", routes![index, get_uer_id_from_jwt, not_authorized])
}
