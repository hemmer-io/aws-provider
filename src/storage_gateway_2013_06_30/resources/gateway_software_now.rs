//! Gateway_software_now resource
//!
//! GatewaySoftwareNow resource

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Gateway_software_now resource handler
pub struct Gateway_software_now<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Gateway_software_now<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }






    /// Update a gateway_software_now
    ///
    /// TODO: Map `id` and update fields to appropriate SDK parameters
    #[allow(unused_variables)]
    pub async fn update(&self, id: &str, gateway_arn: Option<String>) -> Result<()> {

        // Note: This is a generated skeleton.
        // TODO: Map resource ID and update fields to SDK parameters
        let _client = &self.provider.storage_gateway_2013_06_30_client;

        Ok(())

    }



}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_gateway_software_now_operations() {
        // Test gateway_software_now CRUD operations
    }
}
