# License_manager_user_subscriptions Service



**Resources**: 2

---

## Overview

The license_manager_user_subscriptions service provides access to 2 resource types:

- [License_server_endpoint](#license_server_endpoint) [CD]
- [Identity_provider_settings](#identity_provider_settings) [U]

---

## Resources


### License_server_endpoint

LicenseServerEndpoint resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | HashMap<String, String> |  | <p>The tags that apply for the license server endpoint.</p> |
| `license_server_settings` | String | ✅ | <p>The <code>LicenseServerSettings</code> resource to create for the endpoint. The settings include the type of license server and the Secrets Manager secret that enables administrators to add or remove users associated with the license server.</p> |
| `identity_provider_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) that identifies the <code>IdentityProvider</code> resource that contains details about a registered identity provider. In the case of Active Directory, that can be a self-managed Active Directory or an Amazon Web Services Managed Active Directory that contains user identity details.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create license_server_endpoint
license_server_endpoint = provider.license_manager_user_subscriptions.License_server_endpoint {
    license_server_settings = "value"  # <p>The <code>LicenseServerSettings</code> resource to create for the endpoint. The settings include the type of license server and the Secrets Manager secret that enables administrators to add or remove users associated with the license server.</p>
    identity_provider_arn = "value"  # <p>The Amazon Resource Name (ARN) that identifies the <code>IdentityProvider</code> resource that contains details about a registered identity provider. In the case of Active Directory, that can be a self-managed Active Directory or an Amazon Web Services Managed Active Directory that contains user identity details.</p>
}

```

---


### Identity_provider_settings

IdentityProviderSettings resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `product` | String |  | <p>The name of the user-based subscription product.</p> <p>Valid values: <code>VISUAL_STUDIO_ENTERPRISE</code> | <code>VISUAL_STUDIO_PROFESSIONAL</code> | <code>OFFICE_PROFESSIONAL_PLUS</code> | <code>REMOTE_DESKTOP_SERVICES</code> </p> |
| `identity_provider_arn` | String |  | <p>The Amazon Resource Name (ARN) of the identity provider to update.</p> |
| `update_settings` | String | ✅ | <p>Updates the registered identity provider’s product related configuration settings. You can update any combination of settings in a single operation such as the:</p> <ul> <li> <p>Subnets which you want to add to provision VPC endpoints.</p> </li> <li> <p>Subnets which you want to remove the VPC endpoints from.</p> </li> <li> <p>Security group ID which permits traffic to the VPC endpoints.</p> </li> </ul> |
| `identity_provider` | String |  |  |



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



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple license_server_endpoint resources
license_server_endpoint_0 = provider.license_manager_user_subscriptions.License_server_endpoint {
    license_server_settings = "value-0"
    identity_provider_arn = "value-0"
}
license_server_endpoint_1 = provider.license_manager_user_subscriptions.License_server_endpoint {
    license_server_settings = "value-1"
    identity_provider_arn = "value-1"
}
license_server_endpoint_2 = provider.license_manager_user_subscriptions.License_server_endpoint {
    license_server_settings = "value-2"
    identity_provider_arn = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    license_server_endpoint = provider.license_manager_user_subscriptions.License_server_endpoint {
        license_server_settings = "production-value"
        identity_provider_arn = "production-value"
    }
```

---

## Related Documentation

- [AWS License_manager_user_subscriptions Documentation](https://docs.aws.amazon.com/license_manager_user_subscriptions/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
