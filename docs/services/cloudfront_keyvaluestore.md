# Cloudfront_keyvaluestore Service



**Resources**: 3

---

## Overview

The cloudfront_keyvaluestore service provides access to 3 resource types:

- [Key_value_store](#key_value_store) [R]
- [Keys](#keys) [U]
- [Key](#key) [CRD]

---

## Resources


### Key_value_store

KeyValueStore resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `created` | String | <p>Date and time when the Key Value Store was created.</p> |
| `last_modified` | String | <p>Date and time when the key value pairs in the Key Value Store was last modified.</p> |
| `failure_reason` | String | <p>The reason for Key Value Store creation failure.</p> |
| `item_count` | i64 | <p>Number of key value pairs in the Key Value Store.</p> |
| `e_tag` | String | <p>The version identifier for the current version of the Key Value Store.</p> |
| `kvs_arn` | String | <p>The Amazon Resource Name (ARN) of the Key Value Store.</p> |
| `status` | String | <p>The current status of the Key Value Store.</p> |
| `total_size_in_bytes` | i64 | <p>Total size of the Key Value Store in bytes.</p> |


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
key_value_store_created = key_value_store.created
key_value_store_last_modified = key_value_store.last_modified
key_value_store_failure_reason = key_value_store.failure_reason
key_value_store_item_count = key_value_store.item_count
key_value_store_e_tag = key_value_store.e_tag
key_value_store_kvs_arn = key_value_store.kvs_arn
key_value_store_status = key_value_store.status
key_value_store_total_size_in_bytes = key_value_store.total_size_in_bytes
```

---


### Keys

Keys resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `puts` | Vec<String> |  | <p>List of key value pairs to put.</p> |
| `kvs_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the Key Value Store.</p> |
| `if_match` | String | ✅ | <p>The current version (ETag) of the Key Value Store that you are updating keys of, which you can get using DescribeKeyValueStore.</p> |
| `deletes` | Vec<String> |  | <p>List of keys to delete.</p> |



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


### Key

Key resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `value` | String | ✅ | <p>The value to put.</p> |
| `if_match` | String | ✅ | <p>The current version (ETag) of the Key Value Store that you are putting keys into, which you can get using DescribeKeyValueStore.</p> |
| `key` | String | ✅ | <p>The key to put.</p> |
| `kvs_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the Key Value Store.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `total_size_in_bytes` | i64 | <p>Total size of the Key Value Store in bytes.</p> |
| `value` | String | <p>The value of the key value pair.</p> |
| `key` | String | <p>The key of the key value pair.</p> |
| `item_count` | i64 | <p>Number of key value pairs in the Key Value Store.</p> |


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
    value = "value"  # <p>The value to put.</p>
    if_match = "value"  # <p>The current version (ETag) of the Key Value Store that you are putting keys into, which you can get using DescribeKeyValueStore.</p>
    key = "value"  # <p>The key to put.</p>
    kvs_arn = "value"  # <p>The Amazon Resource Name (ARN) of the Key Value Store.</p>
}

# Access key outputs
key_id = key.id
key_total_size_in_bytes = key.total_size_in_bytes
key_value = key.value
key_key = key.key
key_item_count = key.item_count
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple key_value_store resources
key_value_store_0 = provider.cloudfront_keyvaluestore.Key_value_store {
}
key_value_store_1 = provider.cloudfront_keyvaluestore.Key_value_store {
}
key_value_store_2 = provider.cloudfront_keyvaluestore.Key_value_store {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    key_value_store = provider.cloudfront_keyvaluestore.Key_value_store {
    }
```

---

## Related Documentation

- [AWS Cloudfront_keyvaluestore Documentation](https://docs.aws.amazon.com/cloudfront_keyvaluestore/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
