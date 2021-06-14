use crate::*;
use actix_web::*;

#[get("/api/users/me")]
async fn default(
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
