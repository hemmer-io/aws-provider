//! Public_sharing_settings resource
//!
//! PublicSharingSettings resource

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Public_sharing_settings resource handler
pub struct Public_sharing_settings<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Public_sharing_settings<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }






    /// Update a public_sharing_settings
    ///
    /// TODO: Map `id` and update fields to appropriate SDK parameters
    #[allow(unused_variables)]
    pub async fn update(&self, id: &str, public_sharing_enabled: Option<bool>, aws_account_id: Option<String>) -> Result<()> {

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
    async fn test_public_sharing_settings_operations() {
        // Test public_sharing_settings CRUD operations
    }
}
