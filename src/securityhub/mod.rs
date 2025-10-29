//! Securityhub Service
//!
//! Auto-generated service module for securityhub

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for securityhub
pub struct SecurityhubService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> SecurityhubService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get security_control_definition resource handler
    pub fn security_control_definition(&self) -> resources::Security_control_definition<'_> {
        resources::Security_control_definition::new(self.provider)
    }
    /// Get connector_v2 resource handler
    pub fn connector_v2(&self) -> resources::Connector_v2<'_> {
        resources::Connector_v2::new(self.provider)
    }
    /// Get invitations_count resource handler
    pub fn invitations_count(&self) -> resources::Invitations_count<'_> {
        resources::Invitations_count::new(self.provider)
    }
    /// Get findings resource handler
    pub fn findings(&self) -> resources::Findings<'_> {
        resources::Findings::new(self.provider)
    }
    /// Get security_control resource handler
    pub fn security_control(&self) -> resources::Security_control<'_> {
        resources::Security_control::new(self.provider)
    }
    /// Get organization_configuration resource handler
    pub fn organization_configuration(&self) -> resources::Organization_configuration<'_> {
        resources::Organization_configuration::new(self.provider)
    }
    /// Get finding_aggregator resource handler
    pub fn finding_aggregator(&self) -> resources::Finding_aggregator<'_> {
        resources::Finding_aggregator::new(self.provider)
    }
    /// Get resources_v2 resource handler
    pub fn resources_v2(&self) -> resources::Resources_v2<'_> {
        resources::Resources_v2::new(self.provider)
    }
    /// Get finding_history resource handler
    pub fn finding_history(&self) -> resources::Finding_history<'_> {
        resources::Finding_history::new(self.provider)
    }
    /// Get standards resource handler
    pub fn standards(&self) -> resources::Standards<'_> {
        resources::Standards::new(self.provider)
    }
    /// Get standards_control resource handler
    pub fn standards_control(&self) -> resources::Standards_control<'_> {
        resources::Standards_control::new(self.provider)
    }
    /// Get insights resource handler
    pub fn insights(&self) -> resources::Insights<'_> {
        resources::Insights::new(self.provider)
    }
    /// Get security_hub_configuration resource handler
    pub fn security_hub_configuration(&self) -> resources::Security_hub_configuration<'_> {
        resources::Security_hub_configuration::new(self.provider)
    }
    /// Get insight_results resource handler
    pub fn insight_results(&self) -> resources::Insight_results<'_> {
        resources::Insight_results::new(self.provider)
    }
    /// Get standards_controls resource handler
    pub fn standards_controls(&self) -> resources::Standards_controls<'_> {
        resources::Standards_controls::new(self.provider)
    }
    /// Get finding_statistics_v2 resource handler
    pub fn finding_statistics_v2(&self) -> resources::Finding_statistics_v2<'_> {
        resources::Finding_statistics_v2::new(self.provider)
    }
    /// Get master_account resource handler
    pub fn master_account(&self) -> resources::Master_account<'_> {
        resources::Master_account::new(self.provider)
    }
    /// Get invitations resource handler
    pub fn invitations(&self) -> resources::Invitations<'_> {
        resources::Invitations::new(self.provider)
    }
    /// Get enabled_standards resource handler
    pub fn enabled_standards(&self) -> resources::Enabled_standards<'_> {
        resources::Enabled_standards::new(self.provider)
    }
    /// Get aggregator_v2 resource handler
    pub fn aggregator_v2(&self) -> resources::Aggregator_v2<'_> {
        resources::Aggregator_v2::new(self.provider)
    }
    /// Get products resource handler
    pub fn products(&self) -> resources::Products<'_> {
        resources::Products::new(self.provider)
    }
    /// Get findings_v2 resource handler
    pub fn findings_v2(&self) -> resources::Findings_v2<'_> {
        resources::Findings_v2::new(self.provider)
    }
    /// Get ticket_v2 resource handler
    pub fn ticket_v2(&self) -> resources::Ticket_v2<'_> {
        resources::Ticket_v2::new(self.provider)
    }
    /// Get insight resource handler
    pub fn insight(&self) -> resources::Insight<'_> {
        resources::Insight::new(self.provider)
    }
    /// Get resources_statistics_v2 resource handler
    pub fn resources_statistics_v2(&self) -> resources::Resources_statistics_v2<'_> {
        resources::Resources_statistics_v2::new(self.provider)
    }
    /// Get action_targets resource handler
    pub fn action_targets(&self) -> resources::Action_targets<'_> {
        resources::Action_targets::new(self.provider)
    }
    /// Get products_v2 resource handler
    pub fn products_v2(&self) -> resources::Products_v2<'_> {
        resources::Products_v2::new(self.provider)
    }
    /// Get automation_rule_v2 resource handler
    pub fn automation_rule_v2(&self) -> resources::Automation_rule_v2<'_> {
        resources::Automation_rule_v2::new(self.provider)
    }
    /// Get automation_rule resource handler
    pub fn automation_rule(&self) -> resources::Automation_rule<'_> {
        resources::Automation_rule::new(self.provider)
    }
    /// Get configuration_policy resource handler
    pub fn configuration_policy(&self) -> resources::Configuration_policy<'_> {
        resources::Configuration_policy::new(self.provider)
    }
    /// Get action_target resource handler
    pub fn action_target(&self) -> resources::Action_target<'_> {
        resources::Action_target::new(self.provider)
    }
    /// Get members resource handler
    pub fn members(&self) -> resources::Members<'_> {
        resources::Members::new(self.provider)
    }
    /// Get configuration_policy_association resource handler
    pub fn configuration_policy_association(&self) -> resources::Configuration_policy_association<'_> {
        resources::Configuration_policy_association::new(self.provider)
    }
    /// Get hub resource handler
    pub fn hub(&self) -> resources::Hub<'_> {
        resources::Hub::new(self.provider)
    }
    /// Get security_hub_v2 resource handler
    pub fn security_hub_v2(&self) -> resources::Security_hub_v2<'_> {
        resources::Security_hub_v2::new(self.provider)
    }
    /// Get administrator_account resource handler
    pub fn administrator_account(&self) -> resources::Administrator_account<'_> {
        resources::Administrator_account::new(self.provider)
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
