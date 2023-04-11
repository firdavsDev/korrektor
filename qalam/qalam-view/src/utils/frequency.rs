use crate::request::Request;
use actix_web::{get, post, web, HttpResponse};
use korrektor::utils::frequency;
use serde_json::json;

#[get("/frequency")]
pub async fn main() -> HttpResponse {
    HttpResponse::BadRequest().json(json!({
        "endpoint": "/frequency",
        "docs": "https://docs.korrektor.uz/frequency"
    }))
}

#[post("/frequency")]
pub async fn content(body: web::Json<Request>) -> HttpResponse {
    let content = body.into_inner().content;

    let process = frequency::count(content.as_str());

    HttpResponse::Ok().json(json!({
        "message": "/frequency",
        "content": process
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_web::test]
    async fn content_test() {
        let text_content = "hello sam hello";
        let process = frequency::count(text_content);

        let response = json!({
            "message": "/frequency",
            "content": process
        });

        let static_json =
            "{\"content\":{\"hello\":2,\"sam\":1},\"message\":\"/frequency\"}";

        assert_eq!(serde_json::to_string(&response).unwrap(), static_json);
    }
}
