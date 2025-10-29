//! S3outposts Service
//!
//! Auto-generated service module for s3outposts

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for s3outposts
pub struct S3outpostsService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> S3outpostsService<'a> {
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
