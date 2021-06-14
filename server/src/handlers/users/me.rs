use crate::*;
use actix_web::*;

#[get("/api/users/me")]
async fn default(
    pool: web::Data<DbPool>,
    req: web::HttpRequest,
    config: web::Data<WebConfig>,
) -> impl Responder {
    use crate::schema::users::dsl::*;
    let conn = pool.get().expect("error getting pool");

    match check_auth(&req, config.app_secret.to_owned()) {
        Some(sub) => {
            match users
                .filter(username.eq(sub.to_owned()))
                .select((username, email))
                .first::<(String, String)>(&conn)
            {
                Ok(user) => {
                    let res = UserResponse {
                        username: user.0.to_owned(),
                        email: user.1.to_owned(),
                    };
                    HttpResponse::Ok().json(res)
                }
                _ => HttpResponse::NotFound().finish(),
            }
        }
        None => HttpResponse::Unauthorized().finish(),
    }
}
