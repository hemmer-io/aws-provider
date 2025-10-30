//! Wellarchitected_2020_03_31 Service
//!
//! Auto-generated service module for wellarchitected_2020_03_31

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for wellarchitected_2020_03_31
pub struct Wellarchitected_2020_03_31Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Wellarchitected_2020_03_31Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get review_template_lens_review resource handler
    pub fn review_template_lens_review(&self) -> resources::Review_template_lens_review<'_> {
        resources::Review_template_lens_review::new(self.provider)
    }
    /// Get milestone resource handler
    pub fn milestone(&self) -> resources::Milestone<'_> {
        resources::Milestone::new(self.provider)
    }
    /// Get workload resource handler
    pub fn workload(&self) -> resources::Workload<'_> {
        resources::Workload::new(self.provider)
    }
    /// Get workload_share resource handler
    pub fn workload_share(&self) -> resources::Workload_share<'_> {
        resources::Workload_share::new(self.provider)
    }
    /// Get profile resource handler
    pub fn profile(&self) -> resources::Profile<'_> {
        resources::Profile::new(self.provider)
    }
    /// Get answer resource handler
    pub fn answer(&self) -> resources::Answer<'_> {
        resources::Answer::new(self.provider)
    }
    /// Get lens_version_difference resource handler
    pub fn lens_version_difference(&self) -> resources::Lens_version_difference<'_> {
        resources::Lens_version_difference::new(self.provider)
    }
    /// Get review_template_answer resource handler
    pub fn review_template_answer(&self) -> resources::Review_template_answer<'_> {
        resources::Review_template_answer::new(self.provider)
    }
    /// Get consolidated_report resource handler
    pub fn consolidated_report(&self) -> resources::Consolidated_report<'_> {
        resources::Consolidated_report::new(self.provider)
    }
    /// Get lens resource handler
    pub fn lens(&self) -> resources::Lens<'_> {
        resources::Lens::new(self.provider)
    }
    /// Get share_invitation resource handler
    pub fn share_invitation(&self) -> resources::Share_invitation<'_> {
        resources::Share_invitation::new(self.provider)
    }
    /// Get review_template resource handler
    pub fn review_template(&self) -> resources::Review_template<'_> {
        resources::Review_template::new(self.provider)
    }
    /// Get lens_review resource handler
    pub fn lens_review(&self) -> resources::Lens_review<'_> {
        resources::Lens_review::new(self.provider)
    }
    /// Get integration resource handler
    pub fn integration(&self) -> resources::Integration<'_> {
        resources::Integration::new(self.provider)
    }
    /// Get profile_share resource handler
    pub fn profile_share(&self) -> resources::Profile_share<'_> {
        resources::Profile_share::new(self.provider)
    }
    /// Get template_share resource handler
    pub fn template_share(&self) -> resources::Template_share<'_> {
        resources::Template_share::new(self.provider)
    }
    /// Get lens_version resource handler
    pub fn lens_version(&self) -> resources::Lens_version<'_> {
        resources::Lens_version::new(self.provider)
    }
    /// Get lens_share resource handler
    pub fn lens_share(&self) -> resources::Lens_share<'_> {
        resources::Lens_share::new(self.provider)
    }
    /// Get global_settings resource handler
    pub fn global_settings(&self) -> resources::Global_settings<'_> {
        resources::Global_settings::new(self.provider)
    }
    /// Get lens_review_report resource handler
    pub fn lens_review_report(&self) -> resources::Lens_review_report<'_> {
        resources::Lens_review_report::new(self.provider)
    }
    /// Get profile_template resource handler
    pub fn profile_template(&self) -> resources::Profile_template<'_> {
        resources::Profile_template::new(self.provider)
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
