pub mod models;
pub mod impls;

use std::{env, fs};
use std::ffi::OsStr;
use std::fs::{create_dir_all, remove_dir_all};
use std::io::Error;
use std::path::PathBuf;
use std::process::{Command, ExitCode};
#[cfg(unix)]
use file_owner::FileOwnerError;
use pkg_select_shared::argument_parser::argument_parser::parse_args;
use pkg_select_shared::current_exec_file_path;
use crate::models::build_target::BuildTarget;

fn main() -> ExitCode {
    const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
    const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    let process_path = match current_exec_file_path() {
        Ok(value) => value,
        Err(error) => {
            println!("{}", error);
            return ExitCode::FAILURE;
        }
    };

    let _build_path: Vec<&OsStr> = (&process_path).iter().collect();

    let project_path = match _build_path.iter()
        .position(|value| value.eq_ignore_ascii_case("target")) {
        None => {
            println!("Installer creator process must be in target folder of Package Select project.");
            return ExitCode::FAILURE;
        }
        Some(index) => {
            if index == 0 {
                println!("\"target\" folder must be in Package Select project.");
                return ExitCode::FAILURE;
            }

            PathBuf::from_iter(_build_path[..index].iter())
        }
    };

    let _args: Vec<String> = env::args().skip(1).collect();
    let args = parse_args(_args.join(" "));

    let user_dir_path = match user_dir_path() {
        None => {
            println!("Couldn't read sudo_user env. Are you running it as root?");
            return ExitCode::FAILURE;
        },
        Some(value) => value,
    };
    let cargo_path = (&args).optional_argument(format!("--cargo-path"))
        .map(| value | PathBuf::from(value))
        .unwrap_or((&user_dir_path).join(".cargo"));
    let rustup_path = (&args).optional_argument(format!("--rustup-path"))
        .map(| value | PathBuf::from(value))
        .unwrap_or((&user_dir_path).join(".rustup"));

    let skip_cargo_build = (&args).optional_flag("--skip-cargo-build".to_string());
    let skip_deb_build = (&args).optional_flag("--skip-deb".to_string());
    let skip_rpm_build = (&args).optional_flag("--skip-rpm".to_string());
    let debug_build = (&args).optional_flag("--debug".to_string());
    let disable_zstd_for_rpm = (&args).optional_flag("--disable-zstd-for-rpm".to_string());

    if skip_deb_build && skip_rpm_build {
        println!("Remove either \"--skip-deb\" or \"--skip-rpm\".");
        return ExitCode::FAILURE;
    }

    let _build_target = match (&args).command.get(0) {
        None => {
            println!("Build target is required.");
            return ExitCode::FAILURE;
        }
        Some(value) => value,
    };

    let build_target = match BuildTarget::from(_build_target.to_string()) {
        Ok(value) => value,
        Err(_) => {
            println!("Build target is invalid.");
            return ExitCode::FAILURE;
        }
    };

    if !skip_cargo_build {
        let mut _command = Command::new((&cargo_path).join("bin/cargo"));
        let mut build_args: Vec<String> = vec![
            "build",
            "--all",
            "--exclude",
            "pkg_select_installer_linux",
            "--target",
        ].iter().map(| value | value.to_string()).collect();

        (&mut build_args).push(build_target.to_string());

        if !debug_build {
            (&mut build_args).push("--release".to_string());
        }

        let mut command = _command.args(build_args)
            .envs([
                ("CARGO_HOME", &cargo_path),
                ("RUSTUP_HOME", &rustup_path)
            ])
            .current_dir(&project_path);

        if !(&args).non_optional.is_empty() {
            let envs: Vec<(&str, &str)> = (&args).non_optional.iter()
                .map(| value | {
                    let single_env_vec: Vec<&str> = value.split("=").collect();

                    if single_env_vec.is_empty() {
                        return None;
                    }

                    Some((single_env_vec[0], single_env_vec[1]))
                }).filter(| value | value.is_some())
                .map(| value | value.unwrap())
                .collect();

            command = command.envs(envs);
        }

        let mut process = match command.spawn() {
            Ok(value) => value,
            Err(error) => {
                println!("{}", error);
                println!("Stopped creating installer due to failed build. (spawn)");
                return ExitCode::FAILURE;
            }
        };

        match process.wait() {
            Ok(_) => {}
            Err(error) => {
                println!("{}", error);
                println!("Stopped creating installer due to failed build. (status)");
                return ExitCode::FAILURE;
            }
        }

        #[cfg(unix)]
        match change_ownership_with_children(&project_path.join("target"), &user_dir_path) {
            Ok(_) => {},
            Err(error) => {
                println!("{}", error);
                println!("Stopped creating installer due to issue during changing owner.");
                return ExitCode::FAILURE;
            },
        };
    }

    if !skip_deb_build {
        match build_deb(
            &project_path,
            AUTHOR,
            DESCRIPTION,
            VERSION,
            &build_target,
            debug_build
        ) {
            Ok(_) => {},
            Err(error) => {
                println!("{}", error);
                println!("Debian package build failed.");
                return ExitCode::FAILURE;
            }
        }
    }

    if !skip_rpm_build {
        match build_rpm(
            &project_path,
            DESCRIPTION,
            VERSION,
            &build_target,
            debug_build,
            disable_zstd_for_rpm,
        ) {
            Ok(_) => {},
            Err(error) => {
                println!("{}", error);
                println!("Fedora package build failed.");
                return ExitCode::FAILURE;
            }
        }
    }

    ExitCode::SUCCESS
}

