use std::collections::HashMap;
use std::rc::Rc;
use crate::models::errors::command_resolve::CommandResolveError;
use crate::models::errors::command_resolve::Type::{DuplicatedName, EmptyNameList};
use crate::models::trait_combos::command_combo::CommandOrCollection;
use crate::traits::cli_command::CLICommand;

pub trait CommandCollection {
    fn collection_names(&self) -> Vec<&str>;
    fn commands(&self) -> Vec<CommandOrCollection>;
    fn resolve(&self, prefixes: &Vec<&str>, destination: &mut HashMap<String, Rc<dyn CLICommand>>) -> Result<(), CommandResolveError> {
        for value in self.commands().into_iter() {
            match value {
                CommandOrCollection::Command(value) => {
                    if (&value).command_names().is_empty() {
                        return Err(
                            CommandResolveError { error_type: EmptyNameList }
                        );
                    }

                    for value2 in (&value).command_names().iter() {
                        let key = if prefixes.is_empty() {
                            value2.to_string()
                        } else {
                            format!("{}/{}", prefixes.join("/"), value2.to_string())
                        };

                        if destination.get(&key).is_some() {
                            return Err(
                                CommandResolveError {
                                    error_type: DuplicatedName(key)
                                }
                            );
                        }

                        destination.insert(key, Rc::clone(&value));
                    }
                }
                CommandOrCollection::Collection(value) => {
                    if value.commands().is_empty() {
                        return Err(
                            CommandResolveError { error_type: EmptyNameList }
                        );
                    }

                    for value2 in value.collection_names().iter() {
                        let mut merged_prefixes = prefixes.to_vec();
                        merged_prefixes.push(value2);

                        value.resolve(&merged_prefixes, destination)?;
                    };
                }
            }
        }

        Ok(())
    }
}
