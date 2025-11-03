# Xray Service



**Resources**: 22

---

## Overview

The xray service provides access to 22 resource types:

- [Encryption_config](#encryption_config) [CR]
- [Time_series_service_statistics](#time_series_service_statistics) [R]
- [Trace_segment_destination](#trace_segment_destination) [RU]
- [Insight_summaries](#insight_summaries) [R]
- [Sampling_statistic_summaries](#sampling_statistic_summaries) [R]
- [Resource_policy](#resource_policy) [CD]
- [Insight](#insight) [R]
- [Sampling_targets](#sampling_targets) [R]
- [Sampling_rules](#sampling_rules) [R]
- [Group](#group) [CRUD]
- [Retrieved_traces_graph](#retrieved_traces_graph) [R]
- [Trace_graph](#trace_graph) [R]
- [Telemetry_records](#telemetry_records) [C]
- [Trace_summaries](#trace_summaries) [R]
- [Groups](#groups) [R]
- [Trace_segments](#trace_segments) [C]
- [Indexing_rule](#indexing_rule) [U]
- [Service_graph](#service_graph) [R]
- [Sampling_rule](#sampling_rule) [CUD]
- [Indexing_rules](#indexing_rules) [R]
- [Insight_impact_graph](#insight_impact_graph) [R]
- [Insight_events](#insight_events) [R]

---

## Resources


### Encryption_config

EncryptionConfig resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `type` | String | ✅ | <p>The type of encryption. Set to <code>KMS</code> to use your own key for encryption. Set
      to <code>NONE</code> for default encryption.</p> |
| `key_id` | String |  | <p>An Amazon Web Services KMS key in one of the following formats:</p>
         <ul>
            <li>
               <p>
                  <b>Alias</b> - The name of the key. For example,
            <code>alias/MyKey</code>.</p>
            </li>
            <li>
               <p>
                  <b>Key ID</b> - The KMS key ID of the key. For example,
          <code>ae4aa6d49-a4d8-9df9-a475-4ff6d7898456</code>. Amazon Web Services X-Ray does not support asymmetric KMS keys.</p>
            </li>
            <li>
               <p>
                  <b>ARN</b> - The full Amazon Resource Name of the key ID or alias.
          For example,
            <code>arn:aws:kms:us-east-2:123456789012:key/ae4aa6d49-a4d8-9df9-a475-4ff6d7898456</code>.
          Use this format to specify a key in a different account.</p>
            </li>
         </ul>
         <p>Omit this key if you set <code>Type</code> to <code>NONE</code>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `encryption_config` | String | <p>The encryption configuration document.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create encryption_config
encryption_config = provider.xray.Encryption_config {
    type = "value"  # <p>The type of encryption. Set to <code>KMS</code> to use your own key for encryption. Set
      to <code>NONE</code> for default encryption.</p>
}

# Access encryption_config outputs
encryption_config_id = encryption_config.id
encryption_config_encryption_config = encryption_config.encryption_config
```

---


### Time_series_service_statistics

TimeSeriesServiceStatistics resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `contains_old_group_versions` | bool | <p>A flag indicating whether or not a group's filter expression has been consistent, or if a returned
            aggregation might show statistics from an older version of the group's filter expression.</p> |
| `next_token` | String | <p>Pagination token.</p> |
| `time_series_service_statistics` | Vec<String> | <p>The collection of statistics.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access time_series_service_statistics outputs
time_series_service_statistics_id = time_series_service_statistics.id
time_series_service_statistics_contains_old_group_versions = time_series_service_statistics.contains_old_group_versions
time_series_service_statistics_next_token = time_series_service_statistics.next_token
time_series_service_statistics_time_series_service_statistics = time_series_service_statistics.time_series_service_statistics
```

---


### Trace_segment_destination

TraceSegmentDestination resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `destination` | String |  | <p>
The configured destination of trace segments.
</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `destination` | String | <p>
Retrieves the current destination.
</p> |
| `status` | String | <p>
    Status of the retrieval.
  </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access trace_segment_destination outputs
trace_segment_destination_id = trace_segment_destination.id
trace_segment_destination_destination = trace_segment_destination.destination
trace_segment_destination_status = trace_segment_destination.status
```

---


### Insight_summaries

InsightSummaries resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>Pagination token.</p> |
| `insight_summaries` | Vec<String> | <p>The summary of each insight within the group matching the provided filters. The summary
         contains the InsightID, start and end time, the root cause service, the root cause and
         client impact statistics, the top anomalous services, and the status of the insight.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access insight_summaries outputs
insight_summaries_id = insight_summaries.id
insight_summaries_next_token = insight_summaries.next_token
insight_summaries_insight_summaries = insight_summaries.insight_summaries
```

---


### Sampling_statistic_summaries

SamplingStatisticSummaries resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `sampling_statistic_summaries` | Vec<String> | <p>Information about the number of requests instrumented for each sampling
         rule.</p> |
| `next_token` | String | <p>Pagination token.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access sampling_statistic_summaries outputs
sampling_statistic_summaries_id = sampling_statistic_summaries.id
sampling_statistic_summaries_sampling_statistic_summaries = sampling_statistic_summaries.sampling_statistic_summaries
sampling_statistic_summaries_next_token = sampling_statistic_summaries.next_token
```

---


### Resource_policy

ResourcePolicy resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `policy_revision_id` | String |  | <p>Specifies a specific policy revision, to ensure an atomic create operation. By default the resource policy is created if it does not exist, or updated with an incremented revision id. 
            The revision id is unique to each policy in the account.</p>
         <p>If the policy revision id does not match the latest revision id, the operation will fail with an <code>InvalidPolicyRevisionIdException</code> exception. You can also provide a 
            <code>PolicyRevisionId</code> of 0. In this case, the operation will fail with an <code>InvalidPolicyRevisionIdException</code> exception if a resource policy with the same name already exists.
        </p> |
| `bypass_policy_lockout_check` | bool |  | <p>A flag to indicate whether to bypass the resource policy lockout safety check.</p>
         <important>
            <p>Setting this value to true increases the risk that the policy becomes unmanageable. Do not set this value to true indiscriminately.</p>
         </important>
         <p>Use this parameter only when you include a policy in the request and you intend to prevent the principal that is making the request from making a subsequent <code>PutResourcePolicy</code> request.</p>
         <p>The default value is false.</p> |
| `policy_name` | String | ✅ | <p>The name of the resource policy. Must be unique within a specific Amazon Web Services account.</p> |
| `policy_document` | String | ✅ | <p>The resource policy document, which can be up to 5kb in size.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create resource_policy
resource_policy = provider.xray.Resource_policy {
    policy_name = "value"  # <p>The name of the resource policy. Must be unique within a specific Amazon Web Services account.</p>
    policy_document = "value"  # <p>The resource policy document, which can be up to 5kb in size.</p>
}

```

---


### Insight

Insight resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `insight` | String | <p>The summary information of an insight.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access insight outputs
insight_id = insight.id
insight_insight = insight.insight
```

---


### Sampling_targets

SamplingTargets resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `unprocessed_statistics` | Vec<String> | <p>Information about <a href="https://docs.aws.amazon.com/xray/latest/api/API_SamplingStatisticsDocument.html">SamplingStatisticsDocument</a> that X-Ray could not
         process.</p> |
| `unprocessed_boost_statistics` | Vec<String> | <p>Information about <a href="https://docs.aws.amazon.com/xray/latest/api/API_SamplingBoostStatisticsDocument.html">SamplingBoostStatisticsDocument</a> that X-Ray could not
         process.</p> |
| `last_rule_modification` | String | <p>The last time a user changed the sampling rule configuration. If
         the sampling rule configuration changed since the service last retrieved it, the service
         should call <a href="https://docs.aws.amazon.com/xray/latest/api/API_GetSamplingRules.html">GetSamplingRules</a> to get the latest version.</p> |
| `sampling_target_documents` | Vec<String> | <p>Updated rules that the service should use to sample requests.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access sampling_targets outputs
sampling_targets_id = sampling_targets.id
sampling_targets_unprocessed_statistics = sampling_targets.unprocessed_statistics
sampling_targets_unprocessed_boost_statistics = sampling_targets.unprocessed_boost_statistics
sampling_targets_last_rule_modification = sampling_targets.last_rule_modification
sampling_targets_sampling_target_documents = sampling_targets.sampling_target_documents
```

---


### Sampling_rules

SamplingRules resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `sampling_rule_records` | Vec<String> | <p>Rule definitions and metadata.</p> |
| `next_token` | String | <p>Pagination token.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access sampling_rules outputs
sampling_rules_id = sampling_rules.id
sampling_rules_sampling_rule_records = sampling_rules.sampling_rule_records
sampling_rules_next_token = sampling_rules.next_token
```

---


### Group

Group resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `insights_configuration` | String |  | <p>The structure containing configurations related to insights.</p>
         <ul>
            <li>
               <p>The InsightsEnabled boolean can be set to true to enable insights for the
                    new group or false to disable insights for the new group.</p>
            </li>
            <li>
               <p>The NotificationsEnabled boolean can be set to true to enable insights
                    notifications for the new group. Notifications may only be enabled on a group
                    with InsightsEnabled set to true.</p>
            </li>
         </ul> |
| `filter_expression` | String |  | <p>The filter expression defining criteria by which to group traces.</p> |
| `group_name` | String | ✅ | <p>The case-sensitive name of the new group. Default is a reserved name and names must
            be unique.</p> |
| `tags` | Vec<String> |  | <p>A map that contains one or more tag keys and tag values to attach to an X-Ray group.
            For more information about ways to use tags, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services
                resources</a> in the <i>Amazon Web Services General Reference</i>.</p>
         <p>The following restrictions apply to tags:</p>
         <ul>
            <li>
               <p>Maximum number of user-applied tags per resource: 50</p>
            </li>
            <li>
               <p>Maximum tag key length: 128 Unicode characters</p>
            </li>
            <li>
               <p>Maximum tag value length: 256 Unicode characters</p>
            </li>
            <li>
               <p>Valid values for key and value: a-z, A-Z, 0-9, space, and the following characters: _ . :
                    / = + - and @</p>
            </li>
            <li>
               <p>Tag keys and values are case sensitive.</p>
            </li>
            <li>
               <p>Don't use <code>aws:</code> as a prefix for keys; it's reserved for Amazon Web Services
                    use.</p>
            </li>
         </ul> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `group` | String | <p>The group that was requested. Contains the name of the group, the ARN of the group,
            the filter expression, and the insight configuration assigned to the group.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create group
group = provider.xray.Group {
    group_name = "value"  # <p>The case-sensitive name of the new group. Default is a reserved name and names must
            be unique.</p>
}

# Access group outputs
group_id = group.id
group_group = group.group
```

---


### Retrieved_traces_graph

RetrievedTracesGraph resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `services` | Vec<String> | <p>
Retrieved services.
</p> |
| `retrieval_status` | String | <p>
Status of the retrieval.
</p> |
| `next_token` | String | <p>
    Specify the pagination token returned by a previous request to retrieve the next page of indexes.
    
    
  </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access retrieved_traces_graph outputs
retrieved_traces_graph_id = retrieved_traces_graph.id
retrieved_traces_graph_services = retrieved_traces_graph.services
retrieved_traces_graph_retrieval_status = retrieved_traces_graph.retrieval_status
retrieved_traces_graph_next_token = retrieved_traces_graph.next_token
```

---


### Trace_graph

TraceGraph resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>Pagination token.</p> |
| `services` | Vec<String> | <p>The services that have processed one of the specified requests.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access trace_graph outputs
trace_graph_id = trace_graph.id
trace_graph_next_token = trace_graph.next_token
trace_graph_services = trace_graph.services
```

---


### Telemetry_records

TelemetryRecords resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `telemetry_records` | Vec<String> | ✅ | <p></p> |
| `ec2_instance_id` | String |  | <p></p> |
| `hostname` | String |  | <p></p> |
| `resource_arn` | String |  | <p></p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create telemetry_records
telemetry_records = provider.xray.Telemetry_records {
    telemetry_records = "value"  # <p></p>
}

```

---


### Trace_summaries

TraceSummaries resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `traces_processed_count` | i64 | <p>The total number of traces processed, including traces that did not match the specified
      filter expression.</p> |
| `next_token` | String | <p>If the requested time frame contained more than one page of results, you can use this token to retrieve the
      next page. The first page contains the most recent results, closest to the end of the time frame.</p> |
| `approximate_time` | String | <p>The start time of this page of results.</p> |
| `trace_summaries` | Vec<String> | <p>Trace IDs and annotations for traces that were found in the specified time
      frame.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access trace_summaries outputs
trace_summaries_id = trace_summaries.id
trace_summaries_traces_processed_count = trace_summaries.traces_processed_count
trace_summaries_next_token = trace_summaries.next_token
trace_summaries_approximate_time = trace_summaries.approximate_time
trace_summaries_trace_summaries = trace_summaries.trace_summaries
```

---


### Groups

Groups resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `groups` | Vec<String> | <p>The collection of all active groups.</p> |
| `next_token` | String | <p>Pagination token.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access groups outputs
groups_id = groups.id
groups_groups = groups.groups
groups_next_token = groups.next_token
```

---


### Trace_segments

TraceSegments resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `trace_segment_documents` | Vec<String> | ✅ | <p>A string containing a JSON document defining one or more segments or
      subsegments.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create trace_segments
trace_segments = provider.xray.Trace_segments {
    trace_segment_documents = "value"  # <p>A string containing a JSON document defining one or more segments or
      subsegments.</p>
}

```

---


### Indexing_rule

IndexingRule resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `rule` | String | ✅ | <p>
    Rule configuration to be updated.
  </p> |
| `name` | String | ✅ | <p>
  Name of the indexing rule to be updated.
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


### Service_graph

ServiceGraph resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `services` | Vec<String> | <p>The services that have processed a traced request during the specified time
      frame.</p> |
| `end_time` | String | <p>The end of the time frame for which the graph was generated.</p> |
| `contains_old_group_versions` | bool | <p>A flag indicating whether the group's filter expression has been consistent, or
      if the returned service graph may show traces from an older version of the group's filter
      expression.</p> |
| `next_token` | String | <p>Pagination token.</p> |
| `start_time` | String | <p>The start of the time frame for which the graph was generated.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access service_graph outputs
service_graph_id = service_graph.id
service_graph_services = service_graph.services
service_graph_end_time = service_graph.end_time
service_graph_contains_old_group_versions = service_graph.contains_old_group_versions
service_graph_next_token = service_graph.next_token
service_graph_start_time = service_graph.start_time
```

---


### Sampling_rule

SamplingRule resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `sampling_rule` | String | ✅ | <p>The rule definition.</p> |
| `tags` | Vec<String> |  | <p>A map that contains one or more tag keys and tag values to attach to an X-Ray sampling
         rule. For more information about ways to use tags, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services
            resources</a> in the <i>Amazon Web Services General Reference</i>.</p>
         <p>The following restrictions apply to tags:</p>
         <ul>
            <li>
               <p>Maximum number of user-applied tags per resource: 50</p>
            </li>
            <li>
               <p>Maximum tag key length: 128 Unicode characters</p>
            </li>
            <li>
               <p>Maximum tag value length: 256 Unicode characters</p>
            </li>
            <li>
               <p>Valid values for key and value: a-z, A-Z, 0-9, space, and the following characters: _ . :
               / = + - and @</p>
            </li>
            <li>
               <p>Tag keys and values are case sensitive.</p>
            </li>
            <li>
               <p>Don't use <code>aws:</code> as a prefix for keys; it's reserved for Amazon Web Services
               use.</p>
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

# Create sampling_rule
sampling_rule = provider.xray.Sampling_rule {
    sampling_rule = "value"  # <p>The rule definition.</p>
}

```

---


### Indexing_rules

IndexingRules resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>
      Specify the pagination token returned by a previous request to retrieve the next page of indexes.
    </p> |
| `indexing_rules` | Vec<String> | <p>
    Retrieves all indexing rules.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access indexing_rules outputs
indexing_rules_id = indexing_rules.id
indexing_rules_next_token = indexing_rules.next_token
indexing_rules_indexing_rules = indexing_rules.indexing_rules
```

---


### Insight_impact_graph

InsightImpactGraph resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>Pagination token.</p> |
| `start_time` | String | <p>The provided start time.</p> |
| `service_graph_start_time` | String | <p>The time, in Unix seconds, at which the service graph started.</p> |
| `services` | Vec<String> | <p>The Amazon Web Services instrumented services related to the insight.</p> |
| `end_time` | String | <p>The provided end time. </p> |
| `service_graph_end_time` | String | <p>The time, in Unix seconds, at which the service graph ended.</p> |
| `insight_id` | String | <p>The insight's unique identifier.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access insight_impact_graph outputs
insight_impact_graph_id = insight_impact_graph.id
insight_impact_graph_next_token = insight_impact_graph.next_token
insight_impact_graph_start_time = insight_impact_graph.start_time
insight_impact_graph_service_graph_start_time = insight_impact_graph.service_graph_start_time
insight_impact_graph_services = insight_impact_graph.services
insight_impact_graph_end_time = insight_impact_graph.end_time
insight_impact_graph_service_graph_end_time = insight_impact_graph.service_graph_end_time
insight_impact_graph_insight_id = insight_impact_graph.insight_id
```

---


### Insight_events

InsightEvents resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>Use this token to retrieve the next page of insight events.</p> |
| `insight_events` | Vec<String> | <p>A detailed description of the event. This includes the time of the event, client and
         root cause impact statistics, and the top anomalous service at the time of the
         event.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access insight_events outputs
insight_events_id = insight_events.id
insight_events_next_token = insight_events.next_token
insight_events_insight_events = insight_events.insight_events
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple encryption_config resources
encryption_config_0 = provider.xray.Encryption_config {
    type = "value-0"
}
encryption_config_1 = provider.xray.Encryption_config {
    type = "value-1"
}
encryption_config_2 = provider.xray.Encryption_config {
    type = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    encryption_config = provider.xray.Encryption_config {
        type = "production-value"
    }
```

---

## Related Documentation

- [AWS Xray Documentation](https://docs.aws.amazon.com/xray/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
