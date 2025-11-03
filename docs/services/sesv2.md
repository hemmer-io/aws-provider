# Sesv2 Service



**Resources**: 50

---

## Overview

The sesv2 service provides access to 50 resource types:

- [Blacklist_reports](#blacklist_reports) [R]
- [Account_dedicated_ip_warmup_attributes](#account_dedicated_ip_warmup_attributes) [C]
- [Dedicated_ip_in_pool](#dedicated_ip_in_pool) [C]
- [Import_job](#import_job) [CR]
- [Export_job](#export_job) [CR]
- [Suppressed_destination](#suppressed_destination) [CRD]
- [Account_vdm_attributes](#account_vdm_attributes) [C]
- [Configuration_set_sending_options](#configuration_set_sending_options) [C]
- [Configuration_set_suppression_options](#configuration_set_suppression_options) [C]
- [Reputation_entity](#reputation_entity) [R]
- [Deliverability_dashboard_options](#deliverability_dashboard_options) [R]
- [Contact](#contact) [CRUD]
- [Configuration_set_event_destinations](#configuration_set_event_destinations) [R]
- [Domain_statistics_report](#domain_statistics_report) [R]
- [Account_sending_attributes](#account_sending_attributes) [C]
- [Contact_list](#contact_list) [CRUD]
- [Multi_region_endpoint](#multi_region_endpoint) [CRD]
- [Configuration_set_archiving_options](#configuration_set_archiving_options) [C]
- [Dedicated_ip_pool_scaling_attributes](#dedicated_ip_pool_scaling_attributes) [C]
- [Email_identity_dkim_attributes](#email_identity_dkim_attributes) [C]
- [Email_identity_feedback_attributes](#email_identity_feedback_attributes) [C]
- [Configuration_set_reputation_options](#configuration_set_reputation_options) [C]
- [Reputation_entity_customer_managed_status](#reputation_entity_customer_managed_status) [U]
- [Dedicated_ip](#dedicated_ip) [R]
- [Email_template](#email_template) [CRUD]
- [Email_identity_dkim_signing_attributes](#email_identity_dkim_signing_attributes) [C]
- [Configuration_set_event_destination](#configuration_set_event_destination) [CUD]
- [Configuration_set_vdm_options](#configuration_set_vdm_options) [C]
- [Reputation_entity_policy](#reputation_entity_policy) [U]
- [Deliverability_dashboard_option](#deliverability_dashboard_option) [C]
- [Dedicated_ip_warmup_attributes](#dedicated_ip_warmup_attributes) [C]
- [Email_identity_policies](#email_identity_policies) [R]
- [Domain_deliverability_campaign](#domain_deliverability_campaign) [R]
- [Tenant_resource_association](#tenant_resource_association) [CD]
- [Deliverability_test_report](#deliverability_test_report) [CR]
- [Account](#account) [R]
- [Dedicated_ips](#dedicated_ips) [R]
- [Account_suppression_attributes](#account_suppression_attributes) [C]
- [Configuration_set_tracking_options](#configuration_set_tracking_options) [C]
- [Email_identity_configuration_set_attributes](#email_identity_configuration_set_attributes) [C]
- [Email_identity](#email_identity) [CRD]
- [Configuration_set](#configuration_set) [CRD]
- [Tenant](#tenant) [CRD]
- [Email_identity_mail_from_attributes](#email_identity_mail_from_attributes) [C]
- [Configuration_set_delivery_options](#configuration_set_delivery_options) [C]
- [Email_identity_policy](#email_identity_policy) [CUD]
- [Account_details](#account_details) [C]
- [Message_insights](#message_insights) [R]
- [Custom_verification_email_template](#custom_verification_email_template) [CRUD]
- [Dedicated_ip_pool](#dedicated_ip_pool) [CRD]

---

## Resources


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


### Account_dedicated_ip_warmup_attributes

AccountDedicatedIpWarmupAttributes resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `auto_warmup_enabled` | bool |  | <p>Enables or disables the automatic warm-up feature for dedicated IP addresses that are
            associated with your Amazon SES account in the current Amazon Web Services Region. Set to <code>true</code>
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
account_dedicated_ip_warmup_attributes = provider.sesv2.Account_dedicated_ip_warmup_attributes {
}

```

---


### Dedicated_ip_in_pool

DedicatedIpInPool resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `destination_pool_name` | String | ✅ | <p>The name of the IP pool that you want to add the dedicated IP address to. You have to
            specify an IP pool that already exists.</p> |
| `ip` | String | ✅ | <p>The IP address that you want to move to the dedicated IP pool. The value you specify
            has to be a dedicated IP address that's associated with your Amazon Web Services account.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create dedicated_ip_in_pool
dedicated_ip_in_pool = provider.sesv2.Dedicated_ip_in_pool {
    destination_pool_name = "value"  # <p>The name of the IP pool that you want to add the dedicated IP address to. You have to
            specify an IP pool that already exists.</p>
    ip = "value"  # <p>The IP address that you want to move to the dedicated IP pool. The value you specify
            has to be a dedicated IP address that's associated with your Amazon Web Services account.</p>
}

```

---


### Import_job

ImportJob resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `import_destination` | String | ✅ | <p>The destination for the import job.</p> |
| `import_data_source` | String | ✅ | <p>The data source for the import job.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `job_status` | String | <p>The status of the import job.</p> |
| `created_timestamp` | String | <p>The time stamp of when the import job was created.</p> |
| `import_data_source` | String | <p>The data source of the import job.</p> |
| `failure_info` | String | <p>The failure details about an import job.</p> |
| `processed_records_count` | i64 | <p>The current number of records processed.</p> |
| `completed_timestamp` | String | <p>The time stamp of when the import job was completed.</p> |
| `import_destination` | String | <p>The destination of the import job.</p> |
| `job_id` | String | <p>A string that represents the import job ID.</p> |
| `failed_records_count` | i64 | <p>The number of records that failed processing because of invalid input or other
            reasons.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create import_job
import_job = provider.sesv2.Import_job {
    import_destination = "value"  # <p>The destination for the import job.</p>
    import_data_source = "value"  # <p>The data source for the import job.</p>
}

# Access import_job outputs
import_job_id = import_job.id
import_job_job_status = import_job.job_status
import_job_created_timestamp = import_job.created_timestamp
import_job_import_data_source = import_job.import_data_source
import_job_failure_info = import_job.failure_info
import_job_processed_records_count = import_job.processed_records_count
import_job_completed_timestamp = import_job.completed_timestamp
import_job_import_destination = import_job.import_destination
import_job_job_id = import_job.job_id
import_job_failed_records_count = import_job.failed_records_count
```

---


### Export_job

ExportJob resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `export_data_source` | String | ✅ | <p>The data source for the export job.</p> |
| `export_destination` | String | ✅ | <p>The destination for the export job.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `export_destination` | String | <p>The destination of the export job.</p> |
| `completed_timestamp` | String | <p>The timestamp of when the export job was completed.</p> |
| `created_timestamp` | String | <p>The timestamp of when the export job was created.</p> |
| `export_data_source` | String | <p>The data source of the export job.</p> |
| `failure_info` | String | <p>The failure details about an export job.</p> |
| `statistics` | String | <p>The statistics about the export job.</p> |
| `export_source_type` | String | <p>The type of source of the export job.</p> |
| `job_status` | String | <p>The status of the export job.</p> |
| `job_id` | String | <p>The export job ID.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create export_job
export_job = provider.sesv2.Export_job {
    export_data_source = "value"  # <p>The data source for the export job.</p>
    export_destination = "value"  # <p>The destination for the export job.</p>
}

# Access export_job outputs
export_job_id = export_job.id
export_job_export_destination = export_job.export_destination
export_job_completed_timestamp = export_job.completed_timestamp
export_job_created_timestamp = export_job.created_timestamp
export_job_export_data_source = export_job.export_data_source
export_job_failure_info = export_job.failure_info
export_job_statistics = export_job.statistics
export_job_export_source_type = export_job.export_source_type
export_job_job_status = export_job.job_status
export_job_job_id = export_job.job_id
```

---


### Suppressed_destination

SuppressedDestination resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `reason` | String | ✅ | <p>The factors that should cause the email address to be added to the suppression list
            for your account.</p> |
| `email_address` | String | ✅ | <p>The email address that should be added to the suppression list for your
            account.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `suppressed_destination` | String | <p>An object containing information about the suppressed email address.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create suppressed_destination
suppressed_destination = provider.sesv2.Suppressed_destination {
    reason = "value"  # <p>The factors that should cause the email address to be added to the suppression list
            for your account.</p>
    email_address = "value"  # <p>The email address that should be added to the suppression list for your
            account.</p>
}

# Access suppressed_destination outputs
suppressed_destination_id = suppressed_destination.id
suppressed_destination_suppressed_destination = suppressed_destination.suppressed_destination
```

---


### Account_vdm_attributes

AccountVdmAttributes resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `vdm_attributes` | String | ✅ | <p>The VDM attributes that you wish to apply to your Amazon SES account.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create account_vdm_attributes
account_vdm_attributes = provider.sesv2.Account_vdm_attributes {
    vdm_attributes = "value"  # <p>The VDM attributes that you wish to apply to your Amazon SES account.</p>
}

```

---


### Configuration_set_sending_options

ConfigurationSetSendingOptions resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `configuration_set_name` | String | ✅ | <p>The name of the configuration set to enable or disable email sending for.</p> |
| `sending_enabled` | bool |  | <p>If <code>true</code>, email sending is enabled for the configuration set. If
                <code>false</code>, email sending is disabled for the configuration set.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create configuration_set_sending_options
configuration_set_sending_options = provider.sesv2.Configuration_set_sending_options {
    configuration_set_name = "value"  # <p>The name of the configuration set to enable or disable email sending for.</p>
}

```

---


### Configuration_set_suppression_options

ConfigurationSetSuppressionOptions resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `suppressed_reasons` | Vec<String> |  | <p>A list that contains the reasons that email addresses are automatically added to the
            suppression list for your account. This list can contain any or all of the
            following:</p>
         <ul>
            <li>
               <p>
                  <code>COMPLAINT</code> – Amazon SES adds an email address to the suppression
                    list for your account when a message sent to that address results in a
                    complaint.</p>
            </li>
            <li>
               <p>
                  <code>BOUNCE</code> – Amazon SES adds an email address to the suppression
                    list for your account when a message sent to that address results in a hard
                    bounce.</p>
            </li>
         </ul> |
| `configuration_set_name` | String | ✅ | <p>The name of the configuration set to change the suppression list preferences
            for.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create configuration_set_suppression_options
configuration_set_suppression_options = provider.sesv2.Configuration_set_suppression_options {
    configuration_set_name = "value"  # <p>The name of the configuration set to change the suppression list preferences
            for.</p>
}

```

---


### Reputation_entity

ReputationEntity resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `reputation_entity` | String | <p>The reputation entity information, including status records, policy configuration, 
            and reputation impact.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access reputation_entity outputs
reputation_entity_id = reputation_entity.id
reputation_entity_reputation_entity = reputation_entity.reputation_entity
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
| `account_status` | String | <p>The current status of your Deliverability dashboard subscription. If this value is
                <code>PENDING_EXPIRATION</code>, your subscription is scheduled to expire at the end
            of the current calendar month.</p> |
| `subscription_expiry_date` | String | <p>The date  when your current subscription to the Deliverability dashboard
            is scheduled to expire, if your subscription is scheduled to expire at the end of the
            current calendar month. This value is null if you have an active subscription that isn’t
            due to expire at the end of the month.</p> |
| `active_subscribed_domains` | Vec<String> | <p>An array of objects, one for each verified domain that you use to send email and
            currently has an active Deliverability dashboard subscription that isn’t scheduled to expire at
            the end of the current calendar month.</p> |
| `dashboard_enabled` | bool | <p>Specifies whether the Deliverability dashboard is enabled. If this value is <code>true</code>,
            the dashboard is enabled.</p> |
| `pending_expiration_subscribed_domains` | Vec<String> | <p>An array of objects, one for each verified domain that you use to send email and
            currently has an active Deliverability dashboard subscription that's scheduled to expire at the
            end of the current calendar month.</p> |


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
deliverability_dashboard_options_account_status = deliverability_dashboard_options.account_status
deliverability_dashboard_options_subscription_expiry_date = deliverability_dashboard_options.subscription_expiry_date
deliverability_dashboard_options_active_subscribed_domains = deliverability_dashboard_options.active_subscribed_domains
deliverability_dashboard_options_dashboard_enabled = deliverability_dashboard_options.dashboard_enabled
deliverability_dashboard_options_pending_expiration_subscribed_domains = deliverability_dashboard_options.pending_expiration_subscribed_domains
```

---


### Contact

Contact resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `contact_list_name` | String | ✅ | <p>The name of the contact list to which the contact should be added.</p> |
| `unsubscribe_all` | bool |  | <p>A boolean value status noting if the contact is unsubscribed from all contact list
            topics.</p> |
| `attributes_data` | String |  | <p>The attribute data attached to a contact.</p> |
| `topic_preferences` | Vec<String> |  | <p>The contact's preferences for being opted-in to or opted-out of topics.</p> |
| `email_address` | String | ✅ | <p>The contact's email address.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `topic_default_preferences` | Vec<String> | <p>The default topic preferences applied to the contact.</p> |
| `attributes_data` | String | <p>The attribute data attached to a contact.</p> |
| `last_updated_timestamp` | String | <p>A timestamp noting the last time the contact's information was updated.</p> |
| `contact_list_name` | String | <p>The name of the contact list to which the contact belongs.</p> |
| `created_timestamp` | String | <p>A timestamp noting when the contact was created.</p> |
| `email_address` | String | <p>The contact's email address.</p> |
| `topic_preferences` | Vec<String> | <p>The contact's preference for being opted-in to or opted-out of a topic.></p> |
| `unsubscribe_all` | bool | <p>A boolean value status noting if the contact is unsubscribed from all contact list
            topics.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create contact
contact = provider.sesv2.Contact {
    contact_list_name = "value"  # <p>The name of the contact list to which the contact should be added.</p>
    email_address = "value"  # <p>The contact's email address.</p>
}

# Access contact outputs
contact_id = contact.id
contact_topic_default_preferences = contact.topic_default_preferences
contact_attributes_data = contact.attributes_data
contact_last_updated_timestamp = contact.last_updated_timestamp
contact_contact_list_name = contact.contact_list_name
contact_created_timestamp = contact.created_timestamp
contact_email_address = contact.email_address
contact_topic_preferences = contact.topic_preferences
contact_unsubscribe_all = contact.unsubscribe_all
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


### Domain_statistics_report

DomainStatisticsReport resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `overall_volume` | String | <p>An object that contains deliverability metrics for the domain that you specified. The
            data in this object is a summary of all of the data that was collected from the
                <code>StartDate</code> to the <code>EndDate</code>.</p> |
| `daily_volumes` | Vec<String> | <p>An object that contains deliverability metrics for the domain that you specified. This
            object contains data for each day, starting on the <code>StartDate</code> and ending on
            the <code>EndDate</code>.</p> |


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
domain_statistics_report_overall_volume = domain_statistics_report.overall_volume
domain_statistics_report_daily_volumes = domain_statistics_report.daily_volumes
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
            <p>If Amazon Web Services paused your account's ability to send email, you can't use this operation
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
account_sending_attributes = provider.sesv2.Account_sending_attributes {
}

```

---


### Contact_list

ContactList resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `contact_list_name` | String | ✅ | <p>The name of the contact list.</p> |
| `tags` | Vec<String> |  | <p>The tags associated with a contact list.</p> |
| `topics` | Vec<String> |  | <p>An interest group, theme, or label within a list. A contact list can have multiple
            topics.</p> |
| `description` | String |  | <p>A description of what the contact list is about.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `created_timestamp` | String | <p>A timestamp noting when the contact list was created.</p> |
| `tags` | Vec<String> | <p>The tags associated with a contact list.</p> |
| `description` | String | <p>A description of what the contact list is about.</p> |
| `topics` | Vec<String> | <p>An interest group, theme, or label within a list. A contact list can have multiple
            topics.</p> |
| `last_updated_timestamp` | String | <p>A timestamp noting the last time the contact list was updated.</p> |
| `contact_list_name` | String | <p>The name of the contact list.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create contact_list
contact_list = provider.sesv2.Contact_list {
    contact_list_name = "value"  # <p>The name of the contact list.</p>
}

# Access contact_list outputs
contact_list_id = contact_list.id
contact_list_created_timestamp = contact_list.created_timestamp
contact_list_tags = contact_list.tags
contact_list_description = contact_list.description
contact_list_topics = contact_list.topics
contact_list_last_updated_timestamp = contact_list.last_updated_timestamp
contact_list_contact_list_name = contact_list.contact_list_name
```

---


### Multi_region_endpoint

MultiRegionEndpoint resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `details` | String | ✅ | <p>Contains details of a multi-region endpoint (global-endpoint) being created.</p> |
| `tags` | Vec<String> |  | <p>An array of objects that define the tags (keys and values) to associate with the multi-region endpoint (global-endpoint).</p> |
| `endpoint_name` | String | ✅ | <p>The name of the multi-region endpoint (global-endpoint).</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | String | <p>The status of the multi-region endpoint (global-endpoint).</p>
         <ul>
            <li>
               <p>
                  <code>CREATING</code> – The resource is being provisioned.</p>
            </li>
            <li>
               <p>
                  <code>READY</code> – The resource is ready to use.</p>
            </li>
            <li>
               <p>
                  <code>FAILED</code> – The resource failed to be provisioned.</p>
            </li>
            <li>
               <p>
                  <code>DELETING</code> – The resource is being deleted as requested.</p>
            </li>
         </ul> |
| `endpoint_name` | String | <p>The name of the multi-region endpoint (global-endpoint).</p> |
| `routes` | Vec<String> | <p>Contains routes information for the multi-region endpoint (global-endpoint).</p> |
| `created_timestamp` | String | <p>The time stamp of when the multi-region endpoint (global-endpoint) was created.</p> |
| `last_updated_timestamp` | String | <p>The time stamp of when the multi-region endpoint (global-endpoint) was last updated.</p> |
| `endpoint_id` | String | <p>The ID of the multi-region endpoint (global-endpoint).</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multi_region_endpoint
multi_region_endpoint = provider.sesv2.Multi_region_endpoint {
    details = "value"  # <p>Contains details of a multi-region endpoint (global-endpoint) being created.</p>
    endpoint_name = "value"  # <p>The name of the multi-region endpoint (global-endpoint).</p>
}

# Access multi_region_endpoint outputs
multi_region_endpoint_id = multi_region_endpoint.id
multi_region_endpoint_status = multi_region_endpoint.status
multi_region_endpoint_endpoint_name = multi_region_endpoint.endpoint_name
multi_region_endpoint_routes = multi_region_endpoint.routes
multi_region_endpoint_created_timestamp = multi_region_endpoint.created_timestamp
multi_region_endpoint_last_updated_timestamp = multi_region_endpoint.last_updated_timestamp
multi_region_endpoint_endpoint_id = multi_region_endpoint.endpoint_id
```

---


### Configuration_set_archiving_options

ConfigurationSetArchivingOptions resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `configuration_set_name` | String | ✅ | <p>The name of the configuration set to associate with a MailManager archive.</p> |
| `archive_arn` | String |  | <p>The Amazon Resource Name (ARN) of the MailManager archive that the Amazon SES API v2 sends email
            to.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create configuration_set_archiving_options
configuration_set_archiving_options = provider.sesv2.Configuration_set_archiving_options {
    configuration_set_name = "value"  # <p>The name of the configuration set to associate with a MailManager archive.</p>
}

```

---


### Dedicated_ip_pool_scaling_attributes

DedicatedIpPoolScalingAttributes resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `pool_name` | String | ✅ | <p>The name of the dedicated IP pool.</p> |
| `scaling_mode` | String | ✅ | <p>The scaling mode to apply to the dedicated IP pool.</p>
         <note>
            <p>Changing the scaling mode from <code>MANAGED</code> to <code>STANDARD</code> is not supported.</p>
         </note> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create dedicated_ip_pool_scaling_attributes
dedicated_ip_pool_scaling_attributes = provider.sesv2.Dedicated_ip_pool_scaling_attributes {
    pool_name = "value"  # <p>The name of the dedicated IP pool.</p>
    scaling_mode = "value"  # <p>The scaling mode to apply to the dedicated IP pool.</p>
         <note>
            <p>Changing the scaling mode from <code>MANAGED</code> to <code>STANDARD</code> is not supported.</p>
         </note>
}

```

---


### Email_identity_dkim_attributes

EmailIdentityDkimAttributes resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `email_identity` | String | ✅ | <p>The email identity.</p> |
| `signing_enabled` | bool |  | <p>Sets the DKIM signing configuration for the identity.</p>
         <p>When you set this value <code>true</code>, then the messages that are sent from the
            identity are signed using DKIM. If you set this value to <code>false</code>, your
            messages are sent without DKIM signing.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create email_identity_dkim_attributes
email_identity_dkim_attributes = provider.sesv2.Email_identity_dkim_attributes {
    email_identity = "value"  # <p>The email identity.</p>
}

```

---


### Email_identity_feedback_attributes

EmailIdentityFeedbackAttributes resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `email_identity` | String | ✅ | <p>The email identity.</p> |
| `email_forwarding_enabled` | bool |  | <p>Sets the feedback forwarding configuration for the identity.</p>
         <p>If the value is <code>true</code>, you receive email notifications when bounce or
            complaint events occur. These notifications are sent to the address that you specified
            in the <code>Return-Path</code> header of the original email.</p>
         <p>You're required to have a method of tracking bounces and complaints. If you haven't
            set up another mechanism for receiving bounce or complaint notifications (for example,
            by setting up an event destination), you receive an email notification when these events
            occur (even if this setting is disabled).</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create email_identity_feedback_attributes
email_identity_feedback_attributes = provider.sesv2.Email_identity_feedback_attributes {
    email_identity = "value"  # <p>The email identity.</p>
}

```

---


### Configuration_set_reputation_options

ConfigurationSetReputationOptions resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `configuration_set_name` | String | ✅ | <p>The name of the configuration set.</p> |
| `reputation_metrics_enabled` | bool |  | <p>If <code>true</code>, tracking of reputation metrics is enabled for the configuration
            set. If <code>false</code>, tracking of reputation metrics is disabled for the
            configuration set.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create configuration_set_reputation_options
configuration_set_reputation_options = provider.sesv2.Configuration_set_reputation_options {
    configuration_set_name = "value"  # <p>The name of the configuration set.</p>
}

```

---


### Reputation_entity_customer_managed_status

ReputationEntityCustomerManagedStatus resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `reputation_entity_reference` | String | ✅ | <p>The unique identifier for the reputation entity. For resource-type entities, 
            this is the Amazon Resource Name (ARN) of the resource.</p> |
| `sending_status` | String | ✅ | <p>The new customer-managed sending status for the reputation entity. This can be one of the following:</p>
         <ul>
            <li>
               <p>
                  <code>ENABLED</code> – Allow sending for this entity.</p>
            </li>
            <li>
               <p>
                  <code>DISABLED</code> – Prevent sending for this entity.</p>
            </li>
            <li>
               <p>
                  <code>REINSTATED</code> – Allow sending even if there are active reputation findings.</p>
            </li>
         </ul> |
| `reputation_entity_type` | String | ✅ | <p>The type of reputation entity. Currently, only <code>RESOURCE</code> type entities are supported.</p> |



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


### Email_template

EmailTemplate resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `template_content` | String | ✅ | <p>The content of the email template, composed of a subject line, an HTML part, and a
            text-only part.</p> |
| `template_name` | String | ✅ | <p>The name of the template.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `template_content` | String | <p>The content of the email template, composed of a subject line, an HTML part, and a
            text-only part.</p> |
| `template_name` | String | <p>The name of the template.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create email_template
email_template = provider.sesv2.Email_template {
    template_content = "value"  # <p>The content of the email template, composed of a subject line, an HTML part, and a
            text-only part.</p>
    template_name = "value"  # <p>The name of the template.</p>
}

# Access email_template outputs
email_template_id = email_template.id
email_template_template_content = email_template.template_content
email_template_template_name = email_template.template_name
```

---


### Email_identity_dkim_signing_attributes

EmailIdentityDkimSigningAttributes resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `signing_attributes` | String |  | <p>An object that contains information about the private key and selector that you want
            to use to configure DKIM for the identity for Bring Your Own DKIM (BYODKIM) for the
            identity, or, configures the key length to be used for <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/easy-dkim.html">Easy DKIM</a>.</p> |
| `signing_attributes_origin` | String | ✅ | <p>The method to use to configure DKIM for the identity. There are the following possible
            values:</p>
         <ul>
            <li>
               <p>
                  <code>AWS_SES</code> – Configure DKIM for the identity by using <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/easy-dkim.html">Easy
                        DKIM</a>.</p>
            </li>
            <li>
               <p>
                  <code>EXTERNAL</code> – Configure DKIM for the identity by using Bring
                    Your Own DKIM (BYODKIM).</p>
            </li>
         </ul> |
| `email_identity` | String | ✅ | <p>The email identity.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create email_identity_dkim_signing_attributes
email_identity_dkim_signing_attributes = provider.sesv2.Email_identity_dkim_signing_attributes {
    signing_attributes_origin = "value"  # <p>The method to use to configure DKIM for the identity. There are the following possible
            values:</p>
         <ul>
            <li>
               <p>
                  <code>AWS_SES</code> – Configure DKIM for the identity by using <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/easy-dkim.html">Easy
                        DKIM</a>.</p>
            </li>
            <li>
               <p>
                  <code>EXTERNAL</code> – Configure DKIM for the identity by using Bring
                    Your Own DKIM (BYODKIM).</p>
            </li>
         </ul>
    email_identity = "value"  # <p>The email identity.</p>
}

```

---


### Configuration_set_event_destination

ConfigurationSetEventDestination resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `event_destination` | String | ✅ | <p>An object that defines the event destination.</p> |
| `event_destination_name` | String | ✅ | <p>A name that identifies the event destination within the configuration set.</p> |
| `configuration_set_name` | String | ✅ | <p>The name of the configuration set .</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create configuration_set_event_destination
configuration_set_event_destination = provider.sesv2.Configuration_set_event_destination {
    event_destination = "value"  # <p>An object that defines the event destination.</p>
    event_destination_name = "value"  # <p>A name that identifies the event destination within the configuration set.</p>
    configuration_set_name = "value"  # <p>The name of the configuration set .</p>
}

```

---


### Configuration_set_vdm_options

ConfigurationSetVdmOptions resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `configuration_set_name` | String | ✅ | <p>The name of the configuration set.</p> |
| `vdm_options` | String |  | <p>The VDM options to apply to the configuration set.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create configuration_set_vdm_options
configuration_set_vdm_options = provider.sesv2.Configuration_set_vdm_options {
    configuration_set_name = "value"  # <p>The name of the configuration set.</p>
}

```

---


### Reputation_entity_policy

ReputationEntityPolicy resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `reputation_entity_reference` | String | ✅ | <p>The unique identifier for the reputation entity. For resource-type entities, 
            this is the Amazon Resource Name (ARN) of the resource.</p> |
| `reputation_entity_policy` | String | ✅ | <p>The Amazon Resource Name (ARN) of the reputation management policy to apply 
            to this entity. This is an Amazon Web Services Amazon SES-managed policy.</p> |
| `reputation_entity_type` | String | ✅ | <p>The type of reputation entity. Currently, only <code>RESOURCE</code> type entities are supported.</p> |



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


### Deliverability_dashboard_option

DeliverabilityDashboardOption resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `dashboard_enabled` | bool | ✅ | <p>Specifies whether to enable the Deliverability dashboard. To enable the dashboard, set this
            value to <code>true</code>.</p> |
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
deliverability_dashboard_option = provider.sesv2.Deliverability_dashboard_option {
    dashboard_enabled = "value"  # <p>Specifies whether to enable the Deliverability dashboard. To enable the dashboard, set this
            value to <code>true</code>.</p>
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
dedicated_ip_warmup_attributes = provider.sesv2.Dedicated_ip_warmup_attributes {
    ip = "value"  # <p>The dedicated IP address that you want to update the warm-up attributes for.</p>
    warmup_percentage = "value"  # <p>The warm-up percentage that you want to associate with the dedicated IP
            address.</p>
}

```

---


### Email_identity_policies

EmailIdentityPolicies resource

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

# Access email_identity_policies outputs
email_identity_policies_id = email_identity_policies.id
email_identity_policies_policies = email_identity_policies.policies
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


### Tenant_resource_association

TenantResourceAssociation resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resource_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the resource to associate with the tenant.</p> |
| `tenant_name` | String | ✅ | <p>The name of the tenant to associate the resource with.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create tenant_resource_association
tenant_resource_association = provider.sesv2.Tenant_resource_association {
    resource_arn = "value"  # <p>The Amazon Resource Name (ARN) of the resource to associate with the tenant.</p>
    tenant_name = "value"  # <p>The name of the tenant to associate the resource with.</p>
}

```

---


### Deliverability_test_report

DeliverabilityTestReport resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `report_name` | String |  | <p>A unique name that helps you to identify the predictive inbox placement test when you retrieve the
            results.</p> |
| `content` | String | ✅ | <p>The HTML body of the message that you sent when you performed the predictive inbox placement test.</p> |
| `tags` | Vec<String> |  | <p>An array of objects that define the tags (keys and values) that you want to associate
            with the predictive inbox placement test.</p> |
| `from_email_address` | String | ✅ | <p>The email address that the predictive inbox placement test email was sent from.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `isp_placements` | Vec<String> | <p>An object that describes how the test email was handled by several email providers,
            including Gmail, Hotmail, Yahoo, AOL, and others.</p> |
| `message` | String | <p>An object that contains the message that you sent when you performed this
            predictive inbox placement test.</p> |
| `tags` | Vec<String> | <p>An array of objects that define the tags (keys and values) that are associated with
            the predictive inbox placement test.</p> |
| `deliverability_test_report` | String | <p>An object that contains the results of the predictive inbox placement test.</p> |
| `overall_placement` | String | <p>An object that specifies how many test messages that were sent during the predictive inbox placement test were
            delivered to recipients' inboxes, how many were sent to recipients' spam folders, and
            how many weren't delivered.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create deliverability_test_report
deliverability_test_report = provider.sesv2.Deliverability_test_report {
    content = "value"  # <p>The HTML body of the message that you sent when you performed the predictive inbox placement test.</p>
    from_email_address = "value"  # <p>The email address that the predictive inbox placement test email was sent from.</p>
}

# Access deliverability_test_report outputs
deliverability_test_report_id = deliverability_test_report.id
deliverability_test_report_isp_placements = deliverability_test_report.isp_placements
deliverability_test_report_message = deliverability_test_report.message
deliverability_test_report_tags = deliverability_test_report.tags
deliverability_test_report_deliverability_test_report = deliverability_test_report.deliverability_test_report
deliverability_test_report_overall_placement = deliverability_test_report.overall_placement
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
| `details` | String | <p>An object that defines your account details.</p> |
| `enforcement_status` | String | <p>The reputation status of your Amazon SES account. The status can be one of the
            following:</p>
         <ul>
            <li>
               <p>
                  <code>HEALTHY</code> – There are no reputation-related issues that
                    currently impact your account.</p>
            </li>
            <li>
               <p>
                  <code>PROBATION</code> – We've identified potential issues with your
                    Amazon SES account. We're placing your account under review while you work on
                    correcting these issues.</p>
            </li>
            <li>
               <p>
                  <code>SHUTDOWN</code> – Your account's ability to send email is
                    currently paused because of an issue with the email sent from your account. When
                    you correct the issue, you can contact us and request that your account's
                    ability to send email is resumed.</p>
            </li>
         </ul> |
| `dedicated_ip_auto_warmup_enabled` | bool | <p>Indicates whether or not the automatic warm-up feature is enabled for dedicated IP
            addresses that are associated with your account.</p> |
| `sending_enabled` | bool | <p>Indicates whether or not email sending is enabled for your Amazon SES account in the
            current Amazon Web Services Region.</p> |
| `vdm_attributes` | String | <p>The VDM attributes that apply to your Amazon SES account.</p> |
| `production_access_enabled` | bool | <p>Indicates whether or not your account has production access in the current Amazon Web Services
            Region.</p>
         <p>If the value is <code>false</code>, then your account is in the
                <i>sandbox</i>. When your account is in the sandbox, you can only send
            email to verified identities.
            </p>
         <p>If the value is <code>true</code>, then your account has production access. When your
            account has production access, you can send email to any address. The sending quota and
            maximum sending rate for your account vary based on your specific use case.</p> |
| `send_quota` | String | <p>An object that contains information about the per-day and per-second sending limits
            for your Amazon SES account in the current Amazon Web Services Region.</p> |
| `suppression_attributes` | String | <p>An object that contains information about the email address suppression preferences
            for your account in the current Amazon Web Services Region.</p> |


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
account_details = account.details
account_enforcement_status = account.enforcement_status
account_dedicated_ip_auto_warmup_enabled = account.dedicated_ip_auto_warmup_enabled
account_sending_enabled = account.sending_enabled
account_vdm_attributes = account.vdm_attributes
account_production_access_enabled = account.production_access_enabled
account_send_quota = account.send_quota
account_suppression_attributes = account.suppression_attributes
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
| `dedicated_ips` | Vec<String> | <p>A list of dedicated IP addresses that are associated with your Amazon Web Services account.</p> |
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


### Account_suppression_attributes

AccountSuppressionAttributes resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `suppressed_reasons` | Vec<String> |  | <p>A list that contains the reasons that email addresses will be automatically added to
            the suppression list for your account. This list can contain any or all of the
            following:</p>
         <ul>
            <li>
               <p>
                  <code>COMPLAINT</code> – Amazon SES adds an email address to the suppression
                    list for your account when a message sent to that address results in a
                    complaint.</p>
            </li>
            <li>
               <p>
                  <code>BOUNCE</code> – Amazon SES adds an email address to the suppression
                    list for your account when a message sent to that address results in a hard
                    bounce.</p>
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

# Create account_suppression_attributes
account_suppression_attributes = provider.sesv2.Account_suppression_attributes {
}

```

---


### Configuration_set_tracking_options

ConfigurationSetTrackingOptions resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `configuration_set_name` | String | ✅ | <p>The name of the configuration set.</p> |
| `https_policy` | String |  |  |
| `custom_redirect_domain` | String |  | <p>The domain to use to track open and click events.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create configuration_set_tracking_options
configuration_set_tracking_options = provider.sesv2.Configuration_set_tracking_options {
    configuration_set_name = "value"  # <p>The name of the configuration set.</p>
}

```

---


### Email_identity_configuration_set_attributes

EmailIdentityConfigurationSetAttributes resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `email_identity` | String | ✅ | <p>The email address or domain to associate with a configuration set.</p> |
| `configuration_set_name` | String |  | <p>The configuration set to associate with an email identity.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create email_identity_configuration_set_attributes
email_identity_configuration_set_attributes = provider.sesv2.Email_identity_configuration_set_attributes {
    email_identity = "value"  # <p>The email address or domain to associate with a configuration set.</p>
}

```

---


### Email_identity

EmailIdentity resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `dkim_signing_attributes` | String |  | <p>If your request includes this object, Amazon SES configures the identity to use Bring Your
            Own DKIM (BYODKIM) for DKIM authentication purposes, or, configures the key length to be
            used for <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/easy-dkim.html">Easy
                DKIM</a>.</p>
         <p>You can only specify this object if the email identity is a domain, as opposed to an
            address.</p> |
| `email_identity` | String | ✅ | <p>The email address or domain to verify.</p> |
| `configuration_set_name` | String |  | <p>The configuration set to use by default when sending from this identity. Note that any
            configuration set defined in the email sending request takes precedence. </p> |
| `tags` | Vec<String> |  | <p>An array of objects that define the tags (keys and values) to associate with the email
            identity.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `configuration_set_name` | String | <p>The configuration set used by default when sending from this identity.</p> |
| `identity_type` | String | <p>The email identity type. Note: the <code>MANAGED_DOMAIN</code> identity type is not
            supported.</p> |
| `mail_from_attributes` | String | <p>An object that contains information about the Mail-From attributes for the email
            identity.</p> |
| `policies` | HashMap<String, String> | <p>A map of policy names to policies.</p> |
| `dkim_attributes` | String | <p>An object that contains information about the DKIM attributes for the identity.</p> |
| `verified_for_sending_status` | bool | <p>Specifies whether or not the identity is verified. You can only send email from
            verified email addresses or domains. For more information about verifying identities,
            see the <a href="https://docs.aws.amazon.com/pinpoint/latest/userguide/channels-email-manage-verify.html">Amazon Pinpoint User Guide</a>.</p> |
| `verification_status` | String | <p>The verification status of the identity. The status can be one of the following:</p>
         <ul>
            <li>
               <p>
                  <code>PENDING</code> – The verification process was initiated, but Amazon SES
                    hasn't yet been able to verify the identity.</p>
            </li>
            <li>
               <p>
                  <code>SUCCESS</code> – The verification process completed
                    successfully.</p>
            </li>
            <li>
               <p>
                  <code>FAILED</code> – The verification process failed.</p>
            </li>
            <li>
               <p>
                  <code>TEMPORARY_FAILURE</code> – A temporary issue is preventing Amazon SES
                    from determining the verification status of the identity.</p>
            </li>
            <li>
               <p>
                  <code>NOT_STARTED</code> – The verification process hasn't been
                    initiated for the identity.</p>
            </li>
         </ul> |
| `verification_info` | String | <p>An object that contains additional information about the verification status for the
            identity.</p> |
| `feedback_forwarding_status` | bool | <p>The feedback forwarding configuration for the identity.</p>
         <p>If the value is <code>true</code>, you receive email notifications when bounce or
            complaint events occur. These notifications are sent to the address that you specified
            in the <code>Return-Path</code> header of the original email.</p>
         <p>You're required to have a method of tracking bounces and complaints. If you haven't
            set up another mechanism for receiving bounce or complaint notifications (for example,
            by setting up an event destination), you receive an email notification when these events
            occur (even if this setting is disabled).</p> |
| `tags` | Vec<String> | <p>An array of objects that define the tags (keys and values) that are associated with
            the email identity.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create email_identity
email_identity = provider.sesv2.Email_identity {
    email_identity = "value"  # <p>The email address or domain to verify.</p>
}

# Access email_identity outputs
email_identity_id = email_identity.id
email_identity_configuration_set_name = email_identity.configuration_set_name
email_identity_identity_type = email_identity.identity_type
email_identity_mail_from_attributes = email_identity.mail_from_attributes
email_identity_policies = email_identity.policies
email_identity_dkim_attributes = email_identity.dkim_attributes
email_identity_verified_for_sending_status = email_identity.verified_for_sending_status
email_identity_verification_status = email_identity.verification_status
email_identity_verification_info = email_identity.verification_info
email_identity_feedback_forwarding_status = email_identity.feedback_forwarding_status
email_identity_tags = email_identity.tags
```

---


### Configuration_set

ConfigurationSet resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `suppression_options` | String |  |  |
| `sending_options` | String |  | <p>An object that defines whether or not Amazon SES can send email that you send using the
            configuration set.</p> |
| `configuration_set_name` | String | ✅ | <p>The name of the configuration set. The name can contain up to 64 alphanumeric
            characters, including letters, numbers, hyphens (-) and underscores (_) only.</p> |
| `tags` | Vec<String> |  | <p>An array of objects that define the tags (keys and values) to associate with the
            configuration set.</p> |
| `archiving_options` | String |  | <p>An object that defines the MailManager archiving options for emails that you send
            using the configuration set.</p> |
| `vdm_options` | String |  | <p>An object that defines the VDM options for emails that you send using the
            configuration set.</p> |
| `tracking_options` | String |  | <p>An object that defines the open and click tracking options for emails that you send
            using the configuration set.</p> |
| `delivery_options` | String |  | <p>An object that defines the dedicated IP pool that is used to send emails that you send
            using the configuration set.</p> |
| `reputation_options` | String |  | <p>An object that defines whether or not Amazon SES collects reputation metrics for the emails
            that you send that use the configuration set.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `tracking_options` | String | <p>An object that defines the open and click tracking options for emails that you send
            using the configuration set.</p> |
| `delivery_options` | String | <p>An object that defines the dedicated IP pool that is used to send emails that you send
            using the configuration set.</p> |
| `tags` | Vec<String> | <p>An array of objects that define the tags (keys and values) that are associated with
            the configuration set.</p> |
| `sending_options` | String | <p>An object that defines whether or not Amazon SES can send email that you send using the
            configuration set.</p> |
| `configuration_set_name` | String | <p>The name of the configuration set.</p> |
| `suppression_options` | String | <p>An object that contains information about the suppression list preferences for your
            account.</p> |
| `vdm_options` | String | <p>An object that contains information about the VDM preferences for your configuration
            set.</p> |
| `archiving_options` | String | <p>An object that defines the MailManager archive where sent emails are archived that you send
            using the configuration set.</p> |
| `reputation_options` | String | <p>An object that defines whether or not Amazon SES collects reputation metrics for the emails
            that you send that use the configuration set.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create configuration_set
configuration_set = provider.sesv2.Configuration_set {
    configuration_set_name = "value"  # <p>The name of the configuration set. The name can contain up to 64 alphanumeric
            characters, including letters, numbers, hyphens (-) and underscores (_) only.</p>
}

# Access configuration_set outputs
configuration_set_id = configuration_set.id
configuration_set_tracking_options = configuration_set.tracking_options
configuration_set_delivery_options = configuration_set.delivery_options
configuration_set_tags = configuration_set.tags
configuration_set_sending_options = configuration_set.sending_options
configuration_set_configuration_set_name = configuration_set.configuration_set_name
configuration_set_suppression_options = configuration_set.suppression_options
configuration_set_vdm_options = configuration_set.vdm_options
configuration_set_archiving_options = configuration_set.archiving_options
configuration_set_reputation_options = configuration_set.reputation_options
```

---


### Tenant

Tenant resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tenant_name` | String | ✅ | <p>The name of the tenant to create. The name can contain up to 64 alphanumeric
            characters, including letters, numbers, hyphens (-) and underscores (_) only.</p> |
| `tags` | Vec<String> |  | <p>An array of objects that define the tags (keys and values) to associate with the tenant</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `tenant` | String | <p>A structure that contains details about the tenant.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create tenant
tenant = provider.sesv2.Tenant {
    tenant_name = "value"  # <p>The name of the tenant to create. The name can contain up to 64 alphanumeric
            characters, including letters, numbers, hyphens (-) and underscores (_) only.</p>
}

# Access tenant outputs
tenant_id = tenant.id
tenant_tenant = tenant.tenant
```

---


### Email_identity_mail_from_attributes

EmailIdentityMailFromAttributes resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
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
| `email_identity` | String | ✅ | <p>The verified email identity.</p> |
| `behavior_on_mx_failure` | String |  | <p>The action to take if the required MX record isn't found when you send an email. When
            you set this value to <code>UseDefaultValue</code>, the mail is sent using
                <i>amazonses.com</i> as the MAIL FROM domain. When you set this value
            to <code>RejectMessage</code>, the Amazon SES API v2 returns a
                <code>MailFromDomainNotVerified</code> error, and doesn't attempt to deliver the
            email.</p>
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
email_identity_mail_from_attributes = provider.sesv2.Email_identity_mail_from_attributes {
    email_identity = "value"  # <p>The verified email identity.</p>
}

```

---


### Configuration_set_delivery_options

ConfigurationSetDeliveryOptions resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tls_policy` | String |  | <p>Specifies whether messages that use the configuration set are required to use
            Transport Layer Security (TLS). If the value is <code>Require</code>, messages are only
            delivered if a TLS connection can be established. If the value is <code>Optional</code>,
            messages can be delivered in plain text if a TLS connection can't be established.</p> |
| `sending_pool_name` | String |  | <p>The name of the dedicated IP pool to associate with the configuration set.</p> |
| `max_delivery_seconds` | i64 |  | <p>The maximum amount of time, in seconds, that Amazon SES API v2 will attempt delivery of email.
            If specified, the value must greater than or equal to 300 seconds (5 minutes)
            and less than or equal to 50400 seconds (840 minutes).
        </p> |
| `configuration_set_name` | String | ✅ | <p>The name of the configuration set to associate with a dedicated IP pool.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create configuration_set_delivery_options
configuration_set_delivery_options = provider.sesv2.Configuration_set_delivery_options {
    configuration_set_name = "value"  # <p>The name of the configuration set to associate with a dedicated IP pool.</p>
}

```

---


### Email_identity_policy

EmailIdentityPolicy resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `policy_name` | String | ✅ | <p>The name of the policy.</p>
         <p>The policy name cannot exceed 64 characters and can only include alphanumeric
            characters, dashes, and underscores.</p> |
| `email_identity` | String | ✅ | <p>The email identity.</p> |
| `policy` | String | ✅ | <p>The text of the policy in JSON format. The policy cannot exceed 4 KB.</p>
         <p>For information about the syntax of sending authorization policies, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization-policies.html">Amazon SES Developer
                Guide</a>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create email_identity_policy
email_identity_policy = provider.sesv2.Email_identity_policy {
    policy_name = "value"  # <p>The name of the policy.</p>
         <p>The policy name cannot exceed 64 characters and can only include alphanumeric
            characters, dashes, and underscores.</p>
    email_identity = "value"  # <p>The email identity.</p>
    policy = "value"  # <p>The text of the policy in JSON format. The policy cannot exceed 4 KB.</p>
         <p>For information about the syntax of sending authorization policies, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization-policies.html">Amazon SES Developer
                Guide</a>.</p>
}

```

---


### Account_details

AccountDetails resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `contact_language` | String |  | <p>The language you would prefer to be contacted with.</p> |
| `website_url` | String | ✅ | <p>The URL of your website. This information helps us better understand the type of
            content that you plan to send.</p> |
| `use_case_description` | String |  | <p>A description of the types of email that you plan to send.</p> |
| `additional_contact_email_addresses` | Vec<String> |  | <p>Additional email addresses that you would like to be notified regarding Amazon SES
            matters.</p> |
| `production_access_enabled` | bool |  | <p>Indicates whether or not your account should have production access in the current
            Amazon Web Services Region.</p>
         <p>If the value is <code>false</code>, then your account is in the
                <i>sandbox</i>. When your account is in the sandbox, you can only send
            email to verified identities.
            </p>
         <p>If the value is <code>true</code>, then your account has production access. When your
            account has production access, you can send email to any address. The sending quota and
            maximum sending rate for your account vary based on your specific use case.</p> |
| `mail_type` | String | ✅ | <p>The type of email your account will send.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create account_details
account_details = provider.sesv2.Account_details {
    website_url = "value"  # <p>The URL of your website. This information helps us better understand the type of
            content that you plan to send.</p>
    mail_type = "value"  # <p>The type of email your account will send.</p>
}

```

---


### Message_insights

MessageInsights resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `from_email_address` | String | <p>The from address used to send the message.</p> |
| `message_id` | String | <p>A unique identifier for the message.</p> |
| `subject` | String | <p>The subject line of the message.</p> |
| `email_tags` | Vec<String> | <p>
            A list of tags, in the form of name/value pairs, that were applied to the email you sent, along with Amazon SES
            <a href="https://docs.aws.amazon.com/ses/latest/dg/monitor-using-event-publishing.html">Auto-Tags</a>.
        </p> |
| `insights` | Vec<String> | <p>A set of insights associated with the message.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access message_insights outputs
message_insights_id = message_insights.id
message_insights_from_email_address = message_insights.from_email_address
message_insights_message_id = message_insights.message_id
message_insights_subject = message_insights.subject
message_insights_email_tags = message_insights.email_tags
message_insights_insights = message_insights.insights
```

---


### Custom_verification_email_template

CustomVerificationEmailTemplate resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `template_subject` | String | ✅ | <p>The subject line of the custom verification email.</p> |
| `from_email_address` | String | ✅ | <p>The email address that the custom verification email is sent from.</p> |
| `failure_redirection_url` | String | ✅ | <p>The URL that the recipient of the verification email is sent to if his or her address
            is not successfully verified.</p> |
| `template_name` | String | ✅ | <p>The name of the custom verification email template.</p> |
| `template_content` | String | ✅ | <p>The content of the custom verification email. The total size of the email must be less
            than 10 MB. The message body may contain HTML, with some limitations. For more
            information, see <a href="https://docs.aws.amazon.com/ses/latest/dg/creating-identities.html#send-email-verify-address-custom-faq">Custom verification email frequently asked questions</a> in the <i>Amazon SES
                Developer Guide</i>.</p> |
| `success_redirection_url` | String | ✅ | <p>The URL that the recipient of the verification email is sent to if his or her address
            is successfully verified.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `template_name` | String | <p>The name of the custom verification email template.</p> |
| `template_subject` | String | <p>The subject line of the custom verification email.</p> |
| `template_content` | String | <p>The content of the custom verification email.</p> |
| `failure_redirection_url` | String | <p>The URL that the recipient of the verification email is sent to if his or her address
            is not successfully verified.</p> |
| `from_email_address` | String | <p>The email address that the custom verification email is sent from.</p> |
| `success_redirection_url` | String | <p>The URL that the recipient of the verification email is sent to if his or her address
            is successfully verified.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create custom_verification_email_template
custom_verification_email_template = provider.sesv2.Custom_verification_email_template {
    template_subject = "value"  # <p>The subject line of the custom verification email.</p>
    from_email_address = "value"  # <p>The email address that the custom verification email is sent from.</p>
    failure_redirection_url = "value"  # <p>The URL that the recipient of the verification email is sent to if his or her address
            is not successfully verified.</p>
    template_name = "value"  # <p>The name of the custom verification email template.</p>
    template_content = "value"  # <p>The content of the custom verification email. The total size of the email must be less
            than 10 MB. The message body may contain HTML, with some limitations. For more
            information, see <a href="https://docs.aws.amazon.com/ses/latest/dg/creating-identities.html#send-email-verify-address-custom-faq">Custom verification email frequently asked questions</a> in the <i>Amazon SES
                Developer Guide</i>.</p>
    success_redirection_url = "value"  # <p>The URL that the recipient of the verification email is sent to if his or her address
            is successfully verified.</p>
}

# Access custom_verification_email_template outputs
custom_verification_email_template_id = custom_verification_email_template.id
custom_verification_email_template_template_name = custom_verification_email_template.template_name
custom_verification_email_template_template_subject = custom_verification_email_template.template_subject
custom_verification_email_template_template_content = custom_verification_email_template.template_content
custom_verification_email_template_failure_redirection_url = custom_verification_email_template.failure_redirection_url
custom_verification_email_template_from_email_address = custom_verification_email_template.from_email_address
custom_verification_email_template_success_redirection_url = custom_verification_email_template.success_redirection_url
```

---


### Dedicated_ip_pool

DedicatedIpPool resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `pool_name` | String | ✅ | <p>The name of the dedicated IP pool.</p> |
| `tags` | Vec<String> |  | <p>An object that defines the tags (keys and values) that you want to associate with the
            pool.</p> |
| `scaling_mode` | String |  | <p>The type of scaling mode.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `dedicated_ip_pool` | String | <p>An object that contains information about a dedicated IP pool.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create dedicated_ip_pool
dedicated_ip_pool = provider.sesv2.Dedicated_ip_pool {
    pool_name = "value"  # <p>The name of the dedicated IP pool.</p>
}

# Access dedicated_ip_pool outputs
dedicated_ip_pool_id = dedicated_ip_pool.id
dedicated_ip_pool_dedicated_ip_pool = dedicated_ip_pool.dedicated_ip_pool
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple blacklist_reports resources
blacklist_reports_0 = provider.sesv2.Blacklist_reports {
}
blacklist_reports_1 = provider.sesv2.Blacklist_reports {
}
blacklist_reports_2 = provider.sesv2.Blacklist_reports {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    blacklist_reports = provider.sesv2.Blacklist_reports {
    }
```

---

## Related Documentation

- [AWS Sesv2 Documentation](https://docs.aws.amazon.com/sesv2/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
