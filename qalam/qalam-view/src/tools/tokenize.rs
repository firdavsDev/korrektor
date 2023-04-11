use crate::auth::middleware;
use crate::request::Request;
use actix_web::{get, post, web, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use korrektor::uzbek::tokenize;
use serde_json::json;

#[get("/tokenize")]
pub async fn main() -> HttpResponse {
    HttpResponse::BadRequest().json(json!({
        "endpoint": "/tokenize",
        "docs": "https://docs.korrektor.uz/tokenize"
    }))
}

#[post("/tokenize")]
pub async fn content(body: web::Json<Request>, auth: BearerAuth) -> HttpResponse {
    let content = body.into_inner().content;

    let process = tokenize::split_text(content.as_str());

    middleware(
        HttpResponse::Ok().json(json!({
            "message": "/tokenize",
            "content": process
        })),
        auth,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_web::test]
    async fn content_test() {
        let text_content = "singil chiroyli чиройли";
        let process = tokenize::split_text(text_content);

        let response = json!({
            "message": "/tokenize",
            "content": process
        });

        let static_json =
            "{\"content\":\"si-ngil chi-roy-li чи-рой-ли\",\"message\":\"/tokenize\"}";

        assert_eq!(serde_json::to_string(&response).unwrap(), static_json);
    }
}
