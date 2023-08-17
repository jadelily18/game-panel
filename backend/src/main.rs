extern crate pretty_env_logger;
#[macro_use] extern crate log;
use actix_cors::Cors;
use actix_web::{get, App, HttpServer, Responder};

mod routes;

#[get("/")]
async fn root() -> impl Responder {
    "Root route :)"
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    match dotenvy::dotenv() {
        Ok(_) => (),
        Err(_) => ()
    }

    pretty_env_logger::init();

    info!("Starting server on port 8080!");

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin() // TODO: Figure out CORS. This should not make it into production!
            )
            .service(root)
            .service(routes::rand_number)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

    