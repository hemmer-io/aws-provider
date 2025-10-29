//! Event_configuration_by_resource_types resource
//!
//! EventConfigurationByResourceTypes resource

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Event_configuration_by_resource_types resource handler
pub struct Event_configuration_by_resource_types<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Event_configuration_by_resource_types<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a event_configuration_by_resource_types
    ///
    /// TODO: Map `id` parameter to appropriate SDK field(s)
    #[allow(unused_variables)]
    pub async fn read(&self, id: &str) -> Result<()> {

        // Note: This is a generated skeleton.
        // TODO: Map resource ID to SDK parameters
        let _client = &self.provider.iot_client;

        Ok(())

    }



    /// Update a event_configuration_by_resource_types
    ///
    /// TODO: Map `id` and update fields to appropriate SDK parameters
    #[allow(unused_variables)]
    pub async fn update(&self, id: &str, proximity: Option<String>, connection_status: Option<String>, join: Option<String>, message_delivery_status: Option<String>, device_registration_state: Option<String>) -> Result<()> {

        // Note: This is a generated skeleton.
        // TODO: Map resource ID and update fields to SDK parameters
        let _client = &self.provider.iot_client;

        Ok(())

    }



}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_event_configuration_by_resource_types_operations() {
        // Test event_configuration_by_resource_types CRUD operations
    }
}
