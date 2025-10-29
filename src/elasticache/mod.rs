//! Elasticache Service
//!
//! Auto-generated service module for elasticache

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for elasticache
pub struct ElasticacheService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> ElasticacheService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get reserved_cache_nodes resource handler
    pub fn reserved_cache_nodes(&self) -> resources::Reserved_cache_nodes<'_> {
        resources::Reserved_cache_nodes::new(self.provider)
    }
    /// Get serverless_caches resource handler
    pub fn serverless_caches(&self) -> resources::Serverless_caches<'_> {
        resources::Serverless_caches::new(self.provider)
    }
    /// Get events resource handler
    pub fn events(&self) -> resources::Events<'_> {
        resources::Events::new(self.provider)
    }
    /// Get global_replication_group resource handler
    pub fn global_replication_group(&self) -> resources::Global_replication_group<'_> {
        resources::Global_replication_group::new(self.provider)
    }
    /// Get user_group resource handler
    pub fn user_group(&self) -> resources::User_group<'_> {
        resources::User_group::new(self.provider)
    }
    /// Get cache_parameter_group resource handler
    pub fn cache_parameter_group(&self) -> resources::Cache_parameter_group<'_> {
        resources::Cache_parameter_group::new(self.provider)
    }
    /// Get user resource handler
    pub fn user(&self) -> resources::User<'_> {
        resources::User::new(self.provider)
    }
    /// Get user_groups resource handler
    pub fn user_groups(&self) -> resources::User_groups<'_> {
        resources::User_groups::new(self.provider)
    }
    /// Get replication_group resource handler
    pub fn replication_group(&self) -> resources::Replication_group<'_> {
        resources::Replication_group::new(self.provider)
    }
    /// Get serverless_cache resource handler
    pub fn serverless_cache(&self) -> resources::Serverless_cache<'_> {
        resources::Serverless_cache::new(self.provider)
    }
    /// Get snapshot resource handler
    pub fn snapshot(&self) -> resources::Snapshot<'_> {
        resources::Snapshot::new(self.provider)
    }
    /// Get global_replication_groups resource handler
    pub fn global_replication_groups(&self) -> resources::Global_replication_groups<'_> {
        resources::Global_replication_groups::new(self.provider)
    }
    /// Get cache_subnet_group resource handler
    pub fn cache_subnet_group(&self) -> resources::Cache_subnet_group<'_> {
        resources::Cache_subnet_group::new(self.provider)
    }
    /// Get cache_subnet_groups resource handler
    pub fn cache_subnet_groups(&self) -> resources::Cache_subnet_groups<'_> {
        resources::Cache_subnet_groups::new(self.provider)
    }
    /// Get cache_security_groups resource handler
    pub fn cache_security_groups(&self) -> resources::Cache_security_groups<'_> {
        resources::Cache_security_groups::new(self.provider)
    }
    /// Get engine_default_parameters resource handler
    pub fn engine_default_parameters(&self) -> resources::Engine_default_parameters<'_> {
        resources::Engine_default_parameters::new(self.provider)
    }
    /// Get users resource handler
    pub fn users(&self) -> resources::Users<'_> {
        resources::Users::new(self.provider)
    }
    /// Get cache_cluster resource handler
    pub fn cache_cluster(&self) -> resources::Cache_cluster<'_> {
        resources::Cache_cluster::new(self.provider)
    }
    /// Get cache_clusters resource handler
    pub fn cache_clusters(&self) -> resources::Cache_clusters<'_> {
        resources::Cache_clusters::new(self.provider)
    }
    /// Get reserved_cache_nodes_offerings resource handler
    pub fn reserved_cache_nodes_offerings(&self) -> resources::Reserved_cache_nodes_offerings<'_> {
        resources::Reserved_cache_nodes_offerings::new(self.provider)
    }
    /// Get update_actions resource handler
    pub fn update_actions(&self) -> resources::Update_actions<'_> {
        resources::Update_actions::new(self.provider)
    }
    /// Get replication_groups resource handler
    pub fn replication_groups(&self) -> resources::Replication_groups<'_> {
        resources::Replication_groups::new(self.provider)
    }
    /// Get serverless_cache_snapshot resource handler
    pub fn serverless_cache_snapshot(&self) -> resources::Serverless_cache_snapshot<'_> {
        resources::Serverless_cache_snapshot::new(self.provider)
    }
    /// Get snapshots resource handler
    pub fn snapshots(&self) -> resources::Snapshots<'_> {
        resources::Snapshots::new(self.provider)
    }
    /// Get cache_parameter_groups resource handler
    pub fn cache_parameter_groups(&self) -> resources::Cache_parameter_groups<'_> {
        resources::Cache_parameter_groups::new(self.provider)
    }
    /// Get cache_parameters resource handler
    pub fn cache_parameters(&self) -> resources::Cache_parameters<'_> {
        resources::Cache_parameters::new(self.provider)
    }
    /// Get serverless_cache_snapshots resource handler
    pub fn serverless_cache_snapshots(&self) -> resources::Serverless_cache_snapshots<'_> {
        resources::Serverless_cache_snapshots::new(self.provider)
    }
    /// Get cache_engine_versions resource handler
    pub fn cache_engine_versions(&self) -> resources::Cache_engine_versions<'_> {
        resources::Cache_engine_versions::new(self.provider)
    }
    /// Get cache_security_group resource handler
    pub fn cache_security_group(&self) -> resources::Cache_security_group<'_> {
        resources::Cache_security_group::new(self.provider)
    }
    /// Get service_updates resource handler
    pub fn service_updates(&self) -> resources::Service_updates<'_> {
        resources::Service_updates::new(self.provider)
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
