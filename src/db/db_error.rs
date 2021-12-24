use std::fmt;

#[derive(Debug)]
pub enum DbError {
    Custom(String),
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
