extern crate diesel;
extern crate hex;

use actix_cors::Cors;
use actix_files::Files;
use actix_files::NamedFile;
use actix_web::{middleware::Logger, web, App, HttpServer, Result};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use env_logger;
use uuid::Uuid;
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

    // set up database connection pool
    let connspec = std::env::var("DATABASE_URL").unwrap_or("./waveflow.db".to_string());
    let port = std::env::var("PORT").unwrap_or("3000".to_string());

    println!("running on port {}", port);

    let manager = ConnectionManager::<SqliteConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    use crate::schema::configs::dsl::*;
    let conn = pool.get().expect("couldn't get db connection from pool");

    // get or create app_secret if not exist
    let app_secret = match configs
        .filter(name.eq("app_secret"))
        .select(value)
        .first::<String>(&conn)
    {
        Ok(app_secret) => app_secret,
        Err(_) => {
            let secret_str = Uuid::new_v4().to_string();
            let secret = models::Config {
                id: None,
                name: String::from("app_secret"),
                value: secret_str.to_owned(),
            };

            let query = diesel::insert_into(configs).values(&secret).execute(&conn);
            if query.is_ok() {
                secret_str
            } else {
                panic!("error creating app_secret");
            }
        }
    };

    // creating the first admin if not exist
    if let None = repository::users::get_by_username(&conn, "admin".to_owned()).await {
        let encrypted_password =
            calculate_sha256_signature(String::from("admin"), app_secret.to_owned()).unwrap();
        let admin = models::User {
            id: None,
            username: String::from("admin"),
            email: String::from("admin@waveflow.io"),
            password: encrypted_password,
            is_admin: true,
        };

        match repository::users::insert(&conn, admin).await {
            Ok(_) => {}
            _ => {
                panic!("error creating the first admin user");
            }
        }
    };

    let web_config = WebConfig { app_secret };

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
