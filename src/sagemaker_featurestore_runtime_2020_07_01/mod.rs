//! Sagemaker_featurestore_runtime_2020_07_01 Service
//!
//! Auto-generated service module for sagemaker_featurestore_runtime_2020_07_01

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for sagemaker_featurestore_runtime_2020_07_01
pub struct Sagemaker_featurestore_runtime_2020_07_01Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Sagemaker_featurestore_runtime_2020_07_01Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get record resource handler
    pub fn record(&self) -> resources::Record<'_> {
        resources::Record::new(self.provider)
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
