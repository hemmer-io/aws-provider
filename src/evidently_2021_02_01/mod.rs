//! Evidently_2021_02_01 Service
//!
//! Auto-generated service module for evidently_2021_02_01

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for evidently_2021_02_01
pub struct Evidently_2021_02_01Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Evidently_2021_02_01Service<'a> {
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
