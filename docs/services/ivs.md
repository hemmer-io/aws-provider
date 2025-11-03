# Ivs Service



**Resources**: 8

---

## Overview

The ivs service provides access to 8 resource types:

- [Playback_restriction_policy](#playback_restriction_policy) [CRUD]
- [Playback_key_pair](#playback_key_pair) [RD]
- [Metadata](#metadata) [C]
- [Recording_configuration](#recording_configuration) [CRD]
- [Stream_session](#stream_session) [R]
- [Stream_key](#stream_key) [CRD]
- [Channel](#channel) [CRUD]
- [Stream](#stream) [R]

---

## Resources


### Playback_restriction_policy

PlaybackRestrictionPolicy resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `enable_strict_origin_enforcement` | bool |  | <p>Whether channel playback is constrained by origin site. Default:
      <code>false</code>.</p> |
| `allowed_origins` | Vec<String> |  | <p>A list of origin sites that control CORS restriction. Allowed values are the same as valid
      values of the Origin header defined at <a href="https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Origin">https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Origin</a>. Default: All
      origins (an empty array).</p> |
| `name` | String |  | <p>Playback-restriction-policy name. The value does not need to be unique.</p> |
| `tags` | HashMap<String, String> |  | <p>Array of 1-50 maps, each of the form <code>string:string (key:value)</code>. See <a href="https://docs.aws.amazon.com/tag-editor/latest/userguide/best-practices-and-strats.html">Best practices and strategies</a> in <i>Tagging Amazon Web Services Resources and Tag Editor</i> for details, including restrictions that apply to tags and "Tag naming limits and requirements"; Amazon IVS has no service-specific constraints beyond what is
      documented there.</p> |
| `allowed_countries` | Vec<String> |  | <p>A list of country codes that control geoblocking restriction. Allowed values are the
      officially assigned <a href="https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2">ISO 3166-1
        alpha-2</a> codes. Default: All countries (an empty array).</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `playback_restriction_policy` | String | <p/> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create playback_restriction_policy
playback_restriction_policy = provider.ivs.Playback_restriction_policy {
}

# Access playback_restriction_policy outputs
playback_restriction_policy_id = playback_restriction_policy.id
playback_restriction_policy_playback_restriction_policy = playback_restriction_policy.playback_restriction_policy
```

---


### Playback_key_pair

PlaybackKeyPair resource

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `key_pair` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access playback_key_pair outputs
playback_key_pair_id = playback_key_pair.id
playback_key_pair_key_pair = playback_key_pair.key_pair
```

---


### Metadata

Metadata resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `metadata` | String | ✅ | <p>Metadata to insert into the stream. Maximum: 1 KB per request.</p> |
| `channel_arn` | String | ✅ | <p>ARN of the channel into which metadata is inserted. This channel must have an active
      stream.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create metadata
metadata = provider.ivs.Metadata {
    metadata = "value"  # <p>Metadata to insert into the stream. Maximum: 1 KB per request.</p>
    channel_arn = "value"  # <p>ARN of the channel into which metadata is inserted. This channel must have an active
      stream.</p>
}

```

---


### Recording_configuration

RecordingConfiguration resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `thumbnail_configuration` | String |  | <p>A complex type that allows you to enable/disable the recording of thumbnails for a live
      session and modify the interval at which thumbnails are generated for the live session.</p> |
| `recording_reconnect_window_seconds` | i64 |  | <p>If a broadcast disconnects and then reconnects within the specified interval, the multiple
      streams will be considered a single broadcast and merged together. Default: 0.</p> |
| `rendition_configuration` | String |  | <p>Object that describes which renditions should be recorded for a stream.</p> |
| `destination_configuration` | String | ✅ | <p>A complex type that contains a destination configuration for where recorded video will be
      stored.</p> |
| `name` | String |  | <p>Recording-configuration name. The value does not need to be unique.</p> |
| `tags` | HashMap<String, String> |  | <p>Array of 1-50 maps, each of the form <code>string:string (key:value)</code>. See <a href="https://docs.aws.amazon.com/tag-editor/latest/userguide/best-practices-and-strats.html">Best practices and strategies</a> in <i>Tagging Amazon Web Services Resources and Tag Editor</i> for details, including restrictions that apply to tags and "Tag naming limits and requirements"; Amazon IVS has no service-specific constraints beyond what is
      documented there.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `recording_configuration` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create recording_configuration
recording_configuration = provider.ivs.Recording_configuration {
    destination_configuration = "value"  # <p>A complex type that contains a destination configuration for where recorded video will be
      stored.</p>
}

# Access recording_configuration outputs
recording_configuration_id = recording_configuration.id
recording_configuration_recording_configuration = recording_configuration.recording_configuration
```

---


### Stream_session

StreamSession resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `stream_session` | String | <p>List of stream details.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access stream_session outputs
stream_session_id = stream_session.id
stream_session_stream_session = stream_session.stream_session
```

---


### Stream_key

StreamKey resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | HashMap<String, String> |  | <p>Array of 1-50 maps, each of the form <code>string:string (key:value)</code>. See <a href="https://docs.aws.amazon.com/tag-editor/latest/userguide/best-practices-and-strats.html">Best practices and strategies</a> in <i>Tagging Amazon Web Services Resources and Tag Editor</i> for details, including restrictions that apply to tags and "Tag naming limits and requirements"; Amazon IVS has no service-specific constraints beyond what is
      documented there.</p> |
| `channel_arn` | String | ✅ | <p>ARN of the channel for which to create the stream key.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `stream_key` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create stream_key
stream_key = provider.ivs.Stream_key {
    channel_arn = "value"  # <p>ARN of the channel for which to create the stream key.</p>
}

# Access stream_key outputs
stream_key_id = stream_key.id
stream_key_stream_key = stream_key.stream_key
```

---


### Channel

Channel resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `authorized` | bool |  | <p>Whether the channel is private (enabled for playback authorization). Default:
        <code>false</code>.</p> |
| `tags` | HashMap<String, String> |  | <p>Array of 1-50 maps, each of the form <code>string:string (key:value)</code>. See <a href="https://docs.aws.amazon.com/tag-editor/latest/userguide/best-practices-and-strats.html">Best practices and strategies</a> in <i>Tagging Amazon Web Services Resources and Tag Editor</i> for details, including restrictions that apply to tags and "Tag naming limits and requirements"; Amazon IVS has no service-specific constraints beyond what is
      documented there.</p> |
| `preset` | String |  | <p>Optional transcode preset for the channel. This is selectable only for
        <code>ADVANCED_HD</code> and <code>ADVANCED_SD</code> channel types. For those channel
      types, the default <code>preset</code> is <code>HIGHER_BANDWIDTH_DELIVERY</code>. For other
      channel types (<code>BASIC</code> and <code>STANDARD</code>), <code>preset</code> is the empty
      string (<code>""</code>).</p> |
| `recording_configuration_arn` | String |  | <p>Recording-configuration ARN. A valid ARN value here both specifies the ARN and enables recording.
     Default: "" (empty string, recording is disabled).</p> |
| `container_format` | String |  | <p>Indicates which content-packaging format is used (MPEG-TS or fMP4). If <code>multitrackInputConfiguration</code> is specified and <code>enabled</code> is <code>true</code>, then <code>containerFormat</code> is required and must be set to <code>FRAGMENTED_MP4</code>. Otherwise, <code>containerFormat</code> may be set to <code>TS</code> or <code>FRAGMENTED_MP4</code>. Default: <code>TS</code>.</p> |
| `type` | String |  | <p>Channel type, which determines the allowable resolution and bitrate. <i>If you
        exceed the allowable input resolution or bitrate, the stream probably will disconnect
        immediately.</i> Default: <code>STANDARD</code>. For details, see <a href="https://docs.aws.amazon.com/ivs/latest/LowLatencyAPIReference/channel-types.html">Channel
        Types</a>.</p> |
| `multitrack_input_configuration` | String |  | <p>Object specifying multitrack input configuration. Default: no multitrack input configuration is specified.</p> |
| `latency_mode` | String |  | <p>Channel latency mode. Use <code>NORMAL</code> to broadcast and deliver live video up to
      Full HD. Use <code>LOW</code> for near-real-time interaction with viewers. Default: <code>LOW</code>.</p> |
| `insecure_ingest` | bool |  | <p>Whether the channel allows insecure RTMP and SRT ingest. Default: <code>false</code>.</p> |
| `playback_restriction_policy_arn` | String |  | <p>Playback-restriction-policy ARN. A valid ARN value here both specifies the ARN and enables playback restriction.
    Default: "" (empty string, no playback restriction policy is applied).</p> |
| `name` | String |  | <p>Channel name.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `channel` | String | <p/> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create channel
channel = provider.ivs.Channel {
}

# Access channel outputs
channel_id = channel.id
channel_channel = channel.channel
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
| `stream` | String | <p/> |


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
stream_stream = stream.stream
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple playback_restriction_policy resources
playback_restriction_policy_0 = provider.ivs.Playback_restriction_policy {
}
playback_restriction_policy_1 = provider.ivs.Playback_restriction_policy {
}
playback_restriction_policy_2 = provider.ivs.Playback_restriction_policy {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    playback_restriction_policy = provider.ivs.Playback_restriction_policy {
    }
```

---

## Related Documentation

- [AWS Ivs Documentation](https://docs.aws.amazon.com/ivs/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
