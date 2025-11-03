# Keyspacesstreams Service



**Resources**: 3

---

## Overview

The keyspacesstreams service provides access to 3 resource types:

- [Shard_iterator](#shard_iterator) [R]
- [Records](#records) [R]
- [Stream](#stream) [R]

---

## Resources


### Shard_iterator

ShardIterator resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `shard_iterator` | String | <p> The unique identifier for the shard iterator. This value is used in the <code>GetRecords</code> operation to retrieve data records from the specified shard. Each shard iterator expires 15 minutes after it is returned to the requester. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access shard_iterator outputs
shard_iterator_id = shard_iterator.id
shard_iterator_shard_iterator = shard_iterator.shard_iterator
```

---


### Records

Records resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_shard_iterator` | String | <p> The next position in the shard from which to start sequentially reading data records. If null, the shard has been closed and the requested iterator doesn't return any more data. </p> |
| `change_records` | Vec<String> | <p> An array of change data records retrieved from the specified shard. Each record represents a single data modification (insert, update, or delete) to a row in the Amazon Keyspaces table. Records include the primary key columns and information about what data was modified. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access records outputs
records_id = records.id
records_next_shard_iterator = records.next_shard_iterator
records_change_records = records.change_records
```

---


### Stream

Stream resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `keyspace_name` | String | <p> The name of the keyspace containing the table associated with this stream. The keyspace name is part of the table's hierarchical identifier in Amazon Keyspaces. </p> |
| `stream_status` | String | <p> The current status of the stream. Values can be <code>ENABLING</code>, <code>ENABLED</code>, <code>DISABLING</code>, or <code>DISABLED</code>. Operations on the stream depend on its current status. </p> |
| `creation_request_date_time` | String | <p> The date and time when the request to create this stream was issued. The value is represented in ISO 8601 format. </p> |
| `stream_arn` | String | <p> The Amazon Resource Name (ARN) that uniquely identifies the stream within Amazon Keyspaces. This ARN can be used in other API operations to reference this specific stream. </p> |
| `stream_label` | String | <p> A timestamp that serves as a unique identifier for this stream, used for debugging and monitoring purposes. The stream label represents the point in time when the stream was created. </p> |
| `shards` | Vec<String> | <p> An array of shard objects associated with this stream. Each shard contains a subset of the stream's data records and has its own unique identifier. The collection of shards represents the complete stream data. </p> |
| `table_name` | String | <p> The name of the table associated with this stream. The stream captures changes to rows in this Amazon Keyspaces table. </p> |
| `next_token` | String | <p> A pagination token that can be used in a subsequent <code>GetStream</code> request. This token is returned if the response contains more shards than can be returned in a single response. </p> |
| `stream_view_type` | String | <p> The format of the data records in this stream. Currently, this can be one of the following options:</p> <ul> <li> <p> <code>NEW_AND_OLD_IMAGES</code> - both versions of the row, before and after the change. This is the default.</p> </li> <li> <p> <code>NEW_IMAGE</code> - the version of the row after the change.</p> </li> <li> <p> <code>OLD_IMAGE</code> - the version of the row before the change.</p> </li> <li> <p> <code>KEYS_ONLY</code> - the partition and clustering keys of the row that was changed.</p> </li> </ul> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access stream outputs
stream_id = stream.id
stream_keyspace_name = stream.keyspace_name
stream_stream_status = stream.stream_status
stream_creation_request_date_time = stream.creation_request_date_time
stream_stream_arn = stream.stream_arn
stream_stream_label = stream.stream_label
stream_shards = stream.shards
stream_table_name = stream.table_name
stream_next_token = stream.next_token
stream_stream_view_type = stream.stream_view_type
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple shard_iterator resources
shard_iterator_0 = provider.keyspacesstreams.Shard_iterator {
}
shard_iterator_1 = provider.keyspacesstreams.Shard_iterator {
}
shard_iterator_2 = provider.keyspacesstreams.Shard_iterator {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    shard_iterator = provider.keyspacesstreams.Shard_iterator {
    }
```

---

## Related Documentation

- [AWS Keyspacesstreams Documentation](https://docs.aws.amazon.com/keyspacesstreams/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
