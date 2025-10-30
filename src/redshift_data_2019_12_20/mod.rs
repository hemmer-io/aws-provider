//! Redshift_data_2019_12_20 Service
//!
//! Auto-generated service module for redshift_data_2019_12_20

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for redshift_data_2019_12_20
pub struct Redshift_data_2019_12_20Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Redshift_data_2019_12_20Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get table resource handler
    pub fn table(&self) -> resources::Table<'_> {
        resources::Table::new(self.provider)
    }
    /// Get statement resource handler
    pub fn statement(&self) -> resources::Statement<'_> {
        resources::Statement::new(self.provider)
    }
    /// Get statement_result_v2 resource handler
    pub fn statement_result_v2(&self) -> resources::Statement_result_v2<'_> {
        resources::Statement_result_v2::new(self.provider)
    }
    /// Get statement_result resource handler
    pub fn statement_result(&self) -> resources::Statement_result<'_> {
        resources::Statement_result::new(self.provider)
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
