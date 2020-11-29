use std::io;

#[derive(Debug)]
pub enum TCPError {
    IHLError { cause: String },
}

impl From<TCPError> for io::Error {
    fn from(e: TCPError) -> Self {
        match e {
            TCPError::IHLError { cause } => {
                io::Error::new(io::ErrorKind::Other, cause)
            }
        }
    }
}

