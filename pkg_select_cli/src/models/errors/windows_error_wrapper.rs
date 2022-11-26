#[derive(Debug)]
#[repr(i32)]
pub enum ErrorCodes {
    ErrCodeRegNotFound = -2147024894,
    Unknown,
}

#[cfg(target_os = "windows")]
impl From<i32> for ErrorCodes {
    fn from(existed: i32) -> Self {
        match existed {
            value if value == ErrorCodes::ErrCodeRegNotFound as i32 => ErrorCodes::ErrCodeRegNotFound,
            _ => ErrorCodes::Unknown
        }
    }
}

#[derive(Debug)]
pub struct WindowsError {
    pub(crate) code: ErrorCodes,
    pub(crate) message: String,
}

#[cfg(target_os = "windows")]
impl From<windows::core::Error> for WindowsError {
    fn from(existed: windows::core::Error) -> Self {
        WindowsError {
            code: ErrorCodes::from(existed.code().0),
            message: existed.message().to_string(),
        }
    }
}
