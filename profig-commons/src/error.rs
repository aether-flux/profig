use std::error::Error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum ProfigError {
    Io(io::Error),
    Parse { format: &'static str, error: String },
    InvalidFormat(String),
    Validation(String),
    Custom(String),
}

impl fmt::Display for ProfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProfigError::Io(e) => write!(f, "I/O Error: {}", e),
            ProfigError::Parse { format, error } => {
                write!(f, "Failed to parse {}: {}", format, error)
            }
            ProfigError::InvalidFormat(msg) => write!(f, "Invalid Format: {}", msg),
            ProfigError::Validation(msg) => write!(f, "Validation Error: {}", msg),
            ProfigError::Custom(msg) => write!(f, "{}", msg),
        }
    }
}

impl Error for ProfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ProfigError::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for ProfigError {
    fn from(err: io::Error) -> Self {
        ProfigError::Io(err)
    }
}
