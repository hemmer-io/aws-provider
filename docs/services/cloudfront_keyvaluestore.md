# Cloudfront_keyvaluestore Service



**Resources**: 3

---

## Overview

The cloudfront_keyvaluestore service provides access to 3 resource types:

- [Key](#key) [CRD]
- [Key_value_store](#key_value_store) [R]
- [Keys](#keys) [U]

---

## Resources


### Key

Key resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `key` | String | ✅ | <p>The key to put.</p> |
| `value` | String | ✅ | <p>The value to put.</p> |
| `kvs_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the Key Value Store.</p> |
| `if_match` | String | ✅ | <p>The current version (ETag) of the Key Value Store that you are putting keys into, which you can get using DescribeKeyValueStore.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `value` | String | <p>The value of the key value pair.</p> |
| `key` | String | <p>The key of the key value pair.</p> |
| `item_count` | i64 | <p>Number of key value pairs in the Key Value Store.</p> |
| `total_size_in_bytes` | i64 | <p>Total size of the Key Value Store in bytes.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create key
key = provider.cloudfront_keyvaluestore.Key {
    key = "value"  # <p>The key to put.</p>
    value = "value"  # <p>The value to put.</p>
    kvs_arn = "value"  # <p>The Amazon Resource Name (ARN) of the Key Value Store.</p>
    if_match = "value"  # <p>The current version (ETag) of the Key Value Store that you are putting keys into, which you can get using DescribeKeyValueStore.</p>
}

# Access key outputs
key_id = key.id
key_value = key.value
key_key = key.key
key_item_count = key.item_count
key_total_size_in_bytes = key.total_size_in_bytes
```

---


### Key_value_store

KeyValueStore resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kvs_arn` | String | <p>The Amazon Resource Name (ARN) of the Key Value Store.</p> |
| `created` | String | <p>Date and time when the Key Value Store was created.</p> |
| `e_tag` | String | <p>The version identifier for the current version of the Key Value Store.</p> |
| `last_modified` | String | <p>Date and time when the key value pairs in the Key Value Store was last modified.</p> |
| `status` | String | <p>The current status of the Key Value Store.</p> |
| `failure_reason` | String | <p>The reason for Key Value Store creation failure.</p> |
| `total_size_in_bytes` | i64 | <p>Total size of the Key Value Store in bytes.</p> |
| `item_count` | i64 | <p>Number of key value pairs in the Key Value Store.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access key_value_store outputs
key_value_store_id = key_value_store.id
key_value_store_kvs_arn = key_value_store.kvs_arn
key_value_store_created = key_value_store.created
key_value_store_e_tag = key_value_store.e_tag
key_value_store_last_modified = key_value_store.last_modified
key_value_store_status = key_value_store.status
key_value_store_failure_reason = key_value_store.failure_reason
key_value_store_total_size_in_bytes = key_value_store.total_size_in_bytes
key_value_store_item_count = key_value_store.item_count
```

---


### Keys

Keys resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `deletes` | Vec<String> |  | <p>List of keys to delete.</p> |
| `kvs_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the Key Value Store.</p> |
| `if_match` | String | ✅ | <p>The current version (ETag) of the Key Value Store that you are updating keys of, which you can get using DescribeKeyValueStore.</p> |
| `puts` | Vec<String> |  | <p>List of key value pairs to put.</p> |



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



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple key resources
key_0 = provider.cloudfront_keyvaluestore.Key {
    key = "value-0"
    value = "value-0"
    kvs_arn = "value-0"
    if_match = "value-0"
}
key_1 = provider.cloudfront_keyvaluestore.Key {
    key = "value-1"
    value = "value-1"
    kvs_arn = "value-1"
    if_match = "value-1"
}
key_2 = provider.cloudfront_keyvaluestore.Key {
    key = "value-2"
    value = "value-2"
    kvs_arn = "value-2"
    if_match = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    key = provider.cloudfront_keyvaluestore.Key {
        key = "production-value"
        value = "production-value"
        kvs_arn = "production-value"
        if_match = "production-value"
    }
```

---

## Related Documentation

- [AWS Cloudfront_keyvaluestore Documentation](https://docs.aws.amazon.com/cloudfront_keyvaluestore/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
