//! Customer Service
//!
//! Auto-generated service module for customer

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for customer
pub struct CustomerService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> CustomerService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get segment_membership resource handler
    pub fn segment_membership(&self) -> resources::Segment_membership<'_> {
        resources::Segment_membership::new(self.provider)
    }
    /// Get event_stream resource handler
    pub fn event_stream(&self) -> resources::Event_stream<'_> {
        resources::Event_stream::new(self.provider)
    }
    /// Get matches resource handler
    pub fn matches(&self) -> resources::Matches<'_> {
        resources::Matches::new(self.provider)
    }
    /// Get integration resource handler
    pub fn integration(&self) -> resources::Integration<'_> {
        resources::Integration::new(self.provider)
    }
    /// Get workflow_steps resource handler
    pub fn workflow_steps(&self) -> resources::Workflow_steps<'_> {
        resources::Workflow_steps::new(self.provider)
    }
    /// Get segment_snapshot resource handler
    pub fn segment_snapshot(&self) -> resources::Segment_snapshot<'_> {
        resources::Segment_snapshot::new(self.provider)
    }
    /// Get calculated_attribute_for_profile resource handler
    pub fn calculated_attribute_for_profile(&self) -> resources::Calculated_attribute_for_profile<'_> {
        resources::Calculated_attribute_for_profile::new(self.provider)
    }
    /// Get profile resource handler
    pub fn profile(&self) -> resources::Profile<'_> {
        resources::Profile::new(self.provider)
    }
    /// Get workflow resource handler
    pub fn workflow(&self) -> resources::Workflow<'_> {
        resources::Workflow::new(self.provider)
    }
    /// Get profile_history_record resource handler
    pub fn profile_history_record(&self) -> resources::Profile_history_record<'_> {
        resources::Profile_history_record::new(self.provider)
    }
    /// Get profile_object_type resource handler
    pub fn profile_object_type(&self) -> resources::Profile_object_type<'_> {
        resources::Profile_object_type::new(self.provider)
    }
    /// Get identity_resolution_job resource handler
    pub fn identity_resolution_job(&self) -> resources::Identity_resolution_job<'_> {
        resources::Identity_resolution_job::new(self.provider)
    }
    /// Get profile_object resource handler
    pub fn profile_object(&self) -> resources::Profile_object<'_> {
        resources::Profile_object::new(self.provider)
    }
    /// Get event_trigger resource handler
    pub fn event_trigger(&self) -> resources::Event_trigger<'_> {
        resources::Event_trigger::new(self.provider)
    }
    /// Get upload_job resource handler
    pub fn upload_job(&self) -> resources::Upload_job<'_> {
        resources::Upload_job::new(self.provider)
    }
    /// Get similar_profiles resource handler
    pub fn similar_profiles(&self) -> resources::Similar_profiles<'_> {
        resources::Similar_profiles::new(self.provider)
    }
    /// Get integration_workflow resource handler
    pub fn integration_workflow(&self) -> resources::Integration_workflow<'_> {
        resources::Integration_workflow::new(self.provider)
    }
    /// Get auto_merging_preview resource handler
    pub fn auto_merging_preview(&self) -> resources::Auto_merging_preview<'_> {
        resources::Auto_merging_preview::new(self.provider)
    }
    /// Get profile_object_type_template resource handler
    pub fn profile_object_type_template(&self) -> resources::Profile_object_type_template<'_> {
        resources::Profile_object_type_template::new(self.provider)
    }
    /// Get upload_job_path resource handler
    pub fn upload_job_path(&self) -> resources::Upload_job_path<'_> {
        resources::Upload_job_path::new(self.provider)
    }
    /// Get segment_estimate resource handler
    pub fn segment_estimate(&self) -> resources::Segment_estimate<'_> {
        resources::Segment_estimate::new(self.provider)
    }
    /// Get calculated_attribute_definition resource handler
    pub fn calculated_attribute_definition(&self) -> resources::Calculated_attribute_definition<'_> {
        resources::Calculated_attribute_definition::new(self.provider)
    }
    /// Get domain_layout resource handler
    pub fn domain_layout(&self) -> resources::Domain_layout<'_> {
        resources::Domain_layout::new(self.provider)
    }
    /// Get profile_key resource handler
    pub fn profile_key(&self) -> resources::Profile_key<'_> {
        resources::Profile_key::new(self.provider)
    }
    /// Get domain resource handler
    pub fn domain(&self) -> resources::Domain<'_> {
        resources::Domain::new(self.provider)
    }
    /// Get segment_definition resource handler
    pub fn segment_definition(&self) -> resources::Segment_definition<'_> {
        resources::Segment_definition::new(self.provider)
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
