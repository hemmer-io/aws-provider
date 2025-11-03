# Cloudtrail_data Service



**Resources**: 1

---

## Overview

The cloudtrail_data service provides access to 1 resource type:

- [Audit_events](#audit_events) [C]

---

## Resources


### Audit_events

AuditEvents resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `channel_arn` | String | ✅ | <p>The ARN or ID (the ARN suffix) of a channel.</p> |
| `external_id` | String |  | <p>A unique identifier that is conditionally required when the channel's resource policy includes an external 
         ID. This value can be any string, 
         such as a passphrase or account number.</p> |
| `audit_events` | Vec<String> | ✅ | <p>The JSON payload of events that you want to ingest. You can also point to the JSON event
         payload in a file.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create audit_events
audit_events = provider.cloudtrail_data.Audit_events {
    channel_arn = "value"  # <p>The ARN or ID (the ARN suffix) of a channel.</p>
    audit_events = "value"  # <p>The JSON payload of events that you want to ingest. You can also point to the JSON event
         payload in a file.</p>
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

# Create multiple audit_events resources
audit_events_0 = provider.cloudtrail_data.Audit_events {
    channel_arn = "value-0"
    audit_events = "value-0"
}
audit_events_1 = provider.cloudtrail_data.Audit_events {
    channel_arn = "value-1"
    audit_events = "value-1"
}
audit_events_2 = provider.cloudtrail_data.Audit_events {
    channel_arn = "value-2"
    audit_events = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    audit_events = provider.cloudtrail_data.Audit_events {
        channel_arn = "production-value"
        audit_events = "production-value"
    }
```

---

## Related Documentation

- [AWS Cloudtrail_data Documentation](https://docs.aws.amazon.com/cloudtrail_data/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
