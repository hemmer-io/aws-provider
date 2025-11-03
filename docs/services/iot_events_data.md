# Iot_events_data Service



**Resources**: 2

---

## Overview

The iot_events_data service provides access to 2 resource types:

- [Alarm](#alarm) [R]
- [Detector](#detector) [R]

---

## Resources


### Alarm

Alarm resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `alarm` | String | <p>Contains information about an alarm.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access alarm outputs
alarm_id = alarm.id
alarm_alarm = alarm.alarm
```

---


### Detector

Detector resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `detector` | String | <p>Information about the detector (instance).</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access detector outputs
detector_id = detector.id
detector_detector = detector.detector
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple alarm resources
alarm_0 = provider.iot_events_data.Alarm {
}
alarm_1 = provider.iot_events_data.Alarm {
}
alarm_2 = provider.iot_events_data.Alarm {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    alarm = provider.iot_events_data.Alarm {
    }
```

---

## Related Documentation

- [AWS Iot_events_data Documentation](https://docs.aws.amazon.com/iot_events_data/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
