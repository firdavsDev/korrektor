use crate::auth::middleware;
use crate::request::Request;
use actix_web::{get, post, web, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use korrektor::uzbek::alphabetic;
use serde_json::json;

#[get("/alphabetic")]
pub async fn main() -> HttpResponse {
    HttpResponse::BadRequest().json(json!({
        "endpoint": "/alphabetic",
        "docs": "https://docs.korrektor.uz/alphabetic"
    }))
}

#[post("/alphabetic")]
pub async fn content(body: web::Json<Request>, auth: BearerAuth) -> HttpResponse {
    let content = body.into_inner().content;

    let process = alphabetic::sort(content.as_str());

    match process {
        Ok(result) => middleware(
            HttpResponse::Ok().json(json!({
                "message": "/alphabetic",
                "content": result
            })),
            auth,
        ),
        Err(err) => {
            let error = err.to_string();
            middleware(
                HttpResponse::BadRequest().json(json!({
                    "message": "/alphabetic",
                    "content": error
                })),
                auth,
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_web::test]
    async fn content_test() {
        let text = "G‘ozal estafeta chilonzor o'zbek chiroyli";
        let process = alphabetic::sort(text);

        let response = match process {
            Ok(result) => {
                json!({
                    "message": "/alphabetic",
                    "content": result
                })
            }
            Err(err) => {
                json!({
                    "message": "/alphabetic",
                    "content": err
                })
            }
        };

        let static_json =
            "{\"content\":\"estafeta o‘zbek chilonzor chiroyli G‘ozal\",\"message\":\"/alphabetic\"}";

        assert_eq!(serde_json::to_string(&response).unwrap(), static_json);
    }
}
