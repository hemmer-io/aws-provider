//! Application_with_token_exchange_grant resource
//!
//! ApplicationWithTokenExchangeGrant resource

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Application_with_token_exchange_grant resource handler
pub struct Application_with_token_exchange_grant<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Application_with_token_exchange_grant<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }






    /// Update a application_with_token_exchange_grant
    ///
    /// TODO: Map `id` and update fields to appropriate SDK parameters
    #[allow(unused_variables)]
    pub async fn update(&self, id: &str, namespace: Option<String>, aws_account_id: Option<String>) -> Result<()> {

        // Note: This is a generated skeleton.
        // TODO: Map resource ID and update fields to SDK parameters
        let _client = &self.provider.quicksight_2018_04_01_client;

        Ok(())

    }



}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_application_with_token_exchange_grant_operations() {
        // Test application_with_token_exchange_grant CRUD operations
    }
}
