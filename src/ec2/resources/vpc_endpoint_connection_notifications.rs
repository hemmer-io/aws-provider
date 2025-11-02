//! Vpc_endpoint_connection_notifications resource
//!
//! VpcEndpointConnectionNotifications resource

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Vpc_endpoint_connection_notifications resource handler
pub struct Vpc_endpoint_connection_notifications<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Vpc_endpoint_connection_notifications<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a vpc_endpoint_connection_notifications
    ///
    /// TODO: Map `id` parameter to appropriate SDK field(s)
    #[allow(unused_variables)]
    pub async fn read(&self, id: &str) -> Result<()> {

        // Note: This is a generated skeleton.
        // TODO: Map resource ID to SDK parameters
        let _client = &self.provider.ec2_client;

        Ok(())

    }





    /// Delete a vpc_endpoint_connection_notifications
    ///
    /// TODO: Map `id` parameter to appropriate SDK field(s)
    #[allow(unused_variables)]
    pub async fn delete(&self, id: &str) -> Result<()> {

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
    async fn test_vpc_endpoint_connection_notifications_operations() {
        // Test vpc_endpoint_connection_notifications CRUD operations
    }
}
