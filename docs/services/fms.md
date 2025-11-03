# Fms Service



**Resources**: 11

---

## Overview

The fms service provides access to 11 resource types:

- [Third_party_firewall_association_status](#third_party_firewall_association_status) [R]
- [Protection_status](#protection_status) [R]
- [Admin_scope](#admin_scope) [R]
- [Compliance_detail](#compliance_detail) [R]
- [Admin_account](#admin_account) [CR]
- [Notification_channel](#notification_channel) [CRD]
- [Protocols_list](#protocols_list) [CRD]
- [Apps_list](#apps_list) [CRD]
- [Policy](#policy) [CRD]
- [Violation_details](#violation_details) [R]
- [Resource_set](#resource_set) [CRD]

---

## Resources


### Third_party_firewall_association_status

ThirdPartyFirewallAssociationStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `marketplace_onboarding_status` | String | <p>The status for subscribing to the third-party firewall vendor in the Amazon Web Services Marketplace.</p>
         <ul>
            <li>
               <p>
                  <code>NO_SUBSCRIPTION</code> - The Firewall Manager policy administrator isn't subscribed to the third-party firewall service in the Amazon Web Services Marketplace.</p>
            </li>
            <li>
               <p>
                  <code>NOT_COMPLETE</code> - The Firewall Manager policy administrator is in the process of subscribing to the third-party firewall service in the Amazon Web Services Marketplace, but doesn't yet have an active subscription.</p>
            </li>
            <li>
               <p>
                  <code>COMPLETE</code> - The Firewall Manager policy administrator has an active subscription to the third-party firewall service in the Amazon Web Services Marketplace.</p>
            </li>
         </ul> |
| `third_party_firewall_status` | String | <p>The current status for setting a Firewall Manager policy administrators account as an administrator of the third-party firewall tenant.</p>
         <ul>
            <li>
               <p>
                  <code>ONBOARDING</code> - The Firewall Manager policy administrator is being designated as a tenant administrator.</p>
            </li>
            <li>
               <p>
                  <code>ONBOARD_COMPLETE</code> - The Firewall Manager policy administrator is designated as a tenant administrator.</p>
            </li>
            <li>
               <p>
                  <code>OFFBOARDING</code> - The Firewall Manager policy administrator is being removed as a tenant administrator.</p>
            </li>
            <li>
               <p>
                  <code>OFFBOARD_COMPLETE</code> - The Firewall Manager policy administrator has been removed as a tenant administrator.</p>
            </li>
            <li>
               <p>
                  <code>NOT_EXIST</code> - The Firewall Manager policy administrator doesn't exist as a tenant administrator.</p>
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

# Access third_party_firewall_association_status outputs
third_party_firewall_association_status_id = third_party_firewall_association_status.id
third_party_firewall_association_status_marketplace_onboarding_status = third_party_firewall_association_status.marketplace_onboarding_status
third_party_firewall_association_status_third_party_firewall_status = third_party_firewall_association_status.third_party_firewall_status
```

---


### Protection_status

ProtectionStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `service_type` | String | <p>The service type that is protected by the policy. Currently, this is always
            <code>SHIELD_ADVANCED</code>.</p> |
| `admin_account_id` | String | <p>The ID of the Firewall Manager administrator account for this policy.</p> |
| `data` | String | <p>Details about the attack, including the following:</p>
         <ul>
            <li>
               <p>Attack type</p>
            </li>
            <li>
               <p>Account ID</p>
            </li>
            <li>
               <p>ARN of the resource attacked</p>
            </li>
            <li>
               <p>Start time of the attack</p>
            </li>
            <li>
               <p>End time of the attack (ongoing attacks will not have an end time)</p>
            </li>
         </ul>
         <p>The details are in JSON format. </p> |
| `next_token` | String | <p>If you have more objects than the number that you specified for <code>MaxResults</code> in the request,
         the response includes a <code>NextToken</code> value. To list more objects, submit another
         <code>GetProtectionStatus</code> request, and specify the <code>NextToken</code> value from the response in the
         <code>NextToken</code> value in the next request.</p>
         <p>Amazon Web Services SDKs provide auto-pagination that identify <code>NextToken</code> in a response and
         make subsequent request calls automatically on your behalf. However, this feature is not
         supported by <code>GetProtectionStatus</code>. You must submit subsequent requests with
            <code>NextToken</code> using your own processes. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access protection_status outputs
protection_status_id = protection_status.id
protection_status_service_type = protection_status.service_type
protection_status_admin_account_id = protection_status.admin_account_id
protection_status_data = protection_status.data
protection_status_next_token = protection_status.next_token
```

---


### Admin_scope

AdminScope resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `admin_scope` | String | <p>Contains details about the administrative scope of the requested account.</p> |
| `status` | String | <p>The current status of the request to onboard a member account as an Firewall Manager administrator.</p>
         <ul>
            <li>
               <p>
                  <code>ONBOARDING</code> - The account is onboarding to Firewall Manager as an administrator.</p>
            </li>
            <li>
               <p>
                  <code>ONBOARDING_COMPLETE</code> - Firewall Manager The account is onboarded to Firewall Manager as an administrator, and can perform actions on the resources defined in their <a>AdminScope</a>.</p>
            </li>
            <li>
               <p>
                  <code>OFFBOARDING</code> - The account is being removed as an Firewall Manager administrator.</p>
            </li>
            <li>
               <p>
                  <code>OFFBOARDING_COMPLETE</code> - The account has been removed as an Firewall Manager administrator.</p>
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

# Access admin_scope outputs
admin_scope_id = admin_scope.id
admin_scope_admin_scope = admin_scope.admin_scope
admin_scope_status = admin_scope.status
```

---


### Compliance_detail

ComplianceDetail resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `policy_compliance_detail` | String | <p>Information about the resources and the policy that you specified in the
        <code>GetComplianceDetail</code> request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access compliance_detail outputs
compliance_detail_id = compliance_detail.id
compliance_detail_policy_compliance_detail = compliance_detail.policy_compliance_detail
```

---


### Admin_account

AdminAccount resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `admin_account` | String | ✅ | <p>The Amazon Web Services account ID to add as an Firewall Manager administrator account. The account must be a member of the organization that was onboarded to Firewall Manager by <a>AssociateAdminAccount</a>. For more information about Organizations, see
        <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts.html">Managing the Amazon Web Services Accounts in Your Organization</a>.</p> |
| `admin_scope` | String |  | <p>Configures the resources that the specified Firewall Manager administrator can manage. As a best practice, set the administrative scope according to the principles of least privilege. Only grant the administrator the specific resources or permissions that they need to perform the duties of their role.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `role_status` | String | <p>The status of the account that you set as the Firewall Manager
      default administrator.</p> |
| `admin_account` | String | <p>The account that is set as the Firewall Manager default administrator.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create admin_account
admin_account = provider.fms.Admin_account {
    admin_account = "value"  # <p>The Amazon Web Services account ID to add as an Firewall Manager administrator account. The account must be a member of the organization that was onboarded to Firewall Manager by <a>AssociateAdminAccount</a>. For more information about Organizations, see
        <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts.html">Managing the Amazon Web Services Accounts in Your Organization</a>.</p>
}

# Access admin_account outputs
admin_account_id = admin_account.id
admin_account_role_status = admin_account.role_status
admin_account_admin_account = admin_account.admin_account
```

---


### Notification_channel

NotificationChannel resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `sns_role_name` | String | ✅ | <p>The Amazon Resource Name (ARN) of the IAM role that allows Amazon SNS to record
      Firewall Manager activity. </p> |
| `sns_topic_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the SNS topic that collects notifications from
      Firewall Manager.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `sns_role_name` | String | <p>The IAM role that is used by Firewall Manager to record activity to SNS.</p> |
| `sns_topic_arn` | String | <p>The SNS topic that records Firewall Manager activity. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create notification_channel
notification_channel = provider.fms.Notification_channel {
    sns_role_name = "value"  # <p>The Amazon Resource Name (ARN) of the IAM role that allows Amazon SNS to record
      Firewall Manager activity. </p>
    sns_topic_arn = "value"  # <p>The Amazon Resource Name (ARN) of the SNS topic that collects notifications from
      Firewall Manager.</p>
}

# Access notification_channel outputs
notification_channel_id = notification_channel.id
notification_channel_sns_role_name = notification_channel.sns_role_name
notification_channel_sns_topic_arn = notification_channel.sns_topic_arn
```

---


### Protocols_list

ProtocolsList resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `protocols_list` | String | ✅ | <p>The details of the Firewall Manager protocols list to be created.</p> |
| `tag_list` | Vec<String> |  | <p>The tags associated with the resource.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `protocols_list` | String | <p>Information about the specified Firewall Manager protocols list.</p> |
| `protocols_list_arn` | String | <p>The Amazon Resource Name (ARN) of the specified protocols list.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create protocols_list
protocols_list = provider.fms.Protocols_list {
    protocols_list = "value"  # <p>The details of the Firewall Manager protocols list to be created.</p>
}

# Access protocols_list outputs
protocols_list_id = protocols_list.id
protocols_list_protocols_list = protocols_list.protocols_list
protocols_list_protocols_list_arn = protocols_list.protocols_list_arn
```

---


### Apps_list

AppsList resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `apps_list` | String | ✅ | <p>The details of the Firewall Manager applications list to be created.</p> |
| `tag_list` | Vec<String> |  | <p>The tags associated with the resource.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `apps_list_arn` | String | <p>The Amazon Resource Name (ARN) of the applications list.</p> |
| `apps_list` | String | <p>Information about the specified Firewall Manager applications list.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create apps_list
apps_list = provider.fms.Apps_list {
    apps_list = "value"  # <p>The details of the Firewall Manager applications list to be created.</p>
}

# Access apps_list outputs
apps_list_id = apps_list.id
apps_list_apps_list_arn = apps_list.apps_list_arn
apps_list_apps_list = apps_list.apps_list
```

---


### Policy

Policy resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tag_list` | Vec<String> |  | <p>The tags to add to the Amazon Web Services resource.</p> |
| `policy` | String | ✅ | <p>The details of the Firewall Manager policy to be created.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `policy_arn` | String | <p>The Amazon Resource Name (ARN) of the specified policy.</p> |
| `policy` | String | <p>Information about the specified Firewall Manager policy.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create policy
policy = provider.fms.Policy {
    policy = "value"  # <p>The details of the Firewall Manager policy to be created.</p>
}

# Access policy outputs
policy_id = policy.id
policy_policy_arn = policy.policy_arn
policy_policy = policy.policy
```

---


### Violation_details

ViolationDetails resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `violation_detail` | String | <p>Violation detail for a resource.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access violation_details outputs
violation_details_id = violation_details.id
violation_details_violation_detail = violation_details.violation_detail
```

---


### Resource_set

ResourceSet resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resource_set` | String | ✅ | <p>Details about the resource set to be created or updated.></p> |
| `tag_list` | Vec<String> |  | <p>Retrieves the tags associated with the specified resource set. Tags are key:value pairs that
         you can use to categorize and manage your resources, for purposes like billing. For
         example, you might set the tag key to "customer" and the value to the customer name or ID.
         You can specify one or more tags to add to each Amazon Web Services resource, up to 50 tags for a
         resource.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `resource_set` | String | <p>Information about the specified resource set.</p> |
| `resource_set_arn` | String | <p>The Amazon Resource Name (ARN) of the resource set.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create resource_set
resource_set = provider.fms.Resource_set {
    resource_set = "value"  # <p>Details about the resource set to be created or updated.></p>
}

# Access resource_set outputs
resource_set_id = resource_set.id
resource_set_resource_set = resource_set.resource_set
resource_set_resource_set_arn = resource_set.resource_set_arn
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple third_party_firewall_association_status resources
third_party_firewall_association_status_0 = provider.fms.Third_party_firewall_association_status {
}
third_party_firewall_association_status_1 = provider.fms.Third_party_firewall_association_status {
}
third_party_firewall_association_status_2 = provider.fms.Third_party_firewall_association_status {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    third_party_firewall_association_status = provider.fms.Third_party_firewall_association_status {
    }
```

---

## Related Documentation

- [AWS Fms Documentation](https://docs.aws.amazon.com/fms/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
