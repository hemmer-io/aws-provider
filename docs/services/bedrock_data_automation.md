# Bedrock_data_automation Service



**Resources**: 1

---

## Overview

The bedrock_data_automation service provides access to 1 resource type:

- [Blueprint_version](#blueprint_version) [C]

---

## Resources


### Blueprint_version

BlueprintVersion resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `blueprint_arn` | String | ✅ | ARN generated at the server side when a Blueprint is created |
| `client_token` | String |  |  |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create blueprint_version
blueprint_version = provider.bedrock_data_automation.Blueprint_version {
    blueprint_arn = "value"  # ARN generated at the server side when a Blueprint is created
}

```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple blueprint_version resources
blueprint_version_0 = provider.bedrock_data_automation.Blueprint_version {
    blueprint_arn = "value-0"
}
blueprint_version_1 = provider.bedrock_data_automation.Blueprint_version {
    blueprint_arn = "value-1"
}
blueprint_version_2 = provider.bedrock_data_automation.Blueprint_version {
    blueprint_arn = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    blueprint_version = provider.bedrock_data_automation.Blueprint_version {
        blueprint_arn = "production-value"
    }
```

---

## Related Documentation

- [AWS Bedrock_data_automation Documentation](https://docs.aws.amazon.com/bedrock_data_automation/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
