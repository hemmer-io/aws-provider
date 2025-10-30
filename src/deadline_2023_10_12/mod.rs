//! Deadline_2023_10_12 Service
//!
//! Auto-generated service module for deadline_2023_10_12

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for deadline_2023_10_12
pub struct Deadline_2023_10_12Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Deadline_2023_10_12Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get queue_fleet_association resource handler
    pub fn queue_fleet_association(&self) -> resources::Queue_fleet_association<'_> {
        resources::Queue_fleet_association::new(self.provider)
    }
    /// Get queue_limit_association resource handler
    pub fn queue_limit_association(&self) -> resources::Queue_limit_association<'_> {
        resources::Queue_limit_association::new(self.provider)
    }
    /// Get sessions_statistics_aggregation resource handler
    pub fn sessions_statistics_aggregation(&self) -> resources::Sessions_statistics_aggregation<'_> {
        resources::Sessions_statistics_aggregation::new(self.provider)
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
