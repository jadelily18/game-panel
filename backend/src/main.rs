use actix_web::{get, App, HttpServer, Responder};

mod routes;

#[get("/")]
async fn root() -> impl Responder {
    "Root route :)"
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(root)
            .service(routes::rand_number)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

