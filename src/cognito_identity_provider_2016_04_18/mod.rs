//! Cognito_identity_provider_2016_04_18 Service
//!
//! Auto-generated service module for cognito_identity_provider_2016_04_18

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for cognito_identity_provider_2016_04_18
pub struct Cognito_identity_provider_2016_04_18Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Cognito_identity_provider_2016_04_18Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get managed_login_branding resource handler
    pub fn managed_login_branding(&self) -> resources::Managed_login_branding<'_> {
        resources::Managed_login_branding::new(self.provider)
    }
    /// Get managed_login_branding_by_client resource handler
    pub fn managed_login_branding_by_client(&self) -> resources::Managed_login_branding_by_client<'_> {
        resources::Managed_login_branding_by_client::new(self.provider)
    }
    /// Get risk_configuration resource handler
    pub fn risk_configuration(&self) -> resources::Risk_configuration<'_> {
        resources::Risk_configuration::new(self.provider)
    }
    /// Get ui_customization resource handler
    pub fn ui_customization(&self) -> resources::Ui_customization<'_> {
        resources::Ui_customization::new(self.provider)
    }
    /// Get user_attribute_verification_code resource handler
    pub fn user_attribute_verification_code(&self) -> resources::User_attribute_verification_code<'_> {
        resources::User_attribute_verification_code::new(self.provider)
    }
    /// Get user_pool_mfa_config resource handler
    pub fn user_pool_mfa_config(&self) -> resources::User_pool_mfa_config<'_> {
        resources::User_pool_mfa_config::new(self.provider)
    }
    /// Get terms resource handler
    pub fn terms(&self) -> resources::Terms<'_> {
        resources::Terms::new(self.provider)
    }
    /// Get csv_header resource handler
    pub fn csv_header(&self) -> resources::Csv_header<'_> {
        resources::Csv_header::new(self.provider)
    }
    /// Get signing_certificate resource handler
    pub fn signing_certificate(&self) -> resources::Signing_certificate<'_> {
        resources::Signing_certificate::new(self.provider)
    }
    /// Get log_delivery_configuration resource handler
    pub fn log_delivery_configuration(&self) -> resources::Log_delivery_configuration<'_> {
        resources::Log_delivery_configuration::new(self.provider)
    }
    /// Get device resource handler
    pub fn device(&self) -> resources::Device<'_> {
        resources::Device::new(self.provider)
    }
    /// Get user_pool_client resource handler
    pub fn user_pool_client(&self) -> resources::User_pool_client<'_> {
        resources::User_pool_client::new(self.provider)
    }
    /// Get identity_provider_by_identifier resource handler
    pub fn identity_provider_by_identifier(&self) -> resources::Identity_provider_by_identifier<'_> {
        resources::Identity_provider_by_identifier::new(self.provider)
    }
    /// Get user_auth_factors resource handler
    pub fn user_auth_factors(&self) -> resources::User_auth_factors<'_> {
        resources::User_auth_factors::new(self.provider)
    }
    /// Get user_attributes resource handler
    pub fn user_attributes(&self) -> resources::User_attributes<'_> {
        resources::User_attributes::new(self.provider)
    }
    /// Get user_pool resource handler
    pub fn user_pool(&self) -> resources::User_pool<'_> {
        resources::User_pool::new(self.provider)
    }
    /// Get identity_provider resource handler
    pub fn identity_provider(&self) -> resources::Identity_provider<'_> {
        resources::Identity_provider::new(self.provider)
    }
    /// Get user_pool_domain resource handler
    pub fn user_pool_domain(&self) -> resources::User_pool_domain<'_> {
        resources::User_pool_domain::new(self.provider)
    }
    /// Get auth_event_feedback resource handler
    pub fn auth_event_feedback(&self) -> resources::Auth_event_feedback<'_> {
        resources::Auth_event_feedback::new(self.provider)
    }
    /// Get user resource handler
    pub fn user(&self) -> resources::User<'_> {
        resources::User::new(self.provider)
    }
    /// Get resource_server resource handler
    pub fn resource_server(&self) -> resources::Resource_server<'_> {
        resources::Resource_server::new(self.provider)
    }
    /// Get user_import_job resource handler
    pub fn user_import_job(&self) -> resources::User_import_job<'_> {
        resources::User_import_job::new(self.provider)
    }
    /// Get tokens_from_refresh_token resource handler
    pub fn tokens_from_refresh_token(&self) -> resources::Tokens_from_refresh_token<'_> {
        resources::Tokens_from_refresh_token::new(self.provider)
    }
    /// Get device_status resource handler
    pub fn device_status(&self) -> resources::Device_status<'_> {
        resources::Device_status::new(self.provider)
    }
    /// Get group resource handler
    pub fn group(&self) -> resources::Group<'_> {
        resources::Group::new(self.provider)
    }
    /// Get web_authn_credential resource handler
    pub fn web_authn_credential(&self) -> resources::Web_authn_credential<'_> {
        resources::Web_authn_credential::new(self.provider)
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
