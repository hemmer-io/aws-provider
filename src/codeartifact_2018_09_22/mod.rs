//! Codeartifact_2018_09_22 Service
//!
//! Auto-generated service module for codeartifact_2018_09_22

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for codeartifact_2018_09_22
pub struct Codeartifact_2018_09_22Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Codeartifact_2018_09_22Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get package_group_origin_configuration resource handler
    pub fn package_group_origin_configuration(&self) -> resources::Package_group_origin_configuration<'_> {
        resources::Package_group_origin_configuration::new(self.provider)
    }
    /// Get domain_permissions_policy resource handler
    pub fn domain_permissions_policy(&self) -> resources::Domain_permissions_policy<'_> {
        resources::Domain_permissions_policy::new(self.provider)
    }
    /// Get package_group resource handler
    pub fn package_group(&self) -> resources::Package_group<'_> {
        resources::Package_group::new(self.provider)
    }
    /// Get package resource handler
    pub fn package(&self) -> resources::Package<'_> {
        resources::Package::new(self.provider)
    }
    /// Get repository_permissions_policy resource handler
    pub fn repository_permissions_policy(&self) -> resources::Repository_permissions_policy<'_> {
        resources::Repository_permissions_policy::new(self.provider)
    }
    /// Get package_version_asset resource handler
    pub fn package_version_asset(&self) -> resources::Package_version_asset<'_> {
        resources::Package_version_asset::new(self.provider)
    }
    /// Get authorization_token resource handler
    pub fn authorization_token(&self) -> resources::Authorization_token<'_> {
        resources::Authorization_token::new(self.provider)
    }
    /// Get repository resource handler
    pub fn repository(&self) -> resources::Repository<'_> {
        resources::Repository::new(self.provider)
    }
    /// Get associated_package_group resource handler
    pub fn associated_package_group(&self) -> resources::Associated_package_group<'_> {
        resources::Associated_package_group::new(self.provider)
    }
    /// Get package_versions_status resource handler
    pub fn package_versions_status(&self) -> resources::Package_versions_status<'_> {
        resources::Package_versions_status::new(self.provider)
    }
    /// Get domain resource handler
    pub fn domain(&self) -> resources::Domain<'_> {
        resources::Domain::new(self.provider)
    }
    /// Get package_versions resource handler
    pub fn package_versions(&self) -> resources::Package_versions<'_> {
        resources::Package_versions::new(self.provider)
    }
    /// Get package_version_readme resource handler
    pub fn package_version_readme(&self) -> resources::Package_version_readme<'_> {
        resources::Package_version_readme::new(self.provider)
    }
    /// Get package_version resource handler
    pub fn package_version(&self) -> resources::Package_version<'_> {
        resources::Package_version::new(self.provider)
    }
    /// Get package_origin_configuration resource handler
    pub fn package_origin_configuration(&self) -> resources::Package_origin_configuration<'_> {
        resources::Package_origin_configuration::new(self.provider)
    }
    /// Get repository_endpoint resource handler
    pub fn repository_endpoint(&self) -> resources::Repository_endpoint<'_> {
        resources::Repository_endpoint::new(self.provider)
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
