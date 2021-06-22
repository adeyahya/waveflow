use crate::*;
use actix_web::*;
use dirs::cache_dir;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[get("/api/workflow/{slg}")]
pub async fn get_by_slug(
    pool: web::Data<DbPool>,
    web::Path(slg): web::Path<String>,
    req: HttpRequest,
    config: web::Data<WebConfig>,
) -> impl Responder {
    let conn = pool.get().unwrap();
    match check_auth(&req, config.app_secret.to_owned()) {
        Some(_) => match repository::workflow::get_by_slug(&conn, &slg).await {
            Ok(workflow) => HttpResponse::Ok().json(workflow),
            Err(_) => HttpResponse::InternalServerError().finish(),
        },
        None => HttpResponse::Unauthorized().finish(),
    }
}

#[get("/api/workflows/history/{id}")]
pub async fn get_single(
    pool: web::Data<DbPool>,
    web::Path(id): web::Path<String>,
) -> impl Responder {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let histories = repository::workflow_history::get_by_workflow_id(&conn, &id).await;

    HttpResponse::Ok().json(histories)
}

#[get("/api/workflows")]
pub async fn get_all(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    config: web::Data<WebConfig>,
) -> impl Responder {
    let conn = pool.get().unwrap();
    match check_auth(&req, config.app_secret.to_owned()) {
        Some(_) => {
            let result = repository::workflow::get_all(&conn)
                .await
                .unwrap_or_default();
            HttpResponse::Ok().json(result)
        }
        None => HttpResponse::Unauthorized().finish(),
    }
}

#[post("/api/workflows")]
pub async fn create(
    pool: web::Data<DbPool>,
    form: web::Json<WorkflowRequest>,
    req: HttpRequest,
    config: web::Data<WebConfig>,
) -> impl Responder {
    let conn = pool.get().unwrap();
    match check_auth(&req, config.app_secret.to_owned()) {
        Some(_) => {
            let uid = Uuid::new_v4().to_string();
            let secret = calculate_sha256_signature(uid.to_owned(), &config.app_secret).unwrap();
            let workflow = models::Workflow {
                name: form.name.to_owned(),
                slug: form.slug.to_owned(),
                secret,
                content: form.content.to_owned(),
                ..Default::default()
            };

            match repository::workflow::insert(&conn, workflow).await {
                Ok(workflow) => HttpResponse::Ok().json(workflow),
                Err(_) => HttpResponse::BadRequest().finish(),
            }
        }
        None => HttpResponse::Unauthorized().finish(),
    }
}

#[post("/api/workflows/trigger/{s}")]
pub async fn trigger(
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
            let verified_signature = calculate_sha256_signature(raw, &workflow.1);

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

#[get("/api/workflows/{id}/history")]
pub async fn get_history(
    pool: web::Data<DbPool>,
    web::Path(id): web::Path<String>,
) -> impl Responder {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let histories = repository::workflow_history::get_by_workflow_id(&conn, &id).await;

    HttpResponse::Ok().json(histories)
}
