use crate::*;
use actix_web::*;
use dirs::cache_dir;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

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
    use crate::schema::workflows::dsl::*;

    let sub = check_auth(&req, config.app_secret.to_owned());
    if let None = sub {
        return HttpResponse::Unauthorized().finish();
    }

    let conn = pool.get().expect("couldn't get db connection from pool");

    match workflows.select((id, name, slug, secret, content)).load::<(
        String,
        String,
        String,
        String,
        String,
    )>(&conn)
    {
        Ok(results) => {
            let result_vec: Vec<models::Workflow> = results
                .into_iter()
                .map(|x| models::Workflow {
                    id: x.0.to_owned(),
                    name: x.1.to_owned(),
                    slug: x.2.to_owned(),
                    secret: x.3.to_owned(),
                    content: x.4.to_owned(),
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

#[post("/api/workflow")]
pub async fn create(
    pool: web::Data<DbPool>,
    form: web::Json<WorkflowRequest>,
    req: HttpRequest,
    config: web::Data<WebConfig>,
) -> impl Responder {
    use crate::schema::workflows::dsl::*;

    let sub = check_auth(&req, config.app_secret.to_owned());
    if let None = sub {
        return HttpResponse::Unauthorized().finish();
    }

    let conn = pool.get().expect("couldn't get db connection from pool");
    let sha_256_secret =
        calculate_sha256_signature(Uuid::new_v4().to_string(), config.app_secret.to_owned())
            .unwrap();
    let workflow = models::Workflow {
        id: Uuid::new_v4().to_string(),
        name: form.name.to_owned(),
        slug: form.slug.to_owned(),
        secret: sha_256_secret,
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

#[get("/api/workflows/history/{id}")]
pub async fn get_history(
    pool: web::Data<DbPool>,
    web::Path(id): web::Path<String>,
) -> impl Responder {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let histories = repository::workflow_history::get_by_workflow_id(&conn, &id).await;

    HttpResponse::Ok().json(histories)
}
