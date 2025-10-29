//! Networkflowmonitor Service
//!
//! Auto-generated service module for networkflowmonitor

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for networkflowmonitor
pub struct NetworkflowmonitorService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> NetworkflowmonitorService<'a> {
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
