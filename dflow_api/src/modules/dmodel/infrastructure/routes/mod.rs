pub mod post_model;
use rocket::{routes, Route};

use self::post_model::post_model_handler;

pub fn dmodel_routes() -> Vec<Route> {
    return routes![post_model_handler];
}
