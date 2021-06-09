use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use diesel::prelude::*;
use uuid::Uuid;

use crate::*;

#[post("/workflows")]
async fn default(
  pool: web::Data<DbPool>,
  form: web::Json<WorkflowRequest>,
  req: HttpRequest,
) -> impl Responder {
  use crate::schema::workflows::dsl::*;

  let sub = check_auth(&req);
  if let None = sub {
    return HttpResponse::Unauthorized().finish();
  }

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
