//! Grafana_2020_08_18 Service
//!
//! Auto-generated service module for grafana_2020_08_18

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for grafana_2020_08_18
pub struct Grafana_2020_08_18Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Grafana_2020_08_18Service<'a> {
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
