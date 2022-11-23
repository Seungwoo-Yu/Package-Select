#[derive(Debug)]
pub enum Type {
    ProjectDirNotFound,
    BaseDirNotFound,
    UserDirNotFound,
}

#[derive(Debug)]
pub struct DirectoryResolveError {
    pub error_type: Type,
}
