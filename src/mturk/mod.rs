//! Mturk Service
//!
//! Auto-generated service module for mturk

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for mturk
pub struct MturkService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> MturkService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get hittype_of_hit resource handler
    pub fn hittype_of_hit(&self) -> resources::Hittype_of_hit<'_> {
        resources::Hittype_of_hit::new(self.provider)
    }
    /// Get account_balance resource handler
    pub fn account_balance(&self) -> resources::Account_balance<'_> {
        resources::Account_balance::new(self.provider)
    }
    /// Get hit resource handler
    pub fn hit(&self) -> resources::Hit<'_> {
        resources::Hit::new(self.provider)
    }
    /// Get file_upload_url resource handler
    pub fn file_upload_url(&self) -> resources::File_upload_url<'_> {
        resources::File_upload_url::new(self.provider)
    }
    /// Get qualification_type resource handler
    pub fn qualification_type(&self) -> resources::Qualification_type<'_> {
        resources::Qualification_type::new(self.provider)
    }
    /// Get additional_assignments_for_hit resource handler
    pub fn additional_assignments_for_hit(&self) -> resources::Additional_assignments_for_hit<'_> {
        resources::Additional_assignments_for_hit::new(self.provider)
    }
    /// Get hitreview_status resource handler
    pub fn hitreview_status(&self) -> resources::Hitreview_status<'_> {
        resources::Hitreview_status::new(self.provider)
    }
    /// Get hitwith_hittype resource handler
    pub fn hitwith_hittype(&self) -> resources::Hitwith_hittype<'_> {
        resources::Hitwith_hittype::new(self.provider)
    }
    /// Get assignment resource handler
    pub fn assignment(&self) -> resources::Assignment<'_> {
        resources::Assignment::new(self.provider)
    }
    /// Get hittype resource handler
    pub fn hittype(&self) -> resources::Hittype<'_> {
        resources::Hittype::new(self.provider)
    }
    /// Get qualification_score resource handler
    pub fn qualification_score(&self) -> resources::Qualification_score<'_> {
        resources::Qualification_score::new(self.provider)
    }
    /// Get notification_settings resource handler
    pub fn notification_settings(&self) -> resources::Notification_settings<'_> {
        resources::Notification_settings::new(self.provider)
    }
    /// Get worker_block resource handler
    pub fn worker_block(&self) -> resources::Worker_block<'_> {
        resources::Worker_block::new(self.provider)
    }
    /// Get expiration_for_hit resource handler
    pub fn expiration_for_hit(&self) -> resources::Expiration_for_hit<'_> {
        resources::Expiration_for_hit::new(self.provider)
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
