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
