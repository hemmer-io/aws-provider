//! Amplifybackend Service
//!
//! Auto-generated service module for amplifybackend

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for amplifybackend
pub struct AmplifybackendService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> AmplifybackendService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get backend_apimodels resource handler
    pub fn backend_apimodels(&self) -> resources::Backend_apimodels<'_> {
        resources::Backend_apimodels::new(self.provider)
    }
    /// Get backend_job resource handler
    pub fn backend_job(&self) -> resources::Backend_job<'_> {
        resources::Backend_job::new(self.provider)
    }
    /// Get backend_storage resource handler
    pub fn backend_storage(&self) -> resources::Backend_storage<'_> {
        resources::Backend_storage::new(self.provider)
    }
    /// Get token resource handler
    pub fn token(&self) -> resources::Token<'_> {
        resources::Token::new(self.provider)
    }
    /// Get backend_auth resource handler
    pub fn backend_auth(&self) -> resources::Backend_auth<'_> {
        resources::Backend_auth::new(self.provider)
    }
    /// Get backend resource handler
    pub fn backend(&self) -> resources::Backend<'_> {
        resources::Backend::new(self.provider)
    }
    /// Get backend_api resource handler
    pub fn backend_api(&self) -> resources::Backend_api<'_> {
        resources::Backend_api::new(self.provider)
    }
    /// Get backend_config resource handler
    pub fn backend_config(&self) -> resources::Backend_config<'_> {
        resources::Backend_config::new(self.provider)
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
