pub mod post_config;
pub mod post_datasource;

use crate::modules::datasource::infrastructure::routes::post_config::post_config_handler;
use crate::modules::datasource::infrastructure::routes::post_datasource::post_datasource_handler;
use rocket::{routes, Route};

pub fn datasource_routes() -> Vec<Route> {
    return routes![post_config_handler, post_datasource_handler];
}
