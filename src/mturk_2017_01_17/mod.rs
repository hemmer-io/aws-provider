//! Mturk_2017_01_17 Service
//!
//! Auto-generated service module for mturk_2017_01_17

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for mturk_2017_01_17
pub struct Mturk_2017_01_17Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Mturk_2017_01_17Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get file_upload_url resource handler
    pub fn file_upload_url(&self) -> resources::File_upload_url<'_> {
        resources::File_upload_url::new(self.provider)
    }
    /// Get hit_type resource handler
    pub fn hit_type(&self) -> resources::Hit_type<'_> {
        resources::Hit_type::new(self.provider)
    }
    /// Get hit_review_status resource handler
    pub fn hit_review_status(&self) -> resources::Hit_review_status<'_> {
        resources::Hit_review_status::new(self.provider)
    }
    /// Get notification_settings resource handler
    pub fn notification_settings(&self) -> resources::Notification_settings<'_> {
        resources::Notification_settings::new(self.provider)
    }
    /// Get qualification_type resource handler
    pub fn qualification_type(&self) -> resources::Qualification_type<'_> {
        resources::Qualification_type::new(self.provider)
    }
    /// Get assignment resource handler
    pub fn assignment(&self) -> resources::Assignment<'_> {
        resources::Assignment::new(self.provider)
    }
    /// Get qualification_score resource handler
    pub fn qualification_score(&self) -> resources::Qualification_score<'_> {
        resources::Qualification_score::new(self.provider)
    }
    /// Get hit_type_of_hit resource handler
    pub fn hit_type_of_hit(&self) -> resources::Hit_type_of_hit<'_> {
        resources::Hit_type_of_hit::new(self.provider)
    }
    /// Get hit_with_hit_type resource handler
    pub fn hit_with_hit_type(&self) -> resources::Hit_with_hit_type<'_> {
        resources::Hit_with_hit_type::new(self.provider)
    }
    /// Get additional_assignments_for_hit resource handler
    pub fn additional_assignments_for_hit(&self) -> resources::Additional_assignments_for_hit<'_> {
        resources::Additional_assignments_for_hit::new(self.provider)
    }
    /// Get expiration_for_hit resource handler
    pub fn expiration_for_hit(&self) -> resources::Expiration_for_hit<'_> {
        resources::Expiration_for_hit::new(self.provider)
    }
    /// Get account_balance resource handler
    pub fn account_balance(&self) -> resources::Account_balance<'_> {
        resources::Account_balance::new(self.provider)
    }
    /// Get worker_block resource handler
    pub fn worker_block(&self) -> resources::Worker_block<'_> {
        resources::Worker_block::new(self.provider)
    }
    /// Get hit resource handler
    pub fn hit(&self) -> resources::Hit<'_> {
        resources::Hit::new(self.provider)
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
