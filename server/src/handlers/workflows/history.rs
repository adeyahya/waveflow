use crate::*;
use actix_web::{get, HttpResponse, Responder};

#[get("/api/workflows/history/{id}")]
async fn default(pool: web::Data<DbPool>, web::Path(id): web::Path<String>) -> impl Responder {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let histories = repository::workflow_history::get_by_workflow_id(&conn, &id).await;

    HttpResponse::Ok().json(histories)
}
