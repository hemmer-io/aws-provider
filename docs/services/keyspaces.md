# Keyspaces Service



**Resources**: 4

---

## Overview

The keyspaces service provides access to 4 resource types:

- [Keyspace](#keyspace) [CRUD]
- [Table_auto_scaling_settings](#table_auto_scaling_settings) [R]
- [Table](#table) [CRUD]
- [Type](#type) [CRD]

---

## Resources


### Keyspace

Keyspace resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>A list of key-value pair tags to be attached to the keyspace.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/keyspaces/latest/devguide/tagging-keyspaces.html">Adding tags and labels to Amazon Keyspaces resources</a> in the <i>Amazon Keyspaces Developer Guide</i>.</p> |
| `keyspace_name` | String | ✅ | <p>The name of the keyspace to be created.</p> |
| `replication_specification` | String |  | <p> The replication specification of the keyspace includes:</p> <ul> <li> <p> <code>replicationStrategy</code> - the required value is <code>SINGLE_REGION</code> or <code>MULTI_REGION</code>.</p> </li> <li> <p> <code>regionList</code> - if the <code>replicationStrategy</code> is <code>MULTI_REGION</code>, the <code>regionList</code> requires the current Region and at least one additional Amazon Web Services Region where the keyspace is going to be replicated in.</p> </li> </ul> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `replication_regions` | Vec<String> | <p> If the <code>replicationStrategy</code> of the keyspace is <code>MULTI_REGION</code>, a list of replication Regions is returned. </p> |
| `keyspace_name` | String | <p>The name of the keyspace.</p> |
| `replication_strategy` | String | <p> Returns the replication strategy of the keyspace. The options are <code>SINGLE_REGION</code> or <code>MULTI_REGION</code>. </p> |
| `replication_group_statuses` | Vec<String> | <p> A list of all Regions the keyspace is replicated in after the update keyspace operation and their status. </p> |
| `resource_arn` | String | <p>Returns the ARN of the keyspace.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create keyspace
keyspace = provider.keyspaces.Keyspace {
    keyspace_name = "value"  # <p>The name of the keyspace to be created.</p>
}

# Access keyspace outputs
keyspace_id = keyspace.id
keyspace_replication_regions = keyspace.replication_regions
keyspace_keyspace_name = keyspace.keyspace_name
keyspace_replication_strategy = keyspace.replication_strategy
keyspace_replication_group_statuses = keyspace.replication_group_statuses
keyspace_resource_arn = keyspace.resource_arn
```

---


### Table_auto_scaling_settings

TableAutoScalingSettings resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `resource_arn` | String | <p>The Amazon Resource Name (ARN) of the table.</p> |
| `auto_scaling_specification` | String | <p>The auto scaling settings of the table.</p> |
| `replica_specifications` | Vec<String> | <p>The Amazon Web Services Region specific settings of a multi-Region table. Returns the settings for all Regions the table is replicated in.</p> |
| `keyspace_name` | String | <p>The name of the keyspace.</p> |
| `table_name` | String | <p>The name of the table.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access table_auto_scaling_settings outputs
table_auto_scaling_settings_id = table_auto_scaling_settings.id
table_auto_scaling_settings_resource_arn = table_auto_scaling_settings.resource_arn
table_auto_scaling_settings_auto_scaling_specification = table_auto_scaling_settings.auto_scaling_specification
table_auto_scaling_settings_replica_specifications = table_auto_scaling_settings.replica_specifications
table_auto_scaling_settings_keyspace_name = table_auto_scaling_settings.keyspace_name
table_auto_scaling_settings_table_name = table_auto_scaling_settings.table_name
```

---


### Table

Table resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `encryption_specification` | String |  | <p>Specifies how the encryption key for encryption at rest is managed for the table. You can choose one of the following KMS key (KMS key):</p> <ul> <li> <p> <code>type:AWS_OWNED_KMS_KEY</code> - This key is owned by Amazon Keyspaces. </p> </li> <li> <p> <code>type:CUSTOMER_MANAGED_KMS_KEY</code> - This key is stored in your account and is created, owned, and managed by you. This option requires the <code>kms_key_identifier</code> of the KMS key in Amazon Resource Name (ARN) format as input.</p> </li> </ul> <p>The default is <code>type:AWS_OWNED_KMS_KEY</code>.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/keyspaces/latest/devguide/EncryptionAtRest.html">Encryption at rest</a> in the <i>Amazon Keyspaces Developer Guide</i>.</p> |
| `client_side_timestamps` | String |  | <p> Enables client-side timestamps for the table. By default, the setting is disabled. You can enable client-side timestamps with the following option:</p> <ul> <li> <p> <code>status: "enabled"</code> </p> </li> </ul> <p>Once client-side timestamps are enabled for a table, this setting cannot be disabled.</p> |
| `auto_scaling_specification` | String |  | <p>The optional auto scaling settings for a table in provisioned capacity mode. Specifies if the service can manage throughput capacity automatically on your behalf.</p> <p>Auto scaling helps you provision throughput capacity for variable workloads efficiently by increasing and decreasing your table's read and write capacity automatically in response to application traffic. For more information, see <a href="https://docs.aws.amazon.com/keyspaces/latest/devguide/autoscaling.html">Managing throughput capacity automatically with Amazon Keyspaces auto scaling</a> in the <i>Amazon Keyspaces Developer Guide</i>.</p> <p>By default, auto scaling is disabled for a table. </p> |
| `cdc_specification` | String |  | <p>The CDC stream settings of the table.</p> |
| `ttl` | String |  | <p>Enables Time to Live custom settings for the table. The options are:</p> <ul> <li> <p> <code>status:enabled</code> </p> </li> <li> <p> <code>status:disabled</code> </p> </li> </ul> <p>The default is <code>status:disabled</code>. After <code>ttl</code> is enabled, you can't disable it for the table.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/keyspaces/latest/devguide/TTL.html">Expiring data by using Amazon Keyspaces Time to Live (TTL)</a> in the <i>Amazon Keyspaces Developer Guide</i>.</p> |
| `point_in_time_recovery` | String |  | <p>Specifies if <code>pointInTimeRecovery</code> is enabled or disabled for the table. The options are:</p> <ul> <li> <p> <code>status=ENABLED</code> </p> </li> <li> <p> <code>status=DISABLED</code> </p> </li> </ul> <p>If it's not specified, the default is <code>status=DISABLED</code>.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/keyspaces/latest/devguide/PointInTimeRecovery.html">Point-in-time recovery</a> in the <i>Amazon Keyspaces Developer Guide</i>.</p> |
| `schema_definition` | String | ✅ | <p>The <code>schemaDefinition</code> consists of the following parameters.</p> <p>For each column to be created:</p> <ul> <li> <p> <code>name</code> - The name of the column.</p> </li> <li> <p> <code>type</code> - An Amazon Keyspaces data type. For more information, see <a href="https://docs.aws.amazon.com/keyspaces/latest/devguide/cql.elements.html#cql.data-types">Data types</a> in the <i>Amazon Keyspaces Developer Guide</i>.</p> </li> </ul> <p>The primary key of the table consists of the following columns:</p> <ul> <li> <p> <code>partitionKeys</code> - The partition key can be a single column, or it can be a compound value composed of two or more columns. The partition key portion of the primary key is required and determines how Amazon Keyspaces stores your data.</p> </li> <li> <p> <code>name</code> - The name of each partition key column.</p> </li> <li> <p> <code>clusteringKeys</code> - The optional clustering column portion of your primary key determines how the data is clustered and sorted within each partition.</p> </li> <li> <p> <code>name</code> - The name of the clustering column. </p> </li> <li> <p> <code>orderBy</code> - Sets the ascendant (<code>ASC</code>) or descendant (<code>DESC</code>) order modifier.</p> <p>To define a column as static use <code>staticColumns</code> - Static columns store values that are shared by all rows in the same partition:</p> </li> <li> <p> <code>name</code> - The name of the column.</p> </li> <li> <p> <code>type</code> - An Amazon Keyspaces data type.</p> </li> </ul> |
| `table_name` | String | ✅ | <p>The name of the table.</p> |
| `keyspace_name` | String | ✅ | <p>The name of the keyspace that the table is going to be created in.</p> |
| `default_time_to_live` | i64 |  | <p>The default Time to Live setting in seconds for the table.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/keyspaces/latest/devguide/TTL-how-it-works.html#ttl-howitworks_default_ttl">Setting the default TTL value for a table</a> in the <i>Amazon Keyspaces Developer Guide</i>.</p> |
| `tags` | Vec<String> |  | <p>A list of key-value pair tags to be attached to the resource. </p> <p>For more information, see <a href="https://docs.aws.amazon.com/keyspaces/latest/devguide/tagging-keyspaces.html">Adding tags and labels to Amazon Keyspaces resources</a> in the <i>Amazon Keyspaces Developer Guide</i>.</p> |
| `replica_specifications` | Vec<String> |  | <p>The optional Amazon Web Services Region specific settings of a multi-Region table. These settings overwrite the general settings of the table for the specified Region. </p> <p>For a multi-Region table in provisioned capacity mode, you can configure the table's read capacity differently for each Region's replica. The write capacity, however, remains synchronized between all replicas to ensure that there's enough capacity to replicate writes across all Regions. To define the read capacity for a table replica in a specific Region, you can do so by configuring the following parameters.</p> <ul> <li> <p> <code>region</code>: The Region where these settings are applied. (Required)</p> </li> <li> <p> <code>readCapacityUnits</code>: The provisioned read capacity units. (Optional)</p> </li> <li> <p> <code>readCapacityAutoScaling</code>: The read capacity auto scaling settings for the table. (Optional) </p> </li> </ul> |
| `comment` | String |  | <p>This parameter allows to enter a description of the table.</p> |
| `capacity_specification` | String |  | <p>Specifies the read/write throughput capacity mode for the table. The options are:</p> <ul> <li> <p> <code>throughputMode:PAY_PER_REQUEST</code> and </p> </li> <li> <p> <code>throughputMode:PROVISIONED</code> - Provisioned capacity mode requires <code>readCapacityUnits</code> and <code>writeCapacityUnits</code> as input.</p> </li> </ul> <p>The default is <code>throughput_mode:PAY_PER_REQUEST</code>.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/keyspaces/latest/devguide/ReadWriteCapacityMode.html">Read/write capacity modes</a> in the <i>Amazon Keyspaces Developer Guide</i>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `table_name` | String | <p>The name of the specified table.</p> |
| `replica_specifications` | Vec<String> | <p>Returns the Amazon Web Services Region specific settings of all Regions a multi-Region table is replicated in.</p> |
| `encryption_specification` | String | <p>The encryption settings of the specified table.</p> |
| `creation_timestamp` | String | <p>The creation timestamp of the specified table.</p> |
| `cdc_specification` | String | <p>The CDC stream settings of the table.</p> |
| `keyspace_name` | String | <p>The name of the keyspace that the specified table is stored in.</p> |
| `default_time_to_live` | i64 | <p>The default Time to Live settings in seconds of the specified table.</p> |
| `resource_arn` | String | <p>The Amazon Resource Name (ARN) of the specified table.</p> |
| `client_side_timestamps` | String | <p> The client-side timestamps setting of the table.</p> |
| `ttl` | String | <p>The custom Time to Live settings of the specified table.</p> |
| `latest_stream_arn` | String | <p>The Amazon Resource Name (ARN) of the stream.</p> |
| `status` | String | <p>The current status of the specified table.</p> |
| `capacity_specification` | String | <p>The read/write throughput capacity mode for a table. The options are:</p> <ul> <li> <p> <code>throughputMode:PAY_PER_REQUEST</code> </p> </li> <li> <p> <code>throughputMode:PROVISIONED</code> </p> </li> </ul> |
| `point_in_time_recovery` | String | <p>The point-in-time recovery status of the specified table.</p> |
| `comment` | String | <p>The the description of the specified table.</p> |
| `schema_definition` | String | <p>The schema definition of the specified table.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create table
table = provider.keyspaces.Table {
    schema_definition = "value"  # <p>The <code>schemaDefinition</code> consists of the following parameters.</p> <p>For each column to be created:</p> <ul> <li> <p> <code>name</code> - The name of the column.</p> </li> <li> <p> <code>type</code> - An Amazon Keyspaces data type. For more information, see <a href="https://docs.aws.amazon.com/keyspaces/latest/devguide/cql.elements.html#cql.data-types">Data types</a> in the <i>Amazon Keyspaces Developer Guide</i>.</p> </li> </ul> <p>The primary key of the table consists of the following columns:</p> <ul> <li> <p> <code>partitionKeys</code> - The partition key can be a single column, or it can be a compound value composed of two or more columns. The partition key portion of the primary key is required and determines how Amazon Keyspaces stores your data.</p> </li> <li> <p> <code>name</code> - The name of each partition key column.</p> </li> <li> <p> <code>clusteringKeys</code> - The optional clustering column portion of your primary key determines how the data is clustered and sorted within each partition.</p> </li> <li> <p> <code>name</code> - The name of the clustering column. </p> </li> <li> <p> <code>orderBy</code> - Sets the ascendant (<code>ASC</code>) or descendant (<code>DESC</code>) order modifier.</p> <p>To define a column as static use <code>staticColumns</code> - Static columns store values that are shared by all rows in the same partition:</p> </li> <li> <p> <code>name</code> - The name of the column.</p> </li> <li> <p> <code>type</code> - An Amazon Keyspaces data type.</p> </li> </ul>
    table_name = "value"  # <p>The name of the table.</p>
    keyspace_name = "value"  # <p>The name of the keyspace that the table is going to be created in.</p>
}

# Access table outputs
table_id = table.id
table_table_name = table.table_name
table_replica_specifications = table.replica_specifications
table_encryption_specification = table.encryption_specification
table_creation_timestamp = table.creation_timestamp
table_cdc_specification = table.cdc_specification
table_keyspace_name = table.keyspace_name
table_default_time_to_live = table.default_time_to_live
table_resource_arn = table.resource_arn
table_client_side_timestamps = table.client_side_timestamps
table_ttl = table.ttl
table_latest_stream_arn = table.latest_stream_arn
table_status = table.status
table_capacity_specification = table.capacity_specification
table_point_in_time_recovery = table.point_in_time_recovery
table_comment = table.comment
table_schema_definition = table.schema_definition
```

---


### Type

Type resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `type_name` | String | ✅ | <p> The name of the user-defined type. </p> <p>UDT names must contain 48 characters or less, must begin with an alphabetic character, and can only contain alpha-numeric characters and underscores. Amazon Keyspaces converts upper case characters automatically into lower case characters. </p> <p>Alternatively, you can declare a UDT name in double quotes. When declaring a UDT name inside double quotes, Amazon Keyspaces preserves upper casing and allows special characters.</p> <p>You can also use double quotes as part of the name when you create the UDT, but you must escape each double quote character with an additional double quote character.</p> |
| `field_definitions` | Vec<String> | ✅ | <p> The field definitions, consisting of names and types, that define this type. </p> |
| `keyspace_name` | String | ✅ | <p> The name of the keyspace. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `type_name` | String | <p> The name of the type. </p> |
| `field_definitions` | Vec<String> | <p> The names and types that define this type. </p> |
| `last_modified_timestamp` | String | <p> The timestamp that shows when this type was last modified. </p> |
| `direct_referring_tables` | Vec<String> | <p> The tables that use this type. </p> |
| `status` | String | <p> The status of this type. </p> |
| `direct_parent_types` | Vec<String> | <p> The types that use this type. </p> |
| `max_nesting_depth` | i64 | <p> The level of nesting implemented for this type. </p> |
| `keyspace_name` | String | <p> The name of the keyspace that contains this type. </p> |
| `keyspace_arn` | String | <p> The unique identifier of the keyspace that contains this type in the format of an Amazon Resource Name (ARN). </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create type
type = provider.keyspaces.Type {
    type_name = "value"  # <p> The name of the user-defined type. </p> <p>UDT names must contain 48 characters or less, must begin with an alphabetic character, and can only contain alpha-numeric characters and underscores. Amazon Keyspaces converts upper case characters automatically into lower case characters. </p> <p>Alternatively, you can declare a UDT name in double quotes. When declaring a UDT name inside double quotes, Amazon Keyspaces preserves upper casing and allows special characters.</p> <p>You can also use double quotes as part of the name when you create the UDT, but you must escape each double quote character with an additional double quote character.</p>
    field_definitions = "value"  # <p> The field definitions, consisting of names and types, that define this type. </p>
    keyspace_name = "value"  # <p> The name of the keyspace. </p>
}

# Access type outputs
type_id = type.id
type_type_name = type.type_name
type_field_definitions = type.field_definitions
type_last_modified_timestamp = type.last_modified_timestamp
type_direct_referring_tables = type.direct_referring_tables
type_status = type.status
type_direct_parent_types = type.direct_parent_types
type_max_nesting_depth = type.max_nesting_depth
type_keyspace_name = type.keyspace_name
type_keyspace_arn = type.keyspace_arn
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple keyspace resources
keyspace_0 = provider.keyspaces.Keyspace {
    keyspace_name = "value-0"
}
keyspace_1 = provider.keyspaces.Keyspace {
    keyspace_name = "value-1"
}
keyspace_2 = provider.keyspaces.Keyspace {
    keyspace_name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    keyspace = provider.keyspaces.Keyspace {
        keyspace_name = "production-value"
    }
```

---

## Related Documentation

- [AWS Keyspaces Documentation](https://docs.aws.amazon.com/keyspaces/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
