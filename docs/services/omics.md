# Omics Service



**Resources**: 1

---

## Overview

The omics service provides access to 1 resource type:

- [S3_access_policy](#s3_access_policy) [CRD]

---

## Resources


### S3_access_policy

S3AccessPolicy resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `s3_access_policy` | String | ✅ | <p>The resource policy that controls S3 access to the store.</p> |
| `s3_access_point_arn` | String | ✅ | <p>The S3 access point ARN where you want to put the access policy.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `store_id` | String | <p>The Amazon Web Services-generated Sequence Store or Reference Store ID.</p> |
| `store_type` | String | <p>The type of store associated with the access point.</p> |
| `update_time` | String | <p>The time when the policy was last updated.</p> |
| `s3_access_point_arn` | String | <p>The S3 access point ARN that has the access policy.</p> |
| `s3_access_policy` | String | <p>The current resource policy that controls S3 access on the store.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create s3_access_policy
s3_access_policy = provider.omics.S3_access_policy {
    s3_access_policy = "value"  # <p>The resource policy that controls S3 access to the store.</p>
    s3_access_point_arn = "value"  # <p>The S3 access point ARN where you want to put the access policy.</p>
}

# Access s3_access_policy outputs
s3_access_policy_id = s3_access_policy.id
s3_access_policy_store_id = s3_access_policy.store_id
s3_access_policy_store_type = s3_access_policy.store_type
s3_access_policy_update_time = s3_access_policy.update_time
s3_access_policy_s3_access_point_arn = s3_access_policy.s3_access_point_arn
s3_access_policy_s3_access_policy = s3_access_policy.s3_access_policy
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple s3_access_policy resources
s3_access_policy_0 = provider.omics.S3_access_policy {
    s3_access_policy = "value-0"
    s3_access_point_arn = "value-0"
}
s3_access_policy_1 = provider.omics.S3_access_policy {
    s3_access_policy = "value-1"
    s3_access_point_arn = "value-1"
}
s3_access_policy_2 = provider.omics.S3_access_policy {
    s3_access_policy = "value-2"
    s3_access_point_arn = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    s3_access_policy = provider.omics.S3_access_policy {
        s3_access_policy = "production-value"
        s3_access_point_arn = "production-value"
    }
```

---

## Related Documentation

- [AWS Omics Documentation](https://docs.aws.amazon.com/omics/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
