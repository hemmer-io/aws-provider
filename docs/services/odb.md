# Odb Service



**Resources**: 1

---

## Overview

The odb service provides access to 1 resource type:

- [Oci_onboarding_status](#oci_onboarding_status) [R]

---

## Resources


### Oci_onboarding_status

OciOnboardingStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `existing_tenancy_activation_link` | String | <p>The existing OCI tenancy activation link for your Amazon Web Services account.</p> |
| `new_tenancy_activation_link` | String | <p>A new OCI tenancy activation link for your Amazon Web Services account.</p> |
| `status` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access oci_onboarding_status outputs
oci_onboarding_status_id = oci_onboarding_status.id
oci_onboarding_status_existing_tenancy_activation_link = oci_onboarding_status.existing_tenancy_activation_link
oci_onboarding_status_new_tenancy_activation_link = oci_onboarding_status.new_tenancy_activation_link
oci_onboarding_status_status = oci_onboarding_status.status
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple oci_onboarding_status resources
oci_onboarding_status_0 = provider.odb.Oci_onboarding_status {
}
oci_onboarding_status_1 = provider.odb.Oci_onboarding_status {
}
oci_onboarding_status_2 = provider.odb.Oci_onboarding_status {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    oci_onboarding_status = provider.odb.Oci_onboarding_status {
    }
```

---

## Related Documentation

- [AWS Odb Documentation](https://docs.aws.amazon.com/odb/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
