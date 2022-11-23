use std::fmt::{Display, Formatter};
use crate::models::build_target::BuildTarget;

impl Display for BuildTarget {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.arch.to_rust_string(), self.metadata)
    }
}