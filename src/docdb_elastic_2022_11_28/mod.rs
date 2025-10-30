//! Docdb_elastic_2022_11_28 Service
//!
//! Auto-generated service module for docdb_elastic_2022_11_28

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for docdb_elastic_2022_11_28
pub struct Docdb_elastic_2022_11_28Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Docdb_elastic_2022_11_28Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get cluster_snapshot resource handler
    pub fn cluster_snapshot(&self) -> resources::Cluster_snapshot<'_> {
        resources::Cluster_snapshot::new(self.provider)
    }
    /// Get pending_maintenance_action resource handler
    pub fn pending_maintenance_action(&self) -> resources::Pending_maintenance_action<'_> {
        resources::Pending_maintenance_action::new(self.provider)
    }
    /// Get cluster resource handler
    pub fn cluster(&self) -> resources::Cluster<'_> {
        resources::Cluster::new(self.provider)
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
