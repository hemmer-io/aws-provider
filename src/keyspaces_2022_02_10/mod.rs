//! Keyspaces_2022_02_10 Service
//!
//! Auto-generated service module for keyspaces_2022_02_10

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for keyspaces_2022_02_10
pub struct Keyspaces_2022_02_10Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Keyspaces_2022_02_10Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get type resource handler
    pub fn type(&self) -> resources::Type<'_> {
        resources::Type::new(self.provider)
    }
    /// Get table_auto_scaling_settings resource handler
    pub fn table_auto_scaling_settings(&self) -> resources::Table_auto_scaling_settings<'_> {
        resources::Table_auto_scaling_settings::new(self.provider)
    }
    /// Get keyspace resource handler
    pub fn keyspace(&self) -> resources::Keyspace<'_> {
        resources::Keyspace::new(self.provider)
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
