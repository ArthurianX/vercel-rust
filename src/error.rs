use std::{error::Error, fmt};

/// This module implements a custom error currently over the AWS Lambda runtime,
/// which can be extended later to support more service providers.
#[derive(Debug)]
pub struct VercelError {
    msg: String,
}
impl VercelError {
    pub fn new(message: &str) -> VercelError {
        VercelError {
            msg: message.to_owned(),
        }
    }
}
impl fmt::Display for VercelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for VercelError {}

impl From<std::num::ParseIntError> for VercelError {
    fn from(i: std::num::ParseIntError) -> Self {
        VercelError::new(&format!("{}", i))
    }
}

impl From<http::Error> for VercelError {
    fn from(i: http::Error) -> Self {
        VercelError::new(&format!("{}", i))
    }
}
