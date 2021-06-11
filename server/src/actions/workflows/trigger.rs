use crate::*;
use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use dirs::cache_dir;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[post("/api/workflows/trigger/{s}")]
async fn default(
    bytes: web::Bytes,
    pool: web::Data<DbPool>,
    web::Path(s): web::Path<String>,
    config: web::Data<WebConfig>,
    req: HttpRequest,
) -> impl Responder {
    use crate::schema::workflows::dsl::*;
    let signature = get_signature(&req);
    let conn = pool.get().expect("couldn't get db connection from pool");

    // getting the workflow content from database
    let workflow = workflows
        .filter(slug.eq(s.to_owned()))
        .select(content)
        .first::<String>(&conn);
    if workflow.is_err() {
        return HttpResponse::InternalServerError().finish();
    }
    let workflow = workflow.unwrap();

    // validate signature header using hmac sha256
    match signature {
        Some(_signature) => {
            let raw = String::from_utf8(bytes.to_vec()).expect("error parsing raw body");
            let verified_signature = calculate_sha256_signature(raw, config.app_secret.to_owned());

            match verified_signature {
                Some(data) => {
                    if data == _signature {
                        // store the script path into
                        // cache folder and execute it immediately
                        let cache_dir =
                            cache_dir().or(Some(PathBuf::from("."))).unwrap().to_owned();
                        let cache_dir = cache_dir.to_str().unwrap();
                        let script_path = format!("{}/{}.sh", cache_dir, s.to_owned());
                        let fd =
                            fs::write(script_path.to_owned(), workflow.to_owned().into_bytes());
                        if fd.is_err() {
                            return HttpResponse::InternalServerError().finish();
                        }
                        let output = Command::new("sh").arg(script_path).spawn();
                        if output.is_err() {
                            return HttpResponse::InternalServerError().finish();
                        }

                        HttpResponse::Ok()
                            .body(format!("calculated: {}, received: {}", data, _signature))
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
