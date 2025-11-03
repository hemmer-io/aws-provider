# Neptune_graph Service



**Resources**: 2

---

## Overview

The neptune_graph service provides access to 2 resource types:

- [Query](#query) [R]
- [Graph_summary](#graph_summary) [R]

---

## Resources


### Query

Query resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `query_string` | String | <p>The query in question.</p> |
| `elapsed` | i64 | <p>The number of milliseconds the query has been running.</p> |
| `state` | String | <p>State of the query.</p> |
| `waited` | i64 | <p>Indicates how long the query waited, in milliseconds.</p> |
| `id` | String | <p>The ID of the query in question.</p> |


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
query_query_string = query.query_string
query_elapsed = query.elapsed
query_state = query.state
query_waited = query.waited
query_id = query.id
```

---


### Graph_summary

GraphSummary resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `version` | String | <p>Display the version of this tool.</p> |
| `graph_summary` | String | <p>The graph summary.</p> |
| `last_statistics_computation_time` | String | <p>The timestamp, in ISO 8601 format, of the time at which Neptune Analytics last computed statistics.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access graph_summary outputs
graph_summary_id = graph_summary.id
graph_summary_version = graph_summary.version
graph_summary_graph_summary = graph_summary.graph_summary
graph_summary_last_statistics_computation_time = graph_summary.last_statistics_computation_time
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple query resources
query_0 = provider.neptune_graph.Query {
}
query_1 = provider.neptune_graph.Query {
}
query_2 = provider.neptune_graph.Query {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    query = provider.neptune_graph.Query {
    }
```

---

## Related Documentation

- [AWS Neptune_graph Documentation](https://docs.aws.amazon.com/neptune_graph/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
