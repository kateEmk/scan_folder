use std::fmt;

#[derive(Debug)]
pub enum ServiceError {
    FailedToCreateDB,
    FailedToFoundCollection,
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServiceError::FailedToCreateDB => write!(f, "Failed to start database"),
            ServiceError::FailedToFoundCollection => write!(f, "Failed to find collection"),
        }
    }
}
