#[derive(Debug)]
pub enum Type {
    DuplicatedTarget(String),
    DuplicatedTargets,
    DestinationNotFile(String),
    LinuxLinkGroupNotFound(String),
    LinuxLinkItemNotFound(String),
}

#[derive(Debug)]
pub struct PathRegistrationError {
    pub error_type: Type,
}