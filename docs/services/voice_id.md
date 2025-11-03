# Voice_id Service



**Resources**: 5

---

## Overview

The voice_id service provides access to 5 resource types:

- [Fraudster_registration_job](#fraudster_registration_job) [R]
- [Fraudster](#fraudster) [RD]
- [Speaker_enrollment_job](#speaker_enrollment_job) [R]
- [Watchlist](#watchlist) [CRUD]
- [Speaker](#speaker) [RD]

---

## Resources


### Fraudster_registration_job

FraudsterRegistrationJob resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `job` | String | <p>Contains details about the specified fraudster registration job.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access fraudster_registration_job outputs
fraudster_registration_job_id = fraudster_registration_job.id
fraudster_registration_job_job = fraudster_registration_job.job
```

---


### Fraudster

Fraudster resource

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `fraudster` | String | <p>Information about the specified fraudster.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access fraudster outputs
fraudster_id = fraudster.id
fraudster_fraudster = fraudster.fraudster
```

---


### Speaker_enrollment_job

SpeakerEnrollmentJob resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `job` | String | <p>Contains details about the specified speaker enrollment job.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access speaker_enrollment_job outputs
speaker_enrollment_job_id = speaker_enrollment_job.id
speaker_enrollment_job_job = speaker_enrollment_job.job
```

---


### Watchlist

Watchlist resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>The name of the watchlist.</p> |
| `description` | String |  | <p>A brief description of this watchlist.</p> |
| `client_token` | String |  | <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the
            request. If not provided, the Amazon Web Services
            SDK populates this field. For more information about idempotency, see
            <a href="https://aws.amazon.com/builders-library/making-retries-safe-with-idempotent-APIs/">Making retries safe with idempotent APIs</a>.</p> |
| `domain_id` | String | ✅ | <p>The identifier of the domain that contains the watchlist.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `watchlist` | String | <p>Information about the specified watchlist.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create watchlist
watchlist = provider.voice_id.Watchlist {
    name = "value"  # <p>The name of the watchlist.</p>
    domain_id = "value"  # <p>The identifier of the domain that contains the watchlist.</p>
}

# Access watchlist outputs
watchlist_id = watchlist.id
watchlist_watchlist = watchlist.watchlist
```

---


### Speaker

Speaker resource

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `speaker` | String | <p>Information about the specified speaker.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access speaker outputs
speaker_id = speaker.id
speaker_speaker = speaker.speaker
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple fraudster_registration_job resources
fraudster_registration_job_0 = provider.voice_id.Fraudster_registration_job {
}
fraudster_registration_job_1 = provider.voice_id.Fraudster_registration_job {
}
fraudster_registration_job_2 = provider.voice_id.Fraudster_registration_job {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    fraudster_registration_job = provider.voice_id.Fraudster_registration_job {
    }
```

---

## Related Documentation

- [AWS Voice_id Documentation](https://docs.aws.amazon.com/voice_id/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
