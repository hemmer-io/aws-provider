//! Elastic_load_balancing Service
//!
//! Auto-generated service module for elastic_load_balancing

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for elastic_load_balancing
pub struct Elastic_load_balancingService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Elastic_load_balancingService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get listener_certificates resource handler
    pub fn listener_certificates(&self) -> resources::Listener_certificates<'_> {
        resources::Listener_certificates::new(self.provider)
    }
    /// Get load_balancer_attributes resource handler
    pub fn load_balancer_attributes(&self) -> resources::Load_balancer_attributes<'_> {
        resources::Load_balancer_attributes::new(self.provider)
    }
    /// Get rule resource handler
    pub fn rule(&self) -> resources::Rule<'_> {
        resources::Rule::new(self.provider)
    }
    /// Get load_balancer resource handler
    pub fn load_balancer(&self) -> resources::Load_balancer<'_> {
        resources::Load_balancer::new(self.provider)
    }
    /// Get target_group resource handler
    pub fn target_group(&self) -> resources::Target_group<'_> {
        resources::Target_group::new(self.provider)
    }
    /// Get trust_store_associations resource handler
    pub fn trust_store_associations(&self) -> resources::Trust_store_associations<'_> {
        resources::Trust_store_associations::new(self.provider)
    }
    /// Get trust_store_revocations resource handler
    pub fn trust_store_revocations(&self) -> resources::Trust_store_revocations<'_> {
        resources::Trust_store_revocations::new(self.provider)
    }
    /// Get resource_policy resource handler
    pub fn resource_policy(&self) -> resources::Resource_policy<'_> {
        resources::Resource_policy::new(self.provider)
    }
    /// Get load_balancers resource handler
    pub fn load_balancers(&self) -> resources::Load_balancers<'_> {
        resources::Load_balancers::new(self.provider)
    }
    /// Get trust_store_ca_certificates_bundle resource handler
    pub fn trust_store_ca_certificates_bundle(&self) -> resources::Trust_store_ca_certificates_bundle<'_> {
        resources::Trust_store_ca_certificates_bundle::new(self.provider)
    }
    /// Get target_groups resource handler
    pub fn target_groups(&self) -> resources::Target_groups<'_> {
        resources::Target_groups::new(self.provider)
    }
    /// Get trust_store_revocation_content resource handler
    pub fn trust_store_revocation_content(&self) -> resources::Trust_store_revocation_content<'_> {
        resources::Trust_store_revocation_content::new(self.provider)
    }
    /// Get shared_trust_store_association resource handler
    pub fn shared_trust_store_association(&self) -> resources::Shared_trust_store_association<'_> {
        resources::Shared_trust_store_association::new(self.provider)
    }
    /// Get listener resource handler
    pub fn listener(&self) -> resources::Listener<'_> {
        resources::Listener::new(self.provider)
    }
    /// Get listener_attributes resource handler
    pub fn listener_attributes(&self) -> resources::Listener_attributes<'_> {
        resources::Listener_attributes::new(self.provider)
    }
    /// Get listeners resource handler
    pub fn listeners(&self) -> resources::Listeners<'_> {
        resources::Listeners::new(self.provider)
    }
    /// Get ssl_policies resource handler
    pub fn ssl_policies(&self) -> resources::Ssl_policies<'_> {
        resources::Ssl_policies::new(self.provider)
    }
    /// Get trust_stores resource handler
    pub fn trust_stores(&self) -> resources::Trust_stores<'_> {
        resources::Trust_stores::new(self.provider)
    }
    /// Get target_group_attributes resource handler
    pub fn target_group_attributes(&self) -> resources::Target_group_attributes<'_> {
        resources::Target_group_attributes::new(self.provider)
    }
    /// Get account_limits resource handler
    pub fn account_limits(&self) -> resources::Account_limits<'_> {
        resources::Account_limits::new(self.provider)
    }
    /// Get capacity_reservation resource handler
    pub fn capacity_reservation(&self) -> resources::Capacity_reservation<'_> {
        resources::Capacity_reservation::new(self.provider)
    }
    /// Get rules resource handler
    pub fn rules(&self) -> resources::Rules<'_> {
        resources::Rules::new(self.provider)
    }
    /// Get trust_store resource handler
    pub fn trust_store(&self) -> resources::Trust_store<'_> {
        resources::Trust_store::new(self.provider)
    }
    /// Get tags resource handler
    pub fn tags(&self) -> resources::Tags<'_> {
        resources::Tags::new(self.provider)
    }
    /// Get target_health resource handler
    pub fn target_health(&self) -> resources::Target_health<'_> {
        resources::Target_health::new(self.provider)
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
