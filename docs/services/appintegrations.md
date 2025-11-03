# Appintegrations Service



**Resources**: 4

---

## Overview

The appintegrations service provides access to 4 resource types:

- [Data_integration](#data_integration) [CRUD]
- [Data_integration_association](#data_integration_association) [CU]
- [Application](#application) [CRUD]
- [Event_integration](#event_integration) [CRUD]

---

## Resources


### Data_integration

DataIntegration resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | <p>A description of the DataIntegration.</p> |
| `file_configuration` | String |  | <p>The configuration for what files should be pulled from the source.</p> |
| `client_token` | String |  | <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the
            request. If not provided, the Amazon Web Services
            SDK populates this field. For more information about idempotency, see
            <a href="https://aws.amazon.com/builders-library/making-retries-safe-with-idempotent-APIs/">Making retries safe with idempotent APIs</a>.</p> |
| `object_configuration` | HashMap<String, HashMap<String, Vec<String>>> |  | <p>The configuration for what data should be pulled from the source.</p> |
| `name` | String | ✅ | <p>The name of the DataIntegration.</p> |
| `kms_key` | String | ✅ | <p>The KMS key ARN for the DataIntegration.</p> |
| `source_uri` | String |  | <p>The URI of the data source.</p> |
| `schedule_config` | String |  | <p>The name of the data and how often it should be pulled from the source.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags used to organize, track, or control access for this resource. For example, { "tags": {"key1":"value1", "key2":"value2"} }.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | <p>The name of the DataIntegration.</p> |
| `kms_key` | String | <p>The KMS key ARN for the DataIntegration.</p> |
| `file_configuration` | String | <p>The configuration for what files should be pulled from the source.</p> |
| `id` | String | <p>A unique identifier.</p> |
| `object_configuration` | HashMap<String, HashMap<String, Vec<String>>> | <p>The configuration for what data should be pulled from the source.</p> |
| `arn` | String | <p>The Amazon Resource Name (ARN) for the DataIntegration.</p> |
| `description` | String | <p>The KMS key ARN for the DataIntegration.</p> |
| `schedule_configuration` | String | <p>The name of the data and how often it should be pulled from the source.</p> |
| `source_uri` | String | <p>The URI of the data source.</p> |
| `tags` | HashMap<String, String> | <p>The tags used to organize, track, or control access for this resource. For example, { "tags": {"key1":"value1", "key2":"value2"} }.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create data_integration
data_integration = provider.appintegrations.Data_integration {
    name = "value"  # <p>The name of the DataIntegration.</p>
    kms_key = "value"  # <p>The KMS key ARN for the DataIntegration.</p>
}

# Access data_integration outputs
data_integration_id = data_integration.id
data_integration_name = data_integration.name
data_integration_kms_key = data_integration.kms_key
data_integration_file_configuration = data_integration.file_configuration
data_integration_id = data_integration.id
data_integration_object_configuration = data_integration.object_configuration
data_integration_arn = data_integration.arn
data_integration_description = data_integration.description
data_integration_schedule_configuration = data_integration.schedule_configuration
data_integration_source_uri = data_integration.source_uri
data_integration_tags = data_integration.tags
```

---


### Data_integration_association

DataIntegrationAssociation resource

**Operations**: ✅ Create ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_token` | String |  | <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the
            request. If not provided, the Amazon Web Services
            SDK populates this field. For more information about idempotency, see
            <a href="https://aws.amazon.com/builders-library/making-retries-safe-with-idempotent-APIs/">Making retries safe with idempotent APIs</a>.</p> |
| `destination_uri` | String |  | <p>The URI of the data destination.</p> |
| `data_integration_identifier` | String | ✅ | <p>A unique identifier for the DataIntegration.</p> |
| `object_configuration` | HashMap<String, HashMap<String, Vec<String>>> |  |  |
| `execution_configuration` | String |  | <p>The configuration for how the files should be pulled from the source.</p> |
| `client_id` | String |  | <p>The identifier for the client that is associated with the DataIntegration
      association.</p> |
| `client_association_metadata` | HashMap<String, String> |  | <p>The mapping of metadata to be extracted from the data.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create data_integration_association
data_integration_association = provider.appintegrations.Data_integration_association {
    data_integration_identifier = "value"  # <p>A unique identifier for the DataIntegration.</p>
}

```

---


### Application

Application resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_token` | String |  | <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the
            request. If not provided, the Amazon Web Services
            SDK populates this field. For more information about idempotency, see
            <a href="https://aws.amazon.com/builders-library/making-retries-safe-with-idempotent-APIs/">Making retries safe with idempotent APIs</a>.</p> |
| `subscriptions` | Vec<String> |  | <p>The events that the application subscribes.</p> |
| `initialization_timeout` | i64 |  | <p>The maximum time in milliseconds allowed to establish a connection with the workspace.</p> |
| `application_config` | String |  | <p>The configuration settings for the application.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags used to organize, track, or control access for this resource. For example, { "tags": {"key1":"value1", "key2":"value2"} }.</p> |
| `iframe_config` | String |  | <p>The iframe configuration for the application.</p> |
| `namespace` | String | ✅ | <p>The namespace of the application.</p> |
| `description` | String |  | <p>The description of the application.</p> |
| `name` | String | ✅ | <p>The name of the application.</p> |
| `application_source_config` | String | ✅ | <p>The configuration for where the application should be loaded from.</p> |
| `publications` | Vec<String> |  | <p>The events that the application publishes.</p> |
| `is_service` | bool |  | <p>Indicates whether the application is a service.</p> |
| `permissions` | Vec<String> |  | <p>The configuration of events or requests that the application has access to.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `subscriptions` | Vec<String> | <p>The events that the application subscribes.</p> |
| `permissions` | Vec<String> | <p>The configuration of events or requests that the application has access to.</p> |
| `last_modified_time` | String | <p>The last modified time of the Application.</p> |
| `application_config` | String | <p>The configuration settings for the application.</p> |
| `name` | String | <p>The name of the application.</p> |
| `iframe_config` | String | <p>The iframe configuration for the application.</p> |
| `application_source_config` | String | <p>The configuration for where the application should be loaded from.</p> |
| `publications` | Vec<String> | <p>The events that the application publishes.</p> |
| `tags` | HashMap<String, String> | <p>The tags used to organize, track, or control access for this resource. For example, { "tags": {"key1":"value1", "key2":"value2"} }.</p> |
| `created_time` | String | <p>The created time of the Application.</p> |
| `initialization_timeout` | i64 | <p>The maximum time in milliseconds allowed to establish a connection with the workspace.</p> |
| `arn` | String | <p>The Amazon Resource Name (ARN) of the Application.</p> |
| `is_service` | bool | <p>Indicates whether the application is a service.</p> |
| `namespace` | String | <p>The namespace of the application.</p> |
| `id` | String | <p>A unique identifier for the Application.</p> |
| `description` | String | <p>The description of the application.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create application
application = provider.appintegrations.Application {
    namespace = "value"  # <p>The namespace of the application.</p>
    name = "value"  # <p>The name of the application.</p>
    application_source_config = "value"  # <p>The configuration for where the application should be loaded from.</p>
}

# Access application outputs
application_id = application.id
application_subscriptions = application.subscriptions
application_permissions = application.permissions
application_last_modified_time = application.last_modified_time
application_application_config = application.application_config
application_name = application.name
application_iframe_config = application.iframe_config
application_application_source_config = application.application_source_config
application_publications = application.publications
application_tags = application.tags
application_created_time = application.created_time
application_initialization_timeout = application.initialization_timeout
application_arn = application.arn
application_is_service = application.is_service
application_namespace = application.namespace
application_id = application.id
application_description = application.description
```

---


### Event_integration

EventIntegration resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_token` | String |  | <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the
            request. If not provided, the Amazon Web Services
            SDK populates this field. For more information about idempotency, see
            <a href="https://aws.amazon.com/builders-library/making-retries-safe-with-idempotent-APIs/">Making retries safe with idempotent APIs</a>.</p> |
| `name` | String | ✅ | <p>The name of the event integration.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags used to organize, track, or control access for this resource. For example, { "tags": {"key1":"value1", "key2":"value2"} }.</p> |
| `event_bridge_bus` | String | ✅ | <p>The EventBridge bus.</p> |
| `description` | String |  | <p>The description of the event integration.</p> |
| `event_filter` | String | ✅ | <p>The event filter.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `event_integration_arn` | String | <p>The Amazon Resource Name (ARN) for the event integration.</p> |
| `description` | String | <p>The description of the event integration.</p> |
| `name` | String | <p>The name of the event integration. </p> |
| `event_bridge_bus` | String | <p>The EventBridge bus.</p> |
| `event_filter` | String | <p>The event filter.</p> |
| `tags` | HashMap<String, String> | <p>The tags used to organize, track, or control access for this resource. For example, { "tags": {"key1":"value1", "key2":"value2"} }.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create event_integration
event_integration = provider.appintegrations.Event_integration {
    name = "value"  # <p>The name of the event integration.</p>
    event_bridge_bus = "value"  # <p>The EventBridge bus.</p>
    event_filter = "value"  # <p>The event filter.</p>
}

# Access event_integration outputs
event_integration_id = event_integration.id
event_integration_event_integration_arn = event_integration.event_integration_arn
event_integration_description = event_integration.description
event_integration_name = event_integration.name
event_integration_event_bridge_bus = event_integration.event_bridge_bus
event_integration_event_filter = event_integration.event_filter
event_integration_tags = event_integration.tags
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple data_integration resources
data_integration_0 = provider.appintegrations.Data_integration {
    name = "value-0"
    kms_key = "value-0"
}
data_integration_1 = provider.appintegrations.Data_integration {
    name = "value-1"
    kms_key = "value-1"
}
data_integration_2 = provider.appintegrations.Data_integration {
    name = "value-2"
    kms_key = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    data_integration = provider.appintegrations.Data_integration {
        name = "production-value"
        kms_key = "production-value"
    }
```

---

## Related Documentation

- [AWS Appintegrations Documentation](https://docs.aws.amazon.com/appintegrations/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
