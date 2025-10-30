//! Member_detectors resource
//!
//! MemberDetectors resource

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Member_detectors resource handler
pub struct Member_detectors<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Member_detectors<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a member_detectors
    ///
    /// TODO: Map `id` parameter to appropriate SDK field(s)
    #[allow(unused_variables)]
    pub async fn read(&self, id: &str) -> Result<()> {

        // Note: This is a generated skeleton.
        // TODO: Map resource ID to SDK parameters
        let _client = &self.provider.guardduty_2017_11_28_client;

        Ok(())

    }



    /// Update a member_detectors
    ///
    /// TODO: Map `id` and update fields to appropriate SDK parameters
    #[allow(unused_variables)]
    pub async fn update(&self, id: &str, data_sources: Option<String>, detector_id: Option<String>, features: Option<Vec<String>>, account_ids: Option<Vec<String>>) -> Result<()> {

        // Note: This is a generated skeleton.
        // TODO: Map resource ID and update fields to SDK parameters
        let _client = &self.provider.guardduty_2017_11_28_client;

        Ok(())

    }



}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_member_detectors_operations() {
        // Test member_detectors CRUD operations
    }
}
