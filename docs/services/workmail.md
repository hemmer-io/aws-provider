# Workmail Service



**Resources**: 29

---

## Overview

The workmail service provides access to 29 resource types:

- [Primary_email_address](#primary_email_address) [U]
- [Mobile_device_access_rule](#mobile_device_access_rule) [CUD]
- [Access_control_rule](#access_control_rule) [CD]
- [Organization](#organization) [CRD]
- [Impersonation_role](#impersonation_role) [CRUD]
- [Resource](#resource) [CRUD]
- [Identity_center_application](#identity_center_application) [CD]
- [Access_control_effect](#access_control_effect) [R]
- [Email_monitoring_configuration](#email_monitoring_configuration) [CRD]
- [Default_mail_domain](#default_mail_domain) [U]
- [Mailbox_quota](#mailbox_quota) [U]
- [Retention_policy](#retention_policy) [CD]
- [Personal_access_token](#personal_access_token) [D]
- [Default_retention_policy](#default_retention_policy) [R]
- [Availability_configuration](#availability_configuration) [CUD]
- [Mail_domain](#mail_domain) [R]
- [Mailbox_details](#mailbox_details) [R]
- [Mobile_device_access_effect](#mobile_device_access_effect) [R]
- [User](#user) [CRUD]
- [Identity_provider_configuration](#identity_provider_configuration) [CRD]
- [Inbound_dmarc_settings](#inbound_dmarc_settings) [CR]
- [Personal_access_token_metadata](#personal_access_token_metadata) [R]
- [Group](#group) [CRUD]
- [Alias](#alias) [CD]
- [Impersonation_role_effect](#impersonation_role_effect) [R]
- [Entity](#entity) [R]
- [Mobile_device_access_override](#mobile_device_access_override) [CRD]
- [Mailbox_permissions](#mailbox_permissions) [CD]
- [Mailbox_export_job](#mailbox_export_job) [R]

---

## Resources


### Primary_email_address

PrimaryEmailAddress resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `entity_id` | String | ✅ | <p>The user, group, or resource to update.</p>
         <p>The identifier can accept <i>UseriD, ResourceId, or GroupId</i>, <i>Username, Resourcename, or Groupname</i>, or <i>email</i>. The following identity formats are available:</p>
         <ul>
            <li>
               <p>Entity ID: 12345678-1234-1234-1234-123456789012, r-0123456789a0123456789b0123456789, or S-1-1-12-1234567890-123456789-123456789-1234</p>
            </li>
            <li>
               <p>Email address: entity@domain.tld</p>
            </li>
            <li>
               <p>Entity name: entity</p>
            </li>
         </ul> |
| `email` | String | ✅ | <p>The value of the email to be updated as primary.</p> |
| `organization_id` | String | ✅ | <p>The organization that contains the user, group, or resource to update.</p> |



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


### Mobile_device_access_rule

MobileDeviceAccessRule resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `organization_id` | String | ✅ | <p>The WorkMail organization under which the rule will be created.</p> |
| `effect` | String | ✅ | <p>The effect of the rule when it matches. Allowed values are <code>ALLOW</code> or <code>DENY</code>.</p> |
| `not_device_types` | Vec<String> |  | <p>Device types that the rule <b>will not</b> match. All other device types will match.</p> |
| `not_device_user_agents` | Vec<String> |  | <p>Device user agents that the rule <b>will not</b> match. All other device user agents will match.</p> |
| `device_types` | Vec<String> |  | <p>Device types that the rule will match.</p> |
| `device_models` | Vec<String> |  | <p>Device models that the rule will match.</p> |
| `not_device_models` | Vec<String> |  | <p>Device models that the rule <b>will not</b> match. All other device models will match.</p> |
| `name` | String | ✅ | <p>The rule name.</p> |
| `device_operating_systems` | Vec<String> |  | <p>Device operating systems that the rule will match.</p> |
| `device_user_agents` | Vec<String> |  | <p>Device user agents that the rule will match.</p> |
| `description` | String |  | <p>The rule description.</p> |
| `client_token` | String |  | <p>The idempotency token for the client request.</p> |
| `not_device_operating_systems` | Vec<String> |  | <p>Device operating systems that the rule <b>will not</b> match. All other device operating systems will match.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create mobile_device_access_rule
mobile_device_access_rule = provider.workmail.Mobile_device_access_rule {
    organization_id = "value"  # <p>The WorkMail organization under which the rule will be created.</p>
    effect = "value"  # <p>The effect of the rule when it matches. Allowed values are <code>ALLOW</code> or <code>DENY</code>.</p>
    name = "value"  # <p>The rule name.</p>
}

```

---


### Access_control_rule

AccessControlRule resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `actions` | Vec<String> |  | <p>Access protocol actions to include in the rule. Valid values include
            <code>ActiveSync</code>, <code>AutoDiscover</code>, <code>EWS</code>, <code>IMAP</code>,
            <code>SMTP</code>, <code>WindowsOutlook</code>, and <code>WebMail</code>.</p> |
| `ip_ranges` | Vec<String> |  | <p>IPv4 CIDR ranges to include in the rule.</p> |
| `name` | String | ✅ | <p>The rule name.</p> |
| `not_actions` | Vec<String> |  | <p>Access protocol actions to exclude from the rule. Valid values include
            <code>ActiveSync</code>, <code>AutoDiscover</code>, <code>EWS</code>, <code>IMAP</code>,
            <code>SMTP</code>, <code>WindowsOutlook</code>, and <code>WebMail</code>.</p> |
| `impersonation_role_ids` | Vec<String> |  | <p>Impersonation role IDs to include in the rule.</p> |
| `not_ip_ranges` | Vec<String> |  | <p>IPv4 CIDR ranges to exclude from the rule.</p> |
| `user_ids` | Vec<String> |  | <p>User IDs to include in the rule.</p> |
| `not_impersonation_role_ids` | Vec<String> |  | <p>Impersonation role IDs to exclude from the rule.</p> |
| `effect` | String | ✅ | <p>The rule effect.</p> |
| `description` | String | ✅ | <p>The rule description.</p> |
| `not_user_ids` | Vec<String> |  | <p>User IDs to exclude from the rule.</p> |
| `organization_id` | String | ✅ | <p>The identifier of the organization.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create access_control_rule
access_control_rule = provider.workmail.Access_control_rule {
    name = "value"  # <p>The rule name.</p>
    effect = "value"  # <p>The rule effect.</p>
    description = "value"  # <p>The rule description.</p>
    organization_id = "value"  # <p>The identifier of the organization.</p>
}

```

---


### Organization

Organization resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `alias` | String | ✅ | <p>The organization alias.</p> |
| `client_token` | String |  | <p>The idempotency token associated with the request.</p> |
| `kms_key_arn` | String |  | <p>The Amazon Resource Name (ARN) of a customer managed key from AWS KMS.</p> |
| `enable_interoperability` | bool |  | <p>When <code>true</code>, allows organization interoperability between WorkMail and
         Microsoft Exchange. If <code>true</code>, you must include a AD Connector directory ID in
         the request.</p> |
| `domains` | Vec<String> |  | <p>The email domains to associate with the organization.</p> |
| `directory_id` | String |  | <p>The AWS Directory Service directory ID.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `organization_id` | String | <p>The identifier of an organization.</p> |
| `directory_id` | String | <p>The identifier for the directory associated with an WorkMail organization.</p> |
| `default_mail_domain` | String | <p>The default mail domain associated with the organization.</p> |
| `completed_date` | String | <p>The date at which the organization became usable in the WorkMail context, in UNIX epoch
         time format.</p> |
| `state` | String | <p>The state of an organization.</p> |
| `arn` | String | <p>The Amazon Resource Name (ARN) of the organization.</p> |
| `migration_admin` | String | <p>The user ID of the migration admin if migration is enabled for the organization.</p> |
| `interoperability_enabled` | bool | <p>Indicates if interoperability is enabled for this organization.</p> |
| `alias` | String | <p>The alias for an organization.</p> |
| `error_message` | String | <p>(Optional) The error message indicating if unexpected behavior was encountered with
         regards to the organization.</p> |
| `directory_type` | String | <p>The type of directory associated with the WorkMail organization.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create organization
organization = provider.workmail.Organization {
    alias = "value"  # <p>The organization alias.</p>
}

# Access organization outputs
organization_id = organization.id
organization_organization_id = organization.organization_id
organization_directory_id = organization.directory_id
organization_default_mail_domain = organization.default_mail_domain
organization_completed_date = organization.completed_date
organization_state = organization.state
organization_arn = organization.arn
organization_migration_admin = organization.migration_admin
organization_interoperability_enabled = organization.interoperability_enabled
organization_alias = organization.alias
organization_error_message = organization.error_message
organization_directory_type = organization.directory_type
```

---


### Impersonation_role

ImpersonationRole resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_token` | String |  | <p>The idempotency token for the client request.</p> |
| `name` | String | ✅ | <p>The name of the new impersonation role.</p> |
| `type` | String | ✅ | <p>The impersonation role's type. The available impersonation role types are
            <code>READ_ONLY</code> or <code>FULL_ACCESS</code>.</p> |
| `organization_id` | String | ✅ | <p>The WorkMail organization to create the new impersonation role within.</p> |
| `rules` | Vec<String> | ✅ | <p>The list of rules for the impersonation role.</p> |
| `description` | String |  | <p>The description of the new impersonation role.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `date_modified` | String | <p>The date when the impersonation role was last modified.</p> |
| `description` | String | <p>The impersonation role description.</p> |
| `impersonation_role_id` | String | <p>The impersonation role ID.</p> |
| `rules` | Vec<String> | <p>The list of rules for the given impersonation role.</p> |
| `date_created` | String | <p>The date when the impersonation role was created.</p> |
| `name` | String | <p>The impersonation role name.</p> |
| `type` | String | <p>The impersonation role type.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create impersonation_role
impersonation_role = provider.workmail.Impersonation_role {
    name = "value"  # <p>The name of the new impersonation role.</p>
    type = "value"  # <p>The impersonation role's type. The available impersonation role types are
            <code>READ_ONLY</code> or <code>FULL_ACCESS</code>.</p>
    organization_id = "value"  # <p>The WorkMail organization to create the new impersonation role within.</p>
    rules = "value"  # <p>The list of rules for the impersonation role.</p>
}

# Access impersonation_role outputs
impersonation_role_id = impersonation_role.id
impersonation_role_date_modified = impersonation_role.date_modified
impersonation_role_description = impersonation_role.description
impersonation_role_impersonation_role_id = impersonation_role.impersonation_role_id
impersonation_role_rules = impersonation_role.rules
impersonation_role_date_created = impersonation_role.date_created
impersonation_role_name = impersonation_role.name
impersonation_role_type = impersonation_role.type
```

---


### Resource

Resource resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>The name of the new resource.</p> |
| `description` | String |  | <p>Resource description.</p> |
| `type` | String | ✅ | <p>The type of the new resource. The available types are <code>equipment</code> and
            <code>room</code>.</p> |
| `hidden_from_global_address_list` | bool |  | <p>If this parameter is enabled, the resource will be hidden from the address book.</p> |
| `organization_id` | String | ✅ | <p>The identifier associated with the organization for which the resource is
         created.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `email` | String | <p>The email of the described resource.</p> |
| `enabled_date` | String | <p>The date and time when a resource was enabled for WorkMail, in UNIX epoch time
         format.</p> |
| `disabled_date` | String | <p>The date and time when a resource was disabled from WorkMail, in UNIX epoch time
         format.</p> |
| `state` | String | <p>The state of the resource: enabled (registered to WorkMail), disabled (deregistered
         or never registered to WorkMail), or deleted.</p> |
| `name` | String | <p>The name of the described resource.</p> |
| `type` | String | <p>The type of the described resource.</p> |
| `description` | String | <p>Description of the resource.</p> |
| `booking_options` | String | <p>The booking options for the described resource.</p> |
| `resource_id` | String | <p>The identifier of the described resource.</p> |
| `hidden_from_global_address_list` | bool | <p>If enabled, the resource is hidden from the global address list.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create resource
resource = provider.workmail.Resource {
    name = "value"  # <p>The name of the new resource.</p>
    type = "value"  # <p>The type of the new resource. The available types are <code>equipment</code> and
            <code>room</code>.</p>
    organization_id = "value"  # <p>The identifier associated with the organization for which the resource is
         created.</p>
}

# Access resource outputs
resource_id = resource.id
resource_email = resource.email
resource_enabled_date = resource.enabled_date
resource_disabled_date = resource.disabled_date
resource_state = resource.state
resource_name = resource.name
resource_type = resource.type
resource_description = resource.description
resource_booking_options = resource.booking_options
resource_resource_id = resource.resource_id
resource_hidden_from_global_address_list = resource.hidden_from_global_address_list
```

---


### Identity_center_application

IdentityCenterApplication resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_token` | String |  | <p>
         The idempotency token associated with the request.
         
         
      </p> |
| `name` | String | ✅ | <p>
         The name of the IAM Identity Center application.
         
         
      </p> |
| `instance_arn` | String | ✅ | <p>
         The Amazon Resource Name (ARN) of the instance.
      </p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create identity_center_application
identity_center_application = provider.workmail.Identity_center_application {
    name = "value"  # <p>
         The name of the IAM Identity Center application.
         
         
      </p>
    instance_arn = "value"  # <p>
         The Amazon Resource Name (ARN) of the instance.
      </p>
}

```

---


### Access_control_effect

AccessControlEffect resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `effect` | String | <p>The rule effect.</p> |
| `matched_rules` | Vec<String> | <p>The rules that match the given parameters, resulting in an effect.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access access_control_effect outputs
access_control_effect_id = access_control_effect.id
access_control_effect_effect = access_control_effect.effect
access_control_effect_matched_rules = access_control_effect.matched_rules
```

---


### Email_monitoring_configuration

EmailMonitoringConfiguration resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `organization_id` | String | ✅ | <p>The ID of the organization for which the email monitoring configuration is set.</p> |
| `role_arn` | String |  | <p>The Amazon Resource Name (ARN) of the IAM Role associated with the email monitoring configuration. If absent, the IAM Role Arn of AWSServiceRoleForAmazonWorkMailEvents will be used.</p> |
| `log_group_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the CloudWatch Log group associated with the email monitoring configuration.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `log_group_arn` | String | <p>The Amazon Resource Name (ARN) of the CloudWatch Log group associated with the email monitoring configuration.</p> |
| `role_arn` | String | <p>The Amazon Resource Name (ARN) of the IAM Role associated with the email monitoring configuration.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create email_monitoring_configuration
email_monitoring_configuration = provider.workmail.Email_monitoring_configuration {
    organization_id = "value"  # <p>The ID of the organization for which the email monitoring configuration is set.</p>
    log_group_arn = "value"  # <p>The Amazon Resource Name (ARN) of the CloudWatch Log group associated with the email monitoring configuration.</p>
}

# Access email_monitoring_configuration outputs
email_monitoring_configuration_id = email_monitoring_configuration.id
email_monitoring_configuration_log_group_arn = email_monitoring_configuration.log_group_arn
email_monitoring_configuration_role_arn = email_monitoring_configuration.role_arn
```

---


### Default_mail_domain

DefaultMailDomain resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `domain_name` | String | ✅ | <p>The domain name that will become the default domain.</p> |
| `organization_id` | String | ✅ | <p>The WorkMail organization for which to list domains.</p> |



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


### Mailbox_quota

MailboxQuota resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `user_id` | String | ✅ | <p>The identifer for the user for whom to update the mailbox quota.</p>
         <p>The identifier can be the <i>UserId</i>, <i>Username</i>, or <i>email</i>. The following identity formats are available:</p>
         <ul>
            <li>
               <p>User ID: 12345678-1234-1234-1234-123456789012 or S-1-1-12-1234567890-123456789-123456789-1234</p>
            </li>
            <li>
               <p>Email address: user@domain.tld</p>
            </li>
            <li>
               <p>User name: user</p>
            </li>
         </ul> |
| `mailbox_quota` | i64 | ✅ | <p>The updated mailbox quota, in MB, for the specified user.</p> |
| `organization_id` | String | ✅ | <p>The identifier for the organization that contains the user for whom to update the
         mailbox quota.</p> |



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


### Retention_policy

RetentionPolicy resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `organization_id` | String | ✅ | <p>The organization ID.</p> |
| `name` | String | ✅ | <p>The retention policy name.</p> |
| `description` | String |  | <p>The retention policy description.</p> |
| `id` | String |  | <p>The retention policy ID.</p> |
| `folder_configurations` | Vec<String> | ✅ | <p>The retention policy folder configurations.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create retention_policy
retention_policy = provider.workmail.Retention_policy {
    organization_id = "value"  # <p>The organization ID.</p>
    name = "value"  # <p>The retention policy name.</p>
    folder_configurations = "value"  # <p>The retention policy folder configurations.</p>
}

```

---


### Personal_access_token

PersonalAccessToken resource

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


### Default_retention_policy

DefaultRetentionPolicy resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `id` | String | <p>The retention policy ID.</p> |
| `folder_configurations` | Vec<String> | <p>The retention policy folder configurations.</p> |
| `name` | String | <p>The retention policy name.</p> |
| `description` | String | <p>The retention policy description.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access default_retention_policy outputs
default_retention_policy_id = default_retention_policy.id
default_retention_policy_id = default_retention_policy.id
default_retention_policy_folder_configurations = default_retention_policy.folder_configurations
default_retention_policy_name = default_retention_policy.name
default_retention_policy_description = default_retention_policy.description
```

---


### Availability_configuration

AvailabilityConfiguration resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `ews_provider` | String |  | <p>Exchange Web Services (EWS) availability provider definition. The request must contain exactly one provider definition, either <code>EwsProvider</code> or <code>LambdaProvider</code>.</p> |
| `client_token` | String |  | <p>An idempotent token that ensures that an API request is executed only once.</p> |
| `organization_id` | String | ✅ | <p>The WorkMail organization for which the <code>AvailabilityConfiguration</code> will be created.</p> |
| `domain_name` | String | ✅ | <p>The domain to which the provider applies.</p> |
| `lambda_provider` | String |  | <p>Lambda availability provider definition. The request must contain exactly one provider definition, either <code>EwsProvider</code> or <code>LambdaProvider</code>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create availability_configuration
availability_configuration = provider.workmail.Availability_configuration {
    organization_id = "value"  # <p>The WorkMail organization for which the <code>AvailabilityConfiguration</code> will be created.</p>
    domain_name = "value"  # <p>The domain to which the provider applies.</p>
}

```

---


### Mail_domain

MailDomain resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `is_default` | bool | <p>Specifies whether the domain is the default domain for your organization.</p> |
| `records` | Vec<String> | <p>A list of the DNS records that WorkMail recommends adding in your DNS provider for the best user experience. The records configure your domain with DMARC, SPF, DKIM, and direct incoming 
         email traffic to SES. See admin guide for more details.</p> |
| `ownership_verification_status` | String | <p> Indicates the status of the domain ownership verification.</p> |
| `is_test_domain` | bool | <p>Specifies whether the domain is a test domain provided by WorkMail, or a custom domain.</p> |
| `dkim_verification_status` | String | <p>Indicates the status of a DKIM verification.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access mail_domain outputs
mail_domain_id = mail_domain.id
mail_domain_is_default = mail_domain.is_default
mail_domain_records = mail_domain.records
mail_domain_ownership_verification_status = mail_domain.ownership_verification_status
mail_domain_is_test_domain = mail_domain.is_test_domain
mail_domain_dkim_verification_status = mail_domain.dkim_verification_status
```

---


### Mailbox_details

MailboxDetails resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `mailbox_quota` | i64 | <p>The maximum allowed mailbox size, in MB, for the specified user.</p> |
| `mailbox_size` | f64 | <p>The current mailbox size, in MB, for the specified user.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access mailbox_details outputs
mailbox_details_id = mailbox_details.id
mailbox_details_mailbox_quota = mailbox_details.mailbox_quota
mailbox_details_mailbox_size = mailbox_details.mailbox_size
```

---


### Mobile_device_access_effect

MobileDeviceAccessEffect resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `effect` | String | <p>The effect of the simulated access, <code>ALLOW</code> or <code>DENY</code>, after evaluating mobile device access rules in the WorkMail organization for the simulated 
         user parameters.</p> |
| `matched_rules` | Vec<String> | <p>A list of the rules which matched the simulated user input and produced the effect.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access mobile_device_access_effect outputs
mobile_device_access_effect_id = mobile_device_access_effect.id
mobile_device_access_effect_effect = mobile_device_access_effect.effect
mobile_device_access_effect_matched_rules = mobile_device_access_effect.matched_rules
```

---


### User

User resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `hidden_from_global_address_list` | bool |  | <p>If this parameter is enabled, the user will be hidden from the address book.</p> |
| `display_name` | String | ✅ | <p>The display name for the new user.</p> |
| `first_name` | String |  | <p>The first name of the new user.</p> |
| `role` | String |  | <p>The role of the new user.</p>
         <p>You cannot pass <i>SYSTEM_USER</i> or <i>RESOURCE</i> role in a single request. When a user role is not selected, the default role of <i>USER</i> is selected.</p> |
| `organization_id` | String | ✅ | <p>The identifier of the organization for which the user is created.</p> |
| `password` | String |  | <p>The password for the new user.</p> |
| `name` | String | ✅ | <p>The name for the new user. WorkMail directory user names have a maximum length of 64. All others have a maximum length of 20.</p> |
| `identity_provider_user_id` | String |  | <p>User ID from the IAM Identity Center. If this parameter is empty it will be updated automatically when the user logs in for the first time to the mailbox associated with WorkMail.</p> |
| `last_name` | String |  | <p>The last name of the new user. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `mailbox_provisioned_date` | String | <p>The date when the mailbox was created for the user.</p> |
| `city` | String | <p>City where the user is located.</p> |
| `country` | String | <p>Country where the user is located.</p> |
| `enabled_date` | String | <p>The date and time at which the user was enabled for WorkMailusage, in UNIX epoch
         time format.</p> |
| `identity_provider_user_id` | String | <p>User ID from the IAM Identity Center. If this parameter is empty it will be updated automatically when the user logs in for the first time to the mailbox associated with WorkMail.</p> |
| `disabled_date` | String | <p>The date and time at which the user was disabled for WorkMail usage, in UNIX epoch
         time format.</p> |
| `identity_provider_identity_store_id` | String | <p>
         Identity Store ID from the IAM Identity Center. If this parameter is empty it will be updated automatically when the user logs in for the first time to the mailbox associated with WorkMail.
      </p> |
| `state` | String | <p>The state of a user: enabled (registered to WorkMail) or disabled (deregistered or
         never registered to WorkMail).</p> |
| `telephone` | String | <p>User's contact number.</p> |
| `office` | String | <p>Office where the user is located.</p> |
| `company` | String | <p>Company of the user.</p> |
| `user_id` | String | <p>The identifier for the described user.</p> |
| `display_name` | String | <p>The display name of the user.</p> |
| `user_role` | String | <p>In certain cases, other entities are modeled as users. If interoperability is
         enabled, resources are imported into WorkMail as users. Because different WorkMail
         organizations rely on different directory types, administrators can distinguish between an
         unregistered user (account is disabled and has a user role) and the directory
         administrators. The values are USER, RESOURCE, SYSTEM_USER, and REMOTE_USER.</p> |
| `mailbox_deprovisioned_date` | String | <p>The date when the mailbox was removed for the user.</p> |
| `zip_code` | String | <p>Zip code of the user.</p> |
| `department` | String | <p>Department of the user.</p> |
| `street` | String | <p>Street where the user is located.</p> |
| `last_name` | String | <p>Last name of the user.</p> |
| `email` | String | <p>The email of the user.</p> |
| `first_name` | String | <p>First name of the user.</p> |
| `hidden_from_global_address_list` | bool | <p>If enabled, the user is hidden from the global address list.</p> |
| `name` | String | <p>The name for the user.</p> |
| `initials` | String | <p>Initials of the user.</p> |
| `job_title` | String | <p>Job title of the user.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create user
user = provider.workmail.User {
    display_name = "value"  # <p>The display name for the new user.</p>
    organization_id = "value"  # <p>The identifier of the organization for which the user is created.</p>
    name = "value"  # <p>The name for the new user. WorkMail directory user names have a maximum length of 64. All others have a maximum length of 20.</p>
}

# Access user outputs
user_id = user.id
user_mailbox_provisioned_date = user.mailbox_provisioned_date
user_city = user.city
user_country = user.country
user_enabled_date = user.enabled_date
user_identity_provider_user_id = user.identity_provider_user_id
user_disabled_date = user.disabled_date
user_identity_provider_identity_store_id = user.identity_provider_identity_store_id
user_state = user.state
user_telephone = user.telephone
user_office = user.office
user_company = user.company
user_user_id = user.user_id
user_display_name = user.display_name
user_user_role = user.user_role
user_mailbox_deprovisioned_date = user.mailbox_deprovisioned_date
user_zip_code = user.zip_code
user_department = user.department
user_street = user.street
user_last_name = user.last_name
user_email = user.email
user_first_name = user.first_name
user_hidden_from_global_address_list = user.hidden_from_global_address_list
user_name = user.name
user_initials = user.initials
user_job_title = user.job_title
```

---


### Identity_provider_configuration

IdentityProviderConfiguration resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `organization_id` | String | ✅ | <p>
         The ID of the WorkMail Organization. </p> |
| `authentication_mode` | String | ✅ | <p>
         The authentication mode used in WorkMail.</p> |
| `personal_access_token_configuration` | String | ✅ | <p>
         The details of the Personal Access Token configuration. 
      </p> |
| `identity_center_configuration` | String | ✅ | <p>
         The details of the IAM Identity Center configuration.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `authentication_mode` | String | <p>
The authentication mode used in WorkMail.</p> |
| `personal_access_token_configuration` | String | <p>
         The details of the Personal Access Token configuration.
      </p> |
| `identity_center_configuration` | String | <p>
         The details of the IAM Identity Center configuration.  
      </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create identity_provider_configuration
identity_provider_configuration = provider.workmail.Identity_provider_configuration {
    organization_id = "value"  # <p>
         The ID of the WorkMail Organization. </p>
    authentication_mode = "value"  # <p>
         The authentication mode used in WorkMail.</p>
    personal_access_token_configuration = "value"  # <p>
         The details of the Personal Access Token configuration. 
      </p>
    identity_center_configuration = "value"  # <p>
         The details of the IAM Identity Center configuration.</p>
}

# Access identity_provider_configuration outputs
identity_provider_configuration_id = identity_provider_configuration.id
identity_provider_configuration_authentication_mode = identity_provider_configuration.authentication_mode
identity_provider_configuration_personal_access_token_configuration = identity_provider_configuration.personal_access_token_configuration
identity_provider_configuration_identity_center_configuration = identity_provider_configuration.identity_center_configuration
```

---


### Inbound_dmarc_settings

InboundDmarcSettings resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `organization_id` | String | ✅ | <p>The ID of the organization that you are applying the DMARC policy to.</p> |
| `enforced` | bool | ✅ | <p>Enforces or suspends a policy after it's applied.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `enforced` | bool | <p>Lists the enforcement setting of the applied policy.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create inbound_dmarc_settings
inbound_dmarc_settings = provider.workmail.Inbound_dmarc_settings {
    organization_id = "value"  # <p>The ID of the organization that you are applying the DMARC policy to.</p>
    enforced = "value"  # <p>Enforces or suspends a policy after it's applied.</p>
}

# Access inbound_dmarc_settings outputs
inbound_dmarc_settings_id = inbound_dmarc_settings.id
inbound_dmarc_settings_enforced = inbound_dmarc_settings.enforced
```

---


### Personal_access_token_metadata

PersonalAccessTokenMetadata resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | <p>
         The Personal Access Token name.
      </p> |
| `date_last_used` | String | <p>
         The date when the Personal Access Token ID was last used.
      </p> |
| `date_created` | String | <p>
         The date when the Personal Access Token ID was created.
      </p> |
| `personal_access_token_id` | String | <p>
         The Personal Access Token ID.</p> |
| `user_id` | String | <p>
         The WorkMail User ID. 
      </p> |
| `expires_time` | String | <p>
         The time when the Personal Access Token ID will expire.
      </p> |
| `scopes` | Vec<String> | <p>
         Lists all the Personal Access Token permissions for a mailbox.
      </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access personal_access_token_metadata outputs
personal_access_token_metadata_id = personal_access_token_metadata.id
personal_access_token_metadata_name = personal_access_token_metadata.name
personal_access_token_metadata_date_last_used = personal_access_token_metadata.date_last_used
personal_access_token_metadata_date_created = personal_access_token_metadata.date_created
personal_access_token_metadata_personal_access_token_id = personal_access_token_metadata.personal_access_token_id
personal_access_token_metadata_user_id = personal_access_token_metadata.user_id
personal_access_token_metadata_expires_time = personal_access_token_metadata.expires_time
personal_access_token_metadata_scopes = personal_access_token_metadata.scopes
```

---


### Group

Group resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `organization_id` | String | ✅ | <p>The organization under which the group is to be created.</p> |
| `name` | String | ✅ | <p>The name of the group.</p> |
| `hidden_from_global_address_list` | bool |  | <p>If this parameter is enabled, the group will be hidden from the address book.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `hidden_from_global_address_list` | bool | <p>If the value is set to <i>true</i>, the group is hidden from the address book.</p> |
| `disabled_date` | String | <p>The date and time when a user was deregistered from WorkMail, in UNIX epoch time
         format.</p> |
| `state` | String | <p>The state of the user: enabled (registered to WorkMail) or disabled (deregistered or
         never registered to WorkMail).</p> |
| `group_id` | String | <p>The identifier of the described group.</p> |
| `name` | String | <p>The name of the described group.</p> |
| `email` | String | <p>The email of the described group.</p> |
| `enabled_date` | String | <p>The date and time when a user was registered to WorkMail, in UNIX epoch time
         format.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create group
group = provider.workmail.Group {
    organization_id = "value"  # <p>The organization under which the group is to be created.</p>
    name = "value"  # <p>The name of the group.</p>
}

# Access group outputs
group_id = group.id
group_hidden_from_global_address_list = group.hidden_from_global_address_list
group_disabled_date = group.disabled_date
group_state = group.state
group_group_id = group.group_id
group_name = group.name
group_email = group.email
group_enabled_date = group.enabled_date
```

---


### Alias

Alias resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `organization_id` | String | ✅ | <p>The organization under which the member (user or group) exists.</p> |
| `alias` | String | ✅ | <p>The alias to add to the member set.</p> |
| `entity_id` | String | ✅ | <p>The member (user or group) to which this alias is added.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create alias
alias = provider.workmail.Alias {
    organization_id = "value"  # <p>The organization under which the member (user or group) exists.</p>
    alias = "value"  # <p>The alias to add to the member set.</p>
    entity_id = "value"  # <p>The member (user or group) to which this alias is added.</p>
}

```

---


### Impersonation_role_effect

ImpersonationRoleEffect resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `type` | String | <p>The impersonation role type.</p> |
| `effect` | String | <p>
            <code></code>Effect of the impersonation role on the target user based on its rules. Available
         effects are <code>ALLOW</code> or <code>DENY</code>.</p> |
| `matched_rules` | Vec<String> | <p>A list of the rules that match the input and produce the configured effect.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access impersonation_role_effect outputs
impersonation_role_effect_id = impersonation_role_effect.id
impersonation_role_effect_type = impersonation_role_effect.type
impersonation_role_effect_effect = impersonation_role_effect.effect
impersonation_role_effect_matched_rules = impersonation_role_effect.matched_rules
```

---


### Entity

Entity resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | <p>Username, GroupName, or ResourceName based on entity type.</p> |
| `type` | String | <p>Entity type.</p> |
| `entity_id` | String | <p>The entity ID under which the entity exists.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access entity outputs
entity_id = entity.id
entity_name = entity.name
entity_type = entity.type
entity_entity_id = entity.entity_id
```

---


### Mobile_device_access_override

MobileDeviceAccessOverride resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `user_id` | String | ✅ | <p>The WorkMail user for which you create the override. Accepts the following types of user identities:</p>
         <ul>
            <li>
               <p>User ID: <code>12345678-1234-1234-1234-123456789012</code> or <code>S-1-1-12-1234567890-123456789-123456789-1234</code>
               </p>
            </li>
            <li>
               <p>Email address: <code>user@domain.tld</code>
               </p>
            </li>
            <li>
               <p>User name: <code>user</code>
               </p>
            </li>
         </ul> |
| `device_id` | String | ✅ | <p>The mobile device for which you create the override. <code>DeviceId</code> is case insensitive.</p> |
| `description` | String |  | <p>A description of the override.</p> |
| `organization_id` | String | ✅ | <p>Identifies the WorkMail organization for which you create the override.</p> |
| `effect` | String | ✅ | <p>The effect of the override, <code>ALLOW</code> or <code>DENY</code>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `effect` | String | <p>The effect of the override, <code>ALLOW</code> or <code>DENY</code>.</p> |
| `date_created` | String | <p>The date the override was first created.</p> |
| `description` | String | <p>A description of the override.</p> |
| `date_modified` | String | <p>The date the description was last modified.</p> |
| `user_id` | String | <p>The WorkMail user to which the access override applies.</p> |
| `device_id` | String | <p>The device to which the access override applies.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create mobile_device_access_override
mobile_device_access_override = provider.workmail.Mobile_device_access_override {
    user_id = "value"  # <p>The WorkMail user for which you create the override. Accepts the following types of user identities:</p>
         <ul>
            <li>
               <p>User ID: <code>12345678-1234-1234-1234-123456789012</code> or <code>S-1-1-12-1234567890-123456789-123456789-1234</code>
               </p>
            </li>
            <li>
               <p>Email address: <code>user@domain.tld</code>
               </p>
            </li>
            <li>
               <p>User name: <code>user</code>
               </p>
            </li>
         </ul>
    device_id = "value"  # <p>The mobile device for which you create the override. <code>DeviceId</code> is case insensitive.</p>
    organization_id = "value"  # <p>Identifies the WorkMail organization for which you create the override.</p>
    effect = "value"  # <p>The effect of the override, <code>ALLOW</code> or <code>DENY</code>.</p>
}

# Access mobile_device_access_override outputs
mobile_device_access_override_id = mobile_device_access_override.id
mobile_device_access_override_effect = mobile_device_access_override.effect
mobile_device_access_override_date_created = mobile_device_access_override.date_created
mobile_device_access_override_description = mobile_device_access_override.description
mobile_device_access_override_date_modified = mobile_device_access_override.date_modified
mobile_device_access_override_user_id = mobile_device_access_override.user_id
mobile_device_access_override_device_id = mobile_device_access_override.device_id
```

---


### Mailbox_permissions

MailboxPermissions resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `permission_values` | Vec<String> | ✅ | <p>The permissions granted to the grantee. SEND_AS allows the grantee to send email as
         the owner of the mailbox (the grantee is not mentioned on these emails). SEND_ON_BEHALF
         allows the grantee to send email on behalf of the owner of the mailbox (the grantee is not
         mentioned as the physical sender of these emails). FULL_ACCESS allows the grantee full
         access to the mailbox, irrespective of other folder-level permissions set on the
         mailbox.</p> |
| `entity_id` | String | ✅ | <p>The identifier of the user or resource for which to update mailbox
         permissions.</p>
         <p>The identifier can be <i>UserId, ResourceID, or Group Id</i>, <i>Username, Resourcename, or Groupname</i>, or <i>email</i>.</p>
         <ul>
            <li>
               <p>Entity ID: 12345678-1234-1234-1234-123456789012, r-0123456789a0123456789b0123456789, or S-1-1-12-1234567890-123456789-123456789-1234</p>
            </li>
            <li>
               <p>Email address: entity@domain.tld</p>
            </li>
            <li>
               <p>Entity name: entity</p>
            </li>
         </ul> |
| `organization_id` | String | ✅ | <p>The identifier of the organization under which the user, group, or resource
         exists.</p> |
| `grantee_id` | String | ✅ | <p>The identifier of the user, group, or resource to which to grant the
         permissions.</p>
         <p>The identifier can be <i>UserId, ResourceID, or Group Id</i>, <i>Username, Resourcename, or Groupname</i>, or <i>email</i>.</p>
         <ul>
            <li>
               <p>Grantee ID: 12345678-1234-1234-1234-123456789012, r-0123456789a0123456789b0123456789, or S-1-1-12-1234567890-123456789-123456789-1234</p>
            </li>
            <li>
               <p>Email address: grantee@domain.tld</p>
            </li>
            <li>
               <p>Grantee name: grantee</p>
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

# Create mailbox_permissions
mailbox_permissions = provider.workmail.Mailbox_permissions {
    permission_values = "value"  # <p>The permissions granted to the grantee. SEND_AS allows the grantee to send email as
         the owner of the mailbox (the grantee is not mentioned on these emails). SEND_ON_BEHALF
         allows the grantee to send email on behalf of the owner of the mailbox (the grantee is not
         mentioned as the physical sender of these emails). FULL_ACCESS allows the grantee full
         access to the mailbox, irrespective of other folder-level permissions set on the
         mailbox.</p>
    entity_id = "value"  # <p>The identifier of the user or resource for which to update mailbox
         permissions.</p>
         <p>The identifier can be <i>UserId, ResourceID, or Group Id</i>, <i>Username, Resourcename, or Groupname</i>, or <i>email</i>.</p>
         <ul>
            <li>
               <p>Entity ID: 12345678-1234-1234-1234-123456789012, r-0123456789a0123456789b0123456789, or S-1-1-12-1234567890-123456789-123456789-1234</p>
            </li>
            <li>
               <p>Email address: entity@domain.tld</p>
            </li>
            <li>
               <p>Entity name: entity</p>
            </li>
         </ul>
    organization_id = "value"  # <p>The identifier of the organization under which the user, group, or resource
         exists.</p>
    grantee_id = "value"  # <p>The identifier of the user, group, or resource to which to grant the
         permissions.</p>
         <p>The identifier can be <i>UserId, ResourceID, or Group Id</i>, <i>Username, Resourcename, or Groupname</i>, or <i>email</i>.</p>
         <ul>
            <li>
               <p>Grantee ID: 12345678-1234-1234-1234-123456789012, r-0123456789a0123456789b0123456789, or S-1-1-12-1234567890-123456789-123456789-1234</p>
            </li>
            <li>
               <p>Email address: grantee@domain.tld</p>
            </li>
            <li>
               <p>Grantee name: grantee</p>
            </li>
         </ul>
}

```

---


### Mailbox_export_job

MailboxExportJob resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `estimated_progress` | i64 | <p>The estimated progress of the mailbox export job, in percentage points.</p> |
| `state` | String | <p>The state of the mailbox export job.</p> |
| `end_time` | String | <p>The mailbox export job end timestamp.</p> |
| `entity_id` | String | <p>The identifier of the user or resource associated with the mailbox.</p> |
| `s3_prefix` | String | <p>The S3 bucket prefix.</p> |
| `start_time` | String | <p>The mailbox export job start timestamp.</p> |
| `error_info` | String | <p>Error information for failed mailbox export jobs.</p> |
| `kms_key_arn` | String | <p>The Amazon Resource Name (ARN) of the symmetric AWS Key Management Service (AWS KMS)
         key that encrypts the exported mailbox content.</p> |
| `description` | String | <p>The mailbox export job description.</p> |
| `s3_bucket_name` | String | <p>The name of the S3 bucket.</p> |
| `role_arn` | String | <p>The ARN of the AWS Identity and Access Management (IAM) role that grants write permission to the Amazon Simple
         Storage Service (Amazon S3) bucket.</p> |
| `s3_path` | String | <p>The path to the S3 bucket and file that the mailbox export job is exporting
         to.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access mailbox_export_job outputs
mailbox_export_job_id = mailbox_export_job.id
mailbox_export_job_estimated_progress = mailbox_export_job.estimated_progress
mailbox_export_job_state = mailbox_export_job.state
mailbox_export_job_end_time = mailbox_export_job.end_time
mailbox_export_job_entity_id = mailbox_export_job.entity_id
mailbox_export_job_s3_prefix = mailbox_export_job.s3_prefix
mailbox_export_job_start_time = mailbox_export_job.start_time
mailbox_export_job_error_info = mailbox_export_job.error_info
mailbox_export_job_kms_key_arn = mailbox_export_job.kms_key_arn
mailbox_export_job_description = mailbox_export_job.description
mailbox_export_job_s3_bucket_name = mailbox_export_job.s3_bucket_name
mailbox_export_job_role_arn = mailbox_export_job.role_arn
mailbox_export_job_s3_path = mailbox_export_job.s3_path
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple primary_email_address resources
primary_email_address_0 = provider.workmail.Primary_email_address {
    entity_id = "value-0"
    email = "value-0"
    organization_id = "value-0"
}
primary_email_address_1 = provider.workmail.Primary_email_address {
    entity_id = "value-1"
    email = "value-1"
    organization_id = "value-1"
}
primary_email_address_2 = provider.workmail.Primary_email_address {
    entity_id = "value-2"
    email = "value-2"
    organization_id = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    primary_email_address = provider.workmail.Primary_email_address {
        entity_id = "production-value"
        email = "production-value"
        organization_id = "production-value"
    }
```

---

## Related Documentation

- [AWS Workmail Documentation](https://docs.aws.amazon.com/workmail/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
