# Iotanalytics Service



**Resources**: 6

---

## Overview

The iotanalytics service provides access to 6 resource types:

- [Datastore](#datastore) [CRUD]
- [Dataset_content](#dataset_content) [CRD]
- [Logging_options](#logging_options) [CR]
- [Pipeline](#pipeline) [CRUD]
- [Dataset](#dataset) [CRUD]
- [Channel](#channel) [CRUD]

---

## Resources


### Datastore

Datastore resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `retention_period` | String |  | <p>How long, in days, message data is kept for the data store. When
        <code>customerManagedS3</code> storage is selected, this parameter is ignored.</p> |
| `datastore_partitions` | String |  | <p> Contains information about the partition dimensions in a data store. </p> |
| `datastore_name` | String | ✅ | <p>The name of the data store.</p> |
| `datastore_storage` | String |  | <p>Where data in a data store is stored.. You can choose <code>serviceManagedS3</code> storage, <code>customerManagedS3</code> storage, or <code>iotSiteWiseMultiLayerStorage</code> storage. The default is <code>serviceManagedS3</code>. You can't change the choice of Amazon S3 storage after your data store is created. </p> |
| `tags` | Vec<String> |  | <p>Metadata which can be used to manage the data store.</p> |
| `file_format_configuration` | String |  | <p>Contains the configuration information of file formats.  IoT Analytics data stores support JSON and <a href="https://parquet.apache.org/">Parquet</a>.</p>
         <p>The default file format is JSON. You can specify only one format.</p>
         <p>You can't change the file format after you create the data store.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `statistics` | String | <p>Additional statistical information about the data store. Included if the
        <code>includeStatistics</code> parameter is set to <code>true</code> in the request.</p> |
| `datastore` | String | <p>Information about the data store.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create datastore
datastore = provider.iotanalytics.Datastore {
    datastore_name = "value"  # <p>The name of the data store.</p>
}

# Access datastore outputs
datastore_id = datastore.id
datastore_statistics = datastore.statistics
datastore_datastore = datastore.datastore
```

---


### Dataset_content

DatasetContent resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `dataset_name` | String | ✅ | <p>The name of the dataset.</p> |
| `version_id` | String |  | <p>The version ID of the dataset content. To specify <code>versionId</code> for a dataset
      content, the dataset must use a <a href="https://docs.aws.amazon.com/iotanalytics/latest/APIReference/API_DeltaTime.html">DeltaTimer</a> filter.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `timestamp` | String | <p>The time when the request was made.</p> |
| `status` | String | <p>The status of the dataset content.</p> |
| `entries` | Vec<String> | <p>A list of <code>DatasetEntry</code> objects.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create dataset_content
dataset_content = provider.iotanalytics.Dataset_content {
    dataset_name = "value"  # <p>The name of the dataset.</p>
}

# Access dataset_content outputs
dataset_content_id = dataset_content.id
dataset_content_timestamp = dataset_content.timestamp
dataset_content_status = dataset_content.status
dataset_content_entries = dataset_content.entries
```

---


### Logging_options

LoggingOptions resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `logging_options` | String | ✅ | <p>The new values of the IoT Analytics logging options.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `logging_options` | String | <p>The current settings of the IoT Analytics logging options.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create logging_options
logging_options = provider.iotanalytics.Logging_options {
    logging_options = "value"  # <p>The new values of the IoT Analytics logging options.</p>
}

# Access logging_options outputs
logging_options_id = logging_options.id
logging_options_logging_options = logging_options.logging_options
```

---


### Pipeline

Pipeline resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `pipeline_activities` | Vec<String> | ✅ | <p>A list of <code>PipelineActivity</code> objects. Activities perform transformations on
      your messages, such as removing, renaming or adding message attributes; filtering messages
      based on attribute values; invoking your Lambda unctions on messages for advanced processing;
      or performing mathematical transformations to normalize device data.</p>
         <p>The list can be 2-25 <code>PipelineActivity</code> objects and must contain both a
        <code>channel</code> and a <code>datastore</code> activity. Each entry in the list must
      contain only one activity. For example:</p>
         <p>
            <code>pipelineActivities = [ { "channel": { ... } }, { "lambda": { ... } }, ...
      ]</code>
         </p> |
| `pipeline_name` | String | ✅ | <p>The name of the pipeline.</p> |
| `tags` | Vec<String> |  | <p>Metadata which can be used to manage the pipeline.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `pipeline` | String | <p>A <code>Pipeline</code> object that contains information about the pipeline.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create pipeline
pipeline = provider.iotanalytics.Pipeline {
    pipeline_activities = "value"  # <p>A list of <code>PipelineActivity</code> objects. Activities perform transformations on
      your messages, such as removing, renaming or adding message attributes; filtering messages
      based on attribute values; invoking your Lambda unctions on messages for advanced processing;
      or performing mathematical transformations to normalize device data.</p>
         <p>The list can be 2-25 <code>PipelineActivity</code> objects and must contain both a
        <code>channel</code> and a <code>datastore</code> activity. Each entry in the list must
      contain only one activity. For example:</p>
         <p>
            <code>pipelineActivities = [ { "channel": { ... } }, { "lambda": { ... } }, ...
      ]</code>
         </p>
    pipeline_name = "value"  # <p>The name of the pipeline.</p>
}

# Access pipeline outputs
pipeline_id = pipeline.id
pipeline_pipeline = pipeline.pipeline
```

---


### Dataset

Dataset resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>Metadata which can be used to manage the dataset.</p> |
| `triggers` | Vec<String> |  | <p>A list of triggers. A trigger causes dataset contents to be populated at a specified time
      interval or when another dataset's contents are created. The list of triggers can be empty or
      contain up to five <code>DataSetTrigger</code> objects.</p> |
| `actions` | Vec<String> | ✅ | <p>A list of actions that create the dataset contents.</p> |
| `versioning_configuration` | String |  | <p>Optional. How many versions of dataset contents are kept. If not specified or set to null,
      only the latest version plus the latest succeeded version (if they are different) are kept for
      the time period specified by the <code>retentionPeriod</code> parameter. For more information,
      see <a href="https://docs.aws.amazon.com/iotanalytics/latest/userguide/getting-started.html#aws-iot-analytics-dataset-versions">Keeping Multiple Versions of IoT Analytics datasets</a> in the
        <i>IoT Analytics User Guide</i>.</p> |
| `dataset_name` | String | ✅ | <p>The name of the dataset.</p> |
| `content_delivery_rules` | Vec<String> |  | <p>When dataset contents are created, they are delivered to destinations specified
      here.</p> |
| `retention_period` | String |  | <p>Optional. How long, in days, versions of dataset contents are kept for the dataset. If not
      specified or set to <code>null</code>, versions of dataset contents are retained for at most
      90 days. The number of versions of dataset contents retained is determined by the
        <code>versioningConfiguration</code> parameter. For more information, see <a href="https://docs.aws.amazon.com/iotanalytics/latest/userguide/getting-started.html#aws-iot-analytics-dataset-versions">
        Keeping Multiple Versions of IoT Analytics datasets</a> in the
        <i>IoT Analytics User Guide</i>.</p> |
| `late_data_rules` | Vec<String> |  | <p>A list of data rules that send notifications to CloudWatch, when data arrives late. To specify <code>lateDataRules</code>, the dataset must use a <a href="https://docs.aws.amazon.com/iotanalytics/latest/APIReference/API_DeltaTime.html">DeltaTimer</a> filter.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `dataset` | String | <p>An object that contains information about the dataset.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create dataset
dataset = provider.iotanalytics.Dataset {
    actions = "value"  # <p>A list of actions that create the dataset contents.</p>
    dataset_name = "value"  # <p>The name of the dataset.</p>
}

# Access dataset outputs
dataset_id = dataset.id
dataset_dataset = dataset.dataset
```

---


### Channel

Channel resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `channel_storage` | String |  | <p>Where channel data is stored. You can choose one of <code>serviceManagedS3</code> or
        <code>customerManagedS3</code> storage. If not specified, the default is
        <code>serviceManagedS3</code>. You can't change this storage option after the channel is
      created.</p> |
| `tags` | Vec<String> |  | <p>Metadata which can be used to manage the channel.</p> |
| `channel_name` | String | ✅ | <p>The name of the channel.</p> |
| `retention_period` | String |  | <p>How long, in days, message data is kept for the channel. When
        <code>customerManagedS3</code> storage is selected, this parameter is ignored.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `statistics` | String | <p>Statistics about the channel. Included if the <code>includeStatistics</code> parameter is
      set to <code>true</code> in the request.</p> |
| `channel` | String | <p>An object that contains information about the channel.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create channel
channel = provider.iotanalytics.Channel {
    channel_name = "value"  # <p>The name of the channel.</p>
}

# Access channel outputs
channel_id = channel.id
channel_statistics = channel.statistics
channel_channel = channel.channel
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple datastore resources
datastore_0 = provider.iotanalytics.Datastore {
    datastore_name = "value-0"
}
datastore_1 = provider.iotanalytics.Datastore {
    datastore_name = "value-1"
}
datastore_2 = provider.iotanalytics.Datastore {
    datastore_name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    datastore = provider.iotanalytics.Datastore {
        datastore_name = "production-value"
    }
```

---

## Related Documentation

- [AWS Iotanalytics Documentation](https://docs.aws.amazon.com/iotanalytics/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
