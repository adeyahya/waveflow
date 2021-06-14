use crate::*;
use actix_web::*;

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

#[post("/api/users")]
pub async fn create(
    pool: web::Data<DbPool>,
    form: web::Json<models::NewUser>,
    config: web::Data<WebConfig>,
) -> impl Responder {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let encrypted_passwod =
        calculate_sha256_signature(form.password.to_owned(), config.app_secret.to_owned()).unwrap();

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
