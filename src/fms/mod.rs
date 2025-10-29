//! Fms Service
//!
//! Auto-generated service module for fms

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for fms
pub struct FmsService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> FmsService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get admin_scope resource handler
    pub fn admin_scope(&self) -> resources::Admin_scope<'_> {
        resources::Admin_scope::new(self.provider)
    }
    /// Get violation_details resource handler
    pub fn violation_details(&self) -> resources::Violation_details<'_> {
        resources::Violation_details::new(self.provider)
    }
    /// Get policy resource handler
    pub fn policy(&self) -> resources::Policy<'_> {
        resources::Policy::new(self.provider)
    }
    /// Get compliance_detail resource handler
    pub fn compliance_detail(&self) -> resources::Compliance_detail<'_> {
        resources::Compliance_detail::new(self.provider)
    }
    /// Get apps_list resource handler
    pub fn apps_list(&self) -> resources::Apps_list<'_> {
        resources::Apps_list::new(self.provider)
    }
    /// Get resource_set resource handler
    pub fn resource_set(&self) -> resources::Resource_set<'_> {
        resources::Resource_set::new(self.provider)
    }
    /// Get third_party_firewall_association_status resource handler
    pub fn third_party_firewall_association_status(&self) -> resources::Third_party_firewall_association_status<'_> {
        resources::Third_party_firewall_association_status::new(self.provider)
    }
    /// Get admin_account resource handler
    pub fn admin_account(&self) -> resources::Admin_account<'_> {
        resources::Admin_account::new(self.provider)
    }
    /// Get protocols_list resource handler
    pub fn protocols_list(&self) -> resources::Protocols_list<'_> {
        resources::Protocols_list::new(self.provider)
    }
    /// Get notification_channel resource handler
    pub fn notification_channel(&self) -> resources::Notification_channel<'_> {
        resources::Notification_channel::new(self.provider)
    }
    /// Get protection_status resource handler
    pub fn protection_status(&self) -> resources::Protection_status<'_> {
        resources::Protection_status::new(self.provider)
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
