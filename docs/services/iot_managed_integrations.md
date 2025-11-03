# Iot_managed_integrations Service



**Resources**: 1

---

## Overview

The iot_managed_integrations service provides access to 1 resource type:

- [Custom_endpoint](#custom_endpoint) [R]

---

## Resources


### Custom_endpoint

CustomEndpoint resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `endpoint_address` | String | <p>The IoT managed integrations dedicated, custom endpoint for the device to route traffic through.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access custom_endpoint outputs
custom_endpoint_id = custom_endpoint.id
custom_endpoint_endpoint_address = custom_endpoint.endpoint_address
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple custom_endpoint resources
custom_endpoint_0 = provider.iot_managed_integrations.Custom_endpoint {
}
custom_endpoint_1 = provider.iot_managed_integrations.Custom_endpoint {
}
custom_endpoint_2 = provider.iot_managed_integrations.Custom_endpoint {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    custom_endpoint = provider.iot_managed_integrations.Custom_endpoint {
    }
```

---

## Related Documentation

- [AWS Iot_managed_integrations Documentation](https://docs.aws.amazon.com/iot_managed_integrations/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
