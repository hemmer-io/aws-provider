//! Cost Service
//!
//! Auto-generated service module for cost

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for cost
pub struct CostService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> CostService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get savings_plans_utilization resource handler
    pub fn savings_plans_utilization(&self) -> resources::Savings_plans_utilization<'_> {
        resources::Savings_plans_utilization::new(self.provider)
    }
    /// Get reservation_utilization resource handler
    pub fn reservation_utilization(&self) -> resources::Reservation_utilization<'_> {
        resources::Reservation_utilization::new(self.provider)
    }
    /// Get anomaly_subscription resource handler
    pub fn anomaly_subscription(&self) -> resources::Anomaly_subscription<'_> {
        resources::Anomaly_subscription::new(self.provider)
    }
    /// Get savings_plan_purchase_recommendation_details resource handler
    pub fn savings_plan_purchase_recommendation_details(&self) -> resources::Savings_plan_purchase_recommendation_details<'_> {
        resources::Savings_plan_purchase_recommendation_details::new(self.provider)
    }
    /// Get cost_categories resource handler
    pub fn cost_categories(&self) -> resources::Cost_categories<'_> {
        resources::Cost_categories::new(self.provider)
    }
    /// Get cost_and_usage_with_resources resource handler
    pub fn cost_and_usage_with_resources(&self) -> resources::Cost_and_usage_with_resources<'_> {
        resources::Cost_and_usage_with_resources::new(self.provider)
    }
    /// Get savings_plans_purchase_recommendation resource handler
    pub fn savings_plans_purchase_recommendation(&self) -> resources::Savings_plans_purchase_recommendation<'_> {
        resources::Savings_plans_purchase_recommendation::new(self.provider)
    }
    /// Get commitment_purchase_analysis resource handler
    pub fn commitment_purchase_analysis(&self) -> resources::Commitment_purchase_analysis<'_> {
        resources::Commitment_purchase_analysis::new(self.provider)
    }
    /// Get approximate_usage_records resource handler
    pub fn approximate_usage_records(&self) -> resources::Approximate_usage_records<'_> {
        resources::Approximate_usage_records::new(self.provider)
    }
    /// Get anomaly_monitors resource handler
    pub fn anomaly_monitors(&self) -> resources::Anomaly_monitors<'_> {
        resources::Anomaly_monitors::new(self.provider)
    }
    /// Get cost_forecast resource handler
    pub fn cost_forecast(&self) -> resources::Cost_forecast<'_> {
        resources::Cost_forecast::new(self.provider)
    }
    /// Get savings_plans_coverage resource handler
    pub fn savings_plans_coverage(&self) -> resources::Savings_plans_coverage<'_> {
        resources::Savings_plans_coverage::new(self.provider)
    }
    /// Get dimension_values resource handler
    pub fn dimension_values(&self) -> resources::Dimension_values<'_> {
        resources::Dimension_values::new(self.provider)
    }
    /// Get tags resource handler
    pub fn tags(&self) -> resources::Tags<'_> {
        resources::Tags::new(self.provider)
    }
    /// Get reservation_coverage resource handler
    pub fn reservation_coverage(&self) -> resources::Reservation_coverage<'_> {
        resources::Reservation_coverage::new(self.provider)
    }
    /// Get savings_plans_utilization_details resource handler
    pub fn savings_plans_utilization_details(&self) -> resources::Savings_plans_utilization_details<'_> {
        resources::Savings_plans_utilization_details::new(self.provider)
    }
    /// Get reservation_purchase_recommendation resource handler
    pub fn reservation_purchase_recommendation(&self) -> resources::Reservation_purchase_recommendation<'_> {
        resources::Reservation_purchase_recommendation::new(self.provider)
    }
    /// Get cost_and_usage_comparisons resource handler
    pub fn cost_and_usage_comparisons(&self) -> resources::Cost_and_usage_comparisons<'_> {
        resources::Cost_and_usage_comparisons::new(self.provider)
    }
    /// Get anomaly_monitor resource handler
    pub fn anomaly_monitor(&self) -> resources::Anomaly_monitor<'_> {
        resources::Anomaly_monitor::new(self.provider)
    }
    /// Get cost_category_definition resource handler
    pub fn cost_category_definition(&self) -> resources::Cost_category_definition<'_> {
        resources::Cost_category_definition::new(self.provider)
    }
    /// Get rightsizing_recommendation resource handler
    pub fn rightsizing_recommendation(&self) -> resources::Rightsizing_recommendation<'_> {
        resources::Rightsizing_recommendation::new(self.provider)
    }
    /// Get cost_comparison_drivers resource handler
    pub fn cost_comparison_drivers(&self) -> resources::Cost_comparison_drivers<'_> {
        resources::Cost_comparison_drivers::new(self.provider)
    }
    /// Get cost_allocation_tags_status resource handler
    pub fn cost_allocation_tags_status(&self) -> resources::Cost_allocation_tags_status<'_> {
        resources::Cost_allocation_tags_status::new(self.provider)
    }
    /// Get anomalies resource handler
    pub fn anomalies(&self) -> resources::Anomalies<'_> {
        resources::Anomalies::new(self.provider)
    }
    /// Get cost_and_usage resource handler
    pub fn cost_and_usage(&self) -> resources::Cost_and_usage<'_> {
        resources::Cost_and_usage::new(self.provider)
    }
    /// Get anomaly_subscriptions resource handler
    pub fn anomaly_subscriptions(&self) -> resources::Anomaly_subscriptions<'_> {
        resources::Anomaly_subscriptions::new(self.provider)
    }
    /// Get usage_forecast resource handler
    pub fn usage_forecast(&self) -> resources::Usage_forecast<'_> {
        resources::Usage_forecast::new(self.provider)
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
