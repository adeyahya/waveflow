use crate::*;
use actix_web::{http, post, web, HttpResponse, Responder};

#[post("/auth/login")]
async fn default(
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
                        let token = generate_jwt_token(
                            config.app_secret.to_owned(),
                            form.username.to_owned(),
                        )
                        .unwrap();
                        let res = LoginResponse {
                            username: form.username.to_owned(),
                            email: user.1,
                        };

                        HttpResponse::Ok()
                            .cookie(
                                http::Cookie::build("access_token", token)
                                    .path("/")
                                    .http_only(true)
                                    .finish(),
                            )
                            .json(res)
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
