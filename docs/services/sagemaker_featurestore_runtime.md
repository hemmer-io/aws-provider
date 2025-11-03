# Sagemaker_featurestore_runtime Service



**Resources**: 1

---

## Overview

The sagemaker_featurestore_runtime service provides access to 1 resource type:

- [Record](#record) [CRD]

---

## Resources


### Record

Record resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `feature_group_name` | String | ✅ | <p>The name or Amazon Resource Name (ARN) of the feature group that you want to insert the
         record into.</p> |
| `target_stores` | Vec<String> |  | <p>A list of stores to which you're adding the record. By default, Feature Store adds the
         record to all of the stores that you're using for the <code>FeatureGroup</code>.</p> |
| `ttl_duration` | String |  | <p>Time to live duration, where the record is hard deleted after the expiration time is
         reached; <code>ExpiresAt</code> = <code>EventTime</code> + <code>TtlDuration</code>. For
         information on HardDelete, see the <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_feature_store_DeleteRecord.html">DeleteRecord</a> API in the Amazon SageMaker API Reference guide.</p> |
| `record` | Vec<String> | ✅ | <p>List of FeatureValues to be inserted. This will be a full over-write. If you only want
         to update few of the feature values, do the following:</p>
         <ul>
            <li>
               <p>Use <code>GetRecord</code> to retrieve the latest record.</p>
            </li>
            <li>
               <p>Update the record returned from <code>GetRecord</code>. </p>
            </li>
            <li>
               <p>Use <code>PutRecord</code> to update feature values.</p>
            </li>
         </ul> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `record` | Vec<String> | <p>The record you requested. A list of <code>FeatureValues</code>.</p> |
| `expires_at` | String | <p>The <code>ExpiresAt</code> ISO string of the requested record.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create record
record = provider.sagemaker_featurestore_runtime.Record {
    feature_group_name = "value"  # <p>The name or Amazon Resource Name (ARN) of the feature group that you want to insert the
         record into.</p>
    record = "value"  # <p>List of FeatureValues to be inserted. This will be a full over-write. If you only want
         to update few of the feature values, do the following:</p>
         <ul>
            <li>
               <p>Use <code>GetRecord</code> to retrieve the latest record.</p>
            </li>
            <li>
               <p>Update the record returned from <code>GetRecord</code>. </p>
            </li>
            <li>
               <p>Use <code>PutRecord</code> to update feature values.</p>
            </li>
         </ul>
}

# Access record outputs
record_id = record.id
record_record = record.record
record_expires_at = record.expires_at
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple record resources
record_0 = provider.sagemaker_featurestore_runtime.Record {
    feature_group_name = "value-0"
    record = "value-0"
}
record_1 = provider.sagemaker_featurestore_runtime.Record {
    feature_group_name = "value-1"
    record = "value-1"
}
record_2 = provider.sagemaker_featurestore_runtime.Record {
    feature_group_name = "value-2"
    record = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    record = provider.sagemaker_featurestore_runtime.Record {
        feature_group_name = "production-value"
        record = "production-value"
    }
```

---

## Related Documentation

- [AWS Sagemaker_featurestore_runtime Documentation](https://docs.aws.amazon.com/sagemaker_featurestore_runtime/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
