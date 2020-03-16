use actix_web::{web, App, HttpResponse, HttpServer, post};
use actix::spawn;
use reqwest::Client;
use serde::Deserialize;
use url::Url;

#[derive(Deserialize)]
struct Webmention {
    #[serde(with = "serde_with::rust::display_fromstr")]
    source: Url,
    #[serde(with = "serde_with::rust::display_fromstr")]
    target: Url,
}

fn supported_scheme(url: &Url) -> bool {
    url.scheme() == "http" || url.scheme() == "https"
}

fn supported_source(url: &Url) -> bool {
    match url.host_str() {
        None => false,
        Some(host) =>
            host != "localhost" && host != "127.0.0.1" && host != "[::1]"
    }
}

fn supported_target(url: &Url) -> bool {
    url.domain() == Some("pniedzielski.net") ||
        url.domain() == Some("www.pniedzielski.net")
}

fn request_verify(wm: &Webmention) -> bool {
    let mut source_fragmentless = wm.source.clone();
    source_fragmentless.set_fragment(None);
    let mut target_fragmentless = wm.target.clone();
    target_fragmentless.set_fragment(None);

    supported_scheme(&source_fragmentless) &&
        supported_scheme(&target_fragmentless) &&
        source_fragmentless != target_fragmentless &&
        supported_source(&source_fragmentless) &&
        supported_target(&target_fragmentless)
}

#[post("/")]
async fn index(form: web::Form<Webmention>) -> HttpResponse {
    let wm = form.into_inner();

    if request_verify(&wm) {
        let source = wm.source.clone();
        let fut = async move {
            let client = Client::new();
            let response = client.get(source)
                .header("User-Agent", "Actix-web")
                .header("Accept", "text/html, text/markdown;q=0.9, text/plain;q=0.8, application/json;q=0.7")
                .send().await;

            eprintln!("Response: {:?}", response);
        };
        spawn(fut);

        eprintln!("VALID: {} -> {}", wm.source, wm.target);
        HttpResponse::Ok().body(
            format!("source: {}\ntarget: {}", wm.source, wm.target)
        )
    } else {
        eprintln!("INVALID: {} -> {}", wm.source, wm.target);
        HttpResponse::BadRequest().body(
            "Could not verify Webmention"
        )
    }
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
