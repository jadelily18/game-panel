use actix_web::{get, HttpResponse, Error};
use rand::prelude::*;


#[get("/rand")]
pub async fn rand_number() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body(rand::thread_rng().gen_range(1..101).to_string()))
}

