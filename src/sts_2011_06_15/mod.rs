//! Sts_2011_06_15 Service
//!
//! Auto-generated service module for sts_2011_06_15

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for sts_2011_06_15
pub struct Sts_2011_06_15Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Sts_2011_06_15Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get access_key_info resource handler
    pub fn access_key_info(&self) -> resources::Access_key_info<'_> {
        resources::Access_key_info::new(self.provider)
    }
    /// Get caller_identity resource handler
    pub fn caller_identity(&self) -> resources::Caller_identity<'_> {
        resources::Caller_identity::new(self.provider)
    }
    /// Get session_token resource handler
    pub fn session_token(&self) -> resources::Session_token<'_> {
        resources::Session_token::new(self.provider)
    }
    /// Get federation_token resource handler
    pub fn federation_token(&self) -> resources::Federation_token<'_> {
        resources::Federation_token::new(self.provider)
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
