//! Dynamodb_2012_08_10 Service
//!
//! Auto-generated service module for dynamodb_2012_08_10

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for dynamodb_2012_08_10
pub struct Dynamodb_2012_08_10Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Dynamodb_2012_08_10Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get global_table_settings resource handler
    pub fn global_table_settings(&self) -> resources::Global_table_settings<'_> {
        resources::Global_table_settings::new(self.provider)
    }
    /// Get endpoints resource handler
    pub fn endpoints(&self) -> resources::Endpoints<'_> {
        resources::Endpoints::new(self.provider)
    }
    /// Get import resource handler
    pub fn import(&self) -> resources::Import<'_> {
        resources::Import::new(self.provider)
    }
    /// Get export resource handler
    pub fn export(&self) -> resources::Export<'_> {
        resources::Export::new(self.provider)
    }
    /// Get table resource handler
    pub fn table(&self) -> resources::Table<'_> {
        resources::Table::new(self.provider)
    }
    /// Get contributor_insights resource handler
    pub fn contributor_insights(&self) -> resources::Contributor_insights<'_> {
        resources::Contributor_insights::new(self.provider)
    }
    /// Get time_to_live resource handler
    pub fn time_to_live(&self) -> resources::Time_to_live<'_> {
        resources::Time_to_live::new(self.provider)
    }
    /// Get table_replica_auto_scaling resource handler
    pub fn table_replica_auto_scaling(&self) -> resources::Table_replica_auto_scaling<'_> {
        resources::Table_replica_auto_scaling::new(self.provider)
    }
    /// Get limits resource handler
    pub fn limits(&self) -> resources::Limits<'_> {
        resources::Limits::new(self.provider)
    }
    /// Get continuous_backups resource handler
    pub fn continuous_backups(&self) -> resources::Continuous_backups<'_> {
        resources::Continuous_backups::new(self.provider)
    }
    /// Get global_table resource handler
    pub fn global_table(&self) -> resources::Global_table<'_> {
        resources::Global_table::new(self.provider)
    }
    /// Get kinesis_streaming_destination resource handler
    pub fn kinesis_streaming_destination(&self) -> resources::Kinesis_streaming_destination<'_> {
        resources::Kinesis_streaming_destination::new(self.provider)
    }
    /// Get backup resource handler
    pub fn backup(&self) -> resources::Backup<'_> {
        resources::Backup::new(self.provider)
    }
    /// Get item resource handler
    pub fn item(&self) -> resources::Item<'_> {
        resources::Item::new(self.provider)
    }
    /// Get resource_policy resource handler
    pub fn resource_policy(&self) -> resources::Resource_policy<'_> {
        resources::Resource_policy::new(self.provider)
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
