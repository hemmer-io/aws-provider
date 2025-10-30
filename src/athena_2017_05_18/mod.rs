//! Athena_2017_05_18 Service
//!
//! Auto-generated service module for athena_2017_05_18

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for athena_2017_05_18
pub struct Athena_2017_05_18Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Athena_2017_05_18Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get calculation_execution_code resource handler
    pub fn calculation_execution_code(&self) -> resources::Calculation_execution_code<'_> {
        resources::Calculation_execution_code::new(self.provider)
    }
    /// Get table_metadata resource handler
    pub fn table_metadata(&self) -> resources::Table_metadata<'_> {
        resources::Table_metadata::new(self.provider)
    }
    /// Get data_catalog resource handler
    pub fn data_catalog(&self) -> resources::Data_catalog<'_> {
        resources::Data_catalog::new(self.provider)
    }
    /// Get query_results resource handler
    pub fn query_results(&self) -> resources::Query_results<'_> {
        resources::Query_results::new(self.provider)
    }
    /// Get query_runtime_statistics resource handler
    pub fn query_runtime_statistics(&self) -> resources::Query_runtime_statistics<'_> {
        resources::Query_runtime_statistics::new(self.provider)
    }
    /// Get query_execution resource handler
    pub fn query_execution(&self) -> resources::Query_execution<'_> {
        resources::Query_execution::new(self.provider)
    }
    /// Get calculation_execution resource handler
    pub fn calculation_execution(&self) -> resources::Calculation_execution<'_> {
        resources::Calculation_execution::new(self.provider)
    }
    /// Get named_query resource handler
    pub fn named_query(&self) -> resources::Named_query<'_> {
        resources::Named_query::new(self.provider)
    }
    /// Get notebook_metadata resource handler
    pub fn notebook_metadata(&self) -> resources::Notebook_metadata<'_> {
        resources::Notebook_metadata::new(self.provider)
    }
    /// Get database resource handler
    pub fn database(&self) -> resources::Database<'_> {
        resources::Database::new(self.provider)
    }
    /// Get capacity_reservation resource handler
    pub fn capacity_reservation(&self) -> resources::Capacity_reservation<'_> {
        resources::Capacity_reservation::new(self.provider)
    }
    /// Get notebook resource handler
    pub fn notebook(&self) -> resources::Notebook<'_> {
        resources::Notebook::new(self.provider)
    }
    /// Get work_group resource handler
    pub fn work_group(&self) -> resources::Work_group<'_> {
        resources::Work_group::new(self.provider)
    }
    /// Get session_status resource handler
    pub fn session_status(&self) -> resources::Session_status<'_> {
        resources::Session_status::new(self.provider)
    }
    /// Get presigned_notebook_url resource handler
    pub fn presigned_notebook_url(&self) -> resources::Presigned_notebook_url<'_> {
        resources::Presigned_notebook_url::new(self.provider)
    }
    /// Get prepared_statement resource handler
    pub fn prepared_statement(&self) -> resources::Prepared_statement<'_> {
        resources::Prepared_statement::new(self.provider)
    }
    /// Get capacity_assignment_configuration resource handler
    pub fn capacity_assignment_configuration(&self) -> resources::Capacity_assignment_configuration<'_> {
        resources::Capacity_assignment_configuration::new(self.provider)
    }
    /// Get calculation_execution_status resource handler
    pub fn calculation_execution_status(&self) -> resources::Calculation_execution_status<'_> {
        resources::Calculation_execution_status::new(self.provider)
    }
    /// Get session resource handler
    pub fn session(&self) -> resources::Session<'_> {
        resources::Session::new(self.provider)
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
