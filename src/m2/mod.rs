//! M2 Service
//!
//! Auto-generated service module for m2

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for m2
pub struct M2Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> M2Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get signed_bluinsights_url resource handler
    pub fn signed_bluinsights_url(&self) -> resources::Signed_bluinsights_url<'_> {
        resources::Signed_bluinsights_url::new(self.provider)
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
