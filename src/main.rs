#[macro_use]
extern crate diesel;
extern crate hex;

use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use deployer::*;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use env_logger;
use log;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

mod models;
mod schema;

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[derive(Clone)]
struct WebConfig {
    app_secret: String,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
struct WorkflowRequest {
    slug: String,
    content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LoginResponse {
    username: String,
    email: String,
    access_token: String,
}

fn get_signature<'a>(req: &'a HttpRequest) -> Option<&'a str> {
    req.headers().get("X-Hub-Signature-256")?.to_str().ok()
}

#[post("/auth/login")]
async fn auth_login(
    pool: web::Data<DbPool>,
    form: web::Json<LoginRequest>,
    config: web::Data<WebConfig>,
) -> impl Responder {
    use crate::schema::users::dsl::*;

    let conn = pool.get().expect("couldn't get db connection from pool");
    match users
        .filter(username.eq(form.username.to_owned()))
        .select((password, email))
        .first::<(String, String)>(&conn)
    {
        Ok(user) => {
            match calculate_sha256_signature(form.password.to_owned(), config.app_secret.to_owned())
            {
                Some(calculated_password) => {
                    if user.0 == calculated_password {
                        let res = LoginResponse {
                            username: form.username.to_owned(),
                            email: user.1,
                            access_token: generate_jwt_token(
                                config.app_secret.to_owned(),
                                form.username.to_owned(),
                            )
                            .unwrap(),
                        };

                        HttpResponse::Ok().json(res)
                    } else {
                        HttpResponse::Unauthorized().finish()
                    }
                }
                None => HttpResponse::InternalServerError().finish(),
            }
        }
        _ => {
            let err_message = HttpErrorMessage {
                code: 400,
                message: format!("user with username {} not found!", form.username),
            };
            HttpResponse::BadRequest().json(err_message)
        }
    }
}

#[post("/workflows")]
async fn create_workflow(
    pool: web::Data<DbPool>,
    form: web::Json<WorkflowRequest>,
) -> impl Responder {
    use crate::schema::workflows::dsl::*;

    let conn = pool.get().expect("couldn't get db connection from pool");

    let workflow = models::Workflow {
        id: Uuid::new_v4().to_string(),
        slug: form.slug.to_owned(),
        secret: Uuid::new_v4().to_string(),
        content: form.content.to_owned(),
    };

    if let Err(err) = diesel::insert_into(workflows)
        .values(&workflow)
        .execute(&conn)
    {
        let err_message = HttpErrorMessage {
            code: 400,
            message: format!("{}", err),
        };
        HttpResponse::BadRequest().json(err_message)
    } else {
        HttpResponse::Ok().json(workflow)
    }
}

#[get("/workflows")]
async fn get_workflows(pool: web::Data<DbPool>) -> impl Responder {
    use crate::schema::workflows::dsl::*;

    let conn = pool.get().expect("couldn't get db connection from pool");

    match workflows.load::<(String, String, String, String)>(&conn) {
        Ok(results) => {
            let result_vec: Vec<models::Workflow> = results
                .into_iter()
                .map(|x| models::Workflow {
                    id: x.0.to_owned(),
                    slug: x.1.to_owned(),
                    secret: x.2.to_owned(),
                    content: x.3.to_owned(),
                })
                .rev()
                .collect();

            HttpResponse::Ok().json(result_vec)
        }
        _ => {
            let err_message = HttpErrorMessage {
                code: 400,
                message: "internal error".to_owned(),
            };

            HttpResponse::InternalServerError().json(err_message)
        }
    }
}

#[post("/users")]
async fn create_user(
    pool: web::Data<DbPool>,
    form: web::Json<models::NewUser>,
    config: web::Data<WebConfig>,
) -> impl Responder {
    use crate::schema::users::dsl::*;
    let conn = pool.get().expect("couldn't get db connection from pool");

    let encrypted_passwod =
        calculate_sha256_signature(form.password.to_owned(), config.app_secret.to_owned()).unwrap();

    let new_user = models::User {
        username: form.username.to_owned(),
        email: form.email.to_owned(),
        password: encrypted_passwod,
    };

    let query = diesel::insert_into(users).values(&new_user).execute(&conn);

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
                code: 400,
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
            let verified_signature = calculate_sha256_signature(raw, secret);

            match verified_signature {
                Some(data) => {
                    if format!("sha256={}", data) == _signature {
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
            .service(deploy)
            .service(create_user)
            .service(create_workflow)
            .service(get_workflows)
            .service(auth_login)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
