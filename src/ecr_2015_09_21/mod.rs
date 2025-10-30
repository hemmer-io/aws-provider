//! Ecr_2015_09_21 Service
//!
//! Auto-generated service module for ecr_2015_09_21

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for ecr_2015_09_21
pub struct Ecr_2015_09_21Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Ecr_2015_09_21Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get registry resource handler
    pub fn registry(&self) -> resources::Registry<'_> {
        resources::Registry::new(self.provider)
    }
    /// Get pull_through_cache_rules resource handler
    pub fn pull_through_cache_rules(&self) -> resources::Pull_through_cache_rules<'_> {
        resources::Pull_through_cache_rules::new(self.provider)
    }
    /// Get download_url_for_layer resource handler
    pub fn download_url_for_layer(&self) -> resources::Download_url_for_layer<'_> {
        resources::Download_url_for_layer::new(self.provider)
    }
    /// Get lifecycle_policy resource handler
    pub fn lifecycle_policy(&self) -> resources::Lifecycle_policy<'_> {
        resources::Lifecycle_policy::new(self.provider)
    }
    /// Get repository_creation_templates resource handler
    pub fn repository_creation_templates(&self) -> resources::Repository_creation_templates<'_> {
        resources::Repository_creation_templates::new(self.provider)
    }
    /// Get image_scanning_configuration resource handler
    pub fn image_scanning_configuration(&self) -> resources::Image_scanning_configuration<'_> {
        resources::Image_scanning_configuration::new(self.provider)
    }
    /// Get pull_through_cache_rule resource handler
    pub fn pull_through_cache_rule(&self) -> resources::Pull_through_cache_rule<'_> {
        resources::Pull_through_cache_rule::new(self.provider)
    }
    /// Get registry_scanning_configuration resource handler
    pub fn registry_scanning_configuration(&self) -> resources::Registry_scanning_configuration<'_> {
        resources::Registry_scanning_configuration::new(self.provider)
    }
    /// Get image_replication_status resource handler
    pub fn image_replication_status(&self) -> resources::Image_replication_status<'_> {
        resources::Image_replication_status::new(self.provider)
    }
    /// Get images resource handler
    pub fn images(&self) -> resources::Images<'_> {
        resources::Images::new(self.provider)
    }
    /// Get registry_policy resource handler
    pub fn registry_policy(&self) -> resources::Registry_policy<'_> {
        resources::Registry_policy::new(self.provider)
    }
    /// Get repository_policy resource handler
    pub fn repository_policy(&self) -> resources::Repository_policy<'_> {
        resources::Repository_policy::new(self.provider)
    }
    /// Get image_scan_findings resource handler
    pub fn image_scan_findings(&self) -> resources::Image_scan_findings<'_> {
        resources::Image_scan_findings::new(self.provider)
    }
    /// Get image resource handler
    pub fn image(&self) -> resources::Image<'_> {
        resources::Image::new(self.provider)
    }
    /// Get repositories resource handler
    pub fn repositories(&self) -> resources::Repositories<'_> {
        resources::Repositories::new(self.provider)
    }
    /// Get image_tag_mutability resource handler
    pub fn image_tag_mutability(&self) -> resources::Image_tag_mutability<'_> {
        resources::Image_tag_mutability::new(self.provider)
    }
    /// Get repository resource handler
    pub fn repository(&self) -> resources::Repository<'_> {
        resources::Repository::new(self.provider)
    }
    /// Get repository_creation_template resource handler
    pub fn repository_creation_template(&self) -> resources::Repository_creation_template<'_> {
        resources::Repository_creation_template::new(self.provider)
    }
    /// Get lifecycle_policy_preview resource handler
    pub fn lifecycle_policy_preview(&self) -> resources::Lifecycle_policy_preview<'_> {
        resources::Lifecycle_policy_preview::new(self.provider)
    }
    /// Get replication_configuration resource handler
    pub fn replication_configuration(&self) -> resources::Replication_configuration<'_> {
        resources::Replication_configuration::new(self.provider)
    }
    /// Get account_setting resource handler
    pub fn account_setting(&self) -> resources::Account_setting<'_> {
        resources::Account_setting::new(self.provider)
    }
    /// Get authorization_token resource handler
    pub fn authorization_token(&self) -> resources::Authorization_token<'_> {
        resources::Authorization_token::new(self.provider)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_creation() {
        // Service creation test
    }
}
