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
    req: HttpRequest,
) -> impl Responder {
    use crate::schema::workflows::dsl::*;
    let signature = get_signature(&req);
    let conn = pool.get().expect("couldn't get db connection from pool");

    // getting the workflow content from database
    let workflow = workflows
        .filter(slug.eq(s.to_owned()))
        .select((content, secret, id))
        .first::<(String, String, String)>(&conn);
    if workflow.is_err() {
        return HttpResponse::InternalServerError().finish();
    }
    let workflow = workflow.unwrap();

    // validate signature header using hmac sha256
    match signature {
        Some(_signature) => {
            let raw = String::from_utf8(bytes.to_vec()).expect("error parsing raw body");
            let verified_signature = calculate_sha256_signature(raw, workflow.1);

            match verified_signature {
                Some(data) => {
                    if data == _signature {
                        // store the script path into cache and execute it immediately
                        let cache_dir =
                            cache_dir().or(Some(PathBuf::from("."))).unwrap().to_owned();
                        let cache_dir = cache_dir.to_str().unwrap();
                        let script_path = format!("{}/{}.sh", cache_dir, s.to_owned());
                        let fd =
                            fs::write(script_path.to_owned(), workflow.0.to_owned().into_bytes());
                        if fd.is_err() {
                            return HttpResponse::InternalServerError().finish();
                        }
                        match Command::new("sh").arg(script_path).output() {
                            Ok(output) => {
                                let log_string = String::from_utf8(output.stdout).unwrap();
                                repository::workflow_history::insert(
                                    &conn,
                                    workflow.2,
                                    Some(log_string),
                                    true,
                                )
                                .await
                                .unwrap();
                            }
                            Err(err) => {
                                repository::workflow_history::insert(
                                    &conn,
                                    workflow.2,
                                    Some(format!("{}", err)),
                                    false,
                                )
                                .await
                                .unwrap();
                            }
                        };

                        HttpResponse::Ok().finish()
                    } else {
                        HttpResponse::Unauthorized().finish()
                    }
                }
                None => HttpResponse::Unauthorized().finish(),
            }
        }
        None => HttpResponse::Unauthorized().finish(),
    }
}
