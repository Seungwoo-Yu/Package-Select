// Refer https://stackoverflow.com/a/62528760
#[cfg(target_family = "unix")]
pub fn is_root() -> bool {
    use nix::unistd::Uid;
    Uid::effective().is_root()
}

// Refer https://stackoverflow.com/a/8196291
#[cfg(target_family = "windows")]
pub fn is_root() -> bool {
    use std::mem;
    use windows::Win32::Foundation::HANDLE;
    use windows::Win32::Security::{GetTokenInformation, TOKEN_ELEVATION, TOKEN_QUERY, TokenElevation};
    use windows::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};

    unsafe {
        let mut token = HANDLE::default();
        let mut token_elevation = TOKEN_ELEVATION::default();
        let token_elevation_ptr: *mut TOKEN_ELEVATION = &mut token_elevation;
        let mut size: u32 = 0;

        match OpenProcessToken(
            GetCurrentProcess(),
            TOKEN_QUERY,
            &mut token,
        ).as_bool() {
            true => {
                match GetTokenInformation(
                    token,
                    TokenElevation,
                    Some(token_elevation_ptr as _),
                    mem::size_of::<TOKEN_ELEVATION>() as u32,
                    &mut size,
                ).as_bool() {
                    true => {
                        return token_elevation.TokenIsElevated != 0;
                    }
                    false => {}
                };
            }
            false => {}
        };
    }

    false
}