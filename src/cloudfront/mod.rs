//! Cloudfront Service
//!
//! Auto-generated service module for cloudfront

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for cloudfront
pub struct CloudfrontService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> CloudfrontService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get key_value_store resource handler
    pub fn key_value_store(&self) -> resources::Key_value_store<'_> {
        resources::Key_value_store::new(self.provider)
    }
    /// Get keys resource handler
    pub fn keys(&self) -> resources::Keys<'_> {
        resources::Keys::new(self.provider)
    }
    /// Get key resource handler
    pub fn key(&self) -> resources::Key<'_> {
        resources::Key::new(self.provider)
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
