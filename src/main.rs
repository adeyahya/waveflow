#[macro_use]
extern crate diesel;
extern crate hex;

use actix_web::{App, HttpServer};
use deployer::*;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use env_logger;

mod actions;
mod models;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    dotenv::dotenv().ok();

    let web_config = WebConfig {
        app_secret: std::env::var("APP_SECRET").expect("APP_SECRET"),
    };

    // set up database connection pool
    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<SqliteConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .data(web_config.clone())
            // users services
            .service(actions::users::create::default)
            // workflow services
            .service(actions::workflows::create::default)
            .service(actions::workflows::get_all::default)
            .service(actions::workflows::hook::default)
            // auth services
            // POST auth/login
            .service(actions::auth::login::default)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
