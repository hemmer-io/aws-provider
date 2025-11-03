# Drs Service



**Resources**: 2

---

## Overview

The drs service provides access to 2 resource types:

- [Extended_source_server](#extended_source_server) [C]
- [Launch_action](#launch_action) [CD]

---

## Resources


### Extended_source_server

ExtendedSourceServer resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `source_server_arn` | String | ✅ | <p>This defines the ARN of the source server in staging Account based on which you want to create an extended source server.</p> |
| `tags` | HashMap<String, String> |  | <p>A list of tags associated with the extended source server.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create extended_source_server
extended_source_server = provider.drs.Extended_source_server {
    source_server_arn = "value"  # <p>This defines the ARN of the source server in staging Account based on which you want to create an extended source server.</p>
}

```

---


### Launch_action

LaunchAction resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ |  |
| `order` | i64 | ✅ |  |
| `description` | String | ✅ |  |
| `category` | String | ✅ |  |
| `action_id` | String | ✅ |  |
| `action_code` | String | ✅ | <p>Launch action code.</p> |
| `active` | bool | ✅ | <p>Whether the launch action is active.</p> |
| `resource_id` | String | ✅ |  |
| `parameters` | HashMap<String, String> |  |  |
| `optional` | bool | ✅ | <p>Whether the launch will not be marked as failed if this action fails.</p> |
| `action_version` | String | ✅ |  |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create launch_action
launch_action = provider.drs.Launch_action {
    name = "value"  # Required field
    order = "value"  # Required field
    description = "value"  # Required field
    category = "value"  # Required field
    action_id = "value"  # Required field
    action_code = "value"  # <p>Launch action code.</p>
    active = "value"  # <p>Whether the launch action is active.</p>
    resource_id = "value"  # Required field
    optional = "value"  # <p>Whether the launch will not be marked as failed if this action fails.</p>
    action_version = "value"  # Required field
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

# Create multiple extended_source_server resources
extended_source_server_0 = provider.drs.Extended_source_server {
    source_server_arn = "value-0"
}
extended_source_server_1 = provider.drs.Extended_source_server {
    source_server_arn = "value-1"
}
extended_source_server_2 = provider.drs.Extended_source_server {
    source_server_arn = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    extended_source_server = provider.drs.Extended_source_server {
        source_server_arn = "production-value"
    }
```

---

## Related Documentation

- [AWS Drs Documentation](https://docs.aws.amazon.com/drs/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
