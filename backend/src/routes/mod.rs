use actix_web::{get, HttpResponse, Error};
use rand::prelude::*;

pub mod container;
pub mod models;


#[get("/rand")]
pub async fn rand_number() -> Result<HttpResponse, Error> {
    let rand_number = rand::thread_rng().gen_range(1..101);

    debug!("Requested number: {}", rand_number.clone().to_string());

    Ok(HttpResponse::Ok().body(rand_number.to_string()))
}

