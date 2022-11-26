use struct_indexer_core::{Indexed, ToAnyTrait};
use pkg_select_shared::argument_parser::models::argument::Argument;
use pkg_select_shared::config_resolver::config_resolver::ConfigResolver;
use pkg_select_shared::common_models::models::runtime_config::RuntimeConfig;
use pkg_select_shared::MutationLocker;
use crate::models::errors::command::CommandError;

pub trait CLICommand : Indexed + ToAnyTrait {
    fn command_names(&self) -> Vec<&str>;
    fn main(
        &self,
        resolver: &ConfigResolver,
        config: &mut MutationLocker<RuntimeConfig>,
        args: &Argument
    ) -> Result<(), Vec<CommandError>>;
    fn run(
        &self,
        resolver: &ConfigResolver,
        config: &mut MutationLocker<RuntimeConfig>,
        args: &Argument
    ) -> Result<(), ()> {
        match self.main(resolver, config, args) {
            Ok(_) => Ok(()),
            Err(error) => {
                for value in error.iter() {
                    match value {
                        CommandError::None => {},
                        CommandError::String(value) => {
                            println!("{}", value);
                        },
                        CommandError::Others(value) => {
                            println!("{}", value);
                        }
                    }
                }

                Err(())
            }
        }
    }
}
