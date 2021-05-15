use std::{fmt, io, result::Result as StdResult};

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    #[cfg(feature = "backend_ocl")]
    Backend(ocl::Error),
    Other(String),
}

pub type Result<T> = StdResult<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "Io: {:?}\n{}", e.kind(), e),
            #[cfg(feature = "backend_ocl")]
            Error::Backend(e) => write!(f, "Backend:\n{}", e),
            Error::Other(s) => write!(f, "Other:\n{}", s),
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}
#[cfg(feature = "backend_ocl")]
impl From<ocl::Error> for Error {
    fn from(e: ocl::Error) -> Self {
        Error::Backend(e)
    }
}
impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Other(s)
    }
}
impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Error::Other(s.to_string())
    }
}
