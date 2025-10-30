//! Partnercentral_selling_2022_07_26 Service
//!
//! Auto-generated service module for partnercentral_selling_2022_07_26

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for partnercentral_selling_2022_07_26
pub struct Partnercentral_selling_2022_07_26Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Partnercentral_selling_2022_07_26Service<'a> {
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
