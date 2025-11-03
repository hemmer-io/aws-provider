# Kafkaconnect Service



**Resources**: 4

---

## Overview

The kafkaconnect service provides access to 4 resource types:

- [Custom_plugin](#custom_plugin) [CRD]
- [Connector_operation](#connector_operation) [R]
- [Connector](#connector) [CRUD]
- [Worker_configuration](#worker_configuration) [CRD]

---

## Resources


### Custom_plugin

CustomPlugin resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | <p>A summary description of the custom plugin.</p> |
| `content_type` | String | ✅ | <p>The type of the plugin file.</p> |
| `location` | String | ✅ | <p>Information about the location of a custom plugin.</p> |
| `name` | String | ✅ | <p>The name of the custom plugin.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags you want to attach to the custom plugin.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | <p>The name of the custom plugin.</p> |
| `state_description` | String | <p>Details about the state of a custom plugin.</p> |
| `custom_plugin_state` | String | <p>The state of the custom plugin.</p> |
| `custom_plugin_arn` | String | <p>The Amazon Resource Name (ARN) of the custom plugin.</p> |
| `creation_time` | String | <p>The time that the custom plugin was created.</p> |
| `description` | String | <p>The description of the custom plugin.</p> |
| `latest_revision` | String | <p>The latest successfully created revision of the custom plugin. If there are no
         successfully created revisions, this field will be absent.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create custom_plugin
custom_plugin = provider.kafkaconnect.Custom_plugin {
    content_type = "value"  # <p>The type of the plugin file.</p>
    location = "value"  # <p>Information about the location of a custom plugin.</p>
    name = "value"  # <p>The name of the custom plugin.</p>
}

# Access custom_plugin outputs
custom_plugin_id = custom_plugin.id
custom_plugin_name = custom_plugin.name
custom_plugin_state_description = custom_plugin.state_description
custom_plugin_custom_plugin_state = custom_plugin.custom_plugin_state
custom_plugin_custom_plugin_arn = custom_plugin.custom_plugin_arn
custom_plugin_creation_time = custom_plugin.creation_time
custom_plugin_description = custom_plugin.description
custom_plugin_latest_revision = custom_plugin.latest_revision
```

---


### Connector_operation

ConnectorOperation resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `error_info` | String |  |
| `end_time` | String | <p>The time when the operation ended.</p> |
| `connector_operation_state` | String | <p>The state of the connector operation.</p> |
| `target_connector_configuration` | HashMap<String, String> | <p>The target connector configuration.</p> |
| `origin_worker_setting` | String | <p>The origin worker setting.</p> |
| `connector_operation_arn` | String | <p>The Amazon Resource Name (ARN) of the connector operation.</p> |
| `operation_steps` | Vec<String> | <p>The array of operation steps taken.</p> |
| `connector_arn` | String | <p>The Amazon Resource Name (ARN) of the connector.</p> |
| `origin_connector_configuration` | HashMap<String, String> | <p>The origin connector configuration.</p> |
| `target_worker_setting` | String | <p>The target worker setting.</p> |
| `creation_time` | String | <p>The time when the operation was created.</p> |
| `connector_operation_type` | String | <p>The type of connector operation performed.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access connector_operation outputs
connector_operation_id = connector_operation.id
connector_operation_error_info = connector_operation.error_info
connector_operation_end_time = connector_operation.end_time
connector_operation_connector_operation_state = connector_operation.connector_operation_state
connector_operation_target_connector_configuration = connector_operation.target_connector_configuration
connector_operation_origin_worker_setting = connector_operation.origin_worker_setting
connector_operation_connector_operation_arn = connector_operation.connector_operation_arn
connector_operation_operation_steps = connector_operation.operation_steps
connector_operation_connector_arn = connector_operation.connector_arn
connector_operation_origin_connector_configuration = connector_operation.origin_connector_configuration
connector_operation_target_worker_setting = connector_operation.target_worker_setting
connector_operation_creation_time = connector_operation.creation_time
connector_operation_connector_operation_type = connector_operation.connector_operation_type
```

---


### Connector

Connector resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `kafka_cluster_client_authentication` | String | ✅ | <p>Details of the client authentication used by the Apache Kafka cluster.</p> |
| `plugins` | Vec<String> | ✅ | <important>
            <p>Amazon MSK Connect does not currently support specifying multiple plugins as a list. To use more than one plugin for your connector, you can create a single custom plugin using a ZIP file that bundles multiple plugins together.</p>
         </important>
         <p>Specifies which plugin to use for the connector. You must specify a single-element list containing one <code>customPlugin</code> object.</p> |
| `worker_configuration` | String |  | <p>Specifies which worker configuration to use with the connector.</p> |
| `capacity` | String | ✅ | <p>Information about the capacity allocated to the connector. Exactly one of the two
         properties must be specified.</p> |
| `connector_configuration` | HashMap<String, String> | ✅ | <p>A map of keys to values that represent the configuration for the connector.</p> |
| `connector_description` | String |  | <p>A summary description of the connector.</p> |
| `log_delivery` | String |  | <p>Details about log delivery.</p> |
| `connector_name` | String | ✅ | <p>The name of the connector.</p> |
| `kafka_cluster_encryption_in_transit` | String | ✅ | <p>Details of encryption in transit to the Apache Kafka cluster.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags you want to attach to the connector.</p> |
| `kafka_cluster` | String | ✅ | <p>Specifies which Apache Kafka cluster to connect to.</p> |
| `service_execution_role_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the IAM role used by the connector to access the
         Amazon Web Services resources that it needs. The types of resources depends on the logic of
         the connector. For example, a connector that has Amazon S3 as a destination must have
         permissions that allow it to write to the S3 destination bucket.</p> |
| `kafka_connect_version` | String | ✅ | <p>The version of Kafka Connect. It has to be compatible with both the Apache Kafka
         cluster's version and the plugins.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `creation_time` | String | <p>The time the connector was created.</p> |
| `kafka_cluster` | String | <p>The Apache Kafka cluster that the connector is connected to.</p> |
| `capacity` | String | <p>Information about the capacity of the connector, whether it is auto scaled or
         provisioned.</p> |
| `connector_arn` | String | <p>The Amazon Resource Name (ARN) of the connector.</p> |
| `kafka_cluster_client_authentication` | String | <p>The type of client authentication used to connect to the Apache Kafka cluster. The value
         is NONE when no client authentication is used.</p> |
| `state_description` | String | <p>Details about the state of a connector.</p> |
| `connector_description` | String | <p>A summary description of the connector.</p> |
| `connector_configuration` | HashMap<String, String> | <p>A map of keys to values that represent the configuration for the connector.</p> |
| `current_version` | String | <p>The current version of the connector.</p> |
| `kafka_cluster_encryption_in_transit` | String | <p>Details of encryption in transit to the Apache Kafka cluster.</p> |
| `connector_state` | String | <p>The state of the connector.</p> |
| `kafka_connect_version` | String | <p>The version of Kafka Connect. It has to be compatible with both the Apache Kafka
         cluster's version and the plugins.</p> |
| `log_delivery` | String | <p>Details about delivering logs to Amazon CloudWatch Logs.</p> |
| `service_execution_role_arn` | String | <p>The Amazon Resource Name (ARN) of the IAM role used by the connector to access Amazon
         Web Services resources.</p> |
| `worker_configuration` | String | <p>Specifies which worker configuration was used for the connector.</p> |
| `connector_name` | String | <p>The name of the connector.</p> |
| `plugins` | Vec<String> | <p>Specifies which plugins were used for this connector.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create connector
connector = provider.kafkaconnect.Connector {
    kafka_cluster_client_authentication = "value"  # <p>Details of the client authentication used by the Apache Kafka cluster.</p>
    plugins = "value"  # <important>
            <p>Amazon MSK Connect does not currently support specifying multiple plugins as a list. To use more than one plugin for your connector, you can create a single custom plugin using a ZIP file that bundles multiple plugins together.</p>
         </important>
         <p>Specifies which plugin to use for the connector. You must specify a single-element list containing one <code>customPlugin</code> object.</p>
    capacity = "value"  # <p>Information about the capacity allocated to the connector. Exactly one of the two
         properties must be specified.</p>
    connector_configuration = "value"  # <p>A map of keys to values that represent the configuration for the connector.</p>
    connector_name = "value"  # <p>The name of the connector.</p>
    kafka_cluster_encryption_in_transit = "value"  # <p>Details of encryption in transit to the Apache Kafka cluster.</p>
    kafka_cluster = "value"  # <p>Specifies which Apache Kafka cluster to connect to.</p>
    service_execution_role_arn = "value"  # <p>The Amazon Resource Name (ARN) of the IAM role used by the connector to access the
         Amazon Web Services resources that it needs. The types of resources depends on the logic of
         the connector. For example, a connector that has Amazon S3 as a destination must have
         permissions that allow it to write to the S3 destination bucket.</p>
    kafka_connect_version = "value"  # <p>The version of Kafka Connect. It has to be compatible with both the Apache Kafka
         cluster's version and the plugins.</p>
}

# Access connector outputs
connector_id = connector.id
connector_creation_time = connector.creation_time
connector_kafka_cluster = connector.kafka_cluster
connector_capacity = connector.capacity
connector_connector_arn = connector.connector_arn
connector_kafka_cluster_client_authentication = connector.kafka_cluster_client_authentication
connector_state_description = connector.state_description
connector_connector_description = connector.connector_description
connector_connector_configuration = connector.connector_configuration
connector_current_version = connector.current_version
connector_kafka_cluster_encryption_in_transit = connector.kafka_cluster_encryption_in_transit
connector_connector_state = connector.connector_state
connector_kafka_connect_version = connector.kafka_connect_version
connector_log_delivery = connector.log_delivery
connector_service_execution_role_arn = connector.service_execution_role_arn
connector_worker_configuration = connector.worker_configuration
connector_connector_name = connector.connector_name
connector_plugins = connector.plugins
```

---


### Worker_configuration

WorkerConfiguration resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `properties_file_content` | String | ✅ | <p>Base64 encoded contents of connect-distributed.properties file.</p> |
| `description` | String |  | <p>A summary description of the worker configuration.</p> |
| `name` | String | ✅ | <p>The name of the worker configuration.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags you want to attach to the worker configuration.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `creation_time` | String | <p>The time that the worker configuration was created.</p> |
| `latest_revision` | String | <p>The latest revision of the custom configuration.</p> |
| `name` | String | <p>The name of the worker configuration.</p> |
| `worker_configuration_arn` | String | <p>The Amazon Resource Name (ARN) of the custom configuration.</p> |
| `worker_configuration_state` | String | <p>The state of the worker configuration.</p> |
| `description` | String | <p>The description of the worker configuration.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create worker_configuration
worker_configuration = provider.kafkaconnect.Worker_configuration {
    properties_file_content = "value"  # <p>Base64 encoded contents of connect-distributed.properties file.</p>
    name = "value"  # <p>The name of the worker configuration.</p>
}

# Access worker_configuration outputs
worker_configuration_id = worker_configuration.id
worker_configuration_creation_time = worker_configuration.creation_time
worker_configuration_latest_revision = worker_configuration.latest_revision
worker_configuration_name = worker_configuration.name
worker_configuration_worker_configuration_arn = worker_configuration.worker_configuration_arn
worker_configuration_worker_configuration_state = worker_configuration.worker_configuration_state
worker_configuration_description = worker_configuration.description
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple custom_plugin resources
custom_plugin_0 = provider.kafkaconnect.Custom_plugin {
    content_type = "value-0"
    location = "value-0"
    name = "value-0"
}
custom_plugin_1 = provider.kafkaconnect.Custom_plugin {
    content_type = "value-1"
    location = "value-1"
    name = "value-1"
}
custom_plugin_2 = provider.kafkaconnect.Custom_plugin {
    content_type = "value-2"
    location = "value-2"
    name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    custom_plugin = provider.kafkaconnect.Custom_plugin {
        content_type = "production-value"
        location = "production-value"
        name = "production-value"
    }
```

---

## Related Documentation

- [AWS Kafkaconnect Documentation](https://docs.aws.amazon.com/kafkaconnect/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
