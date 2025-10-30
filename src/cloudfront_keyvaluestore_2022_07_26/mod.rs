//! Cloudfront_keyvaluestore_2022_07_26 Service
//!
//! Auto-generated service module for cloudfront_keyvaluestore_2022_07_26

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for cloudfront_keyvaluestore_2022_07_26
pub struct Cloudfront_keyvaluestore_2022_07_26Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Cloudfront_keyvaluestore_2022_07_26Service<'a> {
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
