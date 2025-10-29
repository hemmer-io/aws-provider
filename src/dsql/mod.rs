//! Dsql Service
//!
//! Auto-generated service module for dsql

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for dsql
pub struct DsqlService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> DsqlService<'a> {
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
