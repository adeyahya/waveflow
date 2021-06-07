extern crate hex;
use actix_web::{post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use hmac::{Hmac, Mac};
use serde::Deserialize;
use sha2::Sha256;

// Create alias for HMAC-SHA256
type HmacSha256 = Hmac<Sha256>;

#[derive(Deserialize)]
struct Repository {
    html_url: String,
}

#[derive(Deserialize)]
struct FormData {
    repository: Repository,
}

fn verify_signature(buff: String, secret: String) -> Option<String> {
    let mut mac = HmacSha256::new_varkey(secret.as_bytes()).unwrap();
    mac.input(buff.as_bytes());
    let result = mac.result().code();
    let r2 = hex::encode(&result);

    Some(format!("sha256={}", r2))
}

fn get_signature<'a>(req: &'a HttpRequest) -> Option<&'a str> {
    req.headers().get("X-Hub-Signature-256")?.to_str().ok()
}

#[post("/deploy")]
async fn deploy(
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
            let verified_signature = verify_signature(raw, secret);

            match verified_signature {
                Some(data) => {
                    if data == _signature {
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(deploy))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
