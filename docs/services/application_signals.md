# Application_signals Service



**Resources**: 2

---

## Overview

The application_signals service provides access to 2 resource types:

- [Grouping_configuration](#grouping_configuration) [CD]
- [Service](#service) [R]

---

## Resources


### Grouping_configuration

GroupingConfiguration resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `grouping_attribute_definitions` | Vec<String> | ✅ | <p>An array of grouping attribute definitions that specify how services should be grouped. Each definition includes the grouping name, source keys, and default values.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create grouping_configuration
grouping_configuration = provider.application_signals.Grouping_configuration {
    grouping_attribute_definitions = "value"  # <p>An array of grouping attribute definitions that specify how services should be grouped. Each definition includes the grouping name, source keys, and default values.</p>
}

```

---


### Service

Service resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `service` | String | <p>A structure containing information about the service.</p> |
| `start_time` | String | <p>The start time of the data included in the response. In a raw HTTP Query API, it is formatted as be epoch time in seconds. For example: <code>1698778057</code>.</p> <p>This displays the time that Application Signals used for the request. It might not match your request exactly, because it was rounded to the nearest hour.</p> |
| `log_group_references` | Vec<HashMap<String, String>> | <p>An array of string-to-string maps that each contain information about one log group associated with this service. Each string-to-string map includes the following fields:</p> <ul> <li> <p> <code>"Type": "AWS::Resource"</code> </p> </li> <li> <p> <code>"ResourceType": "AWS::Logs::LogGroup"</code> </p> </li> <li> <p> <code>"Identifier": "<i>name-of-log-group</i>"</code> </p> </li> </ul> |
| `end_time` | String | <p>The end time of the data included in the response. In a raw HTTP Query API, it is formatted as be epoch time in seconds. For example: <code>1698778057</code>.</p> <p>This displays the time that Application Signals used for the request. It might not match your request exactly, because it was rounded to the nearest hour.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access service outputs
service_id = service.id
service_service = service.service
service_start_time = service.start_time
service_log_group_references = service.log_group_references
service_end_time = service.end_time
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple grouping_configuration resources
grouping_configuration_0 = provider.application_signals.Grouping_configuration {
    grouping_attribute_definitions = "value-0"
}
grouping_configuration_1 = provider.application_signals.Grouping_configuration {
    grouping_attribute_definitions = "value-1"
}
grouping_configuration_2 = provider.application_signals.Grouping_configuration {
    grouping_attribute_definitions = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    grouping_configuration = provider.application_signals.Grouping_configuration {
        grouping_attribute_definitions = "production-value"
    }
```

---

## Related Documentation

- [AWS Application_signals Documentation](https://docs.aws.amazon.com/application_signals/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
