use actix_web::{web, http, HttpResponse, Responder};
use crate::models::UserData;

pub async fn index(user: web::Data<UserData>) -> impl Responder {
    HttpResponse::Ok()
        .insert_header((http::header::SERVER, "Cowboy/1.0"))
        .body(format!("Hello, {}!", user.username))
}
