use std::fmt;
pub use std::error::Error;

#[derive(Debug)]
pub enum GropeError {
    NoPattern,
    InvalidArgument(String),
    Io(std::io::Error),
}

impl fmt::Display for GropeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GropeError::NoPattern => write!(f, "No pattern provided"),
            GropeError::InvalidArgument(arg) => write!(f, "Invalid argument: {}", arg),
            GropeError::Io(err) => write!(f, "IO error: {}", err),
        }
    }
}

impl Error for GropeError {}

impl From<std::io::Error> for GropeError {
    fn from(err: std::io::Error) -> Self {
        GropeError::Io(err)
    }
}
