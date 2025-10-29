//! Ecs Service
//!
//! Auto-generated service module for ecs

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for ecs
pub struct EcsService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> EcsService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get container_agent resource handler
    pub fn container_agent(&self) -> resources::Container_agent<'_> {
        resources::Container_agent::new(self.provider)
    }
    /// Get task_protection resource handler
    pub fn task_protection(&self) -> resources::Task_protection<'_> {
        resources::Task_protection::new(self.provider)
    }
    /// Get task_set resource handler
    pub fn task_set(&self) -> resources::Task_set<'_> {
        resources::Task_set::new(self.provider)
    }
    /// Get account_setting resource handler
    pub fn account_setting(&self) -> resources::Account_setting<'_> {
        resources::Account_setting::new(self.provider)
    }
    /// Get cluster_settings resource handler
    pub fn cluster_settings(&self) -> resources::Cluster_settings<'_> {
        resources::Cluster_settings::new(self.provider)
    }
    /// Get service_revisions resource handler
    pub fn service_revisions(&self) -> resources::Service_revisions<'_> {
        resources::Service_revisions::new(self.provider)
    }
    /// Get cluster_capacity_providers resource handler
    pub fn cluster_capacity_providers(&self) -> resources::Cluster_capacity_providers<'_> {
        resources::Cluster_capacity_providers::new(self.provider)
    }
    /// Get cluster resource handler
    pub fn cluster(&self) -> resources::Cluster<'_> {
        resources::Cluster::new(self.provider)
    }
    /// Get service resource handler
    pub fn service(&self) -> resources::Service<'_> {
        resources::Service::new(self.provider)
    }
    /// Get services resource handler
    pub fn services(&self) -> resources::Services<'_> {
        resources::Services::new(self.provider)
    }
    /// Get service_deployments resource handler
    pub fn service_deployments(&self) -> resources::Service_deployments<'_> {
        resources::Service_deployments::new(self.provider)
    }
    /// Get account_setting_default resource handler
    pub fn account_setting_default(&self) -> resources::Account_setting_default<'_> {
        resources::Account_setting_default::new(self.provider)
    }
    /// Get container_instances resource handler
    pub fn container_instances(&self) -> resources::Container_instances<'_> {
        resources::Container_instances::new(self.provider)
    }
    /// Get service_primary_task_set resource handler
    pub fn service_primary_task_set(&self) -> resources::Service_primary_task_set<'_> {
        resources::Service_primary_task_set::new(self.provider)
    }
    /// Get attributes resource handler
    pub fn attributes(&self) -> resources::Attributes<'_> {
        resources::Attributes::new(self.provider)
    }
    /// Get clusters resource handler
    pub fn clusters(&self) -> resources::Clusters<'_> {
        resources::Clusters::new(self.provider)
    }
    /// Get tasks resource handler
    pub fn tasks(&self) -> resources::Tasks<'_> {
        resources::Tasks::new(self.provider)
    }
    /// Get task_sets resource handler
    pub fn task_sets(&self) -> resources::Task_sets<'_> {
        resources::Task_sets::new(self.provider)
    }
    /// Get capacity_provider resource handler
    pub fn capacity_provider(&self) -> resources::Capacity_provider<'_> {
        resources::Capacity_provider::new(self.provider)
    }
    /// Get task_definition resource handler
    pub fn task_definition(&self) -> resources::Task_definition<'_> {
        resources::Task_definition::new(self.provider)
    }
    /// Get container_instances_state resource handler
    pub fn container_instances_state(&self) -> resources::Container_instances_state<'_> {
        resources::Container_instances_state::new(self.provider)
    }
    /// Get task_definitions resource handler
    pub fn task_definitions(&self) -> resources::Task_definitions<'_> {
        resources::Task_definitions::new(self.provider)
    }
    /// Get capacity_providers resource handler
    pub fn capacity_providers(&self) -> resources::Capacity_providers<'_> {
        resources::Capacity_providers::new(self.provider)
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
