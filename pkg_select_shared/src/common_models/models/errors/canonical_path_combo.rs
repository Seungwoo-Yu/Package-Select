use crate::CanonicalPathError;
use std::io;

#[derive(Debug)]
pub enum IOCanonicalError {
    CanonicalError(CanonicalPathError),
    IOError(io::Error),
}
