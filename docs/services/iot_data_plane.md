# Iot_data_plane Service



**Resources**: 3

---

## Overview

The iot_data_plane service provides access to 3 resource types:

- [Connection](#connection) [D]
- [Thing_shadow](#thing_shadow) [RUD]
- [Retained_message](#retained_message) [R]

---

## Resources


### Connection

Connection resource

**Operations**: ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|



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


### Thing_shadow

ThingShadow resource

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `payload` | String | ✅ | <p>The state information, in JSON format.</p> |
| `shadow_name` | String |  | <p>The name of the shadow.</p> |
| `thing_name` | String | ✅ | <p>The name of the thing.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `payload` | String | <p>The state information, in JSON format.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access thing_shadow outputs
thing_shadow_id = thing_shadow.id
thing_shadow_payload = thing_shadow.payload
```

---


### Retained_message

RetainedMessage resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `user_properties` | String | <p>A base64-encoded JSON string that includes an array of JSON objects, or null if the
      retained message doesn't include any user properties.</p>
         <p>The following example <code>userProperties</code> parameter is a JSON string that
      represents two user properties. Note that it will be base64-encoded:</p>
         <p>
            <code>[{"deviceName": "alpha"}, {"deviceCnt": "45"}]</code>
         </p> |
| `last_modified_time` | String | <p>The Epoch date and time, in milliseconds, when the retained message was stored by IoT.</p> |
| `topic` | String | <p>The topic name to which the retained message was published.</p> |
| `payload` | String | <p>The Base64-encoded message payload of the retained message body.</p> |
| `qos` | i64 | <p>The quality of service (QoS) level used to publish the retained message.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access retained_message outputs
retained_message_id = retained_message.id
retained_message_user_properties = retained_message.user_properties
retained_message_last_modified_time = retained_message.last_modified_time
retained_message_topic = retained_message.topic
retained_message_payload = retained_message.payload
retained_message_qos = retained_message.qos
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple connection resources
connection_0 = provider.iot_data_plane.Connection {
}
connection_1 = provider.iot_data_plane.Connection {
}
connection_2 = provider.iot_data_plane.Connection {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    connection = provider.iot_data_plane.Connection {
    }
```

---

## Related Documentation

- [AWS Iot_data_plane Documentation](https://docs.aws.amazon.com/iot_data_plane/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
