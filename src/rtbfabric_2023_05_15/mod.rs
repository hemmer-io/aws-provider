//! Rtbfabric_2023_05_15 Service
//!
//! Auto-generated service module for rtbfabric_2023_05_15

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for rtbfabric_2023_05_15
pub struct Rtbfabric_2023_05_15Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Rtbfabric_2023_05_15Service<'a> {
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
