//! Wafv2 Service
//!
//! Auto-generated service module for wafv2

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for wafv2
pub struct Wafv2Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Wafv2Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get rule_group resource handler
    pub fn rule_group(&self) -> resources::Rule_group<'_> {
        resources::Rule_group::new(self.provider)
    }
    /// Get managed_products_by_vendor resource handler
    pub fn managed_products_by_vendor(&self) -> resources::Managed_products_by_vendor<'_> {
        resources::Managed_products_by_vendor::new(self.provider)
    }
    /// Get rate_based_statement_managed_keys resource handler
    pub fn rate_based_statement_managed_keys(&self) -> resources::Rate_based_statement_managed_keys<'_> {
        resources::Rate_based_statement_managed_keys::new(self.provider)
    }
    /// Get web_aclfor_resource resource handler
    pub fn web_aclfor_resource(&self) -> resources::Web_aclfor_resource<'_> {
        resources::Web_aclfor_resource::new(self.provider)
    }
    /// Get apikey resource handler
    pub fn apikey(&self) -> resources::Apikey<'_> {
        resources::Apikey::new(self.provider)
    }
    /// Get web_acl resource handler
    pub fn web_acl(&self) -> resources::Web_acl<'_> {
        resources::Web_acl::new(self.provider)
    }
    /// Get logging_configuration resource handler
    pub fn logging_configuration(&self) -> resources::Logging_configuration<'_> {
        resources::Logging_configuration::new(self.provider)
    }
    /// Get managed_rule_set resource handler
    pub fn managed_rule_set(&self) -> resources::Managed_rule_set<'_> {
        resources::Managed_rule_set::new(self.provider)
    }
    /// Get decrypted_apikey resource handler
    pub fn decrypted_apikey(&self) -> resources::Decrypted_apikey<'_> {
        resources::Decrypted_apikey::new(self.provider)
    }
    /// Get all_managed_products resource handler
    pub fn all_managed_products(&self) -> resources::All_managed_products<'_> {
        resources::All_managed_products::new(self.provider)
    }
    /// Get managed_rule_group resource handler
    pub fn managed_rule_group(&self) -> resources::Managed_rule_group<'_> {
        resources::Managed_rule_group::new(self.provider)
    }
    /// Get firewall_manager_rule_groups resource handler
    pub fn firewall_manager_rule_groups(&self) -> resources::Firewall_manager_rule_groups<'_> {
        resources::Firewall_manager_rule_groups::new(self.provider)
    }
    /// Get ipset resource handler
    pub fn ipset(&self) -> resources::Ipset<'_> {
        resources::Ipset::new(self.provider)
    }
    /// Get managed_rule_set_versions resource handler
    pub fn managed_rule_set_versions(&self) -> resources::Managed_rule_set_versions<'_> {
        resources::Managed_rule_set_versions::new(self.provider)
    }
    /// Get managed_rule_set_version_expiry_date resource handler
    pub fn managed_rule_set_version_expiry_date(&self) -> resources::Managed_rule_set_version_expiry_date<'_> {
        resources::Managed_rule_set_version_expiry_date::new(self.provider)
    }
    /// Get sampled_requests resource handler
    pub fn sampled_requests(&self) -> resources::Sampled_requests<'_> {
        resources::Sampled_requests::new(self.provider)
    }
    /// Get permission_policy resource handler
    pub fn permission_policy(&self) -> resources::Permission_policy<'_> {
        resources::Permission_policy::new(self.provider)
    }
    /// Get mobile_sdk_release resource handler
    pub fn mobile_sdk_release(&self) -> resources::Mobile_sdk_release<'_> {
        resources::Mobile_sdk_release::new(self.provider)
    }
    /// Get regex_pattern_set resource handler
    pub fn regex_pattern_set(&self) -> resources::Regex_pattern_set<'_> {
        resources::Regex_pattern_set::new(self.provider)
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
