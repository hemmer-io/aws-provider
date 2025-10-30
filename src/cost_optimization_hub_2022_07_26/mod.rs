//! Cost_optimization_hub_2022_07_26 Service
//!
//! Auto-generated service module for cost_optimization_hub_2022_07_26

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for cost_optimization_hub_2022_07_26
pub struct Cost_optimization_hub_2022_07_26Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Cost_optimization_hub_2022_07_26Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get recommendation resource handler
    pub fn recommendation(&self) -> resources::Recommendation<'_> {
        resources::Recommendation::new(self.provider)
    }
    /// Get enrollment_status resource handler
    pub fn enrollment_status(&self) -> resources::Enrollment_status<'_> {
        resources::Enrollment_status::new(self.provider)
    }
    /// Get preferences resource handler
    pub fn preferences(&self) -> resources::Preferences<'_> {
        resources::Preferences::new(self.provider)
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
