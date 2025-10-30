//! Workmail_2017_10_01 Service
//!
//! Auto-generated service module for workmail_2017_10_01

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for workmail_2017_10_01
pub struct Workmail_2017_10_01Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Workmail_2017_10_01Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get default_retention_policy resource handler
    pub fn default_retention_policy(&self) -> resources::Default_retention_policy<'_> {
        resources::Default_retention_policy::new(self.provider)
    }
    /// Get inbound_dmarc_settings resource handler
    pub fn inbound_dmarc_settings(&self) -> resources::Inbound_dmarc_settings<'_> {
        resources::Inbound_dmarc_settings::new(self.provider)
    }
    /// Get mobile_device_access_rule resource handler
    pub fn mobile_device_access_rule(&self) -> resources::Mobile_device_access_rule<'_> {
        resources::Mobile_device_access_rule::new(self.provider)
    }
    /// Get access_control_effect resource handler
    pub fn access_control_effect(&self) -> resources::Access_control_effect<'_> {
        resources::Access_control_effect::new(self.provider)
    }
    /// Get access_control_rule resource handler
    pub fn access_control_rule(&self) -> resources::Access_control_rule<'_> {
        resources::Access_control_rule::new(self.provider)
    }
    /// Get mail_domain resource handler
    pub fn mail_domain(&self) -> resources::Mail_domain<'_> {
        resources::Mail_domain::new(self.provider)
    }
    /// Get email_monitoring_configuration resource handler
    pub fn email_monitoring_configuration(&self) -> resources::Email_monitoring_configuration<'_> {
        resources::Email_monitoring_configuration::new(self.provider)
    }
    /// Get impersonation_role_effect resource handler
    pub fn impersonation_role_effect(&self) -> resources::Impersonation_role_effect<'_> {
        resources::Impersonation_role_effect::new(self.provider)
    }
    /// Get mailbox_export_job resource handler
    pub fn mailbox_export_job(&self) -> resources::Mailbox_export_job<'_> {
        resources::Mailbox_export_job::new(self.provider)
    }
    /// Get primary_email_address resource handler
    pub fn primary_email_address(&self) -> resources::Primary_email_address<'_> {
        resources::Primary_email_address::new(self.provider)
    }
    /// Get availability_configuration resource handler
    pub fn availability_configuration(&self) -> resources::Availability_configuration<'_> {
        resources::Availability_configuration::new(self.provider)
    }
    /// Get identity_center_application resource handler
    pub fn identity_center_application(&self) -> resources::Identity_center_application<'_> {
        resources::Identity_center_application::new(self.provider)
    }
    /// Get user resource handler
    pub fn user(&self) -> resources::User<'_> {
        resources::User::new(self.provider)
    }
    /// Get alias resource handler
    pub fn alias(&self) -> resources::Alias<'_> {
        resources::Alias::new(self.provider)
    }
    /// Get impersonation_role resource handler
    pub fn impersonation_role(&self) -> resources::Impersonation_role<'_> {
        resources::Impersonation_role::new(self.provider)
    }
    /// Get mailbox_details resource handler
    pub fn mailbox_details(&self) -> resources::Mailbox_details<'_> {
        resources::Mailbox_details::new(self.provider)
    }
    /// Get personal_access_token resource handler
    pub fn personal_access_token(&self) -> resources::Personal_access_token<'_> {
        resources::Personal_access_token::new(self.provider)
    }
    /// Get mobile_device_access_effect resource handler
    pub fn mobile_device_access_effect(&self) -> resources::Mobile_device_access_effect<'_> {
        resources::Mobile_device_access_effect::new(self.provider)
    }
    /// Get entity resource handler
    pub fn entity(&self) -> resources::Entity<'_> {
        resources::Entity::new(self.provider)
    }
    /// Get mobile_device_access_override resource handler
    pub fn mobile_device_access_override(&self) -> resources::Mobile_device_access_override<'_> {
        resources::Mobile_device_access_override::new(self.provider)
    }
    /// Get personal_access_token_metadata resource handler
    pub fn personal_access_token_metadata(&self) -> resources::Personal_access_token_metadata<'_> {
        resources::Personal_access_token_metadata::new(self.provider)
    }
    /// Get mailbox_quota resource handler
    pub fn mailbox_quota(&self) -> resources::Mailbox_quota<'_> {
        resources::Mailbox_quota::new(self.provider)
    }
    /// Get identity_provider_configuration resource handler
    pub fn identity_provider_configuration(&self) -> resources::Identity_provider_configuration<'_> {
        resources::Identity_provider_configuration::new(self.provider)
    }
    /// Get retention_policy resource handler
    pub fn retention_policy(&self) -> resources::Retention_policy<'_> {
        resources::Retention_policy::new(self.provider)
    }
    /// Get mailbox_permissions resource handler
    pub fn mailbox_permissions(&self) -> resources::Mailbox_permissions<'_> {
        resources::Mailbox_permissions::new(self.provider)
    }
    /// Get resource resource handler
    pub fn resource(&self) -> resources::Resource<'_> {
        resources::Resource::new(self.provider)
    }
    /// Get group resource handler
    pub fn group(&self) -> resources::Group<'_> {
        resources::Group::new(self.provider)
    }
    /// Get default_mail_domain resource handler
    pub fn default_mail_domain(&self) -> resources::Default_mail_domain<'_> {
        resources::Default_mail_domain::new(self.provider)
    }
    /// Get organization resource handler
    pub fn organization(&self) -> resources::Organization<'_> {
        resources::Organization::new(self.provider)
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
