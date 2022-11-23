use crate::common_models::models::configurations::package_category::PackageCategory;
use crate::common_models::models::configurations::runnable_package::RunnablePackage;
use crate::common_models::models::runtime_config::RuntimeConfig;

pub trait CategorySearch {
    fn find_by_name<'t>(
        &self,
        config: &'t RuntimeConfig,
        name: &'t String,
    ) -> Option<&'t PackageCategory>;
    fn find_by_package<'t>(
        &self,
        config: &'t RuntimeConfig,
        package: &'t RunnablePackage,
    ) -> Option<&'t PackageCategory>;
    // fn find_by_execution_path<'t>(&self, config: &'tRuntimeConfig, path: &'tPathBuf) -> Result<Option<&'t PackageCategory>, ()>;
    // TODO: implement this in future
    // fn find_all_by_path(&self, config: &RuntimeConfig, path: &PathBuf) -> Vec<PackageCategory>;
}
