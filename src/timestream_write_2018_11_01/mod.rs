//! Timestream_write_2018_11_01 Service
//!
//! Auto-generated service module for timestream_write_2018_11_01

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for timestream_write_2018_11_01
pub struct Timestream_write_2018_11_01Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Timestream_write_2018_11_01Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get database resource handler
    pub fn database(&self) -> resources::Database<'_> {
        resources::Database::new(self.provider)
    }
    /// Get endpoints resource handler
    pub fn endpoints(&self) -> resources::Endpoints<'_> {
        resources::Endpoints::new(self.provider)
    }
    /// Get batch_load_task resource handler
    pub fn batch_load_task(&self) -> resources::Batch_load_task<'_> {
        resources::Batch_load_task::new(self.provider)
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
