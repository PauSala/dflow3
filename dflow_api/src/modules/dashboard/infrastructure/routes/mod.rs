pub mod get_dashbaords;
pub mod get_panel;
pub mod post_dashboard;
pub mod post_panel;

use crate::modules::dashboard::infrastructure::routes::get_dashbaords::get_dashboards_by_user_handler;
use crate::modules::dashboard::infrastructure::routes::get_panel::get_panel_handler;
use crate::modules::dashboard::infrastructure::routes::post_dashboard::post_dashboard_handler;
use crate::modules::dashboard::infrastructure::routes::post_panel::post_panel_handler;
use rocket::{routes, Route};

pub fn dashboard_routes() -> Vec<Route> {
    return routes![
        post_panel_handler,
        get_panel_handler,
        get_dashboards_by_user_handler,
        post_dashboard_handler
    ];
}
