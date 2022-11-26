use crate::models::errors::command_resolve::CommandResolveError;
use crate::models::trait_combos::command_combo::CommandOrCollection;

pub trait CommandResolve {
    fn resolve(&mut self, target: CommandOrCollection) -> Result<(), CommandResolveError>;
}
