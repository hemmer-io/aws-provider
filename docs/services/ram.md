# Ram Service



**Resources**: 7

---

## Overview

The ram service provides access to 7 resource types:

- [Permission_version](#permission_version) [CD]
- [Resource_policies](#resource_policies) [R]
- [Resource_share_invitations](#resource_share_invitations) [R]
- [Permission](#permission) [CRD]
- [Resource_shares](#resource_shares) [R]
- [Resource_share](#resource_share) [CUD]
- [Resource_share_associations](#resource_share_associations) [R]

---

## Resources


### Permission_version

PermissionVersion resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_token` | String |  | <p>Specifies a unique, case-sensitive identifier that you provide to
             ensure the idempotency of the request. This lets you safely retry the request without
             accidentally performing the same operation a second time. Passing the same value to a
             later call to an operation requires that you also pass the same value for all other 
             parameters. We recommend that you use a <a href="https://wikipedia.org/wiki/Universally_unique_identifier">UUID type of 
             value.</a>.</p>
         <p>If you don't provide this value, then Amazon Web Services generates a random one for
             you.</p>
         <p>If you retry the operation with the same <code>ClientToken</code>, but with 
             different parameters, the retry fails with an <code>IdempotentParameterMismatch</code>
             error.</p> |
| `permission_arn` | String | ✅ | <p>Specifies the <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Name (ARN)</a> of the customer managed permission you're creating a new version for.</p> |
| `policy_template` | String | ✅ | <p>A string in JSON format string that contains the following elements of a
            resource-based policy:</p>
         <ul>
            <li>
               <p>
                  <b>Effect</b>: must be set to
                    <code>ALLOW</code>.</p>
            </li>
            <li>
               <p>
                  <b>Action</b>: specifies the actions that are
                    allowed by this customer managed permission. The list must contain only actions that are supported by
                    the specified resource type. For a list of all actions supported by each
                    resource type, see <a href="https://docs.aws.amazon.com/service-authorization/latest/reference/reference_policies_actions-resources-contextkeys.html">Actions, resources, and condition keys for Amazon Web Services services</a> in the
                        <i>Identity and Access Management User Guide</i>.</p>
            </li>
            <li>
               <p>
                  <b>Condition</b>: (optional) specifies conditional 
                    parameters that must evaluate to true when a user attempts an action for that 
                    action to be allowed. For more information about the Condition element, see 
                    <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_condition.html">IAM
                        policies: Condition element</a> in the <i>Identity and Access Management User
                    Guide</i>.</p>
            </li>
         </ul>
         <p>This template can't include either the <code>Resource</code> or
            <code>Principal</code> elements. Those are both filled in by RAM when it instantiates 
            the resource-based policy on each resource shared using this managed permission. The 
            <code>Resource</code> comes from the ARN of the specific resource that you are sharing. 
            The <code>Principal</code> comes from the list of identities added to the resource 
            share.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create permission_version
permission_version = provider.ram.Permission_version {
    permission_arn = "value"  # <p>Specifies the <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Name (ARN)</a> of the customer managed permission you're creating a new version for.</p>
    policy_template = "value"  # <p>A string in JSON format string that contains the following elements of a
            resource-based policy:</p>
         <ul>
            <li>
               <p>
                  <b>Effect</b>: must be set to
                    <code>ALLOW</code>.</p>
            </li>
            <li>
               <p>
                  <b>Action</b>: specifies the actions that are
                    allowed by this customer managed permission. The list must contain only actions that are supported by
                    the specified resource type. For a list of all actions supported by each
                    resource type, see <a href="https://docs.aws.amazon.com/service-authorization/latest/reference/reference_policies_actions-resources-contextkeys.html">Actions, resources, and condition keys for Amazon Web Services services</a> in the
                        <i>Identity and Access Management User Guide</i>.</p>
            </li>
            <li>
               <p>
                  <b>Condition</b>: (optional) specifies conditional 
                    parameters that must evaluate to true when a user attempts an action for that 
                    action to be allowed. For more information about the Condition element, see 
                    <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_condition.html">IAM
                        policies: Condition element</a> in the <i>Identity and Access Management User
                    Guide</i>.</p>
            </li>
         </ul>
         <p>This template can't include either the <code>Resource</code> or
            <code>Principal</code> elements. Those are both filled in by RAM when it instantiates 
            the resource-based policy on each resource shared using this managed permission. The 
            <code>Resource</code> comes from the ARN of the specific resource that you are sharing. 
            The <code>Principal</code> comes from the list of identities added to the resource 
            share.</p>
}

```

---


### Resource_policies

ResourcePolicies resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `policies` | Vec<String> | <p>An array of resource policy documents in JSON format.</p> |
| `next_token` | String | <p>If present, this value indicates that more output is available than 
             is included in the current response. Use this value in the <code>NextToken</code> 
             request parameter in a subsequent call to the operation to get the next part of the 
             output. You should repeat this until the <code>NextToken</code> response element comes 
             back as <code>null</code>. This indicates that this is the last page of results.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access resource_policies outputs
resource_policies_id = resource_policies.id
resource_policies_policies = resource_policies.policies
resource_policies_next_token = resource_policies.next_token
```

---


### Resource_share_invitations

ResourceShareInvitations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>If present, this value indicates that more output is available than 
             is included in the current response. Use this value in the <code>NextToken</code> 
             request parameter in a subsequent call to the operation to get the next part of the 
             output. You should repeat this until the <code>NextToken</code> response element comes 
             back as <code>null</code>. This indicates that this is the last page of results.</p> |
| `resource_share_invitations` | Vec<String> | <p>An array of objects that contain the details about the invitations.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access resource_share_invitations outputs
resource_share_invitations_id = resource_share_invitations.id
resource_share_invitations_next_token = resource_share_invitations.next_token
resource_share_invitations_resource_share_invitations = resource_share_invitations.resource_share_invitations
```

---


### Permission

Permission resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `policy_template` | String | ✅ | <p>A string in JSON format string that contains the following elements of a
            resource-based policy:</p>
         <ul>
            <li>
               <p>
                  <b>Effect</b>: must be set to
                    <code>ALLOW</code>.</p>
            </li>
            <li>
               <p>
                  <b>Action</b>: specifies the actions that are
                    allowed by this customer managed permission. The list must contain only actions that are supported by
                    the specified resource type. For a list of all actions supported by each
                    resource type, see <a href="https://docs.aws.amazon.com/service-authorization/latest/reference/reference_policies_actions-resources-contextkeys.html">Actions, resources, and condition keys for Amazon Web Services services</a> in the
                        <i>Identity and Access Management User Guide</i>.</p>
            </li>
            <li>
               <p>
                  <b>Condition</b>: (optional) specifies conditional 
                    parameters that must evaluate to true when a user attempts an action for that 
                    action to be allowed. For more information about the Condition element, see 
                    <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_condition.html">IAM
                        policies: Condition element</a> in the <i>Identity and Access Management User
                    Guide</i>.</p>
            </li>
         </ul>
         <p>This template can't include either the <code>Resource</code> or
            <code>Principal</code> elements. Those are both filled in by RAM when it instantiates 
            the resource-based policy on each resource shared using this managed permission. The 
            <code>Resource</code> comes from the ARN of the specific resource that you are sharing. 
            The <code>Principal</code> comes from the list of identities added to the resource 
            share.</p> |
| `name` | String | ✅ | <p>Specifies the name of the customer managed permission. The name must be unique within the
            Amazon Web Services Region.</p> |
| `resource_type` | String | ✅ | <p>Specifies the name of the resource type that this customer managed permission applies to.</p>
         <p>The format is
                    <code>
               <i><service-code></i>:<i><resource-type></i>
            </code>
            and is not case sensitive. For example, to specify an Amazon EC2 Subnet, you can use the
            string <code>ec2:subnet</code>. To see the list of valid values for this parameter,
            query the <a>ListResourceTypes</a> operation.</p> |
| `client_token` | String |  | <p>Specifies a unique, case-sensitive identifier that you provide to
             ensure the idempotency of the request. This lets you safely retry the request without
             accidentally performing the same operation a second time. Passing the same value to a
             later call to an operation requires that you also pass the same value for all other 
             parameters. We recommend that you use a <a href="https://wikipedia.org/wiki/Universally_unique_identifier">UUID type of 
             value.</a>.</p>
         <p>If you don't provide this value, then Amazon Web Services generates a random one for
             you.</p>
         <p>If you retry the operation with the same <code>ClientToken</code>, but with 
             different parameters, the retry fails with an <code>IdempotentParameterMismatch</code>
             error.</p> |
| `tags` | Vec<String> |  | <p>Specifies a list of one or more tag key and value pairs to attach to the
            permission.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `permission` | String | <p>An object with details about the permission.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create permission
permission = provider.ram.Permission {
    policy_template = "value"  # <p>A string in JSON format string that contains the following elements of a
            resource-based policy:</p>
         <ul>
            <li>
               <p>
                  <b>Effect</b>: must be set to
                    <code>ALLOW</code>.</p>
            </li>
            <li>
               <p>
                  <b>Action</b>: specifies the actions that are
                    allowed by this customer managed permission. The list must contain only actions that are supported by
                    the specified resource type. For a list of all actions supported by each
                    resource type, see <a href="https://docs.aws.amazon.com/service-authorization/latest/reference/reference_policies_actions-resources-contextkeys.html">Actions, resources, and condition keys for Amazon Web Services services</a> in the
                        <i>Identity and Access Management User Guide</i>.</p>
            </li>
            <li>
               <p>
                  <b>Condition</b>: (optional) specifies conditional 
                    parameters that must evaluate to true when a user attempts an action for that 
                    action to be allowed. For more information about the Condition element, see 
                    <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_condition.html">IAM
                        policies: Condition element</a> in the <i>Identity and Access Management User
                    Guide</i>.</p>
            </li>
         </ul>
         <p>This template can't include either the <code>Resource</code> or
            <code>Principal</code> elements. Those are both filled in by RAM when it instantiates 
            the resource-based policy on each resource shared using this managed permission. The 
            <code>Resource</code> comes from the ARN of the specific resource that you are sharing. 
            The <code>Principal</code> comes from the list of identities added to the resource 
            share.</p>
    name = "value"  # <p>Specifies the name of the customer managed permission. The name must be unique within the
            Amazon Web Services Region.</p>
    resource_type = "value"  # <p>Specifies the name of the resource type that this customer managed permission applies to.</p>
         <p>The format is
                    <code>
               <i><service-code></i>:<i><resource-type></i>
            </code>
            and is not case sensitive. For example, to specify an Amazon EC2 Subnet, you can use the
            string <code>ec2:subnet</code>. To see the list of valid values for this parameter,
            query the <a>ListResourceTypes</a> operation.</p>
}

# Access permission outputs
permission_id = permission.id
permission_permission = permission.permission
```

---


### Resource_shares

ResourceShares resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `resource_shares` | Vec<String> | <p>An array of objects that contain the information about the resource shares.</p> |
| `next_token` | String | <p>If present, this value indicates that more output is available than 
             is included in the current response. Use this value in the <code>NextToken</code> 
             request parameter in a subsequent call to the operation to get the next part of the 
             output. You should repeat this until the <code>NextToken</code> response element comes 
             back as <code>null</code>. This indicates that this is the last page of results.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access resource_shares outputs
resource_shares_id = resource_shares.id
resource_shares_resource_shares = resource_shares.resource_shares
resource_shares_next_token = resource_shares.next_token
```

---


### Resource_share

ResourceShare resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>Specifies one or more tags to attach to the resource share itself. It doesn't attach the tags to
            the resources associated with the resource share.</p> |
| `sources` | Vec<String> |  | <p>Specifies from which source accounts the service principal
            has access to the resources in this resource share.</p> |
| `principals` | Vec<String> |  | <p>Specifies a list of one or more principals to associate with the resource share.</p>
         <p>You can include the following values:</p>
         <ul>
            <li>
               <p>An Amazon Web Services account ID, for example: <code>123456789012</code>
               </p>
            </li>
            <li>
               <p>An <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Name (ARN)</a> of an organization in Organizations, for example:
                        <code>organizations::123456789012:organization/o-exampleorgid</code>
               </p>
            </li>
            <li>
               <p>An ARN of an organizational unit (OU) in Organizations, for example:
                        <code>organizations::123456789012:ou/o-exampleorgid/ou-examplerootid-exampleouid123</code>
               </p>
            </li>
            <li>
               <p>An ARN of an IAM role, for example:
                        <code>iam::123456789012:role/rolename</code>
               </p>
            </li>
            <li>
               <p>An ARN of an IAM user, for example:
                        <code>iam::123456789012user/username</code>
               </p>
            </li>
         </ul>
         <note>
            <p>Not all resource types can be shared with IAM roles and users. 
             For more information, see <a href="https://docs.aws.amazon.com/ram/latest/userguide/permissions.html#permissions-rbp-supported-resource-types">Sharing with IAM roles and users</a> in the <i>Resource Access Manager User
                    Guide</i>.</p>
         </note> |
| `resource_arns` | Vec<String> |  | <p>Specifies a list of one or more ARNs of the resources to associate with the
            resource share.</p> |
| `client_token` | String |  | <p>Specifies a unique, case-sensitive identifier that you provide to
             ensure the idempotency of the request. This lets you safely retry the request without
             accidentally performing the same operation a second time. Passing the same value to a
             later call to an operation requires that you also pass the same value for all other 
             parameters. We recommend that you use a <a href="https://wikipedia.org/wiki/Universally_unique_identifier">UUID type of 
             value.</a>.</p>
         <p>If you don't provide this value, then Amazon Web Services generates a random one for
             you.</p>
         <p>If you retry the operation with the same <code>ClientToken</code>, but with 
             different parameters, the retry fails with an <code>IdempotentParameterMismatch</code>
             error.</p> |
| `allow_external_principals` | bool |  | <p>Specifies whether principals outside your organization in Organizations can be associated
            with a resource share. A value of <code>true</code> lets you share with individual Amazon Web Services accounts
            that are <i>not</i> in your organization. A value of <code>false</code>
            only has meaning if your account is a member of an Amazon Web Services Organization. The default value
            is <code>true</code>.</p> |
| `name` | String | ✅ | <p>Specifies the name of the resource share.</p> |
| `permission_arns` | Vec<String> |  | <p>Specifies the <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> of the RAM permission to associate with the resource share. If you do
            not specify an ARN for the permission, RAM automatically attaches the default version
            of the permission for each resource type. You can associate only one permission with
            each resource type included in the resource share.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create resource_share
resource_share = provider.ram.Resource_share {
    name = "value"  # <p>Specifies the name of the resource share.</p>
}

```

---


### Resource_share_associations

ResourceShareAssociations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>If present, this value indicates that more output is available than 
             is included in the current response. Use this value in the <code>NextToken</code> 
             request parameter in a subsequent call to the operation to get the next part of the 
             output. You should repeat this until the <code>NextToken</code> response element comes 
             back as <code>null</code>. This indicates that this is the last page of results.</p> |
| `resource_share_associations` | Vec<String> | <p>An array of objects that contain the details about the associations.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access resource_share_associations outputs
resource_share_associations_id = resource_share_associations.id
resource_share_associations_next_token = resource_share_associations.next_token
resource_share_associations_resource_share_associations = resource_share_associations.resource_share_associations
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple permission_version resources
permission_version_0 = provider.ram.Permission_version {
    permission_arn = "value-0"
    policy_template = "value-0"
}
permission_version_1 = provider.ram.Permission_version {
    permission_arn = "value-1"
    policy_template = "value-1"
}
permission_version_2 = provider.ram.Permission_version {
    permission_arn = "value-2"
    policy_template = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    permission_version = provider.ram.Permission_version {
        permission_arn = "production-value"
        policy_template = "production-value"
    }
```

---

## Related Documentation

- [AWS Ram Documentation](https://docs.aws.amazon.com/ram/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
