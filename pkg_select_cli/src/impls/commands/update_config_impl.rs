use std::rc::Rc;
use struct_indexer_core::ToNamedRcStruct;
use pkg_select_shared::common_models::models::runtime_config::RuntimeConfig;
use pkg_select_shared::config_resolver::config_resolver::ConfigResolver;
use pkg_select_shared::{MutationLocker, pause_command_line, read_input};
use pkg_select_shared::argument_parser::argument_parser::parse_args;
use pkg_select_shared::argument_parser::models::argument::Argument;
use crate::models::command_resolver::CommandResolver;
use crate::models::commands::update_config::UpdateConfig;
use crate::models::commands::update_subcommands::commit_changes::CommitChanges;
use crate::models::commands::update_subcommands::env_var::EnvVar;
use crate::models::commands::update_subcommands::package_category::PackageCategory;
use crate::models::commands::update_subcommands::runnable_package::RunnablePackage;
use crate::models::commands::update_subcommands::target_binder::TargetBinder;
use crate::models::errors::command::CommandError;
use crate::models::errors::command_resolve::CommandResolveError;
use crate::models::trait_combos::command_combo::CommandOrCollection;
use crate::traits::cli_command::CLICommand;
use crate::traits::command_resolve::CommandResolve;
use crate::traits::command_search::CommandSearch;

impl CLICommand for UpdateConfig {
    fn command_names(&self) -> Vec<&str> {
        vec!["update", "update-config"]
    }

    fn main(&self, resolver: &ConfigResolver, config: &mut MutationLocker<RuntimeConfig>, _: &Argument) -> Result<(), Vec<CommandError>> {
        let mut copied = config.value().clone();
        let mut config_mutation_locker = MutationLocker::create(&mut copied, true);

        let mut command_resolver = CommandResolver::default();
        match resolve_commands(&mut command_resolver) {
            Ok(_) => {}
            Err(error) => {
                return Err(vec![
                    CommandError::String(
                        format!("couldn't resolve commands.")
                    ),
                    CommandError::Others(Box::new(error)),
                ]);
            }
        };

        match run_command_by_input(resolver, &command_resolver, &mut config_mutation_locker) {
            Ok(_) => Ok(()),
            Err(_) => Err(vec![]),
        }
    }
}

fn run_command_by_input(
    config_resolver: &ConfigResolver,
    command_resolver: &CommandResolver,
    config: &mut MutationLocker<RuntimeConfig>,
) -> Result<(), ()> {
    print!("> ");
    let input = read_input().expect("couldn't get input from terminal.");

    let args = parse_args((&input).replace("\n", ""));
    dbg!(&args);
    let command_name = (&args).command.join("/");
    let command = match command_resolver.find_by_name(&command_name) {
        None => {
            println!("couldn't find \"{}\" command", &command_name.replace("/", " "));

            pause_command_line().expect("couldn't pause.");

            return run_command_by_input(config_resolver, command_resolver, config);
        }
        Some(value) => value,
    };

    return match command.run(&config_resolver, config, &args) {
        Ok(_) => {
            match Rc::clone(command).to_named_rc_struct::<CommitChanges>() {
                Ok(_) => {
                    return Ok(());
                }
                Err(_) => {}
            }

            run_command_by_input(config_resolver, command_resolver, config)
        }
        Err(_) => {
            println!("Failed running {}.", &command_name.replace("/", " "));

            run_command_by_input(config_resolver, command_resolver, config)
        }
    }
}

fn resolve_commands(resolver: &mut CommandResolver) -> Result<(), CommandResolveError> {
    resolver.resolve(CommandOrCollection::Command(Rc::new(CommitChanges {})))?;
    resolver.resolve(CommandOrCollection::Collection(Rc::new(PackageCategory {})))?;
    resolver.resolve(CommandOrCollection::Collection(Rc::new(RunnablePackage {})))?;
    resolver.resolve(CommandOrCollection::Collection(Rc::new(EnvVar {})))?;
    resolver.resolve(CommandOrCollection::Collection(Rc::new(TargetBinder {})))
}
