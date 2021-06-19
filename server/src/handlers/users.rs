use crate::*;
use actix_web::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ChangePasswordInput {
    pub password: String,
    pub new_password: String,
}

#[get("/api/users/me")]
pub async fn me(
    pool: web::Data<DbPool>,
    req: web::HttpRequest,
    config: web::Data<WebConfig>,
) -> impl Responder {
    let conn = pool.get().expect("error getting pool");

    match check_auth(&req, config.app_secret.to_owned()) {
        Some(sub) => match repository::users::get_by_username(&conn, sub).await {
            Some(user) => {
                let res = UserResponse {
                    username: user.username,
                    email: user.email,
                };
                HttpResponse::Ok().json(res)
            }
            _ => HttpResponse::NotFound().finish(),
        },
        None => HttpResponse::Unauthorized().finish(),
    }
}

#[patch("/api/users/me")]
pub async fn update_password(
    pool: web::Data<DbPool>,
    form: web::Json<ChangePasswordInput>,
    config: web::Data<WebConfig>,
    req: web::HttpRequest,
) -> impl Responder {
    let username = check_auth(&req, config.app_secret.to_owned());
    if username.is_none() {
        return HttpResponse::Unauthorized().finish();
    }
    let conn = pool.get().expect("couldn't get db connection from pool");
    let user = repository::users::get_by_username(&conn, username.to_owned().unwrap())
        .await
        .unwrap();

    match calculate_sha256_signature(form.password.to_owned(), &config.app_secret) {
        Some(calculated_password) => {
            if calculated_password != user.password {
                return HttpResponse::BadRequest().finish();
            }

            repository::users::update_password(
                &conn,
                username.unwrap(),
                form.new_password.to_owned(),
            )
            .await
            .unwrap();

            HttpResponse::Ok().finish()
        }
        None => HttpResponse::BadRequest().finish(),
    }
}

#[post("/api/users")]
pub async fn create(
    pool: web::Data<DbPool>,
    form: web::Json<models::NewUser>,
    req: web::HttpRequest,
    config: web::Data<WebConfig>,
) -> impl Responder {
    if let None = check_auth(&req, config.app_secret.to_owned()) {
        return HttpResponse::Unauthorized().finish();
    }

    let conn = pool.get().expect("couldn't get db connection from pool");
    let encrypted_passwod =
        calculate_sha256_signature(form.password.to_owned(), &config.app_secret).unwrap();

    let new_user = models::User {
        id: None,
        username: form.username.to_owned(),
        email: form.email.to_owned(),
        password: encrypted_passwod,
        is_admin: false,
    };

    match repository::users::insert(&conn, new_user.to_owned()).await {
        Ok(_) => {
            let response = UserResponse {
                username: new_user.username,
                email: new_user.email,
            };
            HttpResponse::Ok().json(response)
        }
        Err(_) => {
            let err_message = HttpErrorMessage {
                code: 400,
                message: "".to_string(),
            };

            HttpResponse::BadRequest().json(err_message)
        }
    }
}
