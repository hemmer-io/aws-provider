//! Ip_restriction resource
//!
//! IpRestriction resource

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Ip_restriction resource handler
pub struct Ip_restriction<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Ip_restriction<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a ip_restriction
    ///
    /// TODO: Map `id` parameter to appropriate SDK field(s)
    #[allow(unused_variables)]
    pub async fn read(&self, id: &str) -> Result<()> {

        // Note: This is a generated skeleton.
        // TODO: Map resource ID to SDK parameters
        let _client = &self.provider.quicksight_client;

        Ok(())

    }



    /// Update a ip_restriction
    ///
    /// TODO: Map `id` and update fields to appropriate SDK parameters
    #[allow(unused_variables)]
    pub async fn update(&self, id: &str, vpc_endpoint_id_restriction_rule_map: Option<HashMap<String, String>>, aws_account_id: Option<String>, vpc_id_restriction_rule_map: Option<HashMap<String, String>>, enabled: Option<bool>, ip_restriction_rule_map: Option<HashMap<String, String>>) -> Result<()> {

        // Note: This is a generated skeleton.
        // TODO: Map resource ID and update fields to SDK parameters
        let _client = &self.provider.quicksight_client;

        Ok(())

    }



}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ip_restriction_operations() {
        // Test ip_restriction CRUD operations
    }
}
