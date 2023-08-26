extern crate pretty_env_logger;
#[macro_use] extern crate log;
use actix_cors::Cors;
use actix_web::{get, App, HttpServer, Responder, web::{Data, scope}};
use routes::{
    container::{
        create_container
    }
};

mod database;
mod docker;
mod errors;
mod routes;
mod utils;


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

    info!("Including migrations...");
    
    sqlx::migrate!("./migrations");

    let maria_pool = match database::maria::create_pool().await {
        Ok(pool) => pool,
        Err(err) => panic!("Failed to connect to database: {}", err)
    };
    let maria_data = Data::new(maria_pool);

    match docker::get_images().await {
        Ok(_) => (),
        Err(err) => {
            panic!("Failed to pull required Docker images: {}", err.to_string())
        }
    }

    let srv = HttpServer::new(move || {
        App::new()
            .app_data(maria_data.clone())
            .wrap(
                Cors::default()
                    .allow_any_origin() // TODO: Figure out CORS. This should (probably?) not make it into production!
            )
            .service(root)
            .service(routes::rand_number)
            .service(
                scope("/container")
                    .service(create_container)
            )
    })
    .bind(("0.0.0.0", 8080))?;

    info!("Started Game Panel backend on port {:#?}!", srv.addrs()[0].port());

    srv.run().await
}

