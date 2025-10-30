//! Ebs_2019_11_02 Service
//!
//! Auto-generated service module for ebs_2019_11_02

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for ebs_2019_11_02
pub struct Ebs_2019_11_02Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Ebs_2019_11_02Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get snapshot_block resource handler
    pub fn snapshot_block(&self) -> resources::Snapshot_block<'_> {
        resources::Snapshot_block::new(self.provider)
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
