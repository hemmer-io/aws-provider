//! Sagemaker_runtime_2017_05_13 Service
//!
//! Auto-generated service module for sagemaker_runtime_2017_05_13

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for sagemaker_runtime_2017_05_13
pub struct Sagemaker_runtime_2017_05_13Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Sagemaker_runtime_2017_05_13Service<'a> {
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
