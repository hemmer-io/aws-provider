# Lambda Service



**Resources**: 1

---

## Overview

The lambda service provides access to 1 resource type:

- [Account_settings](#account_settings) [R]

---

## Resources


### Account_settings

AccountSettings resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `account_usage` | String | <p>The number of functions and amount of storage in use.</p> |
| `account_limit` | String | <p>Limits that are related to concurrency and code storage.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access account_settings outputs
account_settings_id = account_settings.id
account_settings_account_usage = account_settings.account_usage
account_settings_account_limit = account_settings.account_limit
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple account_settings resources
account_settings_0 = provider.lambda.Account_settings {
}
account_settings_1 = provider.lambda.Account_settings {
}
account_settings_2 = provider.lambda.Account_settings {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    account_settings = provider.lambda.Account_settings {
    }
```

---

## Related Documentation

- [AWS Lambda Documentation](https://docs.aws.amazon.com/lambda/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
