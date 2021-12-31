//! This modules contains everything related to error handling
//! 

use std::fmt;

#[derive(Debug)]
/**
 * This is the global type used for handling errors
 */
pub enum DbError {
    /**
     * Basic String error
     */
    Custom(String),
    /**
     * Reference to a std::io::Error
     */
    IoError(std::io::Error),
}

impl From<std::io::Error> for DbError {
    fn from(error: std::io::Error) -> Self {
        DbError::IoError(error)
    }
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DbError::Custom(s) => write!(f, "{}", s),
            DbError::IoError(e) => write!(f, "{:?}", e),
        }
    }
}
