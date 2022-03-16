use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};

async fn health(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().body("ok!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/health", web::get().to(health)))
        .bind(("0.0.0.0", {{ container_port }}))?
        .run()
        .await
}

#[cfg(test)]
mod tests {
    // unit test
    use super::*;
    use actix_web::{
        http::{self, header::ContentType},
        test,
    };

    #[actix_web::test]
    async fn test_health_ok() {
        let req = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .to_http_request();
        let resp = health(req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    // test integration
}
