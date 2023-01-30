use crate::auth::middleware;
use actix_web::{get, web, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use korrektor_rs_private;
use serde_json::json;

#[get("/transliterate")]
pub async fn main() -> HttpResponse {
    HttpResponse::Ok().body("Transliteration module")
}

#[get("/transliterate/{lang}/{content}")]
pub async fn content(path: web::Path<(String, String)>, auth: BearerAuth) -> HttpResponse {
    let (language, content) = path.into_inner();
    let process = korrektor_rs_private::transliterator::to(content.clone(), &language);

    middleware(
        HttpResponse::Ok().json(json!({
            "message": "private/transliterate",
            "query": content,
            "content": process
        })),
        auth,
    )
}

#[cfg(test)]
mod tests {
    use korrektor_rs_private::{corrector, transliterator};
    use super::*;

    #[actix_web::test]
    async fn content_lat_test() {
        let text_content = "ғозал ҒОЗАЛ Ғозал гелий";
        let process = transliterator::to(text_content.to_string(), "lat");

        let response = json!({
        "message": "private/correct/transliterate",
        "query": text_content,
        "content": process
    });


        let static_json =
            "{\"content\":\"g‘ozal GʼOZAL Gʼozal geliy\",\"message\":\"private/correct/transliterate\",\"query\":\"ғозал ҒОЗАЛ Ғозал гелий\"}";

        assert_eq!(serde_json::to_string(&response).unwrap(), static_json);
    }

    #[actix_web::test]
    async fn content_cyr_test() {
        let text_content = "g'ozal G'OZAL G'ozal geliy";
        let process = transliterator::to(text_content.to_string(), "cyr");

        let response = json!({
        "message": "private/correct/transliterate",
        "query": text_content,
        "content": process
    });


        let static_json =
            "{\"content\":\"ғозал ҒОЗАЛ Ғозал гелий\",\"message\":\"private/correct/transliterate\",\"query\":\"g'ozal G'OZAL G'ozal geliy\"}";

        assert_eq!(serde_json::to_string(&response).unwrap(), static_json);
    }
}
