//! Bedrock_data_automation_2023_07_26 Service
//!
//! Auto-generated service module for bedrock_data_automation_2023_07_26

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for bedrock_data_automation_2023_07_26
pub struct Bedrock_data_automation_2023_07_26Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Bedrock_data_automation_2023_07_26Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get blueprint_version resource handler
    pub fn blueprint_version(&self) -> resources::Blueprint_version<'_> {
        resources::Blueprint_version::new(self.provider)
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
