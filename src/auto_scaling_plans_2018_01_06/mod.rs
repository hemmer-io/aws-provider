//! Auto_scaling_plans_2018_01_06 Service
//!
//! Auto-generated service module for auto_scaling_plans_2018_01_06

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for auto_scaling_plans_2018_01_06
pub struct Auto_scaling_plans_2018_01_06Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Auto_scaling_plans_2018_01_06Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get scaling_plan resource handler
    pub fn scaling_plan(&self) -> resources::Scaling_plan<'_> {
        resources::Scaling_plan::new(self.provider)
    }
    /// Get scaling_plan_resources resource handler
    pub fn scaling_plan_resources(&self) -> resources::Scaling_plan_resources<'_> {
        resources::Scaling_plan_resources::new(self.provider)
    }
    /// Get scaling_plans resource handler
    pub fn scaling_plans(&self) -> resources::Scaling_plans<'_> {
        resources::Scaling_plans::new(self.provider)
    }
    /// Get scaling_plan_resource_forecast_data resource handler
    pub fn scaling_plan_resource_forecast_data(&self) -> resources::Scaling_plan_resource_forecast_data<'_> {
        resources::Scaling_plan_resource_forecast_data::new(self.provider)
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
