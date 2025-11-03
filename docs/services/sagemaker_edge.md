# Sagemaker_edge Service



**Resources**: 2

---

## Overview

The sagemaker_edge service provides access to 2 resource types:

- [Deployments](#deployments) [R]
- [Device_registration](#device_registration) [R]

---

## Resources


### Deployments

Deployments resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `deployments` | Vec<String> | <p>Returns a list of the configurations of the active deployments on the device.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access deployments outputs
deployments_id = deployments.id
deployments_deployments = deployments.deployments
```

---


### Device_registration

DeviceRegistration resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `device_registration` | String | <p>Describes if the device is currently registered with SageMaker Edge Manager.</p> |
| `cache_ttl` | String | <p>The amount of time, in seconds, that the registration status is stored on the device’s cache before it is refreshed.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access device_registration outputs
device_registration_id = device_registration.id
device_registration_device_registration = device_registration.device_registration
device_registration_cache_ttl = device_registration.cache_ttl
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple deployments resources
deployments_0 = provider.sagemaker_edge.Deployments {
}
deployments_1 = provider.sagemaker_edge.Deployments {
}
deployments_2 = provider.sagemaker_edge.Deployments {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    deployments = provider.sagemaker_edge.Deployments {
    }
```

---

## Related Documentation

- [AWS Sagemaker_edge Documentation](https://docs.aws.amazon.com/sagemaker_edge/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
