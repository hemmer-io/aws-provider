# Groundstation Service



**Resources**: 1

---

## Overview

The groundstation service provides access to 1 resource type:

- [Minute_usage](#minute_usage) [R]

---

## Resources


### Minute_usage

MinuteUsage resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `total_reserved_minute_allocation` | i64 | <p>Total number of reserved minutes allocated, specific to the month being requested.</p> |
| `estimated_minutes_remaining` | i64 | <p>Estimated number of minutes remaining for an account, specific to the month being requested.</p> |
| `upcoming_minutes_scheduled` | i64 | <p>Upcoming minutes scheduled for an account, specific to the month being requested.</p> |
| `total_scheduled_minutes` | i64 | <p>Total scheduled minutes for an account, specific to the month being requested.</p> |
| `is_reserved_minutes_customer` | bool | <p>Returns whether or not an account has signed up for the reserved minutes pricing plan, specific to the month being requested.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access minute_usage outputs
minute_usage_id = minute_usage.id
minute_usage_total_reserved_minute_allocation = minute_usage.total_reserved_minute_allocation
minute_usage_estimated_minutes_remaining = minute_usage.estimated_minutes_remaining
minute_usage_upcoming_minutes_scheduled = minute_usage.upcoming_minutes_scheduled
minute_usage_total_scheduled_minutes = minute_usage.total_scheduled_minutes
minute_usage_is_reserved_minutes_customer = minute_usage.is_reserved_minutes_customer
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple minute_usage resources
minute_usage_0 = provider.groundstation.Minute_usage {
}
minute_usage_1 = provider.groundstation.Minute_usage {
}
minute_usage_2 = provider.groundstation.Minute_usage {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    minute_usage = provider.groundstation.Minute_usage {
    }
```

---

## Related Documentation

- [AWS Groundstation Documentation](https://docs.aws.amazon.com/groundstation/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
