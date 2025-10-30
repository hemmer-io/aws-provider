//! Security_group_rule_descriptions_ingress resource
//!
//! SecurityGroupRuleDescriptionsIngress resource

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Security_group_rule_descriptions_ingress resource handler
pub struct Security_group_rule_descriptions_ingress<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Security_group_rule_descriptions_ingress<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }






    /// Update a security_group_rule_descriptions_ingress
    ///
    /// TODO: Map `id` and update fields to appropriate SDK parameters
    #[allow(unused_variables)]
    pub async fn update(&self, id: &str, group_name: Option<String>, dry_run: Option<bool>, group_id: Option<String>, ip_permissions: Option<Vec<String>>, security_group_rule_descriptions: Option<Vec<String>>) -> Result<()> {

        // Note: This is a generated skeleton.
        // TODO: Map resource ID and update fields to SDK parameters
        let _client = &self.provider.ec2_2016_11_15_client;

        Ok(())

    }



}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_security_group_rule_descriptions_ingress_operations() {
        // Test security_group_rule_descriptions_ingress CRUD operations
    }
}
