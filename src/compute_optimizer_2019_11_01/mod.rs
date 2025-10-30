//! Compute_optimizer_2019_11_01 Service
//!
//! Auto-generated service module for compute_optimizer_2019_11_01

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for compute_optimizer_2019_11_01
pub struct Compute_optimizer_2019_11_01Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Compute_optimizer_2019_11_01Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get ebs_volume_recommendations resource handler
    pub fn ebs_volume_recommendations(&self) -> resources::Ebs_volume_recommendations<'_> {
        resources::Ebs_volume_recommendations::new(self.provider)
    }
    /// Get lambda_function_recommendations resource handler
    pub fn lambda_function_recommendations(&self) -> resources::Lambda_function_recommendations<'_> {
        resources::Lambda_function_recommendations::new(self.provider)
    }
    /// Get rds_database_recommendations resource handler
    pub fn rds_database_recommendations(&self) -> resources::Rds_database_recommendations<'_> {
        resources::Rds_database_recommendations::new(self.provider)
    }
    /// Get recommendation_export_jobs resource handler
    pub fn recommendation_export_jobs(&self) -> resources::Recommendation_export_jobs<'_> {
        resources::Recommendation_export_jobs::new(self.provider)
    }
    /// Get ec2_instance_recommendations resource handler
    pub fn ec2_instance_recommendations(&self) -> resources::Ec2_instance_recommendations<'_> {
        resources::Ec2_instance_recommendations::new(self.provider)
    }
    /// Get license_recommendations resource handler
    pub fn license_recommendations(&self) -> resources::License_recommendations<'_> {
        resources::License_recommendations::new(self.provider)
    }
    /// Get recommendation_preferences resource handler
    pub fn recommendation_preferences(&self) -> resources::Recommendation_preferences<'_> {
        resources::Recommendation_preferences::new(self.provider)
    }
    /// Get ecs_service_recommendation_projected_metrics resource handler
    pub fn ecs_service_recommendation_projected_metrics(&self) -> resources::Ecs_service_recommendation_projected_metrics<'_> {
        resources::Ecs_service_recommendation_projected_metrics::new(self.provider)
    }
    /// Get enrollment_status resource handler
    pub fn enrollment_status(&self) -> resources::Enrollment_status<'_> {
        resources::Enrollment_status::new(self.provider)
    }
    /// Get ecs_service_recommendations resource handler
    pub fn ecs_service_recommendations(&self) -> resources::Ecs_service_recommendations<'_> {
        resources::Ecs_service_recommendations::new(self.provider)
    }
    /// Get auto_scaling_group_recommendations resource handler
    pub fn auto_scaling_group_recommendations(&self) -> resources::Auto_scaling_group_recommendations<'_> {
        resources::Auto_scaling_group_recommendations::new(self.provider)
    }
    /// Get idle_recommendations resource handler
    pub fn idle_recommendations(&self) -> resources::Idle_recommendations<'_> {
        resources::Idle_recommendations::new(self.provider)
    }
    /// Get effective_recommendation_preferences resource handler
    pub fn effective_recommendation_preferences(&self) -> resources::Effective_recommendation_preferences<'_> {
        resources::Effective_recommendation_preferences::new(self.provider)
    }
    /// Get rds_database_recommendation_projected_metrics resource handler
    pub fn rds_database_recommendation_projected_metrics(&self) -> resources::Rds_database_recommendation_projected_metrics<'_> {
        resources::Rds_database_recommendation_projected_metrics::new(self.provider)
    }
    /// Get enrollment_statuses_for_organization resource handler
    pub fn enrollment_statuses_for_organization(&self) -> resources::Enrollment_statuses_for_organization<'_> {
        resources::Enrollment_statuses_for_organization::new(self.provider)
    }
    /// Get recommendation_summaries resource handler
    pub fn recommendation_summaries(&self) -> resources::Recommendation_summaries<'_> {
        resources::Recommendation_summaries::new(self.provider)
    }
    /// Get ec2_recommendation_projected_metrics resource handler
    pub fn ec2_recommendation_projected_metrics(&self) -> resources::Ec2_recommendation_projected_metrics<'_> {
        resources::Ec2_recommendation_projected_metrics::new(self.provider)
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
