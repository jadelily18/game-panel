use thiserror::Error;


#[derive(Debug, Error)]
pub enum PortError {
    #[error("error finding available port")]
    FindPortError(String)
}
