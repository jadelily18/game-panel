use std::collections::HashMap;
use actix_web::{
    post, HttpResponse, Error,
    web::{Json, Data}, error::{ErrorInternalServerError}
};
use bollard::{
    Docker,
    container::{
        Config,
        CreateContainerOptions
    }, service::{HostConfig, PortBinding}
};
use sqlx::MySqlPool;
use crate::{
    routes::models::{
        container::CreateContainerRequest
    },
    utils::get_available_port
};


#[post("")]
pub async fn create_container(
    container_req: Json<CreateContainerRequest>,
    maria: Data<MySqlPool>
) -> Result<HttpResponse, Error> {
    let docker = match Docker::connect_with_socket_defaults() {
        Ok(docker) => {
            docker
        },
        Err(err) => {
            error!("Couldn't connect to Docker socket: {:?}", err);
            return Err(ErrorInternalServerError(err))
        }
    };

    //*
    // Create container
    //*

    let new_id = uuid::Uuid::new_v4();

    info!("Creating Docker container...");

    let create_container_opts = Some(
        CreateContainerOptions {
            name: format!("minecraft-{}", new_id.to_string()),
            platform: None
        }
    );

    let available_port = match get_available_port(maria.clone()).await {
        Ok(port) => port,
        Err(err) => panic!("Failed to find port: {}", err) // probably shouldn't panic here
    };

    let mut port_bind_map: HashMap<String, Option<Vec<PortBinding>>> = HashMap::new();

    let main_port = PortBinding {
        host_ip: Some("0.0.0.0".to_string()),
        host_port: Some(available_port.clone()),
    };

    port_bind_map.insert(
        "25565/tcp".to_string(),
        Some(vec![
            main_port
        ])
    );


    let host_config = HostConfig {
        port_bindings: Some(port_bind_map),
        ..Default::default()
    };


    // Environment Variables
    // TODO: These should all be validated before actually making the container
    let eula_string = format!("EULA={}", container_req.accept_eula.clone().to_string().to_ascii_uppercase());
    let type_string = format!("TYPE={}", container_req.loader);
    let mc_version_string = format!("VERSION={}", container_req.game_version);

    let config = Config {
        image: Some("itzg/minecraft-server"),
        cmd: None,
        env: Some(vec![
            eula_string.as_str(),
            type_string.as_str(),
            mc_version_string.as_str()
        ]),
        host_config: Some(host_config),
        ..Default::default()
    };


    match docker.create_container(create_container_opts, config).await {
        Ok(_) => (),
        Err(err) => return Err(ErrorInternalServerError(err))
    }

    let db_req = sqlx::query!(
        r#"
        INSERT INTO container (id, label, port)
        VALUES (?, ?, ?)
        "#, new_id.to_string(), container_req.name, available_port
    ).execute(&**maria).await;

    match db_req {
        Ok(_) => (),
        Err(err) => return Err(ErrorInternalServerError(err))
    }
    
    Ok(HttpResponse::Ok().finish())
}

