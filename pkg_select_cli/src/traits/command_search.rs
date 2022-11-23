use std::rc::Rc;
use crate::traits::cli_command::CLICommand;

pub trait CommandSearch {
    fn find_by_name(&self, name: &str) -> Option<&Rc<dyn CLICommand>>;
}