//! Cloudformation Service
//!
//! Auto-generated service module for cloudformation

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for cloudformation
pub struct CloudformationService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> CloudformationService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get stack_events resource handler
    pub fn stack_events(&self) -> resources::Stack_events<'_> {
        resources::Stack_events::new(self.provider)
    }
    /// Get type resource handler
    pub fn type(&self) -> resources::Type<'_> {
        resources::Type::new(self.provider)
    }
    /// Get resource_scan resource handler
    pub fn resource_scan(&self) -> resources::Resource_scan<'_> {
        resources::Resource_scan::new(self.provider)
    }
    /// Get publisher resource handler
    pub fn publisher(&self) -> resources::Publisher<'_> {
        resources::Publisher::new(self.provider)
    }
    /// Get stack_instance resource handler
    pub fn stack_instance(&self) -> resources::Stack_instance<'_> {
        resources::Stack_instance::new(self.provider)
    }
    /// Get termination_protection resource handler
    pub fn termination_protection(&self) -> resources::Termination_protection<'_> {
        resources::Termination_protection::new(self.provider)
    }
    /// Get stack_instances resource handler
    pub fn stack_instances(&self) -> resources::Stack_instances<'_> {
        resources::Stack_instances::new(self.provider)
    }
    /// Get stack_policy resource handler
    pub fn stack_policy(&self) -> resources::Stack_policy<'_> {
        resources::Stack_policy::new(self.provider)
    }
    /// Get change_set resource handler
    pub fn change_set(&self) -> resources::Change_set<'_> {
        resources::Change_set::new(self.provider)
    }
    /// Get stack_set_operation resource handler
    pub fn stack_set_operation(&self) -> resources::Stack_set_operation<'_> {
        resources::Stack_set_operation::new(self.provider)
    }
    /// Get organizations_access resource handler
    pub fn organizations_access(&self) -> resources::Organizations_access<'_> {
        resources::Organizations_access::new(self.provider)
    }
    /// Get account_limits resource handler
    pub fn account_limits(&self) -> resources::Account_limits<'_> {
        resources::Account_limits::new(self.provider)
    }
    /// Get stack_refactor resource handler
    pub fn stack_refactor(&self) -> resources::Stack_refactor<'_> {
        resources::Stack_refactor::new(self.provider)
    }
    /// Get change_set_hooks resource handler
    pub fn change_set_hooks(&self) -> resources::Change_set_hooks<'_> {
        resources::Change_set_hooks::new(self.provider)
    }
    /// Get stack_set resource handler
    pub fn stack_set(&self) -> resources::Stack_set<'_> {
        resources::Stack_set::new(self.provider)
    }
    /// Get stack_drift_detection_status resource handler
    pub fn stack_drift_detection_status(&self) -> resources::Stack_drift_detection_status<'_> {
        resources::Stack_drift_detection_status::new(self.provider)
    }
    /// Get stacks resource handler
    pub fn stacks(&self) -> resources::Stacks<'_> {
        resources::Stacks::new(self.provider)
    }
    /// Get stack_resource_drifts resource handler
    pub fn stack_resource_drifts(&self) -> resources::Stack_resource_drifts<'_> {
        resources::Stack_resource_drifts::new(self.provider)
    }
    /// Get stack_resource resource handler
    pub fn stack_resource(&self) -> resources::Stack_resource<'_> {
        resources::Stack_resource::new(self.provider)
    }
    /// Get stack_resources resource handler
    pub fn stack_resources(&self) -> resources::Stack_resources<'_> {
        resources::Stack_resources::new(self.provider)
    }
    /// Get template resource handler
    pub fn template(&self) -> resources::Template<'_> {
        resources::Template::new(self.provider)
    }
    /// Get stack resource handler
    pub fn stack(&self) -> resources::Stack<'_> {
        resources::Stack::new(self.provider)
    }
    /// Get template_summary resource handler
    pub fn template_summary(&self) -> resources::Template_summary<'_> {
        resources::Template_summary::new(self.provider)
    }
    /// Get type_registration resource handler
    pub fn type_registration(&self) -> resources::Type_registration<'_> {
        resources::Type_registration::new(self.provider)
    }
    /// Get generated_template resource handler
    pub fn generated_template(&self) -> resources::Generated_template<'_> {
        resources::Generated_template::new(self.provider)
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
