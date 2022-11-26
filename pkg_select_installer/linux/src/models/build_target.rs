use crate::models::arch::Arch;
use crate::models::errors::conversion_error::ConversionError;

#[derive(Debug)]
pub struct BuildTarget {
    pub arch: Arch,
    pub metadata: String,
}

impl BuildTarget {
    pub fn from(value: String) -> Result<Self, ConversionError> {
        let split: Vec<&str> = value.split("-").collect();

        if split.len() < 2 {
            return Err(ConversionError {});
        }

        let arch = Arch::from_rust_string(split[0].to_string())?;

        Ok(
            BuildTarget {
                arch,
                metadata: split[1..].join("-").to_string(),
            }
        )
    }
}
