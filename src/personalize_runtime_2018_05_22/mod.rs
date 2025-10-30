//! Personalize_runtime_2018_05_22 Service
//!
//! Auto-generated service module for personalize_runtime_2018_05_22

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for personalize_runtime_2018_05_22
pub struct Personalize_runtime_2018_05_22Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Personalize_runtime_2018_05_22Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get action_recommendations resource handler
    pub fn action_recommendations(&self) -> resources::Action_recommendations<'_> {
        resources::Action_recommendations::new(self.provider)
    }
    /// Get personalized_ranking resource handler
    pub fn personalized_ranking(&self) -> resources::Personalized_ranking<'_> {
        resources::Personalized_ranking::new(self.provider)
    }
    /// Get recommendations resource handler
    pub fn recommendations(&self) -> resources::Recommendations<'_> {
        resources::Recommendations::new(self.provider)
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
