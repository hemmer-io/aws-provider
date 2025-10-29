//! Mgn Service
//!
//! Auto-generated service module for mgn

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for mgn
pub struct MgnService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> MgnService<'a> {
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
