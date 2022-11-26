#[cfg(target_os = "linux")]
use linux_alternative_resolver::alternative_resolver::AlternativeResolver;
#[cfg(target_os = "linux")]
use linux_alternative_resolver_shared::common_models::models::alt_config::AltConfig;

#[cfg(target_os = "linux")]
pub struct PathRegistrationResolver {
    pub alt_config: AltConfig,
    pub alternative_resolver: AlternativeResolver
}

#[cfg(not(target_os = "linux"))]
pub struct PathRegistrationResolver {}
