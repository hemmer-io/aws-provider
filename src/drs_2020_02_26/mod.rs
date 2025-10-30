//! Drs_2020_02_26 Service
//!
//! Auto-generated service module for drs_2020_02_26

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for drs_2020_02_26
pub struct Drs_2020_02_26Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Drs_2020_02_26Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get launch_action resource handler
    pub fn launch_action(&self) -> resources::Launch_action<'_> {
        resources::Launch_action::new(self.provider)
    }
    /// Get extended_source_server resource handler
    pub fn extended_source_server(&self) -> resources::Extended_source_server<'_> {
        resources::Extended_source_server::new(self.provider)
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
