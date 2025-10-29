//! Waf Service
//!
//! Auto-generated service module for waf

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for waf
pub struct WafService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> WafService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get rule resource handler
    pub fn rule(&self) -> resources::Rule<'_> {
        resources::Rule::new(self.provider)
    }
    /// Get size_constraint_set resource handler
    pub fn size_constraint_set(&self) -> resources::Size_constraint_set<'_> {
        resources::Size_constraint_set::new(self.provider)
    }
    /// Get change_token_status resource handler
    pub fn change_token_status(&self) -> resources::Change_token_status<'_> {
        resources::Change_token_status::new(self.provider)
    }
    /// Get change_token resource handler
    pub fn change_token(&self) -> resources::Change_token<'_> {
        resources::Change_token::new(self.provider)
    }
    /// Get permission_policy resource handler
    pub fn permission_policy(&self) -> resources::Permission_policy<'_> {
        resources::Permission_policy::new(self.provider)
    }
    /// Get geo_match_set resource handler
    pub fn geo_match_set(&self) -> resources::Geo_match_set<'_> {
        resources::Geo_match_set::new(self.provider)
    }
    /// Get regex_match_set resource handler
    pub fn regex_match_set(&self) -> resources::Regex_match_set<'_> {
        resources::Regex_match_set::new(self.provider)
    }
    /// Get sql_injection_match_set resource handler
    pub fn sql_injection_match_set(&self) -> resources::Sql_injection_match_set<'_> {
        resources::Sql_injection_match_set::new(self.provider)
    }
    /// Get sampled_requests resource handler
    pub fn sampled_requests(&self) -> resources::Sampled_requests<'_> {
        resources::Sampled_requests::new(self.provider)
    }
    /// Get byte_match_set resource handler
    pub fn byte_match_set(&self) -> resources::Byte_match_set<'_> {
        resources::Byte_match_set::new(self.provider)
    }
    /// Get xss_match_set resource handler
    pub fn xss_match_set(&self) -> resources::Xss_match_set<'_> {
        resources::Xss_match_set::new(self.provider)
    }
    /// Get web_acl resource handler
    pub fn web_acl(&self) -> resources::Web_acl<'_> {
        resources::Web_acl::new(self.provider)
    }
    /// Get ipset resource handler
    pub fn ipset(&self) -> resources::Ipset<'_> {
        resources::Ipset::new(self.provider)
    }
    /// Get rule_group resource handler
    pub fn rule_group(&self) -> resources::Rule_group<'_> {
        resources::Rule_group::new(self.provider)
    }
    /// Get web_aclmigration_stack resource handler
    pub fn web_aclmigration_stack(&self) -> resources::Web_aclmigration_stack<'_> {
        resources::Web_aclmigration_stack::new(self.provider)
    }
    /// Get rate_based_rule resource handler
    pub fn rate_based_rule(&self) -> resources::Rate_based_rule<'_> {
        resources::Rate_based_rule::new(self.provider)
    }
    /// Get logging_configuration resource handler
    pub fn logging_configuration(&self) -> resources::Logging_configuration<'_> {
        resources::Logging_configuration::new(self.provider)
    }
    /// Get rate_based_rule_managed_keys resource handler
    pub fn rate_based_rule_managed_keys(&self) -> resources::Rate_based_rule_managed_keys<'_> {
        resources::Rate_based_rule_managed_keys::new(self.provider)
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
