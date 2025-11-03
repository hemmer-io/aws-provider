# Transcribe_streaming Service



**Resources**: 1

---

## Overview

The transcribe_streaming service provides access to 1 resource type:

- [Medical_scribe_stream](#medical_scribe_stream) [R]

---

## Resources


### Medical_scribe_stream

MedicalScribeStream resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `medical_scribe_stream_details` | String | <p>Provides details about a HealthScribe streaming session.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access medical_scribe_stream outputs
medical_scribe_stream_id = medical_scribe_stream.id
medical_scribe_stream_medical_scribe_stream_details = medical_scribe_stream.medical_scribe_stream_details
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple medical_scribe_stream resources
medical_scribe_stream_0 = provider.transcribe_streaming.Medical_scribe_stream {
}
medical_scribe_stream_1 = provider.transcribe_streaming.Medical_scribe_stream {
}
medical_scribe_stream_2 = provider.transcribe_streaming.Medical_scribe_stream {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    medical_scribe_stream = provider.transcribe_streaming.Medical_scribe_stream {
    }
```

---

## Related Documentation

- [AWS Transcribe_streaming Documentation](https://docs.aws.amazon.com/transcribe_streaming/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
