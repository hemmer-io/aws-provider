# Marketplace_entitlement_service Service



**Resources**: 1

---

## Overview

The marketplace_entitlement_service service provides access to 1 resource type:

- [Entitlements](#entitlements) [R]

---

## Resources


### Entitlements

Entitlements resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `entitlements` | Vec<String> | <p>The set of entitlements found through the GetEntitlements operation. If the result
      contains an empty set of entitlements, NextToken might still be present and should be
      used.</p> |
| `next_token` | String | <p>For paginated results, use NextToken in subsequent calls to GetEntitlements. If the
      result contains an empty set of entitlements, NextToken might still be present and should be
      used.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access entitlements outputs
entitlements_id = entitlements.id
entitlements_entitlements = entitlements.entitlements
entitlements_next_token = entitlements.next_token
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple entitlements resources
entitlements_0 = provider.marketplace_entitlement_service.Entitlements {
}
entitlements_1 = provider.marketplace_entitlement_service.Entitlements {
}
entitlements_2 = provider.marketplace_entitlement_service.Entitlements {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    entitlements = provider.marketplace_entitlement_service.Entitlements {
    }
```

---

## Related Documentation

- [AWS Marketplace_entitlement_service Documentation](https://docs.aws.amazon.com/marketplace_entitlement_service/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
