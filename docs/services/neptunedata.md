# Neptunedata Service



**Resources**: 14

---

## Overview

The neptunedata service provides access to 14 resource types:

- [Loader_job_status](#loader_job_status) [R]
- [Ml_data_processing_job](#ml_data_processing_job) [R]
- [Gremlin_query_status](#gremlin_query_status) [R]
- [Sparql_statistics](#sparql_statistics) [RD]
- [Propertygraph_summary](#propertygraph_summary) [R]
- [Propertygraph_statistics](#propertygraph_statistics) [RD]
- [Ml_model_training_job](#ml_model_training_job) [R]
- [Propertygraph_stream](#propertygraph_stream) [R]
- [Open_cypher_query_status](#open_cypher_query_status) [R]
- [Ml_model_transform_job](#ml_model_transform_job) [R]
- [Sparql_stream](#sparql_stream) [R]
- [Rdf_graph_summary](#rdf_graph_summary) [R]
- [Ml_endpoint](#ml_endpoint) [CRD]
- [Engine_status](#engine_status) [R]

---

## Resources


### Loader_job_status

LoaderJobStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `payload` | String | <p>Status information about the load job, in a layout that could look like this:</p> |
| `status` | String | <p>The HTTP response code for the request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access loader_job_status outputs
loader_job_status_id = loader_job_status.id
loader_job_status_payload = loader_job_status.payload
loader_job_status_status = loader_job_status.status
```

---


### Ml_data_processing_job

MLDataProcessingJob resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `processing_job` | String | <p>Definition of the data processing job.</p> |
| `status` | String | <p>Status of the data processing job.</p> |
| `id` | String | <p>The unique identifier of this data-processing job.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access ml_data_processing_job outputs
ml_data_processing_job_id = ml_data_processing_job.id
ml_data_processing_job_processing_job = ml_data_processing_job.processing_job
ml_data_processing_job_status = ml_data_processing_job.status
ml_data_processing_job_id = ml_data_processing_job.id
```

---


### Gremlin_query_status

GremlinQueryStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `query_id` | String | <p>The ID of the query for which status is being returned.</p> |
| `query_eval_stats` | String | <p>The evaluation status of the Gremlin query.</p> |
| `query_string` | String | <p>The Gremlin query string.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access gremlin_query_status outputs
gremlin_query_status_id = gremlin_query_status.id
gremlin_query_status_query_id = gremlin_query_status.query_id
gremlin_query_status_query_eval_stats = gremlin_query_status.query_eval_stats
gremlin_query_status_query_string = gremlin_query_status.query_string
```

---


### Sparql_statistics

SparqlStatistics resource

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | String | <p>The HTTP return code of the request. If the request succeeded, the code is 200. See <a href="https://docs.aws.amazon.com/neptune/latest/userguide/neptune-dfe-statistics.html#neptune-dfe-statistics-errors">Common error codes for DFE statistics request</a> for a list of common errors.</p> <p>When invoking this operation in a Neptune cluster that has IAM authentication enabled, the IAM user or role making the request must have a policy attached that allows the <a href="https://docs.aws.amazon.com/neptune/latest/userguide/iam-dp-actions.html#getstatisticsstatus">neptune-db:GetStatisticsStatus</a> IAM action in that cluster.</p> |
| `payload` | String | <p>Statistics for RDF data.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access sparql_statistics outputs
sparql_statistics_id = sparql_statistics.id
sparql_statistics_status = sparql_statistics.status
sparql_statistics_payload = sparql_statistics.payload
```

---


### Propertygraph_summary

PropertygraphSummary resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status_code` | i64 | <p>The HTTP return code of the request. If the request succeeded, the code is 200.</p> |
| `payload` | String | <p>Payload containing the property graph summary response.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access propertygraph_summary outputs
propertygraph_summary_id = propertygraph_summary.id
propertygraph_summary_status_code = propertygraph_summary.status_code
propertygraph_summary_payload = propertygraph_summary.payload
```

---


### Propertygraph_statistics

PropertygraphStatistics resource

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `payload` | String | <p>Statistics for property-graph data.</p> |
| `status` | String | <p>The HTTP return code of the request. If the request succeeded, the code is 200. See <a href="https://docs.aws.amazon.com/neptune/latest/userguide/neptune-dfe-statistics.html#neptune-dfe-statistics-errors">Common error codes for DFE statistics request</a> for a list of common errors.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access propertygraph_statistics outputs
propertygraph_statistics_id = propertygraph_statistics.id
propertygraph_statistics_payload = propertygraph_statistics.payload
propertygraph_statistics_status = propertygraph_statistics.status
```

---


### Ml_model_training_job

MLModelTrainingJob resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | String | <p>The status of the model training job.</p> |
| `model_transform_job` | String | <p>The model transform job.</p> |
| `ml_models` | Vec<String> | <p>A list of the configurations of the ML models being used.</p> |
| `id` | String | <p>The unique identifier of this model-training job.</p> |
| `processing_job` | String | <p>The data processing job.</p> |
| `hpo_job` | String | <p>The HPO job.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access ml_model_training_job outputs
ml_model_training_job_id = ml_model_training_job.id
ml_model_training_job_status = ml_model_training_job.status
ml_model_training_job_model_transform_job = ml_model_training_job.model_transform_job
ml_model_training_job_ml_models = ml_model_training_job.ml_models
ml_model_training_job_id = ml_model_training_job.id
ml_model_training_job_processing_job = ml_model_training_job.processing_job
ml_model_training_job_hpo_job = ml_model_training_job.hpo_job
```

---


### Propertygraph_stream

PropertygraphStream resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `total_records` | i64 | <p>The total number of records in the response.</p> |
| `format` | String | <p>Serialization format for the change records being returned. Currently, the only supported value is <code>PG_JSON</code>.</p> |
| `last_event_id` | String | <p>Sequence identifier of the last change in the stream response.</p> <p>An event ID is composed of two fields: a <code>commitNum</code>, which identifies a transaction that changed the graph, and an <code>opNum</code>, which identifies a specific operation within that transaction:</p> |
| `last_trx_timestamp_in_millis` | i64 | <p>The time at which the commit for the transaction was requested, in milliseconds from the Unix epoch.</p> |
| `records` | Vec<String> | <p>An array of serialized change-log stream records included in the response.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access propertygraph_stream outputs
propertygraph_stream_id = propertygraph_stream.id
propertygraph_stream_total_records = propertygraph_stream.total_records
propertygraph_stream_format = propertygraph_stream.format
propertygraph_stream_last_event_id = propertygraph_stream.last_event_id
propertygraph_stream_last_trx_timestamp_in_millis = propertygraph_stream.last_trx_timestamp_in_millis
propertygraph_stream_records = propertygraph_stream.records
```

---


### Open_cypher_query_status

OpenCypherQueryStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `query_string` | String | <p>The openCypher query string.</p> |
| `query_id` | String | <p>The unique ID of the query for which status is being returned.</p> |
| `query_eval_stats` | String | <p>The openCypher query evaluation status.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access open_cypher_query_status outputs
open_cypher_query_status_id = open_cypher_query_status.id
open_cypher_query_status_query_string = open_cypher_query_status.query_string
open_cypher_query_status_query_id = open_cypher_query_status.query_id
open_cypher_query_status_query_eval_stats = open_cypher_query_status.query_eval_stats
```

---


### Ml_model_transform_job

MLModelTransformJob resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | String | <p>The status of the model-transform job.</p> |
| `remote_model_transform_job` | String | <p>The remote model transform job.</p> |
| `models` | Vec<String> | <p>A list of the configuration information for the models being used.</p> |
| `base_processing_job` | String | <p>The base data processing job.</p> |
| `id` | String | <p>The unique identifier of the model-transform job to be retrieved.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access ml_model_transform_job outputs
ml_model_transform_job_id = ml_model_transform_job.id
ml_model_transform_job_status = ml_model_transform_job.status
ml_model_transform_job_remote_model_transform_job = ml_model_transform_job.remote_model_transform_job
ml_model_transform_job_models = ml_model_transform_job.models
ml_model_transform_job_base_processing_job = ml_model_transform_job.base_processing_job
ml_model_transform_job_id = ml_model_transform_job.id
```

---


### Sparql_stream

SparqlStream resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `records` | Vec<String> | <p>An array of serialized change-log stream records included in the response.</p> |
| `last_event_id` | String | <p>Sequence identifier of the last change in the stream response.</p> <p>An event ID is composed of two fields: a <code>commitNum</code>, which identifies a transaction that changed the graph, and an <code>opNum</code>, which identifies a specific operation within that transaction:</p> |
| `total_records` | i64 | <p>The total number of records in the response.</p> |
| `format` | String | <p>Serialization format for the change records being returned. Currently, the only supported value is <code>NQUADS</code>.</p> |
| `last_trx_timestamp_in_millis` | i64 | <p>The time at which the commit for the transaction was requested, in milliseconds from the Unix epoch.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access sparql_stream outputs
sparql_stream_id = sparql_stream.id
sparql_stream_records = sparql_stream.records
sparql_stream_last_event_id = sparql_stream.last_event_id
sparql_stream_total_records = sparql_stream.total_records
sparql_stream_format = sparql_stream.format
sparql_stream_last_trx_timestamp_in_millis = sparql_stream.last_trx_timestamp_in_millis
```

---


### Rdf_graph_summary

RDFGraphSummary resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status_code` | i64 | <p>The HTTP return code of the request. If the request succeeded, the code is 200.</p> |
| `payload` | String | <p>Payload for an RDF graph summary response</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access rdf_graph_summary outputs
rdf_graph_summary_id = rdf_graph_summary.id
rdf_graph_summary_status_code = rdf_graph_summary.status_code
rdf_graph_summary_payload = rdf_graph_summary.payload
```

---


### Ml_endpoint

MLEndpoint resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `model_name` | String |  | <p>Model type for training. By default the Neptune ML model is automatically based on the <code>modelType</code> used in data processing, but you can specify a different model type here. The default is <code>rgcn</code> for heterogeneous graphs and <code>kge</code> for knowledge graphs. The only valid value for heterogeneous graphs is <code>rgcn</code>. Valid values for knowledge graphs are: <code>kge</code>, <code>transe</code>, <code>distmult</code>, and <code>rotate</code>.</p> |
| `ml_model_training_job_id` | String |  | <p>The job Id of the completed model-training job that has created the model that the inference endpoint will point to. You must supply either the <code>mlModelTrainingJobId</code> or the <code>mlModelTransformJobId</code>.</p> |
| `ml_model_transform_job_id` | String |  | <p>The job Id of the completed model-transform job. You must supply either the <code>mlModelTrainingJobId</code> or the <code>mlModelTransformJobId</code>.</p> |
| `volume_encryption_kms_key` | String |  | <p>The Amazon Key Management Service (Amazon KMS) key that SageMaker uses to encrypt data on the storage volume attached to the ML compute instances that run the training job. The default is None.</p> |
| `id` | String |  | <p>A unique identifier for the new inference endpoint. The default is an autogenerated timestamped name.</p> |
| `instance_type` | String |  | <p>The type of Neptune ML instance to use for online servicing. The default is <code>ml.m5.xlarge</code>. Choosing the ML instance for an inference endpoint depends on the task type, the graph size, and your budget.</p> |
| `instance_count` | i64 |  | <p>The minimum number of Amazon EC2 instances to deploy to an endpoint for prediction. The default is 1</p> |
| `neptune_iam_role_arn` | String |  | <p>The ARN of an IAM role providing Neptune access to SageMaker and Amazon S3 resources. This must be listed in your DB cluster parameter group or an error will be thrown.</p> |
| `update` | bool |  | <p>If set to <code>true</code>, <code>update</code> indicates that this is an update request. The default is <code>false</code>. You must supply either the <code>mlModelTrainingJobId</code> or the <code>mlModelTransformJobId</code>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `endpoint` | String | <p>The endpoint definition.</p> |
| `id` | String | <p>The unique identifier of the inference endpoint.</p> |
| `endpoint_config` | String | <p>The endpoint configuration</p> |
| `status` | String | <p>The status of the inference endpoint.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create ml_endpoint
ml_endpoint = provider.neptunedata.Ml_endpoint {
}

# Access ml_endpoint outputs
ml_endpoint_id = ml_endpoint.id
ml_endpoint_endpoint = ml_endpoint.endpoint
ml_endpoint_id = ml_endpoint.id
ml_endpoint_endpoint_config = ml_endpoint.endpoint_config
ml_endpoint_status = ml_endpoint.status
```

---


### Engine_status

EngineStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `sparql` | String | <p>Contains information about the SPARQL query language available on your cluster. Specifically, it contains a version field that specifies the current SPARQL version being used by the engine.</p> |
| `start_time` | String | <p>Set to the UTC time at which the current server process started.</p> |
| `db_engine_version` | String | <p>Set to the Neptune engine version running on your DB cluster. If this engine version has been manually patched since it was released, the version number is prefixed by <code>Patch-</code>.</p> |
| `role` | String | <p>Set to <code>reader</code> if the instance is a read-replica, or to <code>writer</code> if the instance is the primary instance.</p> |
| `lab_mode` | String | <p>Contains Lab Mode settings being used by the engine.</p> |
| `gremlin` | String | <p>Contains information about the Gremlin query language available on your cluster. Specifically, it contains a version field that specifies the current TinkerPop version being used by the engine.</p> |
| `status` | String | <p>Set to <code>healthy</code> if the instance is not experiencing problems. If the instance is recovering from a crash or from being rebooted and there are active transactions running from the latest server shutdown, status is set to <code>recovery</code>.</p> |
| `opencypher` | String | <p>Contains information about the openCypher query language available on your cluster. Specifically, it contains a version field that specifies the current operCypher version being used by the engine.</p> |
| `rolling_back_trx_count` | i64 | <p>If there are transactions being rolled back, this field is set to the number of such transactions. If there are none, the field doesn't appear at all.</p> |
| `features` | HashMap<String, String> | <p>Contains status information about the features enabled on your DB cluster.</p> |
| `settings` | String | <p>Contains information about the current settings on your DB cluster. For example, contains the current cluster query timeout setting (<code>clusterQueryTimeoutInMs</code>).</p> |
| `dfe_query_engine` | String | <p>Set to <code>enabled</code> if the DFE engine is fully enabled, or to <code>viaQueryHint</code> (the default) if the DFE engine is only used with queries that have the <code>useDFE</code> query hint set to <code>true</code>.</p> |
| `rolling_back_trx_earliest_start_time` | String | <p>Set to the start time of the earliest transaction being rolled back. If no transactions are being rolled back, the field doesn't appear at all.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access engine_status outputs
engine_status_id = engine_status.id
engine_status_sparql = engine_status.sparql
engine_status_start_time = engine_status.start_time
engine_status_db_engine_version = engine_status.db_engine_version
engine_status_role = engine_status.role
engine_status_lab_mode = engine_status.lab_mode
engine_status_gremlin = engine_status.gremlin
engine_status_status = engine_status.status
engine_status_opencypher = engine_status.opencypher
engine_status_rolling_back_trx_count = engine_status.rolling_back_trx_count
engine_status_features = engine_status.features
engine_status_settings = engine_status.settings
engine_status_dfe_query_engine = engine_status.dfe_query_engine
engine_status_rolling_back_trx_earliest_start_time = engine_status.rolling_back_trx_earliest_start_time
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple loader_job_status resources
loader_job_status_0 = provider.neptunedata.Loader_job_status {
}
loader_job_status_1 = provider.neptunedata.Loader_job_status {
}
loader_job_status_2 = provider.neptunedata.Loader_job_status {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    loader_job_status = provider.neptunedata.Loader_job_status {
    }
```

---

## Related Documentation

- [AWS Neptunedata Documentation](https://docs.aws.amazon.com/neptunedata/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
