# Appconfigdata Service



**Resources**: 1

---

## Overview

The appconfigdata service provides access to 1 resource type:

- [Latest_configuration](#latest_configuration) [R]

---

## Resources


### Latest_configuration

LatestConfiguration resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_poll_interval_in_seconds` | i64 | <p>The amount of time the client should wait before polling for configuration updates
         again. Use <code>RequiredMinimumPollIntervalInSeconds</code> to set the desired poll
         interval.</p> |
| `version_label` | String | <p>The user-defined label for the AppConfig hosted configuration version. This attribute doesn't apply if the configuration is not from an AppConfig hosted configuration version. If the client already has the latest version of the configuration data, this value is empty.</p> |
| `content_type` | String | <p>A standard MIME type describing the format of the configuration content.</p> |
| `configuration` | String | <p>The data of the configuration. This may be empty if the client already has the latest
         version of configuration.</p> |
| `next_poll_configuration_token` | String | <p>The latest token describing the current state of the configuration session. This
            <i>must</i> be provided to the next call to
            <code>GetLatestConfiguration.</code>
         </p>
         <important>
            <p>This token should only be used once. To support long poll
            use cases, the token is valid for up to 24 hours. If a
            <code>GetLatestConfiguration</code> call uses an expired token, the system returns
            <code>BadRequestException</code>.</p>
         </important> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access latest_configuration outputs
latest_configuration_id = latest_configuration.id
latest_configuration_next_poll_interval_in_seconds = latest_configuration.next_poll_interval_in_seconds
latest_configuration_version_label = latest_configuration.version_label
latest_configuration_content_type = latest_configuration.content_type
latest_configuration_configuration = latest_configuration.configuration
latest_configuration_next_poll_configuration_token = latest_configuration.next_poll_configuration_token
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple latest_configuration resources
latest_configuration_0 = provider.appconfigdata.Latest_configuration {
}
latest_configuration_1 = provider.appconfigdata.Latest_configuration {
}
latest_configuration_2 = provider.appconfigdata.Latest_configuration {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    latest_configuration = provider.appconfigdata.Latest_configuration {
    }
```

---

## Related Documentation

- [AWS Appconfigdata Documentation](https://docs.aws.amazon.com/appconfigdata/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
