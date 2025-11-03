# Dynamodb_streams Service



**Resources**: 3

---

## Overview

The dynamodb_streams service provides access to 3 resource types:

- [Records](#records) [R]
- [Stream](#stream) [R]
- [Shard_iterator](#shard_iterator) [R]

---

## Resources


### Records

Records resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_shard_iterator` | String | <p>The next position in the shard from which to start sequentially reading stream records. If
      set to <code>null</code>, the shard has been closed and the requested iterator will not return
      any more data.</p> |
| `records` | Vec<String> | <p>The stream records from the shard, which were retrieved using the shard iterator.</p> |


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
records_records = records.records
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
| `stream_description` | String | <p>A complete description of the stream, including its creation date and time, the DynamoDB table associated with the stream, the shard IDs within the stream, and the beginning and ending sequence numbers of stream records within the shards.</p> |


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
stream_stream_description = stream.stream_description
```

---


### Shard_iterator

ShardIterator resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `shard_iterator` | String | <p>The position in the shard from which to start reading stream records sequentially. A shard iterator specifies this position using the sequence number of a stream record in a shard.</p> |


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



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple records resources
records_0 = provider.dynamodb_streams.Records {
}
records_1 = provider.dynamodb_streams.Records {
}
records_2 = provider.dynamodb_streams.Records {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    records = provider.dynamodb_streams.Records {
    }
```

---

## Related Documentation

- [AWS Dynamodb_streams Documentation](https://docs.aws.amazon.com/dynamodb_streams/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
