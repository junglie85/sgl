use thiserror::Error;

#[derive(Error, Debug)]
pub enum SglError {
    #[error("{0}")]
    General(String),
}
