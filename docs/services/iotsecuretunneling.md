# Iotsecuretunneling Service



**Resources**: 1

---

## Overview

The iotsecuretunneling service provides access to 1 resource type:

- [Tunnel](#tunnel) [R]

---

## Resources


### Tunnel

Tunnel resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `tunnel` | String | <p>The tunnel being described.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access tunnel outputs
tunnel_id = tunnel.id
tunnel_tunnel = tunnel.tunnel
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple tunnel resources
tunnel_0 = provider.iotsecuretunneling.Tunnel {
}
tunnel_1 = provider.iotsecuretunneling.Tunnel {
}
tunnel_2 = provider.iotsecuretunneling.Tunnel {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    tunnel = provider.iotsecuretunneling.Tunnel {
    }
```

---

## Related Documentation

- [AWS Iotsecuretunneling Documentation](https://docs.aws.amazon.com/iotsecuretunneling/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
