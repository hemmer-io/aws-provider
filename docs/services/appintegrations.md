# Appintegrations Service



**Resources**: 4

---

## Overview

The appintegrations service provides access to 4 resource types:

- [Application](#application) [CRUD]
- [Event_integration](#event_integration) [CRUD]
- [Data_integration](#data_integration) [CRUD]
- [Data_integration_association](#data_integration_association) [CU]

---

## Resources


### Application

Application resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `subscriptions` | Vec<String> |  | <p>The events that the application subscribes.</p> |
| `initialization_timeout` | i64 |  | <p>The maximum time in milliseconds allowed to establish a connection with the workspace.</p> |
| `application_config` | String |  | <p>The configuration settings for the application.</p> |
| `iframe_config` | String |  | <p>The iframe configuration for the application.</p> |
| `namespace` | String | ✅ | <p>The namespace of the application.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags used to organize, track, or control access for this resource. For example, { "tags": {"key1":"value1", "key2":"value2"} }.</p> |
| `permissions` | Vec<String> |  | <p>The configuration of events or requests that the application has access to.</p> |
| `description` | String |  | <p>The description of the application.</p> |
| `client_token` | String |  | <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the
            request. If not provided, the Amazon Web Services
            SDK populates this field. For more information about idempotency, see
            <a href="https://aws.amazon.com/builders-library/making-retries-safe-with-idempotent-APIs/">Making retries safe with idempotent APIs</a>.</p> |
| `is_service` | bool |  | <p>Indicates whether the application is a service.</p> |
| `publications` | Vec<String> |  | <p>The events that the application publishes.</p> |
| `name` | String | ✅ | <p>The name of the application.</p> |
| `application_source_config` | String | ✅ | <p>The configuration for where the application should be loaded from.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `id` | String | <p>A unique identifier for the Application.</p> |
| `initialization_timeout` | i64 | <p>The maximum time in milliseconds allowed to establish a connection with the workspace.</p> |
| `subscriptions` | Vec<String> | <p>The events that the application subscribes.</p> |
| `description` | String | <p>The description of the application.</p> |
| `namespace` | String | <p>The namespace of the application.</p> |
| `publications` | Vec<String> | <p>The events that the application publishes.</p> |
| `tags` | HashMap<String, String> | <p>The tags used to organize, track, or control access for this resource. For example, { "tags": {"key1":"value1", "key2":"value2"} }.</p> |
| `last_modified_time` | String | <p>The last modified time of the Application.</p> |
| `iframe_config` | String | <p>The iframe configuration for the application.</p> |
| `application_source_config` | String | <p>The configuration for where the application should be loaded from.</p> |
| `created_time` | String | <p>The created time of the Application.</p> |
| `arn` | String | <p>The Amazon Resource Name (ARN) of the Application.</p> |
| `name` | String | <p>The name of the application.</p> |
| `application_config` | String | <p>The configuration settings for the application.</p> |
| `is_service` | bool | <p>Indicates whether the application is a service.</p> |
| `permissions` | Vec<String> | <p>The configuration of events or requests that the application has access to.</p> |


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
application_id = application.id
application_initialization_timeout = application.initialization_timeout
application_subscriptions = application.subscriptions
application_description = application.description
application_namespace = application.namespace
application_publications = application.publications
application_tags = application.tags
application_last_modified_time = application.last_modified_time
application_iframe_config = application.iframe_config
application_application_source_config = application.application_source_config
application_created_time = application.created_time
application_arn = application.arn
application_name = application.name
application_application_config = application.application_config
application_is_service = application.is_service
application_permissions = application.permissions
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
| `description` | String |  | <p>The description of the event integration.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags used to organize, track, or control access for this resource. For example, { "tags": {"key1":"value1", "key2":"value2"} }.</p> |
| `event_filter` | String | ✅ | <p>The event filter.</p> |
| `event_bridge_bus` | String | ✅ | <p>The EventBridge bus.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `event_filter` | String | <p>The event filter.</p> |
| `tags` | HashMap<String, String> | <p>The tags used to organize, track, or control access for this resource. For example, { "tags": {"key1":"value1", "key2":"value2"} }.</p> |
| `name` | String | <p>The name of the event integration. </p> |
| `event_integration_arn` | String | <p>The Amazon Resource Name (ARN) for the event integration.</p> |
| `description` | String | <p>The description of the event integration.</p> |
| `event_bridge_bus` | String | <p>The EventBridge bus.</p> |


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
    event_filter = "value"  # <p>The event filter.</p>
    event_bridge_bus = "value"  # <p>The EventBridge bus.</p>
}

# Access event_integration outputs
event_integration_id = event_integration.id
event_integration_event_filter = event_integration.event_filter
event_integration_tags = event_integration.tags
event_integration_name = event_integration.name
event_integration_event_integration_arn = event_integration.event_integration_arn
event_integration_description = event_integration.description
event_integration_event_bridge_bus = event_integration.event_bridge_bus
```

---


### Data_integration

DataIntegration resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `file_configuration` | String |  | <p>The configuration for what files should be pulled from the source.</p> |
| `object_configuration` | HashMap<String, HashMap<String, Vec<String>>> |  | <p>The configuration for what data should be pulled from the source.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags used to organize, track, or control access for this resource. For example, { "tags": {"key1":"value1", "key2":"value2"} }.</p> |
| `name` | String | ✅ | <p>The name of the DataIntegration.</p> |
| `description` | String |  | <p>A description of the DataIntegration.</p> |
| `source_uri` | String |  | <p>The URI of the data source.</p> |
| `kms_key` | String | ✅ | <p>The KMS key ARN for the DataIntegration.</p> |
| `schedule_config` | String |  | <p>The name of the data and how often it should be pulled from the source.</p> |
| `client_token` | String |  | <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the
            request. If not provided, the Amazon Web Services
            SDK populates this field. For more information about idempotency, see
            <a href="https://aws.amazon.com/builders-library/making-retries-safe-with-idempotent-APIs/">Making retries safe with idempotent APIs</a>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | <p>The KMS key ARN for the DataIntegration.</p> |
| `name` | String | <p>The name of the DataIntegration.</p> |
| `kms_key` | String | <p>The KMS key ARN for the DataIntegration.</p> |
| `file_configuration` | String | <p>The configuration for what files should be pulled from the source.</p> |
| `schedule_configuration` | String | <p>The name of the data and how often it should be pulled from the source.</p> |
| `source_uri` | String | <p>The URI of the data source.</p> |
| `arn` | String | <p>The Amazon Resource Name (ARN) for the DataIntegration.</p> |
| `id` | String | <p>A unique identifier.</p> |
| `tags` | HashMap<String, String> | <p>The tags used to organize, track, or control access for this resource. For example, { "tags": {"key1":"value1", "key2":"value2"} }.</p> |
| `object_configuration` | HashMap<String, HashMap<String, Vec<String>>> | <p>The configuration for what data should be pulled from the source.</p> |


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
data_integration_description = data_integration.description
data_integration_name = data_integration.name
data_integration_kms_key = data_integration.kms_key
data_integration_file_configuration = data_integration.file_configuration
data_integration_schedule_configuration = data_integration.schedule_configuration
data_integration_source_uri = data_integration.source_uri
data_integration_arn = data_integration.arn
data_integration_id = data_integration.id
data_integration_tags = data_integration.tags
data_integration_object_configuration = data_integration.object_configuration
```

---


### Data_integration_association

DataIntegrationAssociation resource

**Operations**: ✅ Create ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `data_integration_identifier` | String | ✅ | <p>A unique identifier for the DataIntegration.</p> |
| `client_association_metadata` | HashMap<String, String> |  | <p>The mapping of metadata to be extracted from the data.</p> |
| `client_id` | String |  | <p>The identifier for the client that is associated with the DataIntegration
      association.</p> |
| `destination_uri` | String |  | <p>The URI of the data destination.</p> |
| `client_token` | String |  | <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the
            request. If not provided, the Amazon Web Services
            SDK populates this field. For more information about idempotency, see
            <a href="https://aws.amazon.com/builders-library/making-retries-safe-with-idempotent-APIs/">Making retries safe with idempotent APIs</a>.</p> |
| `execution_configuration` | String |  | <p>The configuration for how the files should be pulled from the source.</p> |
| `object_configuration` | HashMap<String, HashMap<String, Vec<String>>> |  |  |



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



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple application resources
application_0 = provider.appintegrations.Application {
    namespace = "value-0"
    name = "value-0"
    application_source_config = "value-0"
}
application_1 = provider.appintegrations.Application {
    namespace = "value-1"
    name = "value-1"
    application_source_config = "value-1"
}
application_2 = provider.appintegrations.Application {
    namespace = "value-2"
    name = "value-2"
    application_source_config = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    application = provider.appintegrations.Application {
        namespace = "production-value"
        name = "production-value"
        application_source_config = "production-value"
    }
```

---

## Related Documentation

- [AWS Appintegrations Documentation](https://docs.aws.amazon.com/appintegrations/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
