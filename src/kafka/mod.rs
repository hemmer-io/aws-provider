//! Kafka Service
//!
//! Auto-generated service module for kafka

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for kafka
pub struct KafkaService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> KafkaService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get cluster_configuration resource handler
    pub fn cluster_configuration(&self) -> resources::Cluster_configuration<'_> {
        resources::Cluster_configuration::new(self.provider)
    }
    /// Get cluster_operation resource handler
    pub fn cluster_operation(&self) -> resources::Cluster_operation<'_> {
        resources::Cluster_operation::new(self.provider)
    }
    /// Get broker_type resource handler
    pub fn broker_type(&self) -> resources::Broker_type<'_> {
        resources::Broker_type::new(self.provider)
    }
    /// Get connectivity resource handler
    pub fn connectivity(&self) -> resources::Connectivity<'_> {
        resources::Connectivity::new(self.provider)
    }
    /// Get monitoring resource handler
    pub fn monitoring(&self) -> resources::Monitoring<'_> {
        resources::Monitoring::new(self.provider)
    }
    /// Get configuration resource handler
    pub fn configuration(&self) -> resources::Configuration<'_> {
        resources::Configuration::new(self.provider)
    }
    /// Get replication_info resource handler
    pub fn replication_info(&self) -> resources::Replication_info<'_> {
        resources::Replication_info::new(self.provider)
    }
    /// Get security resource handler
    pub fn security(&self) -> resources::Security<'_> {
        resources::Security::new(self.provider)
    }
    /// Get cluster_policy resource handler
    pub fn cluster_policy(&self) -> resources::Cluster_policy<'_> {
        resources::Cluster_policy::new(self.provider)
    }
    /// Get broker_storage resource handler
    pub fn broker_storage(&self) -> resources::Broker_storage<'_> {
        resources::Broker_storage::new(self.provider)
    }
    /// Get cluster_v2 resource handler
    pub fn cluster_v2(&self) -> resources::Cluster_v2<'_> {
        resources::Cluster_v2::new(self.provider)
    }
    /// Get replicator resource handler
    pub fn replicator(&self) -> resources::Replicator<'_> {
        resources::Replicator::new(self.provider)
    }
    /// Get compatible_kafka_versions resource handler
    pub fn compatible_kafka_versions(&self) -> resources::Compatible_kafka_versions<'_> {
        resources::Compatible_kafka_versions::new(self.provider)
    }
    /// Get broker_count resource handler
    pub fn broker_count(&self) -> resources::Broker_count<'_> {
        resources::Broker_count::new(self.provider)
    }
    /// Get cluster_operation_v2 resource handler
    pub fn cluster_operation_v2(&self) -> resources::Cluster_operation_v2<'_> {
        resources::Cluster_operation_v2::new(self.provider)
    }
    /// Get vpc_connection resource handler
    pub fn vpc_connection(&self) -> resources::Vpc_connection<'_> {
        resources::Vpc_connection::new(self.provider)
    }
    /// Get cluster_kafka_version resource handler
    pub fn cluster_kafka_version(&self) -> resources::Cluster_kafka_version<'_> {
        resources::Cluster_kafka_version::new(self.provider)
    }
    /// Get cluster resource handler
    pub fn cluster(&self) -> resources::Cluster<'_> {
        resources::Cluster::new(self.provider)
    }
    /// Get configuration_revision resource handler
    pub fn configuration_revision(&self) -> resources::Configuration_revision<'_> {
        resources::Configuration_revision::new(self.provider)
    }
    /// Get bootstrap_brokers resource handler
    pub fn bootstrap_brokers(&self) -> resources::Bootstrap_brokers<'_> {
        resources::Bootstrap_brokers::new(self.provider)
    }
    /// Get storage resource handler
    pub fn storage(&self) -> resources::Storage<'_> {
        resources::Storage::new(self.provider)
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
