//! Local_gateways resource
//!
//! LocalGateways resource

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Local_gateways resource handler
pub struct Local_gateways<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Local_gateways<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a local_gateways
    ///
    /// TODO: Map `id` parameter to appropriate SDK field(s)
    #[allow(unused_variables)]
    pub async fn read(&self, id: &str) -> Result<()> {

        // Note: This is a generated skeleton.
        // TODO: Map resource ID to SDK parameters
        let _client = &self.provider.ec2_client;

        Ok(())

    }





}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_local_gateways_operations() {
        // Test local_gateways CRUD operations
    }
}