fn build_deb(
    project_path: &PathBuf,
    author: &str,
    description: &str,
    version: &str,
    build_target: &BuildTarget,
    debug_build: bool
) -> Result<(), Error> {
    let build_code = format!("Package-Select-{}-{}", version, (&build_target.arch).to_debian_string());
    let deb_temp_path = project_path.join("pkg_select_installer/linux/.deb-build");
    let root_path = (&deb_temp_path).join(&build_code);
    let destination_path = (&root_path).join("usr/lib/package-select");
    let debian_folder_path = (&root_path).join("DEBIAN");

    // Remove old build files and create folder structures
    let _ = remove_dir_all(&root_path);
    create_dir_all(&destination_path)?;
    create_dir_all(&debian_folder_path)?;

    // Copy fresh packages
    fs::copy(
        project_path.join(format!(
            "target/{}/{}/pkg_select_cli",
            &build_target,
            if debug_build { "debug" } else { "release" }
        )),
        &destination_path.join("pkg_select_cli")
    )?;

    #[cfg(unix)]
    set_path_permission(&destination_path.join("pkg_select_cli"), 0o755)?;

    fs::copy(
        project_path.join(format!(
            "target/{}/{}/pkg_select_runner",
            &build_target,
            if debug_build { "debug" } else { "release" }
        )),
        &destination_path.join("pkg_select_runner")
    )?;

    #[cfg(unix)]
    set_path_permission(&destination_path.join("pkg_select_runner"), 0o755)?;

    // Add control containing build info file
    fs::write(
        &debian_folder_path.join("control"),
        format!(
            "{}{}{}{}{}",
            "Package: Package-Select\n",
            format!("Version: {}\n", version),
            format!("Architecture: {}\n", (&build_target.arch).to_debian_string()),
            format!("Maintainer: {}\n", author),
            format!("Description: {}\n", if description == "" { "N/A" } else { description }),
        )
    )?;

    #[cfg(unix)]
    set_path_permission(&root_path, 0o755)?;

    // Add postinst file
    let postinst_path = &debian_folder_path.join("postinst");
    fs::write(
        postinst_path,
        format!(
            "{}",
            "ln -s /usr/lib/package-select/pkg_select_cli /usr/bin/pkg_select_cli\n",
        )
    )?;

    #[cfg(unix)]
    set_path_permission(&postinst_path, 0o755)?;

    // Add prerm file
    let prerm_path = &debian_folder_path.join("prerm");
    fs::write(
        prerm_path,
        format!(
            "{}{}{}",
            "/usr/bin/pkg_select_cli purge --skip-confirm\n",
            "rm /usr/bin/pkg_select_cli\n",
            "rm -rf /usr/lib/package-select\n",
        )
    )?;

    #[cfg(unix)]
    set_path_permission(&prerm_path, 0o755)?;

    // Run dpkg-deb
    let mut _cmd = Command::new("dpkg-deb").args(&[
        "--build",
        "--root-owner-group",
        (&build_code).as_str(),
    ])
        .current_dir(&deb_temp_path)
        .spawn();

    let cmd = match &mut _cmd {
        Ok(value) => value,
        Err(error) => {
            return Err(Error::from(error.kind()));
        },
    };

    match cmd.wait() {
        Ok(..) => Ok(()),
        Err(error) => {
            return Err(error);
        },
    }
}

