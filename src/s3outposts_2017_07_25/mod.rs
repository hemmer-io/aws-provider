//! S3outposts_2017_07_25 Service
//!
//! Auto-generated service module for s3outposts_2017_07_25

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for s3outposts_2017_07_25
pub struct S3outposts_2017_07_25Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> S3outposts_2017_07_25Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get endpoint resource handler
    pub fn endpoint(&self) -> resources::Endpoint<'_> {
        resources::Endpoint::new(self.provider)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_creation() {
        // Service creation test
    }
}
