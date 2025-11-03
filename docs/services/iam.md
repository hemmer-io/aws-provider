# Iam Service



**Resources**: 37

---

## Overview

The iam service provides access to 37 resource types:

- [Role_permissions_boundary](#role_permissions_boundary) [CD]
- [Service_specific_credential](#service_specific_credential) [CUD]
- [Server_certificate](#server_certificate) [RUD]
- [Role_description](#role_description) [U]
- [Policy](#policy) [CRD]
- [Instance_profile](#instance_profile) [CRD]
- [Account_summary](#account_summary) [R]
- [Mfa_device](#mfa_device) [R]
- [Open_id_connect_provider](#open_id_connect_provider) [CRD]
- [Account_alias](#account_alias) [CD]
- [Role](#role) [CRUD]
- [User](#user) [CRUD]
- [Signing_certificate](#signing_certificate) [UD]
- [Organizations_access_report](#organizations_access_report) [R]
- [User_permissions_boundary](#user_permissions_boundary) [CD]
- [Login_profile](#login_profile) [CRUD]
- [Group_policy](#group_policy) [CRD]
- [Context_keys_for_custom_policy](#context_keys_for_custom_policy) [R]
- [Open_id_connect_provider_thumbprint](#open_id_connect_provider_thumbprint) [U]
- [User_policy](#user_policy) [CRD]
- [Account_password_policy](#account_password_policy) [RUD]
- [Credential_report](#credential_report) [R]
- [Service_last_accessed_details_with_entities](#service_last_accessed_details_with_entities) [R]
- [Service_linked_role_deletion_status](#service_linked_role_deletion_status) [R]
- [Policy_version](#policy_version) [CRD]
- [Virtual_mfa_device](#virtual_mfa_device) [CD]
- [Assume_role_policy](#assume_role_policy) [U]
- [Saml_provider](#saml_provider) [CRUD]
- [Context_keys_for_principal_policy](#context_keys_for_principal_policy) [R]
- [Service_last_accessed_details](#service_last_accessed_details) [R]
- [Role_policy](#role_policy) [CRD]
- [Ssh_public_key](#ssh_public_key) [RUD]
- [Account_authorization_details](#account_authorization_details) [R]
- [Service_linked_role](#service_linked_role) [CD]
- [Group](#group) [CRUD]
- [Access_key](#access_key) [CUD]
- [Access_key_last_used](#access_key_last_used) [R]

---

## Resources


### Role_permissions_boundary

RolePermissionsBoundary resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `permissions_boundary` | String | ✅ | <p>The ARN of the managed policy that is used to set the permissions boundary for the
            role.</p>
         <p>A permissions boundary policy defines the maximum permissions that identity-based
            policies can grant to an entity, but does not grant permissions. Permissions boundaries
            do not define the maximum permissions that a resource-based policy can grant to an
            entity. To learn more, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies_boundaries.html">Permissions boundaries
                for IAM entities</a> in the <i>IAM User Guide</i>.</p>
         <p>For more information about policy types, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies.html#access_policy-types">Policy types
            </a> in the <i>IAM User Guide</i>.</p> |
| `role_name` | String | ✅ | <p>The name (friendly name, not ARN) of the IAM role for which you want to set the
            permissions boundary.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create role_permissions_boundary
role_permissions_boundary = provider.iam.Role_permissions_boundary {
    permissions_boundary = "value"  # <p>The ARN of the managed policy that is used to set the permissions boundary for the
            role.</p>
         <p>A permissions boundary policy defines the maximum permissions that identity-based
            policies can grant to an entity, but does not grant permissions. Permissions boundaries
            do not define the maximum permissions that a resource-based policy can grant to an
            entity. To learn more, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies_boundaries.html">Permissions boundaries
                for IAM entities</a> in the <i>IAM User Guide</i>.</p>
         <p>For more information about policy types, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies.html#access_policy-types">Policy types
            </a> in the <i>IAM User Guide</i>.</p>
    role_name = "value"  # <p>The name (friendly name, not ARN) of the IAM role for which you want to set the
            permissions boundary.</p>
}

```

---


### Service_specific_credential

ServiceSpecificCredential resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `service_name` | String | ✅ | <p>The name of the Amazon Web Services service that is to be associated with the credentials. The
            service you specify here is the only service that can be accessed using these
            credentials.</p> |
| `user_name` | String | ✅ | <p>The name of the IAM user that is to be associated with the credentials. The new
            service-specific credentials have the same permissions as the associated user except
            that they can be used only to access the specified service.</p>
         <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric 
    characters with no spaces. You can also include any of the following characters: _+=,.@-</p> |
| `credential_age_days` | i64 |  | <p>The number of days until the service specific credential expires. This field is only
            valid for Bedrock API keys and must be a positive integer. When not specified, the
            credential will not expire.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create service_specific_credential
service_specific_credential = provider.iam.Service_specific_credential {
    service_name = "value"  # <p>The name of the Amazon Web Services service that is to be associated with the credentials. The
            service you specify here is the only service that can be accessed using these
            credentials.</p>
    user_name = "value"  # <p>The name of the IAM user that is to be associated with the credentials. The new
            service-specific credentials have the same permissions as the associated user except
            that they can be used only to access the specified service.</p>
         <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric 
    characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
}

```

---


### Server_certificate

ServerCertificate resource

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `server_certificate_name` | String | ✅ | <p>The name of the server certificate that you want to update.</p>
         <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric 
    characters with no spaces. You can also include any of the following characters: _+=,.@-</p> |
| `new_path` | String |  | <p>The new path for the server certificate. Include this only if you are updating the
            server certificate's path.</p>
         <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting 
    of either a forward slash (/) by itself or a string that must begin and end with forward slashes.
    In addition, it can contain any ASCII character from the ! (<code>\u0021</code>) through the DEL character (<code>\u007F</code>), including 
    most punctuation characters, digits, and upper and lowercased letters.</p> |
| `new_server_certificate_name` | String |  | <p>The new name for the server certificate. Include this only if you are updating the
            server certificate's name. The name of the certificate cannot contain any spaces.</p>
         <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric 
    characters with no spaces. You can also include any of the following characters: _+=,.@-</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `server_certificate` | String | <p>A structure containing details about the server certificate.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access server_certificate outputs
server_certificate_id = server_certificate.id
server_certificate_server_certificate = server_certificate.server_certificate
```

---


### Role_description

RoleDescription resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `role_name` | String | ✅ | <p>The name of the role that you want to modify.</p> |
| `description` | String | ✅ | <p>The new description that you want to apply to the specified role.</p> |



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


### Policy

Policy resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `path` | String |  | <p>The path for the policy.</p>
         <p>For more information about paths, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html">IAM identifiers</a> in the
                <i>IAM User Guide</i>.</p>
         <p>This parameter is optional. If it is not included, it defaults to a slash (/).</p>
         <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting 
    of either a forward slash (/) by itself or a string that must begin and end with forward slashes.
    In addition, it can contain any ASCII character from the ! (<code>\u0021</code>) through the DEL character (<code>\u007F</code>), including 
    most punctuation characters, digits, and upper and lowercased letters.</p>
         <note>
            <p>You cannot use an asterisk (*) in the path name.</p>
         </note> |
| `policy_document` | String | ✅ | <p>The JSON policy document that you want to use as the content for the new
            policy.</p>
         <p>You must provide policies in JSON format in IAM. However, for CloudFormation
            templates formatted in YAML, you can provide the policy in JSON or YAML format. CloudFormation always converts a YAML policy to JSON format before submitting it to
            IAM.</p>
         <p>The maximum length of the policy document that you can pass in this operation,
            including whitespace, is listed below. To view the maximum character counts of a managed policy with no whitespaces, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_iam-quotas.html#reference_iam-quotas-entity-length">IAM and STS character quotas</a>.</p>
         <p>To learn more about JSON policy grammar, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_grammar.html">Grammar of the IAM JSON
                policy language</a> in the <i>IAM User Guide</i>. </p>
         <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> 
    used to validate this parameter is a string of characters consisting of the following:</p>
         <ul>
            <li>
               <p>Any printable ASCII 
    character ranging from the space character (<code>\u0020</code>) through the end of the ASCII character range</p>
            </li>
            <li>
               <p>The printable characters in the Basic Latin and  Latin-1 Supplement character set 
    (through <code>\u00FF</code>)</p>
            </li>
            <li>
               <p>The special characters tab (<code>\u0009</code>), line feed (<code>\u000A</code>), and 
    carriage return (<code>\u000D</code>)</p>
            </li>
         </ul> |
| `policy_name` | String | ✅ | <p>The friendly name of the policy.</p>
         <p>IAM user, group, role, and policy names must be unique within the account. Names are
            not distinguished by case. For example, you cannot create resources named both
            "MyResource" and "myresource".</p> |
| `description` | String |  | <p>A friendly description of the policy.</p>
         <p>Typically used to store information about the permissions defined in the policy. For
            example, "Grants access to production DynamoDB tables."</p>
         <p>The policy description is immutable. After a value is assigned, it cannot be
            changed.</p> |
| `tags` | Vec<String> |  | <p>A list of tags that you want to attach to the new IAM customer managed policy.
      Each tag consists of a key name and an associated value. For more information about tagging, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_tags.html">Tagging IAM resources</a> in the
      <i>IAM User Guide</i>.</p>
         <note>
            <p>If any one of the tags is invalid or if you exceed the allowed maximum number of tags, then the entire request 
   fails and the resource is not created.</p>
         </note> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `policy` | String | <p>A structure containing details about the policy.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create policy
policy = provider.iam.Policy {
    policy_document = "value"  # <p>The JSON policy document that you want to use as the content for the new
            policy.</p>
         <p>You must provide policies in JSON format in IAM. However, for CloudFormation
            templates formatted in YAML, you can provide the policy in JSON or YAML format. CloudFormation always converts a YAML policy to JSON format before submitting it to
            IAM.</p>
         <p>The maximum length of the policy document that you can pass in this operation,
            including whitespace, is listed below. To view the maximum character counts of a managed policy with no whitespaces, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_iam-quotas.html#reference_iam-quotas-entity-length">IAM and STS character quotas</a>.</p>
         <p>To learn more about JSON policy grammar, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_grammar.html">Grammar of the IAM JSON
                policy language</a> in the <i>IAM User Guide</i>. </p>
         <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> 
    used to validate this parameter is a string of characters consisting of the following:</p>
         <ul>
            <li>
               <p>Any printable ASCII 
    character ranging from the space character (<code>\u0020</code>) through the end of the ASCII character range</p>
            </li>
            <li>
               <p>The printable characters in the Basic Latin and  Latin-1 Supplement character set 
    (through <code>\u00FF</code>)</p>
            </li>
            <li>
               <p>The special characters tab (<code>\u0009</code>), line feed (<code>\u000A</code>), and 
    carriage return (<code>\u000D</code>)</p>
            </li>
         </ul>
    policy_name = "value"  # <p>The friendly name of the policy.</p>
         <p>IAM user, group, role, and policy names must be unique within the account. Names are
            not distinguished by case. For example, you cannot create resources named both
            "MyResource" and "myresource".</p>
}

# Access policy outputs
policy_id = policy.id
policy_policy = policy.policy
```

---


### Instance_profile

InstanceProfile resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `instance_profile_name` | String | ✅ | <p>The name of the instance profile to create.</p>
         <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric 
    characters with no spaces. You can also include any of the following characters: _+=,.@-</p> |
| `path` | String |  | <p> The path to the instance profile. For more information about paths, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html">IAM
                Identifiers</a> in the <i>IAM User Guide</i>.</p>
         <p>This parameter is optional. If it is not included, it defaults to a slash (/).</p>
         <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting 
    of either a forward slash (/) by itself or a string that must begin and end with forward slashes.
    In addition, it can contain any ASCII character from the ! (<code>\u0021</code>) through the DEL character (<code>\u007F</code>), including 
    most punctuation characters, digits, and upper and lowercased letters.</p> |
| `tags` | Vec<String> |  | <p>A list of tags that you want to attach to the newly created IAM instance profile.
      Each tag consists of a key name and an associated value. For more information about tagging, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_tags.html">Tagging IAM resources</a> in the
      <i>IAM User Guide</i>.</p>
         <note>
            <p>If any one of the tags is invalid or if you exceed the allowed maximum number of tags, then the entire request 
   fails and the resource is not created.</p>
         </note> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `instance_profile` | String | <p>A structure containing details about the instance profile.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create instance_profile
instance_profile = provider.iam.Instance_profile {
    instance_profile_name = "value"  # <p>The name of the instance profile to create.</p>
         <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric 
    characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
}

# Access instance_profile outputs
instance_profile_id = instance_profile.id
instance_profile_instance_profile = instance_profile.instance_profile
```

---


### Account_summary

AccountSummary resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `summary_map` | HashMap<String, i64> | <p>A set of key–value pairs containing information about IAM entity usage and
            IAM quotas.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access account_summary outputs
account_summary_id = account_summary.id
account_summary_summary_map = account_summary.summary_map
```

---


### Mfa_device

MFADevice resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `enable_date` | String | <p>The date that a specified user's MFA device was first enabled.</p> |
| `user_name` | String | <p>The friendly name identifying the user.</p> |
| `serial_number` | String | <p>Serial number that uniquely identifies the MFA device. For this API, we only accept
            FIDO security key <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference-arns.html">ARNs</a>.</p> |
| `certifications` | HashMap<String, String> | <p>The certifications of a specified user's MFA device. We currently provide FIPS-140-2,
            FIPS-140-3, and FIDO certification levels obtained from <a href="https://fidoalliance.org/metadata/"> FIDO Alliance Metadata Service
                (MDS)</a>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access mfa_device outputs
mfa_device_id = mfa_device.id
mfa_device_enable_date = mfa_device.enable_date
mfa_device_user_name = mfa_device.user_name
mfa_device_serial_number = mfa_device.serial_number
mfa_device_certifications = mfa_device.certifications
```

---


### Open_id_connect_provider

OpenIDConnectProvider resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `url` | String | ✅ | <p>The URL of the identity provider. The URL must begin with <code>https://</code> and
            should correspond to the <code>iss</code> claim in the provider's OpenID Connect ID
            tokens. Per the OIDC standard, path components are allowed but query parameters are not.
            Typically the URL consists of only a hostname, like
                <code>https://server.example.org</code> or <code>https://example.com</code>. The URL
            should not contain a port number. </p>
         <p>You cannot register the same provider multiple times in a single Amazon Web Services account. If you
            try to submit a URL that has already been used for an OpenID Connect provider in the
            Amazon Web Services account, you will get an error.</p> |
| `client_id_list` | Vec<String> |  | <p>Provides a list of client IDs, also known as audiences. When a mobile or web app
            registers with an OpenID Connect provider, they establish a value that identifies the
            application. This is the value that's sent as the <code>client_id</code> parameter on
            OAuth requests.</p>
         <p>You can register multiple client IDs with the same provider. For example, you might
            have multiple applications that use the same OIDC provider. You cannot register more
            than 100 client IDs with a single IAM OIDC provider.</p>
         <p>There is no defined format for a client ID. The
                <code>CreateOpenIDConnectProviderRequest</code> operation accepts client IDs up to
            255 characters long.</p> |
| `thumbprint_list` | Vec<String> |  | <p>A list of server certificate thumbprints for the OpenID Connect (OIDC) identity
            provider's server certificates. Typically this list includes only one entry. However,
            IAM lets you have up to five thumbprints for an OIDC provider. This lets you maintain
            multiple thumbprints if the identity provider is rotating certificates.</p>
         <p>This parameter is optional. If it is not included, IAM will retrieve and use the top
            intermediate certificate authority (CA) thumbprint of the OpenID Connect identity
            provider server certificate.</p>
         <p>The server certificate thumbprint is the hex-encoded SHA-1 hash value of the X.509
            certificate used by the domain where the OpenID Connect provider makes its keys
            available. It is always a 40-character string.</p>
         <p>For example, assume that the OIDC provider is <code>server.example.com</code> and the
            provider stores its keys at https://keys.server.example.com/openid-connect. In that
            case, the thumbprint string would be the hex-encoded SHA-1 hash value of the certificate
            used by <code>https://keys.server.example.com.</code>
         </p>
         <p>For more information about obtaining the OIDC provider thumbprint, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/identity-providers-oidc-obtain-thumbprint.html">Obtaining the
                thumbprint for an OpenID Connect provider</a> in the <i>IAM user
                Guide</i>.</p> |
| `tags` | Vec<String> |  | <p>A list of tags that you want to attach to the new IAM OpenID Connect (OIDC) provider.
      Each tag consists of a key name and an associated value. For more information about tagging, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_tags.html">Tagging IAM resources</a> in the
      <i>IAM User Guide</i>.</p>
         <note>
            <p>If any one of the tags is invalid or if you exceed the allowed maximum number of tags, then the entire request 
   fails and the resource is not created.</p>
         </note> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `client_id_list` | Vec<String> | <p>A list of client IDs (also known as audiences) that are associated with the specified
            IAM OIDC provider resource object. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/APIReference/API_CreateOpenIDConnectProvider.html">CreateOpenIDConnectProvider</a>.</p> |
| `tags` | Vec<String> | <p>A list of tags that are attached to the specified IAM OIDC provider. The returned list of tags is sorted by tag key.
      For more information about tagging, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_tags.html">Tagging IAM resources</a> in the
      <i>IAM User Guide</i>.</p> |
| `url` | String | <p>The URL that the IAM OIDC provider resource object is associated with. For more
            information, see <a href="https://docs.aws.amazon.com/IAM/latest/APIReference/API_CreateOpenIDConnectProvider.html">CreateOpenIDConnectProvider</a>.</p> |
| `thumbprint_list` | Vec<String> | <p>A list of certificate thumbprints that are associated with the specified IAM OIDC
            provider resource object. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/APIReference/API_CreateOpenIDConnectProvider.html">CreateOpenIDConnectProvider</a>. </p> |
| `create_date` | String | <p>The date and time when the IAM OIDC provider resource object was created in the
            Amazon Web Services account.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create open_id_connect_provider
open_id_connect_provider = provider.iam.Open_id_connect_provider {
    url = "value"  # <p>The URL of the identity provider. The URL must begin with <code>https://</code> and
            should correspond to the <code>iss</code> claim in the provider's OpenID Connect ID
            tokens. Per the OIDC standard, path components are allowed but query parameters are not.
            Typically the URL consists of only a hostname, like
                <code>https://server.example.org</code> or <code>https://example.com</code>. The URL
            should not contain a port number. </p>
         <p>You cannot register the same provider multiple times in a single Amazon Web Services account. If you
            try to submit a URL that has already been used for an OpenID Connect provider in the
            Amazon Web Services account, you will get an error.</p>
}

# Access open_id_connect_provider outputs
open_id_connect_provider_id = open_id_connect_provider.id
open_id_connect_provider_client_id_list = open_id_connect_provider.client_id_list
open_id_connect_provider_tags = open_id_connect_provider.tags
open_id_connect_provider_url = open_id_connect_provider.url
open_id_connect_provider_thumbprint_list = open_id_connect_provider.thumbprint_list
open_id_connect_provider_create_date = open_id_connect_provider.create_date
```

---


### Account_alias

AccountAlias resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `account_alias` | String | ✅ | <p>The account alias to create.</p>
         <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of 
    lowercase letters, digits, and dashes. You cannot start or finish with a dash, nor can you have 
    two dashes in a row.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create account_alias
account_alias = provider.iam.Account_alias {
    account_alias = "value"  # <p>The account alias to create.</p>
         <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of 
    lowercase letters, digits, and dashes. You cannot start or finish with a dash, nor can you have 
    two dashes in a row.</p>
}

```

---


### Role

Role resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `role_name` | String | ✅ | <p>The name of the role to create.</p>
         <p>IAM user, group, role, and policy names must be unique within the account. Names are
            not distinguished by case. For example, you cannot create resources named both
            "MyResource" and "myresource".</p>
         <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric 
    characters with no spaces. You can also include any of the following characters: _+=,.@-</p> |
| `assume_role_policy_document` | String | ✅ | <p>The trust relationship policy document that grants an entity permission to assume the
            role.</p>
         <p>In IAM, you must provide a JSON policy that has been converted to a string. However,
            for CloudFormation templates formatted in YAML, you can provide the policy in JSON or YAML
            format. CloudFormation always converts a YAML policy to JSON format before submitting it to
            IAM.</p>
         <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> 
    used to validate this parameter is a string of characters consisting of the following:</p>
         <ul>
            <li>
               <p>Any printable ASCII 
    character ranging from the space character (<code>\u0020</code>) through the end of the ASCII character range</p>
            </li>
            <li>
               <p>The printable characters in the Basic Latin and  Latin-1 Supplement character set 
    (through <code>\u00FF</code>)</p>
            </li>
            <li>
               <p>The special characters tab (<code>\u0009</code>), line feed (<code>\u000A</code>), and 
    carriage return (<code>\u000D</code>)</p>
            </li>
         </ul>
         <p> Upon success, the response includes the same trust policy in JSON format.</p> |
| `max_session_duration` | i64 |  | <p>The maximum session duration (in seconds) that you want to set for the specified role.
            If you do not specify a value for this setting, the default value of one hour is
            applied. This setting can have a value from 1 hour to 12 hours.</p>
         <p>Anyone who assumes the role from the CLI or API can use the
                <code>DurationSeconds</code> API parameter or the <code>duration-seconds</code>
            CLI parameter to request a longer session. The <code>MaxSessionDuration</code> setting
            determines the maximum duration that can be requested using the
                <code>DurationSeconds</code> parameter. If users don't specify a value for the
                <code>DurationSeconds</code> parameter, their security credentials are valid for one
            hour by default. This applies when you use the <code>AssumeRole*</code> API operations
            or the <code>assume-role*</code> CLI operations but does not apply when you use those
            operations to create a console URL. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_use.html">Using IAM
                roles</a> in the <i>IAM User Guide</i>.</p> |
| `tags` | Vec<String> |  | <p>A list of tags that you want to attach to the new role. Each tag consists of a key name and an associated value.
      For more information about tagging, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_tags.html">Tagging IAM resources</a> in the
      <i>IAM User Guide</i>.</p>
         <note>
            <p>If any one of the tags is invalid or if you exceed the allowed maximum number of tags, then the entire request 
   fails and the resource is not created.</p>
         </note> |
| `path` | String |  | <p> The path to the role. For more information about paths, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html">IAM
                Identifiers</a> in the <i>IAM User Guide</i>.</p>
         <p>This parameter is optional. If it is not included, it defaults to a slash (/).</p>
         <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting 
    of either a forward slash (/) by itself or a string that must begin and end with forward slashes.
    In addition, it can contain any ASCII character from the ! (<code>\u0021</code>) through the DEL character (<code>\u007F</code>), including 
    most punctuation characters, digits, and upper and lowercased letters.</p> |
| `permissions_boundary` | String |  | <p>The ARN of the managed policy that is used to set the permissions boundary for the
            role.</p>
         <p>A permissions boundary policy defines the maximum permissions that identity-based
            policies can grant to an entity, but does not grant permissions. Permissions boundaries
            do not define the maximum permissions that a resource-based policy can grant to an
            entity. To learn more, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies_boundaries.html">Permissions boundaries
                for IAM entities</a> in the <i>IAM User Guide</i>.</p>
         <p>For more information about policy types, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies.html#access_policy-types">Policy types
            </a> in the <i>IAM User Guide</i>.</p> |
| `description` | String |  | <p>A description of the role.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `role` | String | <p>A structure containing details about the IAM role.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create role
role = provider.iam.Role {
    role_name = "value"  # <p>The name of the role to create.</p>
         <p>IAM user, group, role, and policy names must be unique within the account. Names are
            not distinguished by case. For example, you cannot create resources named both
            "MyResource" and "myresource".</p>
         <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric 
    characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    assume_role_policy_document = "value"  # <p>The trust relationship policy document that grants an entity permission to assume the
            role.</p>
         <p>In IAM, you must provide a JSON policy that has been converted to a string. However,
            for CloudFormation templates formatted in YAML, you can provide the policy in JSON or YAML
            format. CloudFormation always converts a YAML policy to JSON format before submitting it to
            IAM.</p>
         <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> 
    used to validate this parameter is a string of characters consisting of the following:</p>
         <ul>
            <li>
               <p>Any printable ASCII 
    character ranging from the space character (<code>\u0020</code>) through the end of the ASCII character range</p>
            </li>
            <li>
               <p>The printable characters in the Basic Latin and  Latin-1 Supplement character set 
    (through <code>\u00FF</code>)</p>
            </li>
            <li>
               <p>The special characters tab (<code>\u0009</code>), line feed (<code>\u000A</code>), and 
    carriage return (<code>\u000D</code>)</p>
            </li>
         </ul>
         <p> Upon success, the response includes the same trust policy in JSON format.</p>
}

# Access role outputs
role_id = role.id
role_role = role.role
```

---


### User

User resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `permissions_boundary` | String |  | <p>The ARN of the managed policy that is used to set the permissions boundary for the
            user.</p>
         <p>A permissions boundary policy defines the maximum permissions that identity-based
            policies can grant to an entity, but does not grant permissions. Permissions boundaries
            do not define the maximum permissions that a resource-based policy can grant to an
            entity. To learn more, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies_boundaries.html">Permissions boundaries
                for IAM entities</a> in the <i>IAM User Guide</i>.</p>
         <p>For more information about policy types, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies.html#access_policy-types">Policy types
            </a> in the <i>IAM User Guide</i>.</p> |
| `user_name` | String | ✅ | <p>The name of the user to create.</p>
         <p>IAM user, group, role, and policy names must be unique within the account. Names are
            not distinguished by case. For example, you cannot create resources named both
            "MyResource" and "myresource".</p> |
| `tags` | Vec<String> |  | <p>A list of tags that you want to attach to the new user. Each tag consists of a key name and an associated value.
      For more information about tagging, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_tags.html">Tagging IAM resources</a> in the
      <i>IAM User Guide</i>.</p>
         <note>
            <p>If any one of the tags is invalid or if you exceed the allowed maximum number of tags, then the entire request 
   fails and the resource is not created.</p>
         </note> |
| `path` | String |  | <p> The path for the user name. For more information about paths, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html">IAM
                identifiers</a> in the <i>IAM User Guide</i>.</p>
         <p>This parameter is optional. If it is not included, it defaults to a slash (/).</p>
         <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting 
    of either a forward slash (/) by itself or a string that must begin and end with forward slashes.
    In addition, it can contain any ASCII character from the ! (<code>\u0021</code>) through the DEL character (<code>\u007F</code>), including 
    most punctuation characters, digits, and upper and lowercased letters.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `user` | String | <p>A structure containing details about the IAM user.</p>
         <important>
            <p>Due to a service issue, password last used data does not include password use from
                May 3, 2018 22:50 PDT to May 23, 2018 14:08 PDT. This affects <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_finding-unused.html">last sign-in</a> dates shown in the IAM console and password last used
                dates in the <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_getting-report.html">IAM credential
                    report</a>, and returned by this operation. If users signed in during the
                affected time, the password last used date that is returned is the date the user
                last signed in before May 3, 2018. For users that signed in after May 23, 2018 14:08
                PDT, the returned password last used date is accurate.</p>
            <p>You can use password last used information to identify unused credentials for
                deletion. For example, you might delete users who did not sign in to Amazon Web Services in the
                last 90 days. In cases like this, we recommend that you adjust your evaluation
                window to include dates after May 23, 2018. Alternatively, if your users use access
                keys to access Amazon Web Services programmatically you can refer to access key last used
                information because it is accurate for all dates. </p>
         </important> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create user
user = provider.iam.User {
    user_name = "value"  # <p>The name of the user to create.</p>
         <p>IAM user, group, role, and policy names must be unique within the account. Names are
            not distinguished by case. For example, you cannot create resources named both
            "MyResource" and "myresource".</p>
}

# Access user outputs
user_id = user.id
user_user = user.user
```

---


### Signing_certificate

SigningCertificate resource

**Operations**: ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `status` | String | ✅ | <p> The status you want to assign to the certificate. <code>Active</code> means that the
            certificate can be used for programmatic calls to Amazon Web Services <code>Inactive</code> means that
            the certificate cannot be used.</p> |
| `certificate_id` | String | ✅ | <p>The ID of the signing certificate you want to update.</p>
         <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters that can 
    consist of any upper or lowercased letter or digit.</p> |
| `user_name` | String |  | <p>The name of the IAM user the signing certificate belongs to.</p>
         <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric 
    characters with no spaces. You can also include any of the following characters: _+=,.@-</p> |



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


### Organizations_access_report

OrganizationsAccessReport resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `job_creation_date` | String | <p>The date and time, in <a href="http://www.iso.org/iso/iso8601">ISO 8601 date-time
                format</a>, when the report job was created.</p> |
| `job_status` | String | <p>The status of the job.</p> |
| `number_of_services_not_accessed` | i64 | <p>The number of services that account principals are allowed but did not attempt to
            access.</p> |
| `is_truncated` | bool | <p>A flag that indicates whether there are more items to return. If your 
    results were truncated, you can make a subsequent pagination request using the <code>Marker</code>
    request parameter to retrieve more items. Note that IAM might return fewer than the 
    <code>MaxItems</code> number of results even when there are more results available. We recommend 
    that you check <code>IsTruncated</code> after every call to ensure that you receive all your 
    results.</p> |
| `marker` | String | <p>When <code>IsTruncated</code> is <code>true</code>, this element
    is present and contains the value to use for the <code>Marker</code> parameter in a subsequent 
    pagination request.</p> |
| `number_of_services_accessible` | i64 | <p>The number of services that the applicable SCPs allow account principals to
            access.</p> |
| `error_details` | String |  |
| `access_details` | Vec<String> | <p>An object that contains details about the most recent attempt to access the
            service.</p> |
| `job_completion_date` | String | <p>The date and time, in <a href="http://www.iso.org/iso/iso8601">ISO 8601 date-time
                format</a>, when the generated report job was completed or failed.</p>
         <p>This field is null if the job is still in progress, as indicated by a job status value
            of <code>IN_PROGRESS</code>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access organizations_access_report outputs
organizations_access_report_id = organizations_access_report.id
organizations_access_report_job_creation_date = organizations_access_report.job_creation_date
organizations_access_report_job_status = organizations_access_report.job_status
organizations_access_report_number_of_services_not_accessed = organizations_access_report.number_of_services_not_accessed
organizations_access_report_is_truncated = organizations_access_report.is_truncated
organizations_access_report_marker = organizations_access_report.marker
organizations_access_report_number_of_services_accessible = organizations_access_report.number_of_services_accessible
organizations_access_report_error_details = organizations_access_report.error_details
organizations_access_report_access_details = organizations_access_report.access_details
organizations_access_report_job_completion_date = organizations_access_report.job_completion_date
```

---


### User_permissions_boundary

UserPermissionsBoundary resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `user_name` | String | ✅ | <p>The name (friendly name, not ARN) of the IAM user for which you want to set the
            permissions boundary.</p> |
| `permissions_boundary` | String | ✅ | <p>The ARN of the managed policy that is used to set the permissions boundary for the
            user.</p>
         <p>A permissions boundary policy defines the maximum permissions that identity-based
            policies can grant to an entity, but does not grant permissions. Permissions boundaries
            do not define the maximum permissions that a resource-based policy can grant to an
            entity. To learn more, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies_boundaries.html">Permissions boundaries
                for IAM entities</a> in the <i>IAM User Guide</i>.</p>
         <p>For more information about policy types, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies.html#access_policy-types">Policy types
            </a> in the <i>IAM User Guide</i>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create user_permissions_boundary
user_permissions_boundary = provider.iam.User_permissions_boundary {
    user_name = "value"  # <p>The name (friendly name, not ARN) of the IAM user for which you want to set the
            permissions boundary.</p>
    permissions_boundary = "value"  # <p>The ARN of the managed policy that is used to set the permissions boundary for the
            user.</p>
         <p>A permissions boundary policy defines the maximum permissions that identity-based
            policies can grant to an entity, but does not grant permissions. Permissions boundaries
            do not define the maximum permissions that a resource-based policy can grant to an
            entity. To learn more, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies_boundaries.html">Permissions boundaries
                for IAM entities</a> in the <i>IAM User Guide</i>.</p>
         <p>For more information about policy types, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies.html#access_policy-types">Policy types
            </a> in the <i>IAM User Guide</i>.</p>
}

```

---


### Login_profile

LoginProfile resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `password` | String |  | <p>The new password for the user.</p>
         <p>This parameter must be omitted when you make the request with an <a href="https://docs.aws.amazon.com/STS/latest/APIReference/API_AssumeRoot.html">AssumeRoot</a> session. It is required in all other cases.</p>
         <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> 
    that is used to validate this parameter is a string of characters. That string can include almost any printable 
    ASCII character from the space (<code>\u0020</code>) through the end of the ASCII character range (<code>\u00FF</code>). 
    You can also include the tab (<code>\u0009</code>), line feed (<code>\u000A</code>), and carriage return (<code>\u000D</code>) 
    characters. Any of these characters are valid in a password. However, many tools, such 
    as the Amazon Web Services Management Console, might restrict the ability to type certain characters because they have 
    special meaning within that tool.</p> |
| `user_name` | String |  | <p>The name of the IAM user to create a password for. The user must already
            exist.</p>
         <p>This parameter is optional. If no user name is included, it defaults to the principal
            making the request. When you make this request with root user credentials, you must use
            an <a href="https://docs.aws.amazon.com/STS/latest/APIReference/API_AssumeRoot.html">AssumeRoot</a> session to omit the user name.</p>
         <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric 
    characters with no spaces. You can also include any of the following characters: _+=,.@-</p> |
| `password_reset_required` | bool |  | <p>Specifies whether the user is required to set a new password on next sign-in.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `login_profile` | String | <p>A structure containing the user name and the profile creation date for the
            user.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create login_profile
login_profile = provider.iam.Login_profile {
}

# Access login_profile outputs
login_profile_id = login_profile.id
login_profile_login_profile = login_profile.login_profile
```

---


### Group_policy

GroupPolicy resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `group_name` | String | ✅ | <p>The name of the group to associate the policy with.</p>
         <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric 
    characters with no spaces. You can also include any of the following characters: _+=,.@-.</p> |
| `policy_document` | String | ✅ | <p>The policy document.</p>
         <p>You must provide policies in JSON format in IAM. However, for CloudFormation templates
            formatted in YAML, you can provide the policy in JSON or YAML format. CloudFormation always
            converts a YAML policy to JSON format before submitting it to IAM.</p>
         <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> 
    used to validate this parameter is a string of characters consisting of the following:</p>
         <ul>
            <li>
               <p>Any printable ASCII 
    character ranging from the space character (<code>\u0020</code>) through the end of the ASCII character range</p>
            </li>
            <li>
               <p>The printable characters in the Basic Latin and  Latin-1 Supplement character set 
    (through <code>\u00FF</code>)</p>
            </li>
            <li>
               <p>The special characters tab (<code>\u0009</code>), line feed (<code>\u000A</code>), and 
    carriage return (<code>\u000D</code>)</p>
            </li>
         </ul> |
| `policy_name` | String | ✅ | <p>The name of the policy document.</p>
         <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric 
    characters with no spaces. You can also include any of the following characters: _+=,.@-</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `policy_name` | String | <p>The name of the policy.</p> |
| `group_name` | String | <p>The group the policy is associated with.</p> |
| `policy_document` | String | <p>The policy document.</p>
         <p>IAM stores policies in JSON format. However, resources that were created using CloudFormation
            templates can be formatted in YAML. CloudFormation always converts a YAML policy to JSON format
            before submitting it to IAM.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create group_policy
group_policy = provider.iam.Group_policy {
    group_name = "value"  # <p>The name of the group to associate the policy with.</p>
         <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric 
    characters with no spaces. You can also include any of the following characters: _+=,.@-.</p>
    policy_document = "value"  # <p>The policy document.</p>
         <p>You must provide policies in JSON format in IAM. However, for CloudFormation templates
            formatted in YAML, you can provide the policy in JSON or YAML format. CloudFormation always
            converts a YAML policy to JSON format before submitting it to IAM.</p>
         <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> 
    used to validate this parameter is a string of characters consisting of the following:</p>
         <ul>
            <li>
               <p>Any printable ASCII 
    character ranging from the space character (<code>\u0020</code>) through the end of the ASCII character range</p>
            </li>
            <li>
               <p>The printable characters in the Basic Latin and  Latin-1 Supplement character set 
    (through <code>\u00FF</code>)</p>
            </li>
            <li>
               <p>The special characters tab (<code>\u0009</code>), line feed (<code>\u000A</code>), and 
    carriage return (<code>\u000D</code>)</p>
            </li>
         </ul>
    policy_name = "value"  # <p>The name of the policy document.</p>
         <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric 
    characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
}

# Access group_policy outputs
group_policy_id = group_policy.id
group_policy_policy_name = group_policy.policy_name
group_policy_group_name = group_policy.group_name
group_policy_policy_document = group_policy.policy_document
```

---


### Context_keys_for_custom_policy

ContextKeysForCustomPolicy resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `context_key_names` | Vec<String> | <p>The list of context keys that are referenced in the input policies.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access context_keys_for_custom_policy outputs
context_keys_for_custom_policy_id = context_keys_for_custom_policy.id
context_keys_for_custom_policy_context_key_names = context_keys_for_custom_policy.context_key_names
```

---


### Open_id_connect_provider_thumbprint

OpenIDConnectProviderThumbprint resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `thumbprint_list` | Vec<String> | ✅ | <p>A list of certificate thumbprints that are associated with the specified IAM OpenID
            Connect provider. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/APIReference/API_CreateOpenIDConnectProvider.html">CreateOpenIDConnectProvider</a>. </p> |
| `open_id_connect_provider_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the IAM OIDC provider resource object for which
            you want to update the thumbprint. You can get a list of OIDC provider ARNs by using the
                <a href="https://docs.aws.amazon.com/IAM/latest/APIReference/API_ListOpenIDConnectProviders.html">ListOpenIDConnectProviders</a> operation.</p>
         <p>For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> in the <i>Amazon Web Services General Reference</i>.</p> |



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


### User_policy

UserPolicy resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `user_name` | String | ✅ | <p>The name of the user to associate the policy with.</p>
         <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric 
    characters with no spaces. You can also include any of the following characters: _+=,.@-</p> |
| `policy_name` | String | ✅ | <p>The name of the policy document.</p>
         <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric 
    characters with no spaces. You can also include any of the following characters: _+=,.@-</p> |
| `policy_document` | String | ✅ | <p>The policy document.</p>
         <p>You must provide policies in JSON format in IAM. However, for CloudFormation
            templates formatted in YAML, you can provide the policy in JSON or YAML format. CloudFormation always converts a YAML policy to JSON format before submitting it to
            IAM.</p>
         <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> 
    used to validate this parameter is a string of characters consisting of the following:</p>
         <ul>
            <li>
               <p>Any printable ASCII 
    character ranging from the space character (<code>\u0020</code>) through the end of the ASCII character range</p>
            </li>
            <li>
               <p>The printable characters in the Basic Latin and  Latin-1 Supplement character set 
    (through <code>\u00FF</code>)</p>
            </li>
            <li>
               <p>The special characters tab (<code>\u0009</code>), line feed (<code>\u000A</code>), and 
    carriage return (<code>\u000D</code>)</p>
            </li>
         </ul> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `policy_document` | String | <p>The policy document.</p>
         <p>IAM stores policies in JSON format. However, resources that were created using CloudFormation
            templates can be formatted in YAML. CloudFormation always converts a YAML policy to JSON format
            before submitting it to IAM.</p> |
| `user_name` | String | <p>The user the policy is associated with.</p> |
| `policy_name` | String | <p>The name of the policy.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create user_policy
user_policy = provider.iam.User_policy {
    user_name = "value"  # <p>The name of the user to associate the policy with.</p>
         <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric 
    characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    policy_name = "value"  # <p>The name of the policy document.</p>
         <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric 
    characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    policy_document = "value"  # <p>The policy document.</p>
         <p>You must provide policies in JSON format in IAM. However, for CloudFormation
            templates formatted in YAML, you can provide the policy in JSON or YAML format. CloudFormation always converts a YAML policy to JSON format before submitting it to
            IAM.</p>
         <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> 
    used to validate this parameter is a string of characters consisting of the following:</p>
         <ul>
            <li>
               <p>Any printable ASCII 
    character ranging from the space character (<code>\u0020</code>) through the end of the ASCII character range</p>
            </li>
            <li>
               <p>The printable characters in the Basic Latin and  Latin-1 Supplement character set 
    (through <code>\u00FF</code>)</p>
            </li>
            <li>
               <p>The special characters tab (<code>\u0009</code>), line feed (<code>\u000A</code>), and 
    carriage return (<code>\u000D</code>)</p>
            </li>
         </ul>
}

# Access user_policy outputs
user_policy_id = user_policy.id
user_policy_policy_document = user_policy.policy_document
user_policy_user_name = user_policy.user_name
user_policy_policy_name = user_policy.policy_name
```

---


### Account_password_policy

AccountPasswordPolicy resource

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `require_uppercase_characters` | bool |  | <p>Specifies whether IAM user passwords must contain at least one uppercase character
            from the ISO basic Latin alphabet (A to Z).</p>
         <p>If you do not specify a value for this parameter, then the operation uses the default
            value of <code>false</code>. The result is that passwords do not require at least one
            uppercase character.</p> |
| `require_numbers` | bool |  | <p>Specifies whether IAM user passwords must contain at least one numeric character (0
            to 9).</p>
         <p>If you do not specify a value for this parameter, then the operation uses the default
            value of <code>false</code>. The result is that passwords do not require at least one
            numeric character.</p> |
| `require_lowercase_characters` | bool |  | <p>Specifies whether IAM user passwords must contain at least one lowercase character
            from the ISO basic Latin alphabet (a to z).</p>
         <p>If you do not specify a value for this parameter, then the operation uses the default
            value of <code>false</code>. The result is that passwords do not require at least one
            lowercase character.</p> |
| `max_password_age` | i64 |  | <p>The number of days that an IAM user password is valid.</p>
         <p>If you do not specify a value for this parameter, then the operation uses the default
            value of <code>0</code>. The result is that IAM user passwords never expire.</p> |
| `password_reuse_prevention` | i64 |  | <p>Specifies the number of previous passwords that IAM users are prevented from
            reusing.</p>
         <p>If you do not specify a value for this parameter, then the operation uses the default
            value of <code>0</code>. The result is that IAM users are not prevented from reusing
            previous passwords.</p> |
| `hard_expiry` | bool |  | <p> Prevents IAM users who are accessing the account via the Amazon Web Services Management Console from setting a
            new console password after their password has expired. The IAM user cannot access the
            console until an administrator resets the password.</p>
         <p>If you do not specify a value for this parameter, then the operation uses the default
            value of <code>false</code>. The result is that IAM users can change their passwords
            after they expire and continue to sign in as the user.</p>
         <note>
            <p> In the Amazon Web Services Management Console, the custom password policy option <b>Allow
                    users to change their own password</b> gives IAM users permissions to
                    <code>iam:ChangePassword</code> for only their user and to the
                    <code>iam:GetAccountPasswordPolicy</code> action. This option does not attach a
                permissions policy to each user, rather the permissions are applied at the
                account-level for all users by IAM. IAM users with
                    <code>iam:ChangePassword</code> permission and active access keys can reset
                their own expired console password using the CLI or API.</p>
         </note> |
| `minimum_password_length` | i64 |  | <p>The minimum number of characters allowed in an IAM user password.</p>
         <p>If you do not specify a value for this parameter, then the operation uses the default
            value of <code>6</code>.</p> |
| `allow_users_to_change_password` | bool |  | <p> Allows all IAM users in your account to use the Amazon Web Services Management Console to change their own
            passwords. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_passwords_enable-user-change.html">Permitting
                IAM users to change their own passwords</a> in the
                <i>IAM User Guide</i>.</p>
         <p>If you do not specify a value for this parameter, then the operation uses the default
            value of <code>false</code>. The result is that IAM users in the account do not
            automatically have permissions to change their own password.</p> |
| `require_symbols` | bool |  | <p>Specifies whether IAM user passwords must contain at least one of the following
            non-alphanumeric characters:</p>
         <p>! @ # $ % ^ & * ( ) _ + - = [ ] { } | '</p>
         <p>If you do not specify a value for this parameter, then the operation uses the default
            value of <code>false</code>. The result is that passwords do not require at least one
            symbol character.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `password_policy` | String | <p>A structure that contains details about the account's password policy.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access account_password_policy outputs
account_password_policy_id = account_password_policy.id
account_password_policy_password_policy = account_password_policy.password_policy
```

---


### Credential_report

CredentialReport resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `content` | String | <p>Contains the credential report. The report is Base64-encoded.</p> |
| `report_format` | String | <p>The format (MIME type) of the credential report.</p> |
| `generated_time` | String | <p> The date and time when the credential report was created, in <a href="http://www.iso.org/iso/iso8601">ISO 8601 date-time format</a>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access credential_report outputs
credential_report_id = credential_report.id
credential_report_content = credential_report.content
credential_report_report_format = credential_report.report_format
credential_report_generated_time = credential_report.generated_time
```

---


### Service_last_accessed_details_with_entities

ServiceLastAccessedDetailsWithEntities resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `job_completion_date` | String | <p>The date and time, in <a href="http://www.iso.org/iso/iso8601">ISO 8601 date-time
                format</a>, when the generated report job was completed or failed.</p>
         <p>This field is null if the job is still in progress, as indicated by a job status value
            of <code>IN_PROGRESS</code>.</p> |
| `job_status` | String | <p>The status of the job.</p> |
| `marker` | String | <p>When <code>IsTruncated</code> is <code>true</code>, this element
    is present and contains the value to use for the <code>Marker</code> parameter in a subsequent 
    pagination request.</p> |
| `job_creation_date` | String | <p>The date and time, in <a href="http://www.iso.org/iso/iso8601">ISO 8601 date-time
                format</a>, when the report job was created.</p> |
| `entity_details_list` | Vec<String> | <p>An <code>EntityDetailsList</code> object that contains details about when an IAM
            entity (user or role) used group or policy permissions in an attempt to access the
            specified Amazon Web Services service.</p> |
| `error` | String | <p>An object that contains details about the reason the operation failed.</p> |
| `is_truncated` | bool | <p>A flag that indicates whether there are more items to return. If your 
    results were truncated, you can make a subsequent pagination request using the <code>Marker</code>
    request parameter to retrieve more items. Note that IAM might return fewer than the 
    <code>MaxItems</code> number of results even when there are more results available. We recommend 
    that you check <code>IsTruncated</code> after every call to ensure that you receive all your 
    results.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access service_last_accessed_details_with_entities outputs
service_last_accessed_details_with_entities_id = service_last_accessed_details_with_entities.id
service_last_accessed_details_with_entities_job_completion_date = service_last_accessed_details_with_entities.job_completion_date
service_last_accessed_details_with_entities_job_status = service_last_accessed_details_with_entities.job_status
service_last_accessed_details_with_entities_marker = service_last_accessed_details_with_entities.marker
service_last_accessed_details_with_entities_job_creation_date = service_last_accessed_details_with_entities.job_creation_date
service_last_accessed_details_with_entities_entity_details_list = service_last_accessed_details_with_entities.entity_details_list
service_last_accessed_details_with_entities_error = service_last_accessed_details_with_entities.error
service_last_accessed_details_with_entities_is_truncated = service_last_accessed_details_with_entities.is_truncated
```

---


### Service_linked_role_deletion_status

ServiceLinkedRoleDeletionStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | String | <p>The status of the deletion.</p> |
| `reason` | String | <p>An object that contains details about the reason the deletion failed.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access service_linked_role_deletion_status outputs
service_linked_role_deletion_status_id = service_linked_role_deletion_status.id
service_linked_role_deletion_status_status = service_linked_role_deletion_status.status
service_linked_role_deletion_status_reason = service_linked_role_deletion_status.reason
```

---


### Policy_version

PolicyVersion resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `policy_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the IAM policy to which you want to add a new
            version.</p>
         <p>For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> in the <i>Amazon Web Services General Reference</i>.</p> |
| `set_as_default` | bool |  | <p>Specifies whether to set this version as the policy's default version.</p>
         <p>When this parameter is <code>true</code>, the new policy version becomes the operative
            version. That is, it becomes the version that is in effect for the IAM users, groups,
            and roles that the policy is attached to.</p>
         <p>For more information about managed policy versions, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/policies-managed-versions.html">Versioning for managed
                policies</a> in the <i>IAM User Guide</i>.</p> |
| `policy_document` | String | ✅ | <p>The JSON policy document that you want to use as the content for this new version of
            the policy.</p>
         <p>You must provide policies in JSON format in IAM. However, for CloudFormation
            templates formatted in YAML, you can provide the policy in JSON or YAML format. CloudFormation always converts a YAML policy to JSON format before submitting it to
            IAM.</p>
         <p>The maximum length of the policy document that you can pass in this operation,
            including whitespace, is listed below. To view the maximum character counts of a managed policy with no whitespaces, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_iam-quotas.html#reference_iam-quotas-entity-length">IAM and STS character quotas</a>.</p>
         <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> 
    used to validate this parameter is a string of characters consisting of the following:</p>
         <ul>
            <li>
               <p>Any printable ASCII 
    character ranging from the space character (<code>\u0020</code>) through the end of the ASCII character range</p>
            </li>
            <li>
               <p>The printable characters in the Basic Latin and  Latin-1 Supplement character set 
    (through <code>\u00FF</code>)</p>
            </li>
            <li>
               <p>The special characters tab (<code>\u0009</code>), line feed (<code>\u000A</code>), and 
    carriage return (<code>\u000D</code>)</p>
            </li>
         </ul> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `policy_version` | String | <p>A structure containing details about the policy version.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create policy_version
policy_version = provider.iam.Policy_version {
    policy_arn = "value"  # <p>The Amazon Resource Name (ARN) of the IAM policy to which you want to add a new
            version.</p>
         <p>For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> in the <i>Amazon Web Services General Reference</i>.</p>
    policy_document = "value"  # <p>The JSON policy document that you want to use as the content for this new version of
            the policy.</p>
         <p>You must provide policies in JSON format in IAM. However, for CloudFormation
            templates formatted in YAML, you can provide the policy in JSON or YAML format. CloudFormation always converts a YAML policy to JSON format before submitting it to
            IAM.</p>
         <p>The maximum length of the policy document that you can pass in this operation,
            including whitespace, is listed below. To view the maximum character counts of a managed policy with no whitespaces, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_iam-quotas.html#reference_iam-quotas-entity-length">IAM and STS character quotas</a>.</p>
         <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> 
    used to validate this parameter is a string of characters consisting of the following:</p>
         <ul>
            <li>
               <p>Any printable ASCII 
    character ranging from the space character (<code>\u0020</code>) through the end of the ASCII character range</p>
            </li>
            <li>
               <p>The printable characters in the Basic Latin and  Latin-1 Supplement character set 
    (through <code>\u00FF</code>)</p>
            </li>
            <li>
               <p>The special characters tab (<code>\u0009</code>), line feed (<code>\u000A</code>), and 
    carriage return (<code>\u000D</code>)</p>
            </li>
         </ul>
}

# Access policy_version outputs
policy_version_id = policy_version.id
policy_version_policy_version = policy_version.policy_version
```

---


### Virtual_mfa_device

VirtualMFADevice resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>A list of tags that you want to attach to the new IAM virtual MFA device.
      Each tag consists of a key name and an associated value. For more information about tagging, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_tags.html">Tagging IAM resources</a> in the
      <i>IAM User Guide</i>.</p>
         <note>
            <p>If any one of the tags is invalid or if you exceed the allowed maximum number of tags, then the entire request 
   fails and the resource is not created.</p>
         </note> |
| `path` | String |  | <p> The path for the virtual MFA device. For more information about paths, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html">IAM
                identifiers</a> in the <i>IAM User Guide</i>.</p>
         <p>This parameter is optional. If it is not included, it defaults to a slash (/).</p>
         <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting 
    of either a forward slash (/) by itself or a string that must begin and end with forward slashes.
    In addition, it can contain any ASCII character from the ! (<code>\u0021</code>) through the DEL character (<code>\u007F</code>), including 
    most punctuation characters, digits, and upper and lowercased letters.</p> |
| `virtual_mfa_device_name` | String | ✅ | <p>The name of the virtual MFA device, which must be unique. Use with path to uniquely
            identify a virtual MFA device.</p>
         <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric 
    characters with no spaces. You can also include any of the following characters: _+=,.@-</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create virtual_mfa_device
virtual_mfa_device = provider.iam.Virtual_mfa_device {
    virtual_mfa_device_name = "value"  # <p>The name of the virtual MFA device, which must be unique. Use with path to uniquely
            identify a virtual MFA device.</p>
         <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric 
    characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
}

```

---


### Assume_role_policy

AssumeRolePolicy resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `role_name` | String | ✅ | <p>The name of the role to update with the new policy.</p>
         <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric 
    characters with no spaces. You can also include any of the following characters: _+=,.@-</p> |
| `policy_document` | String | ✅ | <p>The policy that grants an entity permission to assume the role.</p>
         <p>You must provide policies in JSON format in IAM. However, for CloudFormation
            templates formatted in YAML, you can provide the policy in JSON or YAML format. CloudFormation always converts a YAML policy to JSON format before submitting it to
            IAM.</p>
         <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> 
    used to validate this parameter is a string of characters consisting of the following:</p>
         <ul>
            <li>
               <p>Any printable ASCII 
    character ranging from the space character (<code>\u0020</code>) through the end of the ASCII character range</p>
            </li>
            <li>
               <p>The printable characters in the Basic Latin and  Latin-1 Supplement character set 
    (through <code>\u00FF</code>)</p>
            </li>
            <li>
               <p>The special characters tab (<code>\u0009</code>), line feed (<code>\u000A</code>), and 
    carriage return (<code>\u000D</code>)</p>
            </li>
         </ul> |



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


### Saml_provider

SAMLProvider resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>The name of the provider to create.</p>
         <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric 
    characters with no spaces. You can also include any of the following characters: _+=,.@-</p> |
| `tags` | Vec<String> |  | <p>A list of tags that you want to attach to the new IAM SAML provider.
      Each tag consists of a key name and an associated value. For more information about tagging, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_tags.html">Tagging IAM resources</a> in the
      <i>IAM User Guide</i>.</p>
         <note>
            <p>If any one of the tags is invalid or if you exceed the allowed maximum number of tags, then the entire request 
   fails and the resource is not created.</p>
         </note> |
| `add_private_key` | String |  | <p>The private key generated from your external identity provider. The private key must
            be a .pem file that uses AES-GCM or AES-CBC encryption algorithm to decrypt SAML
            assertions.</p> |
| `assertion_encryption_mode` | String |  | <p>Specifies the encryption setting for the SAML provider.</p> |
| `saml_metadata_document` | String | ✅ | <p>An XML document generated by an identity provider (IdP) that supports SAML 2.0. The
            document includes the issuer's name, expiration information, and keys that can be used
            to validate the SAML authentication response (assertions) that are received from the
            IdP. You must generate the metadata document using the identity management software that
            is used as your organization's IdP.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_providers_saml.html">About SAML 2.0-based
                federation</a> in the <i>IAM User Guide</i>
         </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `assertion_encryption_mode` | String | <p>Specifies the encryption setting for the SAML provider.</p> |
| `private_key_list` | Vec<String> | <p>The private key metadata for the SAML provider.</p> |
| `saml_metadata_document` | String | <p>The XML metadata document that includes information about an identity provider.</p> |
| `saml_provider_uuid` | String | <p>The unique identifier assigned to the SAML provider.</p> |
| `valid_until` | String | <p>The expiration date and time for the SAML provider.</p> |
| `tags` | Vec<String> | <p>A list of tags that are attached to the specified IAM SAML provider. The returned list of tags is sorted by tag key.
      For more information about tagging, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_tags.html">Tagging IAM resources</a> in the
      <i>IAM User Guide</i>.</p> |
| `create_date` | String | <p>The date and time when the SAML provider was created.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create saml_provider
saml_provider = provider.iam.Saml_provider {
    name = "value"  # <p>The name of the provider to create.</p>
         <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric 
    characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    saml_metadata_document = "value"  # <p>An XML document generated by an identity provider (IdP) that supports SAML 2.0. The
            document includes the issuer's name, expiration information, and keys that can be used
            to validate the SAML authentication response (assertions) that are received from the
            IdP. You must generate the metadata document using the identity management software that
            is used as your organization's IdP.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_providers_saml.html">About SAML 2.0-based
                federation</a> in the <i>IAM User Guide</i>
         </p>
}

# Access saml_provider outputs
saml_provider_id = saml_provider.id
saml_provider_assertion_encryption_mode = saml_provider.assertion_encryption_mode
saml_provider_private_key_list = saml_provider.private_key_list
saml_provider_saml_metadata_document = saml_provider.saml_metadata_document
saml_provider_saml_provider_uuid = saml_provider.saml_provider_uuid
saml_provider_valid_until = saml_provider.valid_until
saml_provider_tags = saml_provider.tags
saml_provider_create_date = saml_provider.create_date
```

---


### Context_keys_for_principal_policy

ContextKeysForPrincipalPolicy resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `context_key_names` | Vec<String> | <p>The list of context keys that are referenced in the input policies.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access context_keys_for_principal_policy outputs
context_keys_for_principal_policy_id = context_keys_for_principal_policy.id
context_keys_for_principal_policy_context_key_names = context_keys_for_principal_policy.context_key_names
```

---


### Service_last_accessed_details

ServiceLastAccessedDetails resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `error` | String | <p>An object that contains details about the reason the operation failed.</p> |
| `job_type` | String | <p>The type of job. Service jobs return information about when each service was last
            accessed. Action jobs also include information about when tracked actions within the
            service were last accessed.</p> |
| `marker` | String | <p>When <code>IsTruncated</code> is <code>true</code>, this element
    is present and contains the value to use for the <code>Marker</code> parameter in a subsequent 
    pagination request.</p> |
| `job_creation_date` | String | <p>The date and time, in <a href="http://www.iso.org/iso/iso8601">ISO 8601 date-time
                format</a>, when the report job was created.</p> |
| `services_last_accessed` | Vec<String> | <p> A <code>ServiceLastAccessed</code> object that contains details about the most recent
            attempt to access the service.</p> |
| `job_status` | String | <p>The status of the job.</p> |
| `job_completion_date` | String | <p>The date and time, in <a href="http://www.iso.org/iso/iso8601">ISO 8601 date-time
                format</a>, when the generated report job was completed or failed.</p>
         <p>This field is null if the job is still in progress, as indicated by a job status value
            of <code>IN_PROGRESS</code>.</p> |
| `is_truncated` | bool | <p>A flag that indicates whether there are more items to return. If your 
    results were truncated, you can make a subsequent pagination request using the <code>Marker</code>
    request parameter to retrieve more items. Note that IAM might return fewer than the 
    <code>MaxItems</code> number of results even when there are more results available. We recommend 
    that you check <code>IsTruncated</code> after every call to ensure that you receive all your 
    results.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access service_last_accessed_details outputs
service_last_accessed_details_id = service_last_accessed_details.id
service_last_accessed_details_error = service_last_accessed_details.error
service_last_accessed_details_job_type = service_last_accessed_details.job_type
service_last_accessed_details_marker = service_last_accessed_details.marker
service_last_accessed_details_job_creation_date = service_last_accessed_details.job_creation_date
service_last_accessed_details_services_last_accessed = service_last_accessed_details.services_last_accessed
service_last_accessed_details_job_status = service_last_accessed_details.job_status
service_last_accessed_details_job_completion_date = service_last_accessed_details.job_completion_date
service_last_accessed_details_is_truncated = service_last_accessed_details.is_truncated
```

---


### Role_policy

RolePolicy resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `policy_document` | String | ✅ | <p>The policy document.</p>
         <p>You must provide policies in JSON format in IAM. However, for CloudFormation
            templates formatted in YAML, you can provide the policy in JSON or YAML format. CloudFormation always converts a YAML policy to JSON format before submitting it to
            IAM.</p>
         <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> 
    used to validate this parameter is a string of characters consisting of the following:</p>
         <ul>
            <li>
               <p>Any printable ASCII 
    character ranging from the space character (<code>\u0020</code>) through the end of the ASCII character range</p>
            </li>
            <li>
               <p>The printable characters in the Basic Latin and  Latin-1 Supplement character set 
    (through <code>\u00FF</code>)</p>
            </li>
            <li>
               <p>The special characters tab (<code>\u0009</code>), line feed (<code>\u000A</code>), and 
    carriage return (<code>\u000D</code>)</p>
            </li>
         </ul> |
| `role_name` | String | ✅ | <p>The name of the role to associate the policy with.</p>
         <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric 
    characters with no spaces. You can also include any of the following characters: _+=,.@-</p> |
| `policy_name` | String | ✅ | <p>The name of the policy document.</p>
         <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric 
    characters with no spaces. You can also include any of the following characters: _+=,.@-</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `policy_name` | String | <p>The name of the policy.</p> |
| `policy_document` | String | <p>The policy document.</p>
         <p>IAM stores policies in JSON format. However, resources that were created using CloudFormation
            templates can be formatted in YAML. CloudFormation always converts a YAML policy to JSON format
            before submitting it to IAM.</p> |
| `role_name` | String | <p>The role the policy is associated with.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create role_policy
role_policy = provider.iam.Role_policy {
    policy_document = "value"  # <p>The policy document.</p>
         <p>You must provide policies in JSON format in IAM. However, for CloudFormation
            templates formatted in YAML, you can provide the policy in JSON or YAML format. CloudFormation always converts a YAML policy to JSON format before submitting it to
            IAM.</p>
         <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> 
    used to validate this parameter is a string of characters consisting of the following:</p>
         <ul>
            <li>
               <p>Any printable ASCII 
    character ranging from the space character (<code>\u0020</code>) through the end of the ASCII character range</p>
            </li>
            <li>
               <p>The printable characters in the Basic Latin and  Latin-1 Supplement character set 
    (through <code>\u00FF</code>)</p>
            </li>
            <li>
               <p>The special characters tab (<code>\u0009</code>), line feed (<code>\u000A</code>), and 
    carriage return (<code>\u000D</code>)</p>
            </li>
         </ul>
    role_name = "value"  # <p>The name of the role to associate the policy with.</p>
         <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric 
    characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    policy_name = "value"  # <p>The name of the policy document.</p>
         <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric 
    characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
}

# Access role_policy outputs
role_policy_id = role_policy.id
role_policy_policy_name = role_policy.policy_name
role_policy_policy_document = role_policy.policy_document
role_policy_role_name = role_policy.role_name
```

---


### Ssh_public_key

SSHPublicKey resource

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `status` | String | ✅ | <p>The status to assign to the SSH public key. <code>Active</code> means that the key can
            be used for authentication with an CodeCommit repository. <code>Inactive</code> means that
            the key cannot be used.</p> |
| `user_name` | String | ✅ | <p>The name of the IAM user associated with the SSH public key.</p>
         <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric 
    characters with no spaces. You can also include any of the following characters: _+=,.@-</p> |
| `ssh_public_key_id` | String | ✅ | <p>The unique identifier for the SSH public key.</p>
         <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters that can 
    consist of any upper or lowercased letter or digit.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `ssh_public_key` | String | <p>A structure containing details about the SSH public key.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access ssh_public_key outputs
ssh_public_key_id = ssh_public_key.id
ssh_public_key_ssh_public_key = ssh_public_key.ssh_public_key
```

---


### Account_authorization_details

AccountAuthorizationDetails resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `policies` | Vec<String> | <p>A list containing information about managed policies.</p> |
| `marker` | String | <p>When <code>IsTruncated</code> is <code>true</code>, this element
    is present and contains the value to use for the <code>Marker</code> parameter in a subsequent 
    pagination request.</p> |
| `group_detail_list` | Vec<String> | <p>A list containing information about IAM groups.</p> |
| `user_detail_list` | Vec<String> | <p>A list containing information about IAM users.</p> |
| `role_detail_list` | Vec<String> | <p>A list containing information about IAM roles.</p> |
| `is_truncated` | bool | <p>A flag that indicates whether there are more items to return. If your 
    results were truncated, you can make a subsequent pagination request using the <code>Marker</code>
    request parameter to retrieve more items. Note that IAM might return fewer than the 
    <code>MaxItems</code> number of results even when there are more results available. We recommend 
    that you check <code>IsTruncated</code> after every call to ensure that you receive all your 
    results.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access account_authorization_details outputs
account_authorization_details_id = account_authorization_details.id
account_authorization_details_policies = account_authorization_details.policies
account_authorization_details_marker = account_authorization_details.marker
account_authorization_details_group_detail_list = account_authorization_details.group_detail_list
account_authorization_details_user_detail_list = account_authorization_details.user_detail_list
account_authorization_details_role_detail_list = account_authorization_details.role_detail_list
account_authorization_details_is_truncated = account_authorization_details.is_truncated
```

---


### Service_linked_role

ServiceLinkedRole resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | <p>The description of the role.</p> |
| `aws_service_name` | String | ✅ | <p>The service principal for the Amazon Web Services service to which this role is attached. You use a
            string similar to a URL but without the http:// in front. For example:
                <code>elasticbeanstalk.amazonaws.com</code>. </p>
         <p>Service principals are unique and case-sensitive. To find the exact service principal
            for your service-linked role, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_aws-services-that-work-with-iam.html">Amazon Web Services services
                that work with IAM</a> in the <i>IAM User Guide</i>. Look for
            the services that have <b>Yes </b>in the <b>Service-Linked Role</b> column. Choose the <b>Yes</b> link to view the service-linked role documentation for that
            service.</p> |
| `custom_suffix` | String |  | <p></p>
         <p>A string that you provide, which is combined with the service-provided prefix to form
            the complete role name. If you make multiple requests for the same service, then you
            must supply a different <code>CustomSuffix</code> for each request. Otherwise the
            request fails with a duplicate role name error. For example, you could add
                <code>-1</code> or <code>-debug</code> to the suffix.</p>
         <p>Some services do not support the <code>CustomSuffix</code> parameter. If you provide
            an optional suffix and the operation fails, try the operation again without the
            suffix.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create service_linked_role
service_linked_role = provider.iam.Service_linked_role {
    aws_service_name = "value"  # <p>The service principal for the Amazon Web Services service to which this role is attached. You use a
            string similar to a URL but without the http:// in front. For example:
                <code>elasticbeanstalk.amazonaws.com</code>. </p>
         <p>Service principals are unique and case-sensitive. To find the exact service principal
            for your service-linked role, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_aws-services-that-work-with-iam.html">Amazon Web Services services
                that work with IAM</a> in the <i>IAM User Guide</i>. Look for
            the services that have <b>Yes </b>in the <b>Service-Linked Role</b> column. Choose the <b>Yes</b> link to view the service-linked role documentation for that
            service.</p>
}

```

---


### Group

Group resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `group_name` | String | ✅ | <p>The name of the group to create. Do not include the path in this value.</p>
         <p>IAM user, group, role, and policy names must be unique within the account. Names are
            not distinguished by case. For example, you cannot create resources named both
            "MyResource" and "myresource".</p> |
| `path` | String |  | <p> The path to the group. For more information about paths, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html">IAM
                identifiers</a> in the <i>IAM User Guide</i>.</p>
         <p>This parameter is optional. If it is not included, it defaults to a slash (/).</p>
         <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting 
    of either a forward slash (/) by itself or a string that must begin and end with forward slashes.
    In addition, it can contain any ASCII character from the ! (<code>\u0021</code>) through the DEL character (<code>\u007F</code>), including 
    most punctuation characters, digits, and upper and lowercased letters.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `marker` | String | <p>When <code>IsTruncated</code> is <code>true</code>, this element
    is present and contains the value to use for the <code>Marker</code> parameter in a subsequent 
    pagination request.</p> |
| `group` | String | <p>A structure that contains details about the group.</p> |
| `is_truncated` | bool | <p>A flag that indicates whether there are more items to return. If your 
    results were truncated, you can make a subsequent pagination request using the <code>Marker</code>
    request parameter to retrieve more items. Note that IAM might return fewer than the 
    <code>MaxItems</code> number of results even when there are more results available. We recommend 
    that you check <code>IsTruncated</code> after every call to ensure that you receive all your 
    results.</p> |
| `users` | Vec<String> | <p>A list of users in the group.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create group
group = provider.iam.Group {
    group_name = "value"  # <p>The name of the group to create. Do not include the path in this value.</p>
         <p>IAM user, group, role, and policy names must be unique within the account. Names are
            not distinguished by case. For example, you cannot create resources named both
            "MyResource" and "myresource".</p>
}

# Access group outputs
group_id = group.id
group_marker = group.marker
group_group = group.group
group_is_truncated = group.is_truncated
group_users = group.users
```

---


### Access_key

AccessKey resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `user_name` | String |  | <p>The name of the IAM user that the new key will belong to.</p>
         <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric 
    characters with no spaces. You can also include any of the following characters: _+=,.@-</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create access_key
access_key = provider.iam.Access_key {
}

```

---


### Access_key_last_used

AccessKeyLastUsed resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `access_key_last_used` | String | <p>Contains information about the last time the access key was used.</p> |
| `user_name` | String | <p>The name of the IAM user that owns this access key.</p>
         <p></p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access access_key_last_used outputs
access_key_last_used_id = access_key_last_used.id
access_key_last_used_access_key_last_used = access_key_last_used.access_key_last_used
access_key_last_used_user_name = access_key_last_used.user_name
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple role_permissions_boundary resources
role_permissions_boundary_0 = provider.iam.Role_permissions_boundary {
    permissions_boundary = "value-0"
    role_name = "value-0"
}
role_permissions_boundary_1 = provider.iam.Role_permissions_boundary {
    permissions_boundary = "value-1"
    role_name = "value-1"
}
role_permissions_boundary_2 = provider.iam.Role_permissions_boundary {
    permissions_boundary = "value-2"
    role_name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    role_permissions_boundary = provider.iam.Role_permissions_boundary {
        permissions_boundary = "production-value"
        role_name = "production-value"
    }
```

---

## Related Documentation

- [AWS Iam Documentation](https://docs.aws.amazon.com/iam/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
