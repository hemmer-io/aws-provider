//! Quicksight Service
//!
//! Auto-generated service module for quicksight

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for quicksight
pub struct QuicksightService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> QuicksightService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get template_definition resource handler
    pub fn template_definition(&self) -> resources::Template_definition<'_> {
        resources::Template_definition::new(self.provider)
    }
    /// Get default_qbusiness_application resource handler
    pub fn default_qbusiness_application(&self) -> resources::Default_qbusiness_application<'_> {
        resources::Default_qbusiness_application::new(self.provider)
    }
    /// Get brand_published_version resource handler
    pub fn brand_published_version(&self) -> resources::Brand_published_version<'_> {
        resources::Brand_published_version::new(self.provider)
    }
    /// Get user resource handler
    pub fn user(&self) -> resources::User<'_> {
        resources::User::new(self.provider)
    }
    /// Get data_set resource handler
    pub fn data_set(&self) -> resources::Data_set<'_> {
        resources::Data_set::new(self.provider)
    }
    /// Get folder_membership resource handler
    pub fn folder_membership(&self) -> resources::Folder_membership<'_> {
        resources::Folder_membership::new(self.provider)
    }
    /// Get quick_sight_qsearch_configuration resource handler
    pub fn quick_sight_qsearch_configuration(&self) -> resources::Quick_sight_qsearch_configuration<'_> {
        resources::Quick_sight_qsearch_configuration::new(self.provider)
    }
    /// Get custom_permissions resource handler
    pub fn custom_permissions(&self) -> resources::Custom_permissions<'_> {
        resources::Custom_permissions::new(self.provider)
    }
    /// Get action_connector resource handler
    pub fn action_connector(&self) -> resources::Action_connector<'_> {
        resources::Action_connector::new(self.provider)
    }
    /// Get analysis_permissions resource handler
    pub fn analysis_permissions(&self) -> resources::Analysis_permissions<'_> {
        resources::Analysis_permissions::new(self.provider)
    }
    /// Get theme resource handler
    pub fn theme(&self) -> resources::Theme<'_> {
        resources::Theme::new(self.provider)
    }
    /// Get asset_bundle_import_job resource handler
    pub fn asset_bundle_import_job(&self) -> resources::Asset_bundle_import_job<'_> {
        resources::Asset_bundle_import_job::new(self.provider)
    }
    /// Get brand_assignment resource handler
    pub fn brand_assignment(&self) -> resources::Brand_assignment<'_> {
        resources::Brand_assignment::new(self.provider)
    }
    /// Get template resource handler
    pub fn template(&self) -> resources::Template<'_> {
        resources::Template::new(self.provider)
    }
    /// Get user_custom_permission resource handler
    pub fn user_custom_permission(&self) -> resources::User_custom_permission<'_> {
        resources::User_custom_permission::new(self.provider)
    }
    /// Get folder_resolved_permissions resource handler
    pub fn folder_resolved_permissions(&self) -> resources::Folder_resolved_permissions<'_> {
        resources::Folder_resolved_permissions::new(self.provider)
    }
    /// Get dashboard_snapshot_job resource handler
    pub fn dashboard_snapshot_job(&self) -> resources::Dashboard_snapshot_job<'_> {
        resources::Dashboard_snapshot_job::new(self.provider)
    }
    /// Get folder_permissions resource handler
    pub fn folder_permissions(&self) -> resources::Folder_permissions<'_> {
        resources::Folder_permissions::new(self.provider)
    }
    /// Get data_set_permissions resource handler
    pub fn data_set_permissions(&self) -> resources::Data_set_permissions<'_> {
        resources::Data_set_permissions::new(self.provider)
    }
    /// Get topic_permissions resource handler
    pub fn topic_permissions(&self) -> resources::Topic_permissions<'_> {
        resources::Topic_permissions::new(self.provider)
    }
    /// Get flow_permissions resource handler
    pub fn flow_permissions(&self) -> resources::Flow_permissions<'_> {
        resources::Flow_permissions::new(self.provider)
    }
    /// Get theme_permissions resource handler
    pub fn theme_permissions(&self) -> resources::Theme_permissions<'_> {
        resources::Theme_permissions::new(self.provider)
    }
    /// Get account_custom_permission resource handler
    pub fn account_custom_permission(&self) -> resources::Account_custom_permission<'_> {
        resources::Account_custom_permission::new(self.provider)
    }
    /// Get topic_refresh resource handler
    pub fn topic_refresh(&self) -> resources::Topic_refresh<'_> {
        resources::Topic_refresh::new(self.provider)
    }
    /// Get identity_propagation_config resource handler
    pub fn identity_propagation_config(&self) -> resources::Identity_propagation_config<'_> {
        resources::Identity_propagation_config::new(self.provider)
    }
    /// Get dashboard resource handler
    pub fn dashboard(&self) -> resources::Dashboard<'_> {
        resources::Dashboard::new(self.provider)
    }
    /// Get session_embed_url resource handler
    pub fn session_embed_url(&self) -> resources::Session_embed_url<'_> {
        resources::Session_embed_url::new(self.provider)
    }
    /// Get topic_refresh_schedule resource handler
    pub fn topic_refresh_schedule(&self) -> resources::Topic_refresh_schedule<'_> {
        resources::Topic_refresh_schedule::new(self.provider)
    }
    /// Get ingestion resource handler
    pub fn ingestion(&self) -> resources::Ingestion<'_> {
        resources::Ingestion::new(self.provider)
    }
    /// Get asset_bundle_export_job resource handler
    pub fn asset_bundle_export_job(&self) -> resources::Asset_bundle_export_job<'_> {
        resources::Asset_bundle_export_job::new(self.provider)
    }
    /// Get ip_restriction resource handler
    pub fn ip_restriction(&self) -> resources::Ip_restriction<'_> {
        resources::Ip_restriction::new(self.provider)
    }
    /// Get iampolicy_assignment resource handler
    pub fn iampolicy_assignment(&self) -> resources::Iampolicy_assignment<'_> {
        resources::Iampolicy_assignment::new(self.provider)
    }
    /// Get group resource handler
    pub fn group(&self) -> resources::Group<'_> {
        resources::Group::new(self.provider)
    }
    /// Get dashboard_snapshot_job_result resource handler
    pub fn dashboard_snapshot_job_result(&self) -> resources::Dashboard_snapshot_job_result<'_> {
        resources::Dashboard_snapshot_job_result::new(self.provider)
    }
    /// Get public_sharing_settings resource handler
    pub fn public_sharing_settings(&self) -> resources::Public_sharing_settings<'_> {
        resources::Public_sharing_settings::new(self.provider)
    }
    /// Get role_custom_permission resource handler
    pub fn role_custom_permission(&self) -> resources::Role_custom_permission<'_> {
        resources::Role_custom_permission::new(self.provider)
    }
    /// Get qpersonalization_configuration resource handler
    pub fn qpersonalization_configuration(&self) -> resources::Qpersonalization_configuration<'_> {
        resources::Qpersonalization_configuration::new(self.provider)
    }
    /// Get dashboard_links resource handler
    pub fn dashboard_links(&self) -> resources::Dashboard_links<'_> {
        resources::Dashboard_links::new(self.provider)
    }
    /// Get vpcconnection resource handler
    pub fn vpcconnection(&self) -> resources::Vpcconnection<'_> {
        resources::Vpcconnection::new(self.provider)
    }
    /// Get group_membership resource handler
    pub fn group_membership(&self) -> resources::Group_membership<'_> {
        resources::Group_membership::new(self.provider)
    }
    /// Get account_subscription resource handler
    pub fn account_subscription(&self) -> resources::Account_subscription<'_> {
        resources::Account_subscription::new(self.provider)
    }
    /// Get dashboard_definition resource handler
    pub fn dashboard_definition(&self) -> resources::Dashboard_definition<'_> {
        resources::Dashboard_definition::new(self.provider)
    }
    /// Get template_permissions resource handler
    pub fn template_permissions(&self) -> resources::Template_permissions<'_> {
        resources::Template_permissions::new(self.provider)
    }
    /// Get template_alias resource handler
    pub fn template_alias(&self) -> resources::Template_alias<'_> {
        resources::Template_alias::new(self.provider)
    }
    /// Get dashboard_permissions resource handler
    pub fn dashboard_permissions(&self) -> resources::Dashboard_permissions<'_> {
        resources::Dashboard_permissions::new(self.provider)
    }
    /// Get account_customization resource handler
    pub fn account_customization(&self) -> resources::Account_customization<'_> {
        resources::Account_customization::new(self.provider)
    }
    /// Get account_settings resource handler
    pub fn account_settings(&self) -> resources::Account_settings<'_> {
        resources::Account_settings::new(self.provider)
    }
    /// Get brand resource handler
    pub fn brand(&self) -> resources::Brand<'_> {
        resources::Brand::new(self.provider)
    }
    /// Get namespace resource handler
    pub fn namespace(&self) -> resources::Namespace<'_> {
        resources::Namespace::new(self.provider)
    }
    /// Get data_set_refresh_properties resource handler
    pub fn data_set_refresh_properties(&self) -> resources::Data_set_refresh_properties<'_> {
        resources::Data_set_refresh_properties::new(self.provider)
    }
    /// Get refresh_schedule resource handler
    pub fn refresh_schedule(&self) -> resources::Refresh_schedule<'_> {
        resources::Refresh_schedule::new(self.provider)
    }
    /// Get data_source resource handler
    pub fn data_source(&self) -> resources::Data_source<'_> {
        resources::Data_source::new(self.provider)
    }
    /// Get role_membership resource handler
    pub fn role_membership(&self) -> resources::Role_membership<'_> {
        resources::Role_membership::new(self.provider)
    }
    /// Get user_by_principal_id resource handler
    pub fn user_by_principal_id(&self) -> resources::User_by_principal_id<'_> {
        resources::User_by_principal_id::new(self.provider)
    }
    /// Get action_connector_permissions resource handler
    pub fn action_connector_permissions(&self) -> resources::Action_connector_permissions<'_> {
        resources::Action_connector_permissions::new(self.provider)
    }
    /// Get key_registration resource handler
    pub fn key_registration(&self) -> resources::Key_registration<'_> {
        resources::Key_registration::new(self.provider)
    }
    /// Get folder resource handler
    pub fn folder(&self) -> resources::Folder<'_> {
        resources::Folder::new(self.provider)
    }
    /// Get application_with_token_exchange_grant resource handler
    pub fn application_with_token_exchange_grant(&self) -> resources::Application_with_token_exchange_grant<'_> {
        resources::Application_with_token_exchange_grant::new(self.provider)
    }
    /// Get dashboards_qaconfiguration resource handler
    pub fn dashboards_qaconfiguration(&self) -> resources::Dashboards_qaconfiguration<'_> {
        resources::Dashboards_qaconfiguration::new(self.provider)
    }
    /// Get analysis resource handler
    pub fn analysis(&self) -> resources::Analysis<'_> {
        resources::Analysis::new(self.provider)
    }
    /// Get theme_alias resource handler
    pub fn theme_alias(&self) -> resources::Theme_alias<'_> {
        resources::Theme_alias::new(self.provider)
    }
    /// Get dashboard_published_version resource handler
    pub fn dashboard_published_version(&self) -> resources::Dashboard_published_version<'_> {
        resources::Dashboard_published_version::new(self.provider)
    }
    /// Get data_source_permissions resource handler
    pub fn data_source_permissions(&self) -> resources::Data_source_permissions<'_> {
        resources::Data_source_permissions::new(self.provider)
    }
    /// Get analysis_definition resource handler
    pub fn analysis_definition(&self) -> resources::Analysis_definition<'_> {
        resources::Analysis_definition::new(self.provider)
    }
    /// Get dashboard_embed_url resource handler
    pub fn dashboard_embed_url(&self) -> resources::Dashboard_embed_url<'_> {
        resources::Dashboard_embed_url::new(self.provider)
    }
    /// Get flow_metadata resource handler
    pub fn flow_metadata(&self) -> resources::Flow_metadata<'_> {
        resources::Flow_metadata::new(self.provider)
    }
    /// Get topic resource handler
    pub fn topic(&self) -> resources::Topic<'_> {
        resources::Topic::new(self.provider)
    }
    /// Get spicecapacity_configuration resource handler
    pub fn spicecapacity_configuration(&self) -> resources::Spicecapacity_configuration<'_> {
        resources::Spicecapacity_configuration::new(self.provider)
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
