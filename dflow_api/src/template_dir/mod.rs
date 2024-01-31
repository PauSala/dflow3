use anyhow::Error;
use askama::Template;
use rocket::http::Status;

#[derive(Template)]
#[template(path = "error_500.html")] 

pub(crate) struct Error500Template {
    pub(crate) error: String, 
}
pub(crate) fn http500(e: Error) -> (Status, Error500Template) {
    (Status::InternalServerError, {
        Error500Template {
            error: e.to_string(),
        }
    })
}

/* #[derive(Template)]
#[template(path = "error_401.html")] 
pub(crate) struct Error401Template;

pub(crate) fn http400() -> (Status, Error401Template) {
    (Status::Forbidden, {
        Error401Template {}
    })
} */

#[derive(Template)]
#[template(path = "error_404.html")] 
pub(crate) struct Error404Template;

pub(crate) fn http404() -> (Status, Error404Template) {
    (Status::Forbidden, {
        Error404Template {}
    })
}
