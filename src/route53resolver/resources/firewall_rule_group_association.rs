//! Firewall_rule_group_association resource
//!
//! FirewallRuleGroupAssociation resource

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Firewall_rule_group_association resource handler
pub struct Firewall_rule_group_association<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Firewall_rule_group_association<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a firewall_rule_group_association
    ///
    /// TODO: Map `id` parameter to appropriate SDK field(s)
    #[allow(unused_variables)]
    pub async fn read(&self, id: &str) -> Result<()> {

        // Note: This is a generated skeleton.
        // TODO: Map resource ID to SDK parameters
        let _client = &self.provider.route53resolver_client;

        Ok(())

    }



    /// Update a firewall_rule_group_association
    ///
    /// TODO: Map `id` and update fields to appropriate SDK parameters
    #[allow(unused_variables)]
    pub async fn update(&self, id: &str, firewall_rule_group_association_id: Option<String>, mutation_protection: Option<String>, name: Option<String>, priority: Option<i64>) -> Result<()> {

        // Note: This is a generated skeleton.
        // TODO: Map resource ID and update fields to SDK parameters
        let _client = &self.provider.route53resolver_client;

        Ok(())

    }



}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_firewall_rule_group_association_operations() {
        // Test firewall_rule_group_association CRUD operations
    }
}
