# Kinesis Service



**Resources**: 28

---

## Overview

The kinesis service provides access to 28 resource types:

- [Hls_streaming_session_url](#hls_streaming_session_url) [R]
- [Clip](#clip) [R]
- [Images](#images) [R]
- [Media_for_fragment_list](#media_for_fragment_list) [R]
- [Dash_streaming_session_url](#dash_streaming_session_url) [R]
- [Media](#media) [R]
- [Notification_configuration](#notification_configuration) [RU]
- [Signaling_channel_endpoint](#signaling_channel_endpoint) [R]
- [Signaling_channel](#signaling_channel) [CRUD]
- [Edge_configuration](#edge_configuration) [RD]
- [Data_endpoint](#data_endpoint) [R]
- [Image_generation_configuration](#image_generation_configuration) [RU]
- [Media_storage_configuration](#media_storage_configuration) [RU]
- [Mapped_resource_configuration](#mapped_resource_configuration) [R]
- [Stream](#stream) [CRUD]
- [Data_retention](#data_retention) [U]
- [Shard_iterator](#shard_iterator) [R]
- [Limits](#limits) [R]
- [Record](#record) [C]
- [Resource_policy](#resource_policy) [CRD]
- [Stream_consumer](#stream_consumer) [R]
- [Records](#records) [CR]
- [Stream_mode](#stream_mode) [U]
- [Shard_count](#shard_count) [U]
- [Stream](#stream) [CRD]
- [Max_record_size](#max_record_size) [U]
- [Stream_summary](#stream_summary) [R]
- [Ice_server_config](#ice_server_config) [R]

---

## Resources


### Hls_streaming_session_url

HLSStreamingSessionURL resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `hls_streaming_session_url` | String | <p>The URL (containing the session token) that a media player can use to retrieve the HLS
            master playlist.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access hls_streaming_session_url outputs
hls_streaming_session_url_id = hls_streaming_session_url.id
hls_streaming_session_url_hls_streaming_session_url = hls_streaming_session_url.hls_streaming_session_url
```

---


### Clip

Clip resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `content_type` | String | <p>The content type of the media in the requested clip.</p> |
| `payload` | String | <p>Traditional MP4 file that contains the media clip from the specified video stream. The
            output will contain the first 100 MB or the first 200 fragments from the specified start
            timestamp. For more information, see <a href="https://docs.aws.amazon.com/kinesisvideostreams/latest/dg/limits.html">Kinesis
                Video Streams Limits</a>. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access clip outputs
clip_id = clip.id
clip_content_type = clip.content_type
clip_payload = clip.payload
```

---


### Images

Images resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `images` | Vec<String> | <p>The list of images generated from the video stream. If there is no media available for the given timestamp, the <code>NO_MEDIA</code> error will be listed in the output.  
            If an error occurs while the image is being generated, the <code>MEDIA_ERROR</code> will be listed in the output as the cause of the missing image. </p> |
| `next_token` | String | <p>The encrypted token that was  used in the request to get more images.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access images outputs
images_id = images.id
images_images = images.images
images_next_token = images.next_token
```

---


### Media_for_fragment_list

MediaForFragmentList resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `content_type` | String | <p>The content type of the requested media.</p> |
| `payload` | String | <p>The payload that Kinesis Video Streams returns is a sequence of chunks from the
            specified stream. For information about the chunks, see <a href="http://docs.aws.amazon.com/kinesisvideostreams/latest/dg/API_dataplane_PutMedia.html">PutMedia</a>. The chunks that Kinesis Video Streams returns in the
                <code>GetMediaForFragmentList</code> call also include the following additional
            Matroska (MKV) tags: </p>
         <ul>
            <li>
               <p>AWS_KINESISVIDEO_FRAGMENT_NUMBER - Fragment number returned in the
                    chunk.</p>
            </li>
            <li>
               <p>AWS_KINESISVIDEO_SERVER_SIDE_TIMESTAMP - Server-side timestamp of the
                    fragment.</p>
            </li>
            <li>
               <p>AWS_KINESISVIDEO_PRODUCER_SIDE_TIMESTAMP - Producer-side timestamp of the
                    fragment.</p>
            </li>
         </ul>
         <p>The following tags will be included if an exception occurs:</p>
         <ul>
            <li>
               <p>AWS_KINESISVIDEO_FRAGMENT_NUMBER - The number of the fragment that threw the exception
                    </p>
            </li>
            <li>
               <p>AWS_KINESISVIDEO_EXCEPTION_ERROR_CODE - The integer code of the
                    </p>
            </li>
            <li>
               <p>AWS_KINESISVIDEO_EXCEPTION_MESSAGE - A text description of the exception
                    </p>
            </li>
         </ul> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access media_for_fragment_list outputs
media_for_fragment_list_id = media_for_fragment_list.id
media_for_fragment_list_content_type = media_for_fragment_list.content_type
media_for_fragment_list_payload = media_for_fragment_list.payload
```

---


### Dash_streaming_session_url

DASHStreamingSessionURL resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `dash_streaming_session_url` | String | <p>The URL (containing the session token) that a media player can use to retrieve the
            MPEG-DASH manifest.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access dash_streaming_session_url outputs
dash_streaming_session_url_id = dash_streaming_session_url.id
dash_streaming_session_url_dash_streaming_session_url = dash_streaming_session_url.dash_streaming_session_url
```

---


### Media

Media resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `content_type` | String | <p>The content type of the requested media.</p> |
| `payload` | String | <p> The payload Kinesis Video Streams returns is a sequence of chunks from the specified
      stream. For information about the chunks, see . The
      chunks that Kinesis Video Streams returns in the <code>GetMedia</code> call also include the
      following additional Matroska (MKV) tags: </p>
         <ul>
            <li>
               <p>AWS_KINESISVIDEO_CONTINUATION_TOKEN (UTF-8 string) - In the event your
            <code>GetMedia</code> call terminates, you can use this continuation token in your next
          request to get the next chunk where the last request terminated.</p>
            </li>
            <li>
               <p>AWS_KINESISVIDEO_MILLIS_BEHIND_NOW (UTF-8 string) - Client applications can use
          this tag value to determine how far behind the chunk returned in the response is from the
          latest chunk on the stream. </p>
            </li>
            <li>
               <p>AWS_KINESISVIDEO_FRAGMENT_NUMBER - Fragment number returned in the chunk.</p>
            </li>
            <li>
               <p>AWS_KINESISVIDEO_SERVER_TIMESTAMP - Server timestamp of the fragment.</p>
            </li>
            <li>
               <p>AWS_KINESISVIDEO_PRODUCER_TIMESTAMP - Producer timestamp of the fragment.</p>
            </li>
         </ul>
         <p>The following tags will be present if an error occurs:</p>
         <ul>
            <li>
               <p>AWS_KINESISVIDEO_ERROR_CODE - String description of an error that caused GetMedia
          to stop.</p>
            </li>
            <li>
               <p>AWS_KINESISVIDEO_ERROR_ID: Integer code of the error.</p>
            </li>
         </ul>
         <p>The error codes are as follows:</p>
         <ul>
            <li>
               <p>3002 - Error writing to the stream</p>
            </li>
            <li>
               <p>4000 - Requested fragment is not found</p>
            </li>
            <li>
               <p>4500 - Access denied for the stream's KMS key</p>
            </li>
            <li>
               <p>4501 - Stream's KMS key is disabled</p>
            </li>
            <li>
               <p>4502 - Validation error on the stream's KMS key</p>
            </li>
            <li>
               <p>4503 - KMS key specified in the stream is unavailable</p>
            </li>
            <li>
               <p>4504 - Invalid usage of the KMS key specified in the stream</p>
            </li>
            <li>
               <p>4505 - Invalid state of the KMS key specified in the stream</p>
            </li>
            <li>
               <p>4506 - Unable to find the KMS key specified in the stream</p>
            </li>
            <li>
               <p>5000 - Internal error</p>
            </li>
         </ul> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access media outputs
media_id = media.id
media_content_type = media.content_type
media_payload = media.payload
```

---


### Notification_configuration

NotificationConfiguration resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `stream_name` | String |  | <p>The name of the stream from which to update the notification configuration. You must specify either the <code>StreamName</code> or the <code>StreamARN</code>.</p> |
| `stream_arn` | String |  | <p>The Amazon Resource Name (ARN) of the Kinesis video stream from where you want to update the notification configuration. You must specify either the <code>StreamName</code> or the <code>StreamARN</code>.</p> |
| `notification_configuration` | String |  | <p>The structure containing the information required for notifications. If the structure is null, the configuration will be deleted from the stream.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `notification_configuration` | String | <p>The structure that contains the information required for notifications. If the structure is null, the configuration will be deleted from the stream.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access notification_configuration outputs
notification_configuration_id = notification_configuration.id
notification_configuration_notification_configuration = notification_configuration.notification_configuration
```

---


### Signaling_channel_endpoint

SignalingChannelEndpoint resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `resource_endpoint_list` | Vec<String> | <p>A list of endpoints for the specified signaling channel.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access signaling_channel_endpoint outputs
signaling_channel_endpoint_id = signaling_channel_endpoint.id
signaling_channel_endpoint_resource_endpoint_list = signaling_channel_endpoint.resource_endpoint_list
```

---


### Signaling_channel

SignalingChannel resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `single_master_configuration` | String |  | <p>A structure containing the configuration for the <code>SINGLE_MASTER</code> channel
            type. </p> |
| `channel_name` | String | ✅ | <p>A name for the signaling channel that you are creating. It must be unique for each Amazon Web Services account and Amazon Web Services Region.</p> |
| `tags` | Vec<String> |  | <p>A set of tags (key-value pairs) that you want to associate with this channel.</p> |
| `channel_type` | String |  | <p>A type of the signaling channel that you are creating. Currently,
                <code>SINGLE_MASTER</code> is the only supported channel type. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `channel_info` | String | <p>A structure that encapsulates the specified signaling channel's metadata and
            properties.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create signaling_channel
signaling_channel = provider.kinesis.Signaling_channel {
    channel_name = "value"  # <p>A name for the signaling channel that you are creating. It must be unique for each Amazon Web Services account and Amazon Web Services Region.</p>
}

# Access signaling_channel outputs
signaling_channel_id = signaling_channel.id
signaling_channel_channel_info = signaling_channel.channel_info
```

---


### Edge_configuration

EdgeConfiguration resource

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `edge_agent_status` | String | <p>An object that contains the latest status details for an edge agent's recorder and uploader jobs. Use this information to determine the current health of an edge agent.</p> |
| `edge_config` | String | <p>A description of the stream's edge configuration that will be used to sync 
            with the Edge Agent IoT Greengrass component. The Edge Agent component will run
            on an IoT Hub Device setup at your premise.</p> |
| `creation_time` | String | <p>The timestamp at which a stream’s edge configuration was first created.</p> |
| `failed_status_details` | String | <p>A description of the generated failure status.</p> |
| `stream_arn` | String | <p>The Amazon Resource Name (ARN) of the stream.</p> |
| `sync_status` | String | <p>The latest status of the edge configuration update.</p> |
| `last_updated_time` | String | <p>The timestamp at which a stream’s edge configuration was last updated.</p> |
| `stream_name` | String | <p>The name of the stream from which the edge configuration was updated.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access edge_configuration outputs
edge_configuration_id = edge_configuration.id
edge_configuration_edge_agent_status = edge_configuration.edge_agent_status
edge_configuration_edge_config = edge_configuration.edge_config
edge_configuration_creation_time = edge_configuration.creation_time
edge_configuration_failed_status_details = edge_configuration.failed_status_details
edge_configuration_stream_arn = edge_configuration.stream_arn
edge_configuration_sync_status = edge_configuration.sync_status
edge_configuration_last_updated_time = edge_configuration.last_updated_time
edge_configuration_stream_name = edge_configuration.stream_name
```

---


### Data_endpoint

DataEndpoint resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `data_endpoint` | String | <p>The endpoint value. To read data from the stream or to write data to it, specify
            this endpoint in your application.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access data_endpoint outputs
data_endpoint_id = data_endpoint.id
data_endpoint_data_endpoint = data_endpoint.data_endpoint
```

---


### Image_generation_configuration

ImageGenerationConfiguration resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `stream_arn` | String |  | <p>The Amazon Resource Name (ARN) of the Kinesis video stream from where you want to update the image generation configuration. You must specify either the <code>StreamName</code> 
            or the <code>StreamARN</code>.</p> |
| `image_generation_configuration` | String |  | <p>The structure that contains the information required for the KVS images delivery. If the structure is null, the configuration will be deleted from the stream.</p> |
| `stream_name` | String |  | <p>The name of the stream from which to update the image generation configuration. You must specify either the <code>StreamName</code> or the <code>StreamARN</code>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `image_generation_configuration` | String | <p>The structure that contains the information required for the Kinesis video stream (KVS) images delivery. If this structure is null, the configuration will be deleted from the stream.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access image_generation_configuration outputs
image_generation_configuration_id = image_generation_configuration.id
image_generation_configuration_image_generation_configuration = image_generation_configuration.image_generation_configuration
```

---


### Media_storage_configuration

MediaStorageConfiguration resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `media_storage_configuration` | String | ✅ | <p>A structure that encapsulates, or contains, the media storage configuration properties.</p> |
| `channel_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the channel.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `media_storage_configuration` | String | <p>A structure that encapsulates, or contains, the media storage configuration properties.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access media_storage_configuration outputs
media_storage_configuration_id = media_storage_configuration.id
media_storage_configuration_media_storage_configuration = media_storage_configuration.media_storage_configuration
```

---


### Mapped_resource_configuration

MappedResourceConfiguration resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token that was used in the <code>NextToken</code>request to fetch the next set of results. </p> |
| `mapped_resource_configuration_list` | Vec<String> | <p>A structure that encapsulates, or contains, the media storage configuration properties.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access mapped_resource_configuration outputs
mapped_resource_configuration_id = mapped_resource_configuration.id
mapped_resource_configuration_next_token = mapped_resource_configuration.next_token
mapped_resource_configuration_mapped_resource_configuration_list = mapped_resource_configuration.mapped_resource_configuration_list
```

---


### Stream

Stream resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `media_type` | String |  | <p>The media type of the stream. Consumers of the stream can use this information when
            processing the stream. For more information about media types, see <a href="http://www.iana.org/assignments/media-types/media-types.xhtml">Media
                Types</a>. If you choose to specify the <code>MediaType</code>, see <a href="https://tools.ietf.org/html/rfc6838#section-4.2">Naming Requirements</a>
            for guidelines.</p>
         <p>Example valid values include "video/h264" and "video/h264,audio/aac".</p>
         <p>This parameter is optional; the default value is <code>null</code> (or empty in
            JSON).</p> |
| `data_retention_in_hours` | i64 |  | <p>The number of hours that you want to retain the data in the stream. Kinesis Video Streams retains the data in a data store that is associated with the stream.</p>
         <p>The default value is 0, indicating that the stream does not persist data.</p>
         <p>When the <code>DataRetentionInHours</code> value is 0, consumers can still consume
            the fragments that remain in the service host buffer, which has a retention time limit
            of 5 minutes and a retention memory limit of 200 MB. Fragments are removed from the
            buffer when either limit is reached.</p> |
| `tags` | HashMap<String, String> |  | <p>A list of tags to associate with the specified stream. Each tag is a key-value pair
            (the value is optional).</p> |
| `kms_key_id` | String |  | <p>The ID of the Key Management Service (KMS) key that you want Kinesis Video
            Streams to use to encrypt stream data.</p>
         <p>If no key ID is specified, the default, Kinesis Video-managed key
            (<code>Amazon Web Services/kinesisvideo</code>) is used.</p>
         <p> For more information, see <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_DescribeKey.html#API_DescribeKey_RequestParameters">DescribeKey</a>. </p> |
| `stream_name` | String | ✅ | <p>A name for the stream that you are creating.</p>
         <p>The stream name is an identifier for the stream, and must be unique for each
            account and region.</p> |
| `device_name` | String |  | <p>The name of the device that is writing to the stream. </p>
         <note>
            <p>In the current implementation, Kinesis Video Streams does not use this
                name.</p>
         </note> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `stream_info` | String | <p>An object that describes the stream.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create stream
stream = provider.kinesis.Stream {
    stream_name = "value"  # <p>A name for the stream that you are creating.</p>
         <p>The stream name is an identifier for the stream, and must be unique for each
            account and region.</p>
}

# Access stream outputs
stream_id = stream.id
stream_stream_info = stream.stream_info
```

---


### Data_retention

DataRetention resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `stream_name` | String |  | <p>The name of the stream whose retention period you want to change.</p> |
| `operation` | String | ✅ | <p>Indicates whether you want to increase or decrease the retention period.</p> |
| `stream_arn` | String |  | <p>The Amazon Resource Name (ARN) of the stream whose retention period you want to
            change.</p> |
| `current_version` | String | ✅ | <p>The version of the stream whose retention period you want to change. To get the
            version, call either the <code>DescribeStream</code> or the <code>ListStreams</code>
            API.</p> |
| `data_retention_change_in_hours` | i64 | ✅ | <p>The number of hours to adjust the current retention by. The value you specify is added to or subtracted from the current value, depending on the <code>operation</code>.</p>
         <p>The minimum value for data retention is 0 and the maximum value is 87600 (ten years).</p> |



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


### Shard_iterator

ShardIterator resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `shard_iterator` | String | <p>The position in the shard from which to start reading data records sequentially. A
            shard iterator specifies this position using the sequence number of a data record in a
            shard.</p> |


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


### Limits

Limits resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `on_demand_stream_count_limit` | i64 | <p> The maximum number of data streams with the on-demand capacity mode. </p> |
| `open_shard_count` | i64 | <p>The number of open shards.</p> |
| `on_demand_stream_count` | i64 | <p> Indicates the number of data streams with the on-demand capacity mode.</p> |
| `shard_limit` | i64 | <p>The maximum number of shards.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access limits outputs
limits_id = limits.id
limits_on_demand_stream_count_limit = limits.on_demand_stream_count_limit
limits_open_shard_count = limits.open_shard_count
limits_on_demand_stream_count = limits.on_demand_stream_count
limits_shard_limit = limits.shard_limit
```

---


### Record

Record resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `explicit_hash_key` | String |  | <p>The hash value used to explicitly determine the shard the data record is assigned to
            by overriding the partition key hash.</p> |
| `stream_name` | String |  | <p>The name of the stream to put the data record into.</p> |
| `partition_key` | String | ✅ | <p>Determines which shard in the stream the data record is assigned to. Partition keys
            are Unicode strings with a maximum length limit of 256 characters for each key. Amazon
            Kinesis Data Streams uses the partition key as input to a hash function that maps the
            partition key and associated data to a specific shard. Specifically, an MD5 hash
            function is used to map partition keys to 128-bit integer values and to map associated
            data records to shards. As a result of this hashing mechanism, all data records with the
            same partition key map to the same shard within the stream.</p> |
| `sequence_number_for_ordering` | String |  | <p>Guarantees strictly increasing sequence numbers, for puts from the same client and to
            the same partition key. Usage: set the <code>SequenceNumberForOrdering</code> of record
                <i>n</i> to the sequence number of record <i>n-1</i> (as
            returned in the result when putting record <i>n-1</i>). If this parameter
            is not set, records are coarsely ordered based on arrival time.</p> |
| `stream_arn` | String |  | <p>The ARN of the stream.</p> |
| `data` | String | ✅ | <p>The data blob to put into the record, which is base64-encoded when the blob is
            serialized. When the data blob (the payload before base64-encoding) is added to the
            partition key size, the total size must not exceed the maximum record size (1
            MiB).</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create record
record = provider.kinesis.Record {
    partition_key = "value"  # <p>Determines which shard in the stream the data record is assigned to. Partition keys
            are Unicode strings with a maximum length limit of 256 characters for each key. Amazon
            Kinesis Data Streams uses the partition key as input to a hash function that maps the
            partition key and associated data to a specific shard. Specifically, an MD5 hash
            function is used to map partition keys to 128-bit integer values and to map associated
            data records to shards. As a result of this hashing mechanism, all data records with the
            same partition key map to the same shard within the stream.</p>
    data = "value"  # <p>The data blob to put into the record, which is base64-encoded when the blob is
            serialized. When the data blob (the payload before base64-encoding) is added to the
            partition key size, the total size must not exceed the maximum record size (1
            MiB).</p>
}

```

---


### Resource_policy

ResourcePolicy resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `policy` | String | ✅ | <p>Details of the resource policy. It must include the identity of the principal and the actions allowed on this resource. This is formatted as a JSON string.</p> |
| `resource_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the data stream or consumer.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `policy` | String | <p>Details of the resource policy. This is formatted as a JSON string.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create resource_policy
resource_policy = provider.kinesis.Resource_policy {
    policy = "value"  # <p>Details of the resource policy. It must include the identity of the principal and the actions allowed on this resource. This is formatted as a JSON string.</p>
    resource_arn = "value"  # <p>The Amazon Resource Name (ARN) of the data stream or consumer.</p>
}

# Access resource_policy outputs
resource_policy_id = resource_policy.id
resource_policy_policy = resource_policy.policy
```

---


### Stream_consumer

StreamConsumer resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `consumer_description` | String | <p>An object that represents the details of the consumer.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access stream_consumer outputs
stream_consumer_id = stream_consumer.id
stream_consumer_consumer_description = stream_consumer.consumer_description
```

---


### Records

Records resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `records` | Vec<String> | ✅ | <p>The records associated with the request.</p> |
| `stream_name` | String |  | <p>The stream name associated with the request.</p> |
| `stream_arn` | String |  | <p>The ARN of the stream.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_shard_iterator` | String | <p>The next position in the shard from which to start sequentially reading data records.
            If set to <code>null</code>, the shard has been closed and the requested iterator does
            not return any more data. </p> |
| `millis_behind_latest` | i64 | <p>The number of milliseconds the <a>GetRecords</a> response is from the tip
            of the stream, indicating how far behind current time the consumer is. A value of zero
            indicates that record processing is caught up, and there are no new records to process
            at this moment.</p> |
| `child_shards` | Vec<String> | <p>The list of the current shard's child shards, returned in the <code>GetRecords</code>
            API's response only when the end of the current shard is reached.</p> |
| `records` | Vec<String> | <p>The data records retrieved from the shard.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create records
records = provider.kinesis.Records {
    records = "value"  # <p>The records associated with the request.</p>
}

# Access records outputs
records_id = records.id
records_next_shard_iterator = records.next_shard_iterator
records_millis_behind_latest = records.millis_behind_latest
records_child_shards = records.child_shards
records_records = records.records
```

---


### Stream_mode

StreamMode resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `stream_mode_details` | String | ✅ | <p> Specifies the capacity mode to which you want to set your data stream. Currently, in
            Kinesis Data Streams, you can choose between an <b>on-demand</b> capacity mode and a <b>provisioned</b> capacity mode for your data streams. </p> |
| `stream_arn` | String | ✅ | <p> Specifies the ARN of the data stream whose capacity mode you want to update. </p> |



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


### Shard_count

ShardCount resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `scaling_type` | String | ✅ | <p>The scaling type. Uniform scaling creates shards of equal size.</p> |
| `stream_arn` | String |  | <p>The ARN of the stream.</p> |
| `target_shard_count` | i64 | ✅ | <p>The new number of shards. This value has the following default limits. By default, you
            cannot do the following: </p>
         <ul>
            <li>
               <p>Set this value to more than double your current shard count for a
                    stream.</p>
            </li>
            <li>
               <p>Set this value below half your current shard count for a stream.</p>
            </li>
            <li>
               <p>Set this value to more than 10000 shards in a stream (the default limit for
                    shard count per stream is 10000 per account per region), unless you request a
                    limit increase.</p>
            </li>
            <li>
               <p>Scale a stream with more than 10000 shards down unless you set this value to
                    less than 10000 shards.</p>
            </li>
         </ul> |
| `stream_name` | String |  | <p>The name of the stream.</p> |



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


### Stream

Stream resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `stream_mode_details` | String |  | <p> Indicates the capacity mode of the data stream. Currently, in Kinesis Data Streams,
            you can choose between an <b>on-demand</b> capacity mode and a
                <b>provisioned</b> capacity mode for your data
            streams.</p> |
| `shard_count` | i64 |  | <p>The number of shards that the stream will use. The throughput of the stream is a
            function of the number of shards; more shards are required for greater provisioned
            throughput.</p> |
| `tags` | HashMap<String, String> |  | <p>A set of up to 50 key-value pairs to use to create the tags. A tag consists of a required key and an optional value.</p> |
| `stream_name` | String | ✅ | <p>A name to identify the stream. The stream name is scoped to the Amazon Web Services
            account used by the application that creates the stream. It is also scoped by Amazon Web Services Region. That is, two streams in two different Amazon Web Services accounts
            can have the same name. Two streams in the same Amazon Web Services account but in two
            different Regions can also have the same name.</p> |
| `max_record_size_in_ki_b` | i64 |  | <p>The maximum record size of a single record in kibibyte (KiB) that you can write to, and read from a stream.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `stream_description` | String | <p>The current status of the stream, the stream Amazon Resource Name (ARN), an array of
            shard objects that comprise the stream, and whether there are more shards
            available.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create stream
stream = provider.kinesis.Stream {
    stream_name = "value"  # <p>A name to identify the stream. The stream name is scoped to the Amazon Web Services
            account used by the application that creates the stream. It is also scoped by Amazon Web Services Region. That is, two streams in two different Amazon Web Services accounts
            can have the same name. Two streams in the same Amazon Web Services account but in two
            different Regions can also have the same name.</p>
}

# Access stream outputs
stream_id = stream.id
stream_stream_description = stream.stream_description
```

---


### Max_record_size

MaxRecordSize resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `stream_arn` | String |  | <p>The Amazon Resource Name (ARN) of the stream for the <code>MaxRecordSize</code> update.</p> |
| `max_record_size_in_ki_b` | i64 | ✅ | <p>The maximum record size of a single record in KiB that you can write to, and read from a stream. Specify a value between 1024 and 10240 KiB (1 to 10 MiB). If you specify a value that is out of this range, <code>UpdateMaxRecordSize</code> sends back an <code>ValidationException</code> message.</p> |



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


### Stream_summary

StreamSummary resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `stream_description_summary` | String | <p>A <a>StreamDescriptionSummary</a> containing information about the
            stream.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access stream_summary outputs
stream_summary_id = stream_summary.id
stream_summary_stream_description_summary = stream_summary.stream_description_summary
```

---


### Ice_server_config

IceServerConfig resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `ice_server_list` | Vec<String> | <p>The list of ICE server information objects.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access ice_server_config outputs
ice_server_config_id = ice_server_config.id
ice_server_config_ice_server_list = ice_server_config.ice_server_list
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple hls_streaming_session_url resources
hls_streaming_session_url_0 = provider.kinesis.Hls_streaming_session_url {
}
hls_streaming_session_url_1 = provider.kinesis.Hls_streaming_session_url {
}
hls_streaming_session_url_2 = provider.kinesis.Hls_streaming_session_url {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    hls_streaming_session_url = provider.kinesis.Hls_streaming_session_url {
    }
```

---

## Related Documentation

- [AWS Kinesis Documentation](https://docs.aws.amazon.com/kinesis/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
