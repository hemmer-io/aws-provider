//! Sagemaker_metrics_2022_09_30 Service
//!
//! Auto-generated service module for sagemaker_metrics_2022_09_30

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for sagemaker_metrics_2022_09_30
pub struct Sagemaker_metrics_2022_09_30Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Sagemaker_metrics_2022_09_30Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
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
