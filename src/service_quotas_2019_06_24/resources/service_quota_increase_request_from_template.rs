//! Service_quota_increase_request_from_template resource
//!
//! ServiceQuotaIncreaseRequestFromTemplate resource

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Service_quota_increase_request_from_template resource handler
pub struct Service_quota_increase_request_from_template<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Service_quota_increase_request_from_template<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a service_quota_increase_request_from_template
    ///
    /// TODO: Map `id` parameter to appropriate SDK field(s)
    #[allow(unused_variables)]
    pub async fn read(&self, id: &str) -> Result<()> {

        // Note: This is a generated skeleton.
        // TODO: Map resource ID to SDK parameters
        let _client = &self.provider.service_quotas_2019_06_24_client;

        Ok(())

    }





    /// Delete a service_quota_increase_request_from_template
    ///
    /// TODO: Map `id` parameter to appropriate SDK field(s)
    #[allow(unused_variables)]
    pub async fn delete(&self, id: &str) -> Result<()> {

        // Note: This is a generated skeleton.
        // TODO: Map resource ID to SDK parameters
        let _client = &self.provider.service_quotas_2019_06_24_client;

        Ok(())

    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_service_quota_increase_request_from_template_operations() {
        // Test service_quota_increase_request_from_template CRUD operations
    }
}
