use crate::models::errors::conversion_error::ConversionError;

// Type command "dpkg-architecture -L" for Debian lists
// Refer "usr/lib/rpm/rpmrc" for Fedora/Redhat lists
// Refer https://doc.rust-lang.org/nightly/rustc/platform-support.html for Rust lists
#[derive(Debug)]
pub enum Arch {
    Aarch64,
    Arm,
    Armv7,
    I686,
    X8664,
}

impl Arch {
    pub fn from_rust_string(value: String) -> Result<Self, ConversionError> {
        let lowercase_value = value.to_lowercase();

        if (&lowercase_value).eq("aarch64") {
            return Ok(Arch::Aarch64)
        }

        if (&lowercase_value).eq("arm") {
            return Ok(Arch::Arm)
        }

        if (&lowercase_value).eq("armv7") {
            return Ok(Arch::Armv7)
        }

        if (&lowercase_value).eq("i686") {
            return Ok(Arch::I686)
        }

        if (&lowercase_value).eq("x86_64") {
            return Ok(Arch::X8664)
        }

        Err(ConversionError {})
    }

    pub fn from_fedora_string(value: String) -> Result<Self, ConversionError> {
        let lowercase_value = value.to_lowercase();

        if (&lowercase_value).eq("aarch64") {
            return Ok(Arch::Aarch64)
        }

        if (&lowercase_value).eq("arm6vl") {
            return Ok(Arch::Arm)
        }

        if (&lowercase_value).eq("arm7vl") {
            return Ok(Arch::Armv7)
        }

        if (&lowercase_value).eq("i386") {
            return Ok(Arch::I686)
        }

        if (&lowercase_value).eq("x86_64") {
            return Ok(Arch::X8664)
        }

            Err(ConversionError {})
    }

    pub fn from_debian_string(value: String) -> Result<Self, ConversionError> {
        let lowercase_value = value.to_lowercase();

        if (&lowercase_value).eq("arm64") {
            return Ok(Arch::Aarch64)
        }

        if (&lowercase_value).eq("arm") {
            return Ok(Arch::Arm)
        }

        if (&lowercase_value).eq("i386") {
            return Ok(Arch::I686)
        }

        if (&lowercase_value).eq("amd64") {
            return Ok(Arch::X8664)
        }

            Err(ConversionError {})
    }

    pub fn to_rust_string(&self) -> &str {
        match self {
            Arch::Aarch64 => "aarch64",
            Arch::Arm => "arm",
            Arch::Armv7 => "armv7",
            Arch::I686 => "i686",
            Arch::X8664 => "x86_64",
        }
    }

    pub fn to_fedora_string(&self) -> &str {
        match self {
            Arch::Aarch64 => "aarch64",
            Arch::Arm => "arm6vl",
            Arch::Armv7 => "arm7vl",
            Arch::I686 => "i386",
            Arch::X8664 => "x86_64",
        }
    }

    pub fn to_debian_string(&self) -> &str {
        match self {
            Arch::Aarch64 => "arm64",
            Arch::Arm => "arm",
            Arch::Armv7 => "arm",
            Arch::I686 => "i386",
            Arch::X8664 => "amd64",
        }
    }
}
