//! Identity_propagation_config resource
//!
//! IdentityPropagationConfig resource

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Identity_propagation_config resource handler
pub struct Identity_propagation_config<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Identity_propagation_config<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }






    /// Update a identity_propagation_config
    ///
    /// TODO: Map `id` and update fields to appropriate SDK parameters
    #[allow(unused_variables)]
    pub async fn update(&self, id: &str, service: Option<String>, aws_account_id: Option<String>, authorized_targets: Option<Vec<String>>) -> Result<()> {

        // Note: This is a generated skeleton.
        // TODO: Map resource ID and update fields to SDK parameters
        let _client = &self.provider.quicksight_2018_04_01_client;

        Ok(())

    }



    /// Delete a identity_propagation_config
    ///
    /// TODO: Map `id` parameter to appropriate SDK field(s)
    #[allow(unused_variables)]
    pub async fn delete(&self, id: &str) -> Result<()> {

        // Note: This is a generated skeleton.
        // TODO: Map resource ID to SDK parameters
        let _client = &self.provider.quicksight_2018_04_01_client;

        Ok(())

    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_identity_propagation_config_operations() {
        // Test identity_propagation_config CRUD operations
    }
}
