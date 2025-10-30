//! Kendra_ranking_2022_10_19 Service
//!
//! Auto-generated service module for kendra_ranking_2022_10_19

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for kendra_ranking_2022_10_19
pub struct Kendra_ranking_2022_10_19Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Kendra_ranking_2022_10_19Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get rescore_execution_plan resource handler
    pub fn rescore_execution_plan(&self) -> resources::Rescore_execution_plan<'_> {
        resources::Rescore_execution_plan::new(self.provider)
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
