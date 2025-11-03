# Bedrock_agentcore_control Service



**Resources**: 1

---

## Overview

The bedrock_agentcore_control service provides access to 1 resource type:

- [Token_vault](#token_vault) [R]

---

## Resources


### Token_vault

TokenVault resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `token_vault_id` | String | <p>The ID of the token vault.</p> |
| `kms_configuration` | String | <p>The KMS configuration for the token vault.</p> |
| `last_modified_date` | String | <p>The timestamp when the token vault was last modified.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access token_vault outputs
token_vault_id = token_vault.id
token_vault_token_vault_id = token_vault.token_vault_id
token_vault_kms_configuration = token_vault.kms_configuration
token_vault_last_modified_date = token_vault.last_modified_date
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple token_vault resources
token_vault_0 = provider.bedrock_agentcore_control.Token_vault {
}
token_vault_1 = provider.bedrock_agentcore_control.Token_vault {
}
token_vault_2 = provider.bedrock_agentcore_control.Token_vault {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    token_vault = provider.bedrock_agentcore_control.Token_vault {
    }
```

---

## Related Documentation

- [AWS Bedrock_agentcore_control Documentation](https://docs.aws.amazon.com/bedrock_agentcore_control/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
