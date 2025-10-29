//! Iam Service
//!
//! Auto-generated service module for iam

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for iam
pub struct IamService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> IamService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get virtual_mfadevice resource handler
    pub fn virtual_mfadevice(&self) -> resources::Virtual_mfadevice<'_> {
        resources::Virtual_mfadevice::new(self.provider)
    }
    /// Get open_idconnect_provider_thumbprint resource handler
    pub fn open_idconnect_provider_thumbprint(&self) -> resources::Open_idconnect_provider_thumbprint<'_> {
        resources::Open_idconnect_provider_thumbprint::new(self.provider)
    }
    /// Get policy_version resource handler
    pub fn policy_version(&self) -> resources::Policy_version<'_> {
        resources::Policy_version::new(self.provider)
    }
    /// Get service_linked_role resource handler
    pub fn service_linked_role(&self) -> resources::Service_linked_role<'_> {
        resources::Service_linked_role::new(self.provider)
    }
    /// Get user_policy resource handler
    pub fn user_policy(&self) -> resources::User_policy<'_> {
        resources::User_policy::new(self.provider)
    }
    /// Get service_linked_role_deletion_status resource handler
    pub fn service_linked_role_deletion_status(&self) -> resources::Service_linked_role_deletion_status<'_> {
        resources::Service_linked_role_deletion_status::new(self.provider)
    }
    /// Get context_keys_for_custom_policy resource handler
    pub fn context_keys_for_custom_policy(&self) -> resources::Context_keys_for_custom_policy<'_> {
        resources::Context_keys_for_custom_policy::new(self.provider)
    }
    /// Get service_last_accessed_details_with_entities resource handler
    pub fn service_last_accessed_details_with_entities(&self) -> resources::Service_last_accessed_details_with_entities<'_> {
        resources::Service_last_accessed_details_with_entities::new(self.provider)
    }
    /// Get role_description resource handler
    pub fn role_description(&self) -> resources::Role_description<'_> {
        resources::Role_description::new(self.provider)
    }
    /// Get role_permissions_boundary resource handler
    pub fn role_permissions_boundary(&self) -> resources::Role_permissions_boundary<'_> {
        resources::Role_permissions_boundary::new(self.provider)
    }
    /// Get service_specific_credential resource handler
    pub fn service_specific_credential(&self) -> resources::Service_specific_credential<'_> {
        resources::Service_specific_credential::new(self.provider)
    }
    /// Get mfadevice resource handler
    pub fn mfadevice(&self) -> resources::Mfadevice<'_> {
        resources::Mfadevice::new(self.provider)
    }
    /// Get role_policy resource handler
    pub fn role_policy(&self) -> resources::Role_policy<'_> {
        resources::Role_policy::new(self.provider)
    }
    /// Get open_idconnect_provider resource handler
    pub fn open_idconnect_provider(&self) -> resources::Open_idconnect_provider<'_> {
        resources::Open_idconnect_provider::new(self.provider)
    }
    /// Get account_alias resource handler
    pub fn account_alias(&self) -> resources::Account_alias<'_> {
        resources::Account_alias::new(self.provider)
    }
    /// Get role resource handler
    pub fn role(&self) -> resources::Role<'_> {
        resources::Role::new(self.provider)
    }
    /// Get organizations_access_report resource handler
    pub fn organizations_access_report(&self) -> resources::Organizations_access_report<'_> {
        resources::Organizations_access_report::new(self.provider)
    }
    /// Get user resource handler
    pub fn user(&self) -> resources::User<'_> {
        resources::User::new(self.provider)
    }
    /// Get access_key resource handler
    pub fn access_key(&self) -> resources::Access_key<'_> {
        resources::Access_key::new(self.provider)
    }
    /// Get sshpublic_key resource handler
    pub fn sshpublic_key(&self) -> resources::Sshpublic_key<'_> {
        resources::Sshpublic_key::new(self.provider)
    }
    /// Get service_last_accessed_details resource handler
    pub fn service_last_accessed_details(&self) -> resources::Service_last_accessed_details<'_> {
        resources::Service_last_accessed_details::new(self.provider)
    }
    /// Get account_password_policy resource handler
    pub fn account_password_policy(&self) -> resources::Account_password_policy<'_> {
        resources::Account_password_policy::new(self.provider)
    }
    /// Get account_summary resource handler
    pub fn account_summary(&self) -> resources::Account_summary<'_> {
        resources::Account_summary::new(self.provider)
    }
    /// Get context_keys_for_principal_policy resource handler
    pub fn context_keys_for_principal_policy(&self) -> resources::Context_keys_for_principal_policy<'_> {
        resources::Context_keys_for_principal_policy::new(self.provider)
    }
    /// Get access_key_last_used resource handler
    pub fn access_key_last_used(&self) -> resources::Access_key_last_used<'_> {
        resources::Access_key_last_used::new(self.provider)
    }
    /// Get policy resource handler
    pub fn policy(&self) -> resources::Policy<'_> {
        resources::Policy::new(self.provider)
    }
    /// Get signing_certificate resource handler
    pub fn signing_certificate(&self) -> resources::Signing_certificate<'_> {
        resources::Signing_certificate::new(self.provider)
    }
    /// Get account_authorization_details resource handler
    pub fn account_authorization_details(&self) -> resources::Account_authorization_details<'_> {
        resources::Account_authorization_details::new(self.provider)
    }
    /// Get group_policy resource handler
    pub fn group_policy(&self) -> resources::Group_policy<'_> {
        resources::Group_policy::new(self.provider)
    }
    /// Get instance_profile resource handler
    pub fn instance_profile(&self) -> resources::Instance_profile<'_> {
        resources::Instance_profile::new(self.provider)
    }
    /// Get user_permissions_boundary resource handler
    pub fn user_permissions_boundary(&self) -> resources::User_permissions_boundary<'_> {
        resources::User_permissions_boundary::new(self.provider)
    }
    /// Get server_certificate resource handler
    pub fn server_certificate(&self) -> resources::Server_certificate<'_> {
        resources::Server_certificate::new(self.provider)
    }
    /// Get group resource handler
    pub fn group(&self) -> resources::Group<'_> {
        resources::Group::new(self.provider)
    }
    /// Get credential_report resource handler
    pub fn credential_report(&self) -> resources::Credential_report<'_> {
        resources::Credential_report::new(self.provider)
    }
    /// Get login_profile resource handler
    pub fn login_profile(&self) -> resources::Login_profile<'_> {
        resources::Login_profile::new(self.provider)
    }
    /// Get samlprovider resource handler
    pub fn samlprovider(&self) -> resources::Samlprovider<'_> {
        resources::Samlprovider::new(self.provider)
    }
    /// Get assume_role_policy resource handler
    pub fn assume_role_policy(&self) -> resources::Assume_role_policy<'_> {
        resources::Assume_role_policy::new(self.provider)
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
