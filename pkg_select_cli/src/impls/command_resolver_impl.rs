use std::rc::Rc;
use crate::models::command_resolver::CommandResolver;
use crate::models::errors::command_resolve::CommandResolveError;
use crate::models::errors::command_resolve::Type::{DuplicatedName, EmptyNameList};
use crate::models::trait_combos::command_combo::CommandOrCollection;
use crate::traits::cli_command::CLICommand;
use crate::traits::command_resolve::CommandResolve;
use crate::traits::command_search::CommandSearch;

impl CommandResolve for CommandResolver {
    fn resolve(&mut self, target: CommandOrCollection) -> Result<(), CommandResolveError> {
        match target {
            CommandOrCollection::Command(value) => {
                if (&value).command_names().is_empty() {
                    return Err(
                        CommandResolveError { error_type: EmptyNameList }
                    );
                }

                for value2 in (&value).command_names().iter() {
                    if self.data.get(*value2).is_some() {
                        return Err(
                            CommandResolveError {
                                error_type: DuplicatedName(value2.to_string())
                            }
                        );
                    }

                    self.data.insert(value2.to_string(), Rc::clone(&value));
                }

                Ok(())
            }
            CommandOrCollection::Collection(value) => {
                for value2 in value.collection_names().iter() {
                    let prefixes: Vec<&str> = vec![value2];

                    value.resolve(&prefixes, &mut self.data)?;
                }

                Ok(())
            }
        }
    }
}

impl CommandSearch for CommandResolver {
    fn find_by_name(&self, name: &str) -> Option<&Rc<dyn CLICommand>> {
        return match self.data.get(name) {
            None => None,
            Some(value) => Some(value),
        }
    }
}