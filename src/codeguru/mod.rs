//! Codeguru Service
//!
//! Auto-generated service module for codeguru

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for codeguru
pub struct CodeguruService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> CodeguruService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get code_review resource handler
    pub fn code_review(&self) -> resources::Code_review<'_> {
        resources::Code_review::new(self.provider)
    }
    /// Get recommendation_feedback resource handler
    pub fn recommendation_feedback(&self) -> resources::Recommendation_feedback<'_> {
        resources::Recommendation_feedback::new(self.provider)
    }
    /// Get repository_association resource handler
    pub fn repository_association(&self) -> resources::Repository_association<'_> {
        resources::Repository_association::new(self.provider)
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
