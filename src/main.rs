use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use routes::{hello, show_data, echo, manual_hello};
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::scope("/api")
            .service(hello)
            .service(echo)
            .service(show_data)
            .route("/hey", web::get().to(manual_hello)))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}