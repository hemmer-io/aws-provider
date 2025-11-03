# Apigatewaymanagementapi Service



**Resources**: 1

---

## Overview

The apigatewaymanagementapi service provides access to 1 resource type:

- [Connection](#connection) [RD]

---

## Resources


### Connection

Connection resource

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `connected_at` | String | <p>The time in ISO 8601 format for when the connection was established.</p> |
| `identity` | String |  |
| `last_active_at` | String | <p>The time in ISO 8601 format for when the connection was last active.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access connection outputs
connection_id = connection.id
connection_connected_at = connection.connected_at
connection_identity = connection.identity
connection_last_active_at = connection.last_active_at
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
connection_0 = provider.apigatewaymanagementapi.Connection {
}
connection_1 = provider.apigatewaymanagementapi.Connection {
}
connection_2 = provider.apigatewaymanagementapi.Connection {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    connection = provider.apigatewaymanagementapi.Connection {
    }
```

---

## Related Documentation

- [AWS Apigatewaymanagementapi Documentation](https://docs.aws.amazon.com/apigatewaymanagementapi/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
