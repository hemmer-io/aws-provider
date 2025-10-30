//! Finspace_2021_03_12 Service
//!
//! Auto-generated service module for finspace_2021_03_12

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for finspace_2021_03_12
pub struct Finspace_2021_03_12Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Finspace_2021_03_12Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get kx_volume resource handler
    pub fn kx_volume(&self) -> resources::Kx_volume<'_> {
        resources::Kx_volume::new(self.provider)
    }
    /// Get kx_scaling_group resource handler
    pub fn kx_scaling_group(&self) -> resources::Kx_scaling_group<'_> {
        resources::Kx_scaling_group::new(self.provider)
    }
    /// Get kx_environment_network resource handler
    pub fn kx_environment_network(&self) -> resources::Kx_environment_network<'_> {
        resources::Kx_environment_network::new(self.provider)
    }
    /// Get kx_database resource handler
    pub fn kx_database(&self) -> resources::Kx_database<'_> {
        resources::Kx_database::new(self.provider)
    }
    /// Get kx_cluster_code_configuration resource handler
    pub fn kx_cluster_code_configuration(&self) -> resources::Kx_cluster_code_configuration<'_> {
        resources::Kx_cluster_code_configuration::new(self.provider)
    }
    /// Get kx_cluster_node resource handler
    pub fn kx_cluster_node(&self) -> resources::Kx_cluster_node<'_> {
        resources::Kx_cluster_node::new(self.provider)
    }
    /// Get kx_user resource handler
    pub fn kx_user(&self) -> resources::Kx_user<'_> {
        resources::Kx_user::new(self.provider)
    }
    /// Get environment resource handler
    pub fn environment(&self) -> resources::Environment<'_> {
        resources::Environment::new(self.provider)
    }
    /// Get kx_dataview resource handler
    pub fn kx_dataview(&self) -> resources::Kx_dataview<'_> {
        resources::Kx_dataview::new(self.provider)
    }
    /// Get kx_connection_string resource handler
    pub fn kx_connection_string(&self) -> resources::Kx_connection_string<'_> {
        resources::Kx_connection_string::new(self.provider)
    }
    /// Get kx_changeset resource handler
    pub fn kx_changeset(&self) -> resources::Kx_changeset<'_> {
        resources::Kx_changeset::new(self.provider)
    }
    /// Get kx_environment resource handler
    pub fn kx_environment(&self) -> resources::Kx_environment<'_> {
        resources::Kx_environment::new(self.provider)
    }
    /// Get kx_cluster resource handler
    pub fn kx_cluster(&self) -> resources::Kx_cluster<'_> {
        resources::Kx_cluster::new(self.provider)
    }
    /// Get kx_cluster_databases resource handler
    pub fn kx_cluster_databases(&self) -> resources::Kx_cluster_databases<'_> {
        resources::Kx_cluster_databases::new(self.provider)
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
