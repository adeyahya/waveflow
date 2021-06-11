extern crate diesel;
extern crate hex;

use actix_cors::Cors;
use actix_web::{middleware::Logger, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use env_logger;
use waveflow::*;

mod actions;

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
            .service(actions::users::create::default)
            .service(actions::users::me::default)
            // workflow services
            .service(actions::workflows::create::default)
            .service(actions::workflows::get_all::default)
            .service(actions::workflows::trigger::default)
            // auth services
            // POST auth/login
            .service(actions::auth::login::default)
    })
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await
}
