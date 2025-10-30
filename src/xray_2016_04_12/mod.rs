//! Xray_2016_04_12 Service
//!
//! Auto-generated service module for xray_2016_04_12

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for xray_2016_04_12
pub struct Xray_2016_04_12Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Xray_2016_04_12Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get indexing_rules resource handler
    pub fn indexing_rules(&self) -> resources::Indexing_rules<'_> {
        resources::Indexing_rules::new(self.provider)
    }
    /// Get group resource handler
    pub fn group(&self) -> resources::Group<'_> {
        resources::Group::new(self.provider)
    }
    /// Get trace_segment_destination resource handler
    pub fn trace_segment_destination(&self) -> resources::Trace_segment_destination<'_> {
        resources::Trace_segment_destination::new(self.provider)
    }
    /// Get time_series_service_statistics resource handler
    pub fn time_series_service_statistics(&self) -> resources::Time_series_service_statistics<'_> {
        resources::Time_series_service_statistics::new(self.provider)
    }
    /// Get sampling_rules resource handler
    pub fn sampling_rules(&self) -> resources::Sampling_rules<'_> {
        resources::Sampling_rules::new(self.provider)
    }
    /// Get trace_graph resource handler
    pub fn trace_graph(&self) -> resources::Trace_graph<'_> {
        resources::Trace_graph::new(self.provider)
    }
    /// Get trace_segments resource handler
    pub fn trace_segments(&self) -> resources::Trace_segments<'_> {
        resources::Trace_segments::new(self.provider)
    }
    /// Get retrieved_traces_graph resource handler
    pub fn retrieved_traces_graph(&self) -> resources::Retrieved_traces_graph<'_> {
        resources::Retrieved_traces_graph::new(self.provider)
    }
    /// Get groups resource handler
    pub fn groups(&self) -> resources::Groups<'_> {
        resources::Groups::new(self.provider)
    }
    /// Get sampling_statistic_summaries resource handler
    pub fn sampling_statistic_summaries(&self) -> resources::Sampling_statistic_summaries<'_> {
        resources::Sampling_statistic_summaries::new(self.provider)
    }
    /// Get sampling_rule resource handler
    pub fn sampling_rule(&self) -> resources::Sampling_rule<'_> {
        resources::Sampling_rule::new(self.provider)
    }
    /// Get insight_impact_graph resource handler
    pub fn insight_impact_graph(&self) -> resources::Insight_impact_graph<'_> {
        resources::Insight_impact_graph::new(self.provider)
    }
    /// Get sampling_targets resource handler
    pub fn sampling_targets(&self) -> resources::Sampling_targets<'_> {
        resources::Sampling_targets::new(self.provider)
    }
    /// Get insight_events resource handler
    pub fn insight_events(&self) -> resources::Insight_events<'_> {
        resources::Insight_events::new(self.provider)
    }
    /// Get trace_summaries resource handler
    pub fn trace_summaries(&self) -> resources::Trace_summaries<'_> {
        resources::Trace_summaries::new(self.provider)
    }
    /// Get telemetry_records resource handler
    pub fn telemetry_records(&self) -> resources::Telemetry_records<'_> {
        resources::Telemetry_records::new(self.provider)
    }
    /// Get indexing_rule resource handler
    pub fn indexing_rule(&self) -> resources::Indexing_rule<'_> {
        resources::Indexing_rule::new(self.provider)
    }
    /// Get service_graph resource handler
    pub fn service_graph(&self) -> resources::Service_graph<'_> {
        resources::Service_graph::new(self.provider)
    }
    /// Get insight_summaries resource handler
    pub fn insight_summaries(&self) -> resources::Insight_summaries<'_> {
        resources::Insight_summaries::new(self.provider)
    }
    /// Get insight resource handler
    pub fn insight(&self) -> resources::Insight<'_> {
        resources::Insight::new(self.provider)
    }
    /// Get encryption_config resource handler
    pub fn encryption_config(&self) -> resources::Encryption_config<'_> {
        resources::Encryption_config::new(self.provider)
    }
    /// Get resource_policy resource handler
    pub fn resource_policy(&self) -> resources::Resource_policy<'_> {
        resources::Resource_policy::new(self.provider)
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
