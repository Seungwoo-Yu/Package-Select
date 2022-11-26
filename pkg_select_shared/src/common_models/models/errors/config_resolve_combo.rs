use crate::common_models::models::errors::directory_resolve::DirectoryResolveError;
use std::io;

#[derive(Debug)]
pub enum SerdeIODirectoryError {
    SerdeError(serde_json::Error),
    IOError(io::Error),
    DirectoryResolveError(DirectoryResolveError),
}
