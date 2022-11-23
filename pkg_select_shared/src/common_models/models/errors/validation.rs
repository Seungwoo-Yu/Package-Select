#[derive(Debug)]
pub enum Type {
    InvalidBinderPath,
    InvalidIncludedPath,
    InvalidExcludedPath,
    InvalidDefaultPackage,
    EmptyCategoryList,
    EmptyPackageList,
    EmptyBinderList,
    DuplicatedBinderExecutionPath,
    DuplicatedIncludedPath,
    DuplicatedExcludedPath,
    DuplicatedPathInIncludedAndExcluded,
    NonUniqueName(String),
    NonUniqueTargetPath(String),
}

#[derive(Debug)]
pub struct ValidationError {
    pub error_type: Type,
}
