use crate::{
  calculate_sha256_signature, models, DbPool, HttpErrorMessage, UserResponse, WebConfig,
};
use actix_web::{post, web, HttpResponse, Responder};
use diesel::prelude::*;

#[post("/users")]
async fn default(
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
    is_admin: false,
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
