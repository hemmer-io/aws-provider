//! Route53profiles_2018_05_10 Service
//!
//! Auto-generated service module for route53profiles_2018_05_10

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for route53profiles_2018_05_10
pub struct Route53profiles_2018_05_10Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Route53profiles_2018_05_10Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get profile resource handler
    pub fn profile(&self) -> resources::Profile<'_> {
        resources::Profile::new(self.provider)
    }
    /// Get profile_resource_association resource handler
    pub fn profile_resource_association(&self) -> resources::Profile_resource_association<'_> {
        resources::Profile_resource_association::new(self.provider)
    }
    /// Get profile_association resource handler
    pub fn profile_association(&self) -> resources::Profile_association<'_> {
        resources::Profile_association::new(self.provider)
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
