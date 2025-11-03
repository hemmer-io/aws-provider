# Workspaces_thin_client Service



**Resources**: 3

---

## Overview

The workspaces_thin_client service provides access to 3 resource types:

- [Software_set](#software_set) [RU]
- [Device](#device) [RUD]
- [Environment](#environment) [CRUD]

---

## Resources


### Software_set

SoftwareSet resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | String | ✅ | <p>The ID of the software set to update.</p> |
| `validation_status` | String | ✅ | <p>An option to define if the software set has been validated.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `software_set` | String | <p>Describes a software set.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access software_set outputs
software_set_id = software_set.id
software_set_software_set = software_set.software_set
```

---


### Device

Device resource

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | <p>The name of the device to update.</p> |
| `desired_software_set_id` | String |  | <p>The ID of the software set to apply.</p> |
| `software_set_update_schedule` | String |  | <p>An option to define if software updates should be applied within a maintenance window.</p> |
| `id` | String | ✅ | <p>The ID of the device to update.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `device` | String | <p>Describes an device.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access device outputs
device_id = device.id
device_device = device.device
```

---


### Environment

Environment resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `maintenance_window` | String |  | <p>A specification for a time window to apply software updates.</p> |
| `software_set_update_mode` | String |  | <p>An option to define which software updates to apply.</p> |
| `desktop_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the desktop to stream from Amazon WorkSpaces, WorkSpaces Secure Browser, or AppStream 2.0.</p> |
| `desktop_endpoint` | String |  | <p>The URL for the identity provider login (only for environments that use AppStream 2.0).</p> |
| `desired_software_set_id` | String |  | <p>The ID of the software set to apply.</p> |
| `client_token` | String |  | <p>Specifies a unique, case-sensitive identifier that you provide to ensure the idempotency of the request. This lets you safely retry the request without accidentally performing the same operation a second time. Passing the same value to a later call to an operation requires that you also pass the same value for all other parameters. We recommend that you use a <a href="https://wikipedia.org/wiki/Universally_unique_identifier">UUID type of value</a>.</p> <p>If you don't provide this value, then Amazon Web Services generates a random one for you.</p> <p>If you retry the operation with the same <code>ClientToken</code>, but with different parameters, the retry fails with an <code>IdempotentParameterMismatch</code> error.</p> |
| `device_creation_tags` | HashMap<String, String> |  | <p>A map of the key-value pairs of the tag or tags to assign to the newly created devices for this environment.</p> |
| `tags` | HashMap<String, String> |  | <p>A map of the key-value pairs of the tag or tags to assign to the resource.</p> |
| `kms_key_arn` | String |  | <p>The Amazon Resource Name (ARN) of the Key Management Service key to use to encrypt the environment.</p> |
| `name` | String |  | <p>The name for the environment.</p> |
| `software_set_update_schedule` | String |  | <p>An option to define if software updates should be applied within a maintenance window.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `environment` | String | <p>Describes an environment.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create environment
environment = provider.workspaces_thin_client.Environment {
    desktop_arn = "value"  # <p>The Amazon Resource Name (ARN) of the desktop to stream from Amazon WorkSpaces, WorkSpaces Secure Browser, or AppStream 2.0.</p>
}

# Access environment outputs
environment_id = environment.id
environment_environment = environment.environment
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple software_set resources
software_set_0 = provider.workspaces_thin_client.Software_set {
    id = "value-0"
    validation_status = "value-0"
}
software_set_1 = provider.workspaces_thin_client.Software_set {
    id = "value-1"
    validation_status = "value-1"
}
software_set_2 = provider.workspaces_thin_client.Software_set {
    id = "value-2"
    validation_status = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    software_set = provider.workspaces_thin_client.Software_set {
        id = "production-value"
        validation_status = "production-value"
    }
```

---

## Related Documentation

- [AWS Workspaces_thin_client Documentation](https://docs.aws.amazon.com/workspaces_thin_client/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
