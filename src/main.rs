mod security;
// use crate::security::{hash_password, verify_password};
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;

mod user;
mod repositary;
mod user_handler;
use crate::user_handler::{register_user, login_user};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Connect to DB
    
    let database_url = "postgres://vimal:12345@localhost/vimal_db";
    let pool = PgPool::connect(database_url).await.expect("Failed to connect to DB");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/register", web::post().to(register_user))
            .route("/login", web::post().to(login_user))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

