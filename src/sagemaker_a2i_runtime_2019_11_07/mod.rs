//! Sagemaker_a2i_runtime_2019_11_07 Service
//!
//! Auto-generated service module for sagemaker_a2i_runtime_2019_11_07

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for sagemaker_a2i_runtime_2019_11_07
pub struct Sagemaker_a2i_runtime_2019_11_07Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Sagemaker_a2i_runtime_2019_11_07Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get human_loop resource handler
    pub fn human_loop(&self) -> resources::Human_loop<'_> {
        resources::Human_loop::new(self.provider)
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