fn build_rpm(
    project_path: &PathBuf,
    description: &str,
    version: &str,
    build_target: &BuildTarget,
    debug_build: bool,
    disable_zstd_for_rpm: bool,
) -> Result<(), Error> {
    let rpm_temp_path = project_path.join("pkg_select_installer/linux/.rpm-build");
    let spec_folder_path = (&rpm_temp_path).join("SPECS");
    let build_folder_path = (&rpm_temp_path).join("BUILD");

    let spec_file_path = (&spec_folder_path).join("pkg-select.spec");

    let _ = remove_dir_all(&spec_folder_path);
    let _ = remove_dir_all(&build_folder_path);
    create_dir_all(&spec_folder_path)?;
    create_dir_all(&build_folder_path)?;

    // Copy fresh packages
    fs::copy(
        project_path.join(format!(
            "target/{}/{}/pkg_select_cli",
            &build_target,
            if debug_build { "debug" } else { "release" }
        )),
        &build_folder_path.join("pkg_select_cli")
    )?;
    fs::copy(
        project_path.join(format!(
            "target/{}/{}/pkg_select_runner",
            &build_target,
            if debug_build { "debug" } else { "release" }
        )),
        &build_folder_path.join("pkg_select_runner")
    )?;

    let mut raw_script: Vec<&str> = vec![];

    if !disable_zstd_for_rpm {
        (&mut raw_script).push("%define _binary_payload w3.zstdio");
    }

    // Define root directory since rpmbuild uses $home/.rpmbuild as main build directory by default
    let top_dir_script = format!(
        "%define _topdir {}",
        (&rpm_temp_path).to_string_lossy(),
    );
    (&mut raw_script).push(&top_dir_script);
    (&mut raw_script).push("");

    // Define build info
    (&mut raw_script).push("Name: Package-Select");
    let version_script = format!("Version: {}", version);
    (&mut raw_script).push(&version_script);
    (&mut raw_script).push("Release: 1");

    let summary_script = format!(
        "Summary: {}",
        if description == "" { "N/A" } else { description.lines().nth(0).unwrap() },
    );
    (&mut raw_script).push(&summary_script);
    (&mut raw_script).push(&arch_script);
    (&mut raw_script).push("License: ASL 2.0");
    (&mut raw_script).push("");

    (&mut raw_script).push("%description");
    let description_script = format!(
        "{}",
        if description == "" { "N/A" } else { description },
    );
    (&mut raw_script).push(
        &description_script,
    );
    (&mut raw_script).push("");

    // Write build script
    (&mut raw_script).push("%install");
    (&mut raw_script).push("rm -rf $RPM_BUILD_ROOT");
    (&mut raw_script).push("mkdir -p $RPM_BUILD_ROOT/%{_libdir}/package-select");
    (&mut raw_script).push("cp -r $RPM_BUILD_DIR/. $RPM_BUILD_ROOT/%{_libdir}/package-select");
    (&mut raw_script).push("");

    // Write post build script
    (&mut raw_script).push("%clean");
    (&mut raw_script).push("rm -rf $RPM_BUILD_ROOT");
    (&mut raw_script).push("");

    // Write created files
    (&mut raw_script).push("%files");
    (&mut raw_script).push("%{_libdir}/pkg-select/*");
    (&mut raw_script).push("");

    // Write post install script
    (&mut raw_script).push("%post");
    (&mut raw_script).push("chmod 755 -R %{_libdir}/package-select");
    (&mut raw_script).push("ln -s %{_libdir}/jdk-selector/pkg_select_cli %{_bindir}/pkg_select_cli");
    (&mut raw_script).push("");

    // Write pre uninstall script
    (&mut raw_script).push("%preun");
    (&mut raw_script).push("%{_bindir}/pkg_select_cli purge --skip-confirm");
    (&mut raw_script).push("rm %{_bindir}/pkg_select_cli");
    (&mut raw_script).push("rm -rf %{_libdir}/package-select");
    (&mut raw_script).push("");

    // Save raw script
    fs::write(
        &spec_file_path,
        (&mut raw_script).join("\n")
    )?;

    #[cfg(unix)]
    set_path_permission(&spec_file_path, 0o755)?;

    // Run rpmbuild
    let mut _cmd = Command::new("rpmbuild").args(&[
        "-bb",
        "--target",
        (&build_target.arch).to_fedora_string(),
        (&spec_file_path).to_string_lossy().to_string().as_str(),
    ])
        .current_dir(&rpm_temp_path)
        .spawn();

    let cmd = match &mut _cmd {
        Ok(value) => value,
        Err(error) => {
            return Err(Error::from(error.kind()));
        },
    };

    match cmd.wait() {
        Ok(_) => {},
        Err(error) => {
            return Err(error);
        },
    }

    Ok(())
}

fn user_dir_path() -> Option<PathBuf> {
    match env::vars().find(| (key, _) | {
        key.to_lowercase().eq("sudo_user")
    }) {
        None => None,
        Some((_, value)) => {
            return Some(PathBuf::from(&format!("/home/{}", value)));
        }
    }
}

#[cfg(unix)]
fn set_path_permission(path: &PathBuf, permission: u32) -> Result<(), Error> {
    use std::fs::{Permissions, set_permissions};
    use std::os::unix::fs::PermissionsExt;

    set_permissions(path, Permissions::from_mode(permission))
}

#[cfg(unix)]
fn change_ownership(path: &PathBuf, user_dir_path: &PathBuf) -> Result<(), FileOwnerError> {
    use file_owner::PathExt;

    let user = user_dir_path.owner()?;

    path.set_owner(user)?;

    Ok(())
}

#[cfg(unix)]
fn change_ownership_with_children(path: &PathBuf, user_dir_path: &PathBuf) -> Result<(), FileOwnerError> {
    use std::fs::{metadata, read_dir};

    let metadata = metadata(path)?;

    if (&metadata).is_dir() {
        for _entry in read_dir(path)? {
            let entry = _entry?;
            let entry_type = entry.file_type()?;

            if !entry_type.is_dir() {
                change_ownership(&(&entry).path(), user_dir_path)?;
                continue;
            }

            change_ownership_with_children(&(&entry).path(), user_dir_path)?;
        }
    }

    change_ownership(path, user_dir_path)?;

    Ok(())
}
