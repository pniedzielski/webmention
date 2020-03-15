use actix_web::{web, App, HttpResponse, HttpServer, post};
use serde::Deserialize;
use url::Url;

#[derive(Deserialize)]
struct Webmention {
    #[serde(with = "serde_with::rust::display_fromstr")]
    source: Url,
    #[serde(with = "serde_with::rust::display_fromstr")]
    target: Url,
}

#[post("/")]
async fn index(form: web::Form<Webmention>) -> HttpResponse {
    HttpResponse::Ok().body(
        format!("source: {}\ntarget: {}", form.source, form.target)
    )
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
