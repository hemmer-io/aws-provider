//! Personalize Service
//!
//! Auto-generated service module for personalize

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for personalize
pub struct PersonalizeService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> PersonalizeService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get personalized_ranking resource handler
    pub fn personalized_ranking(&self) -> resources::Personalized_ranking<'_> {
        resources::Personalized_ranking::new(self.provider)
    }
    /// Get recommendations resource handler
    pub fn recommendations(&self) -> resources::Recommendations<'_> {
        resources::Recommendations::new(self.provider)
    }
    /// Get action_recommendations resource handler
    pub fn action_recommendations(&self) -> resources::Action_recommendations<'_> {
        resources::Action_recommendations::new(self.provider)
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
