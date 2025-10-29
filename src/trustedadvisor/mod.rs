//! Trustedadvisor Service
//!
//! Auto-generated service module for trustedadvisor

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for trustedadvisor
pub struct TrustedadvisorService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> TrustedadvisorService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get organization_recommendation_lifecycle resource handler
    pub fn organization_recommendation_lifecycle(&self) -> resources::Organization_recommendation_lifecycle<'_> {
        resources::Organization_recommendation_lifecycle::new(self.provider)
    }
    /// Get organization_recommendation resource handler
    pub fn organization_recommendation(&self) -> resources::Organization_recommendation<'_> {
        resources::Organization_recommendation::new(self.provider)
    }
    /// Get recommendation_lifecycle resource handler
    pub fn recommendation_lifecycle(&self) -> resources::Recommendation_lifecycle<'_> {
        resources::Recommendation_lifecycle::new(self.provider)
    }
    /// Get recommendation resource handler
    pub fn recommendation(&self) -> resources::Recommendation<'_> {
        resources::Recommendation::new(self.provider)
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
