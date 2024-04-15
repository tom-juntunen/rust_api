mod handlers;
mod middleware;
mod models;

use actix_web::{web, App, HttpServer, middleware::Logger};
use handlers::*;
use middleware::AuthMiddleware;
use models::UserData;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let user_data = web::Data::new(UserData {
        username: "john_doe".to_string(),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(user_data.clone())
            .wrap(Logger::default())
            .wrap(AuthMiddleware) 
            .service(web::resource("/").route(web::get().to(index)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
