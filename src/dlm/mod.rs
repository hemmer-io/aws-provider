//! Dlm Service
//!
//! Auto-generated service module for dlm

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for dlm
pub struct DlmService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> DlmService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get lifecycle_policy resource handler
    pub fn lifecycle_policy(&self) -> resources::Lifecycle_policy<'_> {
        resources::Lifecycle_policy::new(self.provider)
    }
    /// Get lifecycle_policies resource handler
    pub fn lifecycle_policies(&self) -> resources::Lifecycle_policies<'_> {
        resources::Lifecycle_policies::new(self.provider)
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
