//! Apigatewaymanagementapi_2018_11_29 Service
//!
//! Auto-generated service module for apigatewaymanagementapi_2018_11_29

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for apigatewaymanagementapi_2018_11_29
pub struct Apigatewaymanagementapi_2018_11_29Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Apigatewaymanagementapi_2018_11_29Service<'a> {
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
