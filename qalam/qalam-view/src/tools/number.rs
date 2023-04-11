use crate::auth::middleware;
use crate::request::Request;
use actix_web::{get, post, web, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use korrektor::uzbek::number;
use serde_json::json;

#[get("/number")]
pub async fn main() -> HttpResponse {
    HttpResponse::BadRequest().json(json!({
        "endpoint": "/number",
        "docs": "https://docs.korrektor.uz/number"
    }))
}

#[post("/number/content")]
pub async fn content(body: web::Json<Request>, auth: BearerAuth) -> HttpResponse {
    let text_content = body.into_inner().content;

    let process = number::numbers_to_word(&text_content);

    match process {
        Ok(result) => middleware(
            HttpResponse::Ok().json(json!({
                "message": "tools/number/content",
                "content": result
            })),
            auth,
        ),
        Err(err) => {
            let error = err.to_string();
            middleware(
                HttpResponse::BadRequest().json(json!({
                    "message": "tools/number/content",
                    "query": text_content,
                    "content": error
                })),
                auth,
            )
        }
    }
}

#[post("/number/integer")]
pub async fn integer(body: web::Json<Request>, auth: BearerAuth) -> HttpResponse {
    let text_content = body.into_inner().content;

    let process = number::integer_to_word(&text_content);

    match process {
        Ok(result) => middleware(
            HttpResponse::Ok().json(json!({
                "message": "tools/number/integer",
                "content": result
            })),
            auth,
        ),
        Err(err) => {
            let error = err.to_string();
            middleware(
                HttpResponse::BadRequest().json(json!({
                    "message": "tools/number/integer",
                    "query": text_content,
                    "content": error
                })),
                auth,
            )
        }
    }
}

#[post("/number/float")]
pub async fn float(body: web::Json<Request>, auth: BearerAuth) -> HttpResponse {
    let text_content = body.into_inner().content;

    let process = number::float_to_word(&text_content);

    match process {
        Ok(result) => middleware(
            HttpResponse::Ok().json(json!({
                "message": "tools/number/float",
                "content": result
            })),
            auth,
        ),
        Err(err) => {
            let error = err.to_string();
            middleware(
                HttpResponse::BadRequest().json(json!({
                    "message": "tools/number/float",
                    "query": text_content,
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
        let text = "12, 998336523409 12.5";
        let process = number::numbers_to_word(text);

        let response = match process {
            Ok(result) => {
                json!({
                    "message": "tools/number/content",
                    "query": text,
                    "content": result
                })
            }
            Err(err) => {
                json!({
                    "message": "tools/number/content",
                    "query": text,
                    "content": err
                })
            }
        };

        let static_json = "{\"content\":\"o‘n ikki, 998336523409 o‘n ikki butun o‘ndan besh\",\"message\":\"tools/number/content\",\"query\":\"12, 998336523409 12.5\"}";

        assert_eq!(serde_json::to_string(&response).unwrap(), static_json);
    }

    #[actix_web::test]
    async fn integer_test() {
        let text = "12";
        let process = number::integer_to_word(text);

        let response = match process {
            Ok(result) => {
                json!({
                    "message": "tools/number/integer",
                    "query": text,
                    "content": result
                })
            }
            Err(err) => {
                json!({
                    "message": "tools/number/integer",
                    "query": text,
                    "content": err
                })
            }
        };

        let static_json =
            "{\"content\":\"o‘n ikki\",\"message\":\"tools/number/integer\",\"query\":\"12\"}";

        assert_eq!(serde_json::to_string(&response).unwrap(), static_json);
    }

    #[actix_web::test]
    async fn float_test() {
        let text = "12.25";
        let process = number::float_to_word(text);

        let response = match process {
            Ok(result) => {
                json!({
                    "message": "tools/number/float",
                    "query": text,
                    "content": result
                })
            }
            Err(err) => {
                json!({
                    "message": "tools/number/float",
                    "query": text,
                    "content": err
                })
            }
        };

        let static_json = "{\"content\":\"o‘n ikki butun yuzdan yigirma besh\",\"message\":\"tools/number/float\",\"query\":\"12.25\"}";

        assert_eq!(serde_json::to_string(&response).unwrap(), static_json);
    }
}
