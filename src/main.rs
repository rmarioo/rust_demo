use std::env;
use actix_web::{middleware, web, App, HttpResponse, HttpServer};

async fn manual_hello() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body("<title>ciao</title>")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "9999".to_string());

    println!("start on host {} and port {}" , host, port);
    HttpServer::new(|| {
        App::new()
            .route("/hey", web::get().to(manual_hello))
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
        assert_eq!("rust is cool", "rust is cool!");
    }
}
