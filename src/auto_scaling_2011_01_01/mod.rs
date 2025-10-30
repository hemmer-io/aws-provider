//! Auto_scaling_2011_01_01 Service
//!
//! Auto-generated service module for auto_scaling_2011_01_01

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for auto_scaling_2011_01_01
pub struct Auto_scaling_2011_01_01Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Auto_scaling_2011_01_01Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get account_limits resource handler
    pub fn account_limits(&self) -> resources::Account_limits<'_> {
        resources::Account_limits::new(self.provider)
    }
    /// Get launch_configurations resource handler
    pub fn launch_configurations(&self) -> resources::Launch_configurations<'_> {
        resources::Launch_configurations::new(self.provider)
    }
    /// Get traffic_sources resource handler
    pub fn traffic_sources(&self) -> resources::Traffic_sources<'_> {
        resources::Traffic_sources::new(self.provider)
    }
    /// Get auto_scaling_notification_types resource handler
    pub fn auto_scaling_notification_types(&self) -> resources::Auto_scaling_notification_types<'_> {
        resources::Auto_scaling_notification_types::new(self.provider)
    }
    /// Get launch_configuration resource handler
    pub fn launch_configuration(&self) -> resources::Launch_configuration<'_> {
        resources::Launch_configuration::new(self.provider)
    }
    /// Get or_update_tags resource handler
    pub fn or_update_tags(&self) -> resources::Or_update_tags<'_> {
        resources::Or_update_tags::new(self.provider)
    }
    /// Get policy resource handler
    pub fn policy(&self) -> resources::Policy<'_> {
        resources::Policy::new(self.provider)
    }
    /// Get lifecycle_hooks resource handler
    pub fn lifecycle_hooks(&self) -> resources::Lifecycle_hooks<'_> {
        resources::Lifecycle_hooks::new(self.provider)
    }
    /// Get load_balancer_target_groups resource handler
    pub fn load_balancer_target_groups(&self) -> resources::Load_balancer_target_groups<'_> {
        resources::Load_balancer_target_groups::new(self.provider)
    }
    /// Get load_balancers resource handler
    pub fn load_balancers(&self) -> resources::Load_balancers<'_> {
        resources::Load_balancers::new(self.provider)
    }
    /// Get scheduled_action resource handler
    pub fn scheduled_action(&self) -> resources::Scheduled_action<'_> {
        resources::Scheduled_action::new(self.provider)
    }
    /// Get auto_scaling_groups resource handler
    pub fn auto_scaling_groups(&self) -> resources::Auto_scaling_groups<'_> {
        resources::Auto_scaling_groups::new(self.provider)
    }
    /// Get scheduled_actions resource handler
    pub fn scheduled_actions(&self) -> resources::Scheduled_actions<'_> {
        resources::Scheduled_actions::new(self.provider)
    }
    /// Get scaling_policy resource handler
    pub fn scaling_policy(&self) -> resources::Scaling_policy<'_> {
        resources::Scaling_policy::new(self.provider)
    }
    /// Get warm_pool resource handler
    pub fn warm_pool(&self) -> resources::Warm_pool<'_> {
        resources::Warm_pool::new(self.provider)
    }
    /// Get lifecycle_hook resource handler
    pub fn lifecycle_hook(&self) -> resources::Lifecycle_hook<'_> {
        resources::Lifecycle_hook::new(self.provider)
    }
    /// Get notification_configurations resource handler
    pub fn notification_configurations(&self) -> resources::Notification_configurations<'_> {
        resources::Notification_configurations::new(self.provider)
    }
    /// Get termination_policy_types resource handler
    pub fn termination_policy_types(&self) -> resources::Termination_policy_types<'_> {
        resources::Termination_policy_types::new(self.provider)
    }
    /// Get scaling_activities resource handler
    pub fn scaling_activities(&self) -> resources::Scaling_activities<'_> {
        resources::Scaling_activities::new(self.provider)
    }
    /// Get instance_refreshes resource handler
    pub fn instance_refreshes(&self) -> resources::Instance_refreshes<'_> {
        resources::Instance_refreshes::new(self.provider)
    }
    /// Get metric_collection_types resource handler
    pub fn metric_collection_types(&self) -> resources::Metric_collection_types<'_> {
        resources::Metric_collection_types::new(self.provider)
    }
    /// Get tags resource handler
    pub fn tags(&self) -> resources::Tags<'_> {
        resources::Tags::new(self.provider)
    }
    /// Get scaling_process_types resource handler
    pub fn scaling_process_types(&self) -> resources::Scaling_process_types<'_> {
        resources::Scaling_process_types::new(self.provider)
    }
    /// Get lifecycle_hook_types resource handler
    pub fn lifecycle_hook_types(&self) -> resources::Lifecycle_hook_types<'_> {
        resources::Lifecycle_hook_types::new(self.provider)
    }
    /// Get auto_scaling_instances resource handler
    pub fn auto_scaling_instances(&self) -> resources::Auto_scaling_instances<'_> {
        resources::Auto_scaling_instances::new(self.provider)
    }
    /// Get scheduled_update_group_action resource handler
    pub fn scheduled_update_group_action(&self) -> resources::Scheduled_update_group_action<'_> {
        resources::Scheduled_update_group_action::new(self.provider)
    }
    /// Get policies resource handler
    pub fn policies(&self) -> resources::Policies<'_> {
        resources::Policies::new(self.provider)
    }
    /// Get auto_scaling_group resource handler
    pub fn auto_scaling_group(&self) -> resources::Auto_scaling_group<'_> {
        resources::Auto_scaling_group::new(self.provider)
    }
    /// Get adjustment_types resource handler
    pub fn adjustment_types(&self) -> resources::Adjustment_types<'_> {
        resources::Adjustment_types::new(self.provider)
    }
    /// Get notification_configuration resource handler
    pub fn notification_configuration(&self) -> resources::Notification_configuration<'_> {
        resources::Notification_configuration::new(self.provider)
    }
    /// Get predictive_scaling_forecast resource handler
    pub fn predictive_scaling_forecast(&self) -> resources::Predictive_scaling_forecast<'_> {
        resources::Predictive_scaling_forecast::new(self.provider)
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
