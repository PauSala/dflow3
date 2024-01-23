pub mod modules;
pub mod template_dir;
use modules::datasource::infrastructure::routes::datasource_routes;
use modules::shared::persistence::SqliteConnection;
use modules::shared::shared_state::shared_connections::SharedConnections;
use rocket::launch;
//use rocket::tokio::sync::RwLock;
use rocket::http::Method;
use rocket::tokio::sync::RwLock;
use rocket_cors::{AllowedOrigins, CorsOptions};
use rocket_dyn_templates::Template;

// Database state share stuff
use rocket_db_pools::sqlx::{self};
use rocket_db_pools::Database;

//sqlite database
#[derive(Database)]
#[database("models")]
pub struct Db(sqlx::SqlitePool);

#[launch]
fn rocket() -> _ {
    let sqlite = SqliteConnection::new();
    sqlite
        .create_db_if_not_exists()
        .expect("Sqlite should be available");

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
}
