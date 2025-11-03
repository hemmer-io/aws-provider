# Rum Service



**Resources**: 1

---

## Overview

The rum service provides access to 1 resource type:

- [Rum_events](#rum_events) [C]

---

## Resources


### Rum_events

RumEvents resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `app_monitor_details` | String | ✅ | <p>A structure that contains information about the app monitor that collected this telemetry information.</p> |
| `user_details` | String | ✅ | <p>A structure that contains information about the user session that this batch of events was collected from.</p> |
| `id` | String | ✅ | <p>The ID of the app monitor that is sending this data.</p> |
| `batch_id` | String | ✅ | <p>A unique identifier for this batch of RUM event data.</p> |
| `alias` | String |  | <p>If the app monitor uses a resource-based policy that requires <code>PutRumEvents</code> requests to specify a certain alias,
         specify that alias here. This alias will be compared to the <code>rum:alias</code> context key in the resource-based policy.  For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch-RUM-resource-policies.html">Using resource-based policies with CloudWatch RUM</a>.</p> |
| `rum_events` | Vec<String> | ✅ | <p>An array of structures that contain the telemetry event data.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create rum_events
rum_events = provider.rum.Rum_events {
    app_monitor_details = "value"  # <p>A structure that contains information about the app monitor that collected this telemetry information.</p>
    user_details = "value"  # <p>A structure that contains information about the user session that this batch of events was collected from.</p>
    id = "value"  # <p>The ID of the app monitor that is sending this data.</p>
    batch_id = "value"  # <p>A unique identifier for this batch of RUM event data.</p>
    rum_events = "value"  # <p>An array of structures that contain the telemetry event data.</p>
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

# Create multiple rum_events resources
rum_events_0 = provider.rum.Rum_events {
    app_monitor_details = "value-0"
    user_details = "value-0"
    id = "value-0"
    batch_id = "value-0"
    rum_events = "value-0"
}
rum_events_1 = provider.rum.Rum_events {
    app_monitor_details = "value-1"
    user_details = "value-1"
    id = "value-1"
    batch_id = "value-1"
    rum_events = "value-1"
}
rum_events_2 = provider.rum.Rum_events {
    app_monitor_details = "value-2"
    user_details = "value-2"
    id = "value-2"
    batch_id = "value-2"
    rum_events = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    rum_events = provider.rum.Rum_events {
        app_monitor_details = "production-value"
        user_details = "production-value"
        id = "production-value"
        batch_id = "production-value"
        rum_events = "production-value"
    }
```

---

## Related Documentation

- [AWS Rum Documentation](https://docs.aws.amazon.com/rum/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
