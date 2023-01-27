use thiserror::Error;

#[derive(Error, Debug)]
pub enum MehError {
    #[error("{0}")]
    General(String),
}
