//! TODO main docs
// #![warn(missing_docs)]

use thiserror::Error;

pub mod client;
pub mod server;

pub type Result<T> = std::result::Result<T, ChellError>;
#[derive(Error, Debug)]
pub enum ChellError {
    #[error("IO error")]
    IoError(#[from] std::io::Error),
}
