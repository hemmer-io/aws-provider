//! Bedrock_agentcore_control_2023_06_05 Service
//!
//! Auto-generated service module for bedrock_agentcore_control_2023_06_05

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for bedrock_agentcore_control_2023_06_05
pub struct Bedrock_agentcore_control_2023_06_05Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Bedrock_agentcore_control_2023_06_05Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get token_vault resource handler
    pub fn token_vault(&self) -> resources::Token_vault<'_> {
        resources::Token_vault::new(self.provider)
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
