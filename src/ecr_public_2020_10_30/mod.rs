//! Ecr_public_2020_10_30 Service
//!
//! Auto-generated service module for ecr_public_2020_10_30

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for ecr_public_2020_10_30
pub struct Ecr_public_2020_10_30Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Ecr_public_2020_10_30Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get registries resource handler
    pub fn registries(&self) -> resources::Registries<'_> {
        resources::Registries::new(self.provider)
    }
    /// Get authorization_token resource handler
    pub fn authorization_token(&self) -> resources::Authorization_token<'_> {
        resources::Authorization_token::new(self.provider)
    }
    /// Get repository resource handler
    pub fn repository(&self) -> resources::Repository<'_> {
        resources::Repository::new(self.provider)
    }
    /// Get repository_policy resource handler
    pub fn repository_policy(&self) -> resources::Repository_policy<'_> {
        resources::Repository_policy::new(self.provider)
    }
    /// Get repository_catalog_data resource handler
    pub fn repository_catalog_data(&self) -> resources::Repository_catalog_data<'_> {
        resources::Repository_catalog_data::new(self.provider)
    }
    /// Get image_tags resource handler
    pub fn image_tags(&self) -> resources::Image_tags<'_> {
        resources::Image_tags::new(self.provider)
    }
    /// Get repositories resource handler
    pub fn repositories(&self) -> resources::Repositories<'_> {
        resources::Repositories::new(self.provider)
    }
    /// Get images resource handler
    pub fn images(&self) -> resources::Images<'_> {
        resources::Images::new(self.provider)
    }
    /// Get registry_catalog_data resource handler
    pub fn registry_catalog_data(&self) -> resources::Registry_catalog_data<'_> {
        resources::Registry_catalog_data::new(self.provider)
    }
    /// Get image resource handler
    pub fn image(&self) -> resources::Image<'_> {
        resources::Image::new(self.provider)
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
