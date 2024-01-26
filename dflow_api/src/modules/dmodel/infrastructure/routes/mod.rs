pub mod post_model;
pub mod get_model;
use rocket::{routes, Route};

use self::post_model::post_model_handler;
use self::get_model::get_model_handler;

pub fn dmodel_routes() -> Vec<Route> {
    return routes![post_model_handler, get_model_handler];
}
