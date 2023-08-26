use std::env;

use actix_web::web::Data;
use sqlx::MySqlPool;
use crate::{
    database::models,
    errors::PortError
};


pub async fn get_available_port(db: Data<MySqlPool>) -> Result<String, PortError> {
    let req_url = match env::var("IS_DOCKER_ENV") {
        Ok(_) => "http://host.docker.internal:3434",
        Err(_) => "http://localhost:3434"
    };
    
    let mut available_port: Option<String> = None;

    while available_port.is_none() {
        let port_found = match reqwest::get(req_url.clone()).await {
            Ok(res) => {
                match res.text().await {
                    Ok(text) => text,
                    Err(err) => panic!("Couldn't parse body from request to `{}`: {}", req_url, err)
                }
            },
            Err(err) => panic!("Failed to connect to port checker: {}", err)
        };

        let port_db_req = sqlx::query_as!(
            models::container::Container,
            r#"SELECT * FROM container WHERE port = ?"#,
            port_found
        ).fetch_one(&**db).await;

        match port_db_req {
            Ok(_) => {
                continue;
            },
            Err(_) => {
                available_port = Some(port_found);
                break;
            }
        }
    }

    match available_port {
        Some(port) => Ok(port),
        None => Err(PortError::FindPortError("Failed to get available port from OS".to_string()))
    }
}


// pub async fn get_available_port(db: Data<MySqlPool>) -> Result<String, PortError> {
//     let min_range = 32000;
//     let max_range = 48000;
    
//     let port_req_url = match env::var("IS_DOCKER_ENV") {
//         Ok(_) => "http://host.docker.internal:3434",
//         Err(_) => "http://localhost:3434"
//     };

//     let mut available_port: Option<String> = None;

//     for possible_port in min_range..max_range {
//         let req_url = format!("{}/available/{}", port_req_url, possible_port);

//         let port_in_use = match reqwest::get(req_url.clone()).await {
//             Ok(res) => {
//                 match res.text().await {
//                     Ok(body) => {
//                         match body.as_str() {
//                             "true" => false,
//                             "false" => true,
//                             _ => panic!("Couldn't parse response from `{}`: response: {}", req_url, body)
//                         }
//                     },
//                     Err(err) => panic!("Failed to parse request body for `{}`: {}\nIs the port checker service running?", req_url, err)
//                 }
//             },
//             Err(err) => panic!("Failed to connect to port checker: {}", err)
//         };

//         if port_in_use {
//             debug!("Port in use: {}", port_in_use);
//             continue;
//         }

//         let port_db_req = sqlx::query_as!(
//             models::container::Container,
//             r#"SELECT * FROM container WHERE port = ?"#,
//             possible_port
//         ).fetch_one(&**db).await;

//         match port_db_req {
//             Ok(_) => {
//                 continue;
//             },
//             Err(_) => {
//                 available_port = Some(possible_port.to_string());
//                 break;
//             }
//         }
//     }

//     match available_port {
//         Some(port) => Ok(port),
//         None => Err(PortError::FindPortError(format!("failed to find port in range `{}-{}`", min_range, max_range)))
//     }
// }




