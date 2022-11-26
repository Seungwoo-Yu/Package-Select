use std::rc::Rc;
use crate::traits::cli_command::CLICommand;
use crate::traits::command_collection::CommandCollection;

pub enum CommandOrCollection {
    Command(Rc<dyn CLICommand>),
    Collection(Rc<dyn CommandCollection>),
}
