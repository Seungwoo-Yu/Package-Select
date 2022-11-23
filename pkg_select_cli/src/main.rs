pub mod traits;
pub mod models;
pub mod impls;
pub mod utils;

use std::env;
use std::process::ExitCode;
use std::rc::Rc;
use colored::Colorize;
use pkg_select_shared::{current_exec_file_path, fix_color_options_on_windows, MutationLocker, pause_project_for_debug, print_dbg_on_debug, println_on_debug};
use pkg_select_shared::argument_parser::argument_parser::parse_args;
use pkg_select_shared::config_resolver::config_resolver::ConfigResolver;
use pkg_select_shared::config_resolver::traits::config_persistence::ConfigPersistence;
use crate::models::command_resolver::CommandResolver;
use crate::models::commands::desync::Desync;
use crate::models::commands::sync::Sync;
use crate::models::commands::update_config::UpdateConfig;
use crate::models::commands::validate::Validate;
use crate::models::errors::command_resolve::CommandResolveError;
use crate::models::trait_combos::command_combo::CommandOrCollection;
use crate::traits::command_resolve::CommandResolve;
use crate::traits::command_search::CommandSearch;
use crate::utils::root::is_root;

fn main() -> ExitCode {
    fix_color_options_on_windows();

    if !is_root() {
        const ROOT_TEXT: &str = if cfg!(target_family = "windows") {
            "administrator privilege"
        } else {
            "root privilege"
        };

        println!("{} {}{}", "Seems like Package Select CLI is running without".bright_red(), ROOT_TEXT.red().bold(), ".".bright_red());
        println!("{} {}{}", "It must run with".bright_red(), ROOT_TEXT.red().bold(), ".".bright_red());
        println!("{}", "Please be aware it may not work as intended.".yellow());
    }

    let _args: Vec<String> = env::args().skip(1).collect();
    let args = parse_args(_args.join(" "));
    print_dbg_on_debug!(&_args);
    print_dbg_on_debug!(&args);
    print_dbg_on_debug!(env::current_exe().ok());
    print_dbg_on_debug!(current_exec_file_path().unwrap());

    let config_resolver = ConfigResolver::default();
    let mut config = match config_resolver.resolve() {
        Ok(value) => value,
        Err(error) => {
            println!("{}", error);
            pause_project_for_debug();

            return ExitCode::FAILURE;
        }
    };
    let mut config_mutation_locker = MutationLocker::create(&mut config, false);

    let mut command_resolver = CommandResolver::default();
    match resolve_commands(&mut command_resolver) {
        Ok(_) => {}
        Err(error) => {
            println!("{}", error);
            println!("couldn't resolve commands.");
            pause_project_for_debug();

            return ExitCode::FAILURE;
        }
    };

    let command_name = (&args).command.join("/");
    let command = match command_resolver.find_by_name(&command_name) {
        None => {
            println!("couldn't find \"{}\" command", &command_name.replace("/", " "));
            pause_project_for_debug();

            return ExitCode::FAILURE;
        }
        Some(value) => value,
    };

    match command.run(&config_resolver, &mut config_mutation_locker, &args) {
        Ok(_) => {
            println_on_debug!("Successfully finished running {}.", &command_name.replace("/", " "))
        }
        Err(_) => {
            println!("Failed running {}.", &command_name.replace("/", " "));
            pause_project_for_debug();

            return ExitCode::FAILURE;
        }
    }

    pause_project_for_debug();

    ExitCode::SUCCESS
}

fn resolve_commands(resolver: &mut CommandResolver) -> Result<(), CommandResolveError> {
    resolver.resolve(CommandOrCollection::Command(Rc::new(Sync {})))?;
    resolver.resolve(CommandOrCollection::Command(Rc::new(Desync {})))?;
    resolver.resolve(CommandOrCollection::Command(Rc::new(Validate {})))?;
    resolver.resolve(CommandOrCollection::Command(Rc::new(UpdateConfig {})))
}
