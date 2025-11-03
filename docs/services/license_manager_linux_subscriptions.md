# License_manager_linux_subscriptions Service



**Resources**: 2

---

## Overview

The license_manager_linux_subscriptions service provides access to 2 resource types:

- [Registered_subscription_provider](#registered_subscription_provider) [R]
- [Service_settings](#service_settings) [RU]

---

## Resources


### Registered_subscription_provider

RegisteredSubscriptionProvider resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `subscription_provider_status` | String | <p>The status of the Linux subscription provider access token from the last 
			successful subscription data request.</p> |
| `subscription_provider_source` | String | <p>The subscription provider for the BYOL registration resource specified 
			in the request.</p> |
| `secret_arn` | String | <p>The Amazon Resource Name (ARN) of the third-party access secret stored in Secrets Manager for the BYOL 
			registration resource specified in the request.</p> |
| `subscription_provider_status_message` | String | <p>The detailed message from your subscription provider token status.</p> |
| `subscription_provider_arn` | String | <p>The Amazon Resource Name (ARN) for the BYOL registration resource specified in the request.</p> |
| `last_successful_data_retrieval_time` | String | <p>The timestamp from the last time License Manager retrieved subscription details 
			from your registered third-party Linux subscription provider.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access registered_subscription_provider outputs
registered_subscription_provider_id = registered_subscription_provider.id
registered_subscription_provider_subscription_provider_status = registered_subscription_provider.subscription_provider_status
registered_subscription_provider_subscription_provider_source = registered_subscription_provider.subscription_provider_source
registered_subscription_provider_secret_arn = registered_subscription_provider.secret_arn
registered_subscription_provider_subscription_provider_status_message = registered_subscription_provider.subscription_provider_status_message
registered_subscription_provider_subscription_provider_arn = registered_subscription_provider.subscription_provider_arn
registered_subscription_provider_last_successful_data_retrieval_time = registered_subscription_provider.last_successful_data_retrieval_time
```

---


### Service_settings

ServiceSettings resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `allow_update` | bool |  | <p>Describes if updates are allowed to the service settings for Linux subscriptions. If you
      allow updates, you can aggregate Linux subscription data in more than one home Region.</p> |
| `linux_subscriptions_discovery_settings` | String | ✅ | <p>The settings defined for Linux subscriptions discovery. The settings include if Organizations
      integration has been enabled, and which Regions data will be aggregated from.</p> |
| `linux_subscriptions_discovery` | String | ✅ | <p>Describes if the discovery of Linux subscriptions is enabled.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `linux_subscriptions_discovery_settings` | String | <p>Lists the settings defined for Linux subscriptions discovery. The settings include if
      Organizations integration has been enabled, and which Regions data will be aggregated from.</p> |
| `linux_subscriptions_discovery` | String | <p>Lists if discovery has been enabled for Linux subscriptions.</p> |
| `status` | String | <p>Indicates the status of Linux subscriptions settings being applied.</p> |
| `status_message` | String | <p>A message which details the Linux subscriptions service settings current status.</p> |
| `home_regions` | String | <p>The Region in which License Manager displays the aggregated data for Linux
      subscriptions.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access service_settings outputs
service_settings_id = service_settings.id
service_settings_linux_subscriptions_discovery_settings = service_settings.linux_subscriptions_discovery_settings
service_settings_linux_subscriptions_discovery = service_settings.linux_subscriptions_discovery
service_settings_status = service_settings.status
service_settings_status_message = service_settings.status_message
service_settings_home_regions = service_settings.home_regions
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple registered_subscription_provider resources
registered_subscription_provider_0 = provider.license_manager_linux_subscriptions.Registered_subscription_provider {
}
registered_subscription_provider_1 = provider.license_manager_linux_subscriptions.Registered_subscription_provider {
}
registered_subscription_provider_2 = provider.license_manager_linux_subscriptions.Registered_subscription_provider {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    registered_subscription_provider = provider.license_manager_linux_subscriptions.Registered_subscription_provider {
    }
```

---

## Related Documentation

- [AWS License_manager_linux_subscriptions Documentation](https://docs.aws.amazon.com/license_manager_linux_subscriptions/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
