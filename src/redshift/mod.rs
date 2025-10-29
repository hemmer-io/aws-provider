//! Redshift Service
//!
//! Auto-generated service module for redshift

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for redshift
pub struct RedshiftService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> RedshiftService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get resource_policy resource handler
    pub fn resource_policy(&self) -> resources::Resource_policy<'_> {
        resources::Resource_policy::new(self.provider)
    }
    /// Get custom_domain_association resource handler
    pub fn custom_domain_association(&self) -> resources::Custom_domain_association<'_> {
        resources::Custom_domain_association::new(self.provider)
    }
    /// Get credentials resource handler
    pub fn credentials(&self) -> resources::Credentials<'_> {
        resources::Credentials::new(self.provider)
    }
    /// Get track resource handler
    pub fn track(&self) -> resources::Track<'_> {
        resources::Track::new(self.provider)
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
