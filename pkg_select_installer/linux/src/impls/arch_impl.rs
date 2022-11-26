use std::fmt::{Display, Formatter};
use crate::models::arch::Arch;

impl Display for Arch {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_rust_string())
    }
}
