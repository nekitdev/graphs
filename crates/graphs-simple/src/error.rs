use thiserror::Error;

#[derive(Debug, Error)]
pub enum NodeError {
    #[error("node limit reached")]
    Limit,
}

#[derive(Debug, Error)]
pub enum EdgeError {
    #[error("edge limit reached")]
    Limit,
}
