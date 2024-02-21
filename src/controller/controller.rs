use actix_web::{http, HttpResponse, HttpResponseBuilder, Responder};

use crate::dto;

pub async fn subscription(actix_web::web::Form(form): actix_web::web::Form<dto::User>) -> impl Responder {
    let mut err_resp_builder = HttpResponseBuilder::new(http::StatusCode::from_u16(400).unwrap());
    let mut suc_resp_builder = HttpResponseBuilder::new(http::StatusCode::from_u16(200).unwrap());
    if form.get_name().is_empty() && form.get_email().is_empty() {
        err_resp_builder.body("name and email not found")
    } else if form.get_name().is_empty() {
        err_resp_builder.body("name and email not found")
    } else if form.get_email().is_empty() {
        err_resp_builder.body("name and email not found")
    } else {
        suc_resp_builder.body("subscription successful")
    }
}

pub async fn health() -> impl Responder {
    HttpResponse::Ok().finish()
}

