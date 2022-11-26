use std::path::PathBuf;
use pkg_select_shared::common_models::models::configurations::target_binder::TargetBinder;
use pkg_select_shared::common_models::traits::binder_converter::BinderConverter;
use linux_alternative_resolver_shared::common_models::models::errors::error_combo::IOParseAlternativeResolveError;
use crate::models::errors::path_registration_combo::DirectoryIOPathRegistrationError;
use crate::models::path_registration_resolver::PathRegistrationResolver;

pub mod windows_impl;
pub mod others_impl;
pub mod linux_impl;

#[cfg(target_os = "linux")]
pub fn path_registration_resolver() -> Result<PathRegistrationResolver, IOParseAlternativeResolveError> {
    use linux_alternative_resolver::alternative_resolver::AlternativeResolver;
    use linux_alternative_resolver::traits::alt_config_persistence::AltConfigPersistence;

    let alternative_resolver = AlternativeResolver {};
    let alt_config = alternative_resolver.resolve()?;

    let instance = PathRegistrationResolver {
        alt_config,
        alternative_resolver,
    };

    Ok(instance)
}

#[cfg(not(target_os = "linux"))]
pub fn path_registration_resolver() -> Result<PathRegistrationResolver, IOParseAlternativeResolveError> {
    Ok(PathRegistrationResolver {})
}

pub fn check_raw_path_registered(
    path_registration_resolver: &PathRegistrationResolver,
    raw_path: &PathBuf,
) -> Result<bool, DirectoryIOPathRegistrationError> {
    #[cfg(not(target_os = "linux"))]
    use crate::traits::path_registration::PathRegistration;
    #[cfg(target_os = "linux")]
    use crate::traits::linux_path_registration::LinuxPathRegistration;

    return Ok(
        path_registration_resolver.registered(&raw_path)?
    )
}

pub fn check_path_registered(
    path_registration_resolver: &PathRegistrationResolver,
    target_binder: &TargetBinder,
) -> Result<bool, DirectoryIOPathRegistrationError> {

    return Ok(
        check_raw_path_registered(
            path_registration_resolver,
            &target_binder.convert_exec_to_pathbuf()
        )?
    )
}

pub fn check_raw_path_unregistered(
    path_registration_resolver: &PathRegistrationResolver,
    raw_path: &PathBuf,
) -> Result<bool, DirectoryIOPathRegistrationError> {
    #[cfg(not(target_os = "linux"))]
    use crate::traits::path_registration::PathRegistration;
    #[cfg(target_os = "linux")]
    use crate::traits::linux_path_registration::LinuxPathRegistration;

    return Ok(
        !path_registration_resolver.registered(&raw_path)?
    )
}

pub fn check_path_unregistered(
    path_registration_resolver: &PathRegistrationResolver,
    target_binder: &TargetBinder,
) -> Result<bool, DirectoryIOPathRegistrationError> {
    return Ok(
        check_raw_path_unregistered(
            path_registration_resolver,
            &target_binder.convert_exec_to_pathbuf()
        )?
    )
}

pub fn filter_paths_registered(
    path_registration_resolver: &PathRegistrationResolver,
    target_binders: &Vec<TargetBinder>,
) -> Result<Vec<TargetBinder>, DirectoryIOPathRegistrationError> {
    let mut registered_binders: Vec<TargetBinder> = vec![];

    for value in target_binders.iter() {
        if check_path_registered(path_registration_resolver, value)? {
            registered_binders.push(value.clone());
        }
    }

    Ok(registered_binders)
}

pub fn filter_paths_unregistered(
    path_registration_resolver: &PathRegistrationResolver,
    target_binders: &Vec<TargetBinder>,
) -> Result<Vec<TargetBinder>, DirectoryIOPathRegistrationError> {
    let mut unregistered_binders: Vec<TargetBinder> = vec![];

    for value in target_binders.iter() {
        if check_path_unregistered(path_registration_resolver, value)? {
            unregistered_binders.push(value.clone());
        }
    }

    Ok(unregistered_binders)
}

pub fn register_paths(
    path_registration_resolver: &mut PathRegistrationResolver,
    target_binders: &Vec<TargetBinder>,
) -> Result<(), DirectoryIOPathRegistrationError> {
    let converted_paths: Vec<PathBuf> = target_binders.iter()
        .map(| value | value.convert_exec_to_pathbuf())
        .collect();
    let converted_path_refs: Vec<&PathBuf> = converted_paths.iter().collect();

    register_raw_paths(path_registration_resolver, &converted_path_refs)
}

pub fn register_raw_paths(
    path_registration_resolver: &mut PathRegistrationResolver,
    raw_paths: &Vec<&PathBuf>,
) -> Result<(), DirectoryIOPathRegistrationError> {
    #[cfg(not(target_os = "linux"))]
    use crate::traits::path_registration::MultiplePathRegistration;
    #[cfg(target_os = "linux")]
    use crate::traits::linux_path_registration::MultipleLinuxPathRegistration;

    path_registration_resolver.register(raw_paths)
}

pub fn unregister_paths(
    path_registration_resolver: &mut PathRegistrationResolver,
    target_binders: &Vec<TargetBinder>,
) -> Result<(), DirectoryIOPathRegistrationError> {
    let converted_paths: Vec<PathBuf> = target_binders.iter()
        .map(| value | value.convert_exec_to_pathbuf())
        .collect();
    let converted_path_refs: Vec<&PathBuf> = converted_paths.iter().collect();

    unregister_raw_paths(path_registration_resolver, &converted_path_refs)
}

pub fn unregister_raw_paths(
    path_registration_resolver: &mut PathRegistrationResolver,
    raw_paths: &Vec<&PathBuf>,
) -> Result<(), DirectoryIOPathRegistrationError> {
    #[cfg(not(target_os = "linux"))]
    use crate::traits::path_registration::MultiplePathRegistration;
    #[cfg(target_os = "linux")]
    use crate::traits::linux_path_registration::MultipleLinuxPathRegistration;

    path_registration_resolver.unregister(raw_paths)
}

pub fn reset_paths(
    path_registration_resolver: &mut PathRegistrationResolver,
) -> Result<(), DirectoryIOPathRegistrationError> {
    #[cfg(not(target_os = "linux"))]
    use crate::traits::path_registration::PathRegistrationReset;
    #[cfg(target_os = "linux")]
    use crate::traits::linux_path_registration::LinuxPathRegistrationReset;

    path_registration_resolver.reset()
}
