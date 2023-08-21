extern crate pretty_env_logger;
#[macro_use] extern crate log;
use actix_cors::Cors;
use actix_web::{get, App, HttpServer, Responder, web};

use routes::{
    container::*
};

mod database;
mod routes;

#[get("/")]
async fn root() -> impl Responder {
    "Root route :)"
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    match dotenvy::dotenv() {
        Ok(_) => {
            warn!(".env file found. You should probably be using a Docker Compose file instead.");
        },
        Err(_) => ()
    }

    pretty_env_logger::init();

    info!("Starting Actix web server!");

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin() // TODO: Figure out CORS. This should (probably?) not make it into production!
            )
            .service(root)
            .service(routes::rand_number)
            .service(
                web::scope("/container")
                    .service(create_container)
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

    