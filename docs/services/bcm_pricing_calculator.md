# Bcm_pricing_calculator Service



**Resources**: 1

---

## Overview

The bcm_pricing_calculator service provides access to 1 resource type:

- [Preferences](#preferences) [RU]

---

## Resources


### Preferences

Preferences resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `member_account_rate_type_selections` | Vec<String> |  | <p> The updated preferred rate types for member accounts. </p> |
| `standalone_account_rate_type_selections` | Vec<String> |  | <p> The updated preferred rate types for a standalone account. </p> |
| `management_account_rate_type_selections` | Vec<String> |  | <p> The updated preferred rate types for the management account. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `standalone_account_rate_type_selections` | Vec<String> | <p> The preferred rate types for a standalone account. </p> |
| `management_account_rate_type_selections` | Vec<String> | <p> The preferred rate types for the management account. </p> |
| `member_account_rate_type_selections` | Vec<String> | <p> The preferred rate types for member accounts. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access preferences outputs
preferences_id = preferences.id
preferences_standalone_account_rate_type_selections = preferences.standalone_account_rate_type_selections
preferences_management_account_rate_type_selections = preferences.management_account_rate_type_selections
preferences_member_account_rate_type_selections = preferences.member_account_rate_type_selections
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple preferences resources
preferences_0 = provider.bcm_pricing_calculator.Preferences {
}
preferences_1 = provider.bcm_pricing_calculator.Preferences {
}
preferences_2 = provider.bcm_pricing_calculator.Preferences {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    preferences = provider.bcm_pricing_calculator.Preferences {
    }
```

---

## Related Documentation

- [AWS Bcm_pricing_calculator Documentation](https://docs.aws.amazon.com/bcm_pricing_calculator/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
