# Firehose Service



**Resources**: 4

---

## Overview

The firehose service provides access to 4 resource types:

- [Destination](#destination) [U]
- [Record](#record) [C]
- [Record_batch](#record_batch) [C]
- [Delivery_stream](#delivery_stream) [CRD]

---

## Resources


### Destination

Destination resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `snowflake_destination_update` | String |  | <p>Update to the Snowflake destination configuration settings.</p> |
| `destination_id` | String | ✅ | <p>The ID of the destination.</p> |
| `s3_destination_update` | String |  | <p>[Deprecated] Describes an update for a destination in Amazon S3.</p> |
| `current_delivery_stream_version_id` | String | ✅ | <p>Obtain this value from the <code>VersionId</code> result of <a>DeliveryStreamDescription</a>. This value is required, and helps the service
         perform conditional operations. For example, if there is an interleaving update and this
         value is null, then the update destination fails. After the update is successful, the
            <code>VersionId</code> value is updated. The service then performs a merge of the old
         configuration with the new configuration.</p> |
| `http_endpoint_destination_update` | String |  | <p>Describes an update to the specified HTTP endpoint destination.</p> |
| `amazon_open_search_serverless_destination_update` | String |  | <p>Describes an update for a destination in the Serverless offering for Amazon OpenSearch
         Service.</p> |
| `iceberg_destination_update` | String |  | <p>
         Describes an update for a destination in Apache Iceberg Tables.
      </p> |
| `extended_s3_destination_update` | String |  | <p>Describes an update for a destination in Amazon S3.</p> |
| `delivery_stream_name` | String | ✅ | <p>The name of the Firehose stream.</p> |
| `redshift_destination_update` | String |  | <p>Describes an update for a destination in Amazon Redshift.</p> |
| `amazonopensearchservice_destination_update` | String |  | <p>Describes an update for a destination in Amazon OpenSearch Service.</p> |
| `splunk_destination_update` | String |  | <p>Describes an update for a destination in Splunk.</p> |
| `elasticsearch_destination_update` | String |  | <p>Describes an update for a destination in Amazon OpenSearch Service.</p> |



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


### Record

Record resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `delivery_stream_name` | String | ✅ | <p>The name of the Firehose stream.</p> |
| `record` | String | ✅ | <p>The record.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create record
record = provider.firehose.Record {
    delivery_stream_name = "value"  # <p>The name of the Firehose stream.</p>
    record = "value"  # <p>The record.</p>
}

```

---


### Record_batch

RecordBatch resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `delivery_stream_name` | String | ✅ | <p>The name of the Firehose stream.</p> |
| `records` | Vec<String> | ✅ | <p>One or more records.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create record_batch
record_batch = provider.firehose.Record_batch {
    delivery_stream_name = "value"  # <p>The name of the Firehose stream.</p>
    records = "value"  # <p>One or more records.</p>
}

```

---


### Delivery_stream

DeliveryStream resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `redshift_destination_configuration` | String |  | <p>The destination in Amazon Redshift. You can specify only one destination.</p> |
| `tags` | Vec<String> |  | <p>A set of tags to assign to the Firehose stream. A tag is a key-value pair that you can
         define and assign to Amazon Web Services resources. Tags are metadata. For example, you can
         add friendly names and descriptions or other types of information that can help you
         distinguish the Firehose stream. For more information about tags, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html">Using
            Cost Allocation Tags</a> in the Amazon Web Services Billing and Cost Management User
         Guide.</p>
         <p>You can specify up to 50 tags when creating a Firehose stream.</p>
         <p>If you specify tags in the <code>CreateDeliveryStream</code> action, Amazon Data
         Firehose performs an additional authorization on the
            <code>firehose:TagDeliveryStream</code> action to verify if users have permissions to
         create tags. If you do not provide this permission, requests to create new Firehose streams
         with IAM resource tags will fail with an <code>AccessDeniedException</code> such as
         following.</p>
         <p>
            <b>AccessDeniedException</b>
         </p>
         <p>User: arn:aws:sts::x:assumed-role/x/x is not authorized to perform: firehose:TagDeliveryStream on resource: arn:aws:firehose:us-east-1:x:deliverystream/x with an explicit deny in an identity-based policy.</p>
         <p>For an example IAM policy, see <a href="https://docs.aws.amazon.com/firehose/latest/APIReference/API_CreateDeliveryStream.html#API_CreateDeliveryStream_Examples">Tag example.</a>
         </p> |
| `amazon_open_search_serverless_destination_configuration` | String |  | <p>The destination in the Serverless offering for Amazon OpenSearch Service. You can
         specify only one destination.</p> |
| `splunk_destination_configuration` | String |  | <p>The destination in Splunk. You can specify only one destination.</p> |
| `direct_put_source_configuration` | String |  | <p>The structure that configures parameters such as <code>ThroughputHintInMBs</code> for a
         stream configured with Direct PUT as a source. </p> |
| `delivery_stream_name` | String | ✅ | <p>The name of the Firehose stream. This name must be unique per Amazon Web Services
         account in the same Amazon Web Services Region. If the Firehose streams are in different
         accounts or different Regions, you can have multiple Firehose streams with the same
         name.</p> |
| `extended_s3_destination_configuration` | String |  | <p>The destination in Amazon S3. You can specify only one destination.</p> |
| `s3_destination_configuration` | String |  | <p>[Deprecated]
         The destination in Amazon S3. You can specify only one destination.</p> |
| `msk_source_configuration` | String |  |  |
| `snowflake_destination_configuration` | String |  | <p>Configure Snowflake destination</p> |
| `delivery_stream_encryption_configuration_input` | String |  | <p>Used to specify the type and Amazon Resource Name (ARN) of the KMS key needed for
         Server-Side Encryption (SSE).</p> |
| `elasticsearch_destination_configuration` | String |  | <p>The destination in Amazon OpenSearch Service. You can specify only one destination.</p> |
| `amazonopensearchservice_destination_configuration` | String |  | <p>The destination in Amazon OpenSearch Service. You can specify only one
         destination.</p> |
| `http_endpoint_destination_configuration` | String |  | <p>Enables configuring Kinesis Firehose to deliver data to any HTTP endpoint destination.
         You can specify only one destination.</p> |
| `database_source_configuration` | String |  | <p>
            The top level object for configuring streams with database as a source. 
         </p>
         <p>Amazon Data Firehose is in preview release and is subject to change.</p> |
| `kinesis_stream_source_configuration` | String |  | <p>When a Kinesis data stream is used as the source for the Firehose stream, a <a>KinesisStreamSourceConfiguration</a> containing the Kinesis data stream Amazon
         Resource Name (ARN) and the role ARN for the source stream.</p> |
| `delivery_stream_type` | String |  | <p>The Firehose stream type. This parameter can be one of the following
         values:</p>
         <ul>
            <li>
               <p>
                  <code>DirectPut</code>: Provider applications access the Firehose stream
               directly.</p>
            </li>
            <li>
               <p>
                  <code>KinesisStreamAsSource</code>: The Firehose stream uses a Kinesis data
               stream as a source.</p>
            </li>
         </ul> |
| `iceberg_destination_configuration` | String |  | <p>
         Configure Apache Iceberg Tables destination.
      </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `delivery_stream_description` | String | <p>Information about the Firehose stream.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create delivery_stream
delivery_stream = provider.firehose.Delivery_stream {
    delivery_stream_name = "value"  # <p>The name of the Firehose stream. This name must be unique per Amazon Web Services
         account in the same Amazon Web Services Region. If the Firehose streams are in different
         accounts or different Regions, you can have multiple Firehose streams with the same
         name.</p>
}

# Access delivery_stream outputs
delivery_stream_id = delivery_stream.id
delivery_stream_delivery_stream_description = delivery_stream.delivery_stream_description
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple destination resources
destination_0 = provider.firehose.Destination {
    destination_id = "value-0"
    current_delivery_stream_version_id = "value-0"
    delivery_stream_name = "value-0"
}
destination_1 = provider.firehose.Destination {
    destination_id = "value-1"
    current_delivery_stream_version_id = "value-1"
    delivery_stream_name = "value-1"
}
destination_2 = provider.firehose.Destination {
    destination_id = "value-2"
    current_delivery_stream_version_id = "value-2"
    delivery_stream_name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    destination = provider.firehose.Destination {
        destination_id = "production-value"
        current_delivery_stream_version_id = "production-value"
        delivery_stream_name = "production-value"
    }
```

---

## Related Documentation

- [AWS Firehose Documentation](https://docs.aws.amazon.com/firehose/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
