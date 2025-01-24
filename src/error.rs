#[derive(Debug)]
pub struct Error {
    message: String,
}

impl Error {
    pub fn from(cause: &str) -> Self {
        Error {
            message: cause.to_string(),
        }
    }
}

impl std::error::Error for Error {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        None
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error: {}", self.message)
    }
}
