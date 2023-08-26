extern crate pretty_env_logger;
#[macro_use] extern crate log;
use actix_web::{get, App, HttpServer, HttpResponse, Error};


#[get("/")]
async fn get_random_port() -> Result<HttpResponse, Error> {
    let srv = HttpServer::new(move || {
        App::new()
    })
    .bind(("127.0.0.1", 0))?;

    Ok(HttpResponse::Ok().body(srv.addrs()[0].port().to_string()))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();

    info!("Starting port checker...");

    HttpServer::new(move || {
        App::new()
            .service(get_random_port)
    })
    .bind(("0.0.0.0", 3434))?
    .run()
    .await
}
