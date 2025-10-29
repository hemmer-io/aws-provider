//! Image_permissions resource
//!
//! ImagePermissions resource

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Image_permissions resource handler
pub struct Image_permissions<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Image_permissions<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a image_permissions
    ///
    /// TODO: Map `id` parameter to appropriate SDK field(s)
    #[allow(unused_variables)]
    pub async fn read(&self, id: &str) -> Result<()> {

        // Note: This is a generated skeleton.
        // TODO: Map resource ID to SDK parameters
        let _client = &self.provider.appstream_client;

        Ok(())

    }



    /// Update a image_permissions
    ///
    /// TODO: Map `id` and update fields to appropriate SDK parameters
    #[allow(unused_variables)]
    pub async fn update(&self, id: &str, name: Option<String>, shared_account_id: Option<String>, image_permissions: Option<String>) -> Result<()> {

        // Note: This is a generated skeleton.
        // TODO: Map resource ID and update fields to SDK parameters
        let _client = &self.provider.appstream_client;

        Ok(())

    }



    /// Delete a image_permissions
    ///
    /// TODO: Map `id` parameter to appropriate SDK field(s)
    #[allow(unused_variables)]
    pub async fn delete(&self, id: &str) -> Result<()> {

        // Note: This is a generated skeleton.
        // TODO: Map resource ID to SDK parameters
        let _client = &self.provider.appstream_client;

        Ok(())

    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_image_permissions_operations() {
        // Test image_permissions CRUD operations
    }
}
