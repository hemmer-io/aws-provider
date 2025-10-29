//! Route53resolver Service
//!
//! Auto-generated service module for route53resolver

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for route53resolver
pub struct Route53resolverService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Route53resolverService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get firewall_config resource handler
    pub fn firewall_config(&self) -> resources::Firewall_config<'_> {
        resources::Firewall_config::new(self.provider)
    }
    /// Get firewall_rule_group resource handler
    pub fn firewall_rule_group(&self) -> resources::Firewall_rule_group<'_> {
        resources::Firewall_rule_group::new(self.provider)
    }
    /// Get resolver_query_log_config_policy resource handler
    pub fn resolver_query_log_config_policy(&self) -> resources::Resolver_query_log_config_policy<'_> {
        resources::Resolver_query_log_config_policy::new(self.provider)
    }
    /// Get resolver_rule_policy resource handler
    pub fn resolver_rule_policy(&self) -> resources::Resolver_rule_policy<'_> {
        resources::Resolver_rule_policy::new(self.provider)
    }
    /// Get firewall_domain_list resource handler
    pub fn firewall_domain_list(&self) -> resources::Firewall_domain_list<'_> {
        resources::Firewall_domain_list::new(self.provider)
    }
    /// Get resolver_query_log_config_association resource handler
    pub fn resolver_query_log_config_association(&self) -> resources::Resolver_query_log_config_association<'_> {
        resources::Resolver_query_log_config_association::new(self.provider)
    }
    /// Get resolver_endpoint resource handler
    pub fn resolver_endpoint(&self) -> resources::Resolver_endpoint<'_> {
        resources::Resolver_endpoint::new(self.provider)
    }
    /// Get resolver_rule resource handler
    pub fn resolver_rule(&self) -> resources::Resolver_rule<'_> {
        resources::Resolver_rule::new(self.provider)
    }
    /// Get outpost_resolver resource handler
    pub fn outpost_resolver(&self) -> resources::Outpost_resolver<'_> {
        resources::Outpost_resolver::new(self.provider)
    }
    /// Get firewall_domains resource handler
    pub fn firewall_domains(&self) -> resources::Firewall_domains<'_> {
        resources::Firewall_domains::new(self.provider)
    }
    /// Get resolver_dnssec_config resource handler
    pub fn resolver_dnssec_config(&self) -> resources::Resolver_dnssec_config<'_> {
        resources::Resolver_dnssec_config::new(self.provider)
    }
    /// Get firewall_rule resource handler
    pub fn firewall_rule(&self) -> resources::Firewall_rule<'_> {
        resources::Firewall_rule::new(self.provider)
    }
    /// Get resolver_query_log_config resource handler
    pub fn resolver_query_log_config(&self) -> resources::Resolver_query_log_config<'_> {
        resources::Resolver_query_log_config::new(self.provider)
    }
    /// Get firewall_rule_group_policy resource handler
    pub fn firewall_rule_group_policy(&self) -> resources::Firewall_rule_group_policy<'_> {
        resources::Firewall_rule_group_policy::new(self.provider)
    }
    /// Get firewall_rule_group_association resource handler
    pub fn firewall_rule_group_association(&self) -> resources::Firewall_rule_group_association<'_> {
        resources::Firewall_rule_group_association::new(self.provider)
    }
    /// Get resolver_config resource handler
    pub fn resolver_config(&self) -> resources::Resolver_config<'_> {
        resources::Resolver_config::new(self.provider)
    }
    /// Get resolver_rule_association resource handler
    pub fn resolver_rule_association(&self) -> resources::Resolver_rule_association<'_> {
        resources::Resolver_rule_association::new(self.provider)
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
