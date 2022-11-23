#[allow(unused_imports)]
use crate::common_models::models::configurations::package_category::PackageCategory;
use crate::common_models::models::configurations::runnable_package::RunnablePackage;
#[allow(unused_imports)]
use crate::common_models::models::configurations::target_binder::TargetBinder;
use crate::common_models::models::runtime_config::RuntimeConfig;
use std::path::PathBuf;

/**
   [PackageSearch] trait provides find options to get [RunnablePackage]
**/
pub trait PackageSearch {
    /**
       Find [RunnablePackage] by given name
    **/
    fn find_by_name<'t>(
        &self,
        config: &'t RuntimeConfig,
        name: &'t String,
    ) -> Option<&'t RunnablePackage>;
    // Seems like there is no needs to implement this at the moment
    // TODO: determine it to implement this or not
    // fn find_all_by_category<'t>(&self, config: &'t RuntimeConfig, category: &'t PackageCategory) -> Vec<&'t RunnablePackage>;
    /**
       Find best suitable [RunnablePackage] even if there is more than one matched.
       If so, the returned package is determined by rule below while there is no priority system by numbers explicitly.

       "process path" indicates where process is located.
       "working path" indicates where working directory is (similar to pwd).
       All condition except rule 5 assumes process path is matched with execution_path in one of [TargetBinder] at least.
       List numbers almost mean priority so the less number will impact greater than the bigger ones on selection.

       1. If working path is in included_paths in one [RunnablePackage] only, it will be selected.
       2. If there is more than one matched, the earliest one has been created will be selected.
       3. If there is none, the default package configured in [PackageCategory] selects [RunnablePackage].
       4. If there is none and the path is in excluded_paths in the default package, it will be *ignored*.
       5. If none of conditions above is satisfied or there is no [TargetBinder] matches its execution_path with process path, return nothing.
    **/
    fn find_by_paths<'t>(
        &self,
        config: &'t RuntimeConfig,
        process_path: &'t PathBuf,
        working_path: &'t PathBuf,
    ) -> Option<&'t RunnablePackage>;
    /**
       Find all [RunnablePackage] by given "process path"
    **/
    fn find_all_by_path<'t>(
        &self,
        config: &'t RuntimeConfig,
        process_path: &'t PathBuf,
    ) -> Vec<&'t RunnablePackage>;
}
