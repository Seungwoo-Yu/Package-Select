use crate::common_models::models::configurations::runnable_package::RunnablePackage;
use crate::common_models::models::runtime_config::RuntimeConfig;
use crate::config_resolver::package_resolver::PackageResolver;
use crate::config_resolver::traits::package_search::PackageSearch;
use std::cmp::Ordering;
use std::ffi::OsStr;
use std::path::PathBuf;
use crate::PathPop;

impl PackageSearch for PackageResolver {
    fn find_by_name<'t>(
        &self,
        config: &'t RuntimeConfig,
        name: &'t String,
    ) -> Option<&'t RunnablePackage> {
        let mut category_index: isize = -1;
        let mut package_index: usize = 0;

        for (index, value) in config.package_categories.iter().enumerate() {
            for (index2, value2) in value.packages.iter().enumerate() {
                if value2.name.eq(name) {
                    category_index = index as isize;
                    package_index = index2;
                    break;
                }
            }
        }

        if category_index.is_negative() {
            return None;
        }

        Some(&config.package_categories[category_index as usize].packages[package_index])
    }

    fn find_by_paths<'t>(
        &self,
        config: &'t RuntimeConfig,
        process_path: &'t PathBuf,
        working_path: &'t PathBuf,
    ) -> Option<&'t RunnablePackage> {
        let process_filename = match process_path.is_file() {
            true => process_path.file_name(),
            false => None,
        };
        let process_path_without_filename = process_path.pop_path();

        let mut found_packages =
            find_all_by_process_internal(config, process_path_without_filename, process_filename);

        if found_packages.is_empty() {
            return None;
        }

        found_packages.sort_unstable_by(|a, b| {
            let a_result = a
                .included_paths
                .iter()
                .find(|value| PathBuf::from(value).eq(working_path));
            let b_result = b
                .included_paths
                .iter()
                .find(|value| PathBuf::from(value).eq(working_path));

            if a_result.is_none() && b_result.is_none() {
                return Ordering::Equal;
            }
            return if b_result.is_some() {
                Ordering::Greater
            } else {
                Ordering::Less
            };
        });

        match found_packages.get(0) {
            None => None,
            Some(value) => Some(value),
        }
    }

    fn find_all_by_path<'t>(
        &self,
        config: &'t RuntimeConfig,
        path: &'t PathBuf,
    ) -> Vec<&'t RunnablePackage> {
        let filename = match path.is_file() {
            true => path.file_name(),
            false => None,
        };
        let path_without_filename = path.pop_path();

        find_all_by_process_internal(config, path_without_filename, filename)
    }
}

fn find_all_by_process_internal<'t>(
    config: &'t RuntimeConfig,
    path_without_filename: PathBuf,
    filename: Option<&'t OsStr>,
) -> Vec<&'t RunnablePackage> {
    let mut found_packages: Vec<&RunnablePackage> = vec![];

    for value in config.package_categories.iter() {
        for value2 in value.packages.iter() {
            let found_binder = value2.binders.iter().find(|value3| {
                let execution_path = PathBuf::from(&value3.execution_path);

                if !(&path_without_filename).eq(&execution_path) {
                    return false;
                }

                match &filename {
                    None => {
                        return false;
                    },
                    Some(value) => {
                        if !value.to_string_lossy().eq(&value3.target_name) {
                            return false;
                        }
                    },
                }

                value2.excluded_paths.iter()
                    .find(| value | path_without_filename.eq(&PathBuf::from(value)))
                    .is_some()
            });

            if found_binder.is_some() {
                let _ = &found_packages.push(&value2);
            }
        }
    }

    found_packages
}
