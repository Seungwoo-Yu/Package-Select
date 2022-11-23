use std::collections::HashMap;
use std::rc::Rc;
use crate::traits::cli_command::CLICommand;

pub struct CommandResolver {
    pub(crate) data: HashMap<String, Rc<dyn CLICommand>>
}

impl Default for CommandResolver {
    fn default() -> Self {
        CommandResolver {
            data: Default::default()
        }
    }
}