//! Bedrock_agentcore_2024_02_28 Service
//!
//! Auto-generated service module for bedrock_agentcore_2024_02_28

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for bedrock_agentcore_2024_02_28
pub struct Bedrock_agentcore_2024_02_28Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Bedrock_agentcore_2024_02_28Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get resource_api_key resource handler
    pub fn resource_api_key(&self) -> resources::Resource_api_key<'_> {
        resources::Resource_api_key::new(self.provider)
    }
    /// Get workload_access_token resource handler
    pub fn workload_access_token(&self) -> resources::Workload_access_token<'_> {
        resources::Workload_access_token::new(self.provider)
    }
    /// Get workload_access_token_for_user_id resource handler
    pub fn workload_access_token_for_user_id(&self) -> resources::Workload_access_token_for_user_id<'_> {
        resources::Workload_access_token_for_user_id::new(self.provider)
    }
    /// Get workload_access_token_for_jwt resource handler
    pub fn workload_access_token_for_jwt(&self) -> resources::Workload_access_token_for_jwt<'_> {
        resources::Workload_access_token_for_jwt::new(self.provider)
    }
    /// Get resource_oauth2_token resource handler
    pub fn resource_oauth2_token(&self) -> resources::Resource_oauth2_token<'_> {
        resources::Resource_oauth2_token::new(self.provider)
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
