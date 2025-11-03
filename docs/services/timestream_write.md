# Timestream_write Service



**Resources**: 4

---

## Overview

The timestream_write service provides access to 4 resource types:

- [Endpoints](#endpoints) [R]
- [Batch_load_task](#batch_load_task) [CR]
- [Database](#database) [CRUD]
- [Table](#table) [CRUD]

---

## Resources


### Endpoints

Endpoints resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `endpoints` | Vec<String> | <p>An <code>Endpoints</code> object is returned when a <code>DescribeEndpoints</code>
         request is made.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access endpoints outputs
endpoints_id = endpoints.id
endpoints_endpoints = endpoints.endpoints
```

---


### Batch_load_task

BatchLoadTask resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_token` | String |  | <p></p> |
| `target_database_name` | String | ✅ | <p>Target Timestream database for a batch load task.</p> |
| `target_table_name` | String | ✅ | <p>Target Timestream table for a batch load task.</p> |
| `record_version` | i64 |  | <p></p> |
| `data_source_configuration` | String | ✅ | <p>Defines configuration details about the data source for a batch load task.</p> |
| `data_model_configuration` | String |  |  |
| `report_configuration` | String | ✅ |  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `batch_load_task_description` | String | <p>Description of the batch load task.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create batch_load_task
batch_load_task = provider.timestream_write.Batch_load_task {
    target_database_name = "value"  # <p>Target Timestream database for a batch load task.</p>
    target_table_name = "value"  # <p>Target Timestream table for a batch load task.</p>
    data_source_configuration = "value"  # <p>Defines configuration details about the data source for a batch load task.</p>
    report_configuration = "value"  # Required field
}

# Access batch_load_task outputs
batch_load_task_id = batch_load_task.id
batch_load_task_batch_load_task_description = batch_load_task.batch_load_task_description
```

---


### Database

Database resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `database_name` | String | ✅ | <p>The name of the Timestream database.</p> |
| `kms_key_id` | String |  | <p>The KMS key for the database. If the KMS key is not
         specified, the database will be encrypted with a Timestream managed KMS key located in your account. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#aws-managed-cmk">Amazon Web Services managed keys</a>.</p> |
| `tags` | Vec<String> |  | <p> A list of key-value pairs to label the table. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `database` | String | <p>The name of the Timestream table.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create database
database = provider.timestream_write.Database {
    database_name = "value"  # <p>The name of the Timestream database.</p>
}

# Access database outputs
database_id = database.id
database_database = database.database
```

---


### Table

Table resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `magnetic_store_write_properties` | String |  | <p>Contains properties to set on the table when enabling magnetic store writes.</p> |
| `table_name` | String | ✅ | <p>The name of the Timestream table.</p> |
| `tags` | Vec<String> |  | <p> A list of key-value pairs to label the table. </p> |
| `retention_properties` | String |  | <p>The duration for which your time-series data must be stored in the memory store and the
         magnetic store.</p> |
| `schema` | String |  | <p> The schema of the table. </p> |
| `database_name` | String | ✅ | <p>The name of the Timestream database.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `table` | String | <p>The Timestream table.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create table
table = provider.timestream_write.Table {
    table_name = "value"  # <p>The name of the Timestream table.</p>
    database_name = "value"  # <p>The name of the Timestream database.</p>
}

# Access table outputs
table_id = table.id
table_table = table.table
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple endpoints resources
endpoints_0 = provider.timestream_write.Endpoints {
}
endpoints_1 = provider.timestream_write.Endpoints {
}
endpoints_2 = provider.timestream_write.Endpoints {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    endpoints = provider.timestream_write.Endpoints {
    }
```

---

## Related Documentation

- [AWS Timestream_write Documentation](https://docs.aws.amazon.com/timestream_write/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
