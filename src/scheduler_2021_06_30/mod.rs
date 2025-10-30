//! Scheduler_2021_06_30 Service
//!
//! Auto-generated service module for scheduler_2021_06_30

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for scheduler_2021_06_30
pub struct Scheduler_2021_06_30Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Scheduler_2021_06_30Service<'a> {
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
