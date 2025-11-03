# Cloudtrail Service



**Resources**: 13

---

## Overview

The cloudtrail service provides access to 13 resource types:

- [Event_data_store](#event_data_store) [CRUD]
- [Trail_status](#trail_status) [R]
- [Channel](#channel) [CRUD]
- [Trails](#trails) [R]
- [Dashboard](#dashboard) [CRUD]
- [Query_results](#query_results) [R]
- [Import](#import) [R]
- [Insight_selectors](#insight_selectors) [CR]
- [Trail](#trail) [CRUD]
- [Resource_policy](#resource_policy) [CRD]
- [Event_configuration](#event_configuration) [CR]
- [Event_selectors](#event_selectors) [CR]
- [Query](#query) [R]

---

## Resources


### Event_data_store

EventDataStore resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>The name of the event data store.</p> |
| `organization_enabled` | bool |  | <p>Specifies whether an event data store collects events logged for an organization in
            Organizations.</p> |
| `retention_period` | i64 |  | <p>The retention period of the event data store, in days. If <code>BillingMode</code> is set to <code>EXTENDABLE_RETENTION_PRICING</code>, you can set a retention period of
         up to 3653 days, the equivalent of 10 years. If <code>BillingMode</code> is set to <code>FIXED_RETENTION_PRICING</code>, you can set a retention period of
         up to 2557 days, the equivalent of seven years.</p>
         <p>CloudTrail  Lake determines whether to retain an event by checking if the <code>eventTime</code> 
         of the event is within the specified retention period. For example, if you set a retention period of 90 days, CloudTrail will remove events 
      when the <code>eventTime</code> is older than 90 days.</p>
         <note>
            <p>If you plan to copy trail events to this event data store, we recommend 
            that you consider both the age of the events that you
            want to copy as well as how long you want to keep the copied events
            in your event data store. For example, if you copy trail events that are 5 years old 
            and specify a retention period of 7 years, the event data store 
            will retain those events for two years.</p>
         </note> |
| `tags_list` | Vec<String> |  |  |
| `kms_key_id` | String |  | <p>Specifies the KMS key ID to use to encrypt the events delivered by
            CloudTrail. The value can be an alias name prefixed by <code>alias/</code>, a
         fully specified ARN to an alias, a fully specified ARN to a key, or a globally unique
         identifier.</p>
         <important>
            <p>Disabling or deleting the KMS key, or removing CloudTrail
            permissions on the key, prevents CloudTrail from logging events to the event data
            store, and prevents users from querying the data in the event data store that was
            encrypted with the key. After you associate an event data store with a KMS key, the KMS key cannot be removed or changed. Before you
            disable or delete a KMS key that you are using with an event data store,
            delete or back up your event data store.</p>
         </important>
         <p>CloudTrail also supports KMS multi-Region keys. For more
         information about multi-Region keys, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/multi-region-keys-overview.html">Using multi-Region
            keys</a> in the <i>Key Management Service Developer Guide</i>.</p>
         <p>Examples:</p>
         <ul>
            <li>
               <p>
                  <code>alias/MyAliasName</code>
               </p>
            </li>
            <li>
               <p>
                  <code>arn:aws:kms:us-east-2:123456789012:alias/MyAliasName</code>
               </p>
            </li>
            <li>
               <p>
                  <code>arn:aws:kms:us-east-2:123456789012:key/12345678-1234-1234-1234-123456789012</code>
               </p>
            </li>
            <li>
               <p>
                  <code>12345678-1234-1234-1234-123456789012</code>
               </p>
            </li>
         </ul> |
| `billing_mode` | String |  | <p>The billing mode for the event data store determines the cost for ingesting events and the default and maximum retention period for the event data store.</p>
         <p>The following are the possible values:</p>
         <ul>
            <li>
               <p>
                  <code>EXTENDABLE_RETENTION_PRICING</code> - This billing mode is generally recommended if you want a flexible retention period of up to 3653 days (about 10 years). 
                The default retention period for this billing mode is 366 days.</p>
            </li>
            <li>
               <p>
                  <code>FIXED_RETENTION_PRICING</code> - This billing mode is recommended if you expect to ingest more than 25 TB of event data per month and need a retention period of up to 2557 days (about 7 years). 
                The default retention period for this billing mode is 2557 days.</p>
            </li>
         </ul>
         <p>The default value is <code>EXTENDABLE_RETENTION_PRICING</code>.</p>
         <p>For more information about CloudTrail pricing, 
         see <a href="http://aws.amazon.com/cloudtrail/pricing/">CloudTrail Pricing</a> and 
         <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/cloudtrail-lake-manage-costs.html">Managing CloudTrail Lake costs</a>.</p> |
| `termination_protection_enabled` | bool |  | <p>Specifies whether termination protection is enabled for the event data store. If
         termination protection is enabled, you cannot delete the event data store until termination
         protection is disabled.</p> |
| `advanced_event_selectors` | Vec<String> |  | <p>The advanced event selectors to use to select the events for the data store. You can
         configure up to five advanced event selectors for each event data store.</p>
         <p> For more information about how to use advanced event selectors to log CloudTrail
         events, see <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/logging-data-events-with-cloudtrail.html#creating-data-event-selectors-advanced">Log events by using advanced event selectors</a> in the CloudTrail User Guide.</p>
         <p>For more information about how to use advanced event selectors to include Config configuration items in your event data store, see <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/lake-eds-cli.html#lake-cli-create-eds-config">Create an event data store for Config configuration
            items</a> in the CloudTrail User Guide.</p>
         <p>For more information about how to use advanced event selectors to include events outside of Amazon Web Services events in your event data store, see <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/lake-integrations-cli.html#lake-cli-create-integration">Create an integration to log events from outside Amazon Web Services</a> in the CloudTrail User Guide.</p> |
| `start_ingestion` | bool |  | <p>Specifies whether the event data store should start ingesting live events. The default is true.</p> |
| `multi_region_enabled` | bool |  | <p>Specifies whether the event data store includes events from all Regions, or only from
         the Region in which the event data store is created.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `billing_mode` | String | <p>The billing mode for the event data store.</p> |
| `termination_protection_enabled` | bool | <p>Indicates that termination protection is enabled.</p> |
| `multi_region_enabled` | bool | <p>Indicates whether the event data store includes events from all Regions, or only from
         the Region in which it was created.</p> |
| `status` | String | <p>The status of an event data store.</p> |
| `updated_timestamp` | String | <p>Shows the time that an event data store was updated, if applicable.
            <code>UpdatedTimestamp</code> is always either the same or newer than the time shown in
            <code>CreatedTimestamp</code>.</p> |
| `created_timestamp` | String | <p>The timestamp of the event data store's creation.</p> |
| `partition_keys` | Vec<String> | <p>The partition keys for the event data store. To improve query performance and efficiency, CloudTrail Lake organizes 
         event data into partitions based on values derived from partition keys.</p> |
| `advanced_event_selectors` | Vec<String> | <p>The advanced event selectors used to select events for the data store.</p> |
| `federation_role_arn` | String | <p>
         If Lake query federation is enabled, provides the ARN of the federation role used to access the resources for the federated event data store.
      </p> |
| `event_data_store_arn` | String | <p>The event data store Amazon Resource Number (ARN).</p> |
| `organization_enabled` | bool | <p>Indicates whether an event data store is collecting logged events for an organization in
            Organizations.</p> |
| `name` | String | <p>The name of the event data store.</p> |
| `retention_period` | i64 | <p>The retention period of the event data store, in days.</p> |
| `kms_key_id` | String | <p>Specifies the KMS key ID that encrypts the events delivered by CloudTrail. The value is a fully specified ARN to a KMS key in the
         following format.</p>
         <p>
            <code>arn:aws:kms:us-east-2:123456789012:key/12345678-1234-1234-1234-123456789012</code>
         </p> |
| `federation_status` | String | <p>
         Indicates the <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/query-federation.html">Lake query federation</a> status. The status is 
         <code>ENABLED</code> if Lake query federation is enabled, or <code>DISABLED</code> if Lake query federation is disabled. You cannot delete an event data store if the <code>FederationStatus</code> is <code>ENABLED</code>.
      </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create event_data_store
event_data_store = provider.cloudtrail.Event_data_store {
    name = "value"  # <p>The name of the event data store.</p>
}

# Access event_data_store outputs
event_data_store_id = event_data_store.id
event_data_store_billing_mode = event_data_store.billing_mode
event_data_store_termination_protection_enabled = event_data_store.termination_protection_enabled
event_data_store_multi_region_enabled = event_data_store.multi_region_enabled
event_data_store_status = event_data_store.status
event_data_store_updated_timestamp = event_data_store.updated_timestamp
event_data_store_created_timestamp = event_data_store.created_timestamp
event_data_store_partition_keys = event_data_store.partition_keys
event_data_store_advanced_event_selectors = event_data_store.advanced_event_selectors
event_data_store_federation_role_arn = event_data_store.federation_role_arn
event_data_store_event_data_store_arn = event_data_store.event_data_store_arn
event_data_store_organization_enabled = event_data_store.organization_enabled
event_data_store_name = event_data_store.name
event_data_store_retention_period = event_data_store.retention_period
event_data_store_kms_key_id = event_data_store.kms_key_id
event_data_store_federation_status = event_data_store.federation_status
```

---


### Trail_status

TrailStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `latest_digest_delivery_time` | String | <p>Specifies the date and time that CloudTrail last delivered a digest file to an
         account's Amazon S3 bucket.</p> |
| `latest_notification_attempt_time` | String | <p>This field is no longer in use.</p> |
| `time_logging_started` | String | <p>This field is no longer in use.</p> |
| `latest_delivery_attempt_time` | String | <p>This field is no longer in use.</p> |
| `latest_delivery_time` | String | <p>Specifies the date and time that CloudTrail last delivered log files to an
         account's Amazon S3 bucket.</p> |
| `time_logging_stopped` | String | <p>This field is no longer in use.</p> |
| `latest_delivery_attempt_succeeded` | String | <p>This field is no longer in use.</p> |
| `latest_delivery_error` | String | <p>Displays any Amazon S3 error that CloudTrail encountered when attempting
         to deliver log files to the designated bucket. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/ErrorResponses.html">Error
            Responses</a> in the Amazon S3 API Reference. </p>
         <note>
            <p>This error occurs only when there is a problem with the destination S3 bucket, and
            does not occur for requests that time out. To resolve the issue, 
            fix the <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/create-s3-bucket-policy-for-cloudtrail.html">bucket policy</a> so that CloudTrail 
            can write to the bucket; or create a new bucket and call <code>UpdateTrail</code> to specify the new bucket.</p>
         </note> |
| `latest_notification_error` | String | <p>Displays any Amazon SNS error that CloudTrail encountered when attempting
         to send a notification. For more information about Amazon SNS errors, see the
            <a href="https://docs.aws.amazon.com/sns/latest/dg/welcome.html">Amazon SNS
            Developer Guide</a>. </p> |
| `start_logging_time` | String | <p>Specifies the most recent date and time when CloudTrail started recording API
         calls for an Amazon Web Services account.</p> |
| `stop_logging_time` | String | <p>Specifies the most recent date and time when CloudTrail stopped recording API
         calls for an Amazon Web Services account.</p> |
| `latest_digest_delivery_error` | String | <p>Displays any Amazon S3 error that CloudTrail encountered when attempting
         to deliver a digest file to the designated bucket. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/ErrorResponses.html">Error
            Responses</a> in the Amazon S3 API Reference. </p>
         <note>
            <p>This error occurs only when there is a problem with the destination S3 bucket, and
            does not occur for requests that time out. To resolve the issue, 
            fix the <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/create-s3-bucket-policy-for-cloudtrail.html">bucket policy</a> so that CloudTrail 
            can write to the bucket; or create a new bucket and call <code>UpdateTrail</code> to specify the new bucket.</p>
         </note> |
| `latest_cloud_watch_logs_delivery_time` | String | <p>Displays the most recent date and time when CloudTrail delivered logs to CloudWatch Logs.</p> |
| `latest_notification_attempt_succeeded` | String | <p>This field is no longer in use.</p> |
| `latest_notification_time` | String | <p>Specifies the date and time of the most recent Amazon SNS notification that
            CloudTrail has written a new log file to an account's Amazon S3
         bucket.</p> |
| `is_logging` | bool | <p>Whether the CloudTrail trail is currently logging Amazon Web Services API
         calls.</p> |
| `latest_cloud_watch_logs_delivery_error` | String | <p>Displays any CloudWatch Logs error that CloudTrail encountered when attempting
         to deliver logs to CloudWatch Logs.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access trail_status outputs
trail_status_id = trail_status.id
trail_status_latest_digest_delivery_time = trail_status.latest_digest_delivery_time
trail_status_latest_notification_attempt_time = trail_status.latest_notification_attempt_time
trail_status_time_logging_started = trail_status.time_logging_started
trail_status_latest_delivery_attempt_time = trail_status.latest_delivery_attempt_time
trail_status_latest_delivery_time = trail_status.latest_delivery_time
trail_status_time_logging_stopped = trail_status.time_logging_stopped
trail_status_latest_delivery_attempt_succeeded = trail_status.latest_delivery_attempt_succeeded
trail_status_latest_delivery_error = trail_status.latest_delivery_error
trail_status_latest_notification_error = trail_status.latest_notification_error
trail_status_start_logging_time = trail_status.start_logging_time
trail_status_stop_logging_time = trail_status.stop_logging_time
trail_status_latest_digest_delivery_error = trail_status.latest_digest_delivery_error
trail_status_latest_cloud_watch_logs_delivery_time = trail_status.latest_cloud_watch_logs_delivery_time
trail_status_latest_notification_attempt_succeeded = trail_status.latest_notification_attempt_succeeded
trail_status_latest_notification_time = trail_status.latest_notification_time
trail_status_is_logging = trail_status.is_logging
trail_status_latest_cloud_watch_logs_delivery_error = trail_status.latest_cloud_watch_logs_delivery_error
```

---


### Channel

Channel resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>The name of the channel.</p> |
| `source` | String | ✅ | <p>The name of the partner or external event source. You cannot change this name after you create the 
         channel. A maximum of one channel is allowed per source.</p>
         <p>
         A source can be either <code>Custom</code> for all valid non-Amazon Web Services
         events, or the name of a partner event source. For information about the source names for available partners, see <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/query-event-data-store-integration.html#cloudtrail-lake-partner-information">Additional information about integration partners</a> in the CloudTrail User Guide.
      </p> |
| `destinations` | Vec<String> | ✅ | <p>One or more event data stores to which events arriving through a channel will be logged.</p> |
| `tags` | Vec<String> |  |  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `source` | String | <p>The source for the CloudTrail channel.</p> |
| `channel_arn` | String | <p>The ARN of an channel returned by a <code>GetChannel</code> request.</p> |
| `ingestion_status` | String | <p>A table showing information about the most recent successful and failed attempts 
      to ingest events.</p> |
| `name` | String | <p> The name of the CloudTrail channel. For service-linked channels, the name is
            <code>aws-service-channel/service-name/custom-suffix</code> where
            <code>service-name</code> represents the name of the Amazon Web Services service that
         created the channel and <code>custom-suffix</code> represents the suffix generated by the
            Amazon Web Services service. </p> |
| `destinations` | Vec<String> | <p>The destinations for the channel. For channels created for integrations, 
         the destinations are the event data stores that log events arriving through the channel. 
         For service-linked channels, the destination is the Amazon Web Services service that created the service-linked channel to receive events.</p> |
| `source_config` | String | <p> Provides information about the advanced event selectors configured for the channel, and
         whether the channel applies to all Regions or a single Region. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create channel
channel = provider.cloudtrail.Channel {
    name = "value"  # <p>The name of the channel.</p>
    source = "value"  # <p>The name of the partner or external event source. You cannot change this name after you create the 
         channel. A maximum of one channel is allowed per source.</p>
         <p>
         A source can be either <code>Custom</code> for all valid non-Amazon Web Services
         events, or the name of a partner event source. For information about the source names for available partners, see <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/query-event-data-store-integration.html#cloudtrail-lake-partner-information">Additional information about integration partners</a> in the CloudTrail User Guide.
      </p>
    destinations = "value"  # <p>One or more event data stores to which events arriving through a channel will be logged.</p>
}

# Access channel outputs
channel_id = channel.id
channel_source = channel.source
channel_channel_arn = channel.channel_arn
channel_ingestion_status = channel.ingestion_status
channel_name = channel.name
channel_destinations = channel.destinations
channel_source_config = channel.source_config
```

---


### Trails

Trails resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `trail_list` | Vec<String> | <p>The list of trail objects. Trail objects with string values are only returned if values
         for the objects exist in a trail's configuration. For example, <code>SNSTopicName</code>
         and <code>SNSTopicARN</code> are only returned in results if a trail is configured to send
         SNS notifications. Similarly, <code>KMSKeyId</code> only appears in results if a trail's
         log files are encrypted with KMS
         customer managed keys.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access trails outputs
trails_id = trails.id
trails_trail_list = trails.trail_list
```

---


### Dashboard

Dashboard resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `refresh_schedule` | String |  | <p>
   The refresh schedule configuration for the dashboard.
</p>
         <p>To create the Highlights dashboard, you must set a refresh schedule and set the <code>Status</code> to <code>ENABLED</code>. The <code>Unit</code> for the refresh schedule must be <code>HOURS</code> 
   and the <code>Value</code> must be <code>6</code>.</p> |
| `tags_list` | Vec<String> |  |  |
| `termination_protection_enabled` | bool |  | <p>
   Specifies whether termination protection is enabled for the dashboard. If termination protection is enabled, you cannot delete the dashboard until termination protection is disabled.
</p> |
| `widgets` | Vec<String> |  | <p>
An array of widgets for a custom dashboard. A custom dashboard can have a maximum of ten widgets.
</p>
         <p>You do not need to specify widgets for the Highlights dashboard.</p> |
| `name` | String | ✅ | <p>
   The name of the dashboard. The name must be unique to your account.
</p>
         <p>To create the Highlights dashboard, the name must be <code>AWSCloudTrail-Highlights</code>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | String | <p>
The status of the dashboard.
</p> |
| `type` | String | <p>
The type of dashboard.
</p> |
| `refresh_schedule` | String | <p>
The refresh schedule for the dashboard, if configured.
</p> |
| `created_timestamp` | String | <p>
   The timestamp that shows when the dashboard was created.
</p> |
| `last_refresh_failure_reason` | String | <p>
Provides information about failures for the last scheduled refresh.
</p> |
| `updated_timestamp` | String | <p>
   The timestamp that shows when the dashboard was last updated.
</p> |
| `dashboard_arn` | String | <p>
   The ARN for the dashboard.
</p> |
| `last_refresh_id` | String | <p>
The ID of the last dashboard refresh.
</p> |
| `termination_protection_enabled` | bool | <p>
Indicates whether termination protection is enabled for the dashboard.
</p> |
| `widgets` | Vec<String> | <p>
An array of widgets for the dashboard.
</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create dashboard
dashboard = provider.cloudtrail.Dashboard {
    name = "value"  # <p>
   The name of the dashboard. The name must be unique to your account.
</p>
         <p>To create the Highlights dashboard, the name must be <code>AWSCloudTrail-Highlights</code>.</p>
}

# Access dashboard outputs
dashboard_id = dashboard.id
dashboard_status = dashboard.status
dashboard_type = dashboard.type
dashboard_refresh_schedule = dashboard.refresh_schedule
dashboard_created_timestamp = dashboard.created_timestamp
dashboard_last_refresh_failure_reason = dashboard.last_refresh_failure_reason
dashboard_updated_timestamp = dashboard.updated_timestamp
dashboard_dashboard_arn = dashboard.dashboard_arn
dashboard_last_refresh_id = dashboard.last_refresh_id
dashboard_termination_protection_enabled = dashboard.termination_protection_enabled
dashboard_widgets = dashboard.widgets
```

---


### Query_results

QueryResults resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>A token you can use to get the next page of query results.</p> |
| `error_message` | String | <p>The error message returned if a query failed.</p> |
| `query_result_rows` | Vec<Vec<HashMap<String, String>>> | <p>Contains the individual event results of the query.</p> |
| `query_statistics` | String | <p>Shows the count of query results.</p> |
| `query_status` | String | <p>The status of the query. Values include <code>QUEUED</code>, <code>RUNNING</code>,
            <code>FINISHED</code>, <code>FAILED</code>, <code>TIMED_OUT</code>, or
            <code>CANCELLED</code>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access query_results outputs
query_results_id = query_results.id
query_results_next_token = query_results.next_token
query_results_error_message = query_results.error_message
query_results_query_result_rows = query_results.query_result_rows
query_results_query_statistics = query_results.query_statistics
query_results_query_status = query_results.query_status
```

---


### Import

Import resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `created_timestamp` | String | <p> The timestamp of the import's creation. </p> |
| `start_event_time` | String | <p> Used with <code>EndEventTime</code> to bound a <code>StartImport</code> request, and
         limit imported trail events to only those events logged within a specified time period.
      </p> |
| `import_statistics` | String | <p> Provides statistics for the import. CloudTrail does not update import statistics
         in real-time. Returned values for parameters such as <code>EventsCompleted</code> may be
         lower than the actual value, because CloudTrail updates statistics incrementally
         over the course of the import. </p> |
| `updated_timestamp` | String | <p> The timestamp of when the import was updated. </p> |
| `end_event_time` | String | <p> Used with <code>StartEventTime</code> to bound a <code>StartImport</code> request, and
         limit imported trail events to only those events logged within a specified time period.
      </p> |
| `import_status` | String | <p> The status of the import. </p> |
| `destinations` | Vec<String> | <p> The ARN of the destination event data store. </p> |
| `import_id` | String | <p> The ID of the import. </p> |
| `import_source` | String | <p> The source S3 bucket. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access import outputs
import_id = import.id
import_created_timestamp = import.created_timestamp
import_start_event_time = import.start_event_time
import_import_statistics = import.import_statistics
import_updated_timestamp = import.updated_timestamp
import_end_event_time = import.end_event_time
import_import_status = import.import_status
import_destinations = import.destinations
import_import_id = import.import_id
import_import_source = import.import_source
```

---


### Insight_selectors

InsightSelectors resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `event_data_store` | String |  | <p>The ARN (or ID suffix of the ARN) of the source event data store for which you want to change or add Insights
         selectors. To enable Insights on an event data store, you must provide both the 
         <code>EventDataStore</code> and <code>InsightsDestination</code> parameters.</p>
         <p>You cannot use this parameter with the <code>TrailName</code> parameter.</p> |
| `trail_name` | String |  | <p>The name of the CloudTrail trail for which you want to change or add Insights
         selectors.</p>
         <p>You cannot use this parameter with the <code>EventDataStore</code> and <code>InsightsDestination</code> parameters.</p> |
| `insight_selectors` | Vec<String> | ✅ | <p>A JSON string that contains the Insights types you want to log on a trail or event data store.
            <code>ApiCallRateInsight</code> and <code>ApiErrorRateInsight</code> are valid Insight
         types.</p>
         <p>The <code>ApiCallRateInsight</code> Insights type analyzes write-only
         management API calls that are aggregated per minute against a baseline API call volume.</p>
         <p>The <code>ApiErrorRateInsight</code> Insights type analyzes management
         API calls that result in error codes. The error is shown if the API call is
         unsuccessful.</p> |
| `insights_destination` | String |  | <p>
         The ARN (or ID suffix of the ARN) of the destination event data store that logs Insights events. To enable Insights on an event data store, you must provide both the 
         <code>EventDataStore</code> and <code>InsightsDestination</code> parameters.
      </p>
         <p>You cannot use this parameter with the <code>TrailName</code> parameter.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `insights_destination` | String | <p>
         The ARN of the destination event data store that logs Insights events.
      </p> |
| `trail_arn` | String | <p>The Amazon Resource Name (ARN) of a trail for which you want to get Insights
         selectors.</p> |
| `insight_selectors` | Vec<String> | <p>A JSON string that contains the Insight types you want to log on a trail or event data store. <code>ApiErrorRateInsight</code> and <code>ApiCallRateInsight</code> are supported
         as Insights types.</p> |
| `event_data_store_arn` | String | <p>
         The ARN of the source event data store that enabled Insights events.
      </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create insight_selectors
insight_selectors = provider.cloudtrail.Insight_selectors {
    insight_selectors = "value"  # <p>A JSON string that contains the Insights types you want to log on a trail or event data store.
            <code>ApiCallRateInsight</code> and <code>ApiErrorRateInsight</code> are valid Insight
         types.</p>
         <p>The <code>ApiCallRateInsight</code> Insights type analyzes write-only
         management API calls that are aggregated per minute against a baseline API call volume.</p>
         <p>The <code>ApiErrorRateInsight</code> Insights type analyzes management
         API calls that result in error codes. The error is shown if the API call is
         unsuccessful.</p>
}

# Access insight_selectors outputs
insight_selectors_id = insight_selectors.id
insight_selectors_insights_destination = insight_selectors.insights_destination
insight_selectors_trail_arn = insight_selectors.trail_arn
insight_selectors_insight_selectors = insight_selectors.insight_selectors
insight_selectors_event_data_store_arn = insight_selectors.event_data_store_arn
```

---


### Trail

Trail resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `s3_key_prefix` | String |  | <p>Specifies the Amazon S3 key prefix that comes after the name of the bucket you
         have designated for log file delivery. For more information, see <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/get-and-view-cloudtrail-log-files.html#cloudtrail-find-log-files">Finding Your CloudTrail Log Files</a>. The maximum length is 200
         characters.</p> |
| `is_multi_region_trail` | bool |  | <p>Specifies whether the trail is created in the current Region or in all Regions. The
         default is false, which creates a trail only in the Region where you are signed in. As a
         best practice, consider creating trails that log events in all Regions.</p> |
| `sns_topic_name` | String |  | <p>Specifies the name or ARN of the Amazon SNS topic defined for notification of log file
         delivery. The maximum length is 256 characters.</p> |
| `name` | String | ✅ | <p>Specifies the name of the trail. The name must meet the following requirements:</p>
         <ul>
            <li>
               <p>Contain only ASCII letters (a-z, A-Z), numbers (0-9), periods (.), underscores
               (_), or dashes (-)</p>
            </li>
            <li>
               <p>Start with a letter or number, and end with a letter or number</p>
            </li>
            <li>
               <p>Be between 3 and 128 characters</p>
            </li>
            <li>
               <p>Have no adjacent periods, underscores or dashes. Names like
                  <code>my-_namespace</code> and <code>my--namespace</code> are not valid.</p>
            </li>
            <li>
               <p>Not be in IP address format (for example, 192.168.5.4)</p>
            </li>
         </ul> |
| `enable_log_file_validation` | bool |  | <p>Specifies whether log file integrity validation is enabled. The default is false.</p>
         <note>
            <p>When you disable log file integrity validation, the chain of digest files is broken
            after one hour. CloudTrail does not create digest files for log files that were
            delivered during a period in which log file integrity validation was disabled. For
            example, if you enable log file integrity validation at noon on January 1, disable it at
            noon on January 2, and re-enable it at noon on January 10, digest files will not be
            created for the log files delivered from noon on January 2 to noon on January 10. The
            same applies whenever you stop CloudTrail logging or delete a trail.</p>
         </note> |
| `cloud_watch_logs_log_group_arn` | String |  | <p>Specifies a log group name using an Amazon Resource Name (ARN), a unique identifier that
         represents the log group to which CloudTrail logs will be delivered. You must use a
         log group that exists in your account.</p>
         <p>Not required unless you specify <code>CloudWatchLogsRoleArn</code>.</p> |
| `cloud_watch_logs_role_arn` | String |  | <p>Specifies the role for the CloudWatch Logs endpoint to assume to write to a user's
         log group. You must use a role that exists in your account.</p> |
| `s3_bucket_name` | String | ✅ | <p>Specifies the name of the Amazon S3 bucket designated for publishing log files. 
         For information about bucket naming rules, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/bucketnamingrules.html">Bucket naming rules</a> 
         in the <i>Amazon Simple Storage Service User Guide</i>.
        </p> |
| `tags_list` | Vec<String> |  |  |
| `kms_key_id` | String |  | <p>Specifies the KMS key ID to use to encrypt the logs delivered by CloudTrail. The value can be an alias name prefixed by <code>alias/</code>, a fully
         specified ARN to an alias, a fully specified ARN to a key, or a globally unique
         identifier.</p>
         <p>CloudTrail also supports KMS multi-Region keys. For more
         information about multi-Region keys, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/multi-region-keys-overview.html">Using multi-Region
            keys</a> in the <i>Key Management Service Developer Guide</i>.</p>
         <p>Examples:</p>
         <ul>
            <li>
               <p>
                  <code>alias/MyAliasName</code>
               </p>
            </li>
            <li>
               <p>
                  <code>arn:aws:kms:us-east-2:123456789012:alias/MyAliasName</code>
               </p>
            </li>
            <li>
               <p>
                  <code>arn:aws:kms:us-east-2:123456789012:key/12345678-1234-1234-1234-123456789012</code>
               </p>
            </li>
            <li>
               <p>
                  <code>12345678-1234-1234-1234-123456789012</code>
               </p>
            </li>
         </ul> |
| `is_organization_trail` | bool |  | <p>Specifies whether the trail is created for all accounts in an organization in Organizations, or only for the current Amazon Web Services account. The default is false,
         and cannot be true unless the call is made on behalf of an Amazon Web Services account that
         is the management account or delegated administrator account for an organization in Organizations.</p> |
| `include_global_service_events` | bool |  | <p>Specifies whether the trail is publishing events from global services such as IAM to the
         log files.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `trail` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create trail
trail = provider.cloudtrail.Trail {
    name = "value"  # <p>Specifies the name of the trail. The name must meet the following requirements:</p>
         <ul>
            <li>
               <p>Contain only ASCII letters (a-z, A-Z), numbers (0-9), periods (.), underscores
               (_), or dashes (-)</p>
            </li>
            <li>
               <p>Start with a letter or number, and end with a letter or number</p>
            </li>
            <li>
               <p>Be between 3 and 128 characters</p>
            </li>
            <li>
               <p>Have no adjacent periods, underscores or dashes. Names like
                  <code>my-_namespace</code> and <code>my--namespace</code> are not valid.</p>
            </li>
            <li>
               <p>Not be in IP address format (for example, 192.168.5.4)</p>
            </li>
         </ul>
    s3_bucket_name = "value"  # <p>Specifies the name of the Amazon S3 bucket designated for publishing log files. 
         For information about bucket naming rules, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/bucketnamingrules.html">Bucket naming rules</a> 
         in the <i>Amazon Simple Storage Service User Guide</i>.
        </p>
}

# Access trail outputs
trail_id = trail.id
trail_trail = trail.trail
```

---


### Resource_policy

ResourcePolicy resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resource_arn` | String | ✅ | <p>
         The Amazon Resource Name (ARN) of the CloudTrail event data store, dashboard, or channel attached to the resource-based policy.</p>
         <p>Example event data store ARN format:
         <code>arn:aws:cloudtrail:us-east-2:123456789012:eventdatastore/EXAMPLE-f852-4e8f-8bd1-bcf6cEXAMPLE</code>
         </p>
         <p>Example dashboard ARN format: <code>arn:aws:cloudtrail:us-east-1:123456789012:dashboard/exampleDash</code>
         </p>
         <p>Example channel ARN format:
         <code>arn:aws:cloudtrail:us-east-2:123456789012:channel/01234567890</code>
         </p> |
| `resource_policy` | String | ✅ | <p>
         A JSON-formatted string for an Amazon Web Services resource-based policy.
      </p>
         <p> For example resource-based policies, see 
         <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/security_iam_resource-based-policy-examples.html">CloudTrail resource-based policy examples</a> 
         in the <i>CloudTrail User Guide</i>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `resource_arn` | String | <p>
         The Amazon Resource Name (ARN) of the CloudTrail event data store, dashboard, or channel attached to resource-based policy. 
      </p>
         <p>Example event data store ARN format:
         <code>arn:aws:cloudtrail:us-east-2:123456789012:eventdatastore/EXAMPLE-f852-4e8f-8bd1-bcf6cEXAMPLE</code>
         </p>
         <p>Example dashboard ARN format: <code>arn:aws:cloudtrail:us-east-1:123456789012:dashboard/exampleDash</code>
         </p>
         <p>Example channel ARN format:
         <code>arn:aws:cloudtrail:us-east-2:123456789012:channel/01234567890</code>
         </p> |
| `resource_policy` | String | <p>
         A JSON-formatted string that contains the resource-based policy attached to the CloudTrail event data store, dashboard, or channel.
      </p> |
| `delegated_admin_resource_policy` | String | <p>
         The default resource-based policy that is automatically generated for the delegated administrator of an Organizations organization. 
         This policy will be evaluated in tandem with any policy you submit for the resource. For more information about this policy, 
         see <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/cloudtrail-lake-organizations.html#cloudtrail-lake-organizations-eds-rbp">Default resource policy for delegated administrators</a>.
      </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create resource_policy
resource_policy = provider.cloudtrail.Resource_policy {
    resource_arn = "value"  # <p>
         The Amazon Resource Name (ARN) of the CloudTrail event data store, dashboard, or channel attached to the resource-based policy.</p>
         <p>Example event data store ARN format:
         <code>arn:aws:cloudtrail:us-east-2:123456789012:eventdatastore/EXAMPLE-f852-4e8f-8bd1-bcf6cEXAMPLE</code>
         </p>
         <p>Example dashboard ARN format: <code>arn:aws:cloudtrail:us-east-1:123456789012:dashboard/exampleDash</code>
         </p>
         <p>Example channel ARN format:
         <code>arn:aws:cloudtrail:us-east-2:123456789012:channel/01234567890</code>
         </p>
    resource_policy = "value"  # <p>
         A JSON-formatted string for an Amazon Web Services resource-based policy.
      </p>
         <p> For example resource-based policies, see 
         <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/security_iam_resource-based-policy-examples.html">CloudTrail resource-based policy examples</a> 
         in the <i>CloudTrail User Guide</i>.</p>
}

# Access resource_policy outputs
resource_policy_id = resource_policy.id
resource_policy_resource_arn = resource_policy.resource_arn
resource_policy_resource_policy = resource_policy.resource_policy
resource_policy_delegated_admin_resource_policy = resource_policy.delegated_admin_resource_policy
```

---


### Event_configuration

EventConfiguration resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `context_key_selectors` | Vec<String> | ✅ | <p>A list of context key selectors that will be included to provide enriched event data.</p> |
| `event_data_store` | String |  | <p>The Amazon Resource Name (ARN) or ID suffix of the ARN of the event data store for which you want to update event configuration settings.</p> |
| `max_event_size` | String | ✅ | <p>The maximum allowed size for events to be stored in the specified event data store. If you are using context key selectors, MaxEventSize must be set to Large.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `context_key_selectors` | Vec<String> | <p>The list of context key selectors that are configured for the event data store.</p> |
| `event_data_store_arn` | String | <p>The Amazon Resource Name (ARN) or ID suffix of the ARN of the event data store for which the event configuration settings are returned.</p> |
| `max_event_size` | String | <p>The maximum allowed size for events stored in the specified event data store.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create event_configuration
event_configuration = provider.cloudtrail.Event_configuration {
    context_key_selectors = "value"  # <p>A list of context key selectors that will be included to provide enriched event data.</p>
    max_event_size = "value"  # <p>The maximum allowed size for events to be stored in the specified event data store. If you are using context key selectors, MaxEventSize must be set to Large.</p>
}

# Access event_configuration outputs
event_configuration_id = event_configuration.id
event_configuration_context_key_selectors = event_configuration.context_key_selectors
event_configuration_event_data_store_arn = event_configuration.event_data_store_arn
event_configuration_max_event_size = event_configuration.max_event_size
```

---


### Event_selectors

EventSelectors resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `trail_name` | String | ✅ | <p>Specifies the name of the trail or trail ARN. If you specify a trail name, the string
         must meet the following requirements:</p>
         <ul>
            <li>
               <p>Contain only ASCII letters (a-z, A-Z), numbers (0-9), periods (.), underscores
               (_), or dashes (-)</p>
            </li>
            <li>
               <p>Start with a letter or number, and end with a letter or number</p>
            </li>
            <li>
               <p>Be between 3 and 128 characters</p>
            </li>
            <li>
               <p>Have no adjacent periods, underscores or dashes. Names like
                  <code>my-_namespace</code> and <code>my--namespace</code> are not valid.</p>
            </li>
            <li>
               <p>Not be in IP address format (for example, 192.168.5.4)</p>
            </li>
         </ul>
         <p>If you specify a trail ARN, it must be in the following format.</p>
         <p>
            <code>arn:aws:cloudtrail:us-east-2:123456789012:trail/MyTrail</code>
         </p> |
| `event_selectors` | Vec<String> |  | <p>Specifies the settings for your event selectors. You can use event selectors to log management events and data events for the following resource types:</p>
         <ul>
            <li>
               <p>
                  <code>AWS::DynamoDB::Table</code>
               </p>
            </li>
            <li>
               <p>
                  <code>AWS::Lambda::Function</code>
               </p>
            </li>
            <li>
               <p>
                  <code>AWS::S3::Object</code>
               </p>
            </li>
         </ul>
         <p>You can't use event selectors to log network activity events.</p>
         <p>You can configure up to five event
         selectors for a trail. You can use either <code>EventSelectors</code> or
            <code>AdvancedEventSelectors</code> in a <code>PutEventSelectors</code> request, but not
         both. If you apply <code>EventSelectors</code> to a trail, any existing
            <code>AdvancedEventSelectors</code> are overwritten.</p> |
| `advanced_event_selectors` | Vec<String> |  | <p> Specifies the settings for advanced event selectors. You can use advanced event selectors to 
         log management events, data events for all resource types, and network activity events.</p>
         <p>You can add advanced event
         selectors, and conditions for your advanced event selectors, up to a maximum of 500 values
         for all conditions and selectors on a trail. You can use either
            <code>AdvancedEventSelectors</code> or <code>EventSelectors</code>, but not both. If you
         apply <code>AdvancedEventSelectors</code> to a trail, any existing
            <code>EventSelectors</code> are overwritten. For more information about advanced event
         selectors, see <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/logging-data-events-with-cloudtrail.html">Logging data events</a> and 
         <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/logging-network-events-with-cloudtrail.html">Logging network activity events</a>
         in the <i>CloudTrail User Guide</i>. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `event_selectors` | Vec<String> | <p>The event selectors that are configured for the trail.</p> |
| `trail_arn` | String | <p>The specified trail ARN that has the event selectors.</p> |
| `advanced_event_selectors` | Vec<String> | <p> The advanced event selectors that are configured for the trail. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create event_selectors
event_selectors = provider.cloudtrail.Event_selectors {
    trail_name = "value"  # <p>Specifies the name of the trail or trail ARN. If you specify a trail name, the string
         must meet the following requirements:</p>
         <ul>
            <li>
               <p>Contain only ASCII letters (a-z, A-Z), numbers (0-9), periods (.), underscores
               (_), or dashes (-)</p>
            </li>
            <li>
               <p>Start with a letter or number, and end with a letter or number</p>
            </li>
            <li>
               <p>Be between 3 and 128 characters</p>
            </li>
            <li>
               <p>Have no adjacent periods, underscores or dashes. Names like
                  <code>my-_namespace</code> and <code>my--namespace</code> are not valid.</p>
            </li>
            <li>
               <p>Not be in IP address format (for example, 192.168.5.4)</p>
            </li>
         </ul>
         <p>If you specify a trail ARN, it must be in the following format.</p>
         <p>
            <code>arn:aws:cloudtrail:us-east-2:123456789012:trail/MyTrail</code>
         </p>
}

# Access event_selectors outputs
event_selectors_id = event_selectors.id
event_selectors_event_selectors = event_selectors.event_selectors
event_selectors_trail_arn = event_selectors.trail_arn
event_selectors_advanced_event_selectors = event_selectors.advanced_event_selectors
```

---


### Query

Query resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `delivery_status` | String | <p>The delivery status.</p> |
| `query_id` | String | <p>The ID of the query.</p> |
| `query_statistics` | String | <p>Metadata about a query, including the number of events that were matched, the total
         number of events scanned, the query run time in milliseconds, and the query's creation
         time.</p> |
| `error_message` | String | <p>The error message returned if a query failed.</p> |
| `prompt` | String | <p>
         The prompt used for a generated query. For information about generated queries, see 
         <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/lake-query-generator.html">Create CloudTrail Lake queries from natural language prompts</a> 
         in the <i>CloudTrail </i> user guide.
      </p> |
| `query_string` | String | <p>The SQL code of a query.</p> |
| `delivery_s3_uri` | String | <p>The URI for the S3 bucket where CloudTrail delivered query results, if
         applicable.</p> |
| `event_data_store_owner_account_id` | String | <p>
   The account ID of the event data store owner.
</p> |
| `query_status` | String | <p>The status of a query. Values for <code>QueryStatus</code> include <code>QUEUED</code>,
            <code>RUNNING</code>, <code>FINISHED</code>, <code>FAILED</code>,
         <code>TIMED_OUT</code>, or <code>CANCELLED</code>
         </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access query outputs
query_id = query.id
query_delivery_status = query.delivery_status
query_query_id = query.query_id
query_query_statistics = query.query_statistics
query_error_message = query.error_message
query_prompt = query.prompt
query_query_string = query.query_string
query_delivery_s3_uri = query.delivery_s3_uri
query_event_data_store_owner_account_id = query.event_data_store_owner_account_id
query_query_status = query.query_status
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple event_data_store resources
event_data_store_0 = provider.cloudtrail.Event_data_store {
    name = "value-0"
}
event_data_store_1 = provider.cloudtrail.Event_data_store {
    name = "value-1"
}
event_data_store_2 = provider.cloudtrail.Event_data_store {
    name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    event_data_store = provider.cloudtrail.Event_data_store {
        name = "production-value"
    }
```

---

## Related Documentation

- [AWS Cloudtrail Documentation](https://docs.aws.amazon.com/cloudtrail/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
