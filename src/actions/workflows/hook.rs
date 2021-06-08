use crate::*;
use actix_web::{post, web, HttpRequest, HttpResponse, Responder};

#[post("/workflows/hook")]
async fn default(
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
            HttpResponse::Ok().body(format!("calculated: {}, received: {}", data, _signature))
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
