//! Eks Service
//!
//! Auto-generated service module for eks

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for eks
pub struct EksService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> EksService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get addon resource handler
    pub fn addon(&self) -> resources::Addon<'_> {
        resources::Addon::new(self.provider)
    }
    /// Get addon_configuration resource handler
    pub fn addon_configuration(&self) -> resources::Addon_configuration<'_> {
        resources::Addon_configuration::new(self.provider)
    }
    /// Get cluster_config resource handler
    pub fn cluster_config(&self) -> resources::Cluster_config<'_> {
        resources::Cluster_config::new(self.provider)
    }
    /// Get identity_provider_config resource handler
    pub fn identity_provider_config(&self) -> resources::Identity_provider_config<'_> {
        resources::Identity_provider_config::new(self.provider)
    }
    /// Get nodegroup_config resource handler
    pub fn nodegroup_config(&self) -> resources::Nodegroup_config<'_> {
        resources::Nodegroup_config::new(self.provider)
    }
    /// Get update resource handler
    pub fn update(&self) -> resources::Update<'_> {
        resources::Update::new(self.provider)
    }
    /// Get insights_refresh resource handler
    pub fn insights_refresh(&self) -> resources::Insights_refresh<'_> {
        resources::Insights_refresh::new(self.provider)
    }
    /// Get addon_versions resource handler
    pub fn addon_versions(&self) -> resources::Addon_versions<'_> {
        resources::Addon_versions::new(self.provider)
    }
    /// Get cluster_versions resource handler
    pub fn cluster_versions(&self) -> resources::Cluster_versions<'_> {
        resources::Cluster_versions::new(self.provider)
    }
    /// Get pod_identity_association resource handler
    pub fn pod_identity_association(&self) -> resources::Pod_identity_association<'_> {
        resources::Pod_identity_association::new(self.provider)
    }
    /// Get access_entry resource handler
    pub fn access_entry(&self) -> resources::Access_entry<'_> {
        resources::Access_entry::new(self.provider)
    }
    /// Get nodegroup resource handler
    pub fn nodegroup(&self) -> resources::Nodegroup<'_> {
        resources::Nodegroup::new(self.provider)
    }
    /// Get nodegroup_version resource handler
    pub fn nodegroup_version(&self) -> resources::Nodegroup_version<'_> {
        resources::Nodegroup_version::new(self.provider)
    }
    /// Get cluster resource handler
    pub fn cluster(&self) -> resources::Cluster<'_> {
        resources::Cluster::new(self.provider)
    }
    /// Get insight resource handler
    pub fn insight(&self) -> resources::Insight<'_> {
        resources::Insight::new(self.provider)
    }
    /// Get eks_anywhere_subscription resource handler
    pub fn eks_anywhere_subscription(&self) -> resources::Eks_anywhere_subscription<'_> {
        resources::Eks_anywhere_subscription::new(self.provider)
    }
    /// Get fargate_profile resource handler
    pub fn fargate_profile(&self) -> resources::Fargate_profile<'_> {
        resources::Fargate_profile::new(self.provider)
    }
    /// Get cluster_version resource handler
    pub fn cluster_version(&self) -> resources::Cluster_version<'_> {
        resources::Cluster_version::new(self.provider)
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
