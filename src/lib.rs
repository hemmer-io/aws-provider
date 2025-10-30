//! Minimal AWS Provider for Hemmer
//!
//! This is a simplified AWS provider implementation that demonstrates
//! the ProviderExecutor trait with S3 and EC2 support.

use async_trait::async_trait;
use hemmer_core::{HemmerError, Result};
use hemmer_provider::{
    ProviderConfig, ProviderExecutor, ResourceInput, ResourceOutput, ResourcePlan,
};

/// AWS Provider implementation
pub struct AwsProvider {
    /// AWS SDK configuration
    config: Option<aws_types::SdkConfig>,
    /// S3 client
    s3_client: Option<aws_sdk_s3::Client>,
    /// EC2 client
    ec2_client: Option<aws_sdk_ec2::Client>,
}

impl AwsProvider {
    /// Create a new unconfigured AWS provider
    pub fn new() -> Self {
        Self {
            config: None,
            s3_client: None,
            ec2_client: None,
        }
    }
}

impl Default for AwsProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ProviderExecutor for AwsProvider {
    async fn configure(&mut self, config: ProviderConfig) -> Result<()> {
        tracing::info!("Configuring AWS provider");

        // Extract region from config (optional)
        let region = config.get_optional_string("region")?;

        // Build AWS SDK config
        let mut aws_config_builder = aws_config::defaults(aws_config::BehaviorVersion::latest());

        if let Some(region_str) = region {
            aws_config_builder = aws_config_builder.region(
                aws_types::region::Region::new(region_str)
            );
        }

        let sdk_config = aws_config_builder.load().await;

        // Initialize clients
        self.s3_client = Some(aws_sdk_s3::Client::new(&sdk_config));
        self.ec2_client = Some(aws_sdk_ec2::Client::new(&sdk_config));
        self.config = Some(sdk_config);

        tracing::info!("AWS provider configured successfully");
        Ok(())
    }

    async fn plan(
        &self,
        resource_type: &str,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        tracing::debug!(resource_type = %resource_type, "Planning resource changes");

        // If no current state, resource needs to be created
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // For now, simple comparison - if any field changed, update
        // TODO: Implement proper field-by-field comparison
        let current = current_state.unwrap();
        let mut has_changes = false;

        for (key, new_value) in &desired_input.fields {
            if let Some(current_value) = current.get_field(key) {
                if current_value != new_value {
                    has_changes = true;
                    break;
                }
            } else {
                has_changes = true;
                break;
            }
        }

        if has_changes {
            Ok(ResourcePlan::update(Vec::new())) // TODO: Add actual field changes
        } else {
            Ok(ResourcePlan::no_op())
        }
    }

    async fn create(&self, resource_type: &str, input: ResourceInput) -> Result<ResourceOutput> {
        tracing::info!(resource_type = %resource_type, "Creating resource");

        match resource_type {
            "bucket" => self.create_s3_bucket(input).await,
            "instance" => self.create_ec2_instance(input).await,
            _ => Err(HemmerError::Provider(format!(
                "Unknown resource type: {}",
                resource_type
            ))),
        }
    }

    async fn read(&self, resource_type: &str, id: &str) -> Result<ResourceOutput> {
        tracing::info!(resource_type = %resource_type, id = %id, "Reading resource");

        match resource_type {
            "bucket" => self.read_s3_bucket(id).await,
            "instance" => self.read_ec2_instance(id).await,
            _ => Err(HemmerError::Provider(format!(
                "Unknown resource type: {}",
                resource_type
            ))),
        }
    }

    async fn update(
        &self,
        resource_type: &str,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        tracing::info!(resource_type = %resource_type, id = %id, "Updating resource");

        match resource_type {
            "bucket" => self.update_s3_bucket(id, input).await,
            "instance" => self.update_ec2_instance(id, input).await,
            _ => Err(HemmerError::Provider(format!(
                "Unknown resource type: {}",
                resource_type
            ))),
        }
    }

    async fn delete(&self, resource_type: &str, id: &str) -> Result<()> {
        tracing::info!(resource_type = %resource_type, id = %id, "Deleting resource");

        match resource_type {
            "bucket" => self.delete_s3_bucket(id).await,
            "instance" => self.delete_ec2_instance(id).await,
            _ => Err(HemmerError::Provider(format!(
                "Unknown resource type: {}",
                resource_type
            ))),
        }
    }
}

// S3 Operations
impl AwsProvider {
    async fn create_s3_bucket(&self, input: ResourceInput) -> Result<ResourceOutput> {
        let client = self.s3_client.as_ref().ok_or_else(|| {
            HemmerError::Provider("Provider not configured".to_string())
        })?;

        let bucket_name = input.get_string("name")?;

        // Check if bucket already exists
        let head_result = client.head_bucket().bucket(&bucket_name).send().await;

        if head_result.is_ok() {
            return Err(HemmerError::Provider(format!(
                "Bucket '{}' already exists",
                bucket_name
            )));
        }

        // Create bucket
        client
            .create_bucket()
            .bucket(&bucket_name)
            .send()
            .await
            .map_err(|e| HemmerError::Provider(format!("Failed to create S3 bucket: {}", e)))?;

        // Return output
        Ok(ResourceOutput::new()
            .with_id(bucket_name.clone())
            .with_field("name", bucket_name.clone())
            .with_field("arn", format!("arn:aws:s3:::{}", bucket_name)))
    }

