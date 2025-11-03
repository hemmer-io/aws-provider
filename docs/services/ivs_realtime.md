# Ivs_realtime Service



**Resources**: 9

---

## Overview

The ivs_realtime service provides access to 9 resource types:

- [Participant_token](#participant_token) [C]
- [Composition](#composition) [R]
- [Storage_configuration](#storage_configuration) [CRD]
- [Participant](#participant) [R]
- [Public_key](#public_key) [RD]
- [Stage_session](#stage_session) [R]
- [Ingest_configuration](#ingest_configuration) [CRUD]
- [Stage](#stage) [CRUD]
- [Encoder_configuration](#encoder_configuration) [CRD]

---

## Resources


### Participant_token

ParticipantToken resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `attributes` | HashMap<String, String> |  | <p>Application-provided attributes to encode into the token and attach to a stage. Map keys
         and values can contain UTF-8 encoded text. The maximum length of this field is 1 KB total.
            <i>This field is exposed to all stage participants and should not be used for
            personally identifying, confidential, or sensitive information.</i>
         </p> |
| `capabilities` | Vec<String> |  | <p>Set of capabilities that the user is allowed to perform in the stage. Default:
            <code>PUBLISH, SUBSCRIBE</code>.</p> |
| `user_id` | String |  | <p>Name that can be specified to help identify the token. This can be any UTF-8 encoded
         text. <i>This field is exposed to all stage participants and should not be used for
            personally identifying, confidential, or sensitive information.</i>
         </p> |
| `stage_arn` | String | ✅ | <p>ARN of the stage to which this token is scoped.</p> |
| `duration` | i64 |  | <p>Duration (in minutes), after which the token expires. Default: 720 (12 hours).</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create participant_token
participant_token = provider.ivs_realtime.Participant_token {
    stage_arn = "value"  # <p>ARN of the stage to which this token is scoped.</p>
}

```

---


### Composition

Composition resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `composition` | String | <p>The Composition that was returned.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access composition outputs
composition_id = composition.id
composition_composition = composition.composition
```

---


### Storage_configuration

StorageConfiguration resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `s3` | String | ✅ | <p>A complex type that contains a storage configuration for where recorded video will be stored.</p> |
| `name` | String |  | <p>Storage configuration name. The value does not need to be unique.</p> |
| `tags` | HashMap<String, String> |  | <p>Tags attached to the resource. Array of maps, each of the form <code>string:string
         (key:value)</code>. See <a href="https://docs.aws.amazon.com/tag-editor/latest/userguide/best-practices-and-strats.html">Best practices and strategies</a>
	 in <i>Tagging AWS Resources and Tag Editor</i> for details, including restrictions that apply to tags and "Tag naming
         limits and requirements"; Amazon IVS has no constraints on tags beyond what is documented
         there.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `storage_configuration` | String | <p>The StorageConfiguration that was returned.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create storage_configuration
storage_configuration = provider.ivs_realtime.Storage_configuration {
    s3 = "value"  # <p>A complex type that contains a storage configuration for where recorded video will be stored.</p>
}

# Access storage_configuration outputs
storage_configuration_id = storage_configuration.id
storage_configuration_storage_configuration = storage_configuration.storage_configuration
```

---


### Participant

Participant resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `participant` | String | <p>The participant that is returned.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access participant outputs
participant_id = participant.id
participant_participant = participant.participant
```

---


### Public_key

PublicKey resource

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `public_key` | String | <p>The public key that is returned.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access public_key outputs
public_key_id = public_key.id
public_key_public_key = public_key.public_key
```

---


### Stage_session

StageSession resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `stage_session` | String | <p>The stage session that is returned.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access stage_session outputs
stage_session_id = stage_session.id
stage_session_stage_session = stage_session.stage_session
```

---


### Ingest_configuration

IngestConfiguration resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `insecure_ingest` | bool |  | <p>Whether the stage allows insecure RTMP ingest. This must be set to <code>true</code>, if <code>ingestProtocol</code> is set to <code>RTMP</code>. Default: <code>false</code>. </p> |
| `stage_arn` | String |  | <p>ARN of the stage with which the IngestConfiguration is associated.</p> |
| `name` | String |  | <p>Optional name that can be specified for the IngestConfiguration being created.</p> |
| `tags` | HashMap<String, String> |  | <p>Tags attached to the resource. Array of maps, each of the form <code>string:string
         (key:value)</code>. See <a href="https://docs.aws.amazon.com/tag-editor/latest/userguide/best-practices-and-strats.html">Best practices and strategies</a>
         in <i>Tagging AWS Resources and Tag Editor</i> for details, including restrictions that apply to tags and "Tag naming
         limits and requirements"; Amazon IVS has no constraints on tags beyond what is documented
	 there.</p> |
| `ingest_protocol` | String | ✅ | <p>Type of ingest protocol that the user employs to broadcast. If this is set to <code>RTMP</code>, <code>insecureIngest</code> must be set to <code>true</code>.</p> |
| `attributes` | HashMap<String, String> |  | <p>Application-provided attributes to store in the IngestConfiguration and attach to a stage. Map keys and values can contain UTF-8 encoded text. The maximum length of this field is 1 KB total.
      <i>This field is exposed to all stage participants and should not be used for personally identifying, confidential, or sensitive information.</i>
         </p> |
| `user_id` | String |  | <p>Customer-assigned name to help identify the participant using the IngestConfiguration; this can be used to link a participant to a user in the customer’s own systems. This can be any UTF-8 encoded text.
      <i>This field is exposed to all stage participants and should not be used for personally identifying, confidential, or sensitive information.</i>
         </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `ingest_configuration` | String | <p>The IngestConfiguration that was returned.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create ingest_configuration
ingest_configuration = provider.ivs_realtime.Ingest_configuration {
    ingest_protocol = "value"  # <p>Type of ingest protocol that the user employs to broadcast. If this is set to <code>RTMP</code>, <code>insecureIngest</code> must be set to <code>true</code>.</p>
}

# Access ingest_configuration outputs
ingest_configuration_id = ingest_configuration.id
ingest_configuration_ingest_configuration = ingest_configuration.ingest_configuration
```

---


### Stage

Stage resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | HashMap<String, String> |  | <p>Tags attached to the resource. Array of maps, each of the form <code>string:string
         (key:value)</code>. See <a href="https://docs.aws.amazon.com/tag-editor/latest/userguide/best-practices-and-strats.html">Best practices and strategies</a>
         in <i>Tagging AWS Resources and Tag Editor</i> for details, including restrictions that apply to tags and "Tag naming
         limits and requirements"; Amazon IVS has no constraints on tags beyond what is documented
         there. </p> |
| `participant_token_configurations` | Vec<String> |  | <p>Array of participant token configuration objects to attach to the new stage.</p> |
| `auto_participant_recording_configuration` | String |  | <p>Configuration object for individual participant recording, to attach to the new stage.</p> |
| `name` | String |  | <p>Optional name that can be specified for the stage being created.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `stage` | String | <p>The stage that is returned.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create stage
stage = provider.ivs_realtime.Stage {
}

# Access stage outputs
stage_id = stage.id
stage_stage = stage.stage
```

---


### Encoder_configuration

EncoderConfiguration resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | <p>Optional name to identify the resource.</p> |
| `video` | String |  | <p>Video configuration. Default: video resolution 1280x720, bitrate 2500 kbps, 30
         fps.</p> |
| `tags` | HashMap<String, String> |  | <p>Tags attached to the resource. Array of maps, each of the form <code>string:string
         (key:value)</code>. See <a href="https://docs.aws.amazon.com/tag-editor/latest/userguide/best-practices-and-strats.html">Best practices and strategies</a>
         in <i>Tagging AWS Resources and Tag Editor</i> for details, including restrictions that apply to tags and "Tag naming
         limits and requirements"; Amazon IVS has no constraints on tags beyond what is documented
         there.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `encoder_configuration` | String | <p>The EncoderConfiguration that was returned.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create encoder_configuration
encoder_configuration = provider.ivs_realtime.Encoder_configuration {
}

# Access encoder_configuration outputs
encoder_configuration_id = encoder_configuration.id
encoder_configuration_encoder_configuration = encoder_configuration.encoder_configuration
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple participant_token resources
participant_token_0 = provider.ivs_realtime.Participant_token {
    stage_arn = "value-0"
}
participant_token_1 = provider.ivs_realtime.Participant_token {
    stage_arn = "value-1"
}
participant_token_2 = provider.ivs_realtime.Participant_token {
    stage_arn = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    participant_token = provider.ivs_realtime.Participant_token {
        stage_arn = "production-value"
    }
```

---

## Related Documentation

- [AWS Ivs_realtime Documentation](https://docs.aws.amazon.com/ivs_realtime/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
