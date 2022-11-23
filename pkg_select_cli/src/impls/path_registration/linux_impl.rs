#![cfg(target_os = "linux")]

use std::cmp::Ordering;
use std::path::PathBuf;
use indexmap::IndexSet;
use linux_alternative_resolver::traits::alt_config_persistence::AltConfigPersistence;
use linux_alternative_resolver_register::traits::path_register::PathRegister;
use linux_alternative_resolver_shared::common_models::models::alt_config::AltConfig;
use linux_alternative_resolver_shared::common_models::models::link_group::LinkGroup;
use linux_alternative_resolver_shared::common_models::models::link_item::LinkItem;
use linux_alternative_resolver_shared::common_models::models::link_path::LinkPath;
use pkg_select_shared::{InsertTo, Upsert};
use pkg_select_shared::common_models::models::runtime_config::RuntimeConfig;
use pkg_select_shared::common_models::traits::binder_converter::BinderConverter;
use crate::models::errors::path_registration::PathRegistrationError;
use crate::models::errors::path_registration::Type::{DestinationNotFile, DuplicatedTarget, LinuxLinkGroupNotFound, LinuxLinkItemNotFound};
use crate::models::errors::path_registration_combo::DirectoryIOPathRegistrationError;
use crate::models::path_registration_resolver::PathRegistrationResolver;
use crate::traits::linux_path_registration::{LinuxPathRegistration, LinuxPathRegistrationReset, MultipleLinuxPathRegistration};

const LINUX_BIN_PATH: &str = "/usr/bin";

impl LinuxPathRegistration for PathRegistrationResolver {
    fn registered(&self, target: &PathBuf) -> Result<bool, DirectoryIOPathRegistrationError> {
        for value in self.alt_config.alternatives.iter() {
            for value2 in value.items.iter() {
                match value2.paths.get_index(0) {
                    None => {}
                    Some(value) => {
                        if PathBuf::from(&value.alternative_path).eq(target) {
                            return Ok(true);
                        }
                    }
                }
            }
        }

        Ok(false)
    }

    fn register(&mut self, target: &PathBuf) -> Result<(), DirectoryIOPathRegistrationError> {
        register(self, target)
    }

    fn unregister(&mut self, target: &PathBuf) -> Result<(), DirectoryIOPathRegistrationError> {
        unregister(self, target)
    }
}

impl MultipleLinuxPathRegistration for PathRegistrationResolver {
    fn register(&mut self, data: &Vec<&PathBuf>) -> Result<(), DirectoryIOPathRegistrationError> {
        for target in data.iter() {
            register(self, target)?;
        }

        Ok(())
    }

    fn unregister(&mut self, data: &Vec<&PathBuf>) -> Result<(), DirectoryIOPathRegistrationError> {
        for target in data.iter() {
            unregister(self, target)?;
        }

        Ok(())
    }
}

impl LinuxPathRegistrationReset for PathRegistrationResolver {
    fn reset(&mut self, config: &RuntimeConfig) -> Result<(), DirectoryIOPathRegistrationError> {
        reset_registrations(self, config)
    }
}

fn reset_registrations(
    resolver: &mut PathRegistrationResolver,
    runtime_config: &RuntimeConfig,
) -> Result<(), DirectoryIOPathRegistrationError> {
    let mut paths: Vec<String> = vec![];
    let mut config = AltConfig { alternatives: IndexSet::new() };

    for value in runtime_config.package_categories.iter() {
        for value2 in value.packages.iter() {
            for value3 in value2.binders.iter() {
                let execution_path = value3.convert_exec_to_pathbuf()
                    .to_string_lossy()
                    .to_string();

                (&mut paths).push(execution_path);
            }
        }
    }

    config.alternatives = config.alternatives.into_iter()
        .map(| mut value | {
            value.items = value.items.into_iter()
                .map(| mut value2 | {
                    value2.paths.retain(| value3 | {
                        (&paths).iter()
                            .position(|value| value.eq(&value3.alternative_path))
                            .is_none()
                    });

                    value2
                })
                .filter(| value2 | value2.paths.len() > 0)
                .collect();

            value
        })
        .filter(| value | value.items.len() > 0)
        .collect();

    match (&resolver).alternative_resolver.update(&config) {
        Ok(_) => {}
        Err(_) => {
            panic!("");
        }
    };

    resolver.alt_config = config;

    Ok(())
}

