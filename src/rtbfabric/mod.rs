//! Rtbfabric Service
//!
//! Auto-generated service module for rtbfabric

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for rtbfabric
pub struct RtbfabricService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> RtbfabricService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
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
