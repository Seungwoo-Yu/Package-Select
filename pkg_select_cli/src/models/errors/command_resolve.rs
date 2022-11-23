#[derive(Debug)]
pub enum Type {
    DuplicatedName(String),
    EmptyNameList,
}

#[derive(Debug)]
pub struct CommandResolveError {
    pub error_type: Type,
}