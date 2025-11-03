# Entityresolution Service



**Resources**: 10

---

## Overview

The entityresolution service provides access to 10 resource types:

- [Provider_service](#provider_service) [R]
- [Matching_workflow](#matching_workflow) [CRUD]
- [Id_mapping_job](#id_mapping_job) [R]
- [Id_mapping_workflow](#id_mapping_workflow) [CRUD]
- [Schema_mapping](#schema_mapping) [CRUD]
- [Policy_statement](#policy_statement) [D]
- [Id_namespace](#id_namespace) [CRUD]
- [Match_id](#match_id) [R]
- [Matching_job](#matching_job) [R]
- [Policy](#policy) [CR]

---

## Resources


### Provider_service

ProviderService resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `provider_service_name` | String | <p>The name of the product that the provider service provides. </p> |
| `provider_name` | String | <p>The name of the provider. This name is typically the company name.</p> |
| `provider_endpoint_configuration` | String | <p>The required configuration fields to use with the provider service.</p> |
| `provider_service_display_name` | String | <p>The display name of the provider service.</p> |
| `provider_service_type` | String | <p>The type of provider service.</p> |
| `provider_configuration_definition` | String | <p>The definition of the provider configuration.</p> |
| `provider_id_name_space_configuration` | String | <p>The provider configuration required for different ID namespace types.</p> |
| `provider_entity_output_definition` | String | <p>The definition of the provider entity output.</p> |
| `provider_intermediate_data_access_configuration` | String | <p>The Amazon Web Services accounts and the S3 permissions that are required by some providers to create an S3 bucket for intermediate data storage.</p> |
| `provider_component_schema` | String | <p>Input schema for the provider service.</p> |
| `anonymized_output` | bool | <p>Specifies whether output data from the provider is anonymized. A value of <code>TRUE</code> means the output will be anonymized and you can't relate the data that comes back from the provider to the identifying input. A value of <code>FALSE</code> means the output won't be anonymized and you can relate the data that comes back from the provider to your source data. </p> |
| `provider_service_arn` | String | <p>The ARN (Amazon Resource Name) that Entity Resolution generated for the provider service.</p> |
| `provider_job_configuration` | String | <p>Provider service job configurations.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access provider_service outputs
provider_service_id = provider_service.id
provider_service_provider_service_name = provider_service.provider_service_name
provider_service_provider_name = provider_service.provider_name
provider_service_provider_endpoint_configuration = provider_service.provider_endpoint_configuration
provider_service_provider_service_display_name = provider_service.provider_service_display_name
provider_service_provider_service_type = provider_service.provider_service_type
provider_service_provider_configuration_definition = provider_service.provider_configuration_definition
provider_service_provider_id_name_space_configuration = provider_service.provider_id_name_space_configuration
provider_service_provider_entity_output_definition = provider_service.provider_entity_output_definition
provider_service_provider_intermediate_data_access_configuration = provider_service.provider_intermediate_data_access_configuration
provider_service_provider_component_schema = provider_service.provider_component_schema
provider_service_anonymized_output = provider_service.anonymized_output
provider_service_provider_service_arn = provider_service.provider_service_arn
provider_service_provider_job_configuration = provider_service.provider_job_configuration
```

---


### Matching_workflow

MatchingWorkflow resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resolution_techniques` | String | ✅ | <p>An object which defines the <code>resolutionType</code> and the <code>ruleBasedProperties</code>.</p> |
| `output_source_config` | Vec<String> | ✅ | <p>A list of <code>OutputSource</code> objects, each of which contains fields <code>outputS3Path</code>, <code>applyNormalization</code>, <code>KMSArn</code>, and <code>output</code>.</p> |
| `input_source_config` | Vec<String> | ✅ | <p>A list of <code>InputSource</code> objects, which have the fields <code>InputSourceARN</code> and <code>SchemaName</code>.</p> |
| `workflow_name` | String | ✅ | <p>The name of the workflow. There can't be multiple <code>MatchingWorkflows</code> with the same name.</p> |
| `incremental_run_config` | String |  | <p>Optional. An object that defines the incremental run type. This object contains only the <code>incrementalRunType</code> field, which appears as "Automatic" in the console. </p> <important> <p>For workflows where <code>resolutionType</code> is <code>ML_MATCHING</code> or <code>PROVIDER</code>, incremental processing is not supported. </p> </important> |
| `role_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the IAM role. Entity Resolution assumes this role to create resources on your behalf as part of workflow execution.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags used to organize, track, or control access for this resource.</p> |
| `description` | String |  | <p>A description of the workflow.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `created_at` | String | <p>The timestamp of when the workflow was created.</p> |
| `incremental_run_config` | String | <p>An object which defines an incremental run type and has only <code>incrementalRunType</code> as a field.</p> |
| `updated_at` | String | <p>The timestamp of when the workflow was last updated.</p> |
| `resolution_techniques` | String | <p>An object which defines the <code>resolutionType</code> and the <code>ruleBasedProperties</code>.</p> |
| `workflow_name` | String | <p>The name of the workflow.</p> |
| `workflow_arn` | String | <p>The ARN (Amazon Resource Name) that Entity Resolution generated for the <code>MatchingWorkflow</code>.</p> |
| `description` | String | <p>A description of the workflow.</p> |
| `output_source_config` | Vec<String> | <p>A list of <code>OutputSource</code> objects, each of which contains fields <code>outputS3Path</code>, <code>applyNormalization</code>, <code>KMSArn</code>, and <code>output</code>.</p> |
| `role_arn` | String | <p>The Amazon Resource Name (ARN) of the IAM role. Entity Resolution assumes this role to access Amazon Web Services resources on your behalf.</p> |
| `tags` | HashMap<String, String> | <p>The tags used to organize, track, or control access for this resource.</p> |
| `input_source_config` | Vec<String> | <p>A list of <code>InputSource</code> objects, which have the fields <code>InputSourceARN</code> and <code>SchemaName</code>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create matching_workflow
matching_workflow = provider.entityresolution.Matching_workflow {
    resolution_techniques = "value"  # <p>An object which defines the <code>resolutionType</code> and the <code>ruleBasedProperties</code>.</p>
    output_source_config = "value"  # <p>A list of <code>OutputSource</code> objects, each of which contains fields <code>outputS3Path</code>, <code>applyNormalization</code>, <code>KMSArn</code>, and <code>output</code>.</p>
    input_source_config = "value"  # <p>A list of <code>InputSource</code> objects, which have the fields <code>InputSourceARN</code> and <code>SchemaName</code>.</p>
    workflow_name = "value"  # <p>The name of the workflow. There can't be multiple <code>MatchingWorkflows</code> with the same name.</p>
    role_arn = "value"  # <p>The Amazon Resource Name (ARN) of the IAM role. Entity Resolution assumes this role to create resources on your behalf as part of workflow execution.</p>
}

# Access matching_workflow outputs
matching_workflow_id = matching_workflow.id
matching_workflow_created_at = matching_workflow.created_at
matching_workflow_incremental_run_config = matching_workflow.incremental_run_config
matching_workflow_updated_at = matching_workflow.updated_at
matching_workflow_resolution_techniques = matching_workflow.resolution_techniques
matching_workflow_workflow_name = matching_workflow.workflow_name
matching_workflow_workflow_arn = matching_workflow.workflow_arn
matching_workflow_description = matching_workflow.description
matching_workflow_output_source_config = matching_workflow.output_source_config
matching_workflow_role_arn = matching_workflow.role_arn
matching_workflow_tags = matching_workflow.tags
matching_workflow_input_source_config = matching_workflow.input_source_config
```

---


### Id_mapping_job

IdMappingJob resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `job_id` | String | <p>The ID of the job.</p> |
| `error_details` | String |  |
| `status` | String | <p>The current status of the job.</p> |
| `start_time` | String | <p>The time at which the job was started.</p> |
| `output_source_config` | Vec<String> | <p>A list of <code>OutputSource</code> objects.</p> |
| `metrics` | String | <p>Metrics associated with the execution, specifically total records processed, unique IDs generated, and records the execution skipped.</p> |
| `job_type` | String | <p> The job type of the ID mapping job.</p> <p>A value of <code>INCREMENTAL</code> indicates that only new or changed data was processed since the last job run. This is the default job type if the workflow was created with an <code>incrementalRunConfig</code>.</p> <p>A value of <code>BATCH</code> indicates that all data was processed from the input source, regardless of previous job runs. This is the default job type if the workflow wasn't created with an <code>incrementalRunConfig</code>.</p> <p>A value of <code>DELETE_ONLY</code> indicates that only deletion requests from <code>BatchDeleteUniqueIds</code> were processed.</p> |
| `end_time` | String | <p>The time at which the job has finished.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access id_mapping_job outputs
id_mapping_job_id = id_mapping_job.id
id_mapping_job_job_id = id_mapping_job.job_id
id_mapping_job_error_details = id_mapping_job.error_details
id_mapping_job_status = id_mapping_job.status
id_mapping_job_start_time = id_mapping_job.start_time
id_mapping_job_output_source_config = id_mapping_job.output_source_config
id_mapping_job_metrics = id_mapping_job.metrics
id_mapping_job_job_type = id_mapping_job.job_type
id_mapping_job_end_time = id_mapping_job.end_time
```

---


### Id_mapping_workflow

IdMappingWorkflow resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `incremental_run_config` | String |  | <p> The incremental run configuration for the ID mapping workflow.</p> |
| `description` | String |  | <p>A description of the workflow.</p> |
| `input_source_config` | Vec<String> | ✅ | <p>A list of <code>InputSource</code> objects, which have the fields <code>InputSourceARN</code> and <code>SchemaName</code>.</p> |
| `workflow_name` | String | ✅ | <p>The name of the workflow. There can't be multiple <code>IdMappingWorkflows</code> with the same name.</p> |
| `output_source_config` | Vec<String> |  | <p>A list of <code>IdMappingWorkflowOutputSource</code> objects, each of which contains fields <code>outputS3Path</code> and <code>KMSArn</code>.</p> |
| `id_mapping_techniques` | String | ✅ | <p>An object which defines the ID mapping technique and any additional configurations.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags used to organize, track, or control access for this resource.</p> |
| `role_arn` | String |  | <p>The Amazon Resource Name (ARN) of the IAM role. Entity Resolution assumes this role to create resources on your behalf as part of workflow execution.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `workflow_arn` | String | <p>The ARN (Amazon Resource Name) that Entity Resolution generated for the <code>IdMappingWorkflow</code> .</p> |
| `description` | String | <p>A description of the workflow.</p> |
| `id_mapping_techniques` | String | <p>An object which defines the ID mapping technique and any additional configurations.</p> |
| `tags` | HashMap<String, String> | <p>The tags used to organize, track, or control access for this resource.</p> |
| `updated_at` | String | <p>The timestamp of when the workflow was last updated.</p> |
| `input_source_config` | Vec<String> | <p>A list of <code>InputSource</code> objects, which have the fields <code>InputSourceARN</code> and <code>SchemaName</code>.</p> |
| `output_source_config` | Vec<String> | <p>A list of <code>OutputSource</code> objects, each of which contains fields <code>outputS3Path</code> and <code>KMSArn</code>.</p> |
| `created_at` | String | <p>The timestamp of when the workflow was created.</p> |
| `incremental_run_config` | String | <p> The incremental run configuration for the ID mapping workflow.</p> |
| `role_arn` | String | <p>The Amazon Resource Name (ARN) of the IAM role. Entity Resolution assumes this role to access Amazon Web Services resources on your behalf.</p> |
| `workflow_name` | String | <p>The name of the workflow.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create id_mapping_workflow
id_mapping_workflow = provider.entityresolution.Id_mapping_workflow {
    input_source_config = "value"  # <p>A list of <code>InputSource</code> objects, which have the fields <code>InputSourceARN</code> and <code>SchemaName</code>.</p>
    workflow_name = "value"  # <p>The name of the workflow. There can't be multiple <code>IdMappingWorkflows</code> with the same name.</p>
    id_mapping_techniques = "value"  # <p>An object which defines the ID mapping technique and any additional configurations.</p>
}

# Access id_mapping_workflow outputs
id_mapping_workflow_id = id_mapping_workflow.id
id_mapping_workflow_workflow_arn = id_mapping_workflow.workflow_arn
id_mapping_workflow_description = id_mapping_workflow.description
id_mapping_workflow_id_mapping_techniques = id_mapping_workflow.id_mapping_techniques
id_mapping_workflow_tags = id_mapping_workflow.tags
id_mapping_workflow_updated_at = id_mapping_workflow.updated_at
id_mapping_workflow_input_source_config = id_mapping_workflow.input_source_config
id_mapping_workflow_output_source_config = id_mapping_workflow.output_source_config
id_mapping_workflow_created_at = id_mapping_workflow.created_at
id_mapping_workflow_incremental_run_config = id_mapping_workflow.incremental_run_config
id_mapping_workflow_role_arn = id_mapping_workflow.role_arn
id_mapping_workflow_workflow_name = id_mapping_workflow.workflow_name
```

---


### Schema_mapping

SchemaMapping resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | <p>A description of the schema.</p> |
| `schema_name` | String | ✅ | <p>The name of the schema. There can't be multiple <code>SchemaMappings</code> with the same name.</p> |
| `mapped_input_fields` | Vec<String> | ✅ | <p>A list of <code>MappedInputFields</code>. Each <code>MappedInputField</code> corresponds to a column the source data table, and contains column name plus additional information that Entity Resolution uses for matching.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags used to organize, track, or control access for this resource.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `tags` | HashMap<String, String> | <p>The tags used to organize, track, or control access for this resource.</p> |
| `mapped_input_fields` | Vec<String> | <p>A list of <code>MappedInputFields</code>. Each <code>MappedInputField</code> corresponds to a column the source data table, and contains column name plus additional information Entity Resolution uses for matching.</p> |
| `created_at` | String | <p>The timestamp of when the <code>SchemaMapping</code> was created.</p> |
| `description` | String | <p>A description of the schema.</p> |
| `schema_name` | String | <p>The name of the schema.</p> |
| `updated_at` | String | <p>The timestamp of when the <code>SchemaMapping</code> was last updated.</p> |
| `has_workflows` | bool | <p>Specifies whether the schema mapping has been applied to a workflow.</p> |
| `schema_arn` | String | <p>The ARN (Amazon Resource Name) that Entity Resolution generated for the SchemaMapping.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create schema_mapping
schema_mapping = provider.entityresolution.Schema_mapping {
    schema_name = "value"  # <p>The name of the schema. There can't be multiple <code>SchemaMappings</code> with the same name.</p>
    mapped_input_fields = "value"  # <p>A list of <code>MappedInputFields</code>. Each <code>MappedInputField</code> corresponds to a column the source data table, and contains column name plus additional information that Entity Resolution uses for matching.</p>
}

# Access schema_mapping outputs
schema_mapping_id = schema_mapping.id
schema_mapping_tags = schema_mapping.tags
schema_mapping_mapped_input_fields = schema_mapping.mapped_input_fields
schema_mapping_created_at = schema_mapping.created_at
schema_mapping_description = schema_mapping.description
schema_mapping_schema_name = schema_mapping.schema_name
schema_mapping_updated_at = schema_mapping.updated_at
schema_mapping_has_workflows = schema_mapping.has_workflows
schema_mapping_schema_arn = schema_mapping.schema_arn
```

---


### Policy_statement

PolicyStatement resource

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


### Id_namespace

IdNamespace resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `input_source_config` | Vec<String> |  | <p>A list of <code>InputSource</code> objects, which have the fields <code>InputSourceARN</code> and <code>SchemaName</code>.</p> |
| `id_namespace_name` | String | ✅ | <p>The name of the ID namespace.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags used to organize, track, or control access for this resource.</p> |
| `description` | String |  | <p>The description of the ID namespace.</p> |
| `id_mapping_workflow_properties` | Vec<String> |  | <p>Determines the properties of <code>IdMappingWorflow</code> where this <code>IdNamespace</code> can be used as a <code>Source</code> or a <code>Target</code>.</p> |
| `type` | String | ✅ | <p>The type of ID namespace. There are two types: <code>SOURCE</code> and <code>TARGET</code>. </p> <p>The <code>SOURCE</code> contains configurations for <code>sourceId</code> data that will be processed in an ID mapping workflow. </p> <p>The <code>TARGET</code> contains a configuration of <code>targetId</code> to which all <code>sourceIds</code> will resolve to.</p> |
| `role_arn` | String |  | <p>The Amazon Resource Name (ARN) of the IAM role. Entity Resolution assumes this role to access the resources defined in this <code>IdNamespace</code> on your behalf as part of the workflow run.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `tags` | HashMap<String, String> | <p>The tags used to organize, track, or control access for this resource.</p> |
| `input_source_config` | Vec<String> | <p>A list of <code>InputSource</code> objects, which have the fields <code>InputSourceARN</code> and <code>SchemaName</code>.</p> |
| `description` | String | <p>The description of the ID namespace.</p> |
| `created_at` | String | <p>The timestamp of when the ID namespace was created.</p> |
| `updated_at` | String | <p>The timestamp of when the ID namespace was last updated.</p> |
| `role_arn` | String | <p>The Amazon Resource Name (ARN) of the IAM role. Entity Resolution assumes this role to access the resources defined in this <code>IdNamespace</code> on your behalf as part of a workflow run.</p> |
| `id_namespace_name` | String | <p>The name of the ID namespace.</p> |
| `id_namespace_arn` | String | <p>The Amazon Resource Name (ARN) of the ID namespace.</p> |
| `id_mapping_workflow_properties` | Vec<String> | <p>Determines the properties of <code>IdMappingWorkflow</code> where this <code>IdNamespace</code> can be used as a <code>Source</code> or a <code>Target</code>.</p> |
| `type` | String | <p>The type of ID namespace. There are two types: <code>SOURCE</code> and <code>TARGET</code>.</p> <p>The <code>SOURCE</code> contains configurations for <code>sourceId</code> data that will be processed in an ID mapping workflow. </p> <p>The <code>TARGET</code> contains a configuration of <code>targetId</code> to which all <code>sourceIds</code> will resolve to.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create id_namespace
id_namespace = provider.entityresolution.Id_namespace {
    id_namespace_name = "value"  # <p>The name of the ID namespace.</p>
    type = "value"  # <p>The type of ID namespace. There are two types: <code>SOURCE</code> and <code>TARGET</code>. </p> <p>The <code>SOURCE</code> contains configurations for <code>sourceId</code> data that will be processed in an ID mapping workflow. </p> <p>The <code>TARGET</code> contains a configuration of <code>targetId</code> to which all <code>sourceIds</code> will resolve to.</p>
}

# Access id_namespace outputs
id_namespace_id = id_namespace.id
id_namespace_tags = id_namespace.tags
id_namespace_input_source_config = id_namespace.input_source_config
id_namespace_description = id_namespace.description
id_namespace_created_at = id_namespace.created_at
id_namespace_updated_at = id_namespace.updated_at
id_namespace_role_arn = id_namespace.role_arn
id_namespace_id_namespace_name = id_namespace.id_namespace_name
id_namespace_id_namespace_arn = id_namespace.id_namespace_arn
id_namespace_id_mapping_workflow_properties = id_namespace.id_mapping_workflow_properties
id_namespace_type = id_namespace.type
```

---


### Match_id

MatchId resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `match_id` | String | <p>The unique identifiers for this group of match records.</p> |
| `match_rule` | String | <p>The rule the record matched on.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access match_id outputs
match_id_id = match_id.id
match_id_match_id = match_id.match_id
match_id_match_rule = match_id.match_rule
```

---


### Matching_job

MatchingJob resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `end_time` | String | <p>The time at which the job has finished.</p> |
| `start_time` | String | <p>The time at which the job was started.</p> |
| `metrics` | String | <p>Metrics associated with the execution, specifically total records processed, unique IDs generated, and records the execution skipped.</p> |
| `error_details` | String | <p>An object containing an error message, if there was an error.</p> |
| `output_source_config` | Vec<String> | <p>A list of <code>OutputSource</code> objects.</p> |
| `job_id` | String | <p>The unique identifier of the matching job.</p> |
| `status` | String | <p>The current status of the job.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access matching_job outputs
matching_job_id = matching_job.id
matching_job_end_time = matching_job.end_time
matching_job_start_time = matching_job.start_time
matching_job_metrics = matching_job.metrics
matching_job_error_details = matching_job.error_details
matching_job_output_source_config = matching_job.output_source_config
matching_job_job_id = matching_job.job_id
matching_job_status = matching_job.status
```

---


### Policy

Policy resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `token` | String |  | <p>A unique identifier for the current revision of the policy.</p> |
| `policy` | String | ✅ | <p>The resource-based policy.</p> <important> <p>If you set the value of the <code>effect</code> parameter in the <code>policy</code> to <code>Deny</code> for the <code>PutPolicy</code> operation, you must also set the value of the <code>effect</code> parameter to <code>Deny</code> for the <code>AddPolicyStatement</code> operation.</p> </important> |
| `arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the resource for which the policy needs to be updated.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `arn` | String | <p>The Entity Resolution resource ARN.</p> |
| `policy` | String | <p>The resource-based policy.</p> |
| `token` | String | <p>A unique identifier for the current revision of the policy.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create policy
policy = provider.entityresolution.Policy {
    policy = "value"  # <p>The resource-based policy.</p> <important> <p>If you set the value of the <code>effect</code> parameter in the <code>policy</code> to <code>Deny</code> for the <code>PutPolicy</code> operation, you must also set the value of the <code>effect</code> parameter to <code>Deny</code> for the <code>AddPolicyStatement</code> operation.</p> </important>
    arn = "value"  # <p>The Amazon Resource Name (ARN) of the resource for which the policy needs to be updated.</p>
}

# Access policy outputs
policy_id = policy.id
policy_arn = policy.arn
policy_policy = policy.policy
policy_token = policy.token
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple provider_service resources
provider_service_0 = provider.entityresolution.Provider_service {
}
provider_service_1 = provider.entityresolution.Provider_service {
}
provider_service_2 = provider.entityresolution.Provider_service {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    provider_service = provider.entityresolution.Provider_service {
    }
```

---

## Related Documentation

- [AWS Entityresolution Documentation](https://docs.aws.amazon.com/entityresolution/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
