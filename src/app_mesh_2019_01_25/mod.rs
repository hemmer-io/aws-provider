//! App_mesh_2019_01_25 Service
//!
//! Auto-generated service module for app_mesh_2019_01_25

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for app_mesh_2019_01_25
pub struct App_mesh_2019_01_25Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> App_mesh_2019_01_25Service<'a> {
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
