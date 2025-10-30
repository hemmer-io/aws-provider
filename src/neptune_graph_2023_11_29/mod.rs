//! Neptune_graph_2023_11_29 Service
//!
//! Auto-generated service module for neptune_graph_2023_11_29

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for neptune_graph_2023_11_29
pub struct Neptune_graph_2023_11_29Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Neptune_graph_2023_11_29Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get graph_summary resource handler
    pub fn graph_summary(&self) -> resources::Graph_summary<'_> {
        resources::Graph_summary::new(self.provider)
    }
    /// Get query resource handler
    pub fn query(&self) -> resources::Query<'_> {
        resources::Query::new(self.provider)
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