fn register(
    resolver: &mut PathRegistrationResolver,
    target: &PathBuf,
) -> Result<(), DirectoryIOPathRegistrationError> {
    let target_path = target.to_string_lossy().to_string();
    let target_filename = match target.file_name()
        .map(| value | value.to_string_lossy().to_string()) {
        None => {
            return Err(
                DirectoryIOPathRegistrationError::PathRegistrationError(
                    PathRegistrationError {
                        error_type: DestinationNotFile(target.to_string_lossy().to_string())
                    }
                )
            )
        }
        Some(value) => value,
    };
    let bin_path = format!("{}/{}", LINUX_BIN_PATH, &target_filename);

    let mut _link_group_index: Option<usize> = None;
    let mut _link_item_index: Option<usize> = None;

    'group_iter: for (index, group) in (&resolver.alt_config).alternatives.iter().enumerate() {
        'item_iter: for (index2, item) in group.items.iter().enumerate() {
            let path = match item.paths.get_index(0) {
                None => {
                    continue 'item_iter;
                },
                Some(value) => value,
            };

            if path.name.eq(&target_filename) {
                _link_group_index = Some(index);

                if path.target_path.eq(&bin_path) &&
                    path.alternative_path.eq(&target_path) {
                    _link_item_index = Some(index2);
                    break 'group_iter;
                }
            }
        }
    }

    let link_group_created = (&_link_group_index).is_none();
    if link_group_created {
        _link_group_index = Some((&resolver.alt_config).alternatives.len());
    }
    let link_group_index = _link_group_index.unwrap();
    let mut link_group = match link_group_created {
        true => LinkGroup {
            name: (&target_filename).to_string(),
            filename: (&target_filename).to_string(),
            selected: None,
            items: IndexSet::default(),
        },
        false => (&resolver.alt_config).alternatives.get_index(link_group_index).unwrap().clone(),
    };

    let link_item_created = (&_link_item_index).is_none();
    if link_item_created {
        _link_item_index = Some((&link_group).items.len());
    }
    let link_item_index = _link_item_index.unwrap();
    let mut link_item = match link_item_created {
        true => LinkItem {
            family: None,
            priority: 0,
            paths: IndexSet::default(),
        },
        false => (&link_group).items.get_index(link_item_index).unwrap().clone(),
    };

    let link_path = match (&link_item).paths.get_index(0) {
        None => LinkPath {
            name: (&target_filename).to_string(),
            target_path: bin_path.to_string(),
            alternative_path: target_path.to_string()
        },
        Some(value) => value.clone(),
    };

    let greatest_priority = link_group.items.iter()
        .max_by(| a, b | {
            if a.priority > b.priority {
                return Ordering::Greater;
            } else if b.priority > a.priority {
                return Ordering::Less;
            }

            Ordering::Equal
        })
        .map(| value | value.priority)
        .unwrap_or(0);

    if (&link_group).items.len() > 0 && (&link_item).priority == greatest_priority {
        return Err(
            DirectoryIOPathRegistrationError::PathRegistrationError(
                PathRegistrationError {
                    error_type: DuplicatedTarget(target_filename.to_string())
                }
            )
        );
    }

    (&mut link_item).paths.insert_to(link_path.clone(), 0);
    (&mut link_item).priority = greatest_priority + 1;
    (&mut link_group).items.upsert_by(link_item, | a, b | {
        a.paths.get_index(0).unwrap().name.eq(&b.paths.get_index(0).unwrap().name)
    });

    let mut config = AltConfig { alternatives: IndexSet::default() };

    if link_group_created {
        (&mut config).alternatives = (&resolver).alt_config.alternatives.clone();
        (&mut config).alternatives.insert(link_group);
    } else {
        let mut cloned = (&resolver).alt_config.alternatives.clone();
        (&mut cloned).upsert_by(link_group, | a, b | {
            a.name.eq(&b.name)
        });
        (&mut config).alternatives = cloned;
    }

    match (&resolver).alternative_resolver.update(&config) {
        Ok(_) => {},
        Err(error) => {
            return Err(
                DirectoryIOPathRegistrationError::IOParseAlternativeResolveError(error)
            )
        }
    }

    dbg!(&link_path.name);

    match (&link_path).register() {
        Ok(_) => {},
        Err(error) => {
            return Err(
                DirectoryIOPathRegistrationError::IOParseAlternativeResolveError(error)
            );
        }
    }

    resolver.alt_config = config;

    Ok(())
}

