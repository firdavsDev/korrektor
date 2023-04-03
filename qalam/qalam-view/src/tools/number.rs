use crate::auth::middleware;
use crate::request::Request;
use actix_web::{get, post, web, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use korrektor::uzbek::number;
use serde_json::json;

#[get("/number")]
pub async fn main() -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "endpoint": "/number",
        "docs": "https://docs.korrektor.uz/number"
    }))
}

#[post("/number/integer")]
pub async fn integer(body: web::Json<Request>, auth: BearerAuth) -> HttpResponse {
    let content = body.into_inner().content;

    let process = number::integer_to_word(&content);

    match process {
        Ok(result) => {
            middleware(
                HttpResponse::Ok().json(json!({
                    "message": "tools/number/integer",
                    "query": content,
                    "content": result
                })),
                auth,
            )
        },
        Err(err) => {
            middleware(
                HttpResponse::BadRequest().json(json!({
                    "message": "tools/number/integer",
                    "query": content,
                    "content": err
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
        let text = "12";
        let process = number::integer_to_word(text);

        let response = match process {
            Ok(result) => {
                    json!({
                    "message": "tools/number/integer",
                    "query": text,
                    "content": result
                })
            },
            Err(err) => {
                    json!({
                    "message": "tools/number/integer",
                    "query": text,
                    "content": err
                })
            }
        };

        let static_json = "{\"content\":\"o‘n ikki\",\"message\":\"tools/number/integer\",\"query\":\"12\"}";

        assert_eq!(serde_json::to_string(&response).unwrap(), static_json);
    }
}
