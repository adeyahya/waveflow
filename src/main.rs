#[macro_use]
extern crate diesel;
extern crate hex;

use actix_web::{post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use env_logger;
use hmac::{Hmac, Mac};
use log;
use serde::{Deserialize, Serialize};
use sha2::Sha256;

mod models;
mod schema;

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

// Create alias for HMAC-SHA256
type HmacSha256 = Hmac<Sha256>;

#[allow(dead_code)]
#[derive(Deserialize)]
struct Repository {
    html_url: String,
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct _FormData {
    repository: Repository,
}

fn verify_signature(buff: String, secret: String) -> Option<String> {
    let mut mac = HmacSha256::new_varkey(secret.as_bytes()).unwrap();
    mac.input(buff.as_bytes());
    let result = mac.result().code();
    let r2 = hex::encode(&result);

    Some(format!("sha256={}", r2))
}

fn get_signature<'a>(req: &'a HttpRequest) -> Option<&'a str> {
    req.headers().get("X-Hub-Signature-256")?.to_str().ok()
}

#[post("/users")]
async fn create_user(pool: web::Data<DbPool>, form: web::Json<models::NewUser>) -> impl Responder {
    use crate::schema::users::dsl::*;
    let conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let new_user = models::User {
        username: form.username.to_owned(),
        email: form.email.to_owned(),
        password: form.password.to_owned(),
    };

    let query = diesel::insert_into(users).values(&new_user).execute(&conn);

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct UserResponse {
        username: String,
        email: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct HttpErrorMessage {
        code: u32,
        message: String,
    }

    match query {
        Ok(_) => {
            let response = UserResponse {
                username: new_user.username,
                email: new_user.email,
            };
            HttpResponse::Ok().json(response)
        }
        Err(error) => {
            log::error!("unable to insert into database {}", error);
            let err_message = HttpErrorMessage {
                code: 402,
                message: format!("{}", error),
            };

            HttpResponse::BadRequest().json(err_message)
        }
    }
}

#[post("/deploy")]
async fn deploy(
    bytes: web::Bytes,
    // form: web::Json<FormData>,
    // web::Path(id): web::Path<String>,
    req: HttpRequest,
) -> impl Responder {
    let signature = get_signature(&req);

    match signature {
        Some(_signature) => {
            let secret = String::from("secretkey");
            let raw = String::from_utf8(bytes.to_vec()).expect("error parsing raw body");
            let verified_signature = verify_signature(raw, secret);

            match verified_signature {
                Some(data) => {
                    if data == _signature {
                        HttpResponse::Ok()
                            .body(format!("calculated: {}, received: {}", data, _signature))
                    } else {
                        HttpResponse::BadRequest().finish()
                    }
                }
                None => HttpResponse::BadRequest().finish(),
            }
        }
        None => HttpResponse::BadRequest().finish(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    dotenv::dotenv().ok();

    // set up database connection pool
    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<SqliteConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(deploy)
            .service(create_user)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
