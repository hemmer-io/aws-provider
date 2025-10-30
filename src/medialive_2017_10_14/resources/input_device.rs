//! Input_device resource
//!
//! InputDevice resource

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Input_device resource handler
pub struct Input_device<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Input_device<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a input_device
    ///
    /// TODO: Map `id` parameter to appropriate SDK field(s)
    #[allow(unused_variables)]
    pub async fn read(&self, id: &str) -> Result<()> {

        // Note: This is a generated skeleton.
        // TODO: Map resource ID to SDK parameters
        let _client = &self.provider.medialive_2017_10_14_client;

        Ok(())

    }



    /// Update a input_device
    ///
    /// TODO: Map `id` and update fields to appropriate SDK parameters
    #[allow(unused_variables)]
    pub async fn update(&self, id: &str, availability_zone: Option<String>, hd_device_settings: Option<String>, input_device_id: Option<String>, name: Option<String>, uhd_device_settings: Option<String>) -> Result<()> {

        // Note: This is a generated skeleton.
        // TODO: Map resource ID and update fields to SDK parameters
        let _client = &self.provider.medialive_2017_10_14_client;

        Ok(())

    }



}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_input_device_operations() {
        // Test input_device CRUD operations
    }
}
