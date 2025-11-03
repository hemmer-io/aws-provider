# Ses Service



**Resources**: 23

---

## Overview

The ses service provides access to 23 resource types:

- [Receipt_filter](#receipt_filter) [CD]
- [Verified_email_address](#verified_email_address) [D]
- [Custom_verification_email_template](#custom_verification_email_template) [CRUD]
- [Receipt_rule](#receipt_rule) [CRUD]
- [Identity_notification_attributes](#identity_notification_attributes) [R]
- [Identity_dkim_attributes](#identity_dkim_attributes) [R]
- [Configuration_set_event_destination](#configuration_set_event_destination) [CUD]
- [Identity](#identity) [D]
- [Identity_policy](#identity_policy) [CD]
- [Identity_policies](#identity_policies) [R]
- [Active_receipt_rule_set](#active_receipt_rule_set) [R]
- [Identity_verification_attributes](#identity_verification_attributes) [R]
- [Template](#template) [CRUD]
- [Send_statistics](#send_statistics) [R]
- [Configuration_set_delivery_options](#configuration_set_delivery_options) [C]
- [Configuration_set_reputation_metrics_enabled](#configuration_set_reputation_metrics_enabled) [U]
- [Receipt_rule_set](#receipt_rule_set) [CRD]
- [Configuration_set](#configuration_set) [CRD]
- [Account_sending_enabled](#account_sending_enabled) [RU]
- [Configuration_set_sending_enabled](#configuration_set_sending_enabled) [U]
- [Identity_mail_from_domain_attributes](#identity_mail_from_domain_attributes) [R]
- [Send_quota](#send_quota) [R]
- [Configuration_set_tracking_options](#configuration_set_tracking_options) [CUD]

---

## Resources


### Receipt_filter

ReceiptFilter resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `filter` | String | ✅ | <p>A data structure that describes the IP address filter to create, which consists of a
            name, an IP address range, and whether to allow or block mail from it.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create receipt_filter
receipt_filter = provider.ses.Receipt_filter {
    filter = "value"  # <p>A data structure that describes the IP address filter to create, which consists of a
            name, an IP address range, and whether to allow or block mail from it.</p>
}

```

---


### Verified_email_address

VerifiedEmailAddress resource

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


### Custom_verification_email_template

CustomVerificationEmailTemplate resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `success_redirection_url` | String | ✅ | <p>The URL that the recipient of the verification email is sent to if his or her address
            is successfully verified.</p> |
| `template_name` | String | ✅ | <p>The name of the custom verification email template.</p> |
| `failure_redirection_url` | String | ✅ | <p>The URL that the recipient of the verification email is sent to if his or her address
            is not successfully verified.</p> |
| `template_content` | String | ✅ | <p>The content of the custom verification email. The total size of the email must be less
            than 10 MB. The message body may contain HTML, with some limitations. For more
            information, see <a href="https://docs.aws.amazon.com/ses/latest/dg/creating-identities.html#send-email-verify-address-custom">Custom
                Verification Email Frequently Asked Questions</a> in the <i>Amazon SES
                Developer Guide</i>.</p> |
| `from_email_address` | String | ✅ | <p>The email address that the custom verification email is sent from.</p> |
| `template_subject` | String | ✅ | <p>The subject line of the custom verification email.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `template_content` | String | <p>The content of the custom verification email.</p> |
| `template_name` | String | <p>The name of the custom verification email template.</p> |
| `from_email_address` | String | <p>The email address that the custom verification email is sent from.</p> |
| `success_redirection_url` | String | <p>The URL that the recipient of the verification email is sent to if his or her address
            is successfully verified.</p> |
| `failure_redirection_url` | String | <p>The URL that the recipient of the verification email is sent to if his or her address
            is not successfully verified.</p> |
| `template_subject` | String | <p>The subject line of the custom verification email.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create custom_verification_email_template
custom_verification_email_template = provider.ses.Custom_verification_email_template {
    success_redirection_url = "value"  # <p>The URL that the recipient of the verification email is sent to if his or her address
            is successfully verified.</p>
    template_name = "value"  # <p>The name of the custom verification email template.</p>
    failure_redirection_url = "value"  # <p>The URL that the recipient of the verification email is sent to if his or her address
            is not successfully verified.</p>
    template_content = "value"  # <p>The content of the custom verification email. The total size of the email must be less
            than 10 MB. The message body may contain HTML, with some limitations. For more
            information, see <a href="https://docs.aws.amazon.com/ses/latest/dg/creating-identities.html#send-email-verify-address-custom">Custom
                Verification Email Frequently Asked Questions</a> in the <i>Amazon SES
                Developer Guide</i>.</p>
    from_email_address = "value"  # <p>The email address that the custom verification email is sent from.</p>
    template_subject = "value"  # <p>The subject line of the custom verification email.</p>
}

# Access custom_verification_email_template outputs
custom_verification_email_template_id = custom_verification_email_template.id
custom_verification_email_template_template_content = custom_verification_email_template.template_content
custom_verification_email_template_template_name = custom_verification_email_template.template_name
custom_verification_email_template_from_email_address = custom_verification_email_template.from_email_address
custom_verification_email_template_success_redirection_url = custom_verification_email_template.success_redirection_url
custom_verification_email_template_failure_redirection_url = custom_verification_email_template.failure_redirection_url
custom_verification_email_template_template_subject = custom_verification_email_template.template_subject
```

---


### Receipt_rule

ReceiptRule resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `after` | String |  | <p>The name of an existing rule after which the new rule is placed. If this parameter is
            null, the new rule is inserted at the beginning of the rule list.</p> |
| `rule` | String | ✅ | <p>A data structure that contains the specified rule's name, actions, recipients,
            domains, enabled status, scan status, and TLS policy.</p> |
| `rule_set_name` | String | ✅ | <p>The name of the rule set where the receipt rule is added.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `rule` | String | <p>A data structure that contains the specified receipt rule's name, actions, recipients,
            domains, enabled status, scan status, and Transport Layer Security (TLS) policy.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create receipt_rule
receipt_rule = provider.ses.Receipt_rule {
    rule = "value"  # <p>A data structure that contains the specified rule's name, actions, recipients,
            domains, enabled status, scan status, and TLS policy.</p>
    rule_set_name = "value"  # <p>The name of the rule set where the receipt rule is added.</p>
}

# Access receipt_rule outputs
receipt_rule_id = receipt_rule.id
receipt_rule_rule = receipt_rule.rule
```

---


### Identity_notification_attributes

IdentityNotificationAttributes resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `notification_attributes` | HashMap<String, String> | <p>A map of Identity to IdentityNotificationAttributes.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access identity_notification_attributes outputs
identity_notification_attributes_id = identity_notification_attributes.id
identity_notification_attributes_notification_attributes = identity_notification_attributes.notification_attributes
```

---


### Identity_dkim_attributes

IdentityDkimAttributes resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `dkim_attributes` | HashMap<String, String> | <p>The DKIM attributes for an email address or a domain.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access identity_dkim_attributes outputs
identity_dkim_attributes_id = identity_dkim_attributes.id
identity_dkim_attributes_dkim_attributes = identity_dkim_attributes.dkim_attributes
```

---


### Configuration_set_event_destination

ConfigurationSetEventDestination resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `event_destination` | String | ✅ | <p>An object that describes the Amazon Web Services service that email sending event where information
            is published.</p> |
| `configuration_set_name` | String | ✅ | <p>The name of the configuration set that the event destination should be associated
            with.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create configuration_set_event_destination
configuration_set_event_destination = provider.ses.Configuration_set_event_destination {
    event_destination = "value"  # <p>An object that describes the Amazon Web Services service that email sending event where information
            is published.</p>
    configuration_set_name = "value"  # <p>The name of the configuration set that the event destination should be associated
            with.</p>
}

```

---


### Identity

Identity resource

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


### Identity_policy

IdentityPolicy resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `identity` | String | ✅ | <p>The identity to which that the policy applies. You can specify an identity by using
            its name or by using its Amazon Resource Name (ARN). Examples:
                <code>user@example.com</code>, <code>example.com</code>,
                <code>arn:aws:ses:us-east-1:123456789012:identity/example.com</code>.</p>
         <p>To successfully call this operation, you must own the identity.</p> |
| `policy` | String | ✅ | <p>The text of the policy in JSON format. The policy cannot exceed 4 KB.</p>
         <p>For information about the syntax of sending authorization policies, see the <a href="https://docs.aws.amazon.com/ses/latest/dg/sending-authorization-policies.html">Amazon SES
                Developer Guide</a>. </p> |
| `policy_name` | String | ✅ | <p>The name of the policy.</p>
         <p>The policy name cannot exceed 64 characters and can only include alphanumeric
            characters, dashes, and underscores.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create identity_policy
identity_policy = provider.ses.Identity_policy {
    identity = "value"  # <p>The identity to which that the policy applies. You can specify an identity by using
            its name or by using its Amazon Resource Name (ARN). Examples:
                <code>user@example.com</code>, <code>example.com</code>,
                <code>arn:aws:ses:us-east-1:123456789012:identity/example.com</code>.</p>
         <p>To successfully call this operation, you must own the identity.</p>
    policy = "value"  # <p>The text of the policy in JSON format. The policy cannot exceed 4 KB.</p>
         <p>For information about the syntax of sending authorization policies, see the <a href="https://docs.aws.amazon.com/ses/latest/dg/sending-authorization-policies.html">Amazon SES
                Developer Guide</a>. </p>
    policy_name = "value"  # <p>The name of the policy.</p>
         <p>The policy name cannot exceed 64 characters and can only include alphanumeric
            characters, dashes, and underscores.</p>
}

```

---


### Identity_policies

IdentityPolicies resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `policies` | HashMap<String, String> | <p>A map of policy names to policies.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access identity_policies outputs
identity_policies_id = identity_policies.id
identity_policies_policies = identity_policies.policies
```

---


### Active_receipt_rule_set

ActiveReceiptRuleSet resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `metadata` | String | <p>The metadata for the currently active receipt rule set. The metadata consists of the
            rule set name and a timestamp of when the rule set was created.</p> |
| `rules` | Vec<String> | <p>The receipt rules that belong to the active rule set.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access active_receipt_rule_set outputs
active_receipt_rule_set_id = active_receipt_rule_set.id
active_receipt_rule_set_metadata = active_receipt_rule_set.metadata
active_receipt_rule_set_rules = active_receipt_rule_set.rules
```

---


### Identity_verification_attributes

IdentityVerificationAttributes resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `verification_attributes` | HashMap<String, String> | <p>A map of Identities to IdentityVerificationAttributes objects.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access identity_verification_attributes outputs
identity_verification_attributes_id = identity_verification_attributes.id
identity_verification_attributes_verification_attributes = identity_verification_attributes.verification_attributes
```

---


### Template

Template resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `template` | String | ✅ | <p>The content of the email, composed of a subject line and either an HTML part or a
            text-only part.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `template` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create template
template = provider.ses.Template {
    template = "value"  # <p>The content of the email, composed of a subject line and either an HTML part or a
            text-only part.</p>
}

# Access template outputs
template_id = template.id
template_template = template.template
```

---


### Send_statistics

SendStatistics resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `send_data_points` | Vec<String> | <p>A list of data points, each of which represents 15 minutes of activity.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access send_statistics outputs
send_statistics_id = send_statistics.id
send_statistics_send_data_points = send_statistics.send_data_points
```

---


### Configuration_set_delivery_options

ConfigurationSetDeliveryOptions resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `configuration_set_name` | String | ✅ | <p>The name of the configuration set.</p> |
| `delivery_options` | String |  | <p>Specifies whether messages that use the configuration set are required to use
            Transport Layer Security (TLS).</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create configuration_set_delivery_options
configuration_set_delivery_options = provider.ses.Configuration_set_delivery_options {
    configuration_set_name = "value"  # <p>The name of the configuration set.</p>
}

```

---


### Configuration_set_reputation_metrics_enabled

ConfigurationSetReputationMetricsEnabled resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `configuration_set_name` | String | ✅ | <p>The name of the configuration set to update.</p> |
| `enabled` | bool | ✅ | <p>Describes whether or not Amazon SES publishes reputation metrics for the configuration set,
            such as bounce and complaint rates, to Amazon CloudWatch.</p> |



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


### Receipt_rule_set

ReceiptRuleSet resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `rule_set_name` | String | ✅ | <p>The name of the rule set to create. The name must meet the following
            requirements:</p>
         <ul>
            <li>
               <p>Contain only ASCII letters (a-z, A-Z), numbers (0-9), underscores (_), or
                    dashes (-).</p>
            </li>
            <li>
               <p>Start and end with a letter or number.</p>
            </li>
            <li>
               <p>Contain 64 characters or fewer.</p>
            </li>
         </ul> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `metadata` | String | <p>The metadata for the receipt rule set, which consists of the rule set name and the
            timestamp of when the rule set was created.</p> |
| `rules` | Vec<String> | <p>A list of the receipt rules that belong to the specified receipt rule set.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create receipt_rule_set
receipt_rule_set = provider.ses.Receipt_rule_set {
    rule_set_name = "value"  # <p>The name of the rule set to create. The name must meet the following
            requirements:</p>
         <ul>
            <li>
               <p>Contain only ASCII letters (a-z, A-Z), numbers (0-9), underscores (_), or
                    dashes (-).</p>
            </li>
            <li>
               <p>Start and end with a letter or number.</p>
            </li>
            <li>
               <p>Contain 64 characters or fewer.</p>
            </li>
         </ul>
}

# Access receipt_rule_set outputs
receipt_rule_set_id = receipt_rule_set.id
receipt_rule_set_metadata = receipt_rule_set.metadata
receipt_rule_set_rules = receipt_rule_set.rules
```

---


### Configuration_set

ConfigurationSet resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `configuration_set` | String | ✅ | <p>A data structure that contains the name of the configuration set.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `configuration_set` | String | <p>The configuration set object associated with the specified configuration set.</p> |
| `tracking_options` | String | <p>The name of the custom open and click tracking domain associated with the
            configuration set.</p> |
| `reputation_options` | String | <p>An object that represents the reputation settings for the configuration set. </p> |
| `event_destinations` | Vec<String> | <p>A list of event destinations associated with the configuration set. </p> |
| `delivery_options` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create configuration_set
configuration_set = provider.ses.Configuration_set {
    configuration_set = "value"  # <p>A data structure that contains the name of the configuration set.</p>
}

# Access configuration_set outputs
configuration_set_id = configuration_set.id
configuration_set_configuration_set = configuration_set.configuration_set
configuration_set_tracking_options = configuration_set.tracking_options
configuration_set_reputation_options = configuration_set.reputation_options
configuration_set_event_destinations = configuration_set.event_destinations
configuration_set_delivery_options = configuration_set.delivery_options
```

---


### Account_sending_enabled

AccountSendingEnabled resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `enabled` | bool |  | <p>Describes whether email sending is enabled or disabled for your Amazon SES account in the
            current Amazon Web Services Region.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `enabled` | bool | <p>Describes whether email sending is enabled or disabled for your Amazon SES account in the
            current Amazon Web Services Region.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access account_sending_enabled outputs
account_sending_enabled_id = account_sending_enabled.id
account_sending_enabled_enabled = account_sending_enabled.enabled
```

---


### Configuration_set_sending_enabled

ConfigurationSetSendingEnabled resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `configuration_set_name` | String | ✅ | <p>The name of the configuration set to update.</p> |
| `enabled` | bool | ✅ | <p>Describes whether email sending is enabled or disabled for the configuration set.
        </p> |



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


### Identity_mail_from_domain_attributes

IdentityMailFromDomainAttributes resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `mail_from_domain_attributes` | HashMap<String, String> | <p>A map of identities to custom MAIL FROM attributes.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access identity_mail_from_domain_attributes outputs
identity_mail_from_domain_attributes_id = identity_mail_from_domain_attributes.id
identity_mail_from_domain_attributes_mail_from_domain_attributes = identity_mail_from_domain_attributes.mail_from_domain_attributes
```

---


### Send_quota

SendQuota resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `max_send_rate` | f64 | <p>The maximum number of emails that Amazon SES can accept from the user's account per
            second.</p>
         <note>
            <p>The rate at which Amazon SES accepts the user's messages might be less than the maximum
                send rate.</p>
         </note> |
| `sent_last24_hours` | f64 | <p>The number of emails sent during the previous 24 hours.</p> |
| `max24_hour_send` | f64 | <p>The maximum number of emails the user is allowed to send in a 24-hour interval. A
            value of -1 signifies an unlimited quota.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access send_quota outputs
send_quota_id = send_quota.id
send_quota_max_send_rate = send_quota.max_send_rate
send_quota_sent_last24_hours = send_quota.sent_last24_hours
send_quota_max24_hour_send = send_quota.max24_hour_send
```

---


### Configuration_set_tracking_options

ConfigurationSetTrackingOptions resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tracking_options` | String | ✅ |  |
| `configuration_set_name` | String | ✅ | <p>The name of the configuration set that the tracking options should be associated
            with.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create configuration_set_tracking_options
configuration_set_tracking_options = provider.ses.Configuration_set_tracking_options {
    tracking_options = "value"  # Required field
    configuration_set_name = "value"  # <p>The name of the configuration set that the tracking options should be associated
            with.</p>
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

# Create multiple receipt_filter resources
receipt_filter_0 = provider.ses.Receipt_filter {
    filter = "value-0"
}
receipt_filter_1 = provider.ses.Receipt_filter {
    filter = "value-1"
}
receipt_filter_2 = provider.ses.Receipt_filter {
    filter = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    receipt_filter = provider.ses.Receipt_filter {
        filter = "production-value"
    }
```

---

## Related Documentation

- [AWS Ses Documentation](https://docs.aws.amazon.com/ses/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
