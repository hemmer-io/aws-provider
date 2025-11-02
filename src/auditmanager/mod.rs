//! Auditmanager service for Aws provider
//!
//! This module handles all auditmanager resources and their CRUD operations.

use hemmer_core::Result;
use hemmer_provider::{ResourceInput, ResourceOutput, ResourcePlan};

/// Auditmanager service handler
pub struct AuditmanagerService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> AuditmanagerService<'a> {
    /// Create a new service handler
    pub fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Plan changes to a resource
    pub async fn plan_resource(
        &self,
        resource_name: &str,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        match resource_name {
            "assessment_framework_share" => {
                self.plan_assessment_framework_share(current_state, desired_input)
                    .await
            }
            "insights" => self.plan_insights(current_state, desired_input).await,
            "evidence_folders_by_assessment_control" => {
                self.plan_evidence_folders_by_assessment_control(current_state, desired_input)
                    .await
            }
            "assessment_report" => {
                self.plan_assessment_report(current_state, desired_input)
                    .await
            }
            "evidence" => self.plan_evidence(current_state, desired_input).await,
            "insights_by_assessment" => {
                self.plan_insights_by_assessment(current_state, desired_input)
                    .await
            }
            "change_logs" => self.plan_change_logs(current_state, desired_input).await,
            "evidence_folders_by_assessment" => {
                self.plan_evidence_folders_by_assessment(current_state, desired_input)
                    .await
            }
            "organization_admin_account" => {
                self.plan_organization_admin_account(current_state, desired_input)
                    .await
            }
            "assessment_framework" => {
                self.plan_assessment_framework(current_state, desired_input)
                    .await
            }
            "evidence_folder" => {
                self.plan_evidence_folder(current_state, desired_input)
                    .await
            }
            "evidence_file_upload_url" => {
                self.plan_evidence_file_upload_url(current_state, desired_input)
                    .await
            }
            "control" => self.plan_control(current_state, desired_input).await,
            "assessment_control_set_status" => {
                self.plan_assessment_control_set_status(current_state, desired_input)
                    .await
            }
            "delegations" => self.plan_delegations(current_state, desired_input).await,
            "settings" => self.plan_settings(current_state, desired_input).await,
            "assessment_control" => {
                self.plan_assessment_control(current_state, desired_input)
                    .await
            }
            "assessment_status" => {
                self.plan_assessment_status(current_state, desired_input)
                    .await
            }
            "assessment_report_url" => {
                self.plan_assessment_report_url(current_state, desired_input)
                    .await
            }
            "account_status" => self.plan_account_status(current_state, desired_input).await,
            "services_in_scope" => {
                self.plan_services_in_scope(current_state, desired_input)
                    .await
            }
            "evidence_by_evidence_folder" => {
                self.plan_evidence_by_evidence_folder(current_state, desired_input)
                    .await
            }
            "assessment" => self.plan_assessment(current_state, desired_input).await,
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "auditmanager", resource_name
            ))),
        }
    }

    /// Create a new resource
    pub async fn create_resource(
        &self,
        resource_name: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        match resource_name {
            "assessment_framework_share" => self.create_assessment_framework_share(input).await,
            "insights" => self.create_insights(input).await,
            "evidence_folders_by_assessment_control" => {
                self.create_evidence_folders_by_assessment_control(input)
                    .await
            }
            "assessment_report" => self.create_assessment_report(input).await,
            "evidence" => self.create_evidence(input).await,
            "insights_by_assessment" => self.create_insights_by_assessment(input).await,
            "change_logs" => self.create_change_logs(input).await,
            "evidence_folders_by_assessment" => {
                self.create_evidence_folders_by_assessment(input).await
            }
            "organization_admin_account" => self.create_organization_admin_account(input).await,
            "assessment_framework" => self.create_assessment_framework(input).await,
            "evidence_folder" => self.create_evidence_folder(input).await,
            "evidence_file_upload_url" => self.create_evidence_file_upload_url(input).await,
            "control" => self.create_control(input).await,
            "assessment_control_set_status" => {
                self.create_assessment_control_set_status(input).await
            }
            "delegations" => self.create_delegations(input).await,
            "settings" => self.create_settings(input).await,
            "assessment_control" => self.create_assessment_control(input).await,
            "assessment_status" => self.create_assessment_status(input).await,
            "assessment_report_url" => self.create_assessment_report_url(input).await,
            "account_status" => self.create_account_status(input).await,
            "services_in_scope" => self.create_services_in_scope(input).await,
            "evidence_by_evidence_folder" => self.create_evidence_by_evidence_folder(input).await,
            "assessment" => self.create_assessment(input).await,
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "auditmanager", resource_name
            ))),
        }
    }

    /// Read resource state
    pub async fn read_resource(&self, resource_name: &str, id: &str) -> Result<ResourceOutput> {
        match resource_name {
            "assessment_framework_share" => self.read_assessment_framework_share(id).await,
            "insights" => self.read_insights(id).await,
            "evidence_folders_by_assessment_control" => {
                self.read_evidence_folders_by_assessment_control(id).await
            }
            "assessment_report" => self.read_assessment_report(id).await,
            "evidence" => self.read_evidence(id).await,
            "insights_by_assessment" => self.read_insights_by_assessment(id).await,
            "change_logs" => self.read_change_logs(id).await,
            "evidence_folders_by_assessment" => self.read_evidence_folders_by_assessment(id).await,
            "organization_admin_account" => self.read_organization_admin_account(id).await,
            "assessment_framework" => self.read_assessment_framework(id).await,
            "evidence_folder" => self.read_evidence_folder(id).await,
            "evidence_file_upload_url" => self.read_evidence_file_upload_url(id).await,
            "control" => self.read_control(id).await,
            "assessment_control_set_status" => self.read_assessment_control_set_status(id).await,
            "delegations" => self.read_delegations(id).await,
            "settings" => self.read_settings(id).await,
            "assessment_control" => self.read_assessment_control(id).await,
            "assessment_status" => self.read_assessment_status(id).await,
            "assessment_report_url" => self.read_assessment_report_url(id).await,
            "account_status" => self.read_account_status(id).await,
            "services_in_scope" => self.read_services_in_scope(id).await,
            "evidence_by_evidence_folder" => self.read_evidence_by_evidence_folder(id).await,
            "assessment" => self.read_assessment(id).await,
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "auditmanager", resource_name
            ))),
        }
    }

    /// Update an existing resource
    pub async fn update_resource(
        &self,
        resource_name: &str,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        match resource_name {
            "assessment_framework_share" => self.update_assessment_framework_share(id, input).await,
            "insights" => self.update_insights(id, input).await,
            "evidence_folders_by_assessment_control" => {
                self.update_evidence_folders_by_assessment_control(id, input)
                    .await
            }
            "assessment_report" => self.update_assessment_report(id, input).await,
            "evidence" => self.update_evidence(id, input).await,
            "insights_by_assessment" => self.update_insights_by_assessment(id, input).await,
            "change_logs" => self.update_change_logs(id, input).await,
            "evidence_folders_by_assessment" => {
                self.update_evidence_folders_by_assessment(id, input).await
            }
            "organization_admin_account" => self.update_organization_admin_account(id, input).await,
            "assessment_framework" => self.update_assessment_framework(id, input).await,
            "evidence_folder" => self.update_evidence_folder(id, input).await,
            "evidence_file_upload_url" => self.update_evidence_file_upload_url(id, input).await,
            "control" => self.update_control(id, input).await,
            "assessment_control_set_status" => {
                self.update_assessment_control_set_status(id, input).await
            }
            "delegations" => self.update_delegations(id, input).await,
            "settings" => self.update_settings(id, input).await,
            "assessment_control" => self.update_assessment_control(id, input).await,
            "assessment_status" => self.update_assessment_status(id, input).await,
            "assessment_report_url" => self.update_assessment_report_url(id, input).await,
            "account_status" => self.update_account_status(id, input).await,
            "services_in_scope" => self.update_services_in_scope(id, input).await,
            "evidence_by_evidence_folder" => {
                self.update_evidence_by_evidence_folder(id, input).await
            }
            "assessment" => self.update_assessment(id, input).await,
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "auditmanager", resource_name
            ))),
        }
    }

    /// Delete a resource
    pub async fn delete_resource(&self, resource_name: &str, id: &str) -> Result<()> {
        match resource_name {
            "assessment_framework_share" => self.delete_assessment_framework_share(id).await,
            "insights" => self.delete_insights(id).await,
            "evidence_folders_by_assessment_control" => {
                self.delete_evidence_folders_by_assessment_control(id).await
            }
            "assessment_report" => self.delete_assessment_report(id).await,
            "evidence" => self.delete_evidence(id).await,
            "insights_by_assessment" => self.delete_insights_by_assessment(id).await,
            "change_logs" => self.delete_change_logs(id).await,
            "evidence_folders_by_assessment" => {
                self.delete_evidence_folders_by_assessment(id).await
            }
            "organization_admin_account" => self.delete_organization_admin_account(id).await,
            "assessment_framework" => self.delete_assessment_framework(id).await,
            "evidence_folder" => self.delete_evidence_folder(id).await,
            "evidence_file_upload_url" => self.delete_evidence_file_upload_url(id).await,
            "control" => self.delete_control(id).await,
            "assessment_control_set_status" => self.delete_assessment_control_set_status(id).await,
            "delegations" => self.delete_delegations(id).await,
            "settings" => self.delete_settings(id).await,
            "assessment_control" => self.delete_assessment_control(id).await,
            "assessment_status" => self.delete_assessment_status(id).await,
            "assessment_report_url" => self.delete_assessment_report_url(id).await,
            "account_status" => self.delete_account_status(id).await,
            "services_in_scope" => self.delete_services_in_scope(id).await,
            "evidence_by_evidence_folder" => self.delete_evidence_by_evidence_folder(id).await,
            "assessment" => self.delete_assessment(id).await,
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "auditmanager", resource_name
            ))),
        }
    }

    // ========================================================================
    // Resource-specific CRUD implementations
    // ========================================================================

    // ------------------------------------------------------------------------
    // Assessment_framework_share resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a assessment_framework_share resource
    async fn plan_assessment_framework_share(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new assessment_framework_share resource
    async fn create_assessment_framework_share(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let request_id = input.get_string("request_id")?;
            let request_type = input.get_string("request_type")?;
            let action = input.get_string("action")?;

            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.auditmanager_client
            //     .create_assessment_framework_share()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("request_id", request_id.unwrap_or_default())
                .with_field("request_type", request_type.unwrap_or_default())
                .with_field("action", action.unwrap_or_default()))
        })
    }

    /// Read a assessment_framework_share resource
    async fn read_assessment_framework_share(&self, id: &str) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.auditmanager_client
            //     .describe_assessment_framework_share()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Update a assessment_framework_share resource
    async fn update_assessment_framework_share(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let request_id = input.get_string("request_id")?;
            let request_type = input.get_string("request_type")?;
            let action = input.get_string("action")?;

            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.auditmanager_client
            //     .update_assessment_framework_share()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("request_id", request_id.unwrap_or_default())
                .with_field("request_type", request_type.unwrap_or_default())
                .with_field("action", action.unwrap_or_default()))
        })
    }

    /// Delete a assessment_framework_share resource
    async fn delete_assessment_framework_share(&self, id: &str) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.auditmanager_client
            //     .delete_assessment_framework_share()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }

    // ------------------------------------------------------------------------
    // Insights resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a insights resource
    async fn plan_insights(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new insights resource
    async fn create_insights(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields

            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.auditmanager_client
            //     .create_insights()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id("placeholder-id"))
        })
    }

    /// Read a insights resource
    async fn read_insights(&self, id: &str) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.auditmanager_client
            //     .describe_insights()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Update a insights resource
    async fn update_insights(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields

            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.auditmanager_client
            //     .update_insights()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Delete a insights resource
    async fn delete_insights(&self, id: &str) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.auditmanager_client
            //     .delete_insights()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }

    // ------------------------------------------------------------------------
    // Evidence_folders_by_assessment_control resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a evidence_folders_by_assessment_control resource
    async fn plan_evidence_folders_by_assessment_control(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new evidence_folders_by_assessment_control resource
    async fn create_evidence_folders_by_assessment_control(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields

            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.auditmanager_client
            //     .create_evidence_folders_by_assessment_control()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id("placeholder-id"))
        })
    }

    /// Read a evidence_folders_by_assessment_control resource
    async fn read_evidence_folders_by_assessment_control(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.auditmanager_client
            //     .describe_evidence_folders_by_assessment_control()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Update a evidence_folders_by_assessment_control resource
    async fn update_evidence_folders_by_assessment_control(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields

            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.auditmanager_client
            //     .update_evidence_folders_by_assessment_control()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Delete a evidence_folders_by_assessment_control resource
    async fn delete_evidence_folders_by_assessment_control(&self, id: &str) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.auditmanager_client
            //     .delete_evidence_folders_by_assessment_control()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }

    // ------------------------------------------------------------------------
    // Assessment_report resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a assessment_report resource
    async fn plan_assessment_report(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new assessment_report resource
    async fn create_assessment_report(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let description = input.get_optional_string("description")?;
            let query_statement = input.get_optional_string("query_statement")?;
            let assessment_id = input.get_string("assessment_id")?;
            let name = input.get_string("name")?;

            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.auditmanager_client
            //     .create_assessment_report()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("description", description.unwrap_or_default())
                .with_field("query_statement", query_statement.unwrap_or_default())
                .with_field("assessment_id", assessment_id.unwrap_or_default())
                .with_field("name", name.unwrap_or_default()))
        })
    }

    /// Read a assessment_report resource
    async fn read_assessment_report(&self, id: &str) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.auditmanager_client
            //     .describe_assessment_report()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Update a assessment_report resource
    async fn update_assessment_report(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let description = input.get_optional_string("description")?;
            let query_statement = input.get_optional_string("query_statement")?;
            let assessment_id = input.get_string("assessment_id")?;
            let name = input.get_string("name")?;

            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.auditmanager_client
            //     .update_assessment_report()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("description", description.unwrap_or_default())
                .with_field("query_statement", query_statement.unwrap_or_default())
                .with_field("assessment_id", assessment_id.unwrap_or_default())
                .with_field("name", name.unwrap_or_default()))
        })
    }

    /// Delete a assessment_report resource
    async fn delete_assessment_report(&self, id: &str) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.auditmanager_client
            //     .delete_assessment_report()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }

    // ------------------------------------------------------------------------
    // Evidence resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a evidence resource
    async fn plan_evidence(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new evidence resource
    async fn create_evidence(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields

            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.auditmanager_client
            //     .create_evidence()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id("placeholder-id"))
        })
    }

    /// Read a evidence resource
    async fn read_evidence(&self, id: &str) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.auditmanager_client
            //     .describe_evidence()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Update a evidence resource
    async fn update_evidence(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields

            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.auditmanager_client
            //     .update_evidence()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Delete a evidence resource
    async fn delete_evidence(&self, id: &str) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.auditmanager_client
            //     .delete_evidence()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }

    // ------------------------------------------------------------------------
    // Insights_by_assessment resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a insights_by_assessment resource
    async fn plan_insights_by_assessment(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new insights_by_assessment resource
    async fn create_insights_by_assessment(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields

            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.auditmanager_client
            //     .create_insights_by_assessment()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id("placeholder-id"))
        })
    }

    /// Read a insights_by_assessment resource
    async fn read_insights_by_assessment(&self, id: &str) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.auditmanager_client
            //     .describe_insights_by_assessment()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Update a insights_by_assessment resource
    async fn update_insights_by_assessment(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields

            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.auditmanager_client
            //     .update_insights_by_assessment()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Delete a insights_by_assessment resource
    async fn delete_insights_by_assessment(&self, id: &str) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.auditmanager_client
            //     .delete_insights_by_assessment()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }

    // ------------------------------------------------------------------------
    // Change_logs resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a change_logs resource
    async fn plan_change_logs(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new change_logs resource
    async fn create_change_logs(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields

            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.auditmanager_client
            //     .create_change_logs()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id("placeholder-id"))
        })
    }

    /// Read a change_logs resource
    async fn read_change_logs(&self, id: &str) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.auditmanager_client
            //     .describe_change_logs()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Update a change_logs resource
    async fn update_change_logs(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields

            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.auditmanager_client
            //     .update_change_logs()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Delete a change_logs resource
    async fn delete_change_logs(&self, id: &str) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.auditmanager_client
            //     .delete_change_logs()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }

    // ------------------------------------------------------------------------
    // Evidence_folders_by_assessment resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a evidence_folders_by_assessment resource
    async fn plan_evidence_folders_by_assessment(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new evidence_folders_by_assessment resource
    async fn create_evidence_folders_by_assessment(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields

            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.auditmanager_client
            //     .create_evidence_folders_by_assessment()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id("placeholder-id"))
        })
    }

    /// Read a evidence_folders_by_assessment resource
    async fn read_evidence_folders_by_assessment(&self, id: &str) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.auditmanager_client
            //     .describe_evidence_folders_by_assessment()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Update a evidence_folders_by_assessment resource
    async fn update_evidence_folders_by_assessment(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields

            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.auditmanager_client
            //     .update_evidence_folders_by_assessment()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Delete a evidence_folders_by_assessment resource
    async fn delete_evidence_folders_by_assessment(&self, id: &str) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.auditmanager_client
            //     .delete_evidence_folders_by_assessment()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }

    // ------------------------------------------------------------------------
    // Organization_admin_account resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a organization_admin_account resource
    async fn plan_organization_admin_account(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new organization_admin_account resource
    async fn create_organization_admin_account(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields

            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.auditmanager_client
            //     .create_organization_admin_account()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id("placeholder-id"))
        })
    }

    /// Read a organization_admin_account resource
    async fn read_organization_admin_account(&self, id: &str) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.auditmanager_client
            //     .describe_organization_admin_account()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Update a organization_admin_account resource
    async fn update_organization_admin_account(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields

            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.auditmanager_client
            //     .update_organization_admin_account()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Delete a organization_admin_account resource
    async fn delete_organization_admin_account(&self, id: &str) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.auditmanager_client
            //     .delete_organization_admin_account()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }

    // ------------------------------------------------------------------------
    // Assessment_framework resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a assessment_framework resource
    async fn plan_assessment_framework(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new assessment_framework resource
    async fn create_assessment_framework(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let tags = input.get_optional_string("tags")?;
            let compliance_type = input.get_optional_string("compliance_type")?;
            let name = input.get_string("name")?;
            let control_sets = input.get_string("control_sets")?;
            let description = input.get_optional_string("description")?;

            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.auditmanager_client
            //     .create_assessment_framework()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("tags", tags.unwrap_or_default())
                .with_field("compliance_type", compliance_type.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
                .with_field("control_sets", control_sets.unwrap_or_default())
                .with_field("description", description.unwrap_or_default()))
        })
    }

    /// Read a assessment_framework resource
    async fn read_assessment_framework(&self, id: &str) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.auditmanager_client
            //     .describe_assessment_framework()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Update a assessment_framework resource
    async fn update_assessment_framework(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let tags = input.get_optional_string("tags")?;
            let compliance_type = input.get_optional_string("compliance_type")?;
            let name = input.get_string("name")?;
            let control_sets = input.get_string("control_sets")?;
            let description = input.get_optional_string("description")?;

            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.auditmanager_client
            //     .update_assessment_framework()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("tags", tags.unwrap_or_default())
                .with_field("compliance_type", compliance_type.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
                .with_field("control_sets", control_sets.unwrap_or_default())
                .with_field("description", description.unwrap_or_default()))
        })
    }

    /// Delete a assessment_framework resource
    async fn delete_assessment_framework(&self, id: &str) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.auditmanager_client
            //     .delete_assessment_framework()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }

    // ------------------------------------------------------------------------
    // Evidence_folder resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a evidence_folder resource
    async fn plan_evidence_folder(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new evidence_folder resource
    async fn create_evidence_folder(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields

            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.auditmanager_client
            //     .create_evidence_folder()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id("placeholder-id"))
        })
    }

    /// Read a evidence_folder resource
    async fn read_evidence_folder(&self, id: &str) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.auditmanager_client
            //     .describe_evidence_folder()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Update a evidence_folder resource
    async fn update_evidence_folder(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields

            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.auditmanager_client
            //     .update_evidence_folder()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Delete a evidence_folder resource
    async fn delete_evidence_folder(&self, id: &str) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.auditmanager_client
            //     .delete_evidence_folder()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }

    // ------------------------------------------------------------------------
    // Evidence_file_upload_url resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a evidence_file_upload_url resource
    async fn plan_evidence_file_upload_url(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new evidence_file_upload_url resource
    async fn create_evidence_file_upload_url(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields

            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.auditmanager_client
            //     .create_evidence_file_upload_url()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id("placeholder-id"))
        })
    }

    /// Read a evidence_file_upload_url resource
    async fn read_evidence_file_upload_url(&self, id: &str) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.auditmanager_client
            //     .describe_evidence_file_upload_url()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Update a evidence_file_upload_url resource
    async fn update_evidence_file_upload_url(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields

            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.auditmanager_client
            //     .update_evidence_file_upload_url()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Delete a evidence_file_upload_url resource
    async fn delete_evidence_file_upload_url(&self, id: &str) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.auditmanager_client
            //     .delete_evidence_file_upload_url()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }

    // ------------------------------------------------------------------------
    // Control resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a control resource
    async fn plan_control(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new control resource
    async fn create_control(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let tags = input.get_optional_string("tags")?;
            let description = input.get_optional_string("description")?;
            let action_plan_instructions = input.get_optional_string("action_plan_instructions")?;
            let testing_information = input.get_optional_string("testing_information")?;
            let name = input.get_string("name")?;
            let action_plan_title = input.get_optional_string("action_plan_title")?;
            let control_mapping_sources = input.get_string("control_mapping_sources")?;

            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.auditmanager_client
            //     .create_control()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("tags", tags.unwrap_or_default())
                .with_field("description", description.unwrap_or_default())
                .with_field(
                    "action_plan_instructions",
                    action_plan_instructions.unwrap_or_default(),
                )
                .with_field(
                    "testing_information",
                    testing_information.unwrap_or_default(),
                )
                .with_field("name", name.unwrap_or_default())
                .with_field("action_plan_title", action_plan_title.unwrap_or_default())
                .with_field(
                    "control_mapping_sources",
                    control_mapping_sources.unwrap_or_default(),
                ))
        })
    }

    /// Read a control resource
    async fn read_control(&self, id: &str) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.auditmanager_client
            //     .describe_control()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Update a control resource
    async fn update_control(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let tags = input.get_optional_string("tags")?;
            let description = input.get_optional_string("description")?;
            let action_plan_instructions = input.get_optional_string("action_plan_instructions")?;
            let testing_information = input.get_optional_string("testing_information")?;
            let name = input.get_string("name")?;
            let action_plan_title = input.get_optional_string("action_plan_title")?;
            let control_mapping_sources = input.get_string("control_mapping_sources")?;

            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.auditmanager_client
            //     .update_control()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("tags", tags.unwrap_or_default())
                .with_field("description", description.unwrap_or_default())
                .with_field(
                    "action_plan_instructions",
                    action_plan_instructions.unwrap_or_default(),
                )
                .with_field(
                    "testing_information",
                    testing_information.unwrap_or_default(),
                )
                .with_field("name", name.unwrap_or_default())
                .with_field("action_plan_title", action_plan_title.unwrap_or_default())
                .with_field(
                    "control_mapping_sources",
                    control_mapping_sources.unwrap_or_default(),
                ))
        })
    }

    /// Delete a control resource
    async fn delete_control(&self, id: &str) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.auditmanager_client
            //     .delete_control()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }

    // ------------------------------------------------------------------------
    // Assessment_control_set_status resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a assessment_control_set_status resource
    async fn plan_assessment_control_set_status(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new assessment_control_set_status resource
    async fn create_assessment_control_set_status(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let control_set_id = input.get_string("control_set_id")?;
            let comment = input.get_string("comment")?;
            let assessment_id = input.get_string("assessment_id")?;
            let status = input.get_string("status")?;

            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.auditmanager_client
            //     .create_assessment_control_set_status()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("control_set_id", control_set_id.unwrap_or_default())
                .with_field("comment", comment.unwrap_or_default())
                .with_field("assessment_id", assessment_id.unwrap_or_default())
                .with_field("status", status.unwrap_or_default()))
        })
    }

    /// Read a assessment_control_set_status resource
    async fn read_assessment_control_set_status(&self, id: &str) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.auditmanager_client
            //     .describe_assessment_control_set_status()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Update a assessment_control_set_status resource
    async fn update_assessment_control_set_status(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let control_set_id = input.get_string("control_set_id")?;
            let comment = input.get_string("comment")?;
            let assessment_id = input.get_string("assessment_id")?;
            let status = input.get_string("status")?;

            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.auditmanager_client
            //     .update_assessment_control_set_status()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("control_set_id", control_set_id.unwrap_or_default())
                .with_field("comment", comment.unwrap_or_default())
                .with_field("assessment_id", assessment_id.unwrap_or_default())
                .with_field("status", status.unwrap_or_default()))
        })
    }

    /// Delete a assessment_control_set_status resource
    async fn delete_assessment_control_set_status(&self, id: &str) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.auditmanager_client
            //     .delete_assessment_control_set_status()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }

    // ------------------------------------------------------------------------
    // Delegations resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a delegations resource
    async fn plan_delegations(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new delegations resource
    async fn create_delegations(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields

            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.auditmanager_client
            //     .create_delegations()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id("placeholder-id"))
        })
    }

    /// Read a delegations resource
    async fn read_delegations(&self, id: &str) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.auditmanager_client
            //     .describe_delegations()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Update a delegations resource
    async fn update_delegations(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields

            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.auditmanager_client
            //     .update_delegations()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Delete a delegations resource
    async fn delete_delegations(&self, id: &str) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.auditmanager_client
            //     .delete_delegations()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }

    // ------------------------------------------------------------------------
    // Settings resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a settings resource
    async fn plan_settings(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new settings resource
    async fn create_settings(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let sns_topic = input.get_optional_string("sns_topic")?;
            let default_assessment_reports_destination =
                input.get_optional_string("default_assessment_reports_destination")?;
            let kms_key = input.get_optional_string("kms_key")?;
            let default_process_owners = input.get_optional_string("default_process_owners")?;
            let evidence_finder_enabled = input.get_optional_string("evidence_finder_enabled")?;
            let deregistration_policy = input.get_optional_string("deregistration_policy")?;
            let default_export_destination =
                input.get_optional_string("default_export_destination")?;

            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.auditmanager_client
            //     .create_settings()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("sns_topic", sns_topic.unwrap_or_default())
                .with_field(
                    "default_assessment_reports_destination",
                    default_assessment_reports_destination.unwrap_or_default(),
                )
                .with_field("kms_key", kms_key.unwrap_or_default())
                .with_field(
                    "default_process_owners",
                    default_process_owners.unwrap_or_default(),
                )
                .with_field(
                    "evidence_finder_enabled",
                    evidence_finder_enabled.unwrap_or_default(),
                )
                .with_field(
                    "deregistration_policy",
                    deregistration_policy.unwrap_or_default(),
                )
                .with_field(
                    "default_export_destination",
                    default_export_destination.unwrap_or_default(),
                ))
        })
    }

    /// Read a settings resource
    async fn read_settings(&self, id: &str) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.auditmanager_client
            //     .describe_settings()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Update a settings resource
    async fn update_settings(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let sns_topic = input.get_optional_string("sns_topic")?;
            let default_assessment_reports_destination =
                input.get_optional_string("default_assessment_reports_destination")?;
            let kms_key = input.get_optional_string("kms_key")?;
            let default_process_owners = input.get_optional_string("default_process_owners")?;
            let evidence_finder_enabled = input.get_optional_string("evidence_finder_enabled")?;
            let deregistration_policy = input.get_optional_string("deregistration_policy")?;
            let default_export_destination =
                input.get_optional_string("default_export_destination")?;

            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.auditmanager_client
            //     .update_settings()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("sns_topic", sns_topic.unwrap_or_default())
                .with_field(
                    "default_assessment_reports_destination",
                    default_assessment_reports_destination.unwrap_or_default(),
                )
                .with_field("kms_key", kms_key.unwrap_or_default())
                .with_field(
                    "default_process_owners",
                    default_process_owners.unwrap_or_default(),
                )
                .with_field(
                    "evidence_finder_enabled",
                    evidence_finder_enabled.unwrap_or_default(),
                )
                .with_field(
                    "deregistration_policy",
                    deregistration_policy.unwrap_or_default(),
                )
                .with_field(
                    "default_export_destination",
                    default_export_destination.unwrap_or_default(),
                ))
        })
    }

    /// Delete a settings resource
    async fn delete_settings(&self, id: &str) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.auditmanager_client
            //     .delete_settings()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }

    // ------------------------------------------------------------------------
    // Assessment_control resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a assessment_control resource
    async fn plan_assessment_control(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new assessment_control resource
    async fn create_assessment_control(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let comment_body = input.get_optional_string("comment_body")?;
            let assessment_id = input.get_string("assessment_id")?;
            let control_id = input.get_string("control_id")?;
            let control_status = input.get_optional_string("control_status")?;
            let control_set_id = input.get_string("control_set_id")?;

            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.auditmanager_client
            //     .create_assessment_control()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("comment_body", comment_body.unwrap_or_default())
                .with_field("assessment_id", assessment_id.unwrap_or_default())
                .with_field("control_id", control_id.unwrap_or_default())
                .with_field("control_status", control_status.unwrap_or_default())
                .with_field("control_set_id", control_set_id.unwrap_or_default()))
        })
    }

    /// Read a assessment_control resource
    async fn read_assessment_control(&self, id: &str) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.auditmanager_client
            //     .describe_assessment_control()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Update a assessment_control resource
    async fn update_assessment_control(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let comment_body = input.get_optional_string("comment_body")?;
            let assessment_id = input.get_string("assessment_id")?;
            let control_id = input.get_string("control_id")?;
            let control_status = input.get_optional_string("control_status")?;
            let control_set_id = input.get_string("control_set_id")?;

            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.auditmanager_client
            //     .update_assessment_control()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("comment_body", comment_body.unwrap_or_default())
                .with_field("assessment_id", assessment_id.unwrap_or_default())
                .with_field("control_id", control_id.unwrap_or_default())
                .with_field("control_status", control_status.unwrap_or_default())
                .with_field("control_set_id", control_set_id.unwrap_or_default()))
        })
    }

    /// Delete a assessment_control resource
    async fn delete_assessment_control(&self, id: &str) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.auditmanager_client
            //     .delete_assessment_control()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }

    // ------------------------------------------------------------------------
    // Assessment_status resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a assessment_status resource
    async fn plan_assessment_status(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new assessment_status resource
    async fn create_assessment_status(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let assessment_id = input.get_string("assessment_id")?;
            let status = input.get_string("status")?;

            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.auditmanager_client
            //     .create_assessment_status()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("assessment_id", assessment_id.unwrap_or_default())
                .with_field("status", status.unwrap_or_default()))
        })
    }

    /// Read a assessment_status resource
    async fn read_assessment_status(&self, id: &str) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.auditmanager_client
            //     .describe_assessment_status()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Update a assessment_status resource
    async fn update_assessment_status(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let assessment_id = input.get_string("assessment_id")?;
            let status = input.get_string("status")?;

            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.auditmanager_client
            //     .update_assessment_status()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("assessment_id", assessment_id.unwrap_or_default())
                .with_field("status", status.unwrap_or_default()))
        })
    }

    /// Delete a assessment_status resource
    async fn delete_assessment_status(&self, id: &str) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.auditmanager_client
            //     .delete_assessment_status()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }

    // ------------------------------------------------------------------------
    // Assessment_report_url resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a assessment_report_url resource
    async fn plan_assessment_report_url(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new assessment_report_url resource
    async fn create_assessment_report_url(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields

            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.auditmanager_client
            //     .create_assessment_report_url()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id("placeholder-id"))
        })
    }

    /// Read a assessment_report_url resource
    async fn read_assessment_report_url(&self, id: &str) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.auditmanager_client
            //     .describe_assessment_report_url()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Update a assessment_report_url resource
    async fn update_assessment_report_url(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields

            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.auditmanager_client
            //     .update_assessment_report_url()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Delete a assessment_report_url resource
    async fn delete_assessment_report_url(&self, id: &str) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.auditmanager_client
            //     .delete_assessment_report_url()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }

    // ------------------------------------------------------------------------
    // Account_status resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a account_status resource
    async fn plan_account_status(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new account_status resource
    async fn create_account_status(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields

            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.auditmanager_client
            //     .create_account_status()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id("placeholder-id"))
        })
    }

    /// Read a account_status resource
    async fn read_account_status(&self, id: &str) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.auditmanager_client
            //     .describe_account_status()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Update a account_status resource
    async fn update_account_status(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields

            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.auditmanager_client
            //     .update_account_status()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Delete a account_status resource
    async fn delete_account_status(&self, id: &str) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.auditmanager_client
            //     .delete_account_status()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }

    // ------------------------------------------------------------------------
    // Services_in_scope resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a services_in_scope resource
    async fn plan_services_in_scope(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new services_in_scope resource
    async fn create_services_in_scope(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields

            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.auditmanager_client
            //     .create_services_in_scope()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id("placeholder-id"))
        })
    }

    /// Read a services_in_scope resource
    async fn read_services_in_scope(&self, id: &str) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.auditmanager_client
            //     .describe_services_in_scope()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Update a services_in_scope resource
    async fn update_services_in_scope(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields

            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.auditmanager_client
            //     .update_services_in_scope()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Delete a services_in_scope resource
    async fn delete_services_in_scope(&self, id: &str) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.auditmanager_client
            //     .delete_services_in_scope()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }

    // ------------------------------------------------------------------------
    // Evidence_by_evidence_folder resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a evidence_by_evidence_folder resource
    async fn plan_evidence_by_evidence_folder(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new evidence_by_evidence_folder resource
    async fn create_evidence_by_evidence_folder(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields

            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.auditmanager_client
            //     .create_evidence_by_evidence_folder()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id("placeholder-id"))
        })
    }

    /// Read a evidence_by_evidence_folder resource
    async fn read_evidence_by_evidence_folder(&self, id: &str) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.auditmanager_client
            //     .describe_evidence_by_evidence_folder()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Update a evidence_by_evidence_folder resource
    async fn update_evidence_by_evidence_folder(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields

            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.auditmanager_client
            //     .update_evidence_by_evidence_folder()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Delete a evidence_by_evidence_folder resource
    async fn delete_evidence_by_evidence_folder(&self, id: &str) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.auditmanager_client
            //     .delete_evidence_by_evidence_folder()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }

    // ------------------------------------------------------------------------
    // Assessment resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a assessment resource
    async fn plan_assessment(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new assessment resource
    async fn create_assessment(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let framework_id = input.get_string("framework_id")?;
            let roles = input.get_string("roles")?;
            let description = input.get_optional_string("description")?;
            let tags = input.get_optional_string("tags")?;
            let scope = input.get_string("scope")?;
            let assessment_reports_destination =
                input.get_string("assessment_reports_destination")?;
            let name = input.get_string("name")?;

            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.auditmanager_client
            //     .create_assessment()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("framework_id", framework_id.unwrap_or_default())
                .with_field("roles", roles.unwrap_or_default())
                .with_field("description", description.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("scope", scope.unwrap_or_default())
                .with_field(
                    "assessment_reports_destination",
                    assessment_reports_destination.unwrap_or_default(),
                )
                .with_field("name", name.unwrap_or_default()))
        })
    }

    /// Read a assessment resource
    async fn read_assessment(&self, id: &str) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.auditmanager_client
            //     .describe_assessment()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Update a assessment resource
    async fn update_assessment(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let framework_id = input.get_string("framework_id")?;
            let roles = input.get_string("roles")?;
            let description = input.get_optional_string("description")?;
            let tags = input.get_optional_string("tags")?;
            let scope = input.get_string("scope")?;
            let assessment_reports_destination =
                input.get_string("assessment_reports_destination")?;
            let name = input.get_string("name")?;

            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.auditmanager_client
            //     .update_assessment()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("framework_id", framework_id.unwrap_or_default())
                .with_field("roles", roles.unwrap_or_default())
                .with_field("description", description.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("scope", scope.unwrap_or_default())
                .with_field(
                    "assessment_reports_destination",
                    assessment_reports_destination.unwrap_or_default(),
                )
                .with_field("name", name.unwrap_or_default()))
        })
    }

    /// Delete a assessment resource
    async fn delete_assessment(&self, id: &str) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.auditmanager_client
            //     .delete_assessment()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }
}
