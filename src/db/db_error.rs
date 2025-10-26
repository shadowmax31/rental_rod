//! This modules contains everything related to error handling

use thiserror::Error;

#[derive(Error, Debug)]
/**
 * This is the global type used for handling errors
 */
pub enum DbError {
    #[error("{0}")]
    Custom(String),

    #[error(transparent)]
    IoError(#[from] std::io::Error),
}
