use rocket::{routes, Route};

use self::user_query::user_query_handler;

pub mod user_query;

pub fn user_query_routes() -> Vec<Route> {
    return routes![user_query_handler];
}
