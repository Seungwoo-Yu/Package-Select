use colored::Colorize;
use pkg_select_shared::common_models::traits::binder_converter::BinderConverter;
use pkg_select_shared::common_models::traits::binder_search::BinderSearch;
use pkg_select_shared::config_resolver::config_resolver::ConfigResolver;
use pkg_select_shared::config_resolver::traits::config_persistence::ConfigPersistence;
use pkg_select_shared::config_resolver::traits::package_search::PackageSearch;
use pkg_select_shared::{
    current_exec_file_path, current_working_path, fix_color_options_on_windows,
    pause_project_for_debug, print_dbg_on_debug,
};
use std::collections::HashMap;
use std::env;
use std::process::{Command, ExitCode};

fn main() -> ExitCode {
    fix_color_options_on_windows();

    let args: Vec<String> = env::args().skip(1).collect();
    print_dbg_on_debug!(args.join(" "));

    let config_resolver = ConfigResolver::default();
    let config = match config_resolver.resolve() {
        Ok(value) => value,
        Err(error) => {
            println!("{}", error);
            pause_project_for_debug();

            return ExitCode::FAILURE;
        }
    };
    let working_path = match current_working_path() {
        Ok(value) => value,
        Err(error) => {
            println!("{}", error);
            pause_project_for_debug();

            return ExitCode::FAILURE;
        }
    };
    let exec_path = match current_exec_file_path() {
        Ok(value) => value,
        Err(error) => {
            println!("{}", error);
            pause_project_for_debug();

            return ExitCode::FAILURE;
        }
    };

    print_dbg_on_debug!(working_path.to_string_lossy());
    print_dbg_on_debug!(exec_path.to_string_lossy());

    let package =
        match config_resolver.package_resolver.find_by_paths(&config, &exec_path, &working_path)
        {
            None => {
                println!("{}", "couldn't find runnable package.".bright_red());
                pause_project_for_debug();

                return ExitCode::FAILURE;
            }
            Some(value) => value,
        };
    let binder = match package.find_binder_by_path(&exec_path) {
        None => {
            println!(
                "{}",
                "couldn't find bound executable in runnable package.".bright_red()
            );
            pause_project_for_debug();

            return ExitCode::FAILURE;
        }
        Some(value) => value,
    };

    print_dbg_on_debug!(binder.convert_target_to_pathbuf());

    let mut _command = Command::new(binder.convert_target_to_pathbuf());
    let mut command = _command.args(args);

    if !package.envs.is_empty() {
        let mut vars: HashMap<String, String> = env::vars().collect();

        for (key, value) in package.envs.iter() {
            vars.insert(key.to_string(), value.to_string());
        }

        command = command.envs(&vars)
    }

    let mut _process = command.spawn();
    let process = match &mut _process {
        Ok(value) => value,
        Err(e) => {
            println!("{}", e);
            println!("couldn't execute command successfully. (spawn)");
            pause_project_for_debug();

            return ExitCode::FAILURE;
        }
    };

    match process.wait() {
        Ok(..) => {}
        Err(e) => {
            println!("{}", e);
            println!("couldn't execute command successfully. (status)");
            pause_project_for_debug();

            return ExitCode::FAILURE;
        }
    }

    pause_project_for_debug();

    ExitCode::SUCCESS
}
