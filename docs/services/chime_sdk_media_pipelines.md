# Chime_sdk_media_pipelines Service



**Resources**: 11

---

## Overview

The chime_sdk_media_pipelines service provides access to 11 resource types:

- [Media_insights_pipeline_status](#media_insights_pipeline_status) [U]
- [Media_capture_pipeline](#media_capture_pipeline) [CRD]
- [Media_insights_pipeline_configuration](#media_insights_pipeline_configuration) [CRUD]
- [Media_live_connector_pipeline](#media_live_connector_pipeline) [C]
- [Media_insights_pipeline](#media_insights_pipeline) [C]
- [Media_stream_pipeline](#media_stream_pipeline) [C]
- [Media_pipeline_kinesis_video_stream_pool](#media_pipeline_kinesis_video_stream_pool) [CRUD]
- [Media_pipeline](#media_pipeline) [RD]
- [Speaker_search_task](#speaker_search_task) [R]
- [Media_concatenation_pipeline](#media_concatenation_pipeline) [C]
- [Voice_tone_analysis_task](#voice_tone_analysis_task) [R]

---

## Resources


### Media_insights_pipeline_status

MediaInsightsPipelineStatus resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `identifier` | String | ✅ | <p>The unique identifier of the resource to be updated. Valid values include the ID and ARN of the media insights pipeline.</p> |
| `update_status` | String | ✅ | <p>The requested status of the media insights pipeline.</p> |



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


### Media_capture_pipeline

MediaCapturePipeline resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `sink_arn` | String | ✅ | <p>The ARN of the sink type.</p> |
| `sse_aws_key_management_params` | String |  | <p>An object that contains server side encryption parameters to be used by media capture
         pipeline. The parameters can also be used by media concatenation pipeline taking media
         capture pipeline as a media source.</p> |
| `sink_iam_role_arn` | String |  | <p>The Amazon Resource Name (ARN) of the sink role to be used with <code>AwsKmsKeyId</code>
         in <code>SseAwsKeyManagementParams</code>. Can only interact with <code>S3Bucket</code>
         sink type. The role must belong to the caller’s account and be able to act on behalf of the
         caller during the API call. All minimum policy permissions requirements for the caller to
         perform sink-related actions are the same for <code>SinkIamRoleArn</code>.</p>
         <p>Additionally, the role must have permission to <code>kms:GenerateDataKey</code> using
         KMS key supplied as <code>AwsKmsKeyId</code> in <code>SseAwsKeyManagementParams</code>. If
         media concatenation will be required later, the role must also have permission to
         <code>kms:Decrypt</code> for the same KMS key.</p> |
| `chime_sdk_meeting_configuration` | String |  | <p>The configuration for a specified media pipeline. <code>SourceType</code> must
         be <code>ChimeSdkMeeting</code>.</p> |
| `source_type` | String | ✅ | <p>Source type from which the media artifacts are captured. A Chime SDK Meeting is the only
         supported source.</p> |
| `tags` | Vec<String> |  | <p>The tag key-value pairs.</p> |
| `client_request_token` | String |  | <p>The unique identifier for the client request. The token makes the API request idempotent. Use a unique token for each media pipeline request.</p> |
| `sink_type` | String | ✅ | <p>Destination type to which the media artifacts are saved. You must use an S3 bucket.</p> |
| `source_arn` | String | ✅ | <p>ARN of the source from which the media artifacts are captured.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `media_capture_pipeline` | String | <p>The media pipeline object.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create media_capture_pipeline
media_capture_pipeline = provider.chime_sdk_media_pipelines.Media_capture_pipeline {
    sink_arn = "value"  # <p>The ARN of the sink type.</p>
    source_type = "value"  # <p>Source type from which the media artifacts are captured. A Chime SDK Meeting is the only
         supported source.</p>
    sink_type = "value"  # <p>Destination type to which the media artifacts are saved. You must use an S3 bucket.</p>
    source_arn = "value"  # <p>ARN of the source from which the media artifacts are captured.</p>
}

# Access media_capture_pipeline outputs
media_capture_pipeline_id = media_capture_pipeline.id
media_capture_pipeline_media_capture_pipeline = media_capture_pipeline.media_capture_pipeline
```

---


### Media_insights_pipeline_configuration

MediaInsightsPipelineConfiguration resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_request_token` | String |  | <p>The unique identifier for the media insights pipeline configuration request.</p> |
| `resource_access_role_arn` | String | ✅ | <p>The ARN of the role used by the service to access Amazon Web Services resources,
         including <code>Transcribe</code> and <code>Transcribe Call Analytics</code>, on the 
         caller’s behalf.</p> |
| `media_insights_pipeline_configuration_name` | String | ✅ | <p>The name of the media insights pipeline configuration.</p> |
| `tags` | Vec<String> |  | <p>The tags assigned to the media insights pipeline configuration.</p> |
| `real_time_alert_configuration` | String |  | <p>The configuration settings for the real-time alerts in a media insights pipeline configuration.</p> |
| `elements` | Vec<String> | ✅ | <p>The elements in the request, such as a processor for Amazon Transcribe or a sink for a Kinesis Data Stream.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `media_insights_pipeline_configuration` | String | <p>The requested media insights pipeline configuration.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create media_insights_pipeline_configuration
media_insights_pipeline_configuration = provider.chime_sdk_media_pipelines.Media_insights_pipeline_configuration {
    resource_access_role_arn = "value"  # <p>The ARN of the role used by the service to access Amazon Web Services resources,
         including <code>Transcribe</code> and <code>Transcribe Call Analytics</code>, on the 
         caller’s behalf.</p>
    media_insights_pipeline_configuration_name = "value"  # <p>The name of the media insights pipeline configuration.</p>
    elements = "value"  # <p>The elements in the request, such as a processor for Amazon Transcribe or a sink for a Kinesis Data Stream.</p>
}

# Access media_insights_pipeline_configuration outputs
media_insights_pipeline_configuration_id = media_insights_pipeline_configuration.id
media_insights_pipeline_configuration_media_insights_pipeline_configuration = media_insights_pipeline_configuration.media_insights_pipeline_configuration
```

---


### Media_live_connector_pipeline

MediaLiveConnectorPipeline resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `sinks` | Vec<String> | ✅ | <p>The media live connector pipeline's data sinks.</p> |
| `tags` | Vec<String> |  | <p>The tags associated with the media live connector pipeline.</p> |
| `client_request_token` | String |  | <p>The token assigned to the client making the request.</p> |
| `sources` | Vec<String> | ✅ | <p>The media live connector pipeline's data sources.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create media_live_connector_pipeline
media_live_connector_pipeline = provider.chime_sdk_media_pipelines.Media_live_connector_pipeline {
    sinks = "value"  # <p>The media live connector pipeline's data sinks.</p>
    sources = "value"  # <p>The media live connector pipeline's data sources.</p>
}

```

---


### Media_insights_pipeline

MediaInsightsPipeline resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_request_token` | String |  | <p>The unique identifier for the media insights pipeline request.</p> |
| `media_insights_pipeline_configuration_arn` | String | ✅ | <p>The ARN of the pipeline's configuration.</p> |
| `kinesis_video_stream_source_runtime_configuration` | String |  | <p>The runtime configuration for the Kinesis video stream source of the media insights
         pipeline.</p> |
| `media_insights_runtime_metadata` | HashMap<String, String> |  | <p>The runtime metadata for the media insights pipeline. Consists of a key-value map of strings.</p> |
| `kinesis_video_stream_recording_source_runtime_configuration` | String |  | <p>The runtime configuration for the Kinesis video recording stream source.</p> |
| `s3_recording_sink_runtime_configuration` | String |  | <p>The runtime configuration for the S3 recording sink. If specified, the settings in this structure override any settings in <code>S3RecordingSinkConfiguration</code>.</p> |
| `tags` | Vec<String> |  | <p>The tags assigned to the media insights pipeline.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create media_insights_pipeline
media_insights_pipeline = provider.chime_sdk_media_pipelines.Media_insights_pipeline {
    media_insights_pipeline_configuration_arn = "value"  # <p>The ARN of the pipeline's configuration.</p>
}

```

---


### Media_stream_pipeline

MediaStreamPipeline resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_request_token` | String |  | <p>The token assigned to the client making the request.</p> |
| `sinks` | Vec<String> | ✅ | <p>The data sink for the media pipeline.</p> |
| `sources` | Vec<String> | ✅ | <p>The data sources for the media pipeline.</p> |
| `tags` | Vec<String> |  | <p>The tags assigned to the media pipeline.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create media_stream_pipeline
media_stream_pipeline = provider.chime_sdk_media_pipelines.Media_stream_pipeline {
    sinks = "value"  # <p>The data sink for the media pipeline.</p>
    sources = "value"  # <p>The data sources for the media pipeline.</p>
}

```

---


### Media_pipeline_kinesis_video_stream_pool

MediaPipelineKinesisVideoStreamPool resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_request_token` | String |  | <p>The token assigned to the client making the request.</p> |
| `stream_configuration` | String | ✅ | <p>The configuration settings for the stream.</p> |
| `pool_name` | String | ✅ | <p>The name of the pool.</p> |
| `tags` | Vec<String> |  | <p>The tags assigned to the stream pool.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kinesis_video_stream_pool_configuration` | String | <p>The video stream pool configuration object.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create media_pipeline_kinesis_video_stream_pool
media_pipeline_kinesis_video_stream_pool = provider.chime_sdk_media_pipelines.Media_pipeline_kinesis_video_stream_pool {
    stream_configuration = "value"  # <p>The configuration settings for the stream.</p>
    pool_name = "value"  # <p>The name of the pool.</p>
}

# Access media_pipeline_kinesis_video_stream_pool outputs
media_pipeline_kinesis_video_stream_pool_id = media_pipeline_kinesis_video_stream_pool.id
media_pipeline_kinesis_video_stream_pool_kinesis_video_stream_pool_configuration = media_pipeline_kinesis_video_stream_pool.kinesis_video_stream_pool_configuration
```

---


### Media_pipeline

MediaPipeline resource

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `media_pipeline` | String | <p>The media pipeline object.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access media_pipeline outputs
media_pipeline_id = media_pipeline.id
media_pipeline_media_pipeline = media_pipeline.media_pipeline
```

---


### Speaker_search_task

SpeakerSearchTask resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `speaker_search_task` | String | <p>The details of the speaker search task.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access speaker_search_task outputs
speaker_search_task_id = speaker_search_task.id
speaker_search_task_speaker_search_task = speaker_search_task.speaker_search_task
```

---


### Media_concatenation_pipeline

MediaConcatenationPipeline resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>The tags associated with the media concatenation pipeline.</p> |
| `sources` | Vec<String> | ✅ | <p>An object that specifies the sources for the media concatenation pipeline.</p> |
| `sinks` | Vec<String> | ✅ | <p>An object that specifies the data sinks for the media concatenation pipeline.</p> |
| `client_request_token` | String |  | <p>The unique identifier for the client request. The token makes the API request
         idempotent. Use a unique token for each media concatenation pipeline request.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create media_concatenation_pipeline
media_concatenation_pipeline = provider.chime_sdk_media_pipelines.Media_concatenation_pipeline {
    sources = "value"  # <p>An object that specifies the sources for the media concatenation pipeline.</p>
    sinks = "value"  # <p>An object that specifies the data sinks for the media concatenation pipeline.</p>
}

```

---


### Voice_tone_analysis_task

VoiceToneAnalysisTask resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `voice_tone_analysis_task` | String | <p>The details of the voice tone analysis task.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access voice_tone_analysis_task outputs
voice_tone_analysis_task_id = voice_tone_analysis_task.id
voice_tone_analysis_task_voice_tone_analysis_task = voice_tone_analysis_task.voice_tone_analysis_task
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple media_insights_pipeline_status resources
media_insights_pipeline_status_0 = provider.chime_sdk_media_pipelines.Media_insights_pipeline_status {
    identifier = "value-0"
    update_status = "value-0"
}
media_insights_pipeline_status_1 = provider.chime_sdk_media_pipelines.Media_insights_pipeline_status {
    identifier = "value-1"
    update_status = "value-1"
}
media_insights_pipeline_status_2 = provider.chime_sdk_media_pipelines.Media_insights_pipeline_status {
    identifier = "value-2"
    update_status = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    media_insights_pipeline_status = provider.chime_sdk_media_pipelines.Media_insights_pipeline_status {
        identifier = "production-value"
        update_status = "production-value"
    }
```

---

## Related Documentation

- [AWS Chime_sdk_media_pipelines Documentation](https://docs.aws.amazon.com/chime_sdk_media_pipelines/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
