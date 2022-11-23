#[derive(Debug)]
pub enum Type {
    BinderNotRegistered(String),
    PathNotRegistered(String),
}

#[derive(Debug)]
pub struct PathBinderRegistrationError {
    pub error_type: Type,
}