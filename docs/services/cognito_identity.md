# Cognito_identity Service



**Resources**: 9

---

## Overview

The cognito_identity service provides access to 9 resource types:

- [Principal_tag_attribute_map](#principal_tag_attribute_map) [R]
- [Identities](#identities) [D]
- [Identity_pool](#identity_pool) [CRUD]
- [Credentials_for_identity](#credentials_for_identity) [R]
- [Identity_pool_roles](#identity_pool_roles) [R]
- [Identity](#identity) [R]
- [Id](#id) [R]
- [Open_id_token_for_developer_identity](#open_id_token_for_developer_identity) [R]
- [Open_id_token](#open_id_token) [R]

---

## Resources


### Principal_tag_attribute_map

PrincipalTagAttributeMap resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `use_defaults` | bool | <p>You can use this operation to list </p> |
| `principal_tags` | HashMap<String, String> | <p>You can use this operation to add principal tags. The
         <code>PrincipalTags</code>operation enables you to reference user attributes in your
            IAM permissions policy.</p> |
| `identity_provider_name` | String | <p>You can use this operation to get the provider name.</p> |
| `identity_pool_id` | String | <p>You can use this operation to get the ID of the Identity Pool you setup attribute
         mappings for.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access principal_tag_attribute_map outputs
principal_tag_attribute_map_id = principal_tag_attribute_map.id
principal_tag_attribute_map_use_defaults = principal_tag_attribute_map.use_defaults
principal_tag_attribute_map_principal_tags = principal_tag_attribute_map.principal_tags
principal_tag_attribute_map_identity_provider_name = principal_tag_attribute_map.identity_provider_name
principal_tag_attribute_map_identity_pool_id = principal_tag_attribute_map.identity_pool_id
```

---


### Identities

Identities resource

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


### Identity_pool

IdentityPool resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `allow_classic_flow` | bool |  | <p>Enables or disables the Basic (Classic) authentication flow. For more information, see
            <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/authentication-flow.html">Identity Pools (Federated Identities) Authentication Flow</a> in the
            <i>Amazon Cognito Developer Guide</i>.</p> |
| `allow_unauthenticated_identities` | bool | ✅ | <p>TRUE if the identity pool supports unauthenticated logins.</p> |
| `cognito_identity_providers` | Vec<String> |  | <p>An array of Amazon Cognito user pools and their client IDs.</p> |
| `supported_login_providers` | HashMap<String, String> |  | <p>Optional key:value pairs mapping provider names to provider app IDs.</p> |
| `developer_provider_name` | String |  | <p>The "domain" by which Cognito will refer to your users. This name acts as a
         placeholder that allows your backend and the Cognito service to communicate about the
         developer provider. For the <code>DeveloperProviderName</code>, you can use letters as well
         as period (<code>.</code>), underscore (<code>_</code>), and dash
         (<code>-</code>).</p>
         <p>Once you have set a developer provider name, you cannot change it. Please take care
         in setting this parameter.</p> |
| `identity_pool_name` | String | ✅ | <p>A string that you provide.</p> |
| `open_id_connect_provider_ar_ns` | Vec<String> |  | <p>The Amazon Resource Names (ARN) of the OpenID Connect providers.</p> |
| `saml_provider_ar_ns` | Vec<String> |  | <p>An array of Amazon Resource Names (ARNs) of the SAML provider for your identity
         pool.</p> |
| `identity_pool_tags` | HashMap<String, String> |  | <p>Tags to assign to the identity pool. A tag is a label that you can apply to identity
         pools to categorize and manage them in different ways, such as by purpose, owner,
         environment, or other criteria.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `identity_pool_tags` | HashMap<String, String> | <p>The tags that are assigned to the identity pool. A tag is a label that you can apply to
         identity pools to categorize and manage them in different ways, such as by purpose, owner,
         environment, or other criteria.</p> |
| `identity_pool_id` | String | <p>An identity pool ID in the format REGION:GUID.</p> |
| `developer_provider_name` | String | <p>The "domain" by which Cognito will refer to your users.</p> |
| `supported_login_providers` | HashMap<String, String> | <p>Optional key:value pairs mapping provider names to provider app IDs.</p> |
| `allow_classic_flow` | bool | <p>Enables or disables the Basic (Classic) authentication flow. For more information, see
            <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/authentication-flow.html">Identity Pools (Federated Identities) Authentication Flow</a> in the
            <i>Amazon Cognito Developer Guide</i>.</p> |
| `saml_provider_ar_ns` | Vec<String> | <p>An array of Amazon Resource Names (ARNs) of the SAML provider for your identity
         pool.</p> |
| `allow_unauthenticated_identities` | bool | <p>TRUE if the identity pool supports unauthenticated logins.</p> |
| `identity_pool_name` | String | <p>A string that you provide.</p> |
| `open_id_connect_provider_ar_ns` | Vec<String> | <p>The ARNs of the OpenID Connect providers.</p> |
| `cognito_identity_providers` | Vec<String> | <p>A list representing an Amazon Cognito user pool and its client ID.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create identity_pool
identity_pool = provider.cognito_identity.Identity_pool {
    allow_unauthenticated_identities = "value"  # <p>TRUE if the identity pool supports unauthenticated logins.</p>
    identity_pool_name = "value"  # <p>A string that you provide.</p>
}

# Access identity_pool outputs
identity_pool_id = identity_pool.id
identity_pool_identity_pool_tags = identity_pool.identity_pool_tags
identity_pool_identity_pool_id = identity_pool.identity_pool_id
identity_pool_developer_provider_name = identity_pool.developer_provider_name
identity_pool_supported_login_providers = identity_pool.supported_login_providers
identity_pool_allow_classic_flow = identity_pool.allow_classic_flow
identity_pool_saml_provider_ar_ns = identity_pool.saml_provider_ar_ns
identity_pool_allow_unauthenticated_identities = identity_pool.allow_unauthenticated_identities
identity_pool_identity_pool_name = identity_pool.identity_pool_name
identity_pool_open_id_connect_provider_ar_ns = identity_pool.open_id_connect_provider_ar_ns
identity_pool_cognito_identity_providers = identity_pool.cognito_identity_providers
```

---


### Credentials_for_identity

CredentialsForIdentity resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `credentials` | String | <p>Credentials for the provided identity ID.</p> |
| `identity_id` | String | <p>A unique identifier in the format REGION:GUID.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access credentials_for_identity outputs
credentials_for_identity_id = credentials_for_identity.id
credentials_for_identity_credentials = credentials_for_identity.credentials
credentials_for_identity_identity_id = credentials_for_identity.identity_id
```

---


### Identity_pool_roles

IdentityPoolRoles resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `identity_pool_id` | String | <p>An identity pool ID in the format REGION:GUID.</p> |
| `role_mappings` | HashMap<String, String> | <p>How users for a specific identity provider are to mapped to roles. This is a
            String-to-<a>RoleMapping</a> object map. The string identifies the identity
         provider, for example, <code>graph.facebook.com</code> or
            <code>cognito-idp.us-east-1.amazonaws.com/us-east-1_abcdefghi:app_client_id</code>.</p> |
| `roles` | HashMap<String, String> | <p>The map of roles associated with this pool. Currently only authenticated and
         unauthenticated roles are supported.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access identity_pool_roles outputs
identity_pool_roles_id = identity_pool_roles.id
identity_pool_roles_identity_pool_id = identity_pool_roles.identity_pool_id
identity_pool_roles_role_mappings = identity_pool_roles.role_mappings
identity_pool_roles_roles = identity_pool_roles.roles
```

---


### Identity

Identity resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `creation_date` | String | <p>Date on which the identity was created.</p> |
| `identity_id` | String | <p>A unique identifier in the format REGION:GUID.</p> |
| `logins` | Vec<String> | <p>The provider names.</p> |
| `last_modified_date` | String | <p>Date on which the identity was last modified.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access identity outputs
identity_id = identity.id
identity_creation_date = identity.creation_date
identity_identity_id = identity.identity_id
identity_logins = identity.logins
identity_last_modified_date = identity.last_modified_date
```

---


### Id

Id resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `identity_id` | String | <p>A unique identifier in the format REGION:GUID.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access id outputs
id_id = id.id
id_identity_id = id.identity_id
```

---


### Open_id_token_for_developer_identity

OpenIdTokenForDeveloperIdentity resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `token` | String | <p>An OpenID token.</p> |
| `identity_id` | String | <p>A unique identifier in the format REGION:GUID.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access open_id_token_for_developer_identity outputs
open_id_token_for_developer_identity_id = open_id_token_for_developer_identity.id
open_id_token_for_developer_identity_token = open_id_token_for_developer_identity.token
open_id_token_for_developer_identity_identity_id = open_id_token_for_developer_identity.identity_id
```

---


### Open_id_token

OpenIdToken resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `identity_id` | String | <p>A unique identifier in the format REGION:GUID. Note that the IdentityId returned may
         not match the one passed on input.</p> |
| `token` | String | <p>An OpenID token, valid for 10 minutes.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access open_id_token outputs
open_id_token_id = open_id_token.id
open_id_token_identity_id = open_id_token.identity_id
open_id_token_token = open_id_token.token
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple principal_tag_attribute_map resources
principal_tag_attribute_map_0 = provider.cognito_identity.Principal_tag_attribute_map {
}
principal_tag_attribute_map_1 = provider.cognito_identity.Principal_tag_attribute_map {
}
principal_tag_attribute_map_2 = provider.cognito_identity.Principal_tag_attribute_map {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    principal_tag_attribute_map = provider.cognito_identity.Principal_tag_attribute_map {
    }
```

---

## Related Documentation

- [AWS Cognito_identity Documentation](https://docs.aws.amazon.com/cognito_identity/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
