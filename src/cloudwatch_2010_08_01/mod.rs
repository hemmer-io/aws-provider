//! Cloudwatch_2010_08_01 Service
//!
//! Auto-generated service module for cloudwatch_2010_08_01

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for cloudwatch_2010_08_01
pub struct Cloudwatch_2010_08_01Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Cloudwatch_2010_08_01Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get insight_rules resource handler
    pub fn insight_rules(&self) -> resources::Insight_rules<'_> {
        resources::Insight_rules::new(self.provider)
    }
    /// Get alarm_contributors resource handler
    pub fn alarm_contributors(&self) -> resources::Alarm_contributors<'_> {
        resources::Alarm_contributors::new(self.provider)
    }
    /// Get dashboard resource handler
    pub fn dashboard(&self) -> resources::Dashboard<'_> {
        resources::Dashboard::new(self.provider)
    }
    /// Get metric_data resource handler
    pub fn metric_data(&self) -> resources::Metric_data<'_> {
        resources::Metric_data::new(self.provider)
    }
    /// Get alarms_for_metric resource handler
    pub fn alarms_for_metric(&self) -> resources::Alarms_for_metric<'_> {
        resources::Alarms_for_metric::new(self.provider)
    }
    /// Get metric_stream resource handler
    pub fn metric_stream(&self) -> resources::Metric_stream<'_> {
        resources::Metric_stream::new(self.provider)
    }
    /// Get alarms resource handler
    pub fn alarms(&self) -> resources::Alarms<'_> {
        resources::Alarms::new(self.provider)
    }
    /// Get metric_statistics resource handler
    pub fn metric_statistics(&self) -> resources::Metric_statistics<'_> {
        resources::Metric_statistics::new(self.provider)
    }
    /// Get composite_alarm resource handler
    pub fn composite_alarm(&self) -> resources::Composite_alarm<'_> {
        resources::Composite_alarm::new(self.provider)
    }
    /// Get insight_rule resource handler
    pub fn insight_rule(&self) -> resources::Insight_rule<'_> {
        resources::Insight_rule::new(self.provider)
    }
    /// Get metric_alarm resource handler
    pub fn metric_alarm(&self) -> resources::Metric_alarm<'_> {
        resources::Metric_alarm::new(self.provider)
    }
    /// Get dashboards resource handler
    pub fn dashboards(&self) -> resources::Dashboards<'_> {
        resources::Dashboards::new(self.provider)
    }
    /// Get anomaly_detectors resource handler
    pub fn anomaly_detectors(&self) -> resources::Anomaly_detectors<'_> {
        resources::Anomaly_detectors::new(self.provider)
    }
    /// Get insight_rule_report resource handler
    pub fn insight_rule_report(&self) -> resources::Insight_rule_report<'_> {
        resources::Insight_rule_report::new(self.provider)
    }
    /// Get anomaly_detector resource handler
    pub fn anomaly_detector(&self) -> resources::Anomaly_detector<'_> {
        resources::Anomaly_detector::new(self.provider)
    }
    /// Get metric_widget_image resource handler
    pub fn metric_widget_image(&self) -> resources::Metric_widget_image<'_> {
        resources::Metric_widget_image::new(self.provider)
    }
    /// Get managed_insight_rules resource handler
    pub fn managed_insight_rules(&self) -> resources::Managed_insight_rules<'_> {
        resources::Managed_insight_rules::new(self.provider)
    }
    /// Get alarm_history resource handler
    pub fn alarm_history(&self) -> resources::Alarm_history<'_> {
        resources::Alarm_history::new(self.provider)
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
