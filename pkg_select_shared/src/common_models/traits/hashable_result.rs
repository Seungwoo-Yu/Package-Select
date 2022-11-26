use std::error::Error;

pub trait HashableResult<T: Error> {
    fn hash(&self) -> Result<String, T>;
}
