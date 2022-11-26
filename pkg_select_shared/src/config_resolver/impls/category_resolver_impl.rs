use crate::common_models::models::configurations::package_category::PackageCategory;
use crate::common_models::models::configurations::runnable_package::RunnablePackage;
use crate::common_models::models::runtime_config::RuntimeConfig;
use crate::config_resolver::category_resolver::CategoryResolver;
use crate::config_resolver::traits::category_search::CategorySearch;
use std::ptr;

impl CategorySearch for CategoryResolver {
    fn find_by_name<'t>(
        &self,
        config: &'t RuntimeConfig,
        name: &'t String,
    ) -> Option<&'t PackageCategory> {
        config
            .package_categories
            .iter()
            .find(|value| value.name.to_lowercase().eq(name))
    }

    fn find_by_package<'t>(
        &self,
        config: &'t RuntimeConfig,
        package: &'t RunnablePackage,
    ) -> Option<&'t PackageCategory> {
        config.package_categories.iter().find(|value| {
            value
                .packages
                .iter()
                .find(|value2| ptr::eq(package, *value2) || value2.name.eq(&package.name))
                .is_some()
        })
    }
}
