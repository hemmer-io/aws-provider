# Cloudwatch_logs Service



**Resources**: 43

---

## Overview

The cloudwatch_logs service provides access to 43 resource types:

- [Delivery_sources](#delivery_sources) [R]
- [Query_definitions](#query_definitions) [R]
- [Log_object](#log_object) [R]
- [Log_group](#log_group) [CD]
- [Configuration_templates](#configuration_templates) [R]
- [Export_task](#export_task) [C]
- [Subscription_filter](#subscription_filter) [CD]
- [Account_policies](#account_policies) [R]
- [Log_anomaly_detector](#log_anomaly_detector) [CRUD]
- [Destinations](#destinations) [R]
- [Account_policy](#account_policy) [CD]
- [Transformer](#transformer) [CRD]
- [Log_streams](#log_streams) [R]
- [Data_protection_policy](#data_protection_policy) [CRD]
- [Destination_policy](#destination_policy) [C]
- [Log_groups](#log_groups) [R]
- [Delivery_configuration](#delivery_configuration) [U]
- [Delivery_destination](#delivery_destination) [CRD]
- [Subscription_filters](#subscription_filters) [R]
- [Delivery_destinations](#delivery_destinations) [R]
- [Index_policies](#index_policies) [R]
- [Delivery](#delivery) [CRD]
- [Integration](#integration) [CRD]
- [Log_record](#log_record) [R]
- [Metric_filter](#metric_filter) [CD]
- [Index_policy](#index_policy) [CD]
- [Delivery_source](#delivery_source) [CRD]
- [Retention_policy](#retention_policy) [CD]
- [Metric_filters](#metric_filters) [R]
- [Resource_policy](#resource_policy) [CD]
- [Log_group_fields](#log_group_fields) [R]
- [Deliveries](#deliveries) [R]
- [Log_stream](#log_stream) [CD]
- [Destination](#destination) [CD]
- [Query_definition](#query_definition) [CD]
- [Queries](#queries) [R]
- [Log_events](#log_events) [CR]
- [Query_results](#query_results) [R]
- [Anomaly](#anomaly) [U]
- [Delivery_destination_policy](#delivery_destination_policy) [CRD]
- [Export_tasks](#export_tasks) [R]
- [Resource_policies](#resource_policies) [R]
- [Field_indexes](#field_indexes) [R]

---

## Resources


### Delivery_sources

DeliverySources resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String |  |
| `delivery_sources` | Vec<String> | <p>An array of structures. Each structure contains information about one delivery source in
      the account.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access delivery_sources outputs
delivery_sources_id = delivery_sources.id
delivery_sources_next_token = delivery_sources.next_token
delivery_sources_delivery_sources = delivery_sources.delivery_sources
```

---


### Query_definitions

QueryDefinitions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `query_definitions` | Vec<String> | <p>The list of query definitions that match your request.</p> |
| `next_token` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access query_definitions outputs
query_definitions_id = query_definitions.id
query_definitions_query_definitions = query_definitions.query_definitions
query_definitions_next_token = query_definitions.next_token
```

---


### Log_object

LogObject resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `field_stream` | String | <p>A stream of structured log data returned by the GetLogObject operation. This stream
      contains log events with their associated metadata and extracted fields.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access log_object outputs
log_object_id = log_object.id
log_object_field_stream = log_object.field_stream
```

---


### Log_group

LogGroup resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `log_group_name` | String | ✅ | <p>A name for the log group.</p> |
| `kms_key_id` | String |  | <p>The Amazon Resource Name (ARN) of the KMS key to use when encrypting log
      data. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-kms">Amazon Resource
        Names</a>.</p> |
| `tags` | HashMap<String, String> |  | <p>The key-value pairs to use for the tags.</p>
         <p>You can grant users access to certain log groups while preventing them from accessing
      other log groups. To do so, tag your groups and use IAM policies that refer to
      those tags. To assign tags when you create a log group, you must have either the
        <code>logs:TagResource</code> or <code>logs:TagLogGroup</code> permission. For more
      information about tagging, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services resources</a>. For
      more information about using tags to control access, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_tags.html">Controlling access to Amazon Web Services
        resources using tags</a>.</p> |
| `log_group_class` | String |  | <p>Use this parameter to specify the log group class for this log group. There are three
      classes:</p>
         <ul>
            <li>
               <p>The <code>Standard</code> log class supports all CloudWatch Logs features.</p>
            </li>
            <li>
               <p>The <code>Infrequent Access</code> log class supports a subset of CloudWatch Logs
          features and incurs lower costs.</p>
            </li>
            <li>
               <p>Use the <code>Delivery</code> log class only for delivering Lambda
          logs to store in Amazon S3 or Amazon Data Firehose. Log events in log groups in
          the Delivery class are kept in CloudWatch Logs for only one day. This log class doesn't
          offer rich CloudWatch Logs capabilities such as CloudWatch Logs Insights
          queries.</p>
            </li>
         </ul>
         <p>If you omit this parameter, the default of <code>STANDARD</code> is used.</p>
         <important>
            <p>The value of <code>logGroupClass</code> can't be changed after a log group is
        created.</p>
         </important>
         <p>For details about the features supported by each class, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/CloudWatch_Logs_Log_Classes.html">Log classes</a>
         </p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create log_group
log_group = provider.cloudwatch_logs.Log_group {
    log_group_name = "value"  # <p>A name for the log group.</p>
}

```

---


### Configuration_templates

ConfigurationTemplates resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `configuration_templates` | Vec<String> | <p>An array of objects, where each object describes one configuration template that matches
      the filters that you specified in the request.</p> |
| `next_token` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access configuration_templates outputs
configuration_templates_id = configuration_templates.id
configuration_templates_configuration_templates = configuration_templates.configuration_templates
configuration_templates_next_token = configuration_templates.next_token
```

---


### Export_task

ExportTask resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `destination_prefix` | String |  | <p>The prefix used as the start of the key for every object exported. If you don't specify
      a value, the default is <code>exportedlogs</code>.</p>
         <p>The length of this parameter must comply with the S3 object key name length limits. The
      object key name is a sequence of Unicode characters with UTF-8 encoding, and can be up to
      1,024 bytes.</p> |
| `destination` | String | ✅ | <p>The name of S3 bucket for the exported log data. The bucket must be in the same Amazon Web Services Region.</p> |
| `from` | String | ✅ | <p>The start time of the range for the request, expressed as the number of milliseconds
      after <code>Jan 1, 1970 00:00:00 UTC</code>. Events with a timestamp earlier than this time
      are not exported.</p> |
| `task_name` | String |  | <p>The name of the export task.</p> |
| `log_group_name` | String | ✅ | <p>The name of the log group.</p> |
| `log_stream_name_prefix` | String |  | <p>Export only log streams that match the provided prefix. If you don't specify a value,
      no prefix filter is applied.</p> |
| `to` | String | ✅ | <p>The end time of the range for the request, expressed as the number of milliseconds
      after <code>Jan 1, 1970 00:00:00 UTC</code>. Events with a timestamp later than this time are
      not exported.</p>
         <p>You must specify a time that is not earlier than when this log group was created.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create export_task
export_task = provider.cloudwatch_logs.Export_task {
    destination = "value"  # <p>The name of S3 bucket for the exported log data. The bucket must be in the same Amazon Web Services Region.</p>
    from = "value"  # <p>The start time of the range for the request, expressed as the number of milliseconds
      after <code>Jan 1, 1970 00:00:00 UTC</code>. Events with a timestamp earlier than this time
      are not exported.</p>
    log_group_name = "value"  # <p>The name of the log group.</p>
    to = "value"  # <p>The end time of the range for the request, expressed as the number of milliseconds
      after <code>Jan 1, 1970 00:00:00 UTC</code>. Events with a timestamp later than this time are
      not exported.</p>
         <p>You must specify a time that is not earlier than when this log group was created.</p>
}

```

---


### Subscription_filter

SubscriptionFilter resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `role_arn` | String |  | <p>The ARN of an IAM role that grants CloudWatch Logs permissions to deliver ingested log
      events to the destination stream. You don't need to provide the ARN when you are working with
      a logical destination for cross-account delivery.</p> |
| `apply_on_transformed_logs` | bool |  | <p>This parameter is valid only for log groups that have an active log transformer. For more
      information about log transformers, see <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_PutTransformer.html">PutTransformer</a>.</p>
         <p>If the log group uses either a log-group level or account-level transformer, and you
      specify <code>true</code>, the subscription filter will be applied on the transformed version
      of the log events instead of the original ingested log events.</p> |
| `filter_name` | String | ✅ | <p>A name for the subscription filter. If you are updating an existing filter, you must
      specify the correct name in <code>filterName</code>. To find the name of the filter currently
      associated with a log group, use <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_DescribeSubscriptionFilters.html">DescribeSubscriptionFilters</a>.</p> |
| `emit_system_fields` | Vec<String> |  | <p>A list of system fields to include in the log events sent to the subscription destination. Valid values are <code>@aws.account</code> and <code>@aws.region</code>. These fields provide source information for centralized log data in the forwarded payload.</p> |
| `destination_arn` | String | ✅ | <p>The ARN of the destination to deliver matching log events to. Currently, the supported
      destinations are:</p>
         <ul>
            <li>
               <p>An Amazon Kinesis stream belonging to the same account as the subscription filter,
          for same-account delivery.</p>
            </li>
            <li>
               <p>A logical destination (specified using an ARN) belonging to a different account,
          for cross-account delivery.</p>
               <p>If you're setting up a cross-account subscription, the destination must have an IAM
          policy associated with it. The IAM policy must allow the sender to send logs to the
          destination. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_PutDestinationPolicy.html">PutDestinationPolicy</a>.</p>
            </li>
            <li>
               <p>A Kinesis Data Firehose delivery stream belonging to the same account as the
          subscription filter, for same-account delivery.</p>
            </li>
            <li>
               <p>A Lambda function belonging to the same account as the subscription
          filter, for same-account delivery.</p>
            </li>
         </ul> |
| `field_selection_criteria` | String |  | <p>A filter expression that specifies which log events should be processed by this subscription filter based on system fields such as source account and source region. Uses selection criteria syntax with operators like <code>=</code>, <code>!=</code>, <code>AND</code>, <code>OR</code>, <code>IN</code>, <code>NOT IN</code>. Example: <code>@aws.region NOT IN ["cn-north-1"]</code> or <code>@aws.account = "123456789012" AND @aws.region = "us-east-1"</code>. Maximum length: 2000 characters.</p> |
| `distribution` | String |  | <p>The method used to distribute log data to the destination. By default, log data is
      grouped by log stream, but the grouping can be set to random for a more even distribution.
      This property is only applicable when the destination is an Amazon Kinesis data stream.
    </p> |
| `log_group_name` | String | ✅ | <p>The name of the log group.</p> |
| `filter_pattern` | String | ✅ | <p>A filter pattern for subscribing to a filtered stream of log events.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create subscription_filter
subscription_filter = provider.cloudwatch_logs.Subscription_filter {
    filter_name = "value"  # <p>A name for the subscription filter. If you are updating an existing filter, you must
      specify the correct name in <code>filterName</code>. To find the name of the filter currently
      associated with a log group, use <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_DescribeSubscriptionFilters.html">DescribeSubscriptionFilters</a>.</p>
    destination_arn = "value"  # <p>The ARN of the destination to deliver matching log events to. Currently, the supported
      destinations are:</p>
         <ul>
            <li>
               <p>An Amazon Kinesis stream belonging to the same account as the subscription filter,
          for same-account delivery.</p>
            </li>
            <li>
               <p>A logical destination (specified using an ARN) belonging to a different account,
          for cross-account delivery.</p>
               <p>If you're setting up a cross-account subscription, the destination must have an IAM
          policy associated with it. The IAM policy must allow the sender to send logs to the
          destination. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_PutDestinationPolicy.html">PutDestinationPolicy</a>.</p>
            </li>
            <li>
               <p>A Kinesis Data Firehose delivery stream belonging to the same account as the
          subscription filter, for same-account delivery.</p>
            </li>
            <li>
               <p>A Lambda function belonging to the same account as the subscription
          filter, for same-account delivery.</p>
            </li>
         </ul>
    log_group_name = "value"  # <p>The name of the log group.</p>
    filter_pattern = "value"  # <p>A filter pattern for subscribing to a filtered stream of log events.</p>
}

```

---


### Account_policies

AccountPolicies resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `account_policies` | Vec<String> | <p>An array of structures that contain information about the CloudWatch Logs account
      policies that match the specified filters.</p> |
| `next_token` | String | <p>The token to use when requesting the next set of items. The token expires after 24
      hours.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access account_policies outputs
account_policies_id = account_policies.id
account_policies_account_policies = account_policies.account_policies
account_policies_next_token = account_policies.next_token
```

---


### Log_anomaly_detector

LogAnomalyDetector resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `kms_key_id` | String |  | <p>Optionally assigns a KMS key to secure this anomaly detector and its
      findings. If a key is assigned, the anomalies found and the model used by this detector are
      encrypted at rest with the key. If a key is assigned to an anomaly detector, a user must have
      permissions for both this key and for the anomaly detector to retrieve information about the
      anomalies that it finds.</p>
         <p> Make sure the value provided is a valid KMS key ARN. For more information
      about using a KMS key and to see the required IAM policy, see
        <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/LogsAnomalyDetection-KMS.html">Use a KMS key with an anomaly detector</a>.</p> |
| `evaluation_frequency` | String |  | <p>Specifies how often the anomaly detector is to run and look for anomalies. Set this value
      according to the frequency that the log group receives new logs. For example, if the log group
      receives new log events every 10 minutes, then 15 minutes might be a good setting for
        <code>evaluationFrequency</code> .</p> |
| `log_group_arn_list` | Vec<String> | ✅ | <p>An array containing the ARN of the log group that this anomaly detector will watch. You
      can specify only one log group ARN.</p> |
| `filter_pattern` | String |  | <p>You can use this parameter to limit the anomaly detection model to examine only log events
      that match the pattern you specify here. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/FilterAndPatternSyntax.html">Filter and Pattern
        Syntax</a>.</p> |
| `anomaly_visibility_time` | i64 |  | <p>The number of days to have visibility on an anomaly. After this time period has elapsed
      for an anomaly, it will be automatically baselined and the anomaly detector will treat new
      occurrences of a similar anomaly as normal. Therefore, if you do not correct the cause of an
      anomaly during the time period specified in <code>anomalyVisibilityTime</code>, it will be
      considered normal going forward and will not be detected as an anomaly.</p> |
| `detector_name` | String |  | <p>A name for this anomaly detector.</p> |
| `tags` | HashMap<String, String> |  | <p>An optional list of key-value pairs to associate with the resource.</p>
         <p>For more information about tagging, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services resources</a>
         </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `anomaly_visibility_time` | i64 | <p>The number of days used as the life cycle of anomalies. After this time, anomalies are
      automatically baselined and the anomaly detector model will treat new occurrences of similar
      event as normal. </p> |
| `creation_time_stamp` | i64 | <p>The date and time when this anomaly detector was created.</p> |
| `kms_key_id` | String | <p>The ARN of the KMS key assigned to this anomaly detector, if any.</p> |
| `anomaly_detector_status` | String | <p>Specifies whether the anomaly detector is currently active. To change its status, use the
        <code>enabled</code> parameter in the <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_UpdateLogAnomalyDetector.html">UpdateLogAnomalyDetector</a> operation.</p> |
| `filter_pattern` | String |  |
| `last_modified_time_stamp` | i64 | <p>The date and time when this anomaly detector was most recently modified.</p> |
| `log_group_arn_list` | Vec<String> | <p>An array of structures, where each structure contains the ARN of a log group associated
      with this anomaly detector.</p> |
| `detector_name` | String | <p>The name of the log anomaly detector</p> |
| `evaluation_frequency` | String | <p>Specifies how often the anomaly detector runs and look for anomalies. Set this value
      according to the frequency that the log group receives new logs. For example, if the log group
      receives new log events every 10 minutes, then setting <code>evaluationFrequency</code> to
        <code>FIFTEEN_MIN</code> might be appropriate.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create log_anomaly_detector
log_anomaly_detector = provider.cloudwatch_logs.Log_anomaly_detector {
    log_group_arn_list = "value"  # <p>An array containing the ARN of the log group that this anomaly detector will watch. You
      can specify only one log group ARN.</p>
}

# Access log_anomaly_detector outputs
log_anomaly_detector_id = log_anomaly_detector.id
log_anomaly_detector_anomaly_visibility_time = log_anomaly_detector.anomaly_visibility_time
log_anomaly_detector_creation_time_stamp = log_anomaly_detector.creation_time_stamp
log_anomaly_detector_kms_key_id = log_anomaly_detector.kms_key_id
log_anomaly_detector_anomaly_detector_status = log_anomaly_detector.anomaly_detector_status
log_anomaly_detector_filter_pattern = log_anomaly_detector.filter_pattern
log_anomaly_detector_last_modified_time_stamp = log_anomaly_detector.last_modified_time_stamp
log_anomaly_detector_log_group_arn_list = log_anomaly_detector.log_group_arn_list
log_anomaly_detector_detector_name = log_anomaly_detector.detector_name
log_anomaly_detector_evaluation_frequency = log_anomaly_detector.evaluation_frequency
```

---


### Destinations

Destinations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `destinations` | Vec<String> | <p>The destinations.</p> |
| `next_token` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access destinations outputs
destinations_id = destinations.id
destinations_destinations = destinations.destinations
destinations_next_token = destinations.next_token
```

---


### Account_policy

AccountPolicy resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `policy_document` | String | ✅ | <p>Specify the policy, in JSON.</p>
         <p>
            <b>Data protection policy</b>
         </p>
         <p>A data protection policy must include two JSON blocks:</p>
         <ul>
            <li>
               <p>The first block must include both a <code>DataIdentifer</code> array and an
            <code>Operation</code> property with an <code>Audit</code> action. The
            <code>DataIdentifer</code> array lists the types of sensitive data that you want to
          mask. For more information about the available options, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/mask-sensitive-log-data-types.html">Types of data that
            you can mask</a>.</p>
               <p>The <code>Operation</code> property with an <code>Audit</code> action is required to
          find the sensitive data terms. This <code>Audit</code> action must contain a
            <code>FindingsDestination</code> object. You can optionally use that
            <code>FindingsDestination</code> object to list one or more destinations to send audit
          findings to. If you specify destinations such as log groups, Firehose streams,
          and S3 buckets, they must already exist.</p>
            </li>
            <li>
               <p>The second block must include both a <code>DataIdentifer</code> array and an
            <code>Operation</code> property with an <code>Deidentify</code> action. The
            <code>DataIdentifer</code> array must exactly match the <code>DataIdentifer</code> array
          in the first block of the policy.</p>
               <p>The <code>Operation</code> property with the <code>Deidentify</code> action is what
          actually masks the data, and it must contain the <code> "MaskConfig": {}</code> object.
          The <code> "MaskConfig": {}</code> object must be empty.</p>
            </li>
         </ul>
         <p>For an example data protection policy, see the <b>Examples</b>
      section on this page.</p>
         <important>
            <p>The contents of the two <code>DataIdentifer</code> arrays must match exactly.</p>
         </important>
         <p>In addition to the two JSON blocks, the <code>policyDocument</code> can also include
        <code>Name</code>, <code>Description</code>, and <code>Version</code> fields. The
        <code>Name</code> is different than the operation's <code>policyName</code> parameter, and
      is used as a dimension when CloudWatch Logs reports audit findings metrics to CloudWatch.</p>
         <p>The JSON specified in <code>policyDocument</code> can be up to 30,720 characters
      long.</p>
         <p>
            <b>Subscription filter policy</b>
         </p>
         <p>A subscription filter policy can include the following attributes in a JSON block:</p>
         <ul>
            <li>
               <p>
                  <b>DestinationArn</b> The ARN of the destination to deliver
          log events to. Supported destinations are:</p>
               <ul>
                  <li>
                     <p>An Kinesis Data Streams data stream in the same account as the subscription policy,
              for same-account delivery.</p>
                  </li>
                  <li>
                     <p>An Firehose data stream in the same account as the subscription policy,
              for same-account delivery.</p>
                  </li>
                  <li>
                     <p>A Lambda function in the same account as the subscription policy, for
              same-account delivery.</p>
                  </li>
                  <li>
                     <p>A logical destination in a different account created with <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_PutDestination.html">PutDestination</a>, for cross-account delivery. Kinesis Data Streams and Firehose are supported as logical destinations.</p>
                  </li>
               </ul>
            </li>
            <li>
               <p>
                  <b>RoleArn</b> The ARN of an IAM role that grants CloudWatch
          Logs permissions to deliver ingested log events to the destination stream. You don't need
          to provide the ARN when you are working with a logical destination for cross-account
          delivery.</p>
            </li>
            <li>
               <p>
                  <b>FilterPattern</b> A filter pattern for subscribing to
          a filtered stream of log events.</p>
            </li>
            <li>
               <p>
                  <b>Distribution</b> The method used to distribute log data
          to the destination. By default, log data is grouped by log stream, but the grouping can be
          set to <code>Random</code> for a more even distribution. This property is only applicable
          when the destination is an Kinesis Data Streams data stream.</p>
            </li>
         </ul>
         <p>
            <b>Transformer policy</b>
         </p>
         <p>A transformer policy must include one JSON block with the array of processors and their
      configurations. For more information about available processors, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/CloudWatch-Logs-Transformation.html#CloudWatch-Logs-Transformation-Processors"> Processors that you can use</a>. </p>
         <p>
            <b>Field index policy</b>
         </p>
         <p>A field index filter policy can include the following attribute in a JSON block:</p>
         <ul>
            <li>
               <p>
                  <b>Fields</b> The array of field indexes to create.</p>
            </li>
         </ul>
         <p>It must contain at least one field index.</p>
         <p>The following is an example of an index policy document that creates two indexes,
        <code>RequestId</code> and <code>TransactionId</code>.</p>
         <p>
            <code>"policyDocument": "{ \"Fields\": [ \"RequestId\", \"TransactionId\" ]
      }"</code>
         </p> |
| `scope` | String |  | <p>Currently the only valid value for this parameter is <code>ALL</code>, which specifies
      that the data protection policy applies to all log groups in the account. If you omit this
      parameter, the default of <code>ALL</code> is used.</p> |
| `policy_name` | String | ✅ | <p>A name for the policy. This must be unique within the account.</p> |
| `policy_type` | String | ✅ | <p>The type of policy that you're creating or updating.</p> |
| `selection_criteria` | String |  | <p>Use this parameter to apply the new policy to a subset of log groups in the
      account.</p>
         <p>Specifying <code>selectionCriteria</code> is valid only when you specify
        <code>SUBSCRIPTION_FILTER_POLICY</code>, <code>FIELD_INDEX_POLICY</code> or
        <code>TRANSFORMER_POLICY</code>for <code>policyType</code>.</p>
         <p>If <code>policyType</code> is <code>SUBSCRIPTION_FILTER_POLICY</code>, the only supported
        <code>selectionCriteria</code> filter is <code>LogGroupName NOT IN []</code>
         </p>
         <p>If <code>policyType</code> is <code>FIELD_INDEX_POLICY</code> or
        <code>TRANSFORMER_POLICY</code>, the only supported <code>selectionCriteria</code> filter is
        <code>LogGroupNamePrefix</code>
         </p>
         <p>The <code>selectionCriteria</code> string can be up to 25KB in length. The length is
      determined by using its UTF-8 bytes.</p>
         <p>Using the <code>selectionCriteria</code> parameter with
        <code>SUBSCRIPTION_FILTER_POLICY</code> is useful to help prevent infinite loops. For more
      information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/Subscriptions-recursion-prevention.html">Log recursion
        prevention</a>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create account_policy
account_policy = provider.cloudwatch_logs.Account_policy {
    policy_document = "value"  # <p>Specify the policy, in JSON.</p>
         <p>
            <b>Data protection policy</b>
         </p>
         <p>A data protection policy must include two JSON blocks:</p>
         <ul>
            <li>
               <p>The first block must include both a <code>DataIdentifer</code> array and an
            <code>Operation</code> property with an <code>Audit</code> action. The
            <code>DataIdentifer</code> array lists the types of sensitive data that you want to
          mask. For more information about the available options, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/mask-sensitive-log-data-types.html">Types of data that
            you can mask</a>.</p>
               <p>The <code>Operation</code> property with an <code>Audit</code> action is required to
          find the sensitive data terms. This <code>Audit</code> action must contain a
            <code>FindingsDestination</code> object. You can optionally use that
            <code>FindingsDestination</code> object to list one or more destinations to send audit
          findings to. If you specify destinations such as log groups, Firehose streams,
          and S3 buckets, they must already exist.</p>
            </li>
            <li>
               <p>The second block must include both a <code>DataIdentifer</code> array and an
            <code>Operation</code> property with an <code>Deidentify</code> action. The
            <code>DataIdentifer</code> array must exactly match the <code>DataIdentifer</code> array
          in the first block of the policy.</p>
               <p>The <code>Operation</code> property with the <code>Deidentify</code> action is what
          actually masks the data, and it must contain the <code> "MaskConfig": {}</code> object.
          The <code> "MaskConfig": {}</code> object must be empty.</p>
            </li>
         </ul>
         <p>For an example data protection policy, see the <b>Examples</b>
      section on this page.</p>
         <important>
            <p>The contents of the two <code>DataIdentifer</code> arrays must match exactly.</p>
         </important>
         <p>In addition to the two JSON blocks, the <code>policyDocument</code> can also include
        <code>Name</code>, <code>Description</code>, and <code>Version</code> fields. The
        <code>Name</code> is different than the operation's <code>policyName</code> parameter, and
      is used as a dimension when CloudWatch Logs reports audit findings metrics to CloudWatch.</p>
         <p>The JSON specified in <code>policyDocument</code> can be up to 30,720 characters
      long.</p>
         <p>
            <b>Subscription filter policy</b>
         </p>
         <p>A subscription filter policy can include the following attributes in a JSON block:</p>
         <ul>
            <li>
               <p>
                  <b>DestinationArn</b> The ARN of the destination to deliver
          log events to. Supported destinations are:</p>
               <ul>
                  <li>
                     <p>An Kinesis Data Streams data stream in the same account as the subscription policy,
              for same-account delivery.</p>
                  </li>
                  <li>
                     <p>An Firehose data stream in the same account as the subscription policy,
              for same-account delivery.</p>
                  </li>
                  <li>
                     <p>A Lambda function in the same account as the subscription policy, for
              same-account delivery.</p>
                  </li>
                  <li>
                     <p>A logical destination in a different account created with <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_PutDestination.html">PutDestination</a>, for cross-account delivery. Kinesis Data Streams and Firehose are supported as logical destinations.</p>
                  </li>
               </ul>
            </li>
            <li>
               <p>
                  <b>RoleArn</b> The ARN of an IAM role that grants CloudWatch
          Logs permissions to deliver ingested log events to the destination stream. You don't need
          to provide the ARN when you are working with a logical destination for cross-account
          delivery.</p>
            </li>
            <li>
               <p>
                  <b>FilterPattern</b> A filter pattern for subscribing to
          a filtered stream of log events.</p>
            </li>
            <li>
               <p>
                  <b>Distribution</b> The method used to distribute log data
          to the destination. By default, log data is grouped by log stream, but the grouping can be
          set to <code>Random</code> for a more even distribution. This property is only applicable
          when the destination is an Kinesis Data Streams data stream.</p>
            </li>
         </ul>
         <p>
            <b>Transformer policy</b>
         </p>
         <p>A transformer policy must include one JSON block with the array of processors and their
      configurations. For more information about available processors, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/CloudWatch-Logs-Transformation.html#CloudWatch-Logs-Transformation-Processors"> Processors that you can use</a>. </p>
         <p>
            <b>Field index policy</b>
         </p>
         <p>A field index filter policy can include the following attribute in a JSON block:</p>
         <ul>
            <li>
               <p>
                  <b>Fields</b> The array of field indexes to create.</p>
            </li>
         </ul>
         <p>It must contain at least one field index.</p>
         <p>The following is an example of an index policy document that creates two indexes,
        <code>RequestId</code> and <code>TransactionId</code>.</p>
         <p>
            <code>"policyDocument": "{ \"Fields\": [ \"RequestId\", \"TransactionId\" ]
      }"</code>
         </p>
    policy_name = "value"  # <p>A name for the policy. This must be unique within the account.</p>
    policy_type = "value"  # <p>The type of policy that you're creating or updating.</p>
}

```

---


### Transformer

Transformer resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `log_group_identifier` | String | ✅ | <p>Specify either the name or ARN of the log group to create the transformer for. </p> |
| `transformer_config` | Vec<String> | ✅ | <p>This structure contains the configuration of this log transformer. A log transformer is an
      array of processors, where each processor applies one type of transformation to the log events
      that are ingested.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `last_modified_time` | String | <p>The date and time when this transformer was most recently modified, expressed as the
      number of milliseconds after Jan 1, 1970 00:00:00 UTC.</p> |
| `transformer_config` | Vec<String> | <p>This sructure contains the configuration of the requested transformer.</p> |
| `log_group_identifier` | String | <p>The ARN of the log group that you specified in your request.</p> |
| `creation_time` | String | <p>The creation time of the transformer, expressed as the number of milliseconds after Jan
      1, 1970 00:00:00 UTC.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create transformer
transformer = provider.cloudwatch_logs.Transformer {
    log_group_identifier = "value"  # <p>Specify either the name or ARN of the log group to create the transformer for. </p>
    transformer_config = "value"  # <p>This structure contains the configuration of this log transformer. A log transformer is an
      array of processors, where each processor applies one type of transformation to the log events
      that are ingested.</p>
}

# Access transformer outputs
transformer_id = transformer.id
transformer_last_modified_time = transformer.last_modified_time
transformer_transformer_config = transformer.transformer_config
transformer_log_group_identifier = transformer.log_group_identifier
transformer_creation_time = transformer.creation_time
```

---


### Log_streams

LogStreams resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String |  |
| `log_streams` | Vec<String> | <p>The log streams.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access log_streams outputs
log_streams_id = log_streams.id
log_streams_next_token = log_streams.next_token
log_streams_log_streams = log_streams.log_streams
```

---


### Data_protection_policy

DataProtectionPolicy resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `policy_document` | String | ✅ | <p>Specify the data protection policy, in JSON.</p>
         <p>This policy must include two JSON blocks:</p>
         <ul>
            <li>
               <p>The first block must include both a <code>DataIdentifer</code> array and an
            <code>Operation</code> property with an <code>Audit</code> action. The
            <code>DataIdentifer</code> array lists the types of sensitive data that you want to
          mask. For more information about the available options, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/mask-sensitive-log-data-types.html">Types of data that
            you can mask</a>.</p>
               <p>The <code>Operation</code> property with an <code>Audit</code> action is required to
          find the sensitive data terms. This <code>Audit</code> action must contain a
            <code>FindingsDestination</code> object. You can optionally use that
            <code>FindingsDestination</code> object to list one or more destinations to send audit
          findings to. If you specify destinations such as log groups, Firehose streams,
          and S3 buckets, they must already exist.</p>
            </li>
            <li>
               <p>The second block must include both a <code>DataIdentifer</code> array and an
            <code>Operation</code> property with an <code>Deidentify</code> action. The
            <code>DataIdentifer</code> array must exactly match the <code>DataIdentifer</code> array
          in the first block of the policy.</p>
               <p>The <code>Operation</code> property with the <code>Deidentify</code> action is what
          actually masks the data, and it must contain the <code> "MaskConfig": {}</code> object.
          The <code> "MaskConfig": {}</code> object must be empty.</p>
            </li>
         </ul>
         <p>For an example data protection policy, see the <b>Examples</b>
      section on this page.</p>
         <important>
            <p>The contents of the two <code>DataIdentifer</code> arrays must match exactly.</p>
         </important>
         <p>In addition to the two JSON blocks, the <code>policyDocument</code> can also include
        <code>Name</code>, <code>Description</code>, and <code>Version</code> fields. The
        <code>Name</code> is used as a dimension when CloudWatch Logs reports audit findings
      metrics to CloudWatch.</p>
         <p>The JSON specified in <code>policyDocument</code> can be up to 30,720 characters.</p> |
| `log_group_identifier` | String | ✅ | <p>Specify either the log group name or log group ARN.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `last_updated_time` | String | <p>The date and time that this policy was most recently updated.</p> |
| `policy_document` | String | <p>The data protection policy document for this log group.</p> |
| `log_group_identifier` | String | <p>The log group name or ARN that you specified in your request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create data_protection_policy
data_protection_policy = provider.cloudwatch_logs.Data_protection_policy {
    policy_document = "value"  # <p>Specify the data protection policy, in JSON.</p>
         <p>This policy must include two JSON blocks:</p>
         <ul>
            <li>
               <p>The first block must include both a <code>DataIdentifer</code> array and an
            <code>Operation</code> property with an <code>Audit</code> action. The
            <code>DataIdentifer</code> array lists the types of sensitive data that you want to
          mask. For more information about the available options, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/mask-sensitive-log-data-types.html">Types of data that
            you can mask</a>.</p>
               <p>The <code>Operation</code> property with an <code>Audit</code> action is required to
          find the sensitive data terms. This <code>Audit</code> action must contain a
            <code>FindingsDestination</code> object. You can optionally use that
            <code>FindingsDestination</code> object to list one or more destinations to send audit
          findings to. If you specify destinations such as log groups, Firehose streams,
          and S3 buckets, they must already exist.</p>
            </li>
            <li>
               <p>The second block must include both a <code>DataIdentifer</code> array and an
            <code>Operation</code> property with an <code>Deidentify</code> action. The
            <code>DataIdentifer</code> array must exactly match the <code>DataIdentifer</code> array
          in the first block of the policy.</p>
               <p>The <code>Operation</code> property with the <code>Deidentify</code> action is what
          actually masks the data, and it must contain the <code> "MaskConfig": {}</code> object.
          The <code> "MaskConfig": {}</code> object must be empty.</p>
            </li>
         </ul>
         <p>For an example data protection policy, see the <b>Examples</b>
      section on this page.</p>
         <important>
            <p>The contents of the two <code>DataIdentifer</code> arrays must match exactly.</p>
         </important>
         <p>In addition to the two JSON blocks, the <code>policyDocument</code> can also include
        <code>Name</code>, <code>Description</code>, and <code>Version</code> fields. The
        <code>Name</code> is used as a dimension when CloudWatch Logs reports audit findings
      metrics to CloudWatch.</p>
         <p>The JSON specified in <code>policyDocument</code> can be up to 30,720 characters.</p>
    log_group_identifier = "value"  # <p>Specify either the log group name or log group ARN.</p>
}

# Access data_protection_policy outputs
data_protection_policy_id = data_protection_policy.id
data_protection_policy_last_updated_time = data_protection_policy.last_updated_time
data_protection_policy_policy_document = data_protection_policy.policy_document
data_protection_policy_log_group_identifier = data_protection_policy.log_group_identifier
```

---


### Destination_policy

DestinationPolicy resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `destination_name` | String | ✅ | <p>A name for an existing destination.</p> |
| `force_update` | bool |  | <p>Specify true if you are updating an existing destination policy to grant permission to an
      organization ID instead of granting permission to individual Amazon Web Services accounts.
      Before you update a destination policy this way, you must first update the subscription
      filters in the accounts that send logs to this destination. If you do not, the subscription
      filters might stop working. By specifying <code>true</code> for <code>forceUpdate</code>, you
      are affirming that you have already updated the subscription filters. For more information,
      see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/Cross-Account-Log_Subscription-Update.html"> Updating an
        existing cross-account subscription</a>
         </p>
         <p>If you omit this parameter, the default of <code>false</code> is used.</p> |
| `access_policy` | String | ✅ | <p>An IAM policy document that authorizes cross-account users to deliver their log events
      to the associated destination. This can be up to 5120 bytes.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create destination_policy
destination_policy = provider.cloudwatch_logs.Destination_policy {
    destination_name = "value"  # <p>A name for an existing destination.</p>
    access_policy = "value"  # <p>An IAM policy document that authorizes cross-account users to deliver their log events
      to the associated destination. This can be up to 5120 bytes.</p>
}

```

---


### Log_groups

LogGroups resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String |  |
| `log_groups` | Vec<String> | <p>An array of structures, where each structure contains the information about one log
      group.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access log_groups outputs
log_groups_id = log_groups.id
log_groups_next_token = log_groups.next_token
log_groups_log_groups = log_groups.log_groups
```

---


### Delivery_configuration

DeliveryConfiguration resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `field_delimiter` | String |  | <p>The field delimiter to use between record fields when the final output format of a
      delivery is in <code>Plain</code>, <code>W3C</code>, or <code>Raw</code> format.</p> |
| `id` | String | ✅ | <p>The ID of the delivery to be updated by this request.</p> |
| `record_fields` | Vec<String> |  | <p>The list of record fields to be delivered to the destination, in order. If the delivery's
      log source has mandatory fields, they must be included in this list.</p> |
| `s3_delivery_configuration` | String |  | <p>This structure contains parameters that are valid only when the delivery's delivery
      destination is an S3 bucket.</p> |



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


### Delivery_destination

DeliveryDestination resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `delivery_destination_type` | String |  | <p>The type of delivery destination. This parameter specifies the target service where log
      data will be delivered. Valid values include:</p>
         <ul>
            <li>
               <p>
                  <code>S3</code> - Amazon S3 for long-term storage and analytics</p>
            </li>
            <li>
               <p>
                  <code>CWL</code> - CloudWatch Logs for centralized log management</p>
            </li>
            <li>
               <p>
                  <code>FH</code> - Amazon Kinesis Data Firehose for real-time data streaming</p>
            </li>
            <li>
               <p>
                  <code>XRAY</code> - Amazon Web Services
          X-Ray for distributed tracing and application monitoring</p>
            </li>
         </ul>
         <p>The delivery destination type determines the format and configuration options available
      for log delivery.</p> |
| `delivery_destination_configuration` | String |  | <p>A structure that contains the ARN of the Amazon Web Services resource that will receive the
      logs.</p>
         <note>
            <p>
               <code>deliveryDestinationConfiguration</code> is required for CloudWatch Logs,
          Amazon S3, Firehose log delivery destinations and not required for
          X-Ray trace delivery destinations. <code>deliveryDestinationType</code> is
        needed for X-Ray trace delivery destinations but not required for other logs
        delivery destinations.</p>
         </note> |
| `tags` | HashMap<String, String> |  | <p>An optional list of key-value pairs to associate with the resource.</p>
         <p>For more information about tagging, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services resources</a>
         </p> |
| `name` | String | ✅ | <p>A name for this delivery destination. This name must be unique for all delivery
      destinations in your account.</p> |
| `output_format` | String |  | <p>The format for the logs that this delivery destination will receive.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `delivery_destination` | String | <p>A structure containing information about the delivery destination.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create delivery_destination
delivery_destination = provider.cloudwatch_logs.Delivery_destination {
    name = "value"  # <p>A name for this delivery destination. This name must be unique for all delivery
      destinations in your account.</p>
}

# Access delivery_destination outputs
delivery_destination_id = delivery_destination.id
delivery_destination_delivery_destination = delivery_destination.delivery_destination
```

---


### Subscription_filters

SubscriptionFilters resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String |  |
| `subscription_filters` | Vec<String> | <p>The subscription filters.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access subscription_filters outputs
subscription_filters_id = subscription_filters.id
subscription_filters_next_token = subscription_filters.next_token
subscription_filters_subscription_filters = subscription_filters.subscription_filters
```

---


### Delivery_destinations

DeliveryDestinations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String |  |
| `delivery_destinations` | Vec<String> | <p>An array of structures. Each structure contains information about one delivery destination
      in the account.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access delivery_destinations outputs
delivery_destinations_id = delivery_destinations.id
delivery_destinations_next_token = delivery_destinations.next_token
delivery_destinations_delivery_destinations = delivery_destinations.delivery_destinations
```

---


### Index_policies

IndexPolicies resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `index_policies` | Vec<String> | <p>An array containing the field index policies.</p> |
| `next_token` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access index_policies outputs
index_policies_id = index_policies.id
index_policies_index_policies = index_policies.index_policies
index_policies_next_token = index_policies.next_token
```

---


### Delivery

Delivery resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `s3_delivery_configuration` | String |  | <p>This structure contains parameters that are valid only when the delivery's delivery
      destination is an S3 bucket.</p> |
| `field_delimiter` | String |  | <p>The field delimiter to use between record fields when the final output format of a
      delivery is in <code>Plain</code>, <code>W3C</code>, or <code>Raw</code> format.</p> |
| `delivery_source_name` | String | ✅ | <p>The name of the delivery source to use for this delivery.</p> |
| `record_fields` | Vec<String> |  | <p>The list of record fields to be delivered to the destination, in order. If the delivery's
      log source has mandatory fields, they must be included in this list.</p> |
| `tags` | HashMap<String, String> |  | <p>An optional list of key-value pairs to associate with the resource.</p>
         <p>For more information about tagging, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services resources</a>
         </p> |
| `delivery_destination_arn` | String | ✅ | <p>The ARN of the delivery destination to use for this delivery.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `delivery` | String | <p>A structure that contains information about the delivery.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create delivery
delivery = provider.cloudwatch_logs.Delivery {
    delivery_source_name = "value"  # <p>The name of the delivery source to use for this delivery.</p>
    delivery_destination_arn = "value"  # <p>The ARN of the delivery destination to use for this delivery.</p>
}

# Access delivery outputs
delivery_id = delivery.id
delivery_delivery = delivery.delivery
```

---


### Integration

Integration resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resource_config` | String | ✅ | <p>A structure that contains configuration information for the integration that you are
      creating.</p> |
| `integration_type` | String | ✅ | <p>The type of integration. Currently, the only supported type is
      <code>OPENSEARCH</code>.</p> |
| `integration_name` | String | ✅ | <p>A name for the integration.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `integration_details` | String | <p>A structure that contains information about the integration configuration. For an
      integration with OpenSearch Service, this includes information about OpenSearch Service
      resources such as the collection, the workspace, and policies.</p> |
| `integration_status` | String | <p>The current status of this integration.</p> |
| `integration_name` | String | <p>The name of the integration.</p> |
| `integration_type` | String | <p>The type of integration. Integrations with OpenSearch Service have the type
        <code>OPENSEARCH</code>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create integration
integration = provider.cloudwatch_logs.Integration {
    resource_config = "value"  # <p>A structure that contains configuration information for the integration that you are
      creating.</p>
    integration_type = "value"  # <p>The type of integration. Currently, the only supported type is
      <code>OPENSEARCH</code>.</p>
    integration_name = "value"  # <p>A name for the integration.</p>
}

# Access integration outputs
integration_id = integration.id
integration_integration_details = integration.integration_details
integration_integration_status = integration.integration_status
integration_integration_name = integration.integration_name
integration_integration_type = integration.integration_type
```

---


### Log_record

LogRecord resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `log_record` | HashMap<String, String> | <p>The requested log event, as a JSON string.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access log_record outputs
log_record_id = log_record.id
log_record_log_record = log_record.log_record
```

---


### Metric_filter

MetricFilter resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `metric_transformations` | Vec<String> | ✅ | <p>A collection of information that defines how metric data gets emitted.</p> |
| `emit_system_field_dimensions` | Vec<String> |  | <p>A list of system fields to emit as additional dimensions in the generated metrics. Valid values are <code>@aws.account</code> and <code>@aws.region</code>. These dimensions help identify the source of centralized log data and count toward the total dimension limit for metric filters.</p> |
| `log_group_name` | String | ✅ | <p>The name of the log group.</p> |
| `apply_on_transformed_logs` | bool |  | <p>This parameter is valid only for log groups that have an active log transformer. For more
      information about log transformers, see <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_PutTransformer.html">PutTransformer</a>.</p>
         <p>If the log group uses either a log-group level or account-level transformer, and you
      specify <code>true</code>, the metric filter will be applied on the transformed version of the
      log events instead of the original ingested log events.</p> |
| `filter_pattern` | String | ✅ | <p>A filter pattern for extracting metric data out of ingested log events.</p> |
| `filter_name` | String | ✅ | <p>A name for the metric filter.</p> |
| `field_selection_criteria` | String |  | <p>A filter expression that specifies which log events should be processed by this metric filter based on system fields such as source account and source region. Uses selection criteria syntax with operators like <code>=</code>, <code>!=</code>, <code>AND</code>, <code>OR</code>, <code>IN</code>, <code>NOT IN</code>. Example: <code>@aws.region = "us-east-1"</code> or <code>@aws.account IN ["123456789012", "987654321098"]</code>. Maximum length: 2000 characters.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create metric_filter
metric_filter = provider.cloudwatch_logs.Metric_filter {
    metric_transformations = "value"  # <p>A collection of information that defines how metric data gets emitted.</p>
    log_group_name = "value"  # <p>The name of the log group.</p>
    filter_pattern = "value"  # <p>A filter pattern for extracting metric data out of ingested log events.</p>
    filter_name = "value"  # <p>A name for the metric filter.</p>
}

```

---


### Index_policy

IndexPolicy resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `log_group_identifier` | String | ✅ | <p>Specify either the log group name or log group ARN to apply this field index policy to. If
      you specify an ARN, use the format
        arn:aws:logs:<i>region</i>:<i>account-id</i>:log-group:<i>log_group_name</i>
      Don't include an * at the end.</p> |
| `policy_document` | String | ✅ | <p>The index policy document, in JSON format. The following is an example of an index policy
      document that creates two indexes, <code>RequestId</code> and
      <code>TransactionId</code>.</p>
         <p>
            <code>"policyDocument": "{ "Fields": [ "RequestId", "TransactionId" ] }"</code>
         </p>
         <p>The policy document must include at least one field index. For more information about the
      fields that can be included and other restrictions, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/CloudWatchLogs-Field-Indexing-Syntax.html">Field index
        syntax and quotas</a>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create index_policy
index_policy = provider.cloudwatch_logs.Index_policy {
    log_group_identifier = "value"  # <p>Specify either the log group name or log group ARN to apply this field index policy to. If
      you specify an ARN, use the format
        arn:aws:logs:<i>region</i>:<i>account-id</i>:log-group:<i>log_group_name</i>
      Don't include an * at the end.</p>
    policy_document = "value"  # <p>The index policy document, in JSON format. The following is an example of an index policy
      document that creates two indexes, <code>RequestId</code> and
      <code>TransactionId</code>.</p>
         <p>
            <code>"policyDocument": "{ "Fields": [ "RequestId", "TransactionId" ] }"</code>
         </p>
         <p>The policy document must include at least one field index. For more information about the
      fields that can be included and other restrictions, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/CloudWatchLogs-Field-Indexing-Syntax.html">Field index
        syntax and quotas</a>.</p>
}

```

---


### Delivery_source

DeliverySource resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>A name for this delivery source. This name must be unique for all delivery sources in your
      account.</p> |
| `resource_arn` | String | ✅ | <p>The ARN of the Amazon Web Services resource that is generating and sending logs. For
      example,
        <code>arn:aws:workmail:us-east-1:123456789012:organization/m-1234EXAMPLEabcd1234abcd1234abcd1234</code>
         </p> |
| `tags` | HashMap<String, String> |  | <p>An optional list of key-value pairs to associate with the resource.</p>
         <p>For more information about tagging, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services resources</a>
         </p> |
| `log_type` | String | ✅ | <p>Defines the type of log that the source is sending.</p>
         <ul>
            <li>
               <p>For Amazon Bedrock, the valid value is <code>APPLICATION_LOGS</code> and
            <code>TRACES</code>.</p>
            </li>
            <li>
               <p>For CloudFront, the valid value is <code>ACCESS_LOGS</code>.</p>
            </li>
            <li>
               <p>For Amazon CodeWhisperer, the valid value is <code>EVENT_LOGS</code>.</p>
            </li>
            <li>
               <p>For Elemental MediaPackage, the valid values are <code>EGRESS_ACCESS_LOGS</code> and
            <code>INGRESS_ACCESS_LOGS</code>.</p>
            </li>
            <li>
               <p>For Elemental MediaTailor, the valid values are <code>AD_DECISION_SERVER_LOGS</code>,
            <code>MANIFEST_SERVICE_LOGS</code>, and <code>TRANSCODE_LOGS</code>.</p>
            </li>
            <li>
               <p>For Entity Resolution, the valid value is <code>WORKFLOW_LOGS</code>.</p>
            </li>
            <li>
               <p>For IAM Identity Center, the valid value is
          <code>ERROR_LOGS</code>.</p>
            </li>
            <li>
               <p>For PCS, the valid values are <code>PCS_SCHEDULER_LOGS</code> and
            <code>PCS_JOBCOMP_LOGS</code>.</p>
            </li>
            <li>
               <p>For Amazon Q, the valid value is <code>EVENT_LOGS</code>.</p>
            </li>
            <li>
               <p>For Amazon SES mail manager, the valid values are <code>APPLICATION_LOG</code>
          and <code>TRAFFIC_POLICY_DEBUG_LOGS</code>.</p>
            </li>
            <li>
               <p>For Amazon WorkMail, the valid values are <code>ACCESS_CONTROL_LOGS</code>,
            <code>AUTHENTICATION_LOGS</code>, <code>WORKMAIL_AVAILABILITY_PROVIDER_LOGS</code>,
            <code>WORKMAIL_MAILBOX_ACCESS_LOGS</code>, and
            <code>WORKMAIL_PERSONAL_ACCESS_TOKEN_LOGS</code>.</p>
            </li>
            <li>
               <p>For Amazon VPC Route Server, the valid value is
          <code>EVENT_LOGS</code>.</p>
            </li>
         </ul> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `delivery_source` | String | <p>A structure containing information about the delivery source.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create delivery_source
delivery_source = provider.cloudwatch_logs.Delivery_source {
    name = "value"  # <p>A name for this delivery source. This name must be unique for all delivery sources in your
      account.</p>
    resource_arn = "value"  # <p>The ARN of the Amazon Web Services resource that is generating and sending logs. For
      example,
        <code>arn:aws:workmail:us-east-1:123456789012:organization/m-1234EXAMPLEabcd1234abcd1234abcd1234</code>
         </p>
    log_type = "value"  # <p>Defines the type of log that the source is sending.</p>
         <ul>
            <li>
               <p>For Amazon Bedrock, the valid value is <code>APPLICATION_LOGS</code> and
            <code>TRACES</code>.</p>
            </li>
            <li>
               <p>For CloudFront, the valid value is <code>ACCESS_LOGS</code>.</p>
            </li>
            <li>
               <p>For Amazon CodeWhisperer, the valid value is <code>EVENT_LOGS</code>.</p>
            </li>
            <li>
               <p>For Elemental MediaPackage, the valid values are <code>EGRESS_ACCESS_LOGS</code> and
            <code>INGRESS_ACCESS_LOGS</code>.</p>
            </li>
            <li>
               <p>For Elemental MediaTailor, the valid values are <code>AD_DECISION_SERVER_LOGS</code>,
            <code>MANIFEST_SERVICE_LOGS</code>, and <code>TRANSCODE_LOGS</code>.</p>
            </li>
            <li>
               <p>For Entity Resolution, the valid value is <code>WORKFLOW_LOGS</code>.</p>
            </li>
            <li>
               <p>For IAM Identity Center, the valid value is
          <code>ERROR_LOGS</code>.</p>
            </li>
            <li>
               <p>For PCS, the valid values are <code>PCS_SCHEDULER_LOGS</code> and
            <code>PCS_JOBCOMP_LOGS</code>.</p>
            </li>
            <li>
               <p>For Amazon Q, the valid value is <code>EVENT_LOGS</code>.</p>
            </li>
            <li>
               <p>For Amazon SES mail manager, the valid values are <code>APPLICATION_LOG</code>
          and <code>TRAFFIC_POLICY_DEBUG_LOGS</code>.</p>
            </li>
            <li>
               <p>For Amazon WorkMail, the valid values are <code>ACCESS_CONTROL_LOGS</code>,
            <code>AUTHENTICATION_LOGS</code>, <code>WORKMAIL_AVAILABILITY_PROVIDER_LOGS</code>,
            <code>WORKMAIL_MAILBOX_ACCESS_LOGS</code>, and
            <code>WORKMAIL_PERSONAL_ACCESS_TOKEN_LOGS</code>.</p>
            </li>
            <li>
               <p>For Amazon VPC Route Server, the valid value is
          <code>EVENT_LOGS</code>.</p>
            </li>
         </ul>
}

# Access delivery_source outputs
delivery_source_id = delivery_source.id
delivery_source_delivery_source = delivery_source.delivery_source
```

---


### Retention_policy

RetentionPolicy resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `retention_in_days` | i64 | ✅ |  |
| `log_group_name` | String | ✅ | <p>The name of the log group.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create retention_policy
retention_policy = provider.cloudwatch_logs.Retention_policy {
    retention_in_days = "value"  # Required field
    log_group_name = "value"  # <p>The name of the log group.</p>
}

```

---


### Metric_filters

MetricFilters resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String |  |
| `metric_filters` | Vec<String> | <p>The metric filters.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access metric_filters outputs
metric_filters_id = metric_filters.id
metric_filters_next_token = metric_filters.next_token
metric_filters_metric_filters = metric_filters.metric_filters
```

---


### Resource_policy

ResourcePolicy resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `policy_document` | String |  | <p>Details of the new policy, including the identity of the principal that is enabled to
      put logs to this account. This is formatted as a JSON string. This parameter is
      required.</p>
         <p>The following example creates a resource policy enabling the Route 53 service to put
      DNS query logs in to the specified log group. Replace <code>"logArn"</code> with the ARN of
      your CloudWatch Logs resource, such as a log group or log stream.</p>
         <p>CloudWatch Logs also supports <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_condition-keys.html#condition-keys-sourcearn">aws:SourceArn</a> and <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_condition-keys.html#condition-keys-sourceaccount">aws:SourceAccount</a> condition context keys.</p>
         <p>In the example resource policy, you would replace the value of <code>SourceArn</code> with
      the resource making the call from Route 53 to CloudWatch Logs. You would also
      replace the value of <code>SourceAccount</code> with the Amazon Web Services account ID making
      that call.</p>
         <p></p>
         <p>
            <code>{ "Version": "2012-10-17", "Statement": [ { "Sid": "Route53LogsToCloudWatchLogs",
        "Effect": "Allow", "Principal": { "Service": [ "route53.amazonaws.com" ] }, "Action":
        "logs:PutLogEvents", "Resource": "logArn", "Condition": { "ArnLike": { "aws:SourceArn":
        "myRoute53ResourceArn" }, "StringEquals": { "aws:SourceAccount": "myAwsAccountId" } } } ]
        }</code>
         </p> |
| `policy_name` | String |  | <p>Name of the new policy. This parameter is required.</p> |
| `resource_arn` | String |  | <p>The ARN of the CloudWatch Logs resource to which the resource policy needs to be added
      or attached. Currently only supports LogGroup ARN.</p> |
| `expected_revision_id` | String |  | <p>The expected revision ID of the resource policy. Required when <code>resourceArn</code> is
      provided to prevent concurrent modifications. Use <code>null</code> when creating a resource
      policy for the first time.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create resource_policy
resource_policy = provider.cloudwatch_logs.Resource_policy {
}

```

---


### Log_group_fields

LogGroupFields resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `log_group_fields` | Vec<String> | <p>The array of fields found in the query. Each object in the array contains the name of the
      field, along with the percentage of time it appeared in the log events that were
      queried.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access log_group_fields outputs
log_group_fields_id = log_group_fields.id
log_group_fields_log_group_fields = log_group_fields.log_group_fields
```

---


### Deliveries

Deliveries resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `deliveries` | Vec<String> | <p>An array of structures. Each structure contains information about one delivery in the
      account.</p> |
| `next_token` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access deliveries outputs
deliveries_id = deliveries.id
deliveries_deliveries = deliveries.deliveries
deliveries_next_token = deliveries.next_token
```

---


### Log_stream

LogStream resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `log_stream_name` | String | ✅ | <p>The name of the log stream.</p> |
| `log_group_name` | String | ✅ | <p>The name of the log group.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create log_stream
log_stream = provider.cloudwatch_logs.Log_stream {
    log_stream_name = "value"  # <p>The name of the log stream.</p>
    log_group_name = "value"  # <p>The name of the log group.</p>
}

```

---


### Destination

Destination resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `destination_name` | String | ✅ | <p>A name for the destination.</p> |
| `target_arn` | String | ✅ | <p>The ARN of an Amazon Kinesis stream to which to deliver matching log events.</p> |
| `role_arn` | String | ✅ | <p>The ARN of an IAM role that grants CloudWatch Logs permissions to call the Amazon
      Kinesis <code>PutRecord</code> operation on the destination stream.</p> |
| `tags` | HashMap<String, String> |  | <p>An optional list of key-value pairs to associate with the resource.</p>
         <p>For more information about tagging, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services resources</a>
         </p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create destination
destination = provider.cloudwatch_logs.Destination {
    destination_name = "value"  # <p>A name for the destination.</p>
    target_arn = "value"  # <p>The ARN of an Amazon Kinesis stream to which to deliver matching log events.</p>
    role_arn = "value"  # <p>The ARN of an IAM role that grants CloudWatch Logs permissions to call the Amazon
      Kinesis <code>PutRecord</code> operation on the destination stream.</p>
}

```

---


### Query_definition

QueryDefinition resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `query_definition_id` | String |  | <p>If you are updating a query definition, use this parameter to specify the ID of the query
      definition that you want to update. You can use <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_DescribeQueryDefinitions.html">DescribeQueryDefinitions</a> to retrieve the IDs of your saved query
      definitions.</p>
         <p>If you are creating a query definition, do not specify this parameter. CloudWatch
      generates a unique ID for the new query definition and include it in the response to this
      operation.</p> |
| `log_group_names` | Vec<String> |  | <p>Use this parameter to include specific log groups as part of your query definition. If
      your query uses the OpenSearch Service query language, you specify the log group names inside
      the <code>querystring</code> instead of here.</p>
         <p>If you are updating an existing query definition for the Logs Insights QL or OpenSearch Service PPL and you omit this parameter, then the updated definition will contain no log
      groups.</p> |
| `query_string` | String | ✅ | <p>The query string to use for this definition. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/CWL_QuerySyntax.html">CloudWatch Logs
        Insights Query Syntax</a>.</p> |
| `client_token` | String |  | <p>Used as an idempotency token, to avoid returning an exception if the service receives the
      same request twice because of a network error.</p> |
| `name` | String | ✅ | <p>A name for the query definition. If you are saving numerous query definitions, we
      recommend that you name them. This way, you can find the ones you want by using the first part
      of the name as a filter in the <code>queryDefinitionNamePrefix</code> parameter of <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_DescribeQueryDefinitions.html">DescribeQueryDefinitions</a>.</p> |
| `query_language` | String |  | <p>Specify the query language to use for this query. The options are Logs Insights QL,
      OpenSearch PPL, and OpenSearch SQL. For more information about the query languages that
        CloudWatch Logs supports, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/CWL_AnalyzeLogData_Languages.html">Supported query
        languages</a>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create query_definition
query_definition = provider.cloudwatch_logs.Query_definition {
    query_string = "value"  # <p>The query string to use for this definition. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/CWL_QuerySyntax.html">CloudWatch Logs
        Insights Query Syntax</a>.</p>
    name = "value"  # <p>A name for the query definition. If you are saving numerous query definitions, we
      recommend that you name them. This way, you can find the ones you want by using the first part
      of the name as a filter in the <code>queryDefinitionNamePrefix</code> parameter of <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_DescribeQueryDefinitions.html">DescribeQueryDefinitions</a>.</p>
}

```

---


### Queries

Queries resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String |  |
| `queries` | Vec<String> | <p>The list of queries that match the request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access queries outputs
queries_id = queries.id
queries_next_token = queries.next_token
queries_queries = queries.queries
```

---


### Log_events

LogEvents resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `log_events` | Vec<String> | ✅ | <p>The log events.</p> |
| `sequence_token` | String |  | <p>The sequence token obtained from the response of the previous <code>PutLogEvents</code>
      call.</p>
         <important>
            <p>The <code>sequenceToken</code> parameter is now ignored in <code>PutLogEvents</code>
        actions. <code>PutLogEvents</code> actions are now accepted and never return
          <code>InvalidSequenceTokenException</code> or <code>DataAlreadyAcceptedException</code>
        even if the sequence token is not valid.</p>
         </important> |
| `entity` | String |  | <p>The entity associated with the log events.</p> |
| `log_group_name` | String | ✅ | <p>The name of the log group.</p> |
| `log_stream_name` | String | ✅ | <p>The name of the log stream.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_forward_token` | String | <p>The token for the next set of items in the forward direction. The token expires after
      24 hours. If you have reached the end of the stream, it returns the same token you passed
      in.</p> |
| `next_backward_token` | String | <p>The token for the next set of items in the backward direction. The token expires after
      24 hours. This token is not null. If you have reached the end of the stream, it returns the
      same token you passed in.</p> |
| `events` | Vec<String> | <p>The events.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create log_events
log_events = provider.cloudwatch_logs.Log_events {
    log_events = "value"  # <p>The log events.</p>
    log_group_name = "value"  # <p>The name of the log group.</p>
    log_stream_name = "value"  # <p>The name of the log stream.</p>
}

# Access log_events outputs
log_events_id = log_events.id
log_events_next_forward_token = log_events.next_forward_token
log_events_next_backward_token = log_events.next_backward_token
log_events_events = log_events.events
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
| `encryption_key` | String | <p>If you associated an KMS key with the CloudWatch Logs Insights
      query results in this account, this field displays the ARN of the key that's used to encrypt
      the query results when <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_StartQuery.html">StartQuery</a> stores
      them.</p> |
| `query_language` | String | <p>The query language used for this query. For more information about the query languages
      that CloudWatch Logs supports, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/CWL_AnalyzeLogData_Languages.html">Supported query
        languages</a>.</p> |
| `results` | Vec<Vec<String>> | <p>The log events that matched the query criteria during the most recent time it ran.</p>
         <p>The <code>results</code> value is an array of arrays. Each log event is one object in the
      top-level array. Each of these log event objects is an array of
        <code>field</code>/<code>value</code> pairs.</p> |
| `statistics` | String | <p>Includes the number of log events scanned by the query, the number of log events that
      matched the query criteria, and the total number of bytes in the scanned log events. These
      values reflect the full raw results of the query.</p> |
| `status` | String | <p>The status of the most recent running of the query. Possible values are
        <code>Cancelled</code>, <code>Complete</code>, <code>Failed</code>, <code>Running</code>,
        <code>Scheduled</code>, <code>Timeout</code>, and <code>Unknown</code>.</p>
         <p>Queries time out after 60 minutes of runtime. To avoid having your queries time out,
      reduce the time range being searched or partition your query into a number of queries.</p> |


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
query_results_encryption_key = query_results.encryption_key
query_results_query_language = query_results.query_language
query_results_results = query_results.results
query_results_statistics = query_results.statistics
query_results_status = query_results.status
```

---


### Anomaly

Anomaly resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `suppression_type` | String |  | <p>Use this to specify whether the suppression to be temporary or infinite. If you specify
        <code>LIMITED</code>, you must also specify a <code>suppressionPeriod</code>. If you specify
        <code>INFINITE</code>, any value for <code>suppressionPeriod</code> is ignored. </p> |
| `anomaly_id` | String |  | <p>If you are suppressing or unsuppressing an anomaly, specify its unique ID here. You can
      find anomaly IDs by using the <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_ListAnomalies.html">ListAnomalies</a>
      operation.</p> |
| `baseline` | bool |  | <p>Set this to <code>true</code> to prevent CloudWatch Logs from displaying this behavior
      as an anomaly in the future. The behavior is then treated as baseline behavior. However, if
      similar but more severe occurrences of this behavior occur in the future, those will still be
      reported as anomalies. </p>
         <p>The default is <code>false</code>
         </p> |
| `anomaly_detector_arn` | String | ✅ | <p>The ARN of the anomaly detector that this operation is to act on.</p> |
| `suppression_period` | String |  | <p>If you are temporarily suppressing an anomaly or pattern, use this structure to specify
      how long the suppression is to last.</p> |
| `pattern_id` | String |  | <p>If you are suppressing or unsuppressing an pattern, specify its unique ID here. You can
      find pattern IDs by using the <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_ListAnomalies.html">ListAnomalies</a>
      operation.</p> |



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


### Delivery_destination_policy

DeliveryDestinationPolicy resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `delivery_destination_name` | String | ✅ | <p>The name of the delivery destination to assign this policy to.</p> |
| `delivery_destination_policy` | String | ✅ | <p>The contents of the policy.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `policy` | String | <p>The IAM policy for this delivery destination.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create delivery_destination_policy
delivery_destination_policy = provider.cloudwatch_logs.Delivery_destination_policy {
    delivery_destination_name = "value"  # <p>The name of the delivery destination to assign this policy to.</p>
    delivery_destination_policy = "value"  # <p>The contents of the policy.</p>
}

# Access delivery_destination_policy outputs
delivery_destination_policy_id = delivery_destination_policy.id
delivery_destination_policy_policy = delivery_destination_policy.policy
```

---


### Export_tasks

ExportTasks resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `export_tasks` | Vec<String> | <p>The export tasks.</p> |
| `next_token` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access export_tasks outputs
export_tasks_id = export_tasks.id
export_tasks_export_tasks = export_tasks.export_tasks
export_tasks_next_token = export_tasks.next_token
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
| `next_token` | String |  |
| `resource_policies` | Vec<String> | <p>The resource policies that exist in this account.</p> |


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
resource_policies_next_token = resource_policies.next_token
resource_policies_resource_policies = resource_policies.resource_policies
```

---


### Field_indexes

FieldIndexes resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `field_indexes` | Vec<String> | <p>An array containing the field index information.</p> |
| `next_token` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access field_indexes outputs
field_indexes_id = field_indexes.id
field_indexes_field_indexes = field_indexes.field_indexes
field_indexes_next_token = field_indexes.next_token
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple delivery_sources resources
delivery_sources_0 = provider.cloudwatch_logs.Delivery_sources {
}
delivery_sources_1 = provider.cloudwatch_logs.Delivery_sources {
}
delivery_sources_2 = provider.cloudwatch_logs.Delivery_sources {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    delivery_sources = provider.cloudwatch_logs.Delivery_sources {
    }
```

---

## Related Documentation

- [AWS Cloudwatch_logs Documentation](https://docs.aws.amazon.com/cloudwatch_logs/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
