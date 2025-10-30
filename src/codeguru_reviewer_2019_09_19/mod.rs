//! Codeguru_reviewer_2019_09_19 Service
//!
//! Auto-generated service module for codeguru_reviewer_2019_09_19

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for codeguru_reviewer_2019_09_19
pub struct Codeguru_reviewer_2019_09_19Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Codeguru_reviewer_2019_09_19Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get recommendation_feedback resource handler
    pub fn recommendation_feedback(&self) -> resources::Recommendation_feedback<'_> {
        resources::Recommendation_feedback::new(self.provider)
    }
    /// Get code_review resource handler
    pub fn code_review(&self) -> resources::Code_review<'_> {
        resources::Code_review::new(self.provider)
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
