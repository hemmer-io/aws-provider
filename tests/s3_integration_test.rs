//! S3 Integration Tests
//!
//! These tests validate S3 bucket operations against real AWS.
//!
//! Prerequisites:
//! - AWS credentials configured (env vars or ~/.aws/credentials)
//! - Proper IAM permissions for S3
//!
//! Run with: cargo test --test s3_integration_test -- --ignored

use hemmer_aws_provider::AwsProvider;
use hemmer_provider::{ProviderConfig, ProviderExecutor, ResourceInput};

/// Generate a unique bucket name for testing
fn generate_test_bucket_name() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    format!("hemmer-test-bucket-{}", timestamp)
}

#[tokio::test]
#[ignore] // Ignore by default - run explicitly with --ignored
async fn test_s3_bucket_crud() {
    // Initialize provider
    let mut provider = AwsProvider::new();

    // Configure with us-east-1 region
    let config = ProviderConfig::new()
        .with_field("region", "us-east-1")
        .expect("Failed to create config");

    provider.configure(config).await
        .expect("Failed to configure provider");

    let bucket_name = generate_test_bucket_name();
    println!("Testing with bucket: {}", bucket_name);

    // CREATE: Create a bucket
    let input = ResourceInput::new()
        .with_field("name", bucket_name.clone());

    let output = provider.create("bucket", input).await
        .expect("Failed to create bucket");

    assert_eq!(output.id, bucket_name);
    assert_eq!(output.get_string("name").unwrap(), bucket_name);
    println!("✓ Created bucket: {}", bucket_name);

    // READ: Verify bucket exists
    let read_output = provider.read("bucket", &bucket_name).await
        .expect("Failed to read bucket");

    assert_eq!(read_output.id, bucket_name);
    println!("✓ Read bucket: {}", bucket_name);

    // UPDATE: Update bucket (limited operations for S3)
    let update_input = ResourceInput::new()
        .with_field("name", bucket_name.clone());

    let update_output = provider.update("bucket", &bucket_name, update_input).await
        .expect("Failed to update bucket");

    assert_eq!(update_output.id, bucket_name);
    println!("✓ Updated bucket: {}", bucket_name);

    // DELETE: Clean up
    provider.delete("bucket", &bucket_name).await
        .expect("Failed to delete bucket");

    println!("✓ Deleted bucket: {}", bucket_name);

    // Verify deletion
    let read_result = provider.read("bucket", &bucket_name).await;
    assert!(read_result.is_err(), "Bucket should not exist after deletion");
    println!("✓ Verified bucket deletion");
}

#[tokio::test]
#[ignore]
async fn test_s3_bucket_create_duplicate() {
    let mut provider = AwsProvider::new();

    let config = ProviderConfig::new()
        .with_field("region", "us-east-1")
        .expect("Failed to create config");

    provider.configure(config).await
        .expect("Failed to configure provider");

    let bucket_name = generate_test_bucket_name();

    // Create bucket
    let input = ResourceInput::new()
        .with_field("name", bucket_name.clone());

    provider.create("bucket", input.clone()).await
        .expect("Failed to create first bucket");

    // Try to create duplicate - should fail
    let result = provider.create("bucket", input).await;
    assert!(result.is_err(), "Creating duplicate bucket should fail");

    // Clean up
    provider.delete("bucket", &bucket_name).await
        .expect("Failed to cleanup");
}

#[tokio::test]
#[ignore]
async fn test_s3_bucket_delete_nonexistent() {
    let mut provider = AwsProvider::new();

    let config = ProviderConfig::new()
        .with_field("region", "us-east-1")
        .expect("Failed to create config");

    provider.configure(config).await
        .expect("Failed to configure provider");

    let bucket_name = "hemmer-nonexistent-bucket-xyz-123";

    // Try to delete non-existent bucket - should fail gracefully
    let result = provider.delete("bucket", bucket_name).await;
    assert!(result.is_err(), "Deleting non-existent bucket should fail");
}

#[tokio::test]
#[ignore]
async fn test_s3_multiple_buckets() {
    let mut provider = AwsProvider::new();

    let config = ProviderConfig::new()
        .with_field("region", "us-east-1")
        .expect("Failed to create config");

    provider.configure(config).await
        .expect("Failed to configure provider");

    let bucket1 = generate_test_bucket_name();
    let bucket2 = format!("{}-2", bucket1);
    let bucket3 = format!("{}-3", bucket1);

    // Create multiple buckets
    for bucket_name in &[&bucket1, &bucket2, &bucket3] {
        let input = ResourceInput::new()
            .with_field("name", bucket_name.to_string());

        provider.create("bucket", input).await
            .expect(&format!("Failed to create bucket: {}", bucket_name));

        println!("✓ Created bucket: {}", bucket_name);
    }

    // Verify all exist
    for bucket_name in &[&bucket1, &bucket2, &bucket3] {
        provider.read("bucket", bucket_name).await
            .expect(&format!("Failed to read bucket: {}", bucket_name));

        println!("✓ Verified bucket: {}", bucket_name);
    }

    // Clean up all buckets
    for bucket_name in &[&bucket1, &bucket2, &bucket3] {
        provider.delete("bucket", bucket_name).await
            .expect(&format!("Failed to delete bucket: {}", bucket_name));

        println!("✓ Deleted bucket: {}", bucket_name);
    }
}

#[tokio::test]
async fn test_provider_creation() {
    // This test doesn't need AWS credentials
    let provider = AwsProvider::new();
    assert!(true, "Provider created successfully");
}

#[tokio::test]
#[ignore]
async fn test_provider_configuration() {
    let mut provider = AwsProvider::new();

    let config = ProviderConfig::new()
        .with_field("region", "us-west-2")
        .expect("Failed to create config");

    let result = provider.configure(config).await;
    assert!(result.is_ok(), "Provider configuration should succeed");
}
