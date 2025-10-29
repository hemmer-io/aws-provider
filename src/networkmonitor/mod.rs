//! Networkmonitor Service
//!
//! Auto-generated service module for networkmonitor

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for networkmonitor
pub struct NetworkmonitorService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> NetworkmonitorService<'a> {
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
