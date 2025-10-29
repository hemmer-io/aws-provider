//! Backupsearch Service
//!
//! Auto-generated service module for backupsearch

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for backupsearch
pub struct BackupsearchService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> BackupsearchService<'a> {
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
