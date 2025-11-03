# Pi Service



**Resources**: 5

---

## Overview

The pi service provides access to 5 resource types:

- [Dimension_keys](#dimension_keys) [R]
- [Dimension_key_details](#dimension_key_details) [R]
- [Resource_metrics](#resource_metrics) [R]
- [Performance_analysis_report](#performance_analysis_report) [CRD]
- [Resource_metadata](#resource_metadata) [R]

---

## Resources


### Dimension_keys

DimensionKeys resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `aligned_end_time` | String | <p>The end time for the returned dimension keys, after alignment to a granular boundary (as specified by <code>PeriodInSeconds</code>).
            <code>AlignedEndTime</code> will be greater than or equal to the value of the user-specified <code>Endtime</code>. </p> |
| `keys` | Vec<String> | <p>The dimension keys that were requested.</p> |
| `aligned_start_time` | String | <p>The start time for the returned dimension keys, after alignment to a granular boundary (as specified by <code>PeriodInSeconds</code>).
            <code>AlignedStartTime</code> will be less than or equal to the value of the user-specified <code>StartTime</code>. </p> |
| `next_token` | String | <p>A pagination token that indicates the response didn’t return all available records because <code>MaxRecords</code> was specified in the
            previous request. To get the remaining records, specify <code>NextToken</code> in a separate request with this value. </p> |
| `partition_keys` | Vec<String> | <p>If <code>PartitionBy</code> was present in the request, <code>PartitionKeys</code> contains the breakdown of dimension keys by the
            specified partitions. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access dimension_keys outputs
dimension_keys_id = dimension_keys.id
dimension_keys_aligned_end_time = dimension_keys.aligned_end_time
dimension_keys_keys = dimension_keys.keys
dimension_keys_aligned_start_time = dimension_keys.aligned_start_time
dimension_keys_next_token = dimension_keys.next_token
dimension_keys_partition_keys = dimension_keys.partition_keys
```

---


### Dimension_key_details

DimensionKeyDetails resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `dimensions` | Vec<String> | <p>The details for the requested dimensions.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access dimension_key_details outputs
dimension_key_details_id = dimension_key_details.id
dimension_key_details_dimensions = dimension_key_details.dimensions
```

---


### Resource_metrics

ResourceMetrics resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `aligned_end_time` | String | <p>The end time for the returned metrics, after alignment to a granular boundary (as specified by <code>PeriodInSeconds</code>).
                <code>AlignedEndTime</code> will be greater than or equal to the value of the user-specified <code>Endtime</code>.</p> |
| `identifier` | String | <p>An immutable identifier for a data source that is unique for an Amazon Web Services Region. Performance Insights gathers metrics from this data source. In the
            console, the identifier is shown as <i>ResourceID</i>. When you call <code>DescribeDBInstances</code>, the identifier is
            returned as <code>DbiResourceId</code>.</p> |
| `metric_list` | Vec<String> | <p>An array of metric results, where each array element contains all of the data points for a 
          particular dimension.</p> |
| `aligned_start_time` | String | <p>The start time for the returned metrics, after alignment to a granular boundary (as specified by <code>PeriodInSeconds</code>).
                <code>AlignedStartTime</code> will be less than or equal to the value of the user-specified <code>StartTime</code>.</p> |
| `next_token` | String | <p>An optional pagination token provided by a previous request. If this parameter is specified, 
          the response includes only records beyond the token, up to the value specified by <code>MaxRecords</code>.
      </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access resource_metrics outputs
resource_metrics_id = resource_metrics.id
resource_metrics_aligned_end_time = resource_metrics.aligned_end_time
resource_metrics_identifier = resource_metrics.identifier
resource_metrics_metric_list = resource_metrics.metric_list
resource_metrics_aligned_start_time = resource_metrics.aligned_start_time
resource_metrics_next_token = resource_metrics.next_token
```

---


### Performance_analysis_report

PerformanceAnalysisReport resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `end_time` | String | ✅ | <p>The end time defined for the analysis report.</p> |
| `identifier` | String | ✅ | <p>An immutable, Amazon Web Services Region-unique identifier for a data source. Performance Insights gathers metrics from
            this data source.</p>
         <p>To use an Amazon RDS instance as a data source, you specify its <code>DbiResourceId</code> value. 
            For example, specify <code>db-ADECBTYHKTSAUMUZQYPDS2GW4A</code>.</p> |
| `start_time` | String | ✅ | <p>The start time defined for the analysis report.</p> |
| `service_type` | String | ✅ | <p>The Amazon Web Services service for which Performance Insights will return metrics. Valid value is <code>RDS</code>.</p> |
| `tags` | Vec<String> |  | <p>The metadata assigned to the analysis report consisting of a key-value pair.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `analysis_report` | String | <p>The summary of the performance analysis report created for a time period.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create performance_analysis_report
performance_analysis_report = provider.pi.Performance_analysis_report {
    end_time = "value"  # <p>The end time defined for the analysis report.</p>
    identifier = "value"  # <p>An immutable, Amazon Web Services Region-unique identifier for a data source. Performance Insights gathers metrics from
            this data source.</p>
         <p>To use an Amazon RDS instance as a data source, you specify its <code>DbiResourceId</code> value. 
            For example, specify <code>db-ADECBTYHKTSAUMUZQYPDS2GW4A</code>.</p>
    start_time = "value"  # <p>The start time defined for the analysis report.</p>
    service_type = "value"  # <p>The Amazon Web Services service for which Performance Insights will return metrics. Valid value is <code>RDS</code>.</p>
}

# Access performance_analysis_report outputs
performance_analysis_report_id = performance_analysis_report.id
performance_analysis_report_analysis_report = performance_analysis_report.analysis_report
```

---


### Resource_metadata

ResourceMetadata resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `features` | HashMap<String, String> | <p>The metadata for different features. For example, the metadata might indicate that a feature is
            turned on or off on a specific DB instance.</p> |
| `identifier` | String | <p>An immutable identifier for a data source that is unique for an Amazon Web Services Region. 
      
      Performance Insights gathers metrics from this data source. To use a DB instance as a data source, 
            specify its <code>DbiResourceId</code> value. For example, specify <code>db-ABCDEFGHIJKLMNOPQRSTU1VW2X</code>.
        </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access resource_metadata outputs
resource_metadata_id = resource_metadata.id
resource_metadata_features = resource_metadata.features
resource_metadata_identifier = resource_metadata.identifier
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple dimension_keys resources
dimension_keys_0 = provider.pi.Dimension_keys {
}
dimension_keys_1 = provider.pi.Dimension_keys {
}
dimension_keys_2 = provider.pi.Dimension_keys {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    dimension_keys = provider.pi.Dimension_keys {
    }
```

---

## Related Documentation

- [AWS Pi Documentation](https://docs.aws.amazon.com/pi/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
