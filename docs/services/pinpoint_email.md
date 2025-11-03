# Pinpoint_email Service



**Resources**: 25

---

## Overview

The pinpoint_email service provides access to 25 resource types:

- [Configuration_set_delivery_options](#configuration_set_delivery_options) [C]
- [Email_identity_dkim_attributes](#email_identity_dkim_attributes) [C]
- [Deliverability_dashboard_option](#deliverability_dashboard_option) [C]
- [Email_identity_feedback_attributes](#email_identity_feedback_attributes) [C]
- [Domain_statistics_report](#domain_statistics_report) [R]
- [Email_identity_mail_from_attributes](#email_identity_mail_from_attributes) [C]
- [Deliverability_dashboard_options](#deliverability_dashboard_options) [R]
- [Dedicated_ips](#dedicated_ips) [R]
- [Configuration_set_event_destinations](#configuration_set_event_destinations) [R]
- [Account_dedicated_ip_warmup_attributes](#account_dedicated_ip_warmup_attributes) [C]
- [Dedicated_ip_in_pool](#dedicated_ip_in_pool) [C]
- [Dedicated_ip_pool](#dedicated_ip_pool) [CD]
- [Deliverability_test_report](#deliverability_test_report) [CR]
- [Account](#account) [R]
- [Blacklist_reports](#blacklist_reports) [R]
- [Configuration_set_tracking_options](#configuration_set_tracking_options) [C]
- [Account_sending_attributes](#account_sending_attributes) [C]
- [Email_identity](#email_identity) [CRD]
- [Dedicated_ip](#dedicated_ip) [R]
- [Configuration_set_sending_options](#configuration_set_sending_options) [C]
- [Configuration_set_reputation_options](#configuration_set_reputation_options) [C]
- [Configuration_set](#configuration_set) [CRD]
- [Domain_deliverability_campaign](#domain_deliverability_campaign) [R]
- [Configuration_set_event_destination](#configuration_set_event_destination) [CUD]
- [Dedicated_ip_warmup_attributes](#dedicated_ip_warmup_attributes) [C]

---

## Resources


### Configuration_set_delivery_options

ConfigurationSetDeliveryOptions resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `configuration_set_name` | String | ✅ | <p>The name of the configuration set that you want to associate with a dedicated IP
            pool.</p> |
| `tls_policy` | String |  | <p>Specifies whether messages that use the configuration set are required to use
            Transport Layer Security (TLS). If the value is <code>Require</code>, messages are only
            delivered if a TLS connection can be established. If the value is <code>Optional</code>,
            messages can be delivered in plain text if a TLS connection can't be established.</p> |
| `sending_pool_name` | String |  | <p>The name of the dedicated IP pool that you want to associate with the configuration
            set.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create configuration_set_delivery_options
configuration_set_delivery_options = provider.pinpoint_email.Configuration_set_delivery_options {
    configuration_set_name = "value"  # <p>The name of the configuration set that you want to associate with a dedicated IP
            pool.</p>
}

```

---


### Email_identity_dkim_attributes

EmailIdentityDkimAttributes resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `email_identity` | String | ✅ | <p>The email identity that you want to change the DKIM settings for.</p> |
| `signing_enabled` | bool |  | <p>Sets the DKIM signing configuration for the identity.</p>
        <p>When you set this value <code>true</code>, then the messages that Amazon Pinpoint sends from the
            identity are DKIM-signed. When you set this value to <code>false</code>, then the
            messages that Amazon Pinpoint sends from the identity aren't DKIM-signed.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create email_identity_dkim_attributes
email_identity_dkim_attributes = provider.pinpoint_email.Email_identity_dkim_attributes {
    email_identity = "value"  # <p>The email identity that you want to change the DKIM settings for.</p>
}

```

---


### Deliverability_dashboard_option

DeliverabilityDashboardOption resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `dashboard_enabled` | bool | ✅ | <p>Specifies whether to enable the Deliverability dashboard for your Amazon Pinpoint account. To enable the
            dashboard, set this value to <code>true</code>.</p> |
| `subscribed_domains` | Vec<String> |  | <p>An array of objects, one for each verified domain that you use to send email and
            enabled the Deliverability dashboard for.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create deliverability_dashboard_option
deliverability_dashboard_option = provider.pinpoint_email.Deliverability_dashboard_option {
    dashboard_enabled = "value"  # <p>Specifies whether to enable the Deliverability dashboard for your Amazon Pinpoint account. To enable the
            dashboard, set this value to <code>true</code>.</p>
}

```

---


### Email_identity_feedback_attributes

EmailIdentityFeedbackAttributes resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `email_forwarding_enabled` | bool |  | <p>Sets the feedback forwarding configuration for the identity.</p>
        <p>If the value is <code>true</code>, Amazon Pinpoint sends you email notifications when bounce or
            complaint events occur. Amazon Pinpoint sends this notification to the address that you specified
            in the Return-Path header of the original email.</p>
        <p>When you set this value to <code>false</code>, Amazon Pinpoint sends notifications through other
            mechanisms, such as by notifying an Amazon SNS topic or another event destination. You're
            required to have a method of tracking bounces and complaints. If you haven't set up
            another mechanism for receiving bounce or complaint notifications, Amazon Pinpoint sends an email
            notification when these events occur (even if this setting is disabled).</p> |
| `email_identity` | String | ✅ | <p>The email identity that you want to configure bounce and complaint feedback forwarding
            for.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create email_identity_feedback_attributes
email_identity_feedback_attributes = provider.pinpoint_email.Email_identity_feedback_attributes {
    email_identity = "value"  # <p>The email identity that you want to configure bounce and complaint feedback forwarding
            for.</p>
}

```

---


### Domain_statistics_report

DomainStatisticsReport resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `daily_volumes` | Vec<String> | <p>An object that contains deliverability metrics for the domain that you specified. This
            object contains data for each day, starting on the <code>StartDate</code> and ending on
            the <code>EndDate</code>.</p> |
| `overall_volume` | String | <p>An object that contains deliverability metrics for the domain that you specified. The
            data in this object is a summary of all of the data that was collected from the
                <code>StartDate</code> to the <code>EndDate</code>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access domain_statistics_report outputs
domain_statistics_report_id = domain_statistics_report.id
domain_statistics_report_daily_volumes = domain_statistics_report.daily_volumes
domain_statistics_report_overall_volume = domain_statistics_report.overall_volume
```

---


### Email_identity_mail_from_attributes

EmailIdentityMailFromAttributes resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `email_identity` | String | ✅ | <p>The verified email identity that you want to set up the custom MAIL FROM domain
            for.</p> |
| `mail_from_domain` | String |  | <p> The custom MAIL FROM domain that you want the verified identity to use. The MAIL FROM
            domain must meet the following criteria:</p>
        <ul>
            <li>
                <p>It has to be a subdomain of the verified identity.</p>
            </li>
            <li>
                <p>It can't be used to receive email.</p>
            </li>
            <li>
                <p>It can't be used in a "From" address if the MAIL FROM domain is a destination
                    for feedback forwarding emails.</p>
            </li>
         </ul> |
| `behavior_on_mx_failure` | String |  | <p>The action that you want Amazon Pinpoint to take if it can't read the required MX record when
            you send an email. When you set this value to <code>UseDefaultValue</code>, Amazon Pinpoint uses
                <i>amazonses.com</i> as the MAIL FROM domain. When you set this value
            to <code>RejectMessage</code>, Amazon Pinpoint returns a <code>MailFromDomainNotVerified</code>
            error, and doesn't attempt to deliver the email.</p>
        <p>These behaviors are taken when the custom MAIL FROM domain configuration is in the
                <code>Pending</code>, <code>Failed</code>, and <code>TemporaryFailure</code>
            states.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create email_identity_mail_from_attributes
email_identity_mail_from_attributes = provider.pinpoint_email.Email_identity_mail_from_attributes {
    email_identity = "value"  # <p>The verified email identity that you want to set up the custom MAIL FROM domain
            for.</p>
}

```

---


### Deliverability_dashboard_options

DeliverabilityDashboardOptions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `subscription_expiry_date` | String | <p>The date, in Unix time format, when your current subscription to the Deliverability dashboard
            is scheduled to expire, if your subscription is scheduled to expire at the end of the
            current calendar month. This value is null if you have an active subscription that isn’t
            due to expire at the end of the month.</p> |
| `dashboard_enabled` | bool | <p>Specifies whether the Deliverability dashboard is enabled for your Amazon Pinpoint account. If this value
            is <code>true</code>, the dashboard is enabled.</p> |
| `active_subscribed_domains` | Vec<String> | <p>An array of objects, one for each verified domain that you use to send email and
            currently has an active Deliverability dashboard subscription that isn’t scheduled to expire at
            the end of the current calendar month.</p> |
| `pending_expiration_subscribed_domains` | Vec<String> | <p>An array of objects, one for each verified domain that you use to send email and
            currently has an active Deliverability dashboard subscription that's scheduled to expire at the
            end of the current calendar month.</p> |
| `account_status` | String | <p>The current status of your Deliverability dashboard subscription. If this value is
                <code>PENDING_EXPIRATION</code>, your subscription is scheduled to expire at the end
            of the current calendar month.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access deliverability_dashboard_options outputs
deliverability_dashboard_options_id = deliverability_dashboard_options.id
deliverability_dashboard_options_subscription_expiry_date = deliverability_dashboard_options.subscription_expiry_date
deliverability_dashboard_options_dashboard_enabled = deliverability_dashboard_options.dashboard_enabled
deliverability_dashboard_options_active_subscribed_domains = deliverability_dashboard_options.active_subscribed_domains
deliverability_dashboard_options_pending_expiration_subscribed_domains = deliverability_dashboard_options.pending_expiration_subscribed_domains
deliverability_dashboard_options_account_status = deliverability_dashboard_options.account_status
```

---


### Dedicated_ips

DedicatedIps resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `dedicated_ips` | Vec<String> | <p>A list of dedicated IP addresses that are reserved for use by your Amazon Pinpoint
            account.</p> |
| `next_token` | String | <p>A token that indicates that there are additional dedicated IP addresses to list. To
            view additional addresses, issue another request to <code>GetDedicatedIps</code>,
            passing this token in the <code>NextToken</code> parameter.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access dedicated_ips outputs
dedicated_ips_id = dedicated_ips.id
dedicated_ips_dedicated_ips = dedicated_ips.dedicated_ips
dedicated_ips_next_token = dedicated_ips.next_token
```

---


### Configuration_set_event_destinations

ConfigurationSetEventDestinations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `event_destinations` | Vec<String> | <p>An array that includes all of the events destinations that have been configured for
            the configuration set.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access configuration_set_event_destinations outputs
configuration_set_event_destinations_id = configuration_set_event_destinations.id
configuration_set_event_destinations_event_destinations = configuration_set_event_destinations.event_destinations
```

---


### Account_dedicated_ip_warmup_attributes

AccountDedicatedIpWarmupAttributes resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `auto_warmup_enabled` | bool |  | <p>Enables or disables the automatic warm-up feature for dedicated IP addresses that are
            associated with your Amazon Pinpoint account in the current AWS Region. Set to <code>true</code>
            to enable the automatic warm-up feature, or set to <code>false</code> to disable
            it.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create account_dedicated_ip_warmup_attributes
account_dedicated_ip_warmup_attributes = provider.pinpoint_email.Account_dedicated_ip_warmup_attributes {
}

```

---


### Dedicated_ip_in_pool

DedicatedIpInPool resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `ip` | String | ✅ | <p>The IP address that you want to move to the dedicated IP pool. The value you specify
            has to be a dedicated IP address that's associated with your Amazon Pinpoint account.</p> |
| `destination_pool_name` | String | ✅ | <p>The name of the IP pool that you want to add the dedicated IP address to. You have to
            specify an IP pool that already exists.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create dedicated_ip_in_pool
dedicated_ip_in_pool = provider.pinpoint_email.Dedicated_ip_in_pool {
    ip = "value"  # <p>The IP address that you want to move to the dedicated IP pool. The value you specify
            has to be a dedicated IP address that's associated with your Amazon Pinpoint account.</p>
    destination_pool_name = "value"  # <p>The name of the IP pool that you want to add the dedicated IP address to. You have to
            specify an IP pool that already exists.</p>
}

```

---


### Dedicated_ip_pool

DedicatedIpPool resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>An object that defines the tags (keys and values) that you want to associate with the
            pool.</p> |
| `pool_name` | String | ✅ | <p>The name of the dedicated IP pool.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create dedicated_ip_pool
dedicated_ip_pool = provider.pinpoint_email.Dedicated_ip_pool {
    pool_name = "value"  # <p>The name of the dedicated IP pool.</p>
}

```

---


### Deliverability_test_report

DeliverabilityTestReport resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>An array of objects that define the tags (keys and values) that you want to associate
            with the predictive inbox placement test.</p> |
| `report_name` | String |  | <p>A unique name that helps you to identify the predictive inbox placement test when you retrieve the
            results.</p> |
| `from_email_address` | String | ✅ | <p>The email address that the predictive inbox placement test email was sent from.</p> |
| `content` | String | ✅ | <p>The HTML body of the message that you sent when you performed the predictive inbox placement test.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `overall_placement` | String | <p>An object that specifies how many test messages that were sent during the predictive inbox placement test were
            delivered to recipients' inboxes, how many were sent to recipients' spam folders, and
            how many weren't delivered.</p> |
| `message` | String | <p>An object that contains the message that you sent when you performed this
            predictive inbox placement test.</p> |
| `deliverability_test_report` | String | <p>An object that contains the results of the predictive inbox placement test.</p> |
| `isp_placements` | Vec<String> | <p>An object that describes how the test email was handled by several email providers,
            including Gmail, Hotmail, Yahoo, AOL, and others.</p> |
| `tags` | Vec<String> | <p>An array of objects that define the tags (keys and values) that are associated with
            the predictive inbox placement test.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create deliverability_test_report
deliverability_test_report = provider.pinpoint_email.Deliverability_test_report {
    from_email_address = "value"  # <p>The email address that the predictive inbox placement test email was sent from.</p>
    content = "value"  # <p>The HTML body of the message that you sent when you performed the predictive inbox placement test.</p>
}

# Access deliverability_test_report outputs
deliverability_test_report_id = deliverability_test_report.id
deliverability_test_report_overall_placement = deliverability_test_report.overall_placement
deliverability_test_report_message = deliverability_test_report.message
deliverability_test_report_deliverability_test_report = deliverability_test_report.deliverability_test_report
deliverability_test_report_isp_placements = deliverability_test_report.isp_placements
deliverability_test_report_tags = deliverability_test_report.tags
```

---


### Account

Account resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `production_access_enabled` | bool | <p>Indicates whether or not your account has production access in the current AWS
            Region.</p>
        <p>If the value is <code>false</code>, then your account is in the
                <i>sandbox</i>. When your account is in the sandbox, you can only send
            email to verified identities. Additionally, the maximum number of emails you can send in
            a 24-hour period (your sending quota) is 200, and the maximum number of emails you can
            send per second (your maximum sending rate) is 1.</p>
        <p>If the value is <code>true</code>, then your account has production access. When your
            account has production access, you can send email to any address. The sending quota and
            maximum sending rate for your account vary based on your specific use case.</p> |
| `dedicated_ip_auto_warmup_enabled` | bool | <p>Indicates whether or not the automatic warm-up feature is enabled for dedicated IP
            addresses that are associated with your account.</p> |
| `sending_enabled` | bool | <p>Indicates whether or not email sending is enabled for your Amazon Pinpoint account in the
            current AWS Region.</p> |
| `send_quota` | String | <p>An object that contains information about the per-day and per-second sending limits
            for your Amazon Pinpoint account in the current AWS Region.</p> |
| `enforcement_status` | String | <p>The reputation status of your Amazon Pinpoint account. The status can be one of the
            following:</p>
        <ul>
            <li>
                <p>
                  <code>HEALTHY</code> – There are no reputation-related issues that
                    currently impact your account.</p>
            </li>
            <li>
                <p>
                  <code>PROBATION</code> – We've identified some issues with your Amazon Pinpoint
                    account. We're placing your account under review while you work on correcting
                    these issues.</p>
            </li>
            <li>
                <p>
                  <code>SHUTDOWN</code> – Your account's ability to send email is
                    currently paused because of an issue with the email sent from your account. When
                    you correct the issue, you can contact us and request that your account's
                    ability to send email is resumed.</p>
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

# Access account outputs
account_id = account.id
account_production_access_enabled = account.production_access_enabled
account_dedicated_ip_auto_warmup_enabled = account.dedicated_ip_auto_warmup_enabled
account_sending_enabled = account.sending_enabled
account_send_quota = account.send_quota
account_enforcement_status = account.enforcement_status
```

---


### Blacklist_reports

BlacklistReports resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `blacklist_report` | HashMap<String, Vec<String>> | <p>An object that contains information about a blacklist that one of your dedicated IP
            addresses appears on.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access blacklist_reports outputs
blacklist_reports_id = blacklist_reports.id
blacklist_reports_blacklist_report = blacklist_reports.blacklist_report
```

---


### Configuration_set_tracking_options

ConfigurationSetTrackingOptions resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `custom_redirect_domain` | String |  | <p>The domain that you want to use to track open and click events.</p> |
| `configuration_set_name` | String | ✅ | <p>The name of the configuration set that you want to add a custom tracking domain
            to.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create configuration_set_tracking_options
configuration_set_tracking_options = provider.pinpoint_email.Configuration_set_tracking_options {
    configuration_set_name = "value"  # <p>The name of the configuration set that you want to add a custom tracking domain
            to.</p>
}

```

---


### Account_sending_attributes

AccountSendingAttributes resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `sending_enabled` | bool |  | <p>Enables or disables your account's ability to send email. Set to <code>true</code> to
            enable email sending, or set to <code>false</code> to disable email sending.</p>
        <note>
            <p>If AWS paused your account's ability to send email, you can't use this operation
                to resume your account's ability to send email.</p>
        </note> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create account_sending_attributes
account_sending_attributes = provider.pinpoint_email.Account_sending_attributes {
}

```

---


### Email_identity

EmailIdentity resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `email_identity` | String | ✅ | <p>The email address or domain that you want to verify.</p> |
| `tags` | Vec<String> |  | <p>An array of objects that define the tags (keys and values) that you want to associate
            with the email identity.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `feedback_forwarding_status` | bool | <p>The feedback forwarding configuration for the identity.</p>
        <p>If the value is <code>true</code>, Amazon Pinpoint sends you email notifications when bounce or
            complaint events occur. Amazon Pinpoint sends this notification to the address that you specified
            in the Return-Path header of the original email.</p>
        <p>When you set this value to <code>false</code>, Amazon Pinpoint sends notifications through other
            mechanisms, such as by notifying an Amazon SNS topic or another event destination. You're
            required to have a method of tracking bounces and complaints. If you haven't set up
            another mechanism for receiving bounce or complaint notifications, Amazon Pinpoint sends an email
            notification when these events occur (even if this setting is disabled).</p> |
| `dkim_attributes` | String | <p>An object that contains information about the DKIM attributes for the identity. This
            object includes the tokens that you use to create the CNAME records that are required to
            complete the DKIM verification process.</p> |
| `tags` | Vec<String> | <p>An array of objects that define the tags (keys and values) that are associated with
            the email identity.</p> |
| `mail_from_attributes` | String | <p>An object that contains information about the Mail-From attributes for the email
            identity.</p> |
| `verified_for_sending_status` | bool | <p>Specifies whether or not the identity is verified. In Amazon Pinpoint, you can only send email
            from verified email addresses or domains. For more information about verifying
            identities, see the <a href="https://docs.aws.amazon.com/pinpoint/latest/userguide/channels-email-manage-verify.html">Amazon Pinpoint User Guide</a>.</p> |
| `identity_type` | String | <p>The email identity type.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create email_identity
email_identity = provider.pinpoint_email.Email_identity {
    email_identity = "value"  # <p>The email address or domain that you want to verify.</p>
}

# Access email_identity outputs
email_identity_id = email_identity.id
email_identity_feedback_forwarding_status = email_identity.feedback_forwarding_status
email_identity_dkim_attributes = email_identity.dkim_attributes
email_identity_tags = email_identity.tags
email_identity_mail_from_attributes = email_identity.mail_from_attributes
email_identity_verified_for_sending_status = email_identity.verified_for_sending_status
email_identity_identity_type = email_identity.identity_type
```

---


### Dedicated_ip

DedicatedIp resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `dedicated_ip` | String | <p>An object that contains information about a dedicated IP address.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access dedicated_ip outputs
dedicated_ip_id = dedicated_ip.id
dedicated_ip_dedicated_ip = dedicated_ip.dedicated_ip
```

---


### Configuration_set_sending_options

ConfigurationSetSendingOptions resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `sending_enabled` | bool |  | <p>If <code>true</code>, email sending is enabled for the configuration set. If
                <code>false</code>, email sending is disabled for the configuration set.</p> |
| `configuration_set_name` | String | ✅ | <p>The name of the configuration set that you want to enable or disable email sending
            for.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create configuration_set_sending_options
configuration_set_sending_options = provider.pinpoint_email.Configuration_set_sending_options {
    configuration_set_name = "value"  # <p>The name of the configuration set that you want to enable or disable email sending
            for.</p>
}

```

---


### Configuration_set_reputation_options

ConfigurationSetReputationOptions resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `reputation_metrics_enabled` | bool |  | <p>If <code>true</code>, tracking of reputation metrics is enabled for the configuration
            set. If <code>false</code>, tracking of reputation metrics is disabled for the
            configuration set.</p> |
| `configuration_set_name` | String | ✅ | <p>The name of the configuration set that you want to enable or disable reputation metric
            tracking for.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create configuration_set_reputation_options
configuration_set_reputation_options = provider.pinpoint_email.Configuration_set_reputation_options {
    configuration_set_name = "value"  # <p>The name of the configuration set that you want to enable or disable reputation metric
            tracking for.</p>
}

```

---


### Configuration_set

ConfigurationSet resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `delivery_options` | String |  | <p>An object that defines the dedicated IP pool that is used to send emails that you send
            using the configuration set.</p> |
| `tracking_options` | String |  | <p>An object that defines the open and click tracking options for emails that you send
            using the configuration set.</p> |
| `sending_options` | String |  | <p>An object that defines whether or not Amazon Pinpoint can send email that you send using the
            configuration set.</p> |
| `reputation_options` | String |  | <p>An object that defines whether or not Amazon Pinpoint collects reputation metrics for the emails
            that you send that use the configuration set.</p> |
| `tags` | Vec<String> |  | <p>An array of objects that define the tags (keys and values) that you want to associate
            with the configuration set.</p> |
| `configuration_set_name` | String | ✅ | <p>The name of the configuration set.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `delivery_options` | String | <p>An object that defines the dedicated IP pool that is used to send emails that you send
            using the configuration set.</p> |
| `reputation_options` | String | <p>An object that defines whether or not Amazon Pinpoint collects reputation metrics for the emails
            that you send that use the configuration set.</p> |
| `tags` | Vec<String> | <p>An array of objects that define the tags (keys and values) that are associated with
            the configuration set.</p> |
| `sending_options` | String | <p>An object that defines whether or not Amazon Pinpoint can send email that you send using the
            configuration set.</p> |
| `configuration_set_name` | String | <p>The name of the configuration set.</p> |
| `tracking_options` | String | <p>An object that defines the open and click tracking options for emails that you send
            using the configuration set.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create configuration_set
configuration_set = provider.pinpoint_email.Configuration_set {
    configuration_set_name = "value"  # <p>The name of the configuration set.</p>
}

# Access configuration_set outputs
configuration_set_id = configuration_set.id
configuration_set_delivery_options = configuration_set.delivery_options
configuration_set_reputation_options = configuration_set.reputation_options
configuration_set_tags = configuration_set.tags
configuration_set_sending_options = configuration_set.sending_options
configuration_set_configuration_set_name = configuration_set.configuration_set_name
configuration_set_tracking_options = configuration_set.tracking_options
```

---


### Domain_deliverability_campaign

DomainDeliverabilityCampaign resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `domain_deliverability_campaign` | String | <p>An object that contains the deliverability data for the campaign.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access domain_deliverability_campaign outputs
domain_deliverability_campaign_id = domain_deliverability_campaign.id
domain_deliverability_campaign_domain_deliverability_campaign = domain_deliverability_campaign.domain_deliverability_campaign
```

---


### Configuration_set_event_destination

ConfigurationSetEventDestination resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `configuration_set_name` | String | ✅ | <p>The name of the configuration set that you want to add an event destination to.</p> |
| `event_destination_name` | String | ✅ | <p>A name that identifies the event destination within the configuration set.</p> |
| `event_destination` | String | ✅ | <p>An object that defines the event destination.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create configuration_set_event_destination
configuration_set_event_destination = provider.pinpoint_email.Configuration_set_event_destination {
    configuration_set_name = "value"  # <p>The name of the configuration set that you want to add an event destination to.</p>
    event_destination_name = "value"  # <p>A name that identifies the event destination within the configuration set.</p>
    event_destination = "value"  # <p>An object that defines the event destination.</p>
}

```

---


### Dedicated_ip_warmup_attributes

DedicatedIpWarmupAttributes resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `ip` | String | ✅ | <p>The dedicated IP address that you want to update the warm-up attributes for.</p> |
| `warmup_percentage` | i64 | ✅ | <p>The warm-up percentage that you want to associate with the dedicated IP
            address.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create dedicated_ip_warmup_attributes
dedicated_ip_warmup_attributes = provider.pinpoint_email.Dedicated_ip_warmup_attributes {
    ip = "value"  # <p>The dedicated IP address that you want to update the warm-up attributes for.</p>
    warmup_percentage = "value"  # <p>The warm-up percentage that you want to associate with the dedicated IP
            address.</p>
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

# Create multiple configuration_set_delivery_options resources
configuration_set_delivery_options_0 = provider.pinpoint_email.Configuration_set_delivery_options {
    configuration_set_name = "value-0"
}
configuration_set_delivery_options_1 = provider.pinpoint_email.Configuration_set_delivery_options {
    configuration_set_name = "value-1"
}
configuration_set_delivery_options_2 = provider.pinpoint_email.Configuration_set_delivery_options {
    configuration_set_name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    configuration_set_delivery_options = provider.pinpoint_email.Configuration_set_delivery_options {
        configuration_set_name = "production-value"
    }
```

---

## Related Documentation

- [AWS Pinpoint_email Documentation](https://docs.aws.amazon.com/pinpoint_email/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
