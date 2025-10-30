//! Forecast_2018_06_26 Service
//!
//! Auto-generated service module for forecast_2018_06_26

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for forecast_2018_06_26
pub struct Forecast_2018_06_26Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Forecast_2018_06_26Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get dataset_group resource handler
    pub fn dataset_group(&self) -> resources::Dataset_group<'_> {
        resources::Dataset_group::new(self.provider)
    }
    /// Get forecast_export_job resource handler
    pub fn forecast_export_job(&self) -> resources::Forecast_export_job<'_> {
        resources::Forecast_export_job::new(self.provider)
    }
    /// Get explainability resource handler
    pub fn explainability(&self) -> resources::Explainability<'_> {
        resources::Explainability::new(self.provider)
    }
    /// Get forecast resource handler
    pub fn forecast(&self) -> resources::Forecast<'_> {
        resources::Forecast::new(self.provider)
    }
    /// Get dataset resource handler
    pub fn dataset(&self) -> resources::Dataset<'_> {
        resources::Dataset::new(self.provider)
    }
    /// Get what_if_analysis resource handler
    pub fn what_if_analysis(&self) -> resources::What_if_analysis<'_> {
        resources::What_if_analysis::new(self.provider)
    }
    /// Get predictor resource handler
    pub fn predictor(&self) -> resources::Predictor<'_> {
        resources::Predictor::new(self.provider)
    }
    /// Get monitor resource handler
    pub fn monitor(&self) -> resources::Monitor<'_> {
        resources::Monitor::new(self.provider)
    }
    /// Get what_if_forecast resource handler
    pub fn what_if_forecast(&self) -> resources::What_if_forecast<'_> {
        resources::What_if_forecast::new(self.provider)
    }
    /// Get accuracy_metrics resource handler
    pub fn accuracy_metrics(&self) -> resources::Accuracy_metrics<'_> {
        resources::Accuracy_metrics::new(self.provider)
    }
    /// Get dataset_import_job resource handler
    pub fn dataset_import_job(&self) -> resources::Dataset_import_job<'_> {
        resources::Dataset_import_job::new(self.provider)
    }
    /// Get predictor_backtest_export_job resource handler
    pub fn predictor_backtest_export_job(&self) -> resources::Predictor_backtest_export_job<'_> {
        resources::Predictor_backtest_export_job::new(self.provider)
    }
    /// Get what_if_forecast_export resource handler
    pub fn what_if_forecast_export(&self) -> resources::What_if_forecast_export<'_> {
        resources::What_if_forecast_export::new(self.provider)
    }
    /// Get auto_predictor resource handler
    pub fn auto_predictor(&self) -> resources::Auto_predictor<'_> {
        resources::Auto_predictor::new(self.provider)
    }
    /// Get resource_tree resource handler
    pub fn resource_tree(&self) -> resources::Resource_tree<'_> {
        resources::Resource_tree::new(self.provider)
    }
    /// Get explainability_export resource handler
    pub fn explainability_export(&self) -> resources::Explainability_export<'_> {
        resources::Explainability_export::new(self.provider)
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
