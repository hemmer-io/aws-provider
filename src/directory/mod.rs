//! Directory Service
//!
//! Auto-generated service module for directory

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for directory
pub struct DirectoryService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> DirectoryService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get domain_controllers resource handler
    pub fn domain_controllers(&self) -> resources::Domain_controllers<'_> {
        resources::Domain_controllers::new(self.provider)
    }
    /// Get computer resource handler
    pub fn computer(&self) -> resources::Computer<'_> {
        resources::Computer::new(self.provider)
    }
    /// Get directory_setup resource handler
    pub fn directory_setup(&self) -> resources::Directory_setup<'_> {
        resources::Directory_setup::new(self.provider)
    }
    /// Get certificate resource handler
    pub fn certificate(&self) -> resources::Certificate<'_> {
        resources::Certificate::new(self.provider)
    }
    /// Get directory resource handler
    pub fn directory(&self) -> resources::Directory<'_> {
        resources::Directory::new(self.provider)
    }
    /// Get directory_limits resource handler
    pub fn directory_limits(&self) -> resources::Directory_limits<'_> {
        resources::Directory_limits::new(self.provider)
    }
    /// Get conditional_forwarders resource handler
    pub fn conditional_forwarders(&self) -> resources::Conditional_forwarders<'_> {
        resources::Conditional_forwarders::new(self.provider)
    }
    /// Get trust resource handler
    pub fn trust(&self) -> resources::Trust<'_> {
        resources::Trust::new(self.provider)
    }
    /// Get caenrollment_policy resource handler
    pub fn caenrollment_policy(&self) -> resources::Caenrollment_policy<'_> {
        resources::Caenrollment_policy::new(self.provider)
    }
    /// Get snapshot_limits resource handler
    pub fn snapshot_limits(&self) -> resources::Snapshot_limits<'_> {
        resources::Snapshot_limits::new(self.provider)
    }
    /// Get conditional_forwarder resource handler
    pub fn conditional_forwarder(&self) -> resources::Conditional_forwarder<'_> {
        resources::Conditional_forwarder::new(self.provider)
    }
    /// Get settings resource handler
    pub fn settings(&self) -> resources::Settings<'_> {
        resources::Settings::new(self.provider)
    }
    /// Get regions resource handler
    pub fn regions(&self) -> resources::Regions<'_> {
        resources::Regions::new(self.provider)
    }
    /// Get trusts resource handler
    pub fn trusts(&self) -> resources::Trusts<'_> {
        resources::Trusts::new(self.provider)
    }
    /// Get directory_data_access resource handler
    pub fn directory_data_access(&self) -> resources::Directory_data_access<'_> {
        resources::Directory_data_access::new(self.provider)
    }
    /// Get shared_directories resource handler
    pub fn shared_directories(&self) -> resources::Shared_directories<'_> {
        resources::Shared_directories::new(self.provider)
    }
    /// Get ldapssettings resource handler
    pub fn ldapssettings(&self) -> resources::Ldapssettings<'_> {
        resources::Ldapssettings::new(self.provider)
    }
    /// Get snapshots resource handler
    pub fn snapshots(&self) -> resources::Snapshots<'_> {
        resources::Snapshots::new(self.provider)
    }
    /// Get directories resource handler
    pub fn directories(&self) -> resources::Directories<'_> {
        resources::Directories::new(self.provider)
    }
    /// Get hybrid_adupdate resource handler
    pub fn hybrid_adupdate(&self) -> resources::Hybrid_adupdate<'_> {
        resources::Hybrid_adupdate::new(self.provider)
    }
    /// Get alias resource handler
    pub fn alias(&self) -> resources::Alias<'_> {
        resources::Alias::new(self.provider)
    }
    /// Get hybrid_ad resource handler
    pub fn hybrid_ad(&self) -> resources::Hybrid_ad<'_> {
        resources::Hybrid_ad::new(self.provider)
    }
    /// Get microsoft_ad resource handler
    pub fn microsoft_ad(&self) -> resources::Microsoft_ad<'_> {
        resources::Microsoft_ad::new(self.provider)
    }
    /// Get adassessment resource handler
    pub fn adassessment(&self) -> resources::Adassessment<'_> {
        resources::Adassessment::new(self.provider)
    }
    /// Get log_subscription resource handler
    pub fn log_subscription(&self) -> resources::Log_subscription<'_> {
        resources::Log_subscription::new(self.provider)
    }
    /// Get number_of_domain_controllers resource handler
    pub fn number_of_domain_controllers(&self) -> resources::Number_of_domain_controllers<'_> {
        resources::Number_of_domain_controllers::new(self.provider)
    }
    /// Get client_authentication_settings resource handler
    pub fn client_authentication_settings(&self) -> resources::Client_authentication_settings<'_> {
        resources::Client_authentication_settings::new(self.provider)
    }
    /// Get update_directory resource handler
    pub fn update_directory(&self) -> resources::Update_directory<'_> {
        resources::Update_directory::new(self.provider)
    }
    /// Get snapshot resource handler
    pub fn snapshot(&self) -> resources::Snapshot<'_> {
        resources::Snapshot::new(self.provider)
    }
    /// Get event_topics resource handler
    pub fn event_topics(&self) -> resources::Event_topics<'_> {
        resources::Event_topics::new(self.provider)
    }
    /// Get radius resource handler
    pub fn radius(&self) -> resources::Radius<'_> {
        resources::Radius::new(self.provider)
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
