//! Connect Service
//!
//! Auto-generated service module for connect

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for connect
pub struct ConnectService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> ConnectService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get user_routing_profile resource handler
    pub fn user_routing_profile(&self) -> resources::User_routing_profile<'_> {
        resources::User_routing_profile::new(self.provider)
    }
    /// Get hours_of_operation resource handler
    pub fn hours_of_operation(&self) -> resources::Hours_of_operation<'_> {
        resources::Hours_of_operation::new(self.provider)
    }
    /// Get persistent_contact_association resource handler
    pub fn persistent_contact_association(&self) -> resources::Persistent_contact_association<'_> {
        resources::Persistent_contact_association::new(self.provider)
    }
    /// Get vocabulary resource handler
    pub fn vocabulary(&self) -> resources::Vocabulary<'_> {
        resources::Vocabulary::new(self.provider)
    }
    /// Get queue resource handler
    pub fn queue(&self) -> resources::Queue<'_> {
        resources::Queue::new(self.provider)
    }
    /// Get user_hierarchy_group resource handler
    pub fn user_hierarchy_group(&self) -> resources::User_hierarchy_group<'_> {
        resources::User_hierarchy_group::new(self.provider)
    }
    /// Get current_metric_data resource handler
    pub fn current_metric_data(&self) -> resources::Current_metric_data<'_> {
        resources::Current_metric_data::new(self.provider)
    }
    /// Get metric_data resource handler
    pub fn metric_data(&self) -> resources::Metric_data<'_> {
        resources::Metric_data::new(self.provider)
    }
    /// Get email_address resource handler
    pub fn email_address(&self) -> resources::Email_address<'_> {
        resources::Email_address::new(self.provider)
    }
    /// Get user resource handler
    pub fn user(&self) -> resources::User<'_> {
        resources::User::new(self.provider)
    }
    /// Get contact_schedule resource handler
    pub fn contact_schedule(&self) -> resources::Contact_schedule<'_> {
        resources::Contact_schedule::new(self.provider)
    }
    /// Get contact_routing_data resource handler
    pub fn contact_routing_data(&self) -> resources::Contact_routing_data<'_> {
        resources::Contact_routing_data::new(self.provider)
    }
    /// Get queue_outbound_caller_config resource handler
    pub fn queue_outbound_caller_config(&self) -> resources::Queue_outbound_caller_config<'_> {
        resources::Queue_outbound_caller_config::new(self.provider)
    }
    /// Get routing_profile_default_outbound_queue resource handler
    pub fn routing_profile_default_outbound_queue(&self) -> resources::Routing_profile_default_outbound_queue<'_> {
        resources::Routing_profile_default_outbound_queue::new(self.provider)
    }
    /// Get user_identity_info resource handler
    pub fn user_identity_info(&self) -> resources::User_identity_info<'_> {
        resources::User_identity_info::new(self.provider)
    }
    /// Get user_security_profiles resource handler
    pub fn user_security_profiles(&self) -> resources::User_security_profiles<'_> {
        resources::User_security_profiles::new(self.provider)
    }
    /// Get contact_flow_content resource handler
    pub fn contact_flow_content(&self) -> resources::Contact_flow_content<'_> {
        resources::Contact_flow_content::new(self.provider)
    }
    /// Get contact_flow_module_content resource handler
    pub fn contact_flow_module_content(&self) -> resources::Contact_flow_module_content<'_> {
        resources::Contact_flow_module_content::new(self.provider)
    }
    /// Get queue_outbound_email_config resource handler
    pub fn queue_outbound_email_config(&self) -> resources::Queue_outbound_email_config<'_> {
        resources::Queue_outbound_email_config::new(self.provider)
    }
    /// Get routing_profile_concurrency resource handler
    pub fn routing_profile_concurrency(&self) -> resources::Routing_profile_concurrency<'_> {
        resources::Routing_profile_concurrency::new(self.provider)
    }
    /// Get view_content resource handler
    pub fn view_content(&self) -> resources::View_content<'_> {
        resources::View_content::new(self.provider)
    }
    /// Get contact_attributes resource handler
    pub fn contact_attributes(&self) -> resources::Contact_attributes<'_> {
        resources::Contact_attributes::new(self.provider)
    }
    /// Get view_metadata resource handler
    pub fn view_metadata(&self) -> resources::View_metadata<'_> {
        resources::View_metadata::new(self.provider)
    }
    /// Get email_address_metadata resource handler
    pub fn email_address_metadata(&self) -> resources::Email_address_metadata<'_> {
        resources::Email_address_metadata::new(self.provider)
    }
    /// Get quick_connect_config resource handler
    pub fn quick_connect_config(&self) -> resources::Quick_connect_config<'_> {
        resources::Quick_connect_config::new(self.provider)
    }
    /// Get routing_profile_agent_availability_timer resource handler
    pub fn routing_profile_agent_availability_timer(&self) -> resources::Routing_profile_agent_availability_timer<'_> {
        resources::Routing_profile_agent_availability_timer::new(self.provider)
    }
    /// Get view_version resource handler
    pub fn view_version(&self) -> resources::View_version<'_> {
        resources::View_version::new(self.provider)
    }
    /// Get evaluation_form resource handler
    pub fn evaluation_form(&self) -> resources::Evaluation_form<'_> {
        resources::Evaluation_form::new(self.provider)
    }
    /// Get phone_number resource handler
    pub fn phone_number(&self) -> resources::Phone_number<'_> {
        resources::Phone_number::new(self.provider)
    }
    /// Get user_hierarchy resource handler
    pub fn user_hierarchy(&self) -> resources::User_hierarchy<'_> {
        resources::User_hierarchy::new(self.provider)
    }
    /// Get contact_metrics resource handler
    pub fn contact_metrics(&self) -> resources::Contact_metrics<'_> {
        resources::Contact_metrics::new(self.provider)
    }
    /// Get rule resource handler
    pub fn rule(&self) -> resources::Rule<'_> {
        resources::Rule::new(self.provider)
    }
    /// Get contact_evaluation resource handler
    pub fn contact_evaluation(&self) -> resources::Contact_evaluation<'_> {
        resources::Contact_evaluation::new(self.provider)
    }
    /// Get authentication_profile resource handler
    pub fn authentication_profile(&self) -> resources::Authentication_profile<'_> {
        resources::Authentication_profile::new(self.provider)
    }
    /// Get federation_token resource handler
    pub fn federation_token(&self) -> resources::Federation_token<'_> {
        resources::Federation_token::new(self.provider)
    }
    /// Get routing_profile resource handler
    pub fn routing_profile(&self) -> resources::Routing_profile<'_> {
        resources::Routing_profile::new(self.provider)
    }
    /// Get instance_storage_config resource handler
    pub fn instance_storage_config(&self) -> resources::Instance_storage_config<'_> {
        resources::Instance_storage_config::new(self.provider)
    }
    /// Get traffic_distribution resource handler
    pub fn traffic_distribution(&self) -> resources::Traffic_distribution<'_> {
        resources::Traffic_distribution::new(self.provider)
    }
    /// Get metric_data_v2 resource handler
    pub fn metric_data_v2(&self) -> resources::Metric_data_v2<'_> {
        resources::Metric_data_v2::new(self.provider)
    }
    /// Get contact resource handler
    pub fn contact(&self) -> resources::Contact<'_> {
        resources::Contact::new(self.provider)
    }
    /// Get user_hierarchy_structure resource handler
    pub fn user_hierarchy_structure(&self) -> resources::User_hierarchy_structure<'_> {
        resources::User_hierarchy_structure::new(self.provider)
    }
    /// Get contact_flow_name resource handler
    pub fn contact_flow_name(&self) -> resources::Contact_flow_name<'_> {
        resources::Contact_flow_name::new(self.provider)
    }
    /// Get queue_name resource handler
    pub fn queue_name(&self) -> resources::Queue_name<'_> {
        resources::Queue_name::new(self.provider)
    }
    /// Get routing_profile_queues resource handler
    pub fn routing_profile_queues(&self) -> resources::Routing_profile_queues<'_> {
        resources::Routing_profile_queues::new(self.provider)
    }
    /// Get predefined_attribute resource handler
    pub fn predefined_attribute(&self) -> resources::Predefined_attribute<'_> {
        resources::Predefined_attribute::new(self.provider)
    }
    /// Get instance resource handler
    pub fn instance(&self) -> resources::Instance<'_> {
        resources::Instance::new(self.provider)
    }
    /// Get integration_association resource handler
    pub fn integration_association(&self) -> resources::Integration_association<'_> {
        resources::Integration_association::new(self.provider)
    }
    /// Get traffic_distribution_group resource handler
    pub fn traffic_distribution_group(&self) -> resources::Traffic_distribution_group<'_> {
        resources::Traffic_distribution_group::new(self.provider)
    }
    /// Get security_profile resource handler
    pub fn security_profile(&self) -> resources::Security_profile<'_> {
        resources::Security_profile::new(self.provider)
    }
    /// Get flow_association resource handler
    pub fn flow_association(&self) -> resources::Flow_association<'_> {
        resources::Flow_association::new(self.provider)
    }
    /// Get participant resource handler
    pub fn participant(&self) -> resources::Participant<'_> {
        resources::Participant::new(self.provider)
    }
    /// Get contact_flow_module_metadata resource handler
    pub fn contact_flow_module_metadata(&self) -> resources::Contact_flow_module_metadata<'_> {
        resources::Contact_flow_module_metadata::new(self.provider)
    }
    /// Get push_notification_registration resource handler
    pub fn push_notification_registration(&self) -> resources::Push_notification_registration<'_> {
        resources::Push_notification_registration::new(self.provider)
    }
    /// Get contact_flow_module resource handler
    pub fn contact_flow_module(&self) -> resources::Contact_flow_module<'_> {
        resources::Contact_flow_module::new(self.provider)
    }
    /// Get hours_of_operation_override resource handler
    pub fn hours_of_operation_override(&self) -> resources::Hours_of_operation_override<'_> {
        resources::Hours_of_operation_override::new(self.provider)
    }
    /// Get prompt resource handler
    pub fn prompt(&self) -> resources::Prompt<'_> {
        resources::Prompt::new(self.provider)
    }
    /// Get attached_file resource handler
    pub fn attached_file(&self) -> resources::Attached_file<'_> {
        resources::Attached_file::new(self.provider)
    }
    /// Get current_user_data resource handler
    pub fn current_user_data(&self) -> resources::Current_user_data<'_> {
        resources::Current_user_data::new(self.provider)
    }
    /// Get use_case resource handler
    pub fn use_case(&self) -> resources::Use_case<'_> {
        resources::Use_case::new(self.provider)
    }
    /// Get contact_flow_metadata resource handler
    pub fn contact_flow_metadata(&self) -> resources::Contact_flow_metadata<'_> {
        resources::Contact_flow_metadata::new(self.provider)
    }
    /// Get user_hierarchy_group_name resource handler
    pub fn user_hierarchy_group_name(&self) -> resources::User_hierarchy_group_name<'_> {
        resources::User_hierarchy_group_name::new(self.provider)
    }
    /// Get user_proficiencies resource handler
    pub fn user_proficiencies(&self) -> resources::User_proficiencies<'_> {
        resources::User_proficiencies::new(self.provider)
    }
    /// Get queue_max_contacts resource handler
    pub fn queue_max_contacts(&self) -> resources::Queue_max_contacts<'_> {
        resources::Queue_max_contacts::new(self.provider)
    }
    /// Get participant_authentication resource handler
    pub fn participant_authentication(&self) -> resources::Participant_authentication<'_> {
        resources::Participant_authentication::new(self.provider)
    }
    /// Get queue_hours_of_operation resource handler
    pub fn queue_hours_of_operation(&self) -> resources::Queue_hours_of_operation<'_> {
        resources::Queue_hours_of_operation::new(self.provider)
    }
    /// Get user_status resource handler
    pub fn user_status(&self) -> resources::User_status<'_> {
        resources::User_status::new(self.provider)
    }
    /// Get phone_number_metadata resource handler
    pub fn phone_number_metadata(&self) -> resources::Phone_number_metadata<'_> {
        resources::Phone_number_metadata::new(self.provider)
    }
    /// Get quick_connect_name resource handler
    pub fn quick_connect_name(&self) -> resources::Quick_connect_name<'_> {
        resources::Quick_connect_name::new(self.provider)
    }
    /// Get quick_connect resource handler
    pub fn quick_connect(&self) -> resources::Quick_connect<'_> {
        resources::Quick_connect::new(self.provider)
    }
    /// Get effective_hours_of_operations resource handler
    pub fn effective_hours_of_operations(&self) -> resources::Effective_hours_of_operations<'_> {
        resources::Effective_hours_of_operations::new(self.provider)
    }
    /// Get queue_status resource handler
    pub fn queue_status(&self) -> resources::Queue_status<'_> {
        resources::Queue_status::new(self.provider)
    }
    /// Get view resource handler
    pub fn view(&self) -> resources::View<'_> {
        resources::View::new(self.provider)
    }
    /// Get instance_attribute resource handler
    pub fn instance_attribute(&self) -> resources::Instance_attribute<'_> {
        resources::Instance_attribute::new(self.provider)
    }
    /// Get routing_profile_name resource handler
    pub fn routing_profile_name(&self) -> resources::Routing_profile_name<'_> {
        resources::Routing_profile_name::new(self.provider)
    }
    /// Get user_phone_config resource handler
    pub fn user_phone_config(&self) -> resources::User_phone_config<'_> {
        resources::User_phone_config::new(self.provider)
    }
    /// Get agent_status resource handler
    pub fn agent_status(&self) -> resources::Agent_status<'_> {
        resources::Agent_status::new(self.provider)
    }
    /// Get prompt_file resource handler
    pub fn prompt_file(&self) -> resources::Prompt_file<'_> {
        resources::Prompt_file::new(self.provider)
    }
    /// Get contact_flow_version resource handler
    pub fn contact_flow_version(&self) -> resources::Contact_flow_version<'_> {
        resources::Contact_flow_version::new(self.provider)
    }
    /// Get participant_role_config resource handler
    pub fn participant_role_config(&self) -> resources::Participant_role_config<'_> {
        resources::Participant_role_config::new(self.provider)
    }
    /// Get contact_flow resource handler
    pub fn contact_flow(&self) -> resources::Contact_flow<'_> {
        resources::Contact_flow::new(self.provider)
    }
    /// Get task_template resource handler
    pub fn task_template(&self) -> resources::Task_template<'_> {
        resources::Task_template::new(self.provider)
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
