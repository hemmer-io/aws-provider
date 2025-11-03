# Securitylake Service



**Resources**: 1

---

## Overview

The securitylake service provides access to 1 resource type:

- [Data_lake_exception_subscription](#data_lake_exception_subscription) [CRUD]

---

## Resources


### Data_lake_exception_subscription

DataLakeExceptionSubscription resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `subscription_protocol` | String | ✅ | <p>The subscription protocol to which exception notifications are posted.</p> |
| `notification_endpoint` | String | ✅ | <p>The Amazon Web Services account where you want to receive exception notifications.</p> |
| `exception_time_to_live` | i64 |  | <p>The expiration period and time-to-live (TTL). It is the duration of time until which the exception message remains.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `notification_endpoint` | String | <p>The Amazon Web Services account where you receive exception notifications.</p> |
| `exception_time_to_live` | i64 | <p>The expiration period and time-to-live (TTL). It is the duration of time until which the exception message remains.</p> |
| `subscription_protocol` | String | <p>The subscription protocol to which exception notifications are posted.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create data_lake_exception_subscription
data_lake_exception_subscription = provider.securitylake.Data_lake_exception_subscription {
    subscription_protocol = "value"  # <p>The subscription protocol to which exception notifications are posted.</p>
    notification_endpoint = "value"  # <p>The Amazon Web Services account where you want to receive exception notifications.</p>
}

# Access data_lake_exception_subscription outputs
data_lake_exception_subscription_id = data_lake_exception_subscription.id
data_lake_exception_subscription_notification_endpoint = data_lake_exception_subscription.notification_endpoint
data_lake_exception_subscription_exception_time_to_live = data_lake_exception_subscription.exception_time_to_live
data_lake_exception_subscription_subscription_protocol = data_lake_exception_subscription.subscription_protocol
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple data_lake_exception_subscription resources
data_lake_exception_subscription_0 = provider.securitylake.Data_lake_exception_subscription {
    subscription_protocol = "value-0"
    notification_endpoint = "value-0"
}
data_lake_exception_subscription_1 = provider.securitylake.Data_lake_exception_subscription {
    subscription_protocol = "value-1"
    notification_endpoint = "value-1"
}
data_lake_exception_subscription_2 = provider.securitylake.Data_lake_exception_subscription {
    subscription_protocol = "value-2"
    notification_endpoint = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    data_lake_exception_subscription = provider.securitylake.Data_lake_exception_subscription {
        subscription_protocol = "production-value"
        notification_endpoint = "production-value"
    }
```

---

## Related Documentation

- [AWS Securitylake Documentation](https://docs.aws.amazon.com/securitylake/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
