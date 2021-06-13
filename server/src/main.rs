extern crate diesel;
extern crate hex;

use actix_cors::Cors;
use actix_files::Files;
use actix_files::NamedFile;
use actix_web::{middleware::Logger, web, App, HttpServer, Result};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use env_logger;
use waveflow::*;

mod handlers;
mod repository;

async fn p404() -> Result<NamedFile> {
    Ok(NamedFile::open("./frontend/index.html")?)
}

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
    let port = std::env::var("PORT").unwrap_or("3000".to_string());

    println!("running on port {}", port);

    let manager = ConnectionManager::<SqliteConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    // creating the first admin if not exist
    use crate::schema::users::dsl::*;
    let conn = pool.get().expect("couldn't get db connection from pool");
    match users
        .filter(username.eq("admin"))
        .select(username)
        .first::<String>(&conn)
    {
        Ok(_) => {}
        _ => {
            let encrypted_password =
                calculate_sha256_signature(String::from("admin"), web_config.app_secret.to_owned())
                    .unwrap();
            let admin = models::User {
                username: String::from("admin"),
                email: String::from("admin@waveflow.io"),
                password: encrypted_password,
                is_admin: true,
            };

            match diesel::insert_into(users).values(&admin).execute(&conn) {
                Ok(_) => {}
                _ => {
                    panic!("error creating the first admin user");
                }
            }
        }
    };

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .supports_credentials()
            .allow_any_header();
        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .data(pool.clone())
            .data(web_config.clone())
            // users services
            .service(handlers::users::create::default)
            .service(handlers::users::me::default)
            // workflow services
            .service(handlers::workflows::create::default)
            .service(handlers::workflows::get_all::default)
            .service(handlers::workflows::trigger::default)
            .service(handlers::workflows::history::default)
            // auth services
            // POST auth/login
            .service(handlers::auth::login::default)
            .service(Files::new("/assets", "./frontend/assets/").prefer_utf8(true))
            .service(
                Files::new("/", "./frontend/index.html")
                    .default_handler(web::get().to(p404))
                    .prefer_utf8(true),
            )
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
