mod security;
// use crate::security::{hash_password, verify_password};
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
mod validation; 
use std::env;
mod user;
mod repositary;
mod user_handler;
mod jwt;
mod errors; 
use crate::user_handler::{register_user, login_user};
use dotenvy::dotenv;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // Load .env

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await.expect("Failed to connect to DB");

    let port = env::var("SERVER_PORT").unwrap_or("8080".to_string());
    let port: u16 = port.parse().expect("PORT must be a number");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/register", web::post().to(register_user))
            .route("/login", web::post().to(login_user))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await

}

