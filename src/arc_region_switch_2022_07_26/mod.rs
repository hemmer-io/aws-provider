//! Arc_region_switch_2022_07_26 Service
//!
//! Auto-generated service module for arc_region_switch_2022_07_26

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for arc_region_switch_2022_07_26
pub struct Arc_region_switch_2022_07_26Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Arc_region_switch_2022_07_26Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get plan_execution resource handler
    pub fn plan_execution(&self) -> resources::Plan_execution<'_> {
        resources::Plan_execution::new(self.provider)
    }
    /// Get plan_evaluation_status resource handler
    pub fn plan_evaluation_status(&self) -> resources::Plan_evaluation_status<'_> {
        resources::Plan_evaluation_status::new(self.provider)
    }
    /// Get plan_in_region resource handler
    pub fn plan_in_region(&self) -> resources::Plan_in_region<'_> {
        resources::Plan_in_region::new(self.provider)
    }
    /// Get plan_execution_step resource handler
    pub fn plan_execution_step(&self) -> resources::Plan_execution_step<'_> {
        resources::Plan_execution_step::new(self.provider)
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
