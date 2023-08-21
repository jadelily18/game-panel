use std::collections::HashMap;

use actix_web::{
    post, HttpResponse, Error,
    web::{Json}, error::{ErrorInternalServerError}
};
use bollard::{
    Docker,
    image::{
        CreateImageOptions
    },
    container::{
        Config,
        CreateContainerOptions,
        StartContainerOptions
    }, volume::CreateVolumeOptions
};
use futures_util::{TryStreamExt, stream::StreamExt};
use crate::routes::models::{
    container::{CreateContainerRequest}
};


#[post("")]
pub async fn create_container(
    container_req: Json<CreateContainerRequest>
) -> Result<HttpResponse, Error> {
    let docker = match Docker::connect_with_socket_defaults() {
        Ok(docker) => {
            // debug!("Host version: {:?}", docker.version().await.unwrap());
            docker
        },
        Err(err) => {
            error!("Couldn't connect to Docker socket: {:?}", err);
            return Err(ErrorInternalServerError(err))
        }
    };

    //*
    // Pull image
    //*

    info!("Pulling image from Docker Hub...");

    let create_image_opts = Some(
        CreateImageOptions {
            from_image: "itzg/minecraft-server:latest",
            ..Default::default()
        }
    );

    match docker.create_image(create_image_opts, None, None)
        .try_collect::<Vec<_>>().await {
            Ok(_) => {
                info!("Pulled image from Docker Hub!")
            },
            Err(err) => {
                error!("Couldn't pull image from Docker Hub.");
                return Err(ErrorInternalServerError(err))
            }
        };
    

    //*
    // Create container
    //*

    let new_uuid = uuid::Uuid::new_v4();

    info!("Creating Docker container...");

    let create_container_opts = Some(
        CreateContainerOptions {
            name: format!("minecraft-{}", new_uuid.to_string()),
            platform: None
        }
    );

    // Environment Variables
    let eula_string = format!("EULA={}", container_req.accept_eula.clone().to_string().to_ascii_uppercase());
    let type_string = format!("TYPE={}", container_req.loader);


    let config = Config {
        image: Some("itzg/minecraft-server"),
        cmd: None,
        env: Some(vec![
            eula_string.as_str(),
            type_string.as_str()
        ]),
        ..Default::default()
    };


    match docker.create_container(create_container_opts, config).await {
        Ok(_) => (),
        Err(err) => return Err(ErrorInternalServerError(err))
    }


    Ok(HttpResponse::Ok().finish())
}

