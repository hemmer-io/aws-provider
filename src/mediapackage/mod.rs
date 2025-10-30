//! Mediapackage Service
//!
//! Auto-generated service module for mediapackage

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for mediapackage
pub struct MediapackageService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> MediapackageService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get asset resource handler
    pub fn asset(&self) -> resources::Asset<'_> {
        resources::Asset::new(self.provider)
    }
    /// Get packaging_configuration resource handler
    pub fn packaging_configuration(&self) -> resources::Packaging_configuration<'_> {
        resources::Packaging_configuration::new(self.provider)
    }
    /// Get packaging_group resource handler
    pub fn packaging_group(&self) -> resources::Packaging_group<'_> {
        resources::Packaging_group::new(self.provider)
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