fn unregister(
    resolver: &mut PathRegistrationResolver,
    target: &PathBuf,
) -> Result<(), DirectoryIOPathRegistrationError> {
    let target_path = target.to_string_lossy().to_string();
    let target_filename = match target.file_name()
        .map(| value | value.to_string_lossy().to_string()) {
        None => {
            return Err(
                DirectoryIOPathRegistrationError::PathRegistrationError(
                    PathRegistrationError {
                        error_type: DestinationNotFile(target.to_string_lossy().to_string())
                    }
                )
            )
        }
        Some(value) => value,
    };
    let bin_path = format!("{}/{}", LINUX_BIN_PATH, &target_filename);

    let mut _link_group_index: Option<usize> = None;
    let mut _link_item_index: Option<usize> = None;

    'group_iter: for (index, group) in (&resolver.alt_config).alternatives.iter().enumerate() {
        'item_iter: for (index2, item) in group.items.iter().enumerate() {
            let path = match item.paths.get_index(0) {
                None => {
                    continue 'item_iter;
                },
                Some(value) => value,
            };

            if path.name.eq(&target_filename) {
                _link_group_index = Some(index);

                if path.target_path.eq(&bin_path) &&
                    path.alternative_path.eq(&target_path) {
                    _link_item_index = Some(index2);
                    break 'group_iter;
                }
            }
        }
    }

    if (&_link_group_index).is_none() {
        return Err(
            DirectoryIOPathRegistrationError::PathRegistrationError(
                PathRegistrationError {
                    error_type: LinuxLinkGroupNotFound(target_filename)
                }
            )
        );
    }
    let link_group_index = _link_group_index.unwrap();
    let mut link_group = (&resolver.alt_config).alternatives.get_index(link_group_index)
        .unwrap()
        .clone();

    let link_item_created = (&_link_item_index).is_none();
    if (&_link_item_index).is_none() {
        _link_item_index = Some((&link_group).items.len());
    }
    let link_item_index = _link_item_index.unwrap();
    let link_item = match link_item_created {
        true => {
            return Err(
                DirectoryIOPathRegistrationError::PathRegistrationError(
                    PathRegistrationError {
                        error_type: LinuxLinkItemNotFound(target_filename)
                    }
                )
            );
        },
        false => (&link_group).items.get_index(link_item_index).unwrap(),
    };

    match (&link_item).paths.get_index(0) {
        None => {
            return Err(
                DirectoryIOPathRegistrationError::PathRegistrationError(
                    PathRegistrationError {
                        error_type: LinuxLinkItemNotFound(
                            format!("{} ({})", target_filename, (&link_item).priority)
                        )
                    }
                )
            );
        }
        Some(value) => {
            match value.unregister() {
                Ok(_) => {},
                Err(error) => {
                    return Err(
                        DirectoryIOPathRegistrationError::IOParseAlternativeResolveError(error)
                    );
                }
            }
        }
    }

    let mut cloned = AltConfig { alternatives: IndexSet::default() };

    (&mut link_group).items.shift_remove_index(link_item_index);
    (&mut cloned).alternatives = (&resolver).alt_config.alternatives.iter()
        .enumerate()
        .map(| (index, value) | {
            match index == link_group_index {
                true => &link_group,
                false => value,
            }.clone()
        })
        .filter(| value | value.items.len() > 0)
        .collect();

    match (&resolver).alternative_resolver.update(&cloned) {
        Ok(_) => {},
        Err(error) => {
            return Err(
                DirectoryIOPathRegistrationError::IOParseAlternativeResolveError(error)
            );
        }
    }

    resolver.alt_config = cloned;

    Ok(())
}