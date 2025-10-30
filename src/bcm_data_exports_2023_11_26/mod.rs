//! Bcm_data_exports_2023_11_26 Service
//!
//! Auto-generated service module for bcm_data_exports_2023_11_26

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for bcm_data_exports_2023_11_26
pub struct Bcm_data_exports_2023_11_26Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Bcm_data_exports_2023_11_26Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get execution resource handler
    pub fn execution(&self) -> resources::Execution<'_> {
        resources::Execution::new(self.provider)
    }
    /// Get export resource handler
    pub fn export(&self) -> resources::Export<'_> {
        resources::Export::new(self.provider)
    }
    /// Get table resource handler
    pub fn table(&self) -> resources::Table<'_> {
        resources::Table::new(self.provider)
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
