# Partnercentral_selling Service



**Resources**: 1

---

## Overview

The partnercentral_selling service provides access to 1 resource type:

- [Selling_system_settings](#selling_system_settings) [CR]

---

## Resources


### Selling_system_settings

SellingSystemSettings resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `catalog` | String | ✅ | <p>Specifies the catalog in which the settings will be updated. Acceptable values include <code>AWS</code> for production and <code>Sandbox</code> for testing environments.</p> |
| `resource_snapshot_job_role_identifier` | String |  | <p>Specifies the ARN of the IAM Role used for resource snapshot job executions.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `catalog` | String | <p>Specifies the catalog in which the settings are defined. Acceptable values include <code>AWS</code> for production and <code>Sandbox</code> for testing environments.</p> |
| `resource_snapshot_job_role_arn` | String | <p>Specifies the ARN of the IAM Role used for resource snapshot job executions.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create selling_system_settings
selling_system_settings = provider.partnercentral_selling.Selling_system_settings {
    catalog = "value"  # <p>Specifies the catalog in which the settings will be updated. Acceptable values include <code>AWS</code> for production and <code>Sandbox</code> for testing environments.</p>
}

# Access selling_system_settings outputs
selling_system_settings_id = selling_system_settings.id
selling_system_settings_catalog = selling_system_settings.catalog
selling_system_settings_resource_snapshot_job_role_arn = selling_system_settings.resource_snapshot_job_role_arn
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple selling_system_settings resources
selling_system_settings_0 = provider.partnercentral_selling.Selling_system_settings {
    catalog = "value-0"
}
selling_system_settings_1 = provider.partnercentral_selling.Selling_system_settings {
    catalog = "value-1"
}
selling_system_settings_2 = provider.partnercentral_selling.Selling_system_settings {
    catalog = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    selling_system_settings = provider.partnercentral_selling.Selling_system_settings {
        catalog = "production-value"
    }
```

---

## Related Documentation

- [AWS Partnercentral_selling Documentation](https://docs.aws.amazon.com/partnercentral_selling/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
