# Rolesanywhere Service



**Resources**: 1

---

## Overview

The rolesanywhere service provides access to 1 resource type:

- [Notification_settings](#notification_settings) [C]

---

## Resources


### Notification_settings

NotificationSettings resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `notification_settings` | Vec<String> | ✅ | <p>A list of notification settings to be associated to the trust anchor.</p> |
| `trust_anchor_id` | String | ✅ | <p>The unique identifier of the trust anchor.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create notification_settings
notification_settings = provider.rolesanywhere.Notification_settings {
    notification_settings = "value"  # <p>A list of notification settings to be associated to the trust anchor.</p>
    trust_anchor_id = "value"  # <p>The unique identifier of the trust anchor.</p>
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

# Create multiple notification_settings resources
notification_settings_0 = provider.rolesanywhere.Notification_settings {
    notification_settings = "value-0"
    trust_anchor_id = "value-0"
}
notification_settings_1 = provider.rolesanywhere.Notification_settings {
    notification_settings = "value-1"
    trust_anchor_id = "value-1"
}
notification_settings_2 = provider.rolesanywhere.Notification_settings {
    notification_settings = "value-2"
    trust_anchor_id = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    notification_settings = provider.rolesanywhere.Notification_settings {
        notification_settings = "production-value"
        trust_anchor_id = "production-value"
    }
```

---

## Related Documentation

- [AWS Rolesanywhere Documentation](https://docs.aws.amazon.com/rolesanywhere/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
