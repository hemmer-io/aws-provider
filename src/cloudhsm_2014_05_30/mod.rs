//! Cloudhsm_2014_05_30 Service
//!
//! Auto-generated service module for cloudhsm_2014_05_30

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for cloudhsm_2014_05_30
pub struct Cloudhsm_2014_05_30Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Cloudhsm_2014_05_30Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get hapg resource handler
    pub fn hapg(&self) -> resources::Hapg<'_> {
        resources::Hapg::new(self.provider)
    }
    /// Get config resource handler
    pub fn config(&self) -> resources::Config<'_> {
        resources::Config::new(self.provider)
    }
    /// Get luna_client resource handler
    pub fn luna_client(&self) -> resources::Luna_client<'_> {
        resources::Luna_client::new(self.provider)
    }
    /// Get hsm resource handler
    pub fn hsm(&self) -> resources::Hsm<'_> {
        resources::Hsm::new(self.provider)
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
