use crate::*;
use actix_web::{get, web, HttpResponse, Responder};
use diesel::r2d2::{self, ConnectionManager};

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[get("/workflows")]
async fn default(pool: web::Data<DbPool>) -> impl Responder {
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
