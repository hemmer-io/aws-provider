//! Entities_detection_v2_job resource
//!
//! EntitiesDetectionV2Job resource

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Entities_detection_v2_job resource handler
pub struct Entities_detection_v2_job<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Entities_detection_v2_job<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a entities_detection_v2_job
    ///
    /// TODO: Map `id` parameter to appropriate SDK field(s)
    #[allow(unused_variables)]
    pub async fn read(&self, id: &str) -> Result<()> {

        // Note: This is a generated skeleton.
        // TODO: Map resource ID to SDK parameters
        let _client = &self.provider.comprehendmedical_client;

        Ok(())

    }





}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_entities_detection_v2_job_operations() {
        // Test entities_detection_v2_job CRUD operations
    }
}
