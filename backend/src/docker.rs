use bollard::{
    Docker,
    image::CreateImageOptions,
    errors::Error 
};
use futures_util::TryStreamExt;


pub async fn get_images() -> Result<(), Error> {
    info!("Connecting to Docker socket...");
    
    let docker = match Docker::connect_with_socket_defaults() {
        Ok(docker) => {
            docker
        },
        Err(err) => {
            error!("Couldn't connect to Docker socket: {:?}", err);
            return Err(err)
        }
    };

    info!("Successfully connected to Docker socket!");

    info!("Pulling required Docker image(s)...");

    let images = vec![
        "itzg/minecraft-server:latest"
    ];

    for image in images {

        let image_opts = Some(
            CreateImageOptions {
                from_image: image,
                ..Default::default()
            }
        );

        match docker.create_image(image_opts, None, None)
            .try_collect::<Vec<_>>().await {
                Ok(_) => {
                    info!("Pulled Docker image `{image}`!")
                },
                Err(err) => {
                    error!("Failed to pull image `{image}`.");
                    return Err(err)
                }
            };
        
        
        info!("Successfully pulled Docker images!");

    }

    Ok(())

}
