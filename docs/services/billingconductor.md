# Billingconductor Service



**Resources**: 1

---

## Overview

The billingconductor service provides access to 1 resource type:

- [Billing_group_cost_report](#billing_group_cost_report) [R]

---

## Resources


### Billing_group_cost_report

BillingGroupCostReport resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The pagination token used on subsequent calls to get reports.</p> |
| `billing_group_cost_report_results` | Vec<String> | <p>The list of margin summary reports.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access billing_group_cost_report outputs
billing_group_cost_report_id = billing_group_cost_report.id
billing_group_cost_report_next_token = billing_group_cost_report.next_token
billing_group_cost_report_billing_group_cost_report_results = billing_group_cost_report.billing_group_cost_report_results
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple billing_group_cost_report resources
billing_group_cost_report_0 = provider.billingconductor.Billing_group_cost_report {
}
billing_group_cost_report_1 = provider.billingconductor.Billing_group_cost_report {
}
billing_group_cost_report_2 = provider.billingconductor.Billing_group_cost_report {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    billing_group_cost_report = provider.billingconductor.Billing_group_cost_report {
    }
```

---

## Related Documentation

- [AWS Billingconductor Documentation](https://docs.aws.amazon.com/billingconductor/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
