//! Internetmonitor Service
//!
//! Auto-generated service module for internetmonitor

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for internetmonitor
pub struct InternetmonitorService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> InternetmonitorService<'a> {
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
