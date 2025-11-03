# Sso_admin Service



**Resources**: 19

---

## Overview

The sso_admin service provides access to 19 resource types:

- [Account_assignment_deletion_status](#account_assignment_deletion_status) [R]
- [Application_assignment_configuration](#application_assignment_configuration) [CR]
- [Inline_policy_from_permission_set](#inline_policy_from_permission_set) [D]
- [Permissions_boundary_from_permission_set](#permissions_boundary_from_permission_set) [D]
- [Permissions_boundary_for_permission_set](#permissions_boundary_for_permission_set) [R]
- [Application](#application) [CRUD]
- [Application_assignment](#application_assignment) [CRD]
- [Account_assignment](#account_assignment) [CD]
- [Account_assignment_creation_status](#account_assignment_creation_status) [R]
- [Instance](#instance) [CRUD]
- [Permission_set](#permission_set) [CRUD]
- [Application_provider](#application_provider) [R]
- [Inline_policy_for_permission_set](#inline_policy_for_permission_set) [R]
- [Instance_access_control_attribute_configuration](#instance_access_control_attribute_configuration) [CRUD]
- [Trusted_token_issuer](#trusted_token_issuer) [CRUD]
- [Application_session_configuration](#application_session_configuration) [CR]
- [Inline_policy_to_permission_set](#inline_policy_to_permission_set) [C]
- [Permission_set_provisioning_status](#permission_set_provisioning_status) [R]
- [Permissions_boundary_to_permission_set](#permissions_boundary_to_permission_set) [C]

---

## Resources


### Account_assignment_deletion_status

AccountAssignmentDeletionStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `account_assignment_deletion_status` | String | <p>The status object for the account assignment deletion operation.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access account_assignment_deletion_status outputs
account_assignment_deletion_status_id = account_assignment_deletion_status.id
account_assignment_deletion_status_account_assignment_deletion_status = account_assignment_deletion_status.account_assignment_deletion_status
```

---


### Application_assignment_configuration

ApplicationAssignmentConfiguration resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `application_arn` | String | ✅ | <p>Specifies the ARN of the application. For more information about ARNs, see <a href="/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and Amazon Web Services Service Namespaces</a> in the <i>Amazon Web Services General Reference</i>.</p> |
| `assignment_required` | bool | ✅ | <p>If <code>AssignmentsRequired</code> is <code>true</code> (default value), users don’t have access to the application unless an assignment is created using the <a href="https://docs.aws.amazon.com/singlesignon/latest/APIReference/API_CreateApplicationAssignment.html">CreateApplicationAssignment API</a>. If <code>false</code>, all users have access to the application. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `assignment_required` | bool | <p>If <code>AssignmentsRequired</code> is <code>true</code> (default value), users don’t have access to the application unless an assignment is created using the <a href="https://docs.aws.amazon.com/singlesignon/latest/APIReference/API_CreateApplicationAssignment.html">CreateApplicationAssignment API</a>. If <code>false</code>, all users have access to the application. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create application_assignment_configuration
application_assignment_configuration = provider.sso_admin.Application_assignment_configuration {
    application_arn = "value"  # <p>Specifies the ARN of the application. For more information about ARNs, see <a href="/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and Amazon Web Services Service Namespaces</a> in the <i>Amazon Web Services General Reference</i>.</p>
    assignment_required = "value"  # <p>If <code>AssignmentsRequired</code> is <code>true</code> (default value), users don’t have access to the application unless an assignment is created using the <a href="https://docs.aws.amazon.com/singlesignon/latest/APIReference/API_CreateApplicationAssignment.html">CreateApplicationAssignment API</a>. If <code>false</code>, all users have access to the application. </p>
}

# Access application_assignment_configuration outputs
application_assignment_configuration_id = application_assignment_configuration.id
application_assignment_configuration_assignment_required = application_assignment_configuration.assignment_required
```

---


### Inline_policy_from_permission_set

InlinePolicyFromPermissionSet resource

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


### Permissions_boundary_from_permission_set

PermissionsBoundaryFromPermissionSet resource

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


### Permissions_boundary_for_permission_set

PermissionsBoundaryForPermissionSet resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `permissions_boundary` | String | <p>The permissions boundary attached to the specified permission set.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access permissions_boundary_for_permission_set outputs
permissions_boundary_for_permission_set_id = permissions_boundary_for_permission_set.id
permissions_boundary_for_permission_set_permissions_boundary = permissions_boundary_for_permission_set.permissions_boundary
```

---


### Application

Application resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `instance_arn` | String | ✅ | <p>The ARN of the instance of IAM Identity Center under which the operation will run. For more information about ARNs, see <a href="/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and Amazon Web Services Service Namespaces</a> in the <i>Amazon Web Services General Reference</i>.</p> |
| `tags` | Vec<String> |  | <p>Specifies tags to be attached to the application.</p> |
| `description` | String |  | <p>The description of the .</p> |
| `status` | String |  | <p>Specifies whether the application is enabled or disabled.</p> |
| `name` | String | ✅ | <p>The name of the .</p> |
| `client_token` | String |  | <p>Specifies a unique, case-sensitive ID that you provide to ensure the idempotency of the request. This lets you safely retry the request without accidentally performing the same operation a second time. Passing the same value to a later call to an operation requires that you also pass the same value for all other parameters. We recommend that you use a <a href="https://wikipedia.org/wiki/Universally_unique_identifier">UUID type of value</a>.</p> <p>If you don't provide this value, then Amazon Web Services generates a random one for you.</p> <p>If you retry the operation with the same <code>ClientToken</code>, but with different parameters, the retry fails with an <code>IdempotentParameterMismatch</code> error.</p> |
| `portal_options` | String |  | <p>A structure that describes the options for the portal associated with an application.</p> |
| `application_provider_arn` | String | ✅ | <p>The ARN of the application provider under which the operation will run.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `instance_arn` | String | <p>The ARN of the IAM Identity Center application under which the operation will run. For more information about ARNs, see <a href="/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and Amazon Web Services Service Namespaces</a> in the <i>Amazon Web Services General Reference</i>.</p> |
| `application_provider_arn` | String | <p>The ARN of the application provider under which the operation will run.</p> |
| `status` | String | <p>Specifies whether the application is enabled or disabled.</p> |
| `name` | String | <p>The application name.</p> |
| `portal_options` | String | <p>A structure that describes the options for the portal associated with an application.</p> |
| `description` | String | <p>The description of the .</p> |
| `application_account` | String | <p>The account ID.</p> |
| `application_arn` | String | <p>Specifies the ARN of the application.</p> |
| `created_date` | String | <p>The date the application was created.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create application
application = provider.sso_admin.Application {
    instance_arn = "value"  # <p>The ARN of the instance of IAM Identity Center under which the operation will run. For more information about ARNs, see <a href="/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and Amazon Web Services Service Namespaces</a> in the <i>Amazon Web Services General Reference</i>.</p>
    name = "value"  # <p>The name of the .</p>
    application_provider_arn = "value"  # <p>The ARN of the application provider under which the operation will run.</p>
}

# Access application outputs
application_id = application.id
application_instance_arn = application.instance_arn
application_application_provider_arn = application.application_provider_arn
application_status = application.status
application_name = application.name
application_portal_options = application.portal_options
application_description = application.description
application_application_account = application.application_account
application_application_arn = application.application_arn
application_created_date = application.created_date
```

---


### Application_assignment

ApplicationAssignment resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `principal_type` | String | ✅ | <p>The entity type for which the assignment will be created.</p> |
| `application_arn` | String | ✅ | <p>The ARN of the application for which the assignment is created.</p> |
| `principal_id` | String | ✅ | <p>An identifier for an object in IAM Identity Center, such as a user or group. PrincipalIds are GUIDs (For example, f81d4fae-7dec-11d0-a765-00a0c91e6bf6). For more information about PrincipalIds in IAM Identity Center, see the <a href="https://docs.aws.amazon.com/singlesignon/latest/IdentityStoreAPIReference/welcome.html">IAM Identity Center Identity Store API Reference</a>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `principal_type` | String | <p>The entity type for which the assignment will be created.</p> |
| `principal_id` | String | <p>An identifier for an object in IAM Identity Center, such as a user or group. PrincipalIds are GUIDs (For example, f81d4fae-7dec-11d0-a765-00a0c91e6bf6). For more information about PrincipalIds in IAM Identity Center, see the <a href="https://docs.aws.amazon.com/singlesignon/latest/IdentityStoreAPIReference/welcome.html">IAM Identity Center Identity Store API Reference</a>.</p> |
| `application_arn` | String | <p>Specifies the ARN of the application. For more information about ARNs, see <a href="/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and Amazon Web Services Service Namespaces</a> in the <i>Amazon Web Services General Reference</i>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create application_assignment
application_assignment = provider.sso_admin.Application_assignment {
    principal_type = "value"  # <p>The entity type for which the assignment will be created.</p>
    application_arn = "value"  # <p>The ARN of the application for which the assignment is created.</p>
    principal_id = "value"  # <p>An identifier for an object in IAM Identity Center, such as a user or group. PrincipalIds are GUIDs (For example, f81d4fae-7dec-11d0-a765-00a0c91e6bf6). For more information about PrincipalIds in IAM Identity Center, see the <a href="https://docs.aws.amazon.com/singlesignon/latest/IdentityStoreAPIReference/welcome.html">IAM Identity Center Identity Store API Reference</a>.</p>
}

# Access application_assignment outputs
application_assignment_id = application_assignment.id
application_assignment_principal_type = application_assignment.principal_type
application_assignment_principal_id = application_assignment.principal_id
application_assignment_application_arn = application_assignment.application_arn
```

---


### Account_assignment

AccountAssignment resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `permission_set_arn` | String | ✅ | <p>The ARN of the permission set that the admin wants to grant the principal access to.</p> |
| `target_type` | String | ✅ | <p>The entity type for which the assignment will be created.</p> |
| `principal_id` | String | ✅ | <p>An identifier for an object in IAM Identity Center, such as a user or group. PrincipalIds are GUIDs (For example, f81d4fae-7dec-11d0-a765-00a0c91e6bf6). For more information about PrincipalIds in IAM Identity Center, see the <a href="/singlesignon/latest/IdentityStoreAPIReference/welcome.html">IAM Identity Center Identity Store API Reference</a>.</p> |
| `principal_type` | String | ✅ | <p>The entity type for which the assignment will be created.</p> |
| `target_id` | String | ✅ | <p>TargetID is an Amazon Web Services account identifier, (For example, 123456789012).</p> |
| `instance_arn` | String | ✅ | <p>The ARN of the IAM Identity Center instance under which the operation will be executed. For more information about ARNs, see <a href="/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and Amazon Web Services Service Namespaces</a> in the <i>Amazon Web Services General Reference</i>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create account_assignment
account_assignment = provider.sso_admin.Account_assignment {
    permission_set_arn = "value"  # <p>The ARN of the permission set that the admin wants to grant the principal access to.</p>
    target_type = "value"  # <p>The entity type for which the assignment will be created.</p>
    principal_id = "value"  # <p>An identifier for an object in IAM Identity Center, such as a user or group. PrincipalIds are GUIDs (For example, f81d4fae-7dec-11d0-a765-00a0c91e6bf6). For more information about PrincipalIds in IAM Identity Center, see the <a href="/singlesignon/latest/IdentityStoreAPIReference/welcome.html">IAM Identity Center Identity Store API Reference</a>.</p>
    principal_type = "value"  # <p>The entity type for which the assignment will be created.</p>
    target_id = "value"  # <p>TargetID is an Amazon Web Services account identifier, (For example, 123456789012).</p>
    instance_arn = "value"  # <p>The ARN of the IAM Identity Center instance under which the operation will be executed. For more information about ARNs, see <a href="/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and Amazon Web Services Service Namespaces</a> in the <i>Amazon Web Services General Reference</i>.</p>
}

```

---


### Account_assignment_creation_status

AccountAssignmentCreationStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `account_assignment_creation_status` | String | <p>The status object for the account assignment creation operation.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access account_assignment_creation_status outputs
account_assignment_creation_status_id = account_assignment_creation_status.id
account_assignment_creation_status_account_assignment_creation_status = account_assignment_creation_status.account_assignment_creation_status
```

---


### Instance

Instance resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | <p>The name of the instance of IAM Identity Center.</p> |
| `client_token` | String |  | <p>Specifies a unique, case-sensitive ID that you provide to ensure the idempotency of the request. This lets you safely retry the request without accidentally performing the same operation a second time. Passing the same value to a later call to an operation requires that you also pass the same value for all other parameters. We recommend that you use a <a href="https://wikipedia.org/wiki/Universally_unique_identifier">UUID type of value</a>.</p> <p>If you don't provide this value, then Amazon Web Services generates a random one for you.</p> <p>If you retry the operation with the same <code>ClientToken</code>, but with different parameters, the retry fails with an <code>IdempotentParameterMismatch</code> error.</p> |
| `tags` | Vec<String> |  | <p>Specifies tags to be attached to the instance of IAM Identity Center.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `owner_account_id` | String | <p>The identifier of the Amazon Web Services account for which the instance was created.</p> |
| `created_date` | String | <p>The date the instance was created.</p> |
| `name` | String | <p>Specifies the instance name.</p> |
| `status` | String | <p>The status of the instance. </p> |
| `instance_arn` | String | <p>The ARN of the instance of IAM Identity Center under which the operation will run. For more information about ARNs, see <a href="/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and Amazon Web Services Service Namespaces</a> in the <i>Amazon Web Services General Reference</i>.</p> |
| `identity_store_id` | String | <p>The identifier of the identity store that is connected to the instance of IAM Identity Center.</p> |
| `status_reason` | String | <p>Provides additional context about the current status of the IAM Identity Center instance. This field is particularly useful when an instance is in a non-ACTIVE state, such as CREATE_FAILED. When an instance fails to create or update, this field contains information about the cause, which may include issues with KMS key configuration, permission problems with the specified KMS key, or service-related errors. </p> |
| `encryption_configuration_details` | String | <p>Contains the encryption configuration for your IAM Identity Center instance, including the encryption status, KMS key type, and KMS key ARN.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create instance
instance = provider.sso_admin.Instance {
}

# Access instance outputs
instance_id = instance.id
instance_owner_account_id = instance.owner_account_id
instance_created_date = instance.created_date
instance_name = instance.name
instance_status = instance.status
instance_instance_arn = instance.instance_arn
instance_identity_store_id = instance.identity_store_id
instance_status_reason = instance.status_reason
instance_encryption_configuration_details = instance.encryption_configuration_details
```

---


### Permission_set

PermissionSet resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `relay_state` | String |  | <p>Used to redirect users within the application during the federation authentication process.</p> |
| `description` | String |  | <p>The description of the <a>PermissionSet</a>.</p> |
| `tags` | Vec<String> |  | <p>The tags to attach to the new <a>PermissionSet</a>.</p> |
| `name` | String | ✅ | <p>The name of the <a>PermissionSet</a>.</p> |
| `session_duration` | String |  | <p>The length of time that the application user sessions are valid in the ISO-8601 standard.</p> |
| `instance_arn` | String | ✅ | <p>The ARN of the IAM Identity Center instance under which the operation will be executed. For more information about ARNs, see <a href="/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and Amazon Web Services Service Namespaces</a> in the <i>Amazon Web Services General Reference</i>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `permission_set` | String | <p>Describes the level of access on an Amazon Web Services account.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create permission_set
permission_set = provider.sso_admin.Permission_set {
    name = "value"  # <p>The name of the <a>PermissionSet</a>.</p>
    instance_arn = "value"  # <p>The ARN of the IAM Identity Center instance under which the operation will be executed. For more information about ARNs, see <a href="/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and Amazon Web Services Service Namespaces</a> in the <i>Amazon Web Services General Reference</i>.</p>
}

# Access permission_set outputs
permission_set_id = permission_set.id
permission_set_permission_set = permission_set.permission_set
```

---


### Application_provider

ApplicationProvider resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `federation_protocol` | String | <p>The protocol used to federate to the application provider.</p> |
| `application_provider_arn` | String | <p>The ARN of the application provider.</p> |
| `display_data` | String | <p>A structure with details about the display data for the application provider.</p> |
| `resource_server_config` | String | <p>A structure with details about the receiving application.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access application_provider outputs
application_provider_id = application_provider.id
application_provider_federation_protocol = application_provider.federation_protocol
application_provider_application_provider_arn = application_provider.application_provider_arn
application_provider_display_data = application_provider.display_data
application_provider_resource_server_config = application_provider.resource_server_config
```

---


### Inline_policy_for_permission_set

InlinePolicyForPermissionSet resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `inline_policy` | String | <p>The inline policy that is attached to the permission set.</p> <note> <p>For <code>Length Constraints</code>, if a valid ARN is provided for a permission set, it is possible for an empty inline policy to be returned.</p> </note> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access inline_policy_for_permission_set outputs
inline_policy_for_permission_set_id = inline_policy_for_permission_set.id
inline_policy_for_permission_set_inline_policy = inline_policy_for_permission_set.inline_policy
```

---


### Instance_access_control_attribute_configuration

InstanceAccessControlAttributeConfiguration resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `instance_arn` | String | ✅ | <p>The ARN of the IAM Identity Center instance under which the operation will be executed.</p> |
| `instance_access_control_attribute_configuration` | String | ✅ | <p>Specifies the IAM Identity Center identity store attributes to add to your ABAC configuration. When using an external identity provider as an identity source, you can pass attributes through the SAML assertion. Doing so provides an alternative to configuring attributes from the IAM Identity Center identity store. If a SAML assertion passes any of these attributes, IAM Identity Center will replace the attribute value with the value from the IAM Identity Center identity store.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status_reason` | String | <p>Provides more details about the current status of the specified attribute.</p> |
| `status` | String | <p>The status of the attribute configuration process.</p> |
| `instance_access_control_attribute_configuration` | String | <p>Gets the list of IAM Identity Center identity store attributes that have been added to your ABAC configuration.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create instance_access_control_attribute_configuration
instance_access_control_attribute_configuration = provider.sso_admin.Instance_access_control_attribute_configuration {
    instance_arn = "value"  # <p>The ARN of the IAM Identity Center instance under which the operation will be executed.</p>
    instance_access_control_attribute_configuration = "value"  # <p>Specifies the IAM Identity Center identity store attributes to add to your ABAC configuration. When using an external identity provider as an identity source, you can pass attributes through the SAML assertion. Doing so provides an alternative to configuring attributes from the IAM Identity Center identity store. If a SAML assertion passes any of these attributes, IAM Identity Center will replace the attribute value with the value from the IAM Identity Center identity store.</p>
}

# Access instance_access_control_attribute_configuration outputs
instance_access_control_attribute_configuration_id = instance_access_control_attribute_configuration.id
instance_access_control_attribute_configuration_status_reason = instance_access_control_attribute_configuration.status_reason
instance_access_control_attribute_configuration_status = instance_access_control_attribute_configuration.status
instance_access_control_attribute_configuration_instance_access_control_attribute_configuration = instance_access_control_attribute_configuration.instance_access_control_attribute_configuration
```

---


### Trusted_token_issuer

TrustedTokenIssuer resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_token` | String |  | <p>Specifies a unique, case-sensitive ID that you provide to ensure the idempotency of the request. This lets you safely retry the request without accidentally performing the same operation a second time. Passing the same value to a later call to an operation requires that you also pass the same value for all other parameters. We recommend that you use a <a href="https://wikipedia.org/wiki/Universally_unique_identifier">UUID type of value.</a>.</p> <p>If you don't provide this value, then Amazon Web Services generates a random one for you.</p> <p>If you retry the operation with the same <code>ClientToken</code>, but with different parameters, the retry fails with an <code>IdempotentParameterMismatch</code> error.</p> |
| `tags` | Vec<String> |  | <p>Specifies tags to be attached to the new trusted token issuer configuration.</p> |
| `name` | String | ✅ | <p>Specifies the name of the new trusted token issuer configuration.</p> |
| `trusted_token_issuer_type` | String | ✅ | <p>Specifies the type of the new trusted token issuer.</p> |
| `instance_arn` | String | ✅ | <p>Specifies the ARN of the instance of IAM Identity Center to contain the new trusted token issuer configuration.</p> |
| `trusted_token_issuer_configuration` | String | ✅ | <p>Specifies settings that apply to the new trusted token issuer configuration. The settings that are available depend on what <code>TrustedTokenIssuerType</code> you specify.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `trusted_token_issuer_arn` | String | <p>The ARN of the trusted token issuer configuration.</p> |
| `trusted_token_issuer_type` | String | <p>The type of the trusted token issuer.</p> |
| `trusted_token_issuer_configuration` | String | <p>A structure the describes the settings that apply of this trusted token issuer.</p> |
| `name` | String | <p>The name of the trusted token issuer configuration.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create trusted_token_issuer
trusted_token_issuer = provider.sso_admin.Trusted_token_issuer {
    name = "value"  # <p>Specifies the name of the new trusted token issuer configuration.</p>
    trusted_token_issuer_type = "value"  # <p>Specifies the type of the new trusted token issuer.</p>
    instance_arn = "value"  # <p>Specifies the ARN of the instance of IAM Identity Center to contain the new trusted token issuer configuration.</p>
    trusted_token_issuer_configuration = "value"  # <p>Specifies settings that apply to the new trusted token issuer configuration. The settings that are available depend on what <code>TrustedTokenIssuerType</code> you specify.</p>
}

# Access trusted_token_issuer outputs
trusted_token_issuer_id = trusted_token_issuer.id
trusted_token_issuer_trusted_token_issuer_arn = trusted_token_issuer.trusted_token_issuer_arn
trusted_token_issuer_trusted_token_issuer_type = trusted_token_issuer.trusted_token_issuer_type
trusted_token_issuer_trusted_token_issuer_configuration = trusted_token_issuer.trusted_token_issuer_configuration
trusted_token_issuer_name = trusted_token_issuer.name
```

---


### Application_session_configuration

ApplicationSessionConfiguration resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `application_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the application for which to update the session configuration.</p> |
| `user_background_session_application_status` | String |  | <p>The status of user background sessions for the application.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `user_background_session_application_status` | String | <p>The status of user background sessions for the application. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create application_session_configuration
application_session_configuration = provider.sso_admin.Application_session_configuration {
    application_arn = "value"  # <p>The Amazon Resource Name (ARN) of the application for which to update the session configuration.</p>
}

# Access application_session_configuration outputs
application_session_configuration_id = application_session_configuration.id
application_session_configuration_user_background_session_application_status = application_session_configuration.user_background_session_application_status
```

---


### Inline_policy_to_permission_set

InlinePolicyToPermissionSet resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `inline_policy` | String | ✅ | <p>The inline policy to attach to a <a>PermissionSet</a>.</p> |
| `permission_set_arn` | String | ✅ | <p>The ARN of the permission set.</p> |
| `instance_arn` | String | ✅ | <p>The ARN of the IAM Identity Center instance under which the operation will be executed. For more information about ARNs, see <a href="/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and Amazon Web Services Service Namespaces</a> in the <i>Amazon Web Services General Reference</i>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create inline_policy_to_permission_set
inline_policy_to_permission_set = provider.sso_admin.Inline_policy_to_permission_set {
    inline_policy = "value"  # <p>The inline policy to attach to a <a>PermissionSet</a>.</p>
    permission_set_arn = "value"  # <p>The ARN of the permission set.</p>
    instance_arn = "value"  # <p>The ARN of the IAM Identity Center instance under which the operation will be executed. For more information about ARNs, see <a href="/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and Amazon Web Services Service Namespaces</a> in the <i>Amazon Web Services General Reference</i>.</p>
}

```

---


### Permission_set_provisioning_status

PermissionSetProvisioningStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `permission_set_provisioning_status` | String | <p>The status object for the permission set provisioning operation.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access permission_set_provisioning_status outputs
permission_set_provisioning_status_id = permission_set_provisioning_status.id
permission_set_provisioning_status_permission_set_provisioning_status = permission_set_provisioning_status.permission_set_provisioning_status
```

---


### Permissions_boundary_to_permission_set

PermissionsBoundaryToPermissionSet resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `permission_set_arn` | String | ✅ | <p>The ARN of the <code>PermissionSet</code>.</p> |
| `permissions_boundary` | String | ✅ | <p>The permissions boundary that you want to attach to a <code>PermissionSet</code>.</p> |
| `instance_arn` | String | ✅ | <p>The ARN of the IAM Identity Center instance under which the operation will be executed. </p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create permissions_boundary_to_permission_set
permissions_boundary_to_permission_set = provider.sso_admin.Permissions_boundary_to_permission_set {
    permission_set_arn = "value"  # <p>The ARN of the <code>PermissionSet</code>.</p>
    permissions_boundary = "value"  # <p>The permissions boundary that you want to attach to a <code>PermissionSet</code>.</p>
    instance_arn = "value"  # <p>The ARN of the IAM Identity Center instance under which the operation will be executed. </p>
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

# Create multiple account_assignment_deletion_status resources
account_assignment_deletion_status_0 = provider.sso_admin.Account_assignment_deletion_status {
}
account_assignment_deletion_status_1 = provider.sso_admin.Account_assignment_deletion_status {
}
account_assignment_deletion_status_2 = provider.sso_admin.Account_assignment_deletion_status {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    account_assignment_deletion_status = provider.sso_admin.Account_assignment_deletion_status {
    }
```

---

## Related Documentation

- [AWS Sso_admin Documentation](https://docs.aws.amazon.com/sso_admin/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
