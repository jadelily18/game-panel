use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct CreateContainerRequest {
    pub accept_eula: bool,
    pub name: String,
    pub loader: String,
    pub game_version: String,
}

