//! Apigatewaymanagementapi Service
//!
//! Auto-generated service module for apigatewaymanagementapi

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for apigatewaymanagementapi
pub struct ApigatewaymanagementapiService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> ApigatewaymanagementapiService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get connection resource handler
    pub fn connection(&self) -> resources::Connection<'_> {
        resources::Connection::new(self.provider)
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