    async fn read_s3_bucket(&self, id: &str) -> Result<ResourceOutput> {
        let client = self.s3_client.as_ref().ok_or_else(|| {
            HemmerError::Provider("Provider not configured".to_string())
        })?;

        // Head bucket to check existence
        client
            .head_bucket()
            .bucket(id)
            .send()
            .await
            .map_err(|e| HemmerError::Provider(format!("Failed to read S3 bucket: {}", e)))?;

        // Return basic output
        Ok(ResourceOutput::new()
            .with_id(id.to_string())
            .with_field("name", id.to_string())
            .with_field("arn", format!("arn:aws:s3:::{}", id)))
    }

    async fn update_s3_bucket(&self, id: &str, _input: ResourceInput) -> Result<ResourceOutput> {
        // S3 buckets have limited update operations
        // For now, just return current state
        self.read_s3_bucket(id).await
    }

    async fn delete_s3_bucket(&self, id: &str) -> Result<()> {
        let client = self.s3_client.as_ref().ok_or_else(|| {
            HemmerError::Provider("Provider not configured".to_string())
        })?;

        client
            .delete_bucket()
            .bucket(id)
            .send()
            .await
            .map_err(|e| HemmerError::Provider(format!("Failed to delete S3 bucket: {}", e)))?;

        Ok(())
    }
}

// EC2 Operations
impl AwsProvider {
    async fn create_ec2_instance(&self, input: ResourceInput) -> Result<ResourceOutput> {
        let client = self.ec2_client.as_ref().ok_or_else(|| {
            HemmerError::Provider("Provider not configured".to_string())
        })?;

        let ami_id = input.get_string("ami")?;
        let instance_type = input.get_string("instance_type")?;

        // Run instance
        let result = client
            .run_instances()
            .image_id(ami_id)
            .instance_type(aws_sdk_ec2::types::InstanceType::from(instance_type.as_str()))
            .min_count(1)
            .max_count(1)
            .send()
            .await
            .map_err(|e| HemmerError::Provider(format!("Failed to create EC2 instance: {}", e)))?;

        let instance = result.instances()
            .first()
            .ok_or_else(|| HemmerError::Provider("No instance returned".to_string()))?;

        let instance_id = instance.instance_id()
            .ok_or_else(|| HemmerError::Provider("No instance ID returned".to_string()))?;

        Ok(ResourceOutput::new()
            .with_id(instance_id.to_string())
            .with_field("instance_id", instance_id.to_string())
            .with_field("ami", input.get_string("ami")?))
    }

    async fn read_ec2_instance(&self, id: &str) -> Result<ResourceOutput> {
        let client = self.ec2_client.as_ref().ok_or_else(|| {
            HemmerError::Provider("Provider not configured".to_string())
        })?;

        let result = client
            .describe_instances()
            .instance_ids(id)
            .send()
            .await
            .map_err(|e| HemmerError::Provider(format!("Failed to read EC2 instance: {}", e)))?;

        let instance = result.reservations()
            .first()
            .and_then(|r| r.instances().first())
            .ok_or_else(|| HemmerError::Provider(format!("Instance {} not found", id)))?;

        let instance_id = instance.instance_id()
            .ok_or_else(|| HemmerError::Provider("No instance ID".to_string()))?;

        Ok(ResourceOutput::new()
            .with_id(instance_id.to_string())
            .with_field("instance_id", instance_id.to_string())
            .with_field("state", format!("{:?}", instance.state())))
    }

    async fn update_ec2_instance(&self, id: &str, _input: ResourceInput) -> Result<ResourceOutput> {
        // EC2 instances have limited update operations
        // For now, just return current state
        self.read_ec2_instance(id).await
    }

    async fn delete_ec2_instance(&self, id: &str) -> Result<()> {
        let client = self.ec2_client.as_ref().ok_or_else(|| {
            HemmerError::Provider("Provider not configured".to_string())
        })?;

        client
            .terminate_instances()
            .instance_ids(id)
            .send()
            .await
            .map_err(|e| HemmerError::Provider(format!("Failed to delete EC2 instance: {}", e)))?;

        Ok(())
    }
}

/// Factory function for dynamic library loading
///
/// This function is exported for the provider bridge to call
#[no_mangle]
pub extern "C" fn create_provider() -> *mut dyn ProviderExecutor {
    Box::into_raw(Box::new(AwsProvider::new()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_creation() {
        let provider = AwsProvider::new();
        assert!(provider.config.is_none());
        assert!(provider.s3_client.is_none());
        assert!(provider.ec2_client.is_none());
    }
}
