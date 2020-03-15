use actix_web::{web, App, HttpResponse, HttpServer, post};
use serde::{de,Deserialize};
use std::fmt;
use url::Url;

#[derive(Deserialize)]
struct Webmention {
    #[serde(deserialize_with = "deserialize_url")]
    source: Url,
    #[serde(deserialize_with = "deserialize_url")]
    target: Url,
}

pub fn deserialize_url<'de, D>(d: D) -> Result<Url, D::Error>
where
    D: de::Deserializer<'de>,
{
    d.deserialize_str(UrlVisitor)
}

struct UrlVisitor;

impl<'de> de::Visitor<'de> for UrlVisitor {
    type Value = Url;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a URL")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match Url::parse(value) {
            Ok(url) => Ok(url),
            Err(e) => Err(E::custom(
                format!("Parse error {} for {}", e, value)
            )),
        }
    }
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
