#[derive(Debug)]
pub enum Type {
    RunnerNotFound(String),
}

#[derive(Debug)]
pub struct BinderRegistrationError {
    pub error_type: Type,
}
