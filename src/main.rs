mod for_compriension;

use std::env;
use actix_web::{web, App, HttpResponse, HttpServer};
use askama::Template;

async fn manual_hello() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body("<title>ciao</title>")
}

#[derive(Template)]
#[template(path = "page_template.html", escape = "none")]
struct PageTemplate {
    header: Header,
    central_section: CentralSection,
}

#[derive(Template)]
#[template(path = "header.html", escape = "none")]
struct Header {
    header: TitleAndContent,
    link: Link
}

struct Link {
    url: String,
    title: String
}

struct TitleAndContent {
    title: String,
    content: String
}

#[derive(Template)]
#[template(path = "central_section.html", escape = "none")]
struct CentralSection {
    column1: TitleAndContent,
    column2: TitleAndContent,
    column3: TitleAndContent,
}

async fn render_html_template() -> HttpResponse {
    let template = PageTemplate {
        header: Header {
            header: TitleAndContent { title: "Rust title".to_string(), content: "rust type system is cool".to_string() },
            link: Link { url: "www.google.com".to_string(), title: "A google link".to_string() }
        },
        central_section: CentralSection {
            column1: TitleAndContent { title: "rust column1 ".to_string(), content: "column1 content blabla".to_string() },
            column2: TitleAndContent { title: "rust column2 ".to_string(), content: "column2 content blabla".to_string() },
            column3: TitleAndContent { title: "rust column3 ".to_string(), content: "column3 content blabla".to_string() },
        }
    };



let  t = template.render().unwrap();
HttpResponse::Ok().content_type("text/html").body(t)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "9999".to_string());

    println!("start on host {} and port {}", host, port);
    HttpServer::new(|| {
        App::new()
            .route("/hey", web::get().to(manual_hello))
            .route("/template", web::get().to(render_html_template))
    })
        .bind(format!("{}:{}", host, port))?
        .run()
        .await
}

#[cfg(test)]
mod test {
    use pretty_assertions::{assert_eq};

    #[test]
    fn check_test_env_is_working() {
        assert_eq!("rust is fine", "rust is cool");
    }
}
