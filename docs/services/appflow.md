# Appflow Service



**Resources**: 8

---

## Overview

The appflow service provides access to 8 resource types:

- [Connector_entity](#connector_entity) [R]
- [Connector](#connector) [R]
- [Connector_profiles](#connector_profiles) [R]
- [Flow](#flow) [CRUD]
- [Connectors](#connectors) [R]
- [Flow_execution_records](#flow_execution_records) [R]
- [Connector_registration](#connector_registration) [U]
- [Connector_profile](#connector_profile) [CUD]

---

## Resources


### Connector_entity

ConnectorEntity resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `connector_entity_fields` | Vec<String> | <p> Describes the fields for that connector entity. For example, for an
        <i>account</i> entity, the fields would be <i>account name</i>,
        <i>account ID</i>, and so on. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access connector_entity outputs
connector_entity_id = connector_entity.id
connector_entity_connector_entity_fields = connector_entity.connector_entity_fields
```

---


### Connector

Connector resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `connector_configuration` | String | <p>Configuration info of all the connectors that the user requested.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access connector outputs
connector_id = connector.id
connector_connector_configuration = connector.connector_configuration
```

---


### Connector_profiles

ConnectorProfiles resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p> The pagination token for the next page of data. If <code>nextToken=null</code>, this
      means that all records have been fetched. </p> |
| `connector_profile_details` | Vec<String> | <p> Returns information about the connector profiles associated with the flow. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access connector_profiles outputs
connector_profiles_id = connector_profiles.id
connector_profiles_next_token = connector_profiles.next_token
connector_profiles_connector_profile_details = connector_profiles.connector_profile_details
```

---


### Flow

Flow resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `kms_arn` | String |  | <p> The ARN (Amazon Resource Name) of the Key Management Service (KMS) key you provide for
      encryption. This is required if you do not want to use the Amazon AppFlow-managed KMS
      key. If you don't provide anything here, Amazon AppFlow uses the Amazon AppFlow-managed KMS key. </p> |
| `trigger_config` | String | ✅ | <p> The trigger settings that determine how and when the flow runs. </p> |
| `destination_flow_config_list` | Vec<String> | ✅ | <p> The configuration that controls how Amazon AppFlow places data in the destination
      connector. </p> |
| `tags` | HashMap<String, String> |  | <p> The tags used to organize, track, or control access for your flow. </p> |
| `tasks` | Vec<String> | ✅ | <p> A list of tasks that Amazon AppFlow performs while transferring the data in the flow
      run. </p> |
| `description` | String |  | <p> A description of the flow you want to create. </p> |
| `metadata_catalog_config` | String |  | <p>Specifies the configuration that Amazon AppFlow uses when it catalogs the data that's
      transferred by the associated flow. When Amazon AppFlow catalogs the data from a flow, it
      stores metadata in a data catalog.</p> |
| `client_token` | String |  | <p>The <code>clientToken</code> parameter is an idempotency token. It ensures that your
        <code>CreateFlow</code> request completes only once. You choose the value to pass. For
      example, if you don't receive a response from your request, you can safely retry the request
      with the same <code>clientToken</code> parameter value.</p>
         <p>If you omit a <code>clientToken</code> value, the Amazon Web Services SDK that you are
      using inserts a value for you. This way, the SDK can safely retry requests multiple times
      after a network error. You must provide your own value for other use cases.</p>
         <p>If you specify input parameters that differ from your first request, an error occurs. If
      you use a different value for <code>clientToken</code>, Amazon AppFlow considers it a new
      call to <code>CreateFlow</code>. The token is active for 8 hours.</p> |
| `source_flow_config` | String | ✅ | <p> The configuration that controls how Amazon AppFlow retrieves data from the source
      connector. </p> |
| `flow_name` | String | ✅ | <p> The specified name of the flow. Spaces are not allowed. Use underscores (_) or hyphens
      (-) only. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `flow_status_message` | String | <p> Contains an error message if the flow status is in a suspended or error state. This
      applies only to scheduled or event-triggered flows. </p> |
| `flow_arn` | String | <p> The flow's Amazon Resource Name (ARN). </p> |
| `tasks` | Vec<String> | <p> A list of tasks that Amazon AppFlow performs while transferring the data in the flow
      run. </p> |
| `destination_flow_config_list` | Vec<String> | <p> The configuration that controls how Amazon AppFlow transfers data to the destination
      connector. </p> |
| `kms_arn` | String | <p> The ARN (Amazon Resource Name) of the Key Management Service (KMS) key you provide for
      encryption. This is required if you do not want to use the Amazon AppFlow-managed KMS
      key. If you don't provide anything here, Amazon AppFlow uses the Amazon AppFlow-managed KMS key. </p> |
| `description` | String | <p> A description of the flow. </p> |
| `created_at` | String | <p> Specifies when the flow was created. </p> |
| `last_run_metadata_catalog_details` | Vec<String> | <p>Describes the metadata catalog, metadata table, and data partitions that Amazon AppFlow used for the associated flow run.</p> |
| `last_run_execution_details` | String | <p> Describes the details of the most recent flow run. </p> |
| `tags` | HashMap<String, String> | <p> The tags used to organize, track, or control access for your flow. </p> |
| `source_flow_config` | String | <p> The configuration that controls how Amazon AppFlow retrieves data from the source
      connector. </p> |
| `flow_status` | String | <p> Indicates the current status of the flow. </p> |
| `created_by` | String | <p> The ARN of the user who created the flow. </p> |
| `last_updated_at` | String | <p> Specifies when the flow was last updated. </p> |
| `schema_version` | i64 | <p>The version number of your data schema. Amazon AppFlow assigns this version number.
      The version number increases by one when you change any of the following settings in your flow
      configuration:</p>
         <ul>
            <li>
               <p>Source-to-destination field mappings</p>
            </li>
            <li>
               <p>Field data types</p>
            </li>
            <li>
               <p>Partition keys</p>
            </li>
         </ul> |
| `last_updated_by` | String | <p> Specifies the user name of the account that performed the most recent update. </p> |
| `flow_name` | String | <p> The specified name of the flow. Spaces are not allowed. Use underscores (_) or hyphens
      (-) only. </p> |
| `metadata_catalog_config` | String | <p>Specifies the configuration that Amazon AppFlow uses when it catalogs the data that's
      transferred by the associated flow. When Amazon AppFlow catalogs the data from a flow, it
      stores metadata in a data catalog.</p> |
| `trigger_config` | String | <p> The trigger settings that determine how and when the flow runs. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create flow
flow = provider.appflow.Flow {
    trigger_config = "value"  # <p> The trigger settings that determine how and when the flow runs. </p>
    destination_flow_config_list = "value"  # <p> The configuration that controls how Amazon AppFlow places data in the destination
      connector. </p>
    tasks = "value"  # <p> A list of tasks that Amazon AppFlow performs while transferring the data in the flow
      run. </p>
    source_flow_config = "value"  # <p> The configuration that controls how Amazon AppFlow retrieves data from the source
      connector. </p>
    flow_name = "value"  # <p> The specified name of the flow. Spaces are not allowed. Use underscores (_) or hyphens
      (-) only. </p>
}

# Access flow outputs
flow_id = flow.id
flow_flow_status_message = flow.flow_status_message
flow_flow_arn = flow.flow_arn
flow_tasks = flow.tasks
flow_destination_flow_config_list = flow.destination_flow_config_list
flow_kms_arn = flow.kms_arn
flow_description = flow.description
flow_created_at = flow.created_at
flow_last_run_metadata_catalog_details = flow.last_run_metadata_catalog_details
flow_last_run_execution_details = flow.last_run_execution_details
flow_tags = flow.tags
flow_source_flow_config = flow.source_flow_config
flow_flow_status = flow.flow_status
flow_created_by = flow.created_by
flow_last_updated_at = flow.last_updated_at
flow_schema_version = flow.schema_version
flow_last_updated_by = flow.last_updated_by
flow_flow_name = flow.flow_name
flow_metadata_catalog_config = flow.metadata_catalog_config
flow_trigger_config = flow.trigger_config
```

---


### Connectors

Connectors resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `connector_configurations` | HashMap<String, String> | <p> The configuration that is applied to the connectors used in the flow. </p> |
| `connectors` | Vec<String> | <p>Information about the connectors supported in Amazon AppFlow.</p> |
| `next_token` | String | <p> The pagination token for the next page of data. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access connectors outputs
connectors_id = connectors.id
connectors_connector_configurations = connectors.connector_configurations
connectors_connectors = connectors.connectors
connectors_next_token = connectors.next_token
```

---


### Flow_execution_records

FlowExecutionRecords resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p> The pagination token for the next page of data. </p> |
| `flow_executions` | Vec<String> | <p> Returns a list of all instances when this flow was run. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access flow_execution_records outputs
flow_execution_records_id = flow_execution_records.id
flow_execution_records_next_token = flow_execution_records.next_token
flow_execution_records_flow_executions = flow_execution_records.flow_executions
```

---


### Connector_registration

ConnectorRegistration resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `connector_label` | String | ✅ | <p>The name of the connector. The name is unique for each connector registration in your AWS
      account.</p> |
| `connector_provisioning_config` | String |  |  |
| `description` | String |  | <p>A description about the update that you're applying to the connector.</p> |
| `client_token` | String |  | <p>The <code>clientToken</code> parameter is an idempotency token. It ensures that your
        <code>UpdateConnectorRegistration</code> request completes only once. You choose the value
      to pass. For example, if you don't receive a response from your request, you can safely retry
      the request with the same <code>clientToken</code> parameter value.</p>
         <p>If you omit a <code>clientToken</code> value, the Amazon Web Services SDK that you are
      using inserts a value for you. This way, the SDK can safely retry requests multiple times
      after a network error. You must provide your own value for other use cases.</p>
         <p>If you specify input parameters that differ from your first request, an error occurs. If
      you use a different value for <code>clientToken</code>, Amazon AppFlow considers it a new
      call to <code>UpdateConnectorRegistration</code>. The token is active for 8 hours.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

```

---


### Connector_profile

ConnectorProfile resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_token` | String |  | <p>The <code>clientToken</code> parameter is an idempotency token. It ensures that your
        <code>CreateConnectorProfile</code> request completes only once. You choose the value to
      pass. For example, if you don't receive a response from your request, you can safely retry the
      request with the same <code>clientToken</code> parameter value.</p>
         <p>If you omit a <code>clientToken</code> value, the Amazon Web Services SDK that you are
      using inserts a value for you. This way, the SDK can safely retry requests multiple times
      after a network error. You must provide your own value for other use cases.</p>
         <p>If you specify input parameters that differ from your first request, an error occurs. If
      you use a different value for <code>clientToken</code>, Amazon AppFlow considers it a new
      call to <code>CreateConnectorProfile</code>. The token is active for 8 hours.</p> |
| `connector_type` | String | ✅ | <p> The type of connector, such as Salesforce, Amplitude, and so on. </p> |
| `connector_profile_name` | String | ✅ | <p> The name of the connector profile. The name is unique for each
        <code>ConnectorProfile</code> in your Amazon Web Services account. </p> |
| `connection_mode` | String | ✅ | <p> Indicates the connection mode and specifies whether it is public or private. Private
      flows use Amazon Web Services PrivateLink to route data over Amazon Web Services infrastructure
      without exposing it to the public internet. </p> |
| `connector_label` | String |  | <p>The label of the connector. The label is unique for each
        <code>ConnectorRegistration</code> in your Amazon Web Services account. Only needed if
      calling for CUSTOMCONNECTOR connector type/.</p> |
| `kms_arn` | String |  | <p> The ARN (Amazon Resource Name) of the Key Management Service (KMS) key you provide for
      encryption. This is required if you do not want to use the Amazon AppFlow-managed KMS
      key. If you don't provide anything here, Amazon AppFlow uses the Amazon AppFlow-managed KMS key. </p> |
| `connector_profile_config` | String | ✅ | <p> Defines the connector-specific configuration and credentials. </p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create connector_profile
connector_profile = provider.appflow.Connector_profile {
    connector_type = "value"  # <p> The type of connector, such as Salesforce, Amplitude, and so on. </p>
    connector_profile_name = "value"  # <p> The name of the connector profile. The name is unique for each
        <code>ConnectorProfile</code> in your Amazon Web Services account. </p>
    connection_mode = "value"  # <p> Indicates the connection mode and specifies whether it is public or private. Private
      flows use Amazon Web Services PrivateLink to route data over Amazon Web Services infrastructure
      without exposing it to the public internet. </p>
    connector_profile_config = "value"  # <p> Defines the connector-specific configuration and credentials. </p>
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

# Create multiple connector_entity resources
connector_entity_0 = provider.appflow.Connector_entity {
}
connector_entity_1 = provider.appflow.Connector_entity {
}
connector_entity_2 = provider.appflow.Connector_entity {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    connector_entity = provider.appflow.Connector_entity {
    }
```

---

## Related Documentation

- [AWS Appflow Documentation](https://docs.aws.amazon.com/appflow/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
