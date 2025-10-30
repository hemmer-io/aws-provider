//! Sagemaker_2017_07_24 Service
//!
//! Auto-generated service module for sagemaker_2017_07_24

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for sagemaker_2017_07_24
pub struct Sagemaker_2017_07_24Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Sagemaker_2017_07_24Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get monitoring_alert resource handler
    pub fn monitoring_alert(&self) -> resources::Monitoring_alert<'_> {
        resources::Monitoring_alert::new(self.provider)
    }
    /// Get pipeline_version resource handler
    pub fn pipeline_version(&self) -> resources::Pipeline_version<'_> {
        resources::Pipeline_version::new(self.provider)
    }
    /// Get devices resource handler
    pub fn devices(&self) -> resources::Devices<'_> {
        resources::Devices::new(self.provider)
    }
    /// Get human_task_ui resource handler
    pub fn human_task_ui(&self) -> resources::Human_task_ui<'_> {
        resources::Human_task_ui::new(self.provider)
    }
    /// Get image resource handler
    pub fn image(&self) -> resources::Image<'_> {
        resources::Image::new(self.provider)
    }
    /// Get model_package resource handler
    pub fn model_package(&self) -> resources::Model_package<'_> {
        resources::Model_package::new(self.provider)
    }
    /// Get studio_lifecycle_config resource handler
    pub fn studio_lifecycle_config(&self) -> resources::Studio_lifecycle_config<'_> {
        resources::Studio_lifecycle_config::new(self.provider)
    }
    /// Get model_bias_job_definition resource handler
    pub fn model_bias_job_definition(&self) -> resources::Model_bias_job_definition<'_> {
        resources::Model_bias_job_definition::new(self.provider)
    }
    /// Get flow_definition resource handler
    pub fn flow_definition(&self) -> resources::Flow_definition<'_> {
        resources::Flow_definition::new(self.provider)
    }
    /// Get inference_experiment resource handler
    pub fn inference_experiment(&self) -> resources::Inference_experiment<'_> {
        resources::Inference_experiment::new(self.provider)
    }
    /// Get training_plan resource handler
    pub fn training_plan(&self) -> resources::Training_plan<'_> {
        resources::Training_plan::new(self.provider)
    }
    /// Get processing_job resource handler
    pub fn processing_job(&self) -> resources::Processing_job<'_> {
        resources::Processing_job::new(self.provider)
    }
    /// Get hub_content_presigned_urls resource handler
    pub fn hub_content_presigned_urls(&self) -> resources::Hub_content_presigned_urls<'_> {
        resources::Hub_content_presigned_urls::new(self.provider)
    }
    /// Get feature_metadata resource handler
    pub fn feature_metadata(&self) -> resources::Feature_metadata<'_> {
        resources::Feature_metadata::new(self.provider)
    }
    /// Get hub_content resource handler
    pub fn hub_content(&self) -> resources::Hub_content<'_> {
        resources::Hub_content::new(self.provider)
    }
    /// Get tags resource handler
    pub fn tags(&self) -> resources::Tags<'_> {
        resources::Tags::new(self.provider)
    }
    /// Get partner_app_presigned_url resource handler
    pub fn partner_app_presigned_url(&self) -> resources::Partner_app_presigned_url<'_> {
        resources::Partner_app_presigned_url::new(self.provider)
    }
    /// Get pipeline_execution resource handler
    pub fn pipeline_execution(&self) -> resources::Pipeline_execution<'_> {
        resources::Pipeline_execution::new(self.provider)
    }
    /// Get hub_content_reference resource handler
    pub fn hub_content_reference(&self) -> resources::Hub_content_reference<'_> {
        resources::Hub_content_reference::new(self.provider)
    }
    /// Get app_image_config resource handler
    pub fn app_image_config(&self) -> resources::App_image_config<'_> {
        resources::App_image_config::new(self.provider)
    }
    /// Get monitoring_schedule resource handler
    pub fn monitoring_schedule(&self) -> resources::Monitoring_schedule<'_> {
        resources::Monitoring_schedule::new(self.provider)
    }
    /// Get optimization_job resource handler
    pub fn optimization_job(&self) -> resources::Optimization_job<'_> {
        resources::Optimization_job::new(self.provider)
    }
    /// Get presigned_notebook_instance_url resource handler
    pub fn presigned_notebook_instance_url(&self) -> resources::Presigned_notebook_instance_url<'_> {
        resources::Presigned_notebook_instance_url::new(self.provider)
    }
    /// Get trial resource handler
    pub fn trial(&self) -> resources::Trial<'_> {
        resources::Trial::new(self.provider)
    }
    /// Get cluster resource handler
    pub fn cluster(&self) -> resources::Cluster<'_> {
        resources::Cluster::new(self.provider)
    }
    /// Get space resource handler
    pub fn space(&self) -> resources::Space<'_> {
        resources::Space::new(self.provider)
    }
    /// Get workteam resource handler
    pub fn workteam(&self) -> resources::Workteam<'_> {
        resources::Workteam::new(self.provider)
    }
    /// Get cluster_node resource handler
    pub fn cluster_node(&self) -> resources::Cluster_node<'_> {
        resources::Cluster_node::new(self.provider)
    }
    /// Get app resource handler
    pub fn app(&self) -> resources::App<'_> {
        resources::App::new(self.provider)
    }
    /// Get labeling_job resource handler
    pub fn labeling_job(&self) -> resources::Labeling_job<'_> {
        resources::Labeling_job::new(self.provider)
    }
    /// Get edge_deployment_stage resource handler
    pub fn edge_deployment_stage(&self) -> resources::Edge_deployment_stage<'_> {
        resources::Edge_deployment_stage::new(self.provider)
    }
    /// Get presigned_domain_url resource handler
    pub fn presigned_domain_url(&self) -> resources::Presigned_domain_url<'_> {
        resources::Presigned_domain_url::new(self.provider)
    }
    /// Get model_package_group_policy resource handler
    pub fn model_package_group_policy(&self) -> resources::Model_package_group_policy<'_> {
        resources::Model_package_group_policy::new(self.provider)
    }
    /// Get algorithm resource handler
    pub fn algorithm(&self) -> resources::Algorithm<'_> {
        resources::Algorithm::new(self.provider)
    }
    /// Get lineage_group_policy resource handler
    pub fn lineage_group_policy(&self) -> resources::Lineage_group_policy<'_> {
        resources::Lineage_group_policy::new(self.provider)
    }
    /// Get search_suggestions resource handler
    pub fn search_suggestions(&self) -> resources::Search_suggestions<'_> {
        resources::Search_suggestions::new(self.provider)
    }
    /// Get action resource handler
    pub fn action(&self) -> resources::Action<'_> {
        resources::Action::new(self.provider)
    }
    /// Get edge_packaging_job resource handler
    pub fn edge_packaging_job(&self) -> resources::Edge_packaging_job<'_> {
        resources::Edge_packaging_job::new(self.provider)
    }
    /// Get compilation_job resource handler
    pub fn compilation_job(&self) -> resources::Compilation_job<'_> {
        resources::Compilation_job::new(self.provider)
    }
    /// Get image_version resource handler
    pub fn image_version(&self) -> resources::Image_version<'_> {
        resources::Image_version::new(self.provider)
    }
    /// Get model_card resource handler
    pub fn model_card(&self) -> resources::Model_card<'_> {
        resources::Model_card::new(self.provider)
    }
    /// Get cluster_scheduler_config resource handler
    pub fn cluster_scheduler_config(&self) -> resources::Cluster_scheduler_config<'_> {
        resources::Cluster_scheduler_config::new(self.provider)
    }
    /// Get project resource handler
    pub fn project(&self) -> resources::Project<'_> {
        resources::Project::new(self.provider)
    }
    /// Get artifact resource handler
    pub fn artifact(&self) -> resources::Artifact<'_> {
        resources::Artifact::new(self.provider)
    }
    /// Get endpoint resource handler
    pub fn endpoint(&self) -> resources::Endpoint<'_> {
        resources::Endpoint::new(self.provider)
    }
    /// Get presigned_mlflow_tracking_server_url resource handler
    pub fn presigned_mlflow_tracking_server_url(&self) -> resources::Presigned_mlflow_tracking_server_url<'_> {
        resources::Presigned_mlflow_tracking_server_url::new(self.provider)
    }
    /// Get device_fleet_report resource handler
    pub fn device_fleet_report(&self) -> resources::Device_fleet_report<'_> {
        resources::Device_fleet_report::new(self.provider)
    }
    /// Get notebook_instance_lifecycle_config resource handler
    pub fn notebook_instance_lifecycle_config(&self) -> resources::Notebook_instance_lifecycle_config<'_> {
        resources::Notebook_instance_lifecycle_config::new(self.provider)
    }
    /// Get data_quality_job_definition resource handler
    pub fn data_quality_job_definition(&self) -> resources::Data_quality_job_definition<'_> {
        resources::Data_quality_job_definition::new(self.provider)
    }
    /// Get mlflow_tracking_server resource handler
    pub fn mlflow_tracking_server(&self) -> resources::Mlflow_tracking_server<'_> {
        resources::Mlflow_tracking_server::new(self.provider)
    }
    /// Get domain resource handler
    pub fn domain(&self) -> resources::Domain<'_> {
        resources::Domain::new(self.provider)
    }
    /// Get auto_ml_job_v2 resource handler
    pub fn auto_ml_job_v2(&self) -> resources::Auto_ml_job_v2<'_> {
        resources::Auto_ml_job_v2::new(self.provider)
    }
    /// Get hyper_parameter_tuning_job resource handler
    pub fn hyper_parameter_tuning_job(&self) -> resources::Hyper_parameter_tuning_job<'_> {
        resources::Hyper_parameter_tuning_job::new(self.provider)
    }
    /// Get feature_group resource handler
    pub fn feature_group(&self) -> resources::Feature_group<'_> {
        resources::Feature_group::new(self.provider)
    }
    /// Get hub resource handler
    pub fn hub(&self) -> resources::Hub<'_> {
        resources::Hub::new(self.provider)
    }
    /// Get inference_component resource handler
    pub fn inference_component(&self) -> resources::Inference_component<'_> {
        resources::Inference_component::new(self.provider)
    }
    /// Get model_card_export_job resource handler
    pub fn model_card_export_job(&self) -> resources::Model_card_export_job<'_> {
        resources::Model_card_export_job::new(self.provider)
    }
    /// Get notebook_instance resource handler
    pub fn notebook_instance(&self) -> resources::Notebook_instance<'_> {
        resources::Notebook_instance::new(self.provider)
    }
    /// Get inference_recommendations_job resource handler
    pub fn inference_recommendations_job(&self) -> resources::Inference_recommendations_job<'_> {
        resources::Inference_recommendations_job::new(self.provider)
    }
    /// Get partner_app resource handler
    pub fn partner_app(&self) -> resources::Partner_app<'_> {
        resources::Partner_app::new(self.provider)
    }
    /// Get code_repository resource handler
    pub fn code_repository(&self) -> resources::Code_repository<'_> {
        resources::Code_repository::new(self.provider)
    }
    /// Get endpoint_config resource handler
    pub fn endpoint_config(&self) -> resources::Endpoint_config<'_> {
        resources::Endpoint_config::new(self.provider)
    }
    /// Get experiment resource handler
    pub fn experiment(&self) -> resources::Experiment<'_> {
        resources::Experiment::new(self.provider)
    }
    /// Get cluster_software resource handler
    pub fn cluster_software(&self) -> resources::Cluster_software<'_> {
        resources::Cluster_software::new(self.provider)
    }
    /// Get workforce resource handler
    pub fn workforce(&self) -> resources::Workforce<'_> {
        resources::Workforce::new(self.provider)
    }
    /// Get inference_component_runtime_config resource handler
    pub fn inference_component_runtime_config(&self) -> resources::Inference_component_runtime_config<'_> {
        resources::Inference_component_runtime_config::new(self.provider)
    }
    /// Get model_package_group resource handler
    pub fn model_package_group(&self) -> resources::Model_package_group<'_> {
        resources::Model_package_group::new(self.provider)
    }
    /// Get cluster_event resource handler
    pub fn cluster_event(&self) -> resources::Cluster_event<'_> {
        resources::Cluster_event::new(self.provider)
    }
    /// Get reserved_capacity resource handler
    pub fn reserved_capacity(&self) -> resources::Reserved_capacity<'_> {
        resources::Reserved_capacity::new(self.provider)
    }
    /// Get context resource handler
    pub fn context(&self) -> resources::Context<'_> {
        resources::Context::new(self.provider)
    }
    /// Get device_fleet resource handler
    pub fn device_fleet(&self) -> resources::Device_fleet<'_> {
        resources::Device_fleet::new(self.provider)
    }
    /// Get scaling_configuration_recommendation resource handler
    pub fn scaling_configuration_recommendation(&self) -> resources::Scaling_configuration_recommendation<'_> {
        resources::Scaling_configuration_recommendation::new(self.provider)
    }
    /// Get pipeline resource handler
    pub fn pipeline(&self) -> resources::Pipeline<'_> {
        resources::Pipeline::new(self.provider)
    }
    /// Get model_quality_job_definition resource handler
    pub fn model_quality_job_definition(&self) -> resources::Model_quality_job_definition<'_> {
        resources::Model_quality_job_definition::new(self.provider)
    }
    /// Get edge_deployment_plan resource handler
    pub fn edge_deployment_plan(&self) -> resources::Edge_deployment_plan<'_> {
        resources::Edge_deployment_plan::new(self.provider)
    }
    /// Get training_job resource handler
    pub fn training_job(&self) -> resources::Training_job<'_> {
        resources::Training_job::new(self.provider)
    }
    /// Get association resource handler
    pub fn association(&self) -> resources::Association<'_> {
        resources::Association::new(self.provider)
    }
    /// Get subscribed_workteam resource handler
    pub fn subscribed_workteam(&self) -> resources::Subscribed_workteam<'_> {
        resources::Subscribed_workteam::new(self.provider)
    }
    /// Get user_profile resource handler
    pub fn user_profile(&self) -> resources::User_profile<'_> {
        resources::User_profile::new(self.provider)
    }
    /// Get auto_ml_job resource handler
    pub fn auto_ml_job(&self) -> resources::Auto_ml_job<'_> {
        resources::Auto_ml_job::new(self.provider)
    }
    /// Get endpoint_weights_and_capacities resource handler
    pub fn endpoint_weights_and_capacities(&self) -> resources::Endpoint_weights_and_capacities<'_> {
        resources::Endpoint_weights_and_capacities::new(self.provider)
    }
    /// Get model_explainability_job_definition resource handler
    pub fn model_explainability_job_definition(&self) -> resources::Model_explainability_job_definition<'_> {
        resources::Model_explainability_job_definition::new(self.provider)
    }
    /// Get compute_quota resource handler
    pub fn compute_quota(&self) -> resources::Compute_quota<'_> {
        resources::Compute_quota::new(self.provider)
    }
    /// Get lineage_group resource handler
    pub fn lineage_group(&self) -> resources::Lineage_group<'_> {
        resources::Lineage_group::new(self.provider)
    }
    /// Get pipeline_definition_for_execution resource handler
    pub fn pipeline_definition_for_execution(&self) -> resources::Pipeline_definition_for_execution<'_> {
        resources::Pipeline_definition_for_execution::new(self.provider)
    }
    /// Get model resource handler
    pub fn model(&self) -> resources::Model<'_> {
        resources::Model::new(self.provider)
    }
    /// Get device resource handler
    pub fn device(&self) -> resources::Device<'_> {
        resources::Device::new(self.provider)
    }
    /// Get trial_component resource handler
    pub fn trial_component(&self) -> resources::Trial_component<'_> {
        resources::Trial_component::new(self.provider)
    }
    /// Get sagemaker_servicecatalog_portfolio_status resource handler
    pub fn sagemaker_servicecatalog_portfolio_status(&self) -> resources::Sagemaker_servicecatalog_portfolio_status<'_> {
        resources::Sagemaker_servicecatalog_portfolio_status::new(self.provider)
    }
    /// Get transform_job resource handler
    pub fn transform_job(&self) -> resources::Transform_job<'_> {
        resources::Transform_job::new(self.provider)
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
