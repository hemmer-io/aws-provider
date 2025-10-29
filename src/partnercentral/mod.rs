//! Partnercentral Service
//!
//! Auto-generated service module for partnercentral

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for partnercentral
pub struct PartnercentralService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> PartnercentralService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get selling_system_settings resource handler
    pub fn selling_system_settings(&self) -> resources::Selling_system_settings<'_> {
        resources::Selling_system_settings::new(self.provider)
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
