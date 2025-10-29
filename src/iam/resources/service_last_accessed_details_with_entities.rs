//! Service_last_accessed_details_with_entities resource
//!
//! ServiceLastAccessedDetailsWithEntities resource

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Service_last_accessed_details_with_entities resource handler
pub struct Service_last_accessed_details_with_entities<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Service_last_accessed_details_with_entities<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a service_last_accessed_details_with_entities
    ///
    /// TODO: Map `id` parameter to appropriate SDK field(s)
    #[allow(unused_variables)]
    pub async fn read(&self, id: &str) -> Result<()> {

        // Note: This is a generated skeleton.
        // TODO: Map resource ID to SDK parameters
        let _client = &self.provider.iam_client;

        Ok(())

    }





}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_service_last_accessed_details_with_entities_operations() {
        // Test service_last_accessed_details_with_entities CRUD operations
    }
}
