# Glue Service



**Resources**: 91

---

## Overview

The glue service provides access to 91 resource types:

- [Schema](#schema) [CRUD]
- [Mapping](#mapping) [R]
- [Ml_transforms](#ml_transforms) [R]
- [Databases](#databases) [R]
- [Resource_policy](#resource_policy) [CRD]
- [Table_optimizer](#table_optimizer) [CRUD]
- [Entity](#entity) [R]
- [Unfiltered_partition_metadata](#unfiltered_partition_metadata) [R]
- [Job_run](#job_run) [R]
- [Data_catalog_encryption_settings](#data_catalog_encryption_settings) [CR]
- [Integration_resource_property](#integration_resource_property) [CRU]
- [Classifier](#classifier) [CRUD]
- [Crawler](#crawler) [CRUD]
- [Ml_transform](#ml_transform) [CRUD]
- [Dataflow_graph](#dataflow_graph) [R]
- [Table_version](#table_version) [RD]
- [Partition_indexes](#partition_indexes) [R]
- [Column_statistics_for_partition](#column_statistics_for_partition) [RUD]
- [Script](#script) [C]
- [Security_configurations](#security_configurations) [R]
- [Partition](#partition) [CRUD]
- [Column_statistics_for_table](#column_statistics_for_table) [RUD]
- [Data_quality_model_result](#data_quality_model_result) [R]
- [Unfiltered_table_metadata](#unfiltered_table_metadata) [R]
- [Connection](#connection) [CRUD]
- [Session](#session) [CRD]
- [Triggers](#triggers) [R]
- [Crawler_schedule](#crawler_schedule) [U]
- [Database](#database) [CRUD]
- [Integration](#integration) [CD]
- [Partition_index](#partition_index) [CD]
- [Partitions](#partitions) [R]
- [Crawlers](#crawlers) [R]
- [Schema_by_definition](#schema_by_definition) [R]
- [Connection_type](#connection_type) [R]
- [Blueprint_runs](#blueprint_runs) [R]
- [Job_bookmark](#job_bookmark) [R]
- [Job_from_source_control](#job_from_source_control) [U]
- [Security_configuration](#security_configuration) [CRD]
- [Glue_identity_center_configuration](#glue_identity_center_configuration) [CRUD]
- [Catalog](#catalog) [CRUD]
- [Dev_endpoints](#dev_endpoints) [R]
- [Ml_task_runs](#ml_task_runs) [R]
- [Dev_endpoint](#dev_endpoint) [CRUD]
- [Workflow_run_properties](#workflow_run_properties) [CR]
- [Column_statistics_task_runs](#column_statistics_task_runs) [R]
- [Job](#job) [CRUD]
- [Jobs](#jobs) [R]
- [Table_versions](#table_versions) [R]
- [Workflow](#workflow) [CRUD]
- [Data_quality_ruleset](#data_quality_ruleset) [CRUD]
- [Tags](#tags) [R]
- [Schema_versions](#schema_versions) [D]
- [User_defined_functions](#user_defined_functions) [R]
- [Workflow_run](#workflow_run) [R]
- [Column_statistics_task_settings](#column_statistics_task_settings) [CRUD]
- [Schema_versions_diff](#schema_versions_diff) [R]
- [Data_quality_result](#data_quality_result) [R]
- [Data_quality_ruleset_evaluation_run](#data_quality_ruleset_evaluation_run) [R]
- [Classifiers](#classifiers) [R]
- [Workflow_runs](#workflow_runs) [R]
- [Job_runs](#job_runs) [R]
- [Blueprint](#blueprint) [CRUD]
- [Data_quality_profile_annotation](#data_quality_profile_annotation) [C]
- [Crawler_metrics](#crawler_metrics) [R]
- [Source_control_from_job](#source_control_from_job) [U]
- [Plan](#plan) [R]
- [Connections](#connections) [R]
- [Inbound_integrations](#inbound_integrations) [R]
- [Column_statistics_task_run](#column_statistics_task_run) [R]
- [Usage_profile](#usage_profile) [CRUD]
- [Integrations](#integrations) [R]
- [Blueprint_run](#blueprint_run) [R]
- [User_defined_function](#user_defined_function) [CRUD]
- [Registry](#registry) [CRUD]
- [Ml_task_run](#ml_task_run) [R]
- [Statement](#statement) [R]
- [Schema_version_metadata](#schema_version_metadata) [C]
- [Integration_table_properties](#integration_table_properties) [CRUD]
- [Data_quality_model](#data_quality_model) [R]
- [Trigger](#trigger) [CRUD]
- [Entity_records](#entity_records) [R]
- [Resource_policies](#resource_policies) [R]
- [Schema_version](#schema_version) [R]
- [Table](#table) [CRUD]
- [Catalogs](#catalogs) [R]
- [Data_quality_rule_recommendation_run](#data_quality_rule_recommendation_run) [R]
- [Unfiltered_partitions_metadata](#unfiltered_partitions_metadata) [R]
- [Catalog_import_status](#catalog_import_status) [R]
- [Custom_entity_type](#custom_entity_type) [CRD]
- [Tables](#tables) [R]

---

## Resources


### Schema

Schema resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `schema_definition` | String |  | <p>The schema definition using the <code>DataFormat</code> setting for <code>SchemaName</code>.</p> |
| `tags` | HashMap<String, String> |  | <p>Amazon Web Services tags that contain a key value pair and may be searched by console, command line, or API. If specified, follows the Amazon Web Services tags-on-create pattern.</p> |
| `data_format` | String | ✅ | <p>The data format of the schema definition. Currently <code>AVRO</code>, <code>JSON</code> and <code>PROTOBUF</code> are supported.</p> |
| `description` | String |  | <p>An optional description of the schema. If description is not provided, there will not be any automatic default value for this.</p> |
| `compatibility` | String |  | <p>The compatibility mode of the schema. The possible values are:</p>
         <ul>
            <li>
               <p>
                  <i>NONE</i>: No compatibility mode applies. You can use this choice in development scenarios or if you do not know the compatibility mode that you want to apply to schemas. Any new version added will be accepted without undergoing a compatibility check.</p>
            </li>
            <li>
               <p>
                  <i>DISABLED</i>: This compatibility choice prevents versioning for a particular schema. You can use this choice to prevent future versioning of a schema.</p>
            </li>
            <li>
               <p>
                  <i>BACKWARD</i>: This compatibility choice is recommended as it allows data receivers to read both the current and one previous schema version. This means that for instance, a new schema version cannot drop data fields or change the type of these fields, so they can't be read by readers using the previous version.</p>
            </li>
            <li>
               <p>
                  <i>BACKWARD_ALL</i>: This compatibility choice allows data receivers to read both the current and all previous schema versions. You can use this choice when you need to delete fields or add optional fields, and check compatibility against all previous schema versions. </p>
            </li>
            <li>
               <p>
                  <i>FORWARD</i>: This compatibility choice allows data receivers to read both the current and one next schema version, but not necessarily later versions. You can use this choice when you need to add fields or delete optional fields, but only check compatibility against the last schema version.</p>
            </li>
            <li>
               <p>
                  <i>FORWARD_ALL</i>: This compatibility choice allows data receivers to read written by producers of any new registered schema. You can use this choice when you need to add fields or delete optional fields, and check compatibility against all previous schema versions.</p>
            </li>
            <li>
               <p>
                  <i>FULL</i>: This compatibility choice allows data receivers to read data written by producers using the previous or next version of the schema, but not necessarily earlier or later versions. You can use this choice when you need to add or remove optional fields, but only check compatibility against the last schema version.</p>
            </li>
            <li>
               <p>
                  <i>FULL_ALL</i>: This compatibility choice allows data receivers to read data written by producers using all previous schema versions. You can use this choice when you need to add or remove optional fields, and check compatibility against all previous schema versions.</p>
            </li>
         </ul> |
| `schema_name` | String | ✅ | <p>Name of the schema to be created of max length of 255, and may only contain letters, numbers, hyphen, underscore, dollar sign, or hash mark. No whitespace.</p> |
| `registry_id` | String |  | <p> This is a wrapper shape to contain the registry identity fields. If this is not provided, the default registry will be used. The ARN format for the same will be: <code>arn:aws:glue:us-east-2:<customer id>:registry/default-registry:random-5-letter-id</code>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_schema_version` | i64 | <p>The next version of the schema associated with the returned schema definition.</p> |
| `compatibility` | String | <p>The compatibility mode of the schema.</p> |
| `registry_name` | String | <p>The name of the registry.</p> |
| `schema_checkpoint` | i64 | <p>The version number of the checkpoint (the last time the compatibility mode was changed).</p> |
| `description` | String | <p>A description of schema if specified when created</p> |
| `data_format` | String | <p>The data format of the schema definition. Currently <code>AVRO</code>, <code>JSON</code> and <code>PROTOBUF</code> are supported.</p> |
| `updated_time` | String | <p>The date and time the schema was updated.</p> |
| `schema_name` | String | <p>The name of the schema.</p> |
| `latest_schema_version` | i64 | <p>The latest version of the schema associated with the returned schema definition.</p> |
| `created_time` | String | <p>The date and time the schema was created.</p> |
| `registry_arn` | String | <p>The Amazon Resource Name (ARN) of the registry.</p> |
| `schema_status` | String | <p>The status of the schema.</p> |
| `schema_arn` | String | <p>The Amazon Resource Name (ARN) of the schema.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create schema
schema = provider.glue.Schema {
    data_format = "value"  # <p>The data format of the schema definition. Currently <code>AVRO</code>, <code>JSON</code> and <code>PROTOBUF</code> are supported.</p>
    schema_name = "value"  # <p>Name of the schema to be created of max length of 255, and may only contain letters, numbers, hyphen, underscore, dollar sign, or hash mark. No whitespace.</p>
}

# Access schema outputs
schema_id = schema.id
schema_next_schema_version = schema.next_schema_version
schema_compatibility = schema.compatibility
schema_registry_name = schema.registry_name
schema_schema_checkpoint = schema.schema_checkpoint
schema_description = schema.description
schema_data_format = schema.data_format
schema_updated_time = schema.updated_time
schema_schema_name = schema.schema_name
schema_latest_schema_version = schema.latest_schema_version
schema_created_time = schema.created_time
schema_registry_arn = schema.registry_arn
schema_schema_status = schema.schema_status
schema_schema_arn = schema.schema_arn
```

---


### Mapping

Mapping resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `mapping` | Vec<String> | <p>A list of mappings to the specified targets.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access mapping outputs
mapping_id = mapping.id
mapping_mapping = mapping.mapping
```

---


### Ml_transforms

MLTransforms resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>A pagination token, if more results are available.</p> |
| `transforms` | Vec<String> | <p>A list of machine learning transforms.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access ml_transforms outputs
ml_transforms_id = ml_transforms.id
ml_transforms_next_token = ml_transforms.next_token
ml_transforms_transforms = ml_transforms.transforms
```

---


### Databases

Databases resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>A continuation token for paginating the returned list of tokens,
      returned if the current segment of the list is not the last.</p> |
| `database_list` | Vec<String> | <p>A list of <code>Database</code> objects from the specified catalog.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access databases outputs
databases_id = databases.id
databases_next_token = databases.next_token
databases_database_list = databases.database_list
```

---


### Resource_policy

ResourcePolicy resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resource_arn` | String |  | <p>Do not use. For internal use only.</p> |
| `enable_hybrid` | String |  | <p>If <code>'TRUE'</code>, indicates that you are using both methods to grant cross-account
      access to Data Catalog resources:</p>
         <ul>
            <li>
               <p>By directly updating the resource policy with <code>PutResourePolicy</code>
               </p>
            </li>
            <li>
               <p>By using the <b>Grant permissions</b> command on the Amazon Web Services Management Console.</p>
            </li>
         </ul>
         <p>Must be set to <code>'TRUE'</code> if you have already used the Management Console to
      grant cross-account access, otherwise the call fails. Default is 'FALSE'.</p> |
| `policy_in_json` | String | ✅ | <p>Contains the policy document to set, in JSON format.</p> |
| `policy_exists_condition` | String |  | <p>A value of <code>MUST_EXIST</code> is used to update a policy. A value of
        <code>NOT_EXIST</code> is used to create a new policy. If a value of <code>NONE</code> or a
      null value is used, the call does not depend on the existence of a policy.</p> |
| `policy_hash_condition` | String |  | <p>The hash value returned when the previous policy was set using
        <code>PutResourcePolicy</code>. Its purpose is to prevent concurrent modifications of a
      policy. Do not use this parameter if no previous policy has been set.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | <p>The date and time at which the policy was last updated.</p> |
| `policy_in_json` | String | <p>Contains the requested policy document, in JSON format.</p> |
| `create_time` | String | <p>The date and time at which the policy was created.</p> |
| `policy_hash` | String | <p>Contains the hash value associated with this policy.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create resource_policy
resource_policy = provider.glue.Resource_policy {
    policy_in_json = "value"  # <p>Contains the policy document to set, in JSON format.</p>
}

# Access resource_policy outputs
resource_policy_id = resource_policy.id
resource_policy_update_time = resource_policy.update_time
resource_policy_policy_in_json = resource_policy.policy_in_json
resource_policy_create_time = resource_policy.create_time
resource_policy_policy_hash = resource_policy.policy_hash
```

---


### Table_optimizer

TableOptimizer resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `table_name` | String | ✅ | <p>The name of the table.</p> |
| `type` | String | ✅ | <p>The type of table optimizer.</p> |
| `database_name` | String | ✅ | <p>The name of the database in the catalog in which the table resides.</p> |
| `table_optimizer_configuration` | String | ✅ | <p>A <code>TableOptimizerConfiguration</code> object representing the configuration of a table optimizer.</p> |
| `catalog_id` | String | ✅ | <p>The Catalog ID of the table.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `catalog_id` | String | <p>The Catalog ID of the table.</p> |
| `table_name` | String | <p>The name of the table.</p> |
| `table_optimizer` | String | <p>The optimizer associated with the specified table.</p> |
| `database_name` | String | <p>The name of the database in the catalog in which the table resides.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create table_optimizer
table_optimizer = provider.glue.Table_optimizer {
    table_name = "value"  # <p>The name of the table.</p>
    type = "value"  # <p>The type of table optimizer.</p>
    database_name = "value"  # <p>The name of the database in the catalog in which the table resides.</p>
    table_optimizer_configuration = "value"  # <p>A <code>TableOptimizerConfiguration</code> object representing the configuration of a table optimizer.</p>
    catalog_id = "value"  # <p>The Catalog ID of the table.</p>
}

# Access table_optimizer outputs
table_optimizer_id = table_optimizer.id
table_optimizer_catalog_id = table_optimizer.catalog_id
table_optimizer_table_name = table_optimizer.table_name
table_optimizer_table_optimizer = table_optimizer.table_optimizer
table_optimizer_database_name = table_optimizer.database_name
```

---


### Entity

Entity resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `fields` | Vec<String> | <p>Describes the fields for that connector entity. This is the list of <code>Field</code> objects. <code>Field</code> is very similar to column in a database. The <code>Field</code> object has information about different properties associated with fields in the connector.</p> |
| `next_token` | String | <p>A continuation token, present if the current segment is not the last.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access entity outputs
entity_id = entity.id
entity_fields = entity.fields
entity_next_token = entity.next_token
```

---


### Unfiltered_partition_metadata

UnfilteredPartitionMetadata resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `is_registered_with_lake_formation` | bool | <p>A Boolean value that indicates whether the partition location is registered 
          with Lake Formation.</p> |
| `partition` | String | <p>A Partition object containing the partition metadata.</p> |
| `authorized_columns` | Vec<String> | <p>A list of column names that the user has been granted access to.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access unfiltered_partition_metadata outputs
unfiltered_partition_metadata_id = unfiltered_partition_metadata.id
unfiltered_partition_metadata_is_registered_with_lake_formation = unfiltered_partition_metadata.is_registered_with_lake_formation
unfiltered_partition_metadata_partition = unfiltered_partition_metadata.partition
unfiltered_partition_metadata_authorized_columns = unfiltered_partition_metadata.authorized_columns
```

---


### Job_run

JobRun resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `job_run` | String | <p>The requested job-run metadata.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access job_run outputs
job_run_id = job_run.id
job_run_job_run = job_run.job_run
```

---


### Data_catalog_encryption_settings

DataCatalogEncryptionSettings resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `catalog_id` | String |  | <p>The ID of the Data Catalog to set the security configuration for. If none is provided, the
      Amazon Web Services account ID is used by default.</p> |
| `data_catalog_encryption_settings` | String | ✅ | <p>The security configuration to set.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `data_catalog_encryption_settings` | String | <p>The requested security configuration.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create data_catalog_encryption_settings
data_catalog_encryption_settings = provider.glue.Data_catalog_encryption_settings {
    data_catalog_encryption_settings = "value"  # <p>The security configuration to set.</p>
}

# Access data_catalog_encryption_settings outputs
data_catalog_encryption_settings_id = data_catalog_encryption_settings.id
data_catalog_encryption_settings_data_catalog_encryption_settings = data_catalog_encryption_settings.data_catalog_encryption_settings
```

---


### Integration_resource_property

IntegrationResourceProperty resource

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `target_processing_properties` | String |  | <p>The resource properties associated with the integration target.</p> |
| `resource_arn` | String | ✅ | <p>The connection ARN of the source, or the database ARN of the target.</p> |
| `source_processing_properties` | String |  | <p>The resource properties associated with the integration source.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `target_processing_properties` | String | <p>The resource properties associated with the integration target.</p> |
| `source_processing_properties` | String | <p>The resource properties associated with the integration source.</p> |
| `resource_arn` | String | <p>The connection ARN of the source, or the database ARN of the target.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create integration_resource_property
integration_resource_property = provider.glue.Integration_resource_property {
    resource_arn = "value"  # <p>The connection ARN of the source, or the database ARN of the target.</p>
}

# Access integration_resource_property outputs
integration_resource_property_id = integration_resource_property.id
integration_resource_property_target_processing_properties = integration_resource_property.target_processing_properties
integration_resource_property_source_processing_properties = integration_resource_property.source_processing_properties
integration_resource_property_resource_arn = integration_resource_property.resource_arn
```

---


### Classifier

Classifier resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `csv_classifier` | String |  | <p>A <code>CsvClassifier</code> object specifying the classifier
      to create.</p> |
| `grok_classifier` | String |  | <p>A <code>GrokClassifier</code> object specifying the classifier
      to create.</p> |
| `json_classifier` | String |  | <p>A <code>JsonClassifier</code> object specifying the classifier
      to create.</p> |
| `xml_classifier` | String |  | <p>An <code>XMLClassifier</code> object specifying the classifier
      to create.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `classifier` | String | <p>The requested classifier.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create classifier
classifier = provider.glue.Classifier {
}

# Access classifier outputs
classifier_id = classifier.id
classifier_classifier = classifier.classifier
```

---


### Crawler

Crawler resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `classifiers` | Vec<String> |  | <p>A list of custom classifiers that the user has registered. By default, all built-in
      classifiers are included in a crawl, but these custom classifiers always override the default
      classifiers for a given classification.</p> |
| `schedule` | String |  | <p>A <code>cron</code> expression used to specify the schedule (see <a href="https://docs.aws.amazon.com/glue/latest/dg/monitor-data-warehouse-schedule.html">Time-Based Schedules for Jobs and Crawlers</a>. For example, to run
      something every day at 12:15 UTC, you would specify:
      <code>cron(15 12 * * ? *)</code>.</p> |
| `name` | String | ✅ | <p>Name of the new crawler.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags to use with this crawler request. You may use tags to limit access to the
            crawler. For more information about tags in Glue, see <a href="https://docs.aws.amazon.com/glue/latest/dg/monitor-tags.html">Amazon Web Services Tags in Glue</a> in the developer
            guide.</p> |
| `role` | String | ✅ | <p>The IAM role or Amazon Resource Name (ARN) of an IAM role used by the new crawler to
      access customer resources.</p> |
| `configuration` | String |  | <p>Crawler configuration information. This versioned JSON
      string allows users to specify aspects of a crawler's behavior.
      For more information, see <a href="https://docs.aws.amazon.com/glue/latest/dg/crawler-configuration.html">Setting crawler configuration options</a>.</p> |
| `targets` | String | ✅ | <p>A list of collection of targets to crawl.</p> |
| `schema_change_policy` | String |  | <p>The policy for the crawler's update and deletion behavior.</p> |
| `database_name` | String |  | <p>The Glue database where results are written, such as:
        <code>arn:aws:daylight:us-east-1::database/sometable/*</code>.</p> |
| `description` | String |  | <p>A description of the new crawler.</p> |
| `table_prefix` | String |  | <p>The table prefix used for catalog tables that are created.</p> |
| `recrawl_policy` | String |  | <p>A policy that specifies whether to crawl the entire dataset again, or to crawl only folders that were added since the last crawler run.</p> |
| `lake_formation_configuration` | String |  | <p>Specifies Lake Formation configuration settings for the crawler.</p> |
| `lineage_configuration` | String |  | <p>Specifies data lineage configuration settings for the crawler.</p> |
| `crawler_security_configuration` | String |  | <p>The name of the <code>SecurityConfiguration</code> structure to be used by this
      crawler.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `crawler` | String | <p>The metadata for the specified crawler.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create crawler
crawler = provider.glue.Crawler {
    name = "value"  # <p>Name of the new crawler.</p>
    role = "value"  # <p>The IAM role or Amazon Resource Name (ARN) of an IAM role used by the new crawler to
      access customer resources.</p>
    targets = "value"  # <p>A list of collection of targets to crawl.</p>
}

# Access crawler outputs
crawler_id = crawler.id
crawler_crawler = crawler.crawler
```

---


### Ml_transform

MLTransform resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `glue_version` | String |  | <p>This value determines which version of Glue this machine learning transform is compatible with. Glue 1.0 is recommended for most customers. If the value is not set, the Glue compatibility defaults to Glue 0.9.  For more information, see <a href="https://docs.aws.amazon.com/glue/latest/dg/release-notes.html#release-notes-versions">Glue Versions</a> in the developer guide.</p> |
| `description` | String |  | <p>A description of the machine learning transform that is being defined. The default is an
      empty string.</p> |
| `worker_type` | String |  | <p>The type of predefined worker that is allocated when this task runs. Accepts a value of Standard, G.1X, or G.2X.</p>
         <ul>
            <li>
               <p>For the <code>Standard</code> worker type, each worker provides 4 vCPU, 16 GB of memory and a 50GB disk, and 2 executors per worker.</p>
            </li>
            <li>
               <p>For the <code>G.1X</code> worker type, each worker provides 4 vCPU, 16 GB of memory and a 64GB disk, and 1 executor per worker.</p>
            </li>
            <li>
               <p>For the <code>G.2X</code> worker type, each worker provides 8 vCPU, 32 GB of memory and a 128GB disk, and 1 executor per worker.</p>
            </li>
         </ul>
         <p>
            <code>MaxCapacity</code> is a mutually exclusive option with <code>NumberOfWorkers</code> and <code>WorkerType</code>.</p>
         <ul>
            <li>
               <p>If either <code>NumberOfWorkers</code> or <code>WorkerType</code> is set, then <code>MaxCapacity</code> cannot be set.</p>
            </li>
            <li>
               <p>If <code>MaxCapacity</code> is set then neither <code>NumberOfWorkers</code> or <code>WorkerType</code> can be set.</p>
            </li>
            <li>
               <p>If <code>WorkerType</code> is set, then <code>NumberOfWorkers</code> is required (and vice versa).</p>
            </li>
            <li>
               <p>
                  <code>MaxCapacity</code> and <code>NumberOfWorkers</code> must both be at least 1.</p>
            </li>
         </ul> |
| `name` | String | ✅ | <p>The unique name that you give the transform when you create it.</p> |
| `role` | String | ✅ | <p>The name or Amazon Resource Name (ARN) of the IAM role with the required permissions. The required permissions include both Glue service role permissions to Glue resources, and Amazon S3 permissions required by the transform. </p>
         <ul>
            <li>
               <p>This role needs Glue service role permissions to allow access to resources in Glue. See <a href="https://docs.aws.amazon.com/glue/latest/dg/attach-policy-iam-user.html">Attach a Policy to IAM Users That Access Glue</a>.</p>
            </li>
            <li>
               <p>This role needs permission to your Amazon Simple Storage Service (Amazon S3) sources, targets, temporary directory, scripts, and any libraries used by the task run for this transform.</p>
            </li>
         </ul> |
| `parameters` | String | ✅ | <p>The algorithmic parameters that are specific to the transform type used. Conditionally
      dependent on the transform type.</p> |
| `max_capacity` | f64 |  | <p>The number of Glue data processing units (DPUs) that are allocated to task runs for this transform. You can allocate from 2 to 100 DPUs; the default is 10. A DPU is a relative measure of
      processing power that consists of 4 vCPUs of compute capacity and 16 GB of memory. For more
      information, see the <a href="https://aws.amazon.com/glue/pricing/">Glue pricing
        page</a>. </p>
         <p>
            <code>MaxCapacity</code> is a mutually exclusive option with <code>NumberOfWorkers</code> and <code>WorkerType</code>.</p>
         <ul>
            <li>
               <p>If either <code>NumberOfWorkers</code> or <code>WorkerType</code> is set, then <code>MaxCapacity</code> cannot be set.</p>
            </li>
            <li>
               <p>If <code>MaxCapacity</code> is set then neither <code>NumberOfWorkers</code> or <code>WorkerType</code> can be set.</p>
            </li>
            <li>
               <p>If <code>WorkerType</code> is set, then <code>NumberOfWorkers</code> is required (and vice versa).</p>
            </li>
            <li>
               <p>
                  <code>MaxCapacity</code> and <code>NumberOfWorkers</code> must both be at least 1.</p>
            </li>
         </ul>
         <p>When the <code>WorkerType</code> field is set to a value other than <code>Standard</code>, the <code>MaxCapacity</code> field is set automatically and becomes read-only.</p>
         <p>When the <code>WorkerType</code> field is set to a value other than <code>Standard</code>, the <code>MaxCapacity</code> field is set automatically and becomes read-only.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags to use with this machine learning transform. You may use tags to limit access to the machine learning transform. For more information about tags in Glue, see <a href="https://docs.aws.amazon.com/glue/latest/dg/monitor-tags.html">Amazon Web Services Tags in Glue</a> in the developer guide.</p> |
| `transform_encryption` | String |  | <p>The encryption-at-rest settings of the transform that apply to accessing user data. Machine learning transforms can access user data encrypted in Amazon S3 using KMS.</p> |
| `timeout` | i64 |  | <p>The timeout of the task run for this transform in minutes. This is the maximum time that a task run for this transform can consume resources before it is terminated and enters <code>TIMEOUT</code> status. The default is 2,880 minutes (48 hours).</p> |
| `input_record_tables` | Vec<String> | ✅ | <p>A list of Glue table definitions used by the transform.</p> |
| `number_of_workers` | i64 |  | <p>The number of workers of a defined <code>workerType</code> that are allocated when this task runs.</p>
         <p>If <code>WorkerType</code> is set, then <code>NumberOfWorkers</code> is required (and vice versa).</p> |
| `max_retries` | i64 |  | <p>The maximum number of times to retry a task for this transform after a task run fails.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `transform_encryption` | String | <p>The encryption-at-rest settings of the transform that apply to accessing user data. Machine learning transforms can access user data encrypted in Amazon S3 using KMS.</p> |
| `schema` | Vec<String> | <p>The <code>Map<Column, Type></code> object that represents the schema that this
      transform accepts. Has an upper bound of 100 columns.</p> |
| `name` | String | <p>The unique name given to the transform when it was created.</p> |
| `number_of_workers` | i64 | <p>The number of workers of a defined <code>workerType</code> that are allocated when this task runs.</p> |
| `max_retries` | i64 | <p>The maximum number of times to retry a task for this transform after a task run fails.</p> |
| `evaluation_metrics` | String | <p>The latest evaluation metrics.</p> |
| `parameters` | String | <p>The configuration parameters that are specific to the algorithm used.</p> |
| `label_count` | i64 | <p>The number of labels available for this transform.</p> |
| `max_capacity` | f64 | <p>The number of Glue data processing units (DPUs) that are allocated to task runs for this transform. You can allocate from 2 to 100 DPUs; the default is 10. A DPU is a relative measure of
      processing power that consists of 4 vCPUs of compute capacity and 16 GB of memory. For more
      information, see the <a href="https://aws.amazon.com/glue/pricing/">Glue pricing
        page</a>. </p>
         <p>When the <code>WorkerType</code> field is set to a value other than <code>Standard</code>, the <code>MaxCapacity</code> field is set automatically and becomes read-only.</p> |
| `created_on` | String | <p>The date and time when the transform was created.</p> |
| `role` | String | <p>The name or Amazon Resource Name (ARN) of the IAM role with the required
      permissions.</p> |
| `glue_version` | String | <p>This value determines which version of Glue this machine learning transform is compatible with. Glue 1.0 is recommended for most customers. If the value is not set, the Glue compatibility defaults to Glue 0.9.  For more information, see <a href="https://docs.aws.amazon.com/glue/latest/dg/release-notes.html#release-notes-versions">Glue Versions</a> in the developer guide.</p> |
| `description` | String | <p>A description of the transform.</p> |
| `last_modified_on` | String | <p>The date and time when the transform was last modified.</p> |
| `transform_id` | String | <p>The unique identifier of the transform, generated at the time that the transform was
      created.</p> |
| `input_record_tables` | Vec<String> | <p>A list of Glue table definitions used by the transform.</p> |
| `worker_type` | String | <p>The type of predefined worker that is allocated when this task runs. Accepts a value of Standard, G.1X, or G.2X.</p>
         <ul>
            <li>
               <p>For the <code>Standard</code> worker type, each worker provides 4 vCPU, 16 GB of memory and a 50GB disk, and 2 executors per worker.</p>
            </li>
            <li>
               <p>For the <code>G.1X</code> worker type, each worker provides 4 vCPU, 16 GB of memory and a 64GB disk, and 1 executor per worker.</p>
            </li>
            <li>
               <p>For the <code>G.2X</code> worker type, each worker provides 8 vCPU, 32 GB of memory and a 128GB disk, and 1 executor per worker.</p>
            </li>
         </ul> |
| `timeout` | i64 | <p>The timeout for a task run for this transform in minutes. This is the maximum time that a task run for this transform can consume resources before it is terminated and enters <code>TIMEOUT</code> status. The default is 2,880 minutes (48 hours).</p> |
| `status` | String | <p>The last known status of the transform (to indicate whether it can be used or not). One of "NOT_READY", "READY", or "DELETING".</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create ml_transform
ml_transform = provider.glue.Ml_transform {
    name = "value"  # <p>The unique name that you give the transform when you create it.</p>
    role = "value"  # <p>The name or Amazon Resource Name (ARN) of the IAM role with the required permissions. The required permissions include both Glue service role permissions to Glue resources, and Amazon S3 permissions required by the transform. </p>
         <ul>
            <li>
               <p>This role needs Glue service role permissions to allow access to resources in Glue. See <a href="https://docs.aws.amazon.com/glue/latest/dg/attach-policy-iam-user.html">Attach a Policy to IAM Users That Access Glue</a>.</p>
            </li>
            <li>
               <p>This role needs permission to your Amazon Simple Storage Service (Amazon S3) sources, targets, temporary directory, scripts, and any libraries used by the task run for this transform.</p>
            </li>
         </ul>
    parameters = "value"  # <p>The algorithmic parameters that are specific to the transform type used. Conditionally
      dependent on the transform type.</p>
    input_record_tables = "value"  # <p>A list of Glue table definitions used by the transform.</p>
}

# Access ml_transform outputs
ml_transform_id = ml_transform.id
ml_transform_transform_encryption = ml_transform.transform_encryption
ml_transform_schema = ml_transform.schema
ml_transform_name = ml_transform.name
ml_transform_number_of_workers = ml_transform.number_of_workers
ml_transform_max_retries = ml_transform.max_retries
ml_transform_evaluation_metrics = ml_transform.evaluation_metrics
ml_transform_parameters = ml_transform.parameters
ml_transform_label_count = ml_transform.label_count
ml_transform_max_capacity = ml_transform.max_capacity
ml_transform_created_on = ml_transform.created_on
ml_transform_role = ml_transform.role
ml_transform_glue_version = ml_transform.glue_version
ml_transform_description = ml_transform.description
ml_transform_last_modified_on = ml_transform.last_modified_on
ml_transform_transform_id = ml_transform.transform_id
ml_transform_input_record_tables = ml_transform.input_record_tables
ml_transform_worker_type = ml_transform.worker_type
ml_transform_timeout = ml_transform.timeout
ml_transform_status = ml_transform.status
```

---


### Dataflow_graph

DataflowGraph resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `dag_nodes` | Vec<String> | <p>A list of the nodes in the resulting DAG.</p> |
| `dag_edges` | Vec<String> | <p>A list of the edges in the resulting DAG.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access dataflow_graph outputs
dataflow_graph_id = dataflow_graph.id
dataflow_graph_dag_nodes = dataflow_graph.dag_nodes
dataflow_graph_dag_edges = dataflow_graph.dag_edges
```

---


### Table_version

TableVersion resource

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `table_version` | String | <p>The requested table version.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access table_version outputs
table_version_id = table_version.id
table_version_table_version = table_version.table_version
```

---


### Partition_indexes

PartitionIndexes resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `partition_index_descriptor_list` | Vec<String> | <p>A list of index descriptors.</p> |
| `next_token` | String | <p>A continuation token, present if the current list segment is not the last.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access partition_indexes outputs
partition_indexes_id = partition_indexes.id
partition_indexes_partition_index_descriptor_list = partition_indexes.partition_index_descriptor_list
partition_indexes_next_token = partition_indexes.next_token
```

---


### Column_statistics_for_partition

ColumnStatisticsForPartition resource

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `column_statistics_list` | Vec<String> | ✅ | <p>A list of the column statistics.</p> |
| `table_name` | String | ✅ | <p>The name of the partitions' table.</p> |
| `database_name` | String | ✅ | <p>The name of the catalog database where the partitions reside.</p> |
| `partition_values` | Vec<String> | ✅ | <p>A list of partition values identifying the partition.</p> |
| `catalog_id` | String |  | <p>The ID of the Data Catalog where the partitions in question reside.
      If none is supplied, the Amazon Web Services account ID is used by default.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `errors` | Vec<String> | <p>Error occurred during retrieving column statistics data.</p> |
| `column_statistics_list` | Vec<String> | <p>List of ColumnStatistics that failed to be retrieved.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access column_statistics_for_partition outputs
column_statistics_for_partition_id = column_statistics_for_partition.id
column_statistics_for_partition_errors = column_statistics_for_partition.errors
column_statistics_for_partition_column_statistics_list = column_statistics_for_partition.column_statistics_list
```

---


### Script

Script resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `language` | String |  | <p>The programming language of the resulting code from the DAG.</p> |
| `dag_nodes` | Vec<String> |  | <p>A list of the nodes in the DAG.</p> |
| `dag_edges` | Vec<String> |  | <p>A list of the edges in the DAG.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create script
script = provider.glue.Script {
}

```

---


### Security_configurations

SecurityConfigurations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `security_configurations` | Vec<String> | <p>A list of security configurations.</p> |
| `next_token` | String | <p>A continuation token, if there are more security
      configurations to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access security_configurations outputs
security_configurations_id = security_configurations.id
security_configurations_security_configurations = security_configurations.security_configurations
security_configurations_next_token = security_configurations.next_token
```

---


### Partition

Partition resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `partition_input` | String | ✅ | <p>A <code>PartitionInput</code> structure defining the partition
      to be created.</p> |
| `database_name` | String | ✅ | <p>The name of the metadata database in which the partition is
      to be created.</p> |
| `table_name` | String | ✅ | <p>The name of the metadata table in which the partition is to be created.</p> |
| `catalog_id` | String |  | <p>The Amazon Web Services account ID of the catalog in which the partition is to be created.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `partition` | String | <p>The requested information, in the form of a <code>Partition</code>
      object.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create partition
partition = provider.glue.Partition {
    partition_input = "value"  # <p>A <code>PartitionInput</code> structure defining the partition
      to be created.</p>
    database_name = "value"  # <p>The name of the metadata database in which the partition is
      to be created.</p>
    table_name = "value"  # <p>The name of the metadata table in which the partition is to be created.</p>
}

# Access partition outputs
partition_id = partition.id
partition_partition = partition.partition
```

---


### Column_statistics_for_table

ColumnStatisticsForTable resource

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `column_statistics_list` | Vec<String> | ✅ | <p>A list of the column statistics.</p> |
| `database_name` | String | ✅ | <p>The name of the catalog database where the partitions reside.</p> |
| `table_name` | String | ✅ | <p>The name of the partitions' table.</p> |
| `catalog_id` | String |  | <p>The ID of the Data Catalog where the partitions in question reside.
      If none is supplied, the Amazon Web Services account ID is used by default.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `column_statistics_list` | Vec<String> | <p>List of ColumnStatistics.</p> |
| `errors` | Vec<String> | <p>List of ColumnStatistics that failed to be retrieved.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access column_statistics_for_table outputs
column_statistics_for_table_id = column_statistics_for_table.id
column_statistics_for_table_column_statistics_list = column_statistics_for_table.column_statistics_list
column_statistics_for_table_errors = column_statistics_for_table.errors
```

---


### Data_quality_model_result

DataQualityModelResult resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `model` | Vec<String> | <p>A list of <code>StatisticModelResult</code>
         </p> |
| `completed_on` | String | <p>The timestamp when the data quality model training completed.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access data_quality_model_result outputs
data_quality_model_result_id = data_quality_model_result.id
data_quality_model_result_model = data_quality_model_result.model
data_quality_model_result_completed_on = data_quality_model_result.completed_on
```

---


### Unfiltered_table_metadata

UnfilteredTableMetadata resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `query_authorization_id` | String | <p>A cryptographically generated query identifier generated by Glue or Lake Formation.</p> |
| `resource_arn` | String | <p>The resource ARN of the parent resource extracted from the request.</p> |
| `table` | String | <p>A Table object containing the table metadata.</p> |
| `is_registered_with_lake_formation` | bool | <p>A Boolean value that indicates whether the partition location is registered 
          with Lake Formation.</p> |
| `row_filter` | String | <p>The filter that applies to the table. For example when applying the filter in SQL, it would go in the <code>WHERE</code> clause and can be evaluated by using an <code>AND</code> operator with any other predicates applied by the user querying the table.</p> |
| `cell_filters` | Vec<String> | <p>A list of column row filters.</p> |
| `authorized_columns` | Vec<String> | <p>A list of column names that the user has been granted access to.</p> |
| `is_multi_dialect_view` | bool | <p>Specifies whether the view supports the SQL dialects of one or more different query engines and can therefore be read by those engines.</p> |
| `is_protected` | bool | <p>A flag that instructs the engine not to push user-provided operations into the logical plan of the view during query planning. However, if set this flag does not guarantee that the engine will comply. Refer to the engine's documentation to understand the guarantees provided, if any.</p> |
| `permissions` | Vec<String> | <p>The Lake Formation data permissions of the caller on the table. Used to authorize the call when no view context is found.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access unfiltered_table_metadata outputs
unfiltered_table_metadata_id = unfiltered_table_metadata.id
unfiltered_table_metadata_query_authorization_id = unfiltered_table_metadata.query_authorization_id
unfiltered_table_metadata_resource_arn = unfiltered_table_metadata.resource_arn
unfiltered_table_metadata_table = unfiltered_table_metadata.table
unfiltered_table_metadata_is_registered_with_lake_formation = unfiltered_table_metadata.is_registered_with_lake_formation
unfiltered_table_metadata_row_filter = unfiltered_table_metadata.row_filter
unfiltered_table_metadata_cell_filters = unfiltered_table_metadata.cell_filters
unfiltered_table_metadata_authorized_columns = unfiltered_table_metadata.authorized_columns
unfiltered_table_metadata_is_multi_dialect_view = unfiltered_table_metadata.is_multi_dialect_view
unfiltered_table_metadata_is_protected = unfiltered_table_metadata.is_protected
unfiltered_table_metadata_permissions = unfiltered_table_metadata.permissions
```

---


### Connection

Connection resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | HashMap<String, String> |  | <p>The tags you assign to the connection.</p> |
| `catalog_id` | String |  | <p>The ID of the Data Catalog in which to create the connection. If none is provided, the Amazon Web Services
      account ID is used by default.</p> |
| `connection_input` | String | ✅ | <p>A <code>ConnectionInput</code> object defining the connection
      to create.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `connection` | String | <p>The requested connection definition.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create connection
connection = provider.glue.Connection {
    connection_input = "value"  # <p>A <code>ConnectionInput</code> object defining the connection
      to create.</p>
}

# Access connection outputs
connection_id = connection.id
connection_connection = connection.connection
```

---


### Session

Session resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `number_of_workers` | i64 |  | <p>The number of workers of a defined <code>WorkerType</code> to use for the session. </p> |
| `glue_version` | String |  | <p>The Glue version determines the versions of Apache Spark and Python that Glue supports. 
      The GlueVersion must be greater than 2.0. </p> |
| `role` | String | ✅ | <p>The IAM Role ARN </p> |
| `default_arguments` | HashMap<String, String> |  | <p>A map array of key-value pairs. Max is 75 pairs. </p> |
| `description` | String |  | <p>The description of the session. </p> |
| `worker_type` | String |  | <p>The type of predefined worker that is allocated when a job runs. Accepts a value of
      G.1X, G.2X, G.4X, or G.8X for Spark jobs. Accepts the value Z.2X for Ray notebooks.</p>
         <ul>
            <li>
               <p>For the <code>G.1X</code> worker type, each worker maps to 1 DPU (4 vCPUs, 16 GB of memory) with 94GB disk, and provides 1 executor per worker. We recommend this worker type for workloads such as data transforms, joins, and queries, to offers a scalable and cost effective way to run most jobs.</p>
            </li>
            <li>
               <p>For the <code>G.2X</code> worker type, each worker maps to 2 DPU (8 vCPUs, 32 GB of memory) with 138GB disk, and provides 1 executor per worker. We recommend this worker type for workloads such as data transforms, joins, and queries, to offers a scalable and cost effective way to run most jobs.</p>
            </li>
            <li>
               <p>For the <code>G.4X</code> worker type, each worker maps to 4 DPU (16 vCPUs, 64 GB of memory) with 256GB disk, and provides 1 executor per worker. We recommend this worker type for jobs whose workloads contain your most demanding transforms, aggregations, joins, and queries. This worker type is available only for Glue version 3.0 or later Spark ETL jobs in the following Amazon Web Services Regions: US East (Ohio), US East (N. Virginia), US West (Oregon), Asia Pacific (Singapore), Asia Pacific (Sydney), Asia Pacific (Tokyo), Canada (Central), Europe (Frankfurt), Europe (Ireland), and Europe (Stockholm).</p>
            </li>
            <li>
               <p>For the <code>G.8X</code> worker type, each worker maps to 8 DPU (32 vCPUs, 128 GB of memory) with 512GB disk, and provides 1 executor per worker. We recommend this worker type for jobs whose workloads contain your most demanding transforms, aggregations, joins, and queries. This worker type is available only for Glue version 3.0 or later Spark ETL jobs, in the same Amazon Web Services Regions as supported for the <code>G.4X</code> worker type.</p>
            </li>
            <li>
               <p>For the <code>Z.2X</code> worker type, each worker maps to 2 M-DPU (8vCPUs, 64 GB of memory) with 128 GB disk, and provides up to 8 Ray workers based on the autoscaler.</p>
            </li>
         </ul> |
| `id` | String | ✅ | <p>The ID of the session request. </p> |
| `request_origin` | String |  | <p>The origin of the request. </p> |
| `security_configuration` | String |  | <p>The name of the SecurityConfiguration structure to be used with the session </p> |
| `tags` | HashMap<String, String> |  | <p>The map of key value pairs (tags) belonging to the session.</p> |
| `connections` | String |  | <p>The number of connections to use for the session. </p> |
| `idle_timeout` | i64 |  | <p>
        The number of minutes when idle before session times out. Default for
        Spark ETL jobs is value of Timeout. Consult the documentation
        for other job types.
    </p> |
| `timeout` | i64 |  | <p>
        The number of minutes before session times out. Default for Spark ETL
        jobs is 48 hours (2880 minutes).
        Consult the documentation for other job types.
    </p> |
| `max_capacity` | f64 |  | <p>The number of Glue data processing units (DPUs) that can be allocated when the job runs. 
      A DPU is a relative measure of processing power that consists of 4 vCPUs of compute capacity and 16 GB memory. </p> |
| `command` | String | ✅ | <p>The <code>SessionCommand</code> that runs the job. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `session` | String | <p>The session object is returned in the response.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create session
session = provider.glue.Session {
    role = "value"  # <p>The IAM Role ARN </p>
    id = "value"  # <p>The ID of the session request. </p>
    command = "value"  # <p>The <code>SessionCommand</code> that runs the job. </p>
}

# Access session outputs
session_id = session.id
session_session = session.session
```

---


### Triggers

Triggers resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>A continuation token, if not all the requested triggers
      have yet been returned.</p> |
| `triggers` | Vec<String> | <p>A list of triggers for the specified job.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access triggers outputs
triggers_id = triggers.id
triggers_next_token = triggers.next_token
triggers_triggers = triggers.triggers
```

---


### Crawler_schedule

CrawlerSchedule resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `schedule` | String |  | <p>The updated <code>cron</code> expression used to specify the schedule (see <a href="https://docs.aws.amazon.com/glue/latest/dg/monitor-data-warehouse-schedule.html">Time-Based Schedules for Jobs and Crawlers</a>. For example, to run
      something every day at 12:15 UTC, you would specify:
      <code>cron(15 12 * * ? *)</code>.</p> |
| `crawler_name` | String | ✅ | <p>The name of the crawler whose schedule to update.</p> |



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


### Database

Database resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `database_input` | String | ✅ | <p>The metadata for the database.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags you assign to the database.</p> |
| `catalog_id` | String |  | <p>The ID of the Data Catalog in which to create the database. If none is provided, the Amazon Web Services
      account ID is used by default.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `database` | String | <p>The definition of the specified database in the Data Catalog.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create database
database = provider.glue.Database {
    database_input = "value"  # <p>The metadata for the database.</p>
}

# Access database outputs
database_id = database.id
database_database = database.database
```

---


### Integration

Integration resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `integration_config` | String |  | <p>The configuration settings.</p> |
| `source_arn` | String | ✅ | <p>The ARN of the source resource for the integration.</p> |
| `target_arn` | String | ✅ | <p>The ARN of the target resource for the integration.</p> |
| `description` | String |  | <p>A description of the integration.</p> |
| `tags` | Vec<String> |  | <p>Metadata assigned to the resource consisting of a list of key-value pairs.</p> |
| `kms_key_id` | String |  | <p>The ARN of a KMS key used for encrypting the channel.</p> |
| `additional_encryption_context` | HashMap<String, String> |  | <p>An optional set of non-secret key–value pairs that contains additional contextual information for encryption. This can only be provided if <code>KMSKeyId</code> is provided.</p> |
| `integration_name` | String | ✅ | <p>A unique name for an integration in Glue.</p> |
| `data_filter` | String |  | <p>Selects source tables for the integration using Maxwell filter syntax.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create integration
integration = provider.glue.Integration {
    source_arn = "value"  # <p>The ARN of the source resource for the integration.</p>
    target_arn = "value"  # <p>The ARN of the target resource for the integration.</p>
    integration_name = "value"  # <p>A unique name for an integration in Glue.</p>
}

```

---


### Partition_index

PartitionIndex resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `catalog_id` | String |  | <p>The catalog ID where the table resides.</p> |
| `table_name` | String | ✅ | <p>Specifies the name of a table in which you want to create a partition index.</p> |
| `partition_index` | String | ✅ | <p>Specifies a <code>PartitionIndex</code> structure to create a partition index in an existing table.</p> |
| `database_name` | String | ✅ | <p>Specifies the name of a database in which you want to create a partition index.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create partition_index
partition_index = provider.glue.Partition_index {
    table_name = "value"  # <p>Specifies the name of a table in which you want to create a partition index.</p>
    partition_index = "value"  # <p>Specifies a <code>PartitionIndex</code> structure to create a partition index in an existing table.</p>
    database_name = "value"  # <p>Specifies the name of a database in which you want to create a partition index.</p>
}

```

---


### Partitions

Partitions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>A continuation token, if the returned list of partitions does not include the last
      one.</p> |
| `partitions` | Vec<String> | <p>A list of requested partitions.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access partitions outputs
partitions_id = partitions.id
partitions_next_token = partitions.next_token
partitions_partitions = partitions.partitions
```

---


### Crawlers

Crawlers resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `crawlers` | Vec<String> | <p>A list of crawler metadata.</p> |
| `next_token` | String | <p>A continuation token, if the returned list has not reached the end
      of those defined in this customer account.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access crawlers outputs
crawlers_id = crawlers.id
crawlers_crawlers = crawlers.crawlers
crawlers_next_token = crawlers.next_token
```

---


### Schema_by_definition

SchemaByDefinition resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `schema_version_id` | String | <p>The schema ID of the schema version.</p> |
| `data_format` | String | <p>The data format of the schema definition. Currently <code>AVRO</code>, <code>JSON</code> and <code>PROTOBUF</code> are supported.</p> |
| `schema_arn` | String | <p>The Amazon Resource Name (ARN) of the schema.</p> |
| `status` | String | <p>The status of the schema version.</p> |
| `created_time` | String | <p>The date and time the schema was created.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access schema_by_definition outputs
schema_by_definition_id = schema_by_definition.id
schema_by_definition_schema_version_id = schema_by_definition.schema_version_id
schema_by_definition_data_format = schema_by_definition.data_format
schema_by_definition_schema_arn = schema_by_definition.schema_arn
schema_by_definition_status = schema_by_definition.status
schema_by_definition_created_time = schema_by_definition.created_time
```

---


### Connection_type

ConnectionType resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `athena_connection_properties` | HashMap<String, String> | <p>Connection properties specific to the Athena compute environment.</p> |
| `description` | String | <p>A description of the connection type.</p> |
| `python_connection_properties` | HashMap<String, String> | <p>Connection properties specific to the Python compute environment.</p> |
| `spark_connection_properties` | HashMap<String, String> | <p>Connection properties specific to the Spark compute environment.</p> |
| `connection_type` | String | <p>The name of the connection type.</p> |
| `connection_options` | HashMap<String, String> | <p>Returns properties that can be set when creating a connection in the <code>ConnectionInput.ConnectionProperties</code>. <code>ConnectionOptions</code> defines parameters that can be set in a Spark ETL script in the connection options map passed to a dataframe.</p> |
| `capabilities` | String | <p>The supported authentication types, data interface types (compute environments), and data operations of the connector.</p> |
| `connection_properties` | HashMap<String, String> | <p>Connection properties which are common across compute environments.</p> |
| `authentication_configuration` | String | <p>The type of authentication used for the connection.</p> |
| `compute_environment_configurations` | HashMap<String, String> | <p>The compute environments that are supported by the connection.</p> |
| `physical_connection_requirements` | HashMap<String, String> | <p>Physical requirements for a connection, such as VPC, Subnet and Security Group specifications.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access connection_type outputs
connection_type_id = connection_type.id
connection_type_athena_connection_properties = connection_type.athena_connection_properties
connection_type_description = connection_type.description
connection_type_python_connection_properties = connection_type.python_connection_properties
connection_type_spark_connection_properties = connection_type.spark_connection_properties
connection_type_connection_type = connection_type.connection_type
connection_type_connection_options = connection_type.connection_options
connection_type_capabilities = connection_type.capabilities
connection_type_connection_properties = connection_type.connection_properties
connection_type_authentication_configuration = connection_type.authentication_configuration
connection_type_compute_environment_configurations = connection_type.compute_environment_configurations
connection_type_physical_connection_requirements = connection_type.physical_connection_requirements
```

---


### Blueprint_runs

BlueprintRuns resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>A continuation token, if not all blueprint runs have been returned.</p> |
| `blueprint_runs` | Vec<String> | <p>Returns a list of <code>BlueprintRun</code> objects.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access blueprint_runs outputs
blueprint_runs_id = blueprint_runs.id
blueprint_runs_next_token = blueprint_runs.next_token
blueprint_runs_blueprint_runs = blueprint_runs.blueprint_runs
```

---


### Job_bookmark

JobBookmark resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `job_bookmark_entry` | String | <p>A structure that defines a point that a job can resume processing.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access job_bookmark outputs
job_bookmark_id = job_bookmark.id
job_bookmark_job_bookmark_entry = job_bookmark.job_bookmark_entry
```

---


### Job_from_source_control

JobFromSourceControl resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `folder` | String |  | <p>An optional folder in the remote repository.</p> |
| `repository_owner` | String |  | <p>The owner of the remote repository that contains the job artifacts.</p> |
| `auth_strategy` | String |  | <p>The type of authentication, which can be an authentication token stored in Amazon Web Services Secrets Manager, or a personal access token.</p> |
| `auth_token` | String |  | <p>The value of the authorization token.</p> |
| `job_name` | String |  | <p>The name of the Glue job to be synchronized to or from the remote repository.</p> |
| `commit_id` | String |  | <p>A commit ID for a commit in the remote repository.</p> |
| `provider` | String |  | <p>
      The provider for the remote repository. Possible values: GITHUB, AWS_CODE_COMMIT, GITLAB, BITBUCKET.
    </p> |
| `repository_name` | String |  | <p>The name of the remote repository that contains the job artifacts. 
      For BitBucket providers, <code>RepositoryName</code> should include <code>WorkspaceName</code>.
      Use the format <code><WorkspaceName>/<RepositoryName></code>. 
    </p> |
| `branch_name` | String |  | <p>An optional branch in the remote repository.</p> |



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


### Security_configuration

SecurityConfiguration resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>The name for the new security configuration.</p> |
| `encryption_configuration` | String | ✅ | <p>The encryption configuration for the new security configuration.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `security_configuration` | String | <p>The requested security configuration.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create security_configuration
security_configuration = provider.glue.Security_configuration {
    name = "value"  # <p>The name for the new security configuration.</p>
    encryption_configuration = "value"  # <p>The encryption configuration for the new security configuration.</p>
}

# Access security_configuration outputs
security_configuration_id = security_configuration.id
security_configuration_security_configuration = security_configuration.security_configuration
```

---


### Glue_identity_center_configuration

GlueIdentityCenterConfiguration resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `instance_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the Identity Center instance to be associated with the Glue configuration.</p> |
| `user_background_sessions_enabled` | bool |  | <p>Specifies whether users can run background sessions when using Identity Center authentication with Glue services.</p> |
| `scopes` | Vec<String> |  | <p>A list of Identity Center scopes that define the permissions and access levels for the Glue configuration.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `instance_arn` | String | <p>The Amazon Resource Name (ARN) of the Identity Center instance associated with the Glue configuration.</p> |
| `scopes` | Vec<String> | <p>A list of Identity Center scopes that define the permissions and access levels for the Glue configuration.</p> |
| `user_background_sessions_enabled` | bool | <p>Indicates whether users can run background sessions when using Identity Center authentication with Glue services.</p> |
| `application_arn` | String | <p>The Amazon Resource Name (ARN) of the Identity Center application associated with the Glue configuration.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create glue_identity_center_configuration
glue_identity_center_configuration = provider.glue.Glue_identity_center_configuration {
    instance_arn = "value"  # <p>The Amazon Resource Name (ARN) of the Identity Center instance to be associated with the Glue configuration.</p>
}

# Access glue_identity_center_configuration outputs
glue_identity_center_configuration_id = glue_identity_center_configuration.id
glue_identity_center_configuration_instance_arn = glue_identity_center_configuration.instance_arn
glue_identity_center_configuration_scopes = glue_identity_center_configuration.scopes
glue_identity_center_configuration_user_background_sessions_enabled = glue_identity_center_configuration.user_background_sessions_enabled
glue_identity_center_configuration_application_arn = glue_identity_center_configuration.application_arn
```

---


### Catalog

Catalog resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | HashMap<String, String> |  | <p>A map array of key-value pairs, not more than 50 pairs. Each key is a UTF-8 string, not less than 1 or more than 128 bytes long. Each value is a UTF-8 string, not more than 256 bytes long. The tags you assign to the catalog.</p> |
| `name` | String | ✅ | <p>The name of the catalog to create.</p> |
| `catalog_input` | String | ✅ | <p>A <code>CatalogInput</code> object that defines the metadata for the catalog.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `catalog` | String | <p>A <code>Catalog</code> object. The definition of the specified catalog in the Glue Data Catalog.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create catalog
catalog = provider.glue.Catalog {
    name = "value"  # <p>The name of the catalog to create.</p>
    catalog_input = "value"  # <p>A <code>CatalogInput</code> object that defines the metadata for the catalog.</p>
}

# Access catalog outputs
catalog_id = catalog.id
catalog_catalog = catalog.catalog
```

---


### Dev_endpoints

DevEndpoints resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `dev_endpoints` | Vec<String> | <p>A list of <code>DevEndpoint</code> definitions.</p> |
| `next_token` | String | <p>A continuation token, if not all <code>DevEndpoint</code> definitions have yet been
      returned.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access dev_endpoints outputs
dev_endpoints_id = dev_endpoints.id
dev_endpoints_dev_endpoints = dev_endpoints.dev_endpoints
dev_endpoints_next_token = dev_endpoints.next_token
```

---


### Ml_task_runs

MLTaskRuns resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>A pagination token, if more results are available.</p> |
| `task_runs` | Vec<String> | <p>A list of task runs that are associated with the transform.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access ml_task_runs outputs
ml_task_runs_id = ml_task_runs.id
ml_task_runs_next_token = ml_task_runs.next_token
ml_task_runs_task_runs = ml_task_runs.task_runs
```

---


### Dev_endpoint

DevEndpoint resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `worker_type` | String |  | <p>The type of predefined worker that is allocated to the development endpoint. Accepts a value of Standard, G.1X, or G.2X.</p>
         <ul>
            <li>
               <p>For the <code>Standard</code> worker type, each worker provides 4 vCPU, 16 GB of memory and a 50GB disk, and 2 executors per worker.</p>
            </li>
            <li>
               <p>For the <code>G.1X</code> worker type, each worker maps to 1 DPU (4 vCPU, 16 GB of memory, 64 GB disk), and provides 1 executor per worker. We recommend this worker type for memory-intensive jobs.</p>
            </li>
            <li>
               <p>For the <code>G.2X</code> worker type, each worker maps to 2 DPU (8 vCPU, 32 GB of memory, 128 GB disk), and provides 1 executor per worker. We recommend this worker type for memory-intensive jobs.</p>
            </li>
         </ul>
         <p>Known issue: when a development endpoint is created with the <code>G.2X</code>
            <code>WorkerType</code> configuration, the Spark drivers for the development endpoint will run on 4 vCPU, 16 GB of memory, and a 64 GB disk. </p> |
| `security_configuration` | String |  | <p>The name of the <code>SecurityConfiguration</code> structure to be used with this
        <code>DevEndpoint</code>.</p> |
| `role_arn` | String | ✅ | <p>The IAM role for the <code>DevEndpoint</code>.</p> |
| `public_keys` | Vec<String> |  | <p>A list of public keys to be used by the development endpoints for authentication. The use
      of this attribute is preferred over a single public key because the public keys allow you to
      have a different private key per client.</p>
         <note>
            <p>If you previously created an endpoint with a public key, you must remove that key to be able
        to set a list of public keys. Call the <code>UpdateDevEndpoint</code> API with the public
        key content in the <code>deletePublicKeys</code> attribute, and the list of new keys in the
          <code>addPublicKeys</code> attribute.</p>
         </note> |
| `extra_jars_s3_path` | String |  | <p>The path to one or more Java <code>.jar</code> files in an S3 bucket that should be loaded
      in your <code>DevEndpoint</code>.</p> |
| `arguments` | HashMap<String, String> |  | <p>A map of arguments used to configure the <code>DevEndpoint</code>.</p> |
| `endpoint_name` | String | ✅ | <p>The name to be assigned to the new <code>DevEndpoint</code>.</p> |
| `glue_version` | String |  | <p>Glue version determines the versions of Apache Spark and Python that Glue supports. The Python version indicates the version supported for running your ETL scripts on development endpoints. </p>
         <p>For more information about the available Glue versions and corresponding Spark and Python versions, see <a href="https://docs.aws.amazon.com/glue/latest/dg/add-job.html">Glue version</a> in the developer guide.</p>
         <p>Development endpoints that are created without specifying a Glue version default to Glue 0.9.</p>
         <p>You can specify a version of Python support for development endpoints by using the <code>Arguments</code> parameter in the <code>CreateDevEndpoint</code> or <code>UpdateDevEndpoint</code> APIs. If no arguments are provided, the version defaults to Python 2.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags to use with this DevEndpoint. You may use tags to limit access to the DevEndpoint. For more information about tags in Glue, see <a href="https://docs.aws.amazon.com/glue/latest/dg/monitor-tags.html">Amazon Web Services Tags in Glue</a> in the developer guide.</p> |
| `security_group_ids` | String |  | <p>Security group IDs for the security groups to be used by the new
      <code>DevEndpoint</code>.</p> |
| `public_key` | String |  | <p>The public key to be used by this <code>DevEndpoint</code> for authentication. This
      attribute is provided for backward compatibility because the recommended attribute to use is
      public keys.</p> |
| `number_of_workers` | i64 |  | <p>The number of workers of a defined <code>workerType</code> that are allocated to the development endpoint.</p>
         <p>The maximum number of workers you can define are 299 for <code>G.1X</code>, and 149 for <code>G.2X</code>. </p> |
| `extra_python_libs_s3_path` | String |  | <p>The paths to one or more Python libraries in an Amazon S3 bucket that should be loaded in
      your <code>DevEndpoint</code>. Multiple values must be complete paths separated by a
      comma.</p>
         <note>
            <p>You can only use pure Python libraries with a <code>DevEndpoint</code>. Libraries that rely on
        C extensions, such as the <a href="http://pandas.pydata.org/">pandas</a> Python data
        analysis library, are not yet supported.</p>
         </note> |
| `number_of_nodes` | i64 |  | <p>The number of Glue Data Processing Units (DPUs) to allocate to this
        <code>DevEndpoint</code>.</p> |
| `subnet_id` | String |  | <p>The subnet ID for the new <code>DevEndpoint</code> to use.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `dev_endpoint` | String | <p>A <code>DevEndpoint</code> definition.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create dev_endpoint
dev_endpoint = provider.glue.Dev_endpoint {
    role_arn = "value"  # <p>The IAM role for the <code>DevEndpoint</code>.</p>
    endpoint_name = "value"  # <p>The name to be assigned to the new <code>DevEndpoint</code>.</p>
}

# Access dev_endpoint outputs
dev_endpoint_id = dev_endpoint.id
dev_endpoint_dev_endpoint = dev_endpoint.dev_endpoint
```

---


### Workflow_run_properties

WorkflowRunProperties resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `run_properties` | HashMap<String, String> | ✅ | <p>The properties to put for the specified run.</p>
         <p>Run properties may be logged. Do not pass plaintext secrets as properties. Retrieve secrets from a Glue Connection, Amazon Web Services Secrets Manager or other secret management mechanism if you intend to use them within the workflow run.</p> |
| `name` | String | ✅ | <p>Name of the workflow which was run.</p> |
| `run_id` | String | ✅ | <p>The ID of the workflow run for which the run properties should be updated.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `run_properties` | HashMap<String, String> | <p>The workflow run properties which were set during the specified run.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create workflow_run_properties
workflow_run_properties = provider.glue.Workflow_run_properties {
    run_properties = "value"  # <p>The properties to put for the specified run.</p>
         <p>Run properties may be logged. Do not pass plaintext secrets as properties. Retrieve secrets from a Glue Connection, Amazon Web Services Secrets Manager or other secret management mechanism if you intend to use them within the workflow run.</p>
    name = "value"  # <p>Name of the workflow which was run.</p>
    run_id = "value"  # <p>The ID of the workflow run for which the run properties should be updated.</p>
}

# Access workflow_run_properties outputs
workflow_run_properties_id = workflow_run_properties.id
workflow_run_properties_run_properties = workflow_run_properties.run_properties
```

---


### Column_statistics_task_runs

ColumnStatisticsTaskRuns resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>A continuation token, if not all task runs have yet been returned.</p> |
| `column_statistics_task_runs` | Vec<String> | <p>A list of column statistics task runs.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access column_statistics_task_runs outputs
column_statistics_task_runs_id = column_statistics_task_runs.id
column_statistics_task_runs_next_token = column_statistics_task_runs.next_token
column_statistics_task_runs_column_statistics_task_runs = column_statistics_task_runs.column_statistics_task_runs
```

---


### Job

Job resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `code_gen_configuration_nodes` | HashMap<String, String> |  | <p>The representation of a directed acyclic graph on which both the Glue Studio visual component and Glue Studio code generation is based.</p> |
| `default_arguments` | HashMap<String, String> |  | <p>The default arguments for every run of this job, specified as name-value pairs.</p>
         <p>You can specify arguments here that your own job-execution script
      consumes, as well as arguments that Glue itself consumes.</p>
         <p>Job arguments may be logged. Do not pass plaintext secrets as arguments. Retrieve secrets
      from a Glue Connection, Secrets Manager or other secret management
      mechanism if you intend to keep them within the Job. </p>
         <p>For information about how to specify and consume your own Job arguments, see the <a href="https://docs.aws.amazon.com/glue/latest/dg/aws-glue-programming-python-calling.html">Calling Glue APIs in Python</a> topic in the developer guide.</p>
         <p>For information about the arguments you can provide to this field when configuring Spark jobs,
     see the <a href="https://docs.aws.amazon.com/glue/latest/dg/aws-glue-programming-etl-glue-arguments.html">Special Parameters Used by Glue</a> topic in the developer guide.</p>
         <p>For information about the arguments you can provide to this field when configuring Ray
      jobs, see <a href="https://docs.aws.amazon.com/glue/latest/dg/author-job-ray-job-parameters.html">Using
      job parameters in Ray jobs</a> in the developer guide.</p> |
| `connections` | String |  | <p>The connections used for this job.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags to use with this job. You may use tags to limit access to the job. For more information about tags in Glue, see <a href="https://docs.aws.amazon.com/glue/latest/dg/monitor-tags.html">Amazon Web Services Tags in Glue</a> in the developer guide.</p> |
| `name` | String | ✅ | <p>The name you assign to this job definition. It must be unique in your account.</p> |
| `description` | String |  | <p>Description of the job being defined.</p> |
| `security_configuration` | String |  | <p>The name of the <code>SecurityConfiguration</code> structure to be used with this
      job.</p> |
| `role` | String | ✅ | <p>The name or Amazon Resource Name (ARN) of the IAM role associated with this job.</p> |
| `max_capacity` | f64 |  | <p>For Glue version 1.0 or earlier jobs, using the standard worker type, the number of
      Glue data processing units (DPUs) that can be allocated when this job runs. A DPU is
      a relative measure of processing power that consists of 4 vCPUs of compute capacity and 16 GB
      of memory. For more information, see the <a href="https://aws.amazon.com/glue/pricing/">
      Glue pricing page</a>.</p>
         <p>For Glue version 2.0+ jobs, you cannot specify a <code>Maximum capacity</code>.
      Instead, you should specify a <code>Worker type</code> and the <code>Number of workers</code>.</p>
         <p>Do not set <code>MaxCapacity</code> if using <code>WorkerType</code> and <code>NumberOfWorkers</code>.</p>
         <p>The value that can be allocated for <code>MaxCapacity</code> depends on whether you are
      running a Python shell job, an Apache Spark ETL job, or an Apache Spark streaming ETL
      job:</p>
         <ul>
            <li>
               <p>When you specify a Python shell job (<code>JobCommand.Name</code>="pythonshell"), you can
          allocate either 0.0625 or 1 DPU. The default is 0.0625 DPU.</p>
            </li>
            <li>
               <p>When you specify an Apache Spark ETL job (<code>JobCommand.Name</code>="glueetl") or Apache 
        Spark streaming ETL job (<code>JobCommand.Name</code>="gluestreaming"), you can allocate from 2 to 100 DPUs. 
        The default is 10 DPUs. This job type cannot have a fractional DPU allocation.</p>
            </li>
         </ul> |
| `glue_version` | String |  | <p>In Spark jobs, <code>GlueVersion</code> determines the versions of Apache Spark and Python
      that Glue available in a job. The Python version indicates the version
      supported for jobs of type Spark. </p>
         <p>Ray jobs should set <code>GlueVersion</code> to <code>4.0</code> or greater. However,
    the versions of Ray, Python and additional libraries available in your Ray job are determined
    by the <code>Runtime</code> parameter of the Job command.</p>
         <p>For more information about the available Glue versions and corresponding
      Spark and Python versions, see <a href="https://docs.aws.amazon.com/glue/latest/dg/add-job.html">Glue version</a> in the developer
      guide.</p>
         <p>Jobs that are created without specifying a Glue version default to Glue 0.9.</p> |
| `execution_property` | String |  | <p>An <code>ExecutionProperty</code> specifying the maximum number of concurrent runs allowed
      for this job.</p> |
| `worker_type` | String |  | <p>The type of predefined worker that is allocated when a job runs. Accepts a value of
      G.1X, G.2X, G.4X, G.8X or G.025X for Spark jobs. Accepts the value Z.2X for Ray jobs.</p>
         <ul>
            <li>
               <p>For the <code>G.1X</code> worker type, each worker maps to 1 DPU (4 vCPUs, 16 GB of memory) with 94GB disk, and provides 1 executor per worker. We recommend this worker type for workloads such as data transforms, joins, and queries, to offers a scalable and cost effective way to run most jobs.</p>
            </li>
            <li>
               <p>For the <code>G.2X</code> worker type, each worker maps to 2 DPU (8 vCPUs, 32 GB of memory) with 138GB disk, and provides 1 executor per worker. We recommend this worker type for workloads such as data transforms, joins, and queries, to offers a scalable and cost effective way to run most jobs.</p>
            </li>
            <li>
               <p>For the <code>G.4X</code> worker type, each worker maps to 4 DPU (16 vCPUs, 64 GB of memory) with 256GB disk, and provides 1 executor per worker. We recommend this worker type for jobs whose workloads contain your most demanding transforms, aggregations, joins, and queries. This worker type is available only for Glue version 3.0 or later Spark ETL jobs in the following Amazon Web Services Regions: US East (Ohio), US East (N. Virginia), US West (N. California), US West (Oregon), Asia Pacific (Mumbai), Asia Pacific (Seoul), Asia Pacific (Singapore), Asia Pacific (Sydney), Asia Pacific (Tokyo), Canada (Central), Europe (Frankfurt), Europe (Ireland), Europe (London), Europe (Spain), Europe (Stockholm), and South America (São Paulo).</p>
            </li>
            <li>
               <p>For the <code>G.8X</code> worker type, each worker maps to 8 DPU (32 vCPUs, 128 GB of memory) with 512GB disk, and provides 1 executor per worker. We recommend this worker type for jobs whose workloads contain your most demanding transforms, aggregations, joins, and queries. This worker type is available only for Glue version 3.0 or later Spark ETL jobs, in the same Amazon Web Services Regions as supported for the <code>G.4X</code> worker type.</p>
            </li>
            <li>
               <p>For the <code>G.025X</code> worker type, each worker maps to 0.25 DPU (2 vCPUs, 4 GB of memory) with 84GB disk, and provides 1 executor per worker. We recommend this worker type for low volume streaming jobs. This worker type is only available for Glue version 3.0 or later streaming jobs.</p>
            </li>
            <li>
               <p>For the <code>Z.2X</code> worker type, each worker maps to 2 M-DPU (8vCPUs, 64 GB of memory) with 128 GB disk, and provides up to 8 Ray workers based on the autoscaler.</p>
            </li>
         </ul> |
| `job_mode` | String |  | <p>A mode that describes how a job was created. Valid values are:</p>
         <ul>
            <li>
               <p>
                  <code>SCRIPT</code> - The job was created using the Glue Studio script editor.</p>
            </li>
            <li>
               <p>
                  <code>VISUAL</code> - The job was created using the Glue Studio visual editor.</p>
            </li>
            <li>
               <p>
                  <code>NOTEBOOK</code> - The job was created using an interactive sessions notebook.</p>
            </li>
         </ul>
         <p>When the <code>JobMode</code> field is missing or null, <code>SCRIPT</code> is assigned as the default value.</p> |
| `maintenance_window` | String |  | <p>This field specifies a day of the week and hour for a maintenance window for streaming jobs. Glue periodically performs maintenance activities. During these maintenance windows, Glue will need to restart your streaming jobs.</p>
         <p>Glue will restart the job within 3 hours of the specified maintenance window. For instance, if you set up the maintenance window for Monday at 10:00AM GMT, your jobs will be restarted between 10:00AM GMT to 1:00PM GMT.</p> |
| `command` | String | ✅ | <p>The <code>JobCommand</code> that runs this job.</p> |
| `log_uri` | String |  | <p>This field is reserved for future use.</p> |
| `non_overridable_arguments` | HashMap<String, String> |  | <p>Arguments for this job that are not overridden when providing job arguments
      in a job run, specified as name-value pairs.</p> |
| `job_run_queuing_enabled` | bool |  | <p>Specifies whether job run queuing is enabled for the job runs for this job.</p>
         <p>A value of true means job run queuing is enabled for the job runs. If false or not populated, the job runs will not be considered for queueing.</p>
         <p>If this field does not match the value set in the job run, then the value from the job run field will be used.</p> |
| `notification_property` | String |  | <p>Specifies configuration properties of a job notification.</p> |
| `allocated_capacity` | i64 |  | <p>This parameter is deprecated. Use <code>MaxCapacity</code> instead.</p>
         <p>The number of Glue data processing units (DPUs) to allocate to this Job. You can
      allocate a minimum of 2 DPUs; the default is 10. A DPU is a relative measure of processing
      power that consists of 4 vCPUs of compute capacity and 16 GB of memory. For more information,
      see the <a href="https://aws.amazon.com/glue/pricing/">Glue pricing
      page</a>.</p> |
| `timeout` | i64 |  | <p>The job timeout in minutes.  This is the maximum time that a job run
      can consume resources before it is terminated and enters <code>TIMEOUT</code>
      status.</p>
         <p>Jobs must have timeout values less than 7 days or 10080 minutes. Otherwise, the jobs will throw an exception.</p>
         <p>When the value is left blank, the timeout is defaulted to 2880 minutes.</p>
         <p>Any existing Glue jobs that had a timeout value greater than 7 days will be defaulted to 7 days. For instance if you have specified a timeout of 20 days for a batch job, it will be stopped on the 7th day.</p>
         <p>For streaming jobs, if you have set up a maintenance window, it will be restarted during the maintenance window after 7 days.</p> |
| `execution_class` | String |  | <p>Indicates whether the job is run with a standard or flexible execution class. The standard execution-class is ideal for time-sensitive workloads that require fast job startup and dedicated resources.</p>
         <p>The flexible execution class is appropriate for time-insensitive jobs whose start and completion times may vary. </p>
         <p>Only jobs with Glue version 3.0 and above and command type <code>glueetl</code> will be allowed to set <code>ExecutionClass</code> to <code>FLEX</code>. The flexible execution class is available for Spark jobs.</p> |
| `source_control_details` | String |  | <p>The details for a source control configuration for a job, allowing synchronization of job artifacts to or from a remote repository.</p> |
| `number_of_workers` | i64 |  | <p>The number of workers of a defined <code>workerType</code> that are allocated when a job runs.</p> |
| `max_retries` | i64 |  | <p>The maximum number of times to retry this job if it fails.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `job` | String | <p>The requested job definition.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create job
job = provider.glue.Job {
    name = "value"  # <p>The name you assign to this job definition. It must be unique in your account.</p>
    role = "value"  # <p>The name or Amazon Resource Name (ARN) of the IAM role associated with this job.</p>
    command = "value"  # <p>The <code>JobCommand</code> that runs this job.</p>
}

# Access job outputs
job_id = job.id
job_job = job.job
```

---


### Jobs

Jobs resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>A continuation token, if not all job definitions have yet been returned.</p> |
| `jobs` | Vec<String> | <p>A list of job definitions.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access jobs outputs
jobs_id = jobs.id
jobs_next_token = jobs.next_token
jobs_jobs = jobs.jobs
```

---


### Table_versions

TableVersions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `table_versions` | Vec<String> | <p>A list of strings identifying available versions of the
      specified table.</p> |
| `next_token` | String | <p>A continuation token, if the list of available versions does
      not include the last one.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access table_versions outputs
table_versions_id = table_versions.id
table_versions_table_versions = table_versions.table_versions
table_versions_next_token = table_versions.next_token
```

---


### Workflow

Workflow resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>The name to be assigned to the workflow. It should be unique within your account.</p> |
| `default_run_properties` | HashMap<String, String> |  | <p>A collection of properties to be used as part of each execution of the workflow.</p>
         <p>Run properties may be logged. Do not pass plaintext secrets as properties. Retrieve secrets from a Glue Connection, Amazon Web Services Secrets Manager or other secret management mechanism if you intend to use them within the workflow run.</p> |
| `description` | String |  | <p>A description of the workflow.</p> |
| `max_concurrent_runs` | i64 |  | <p>You can use this parameter to prevent unwanted multiple updates to data, to control costs, or in some cases, to prevent exceeding the maximum number of concurrent runs of any of the component jobs. If you leave this parameter blank, there is no limit to the number of concurrent workflow runs.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags to be used with this workflow.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `workflow` | String | <p>The resource metadata for the workflow.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create workflow
workflow = provider.glue.Workflow {
    name = "value"  # <p>The name to be assigned to the workflow. It should be unique within your account.</p>
}

# Access workflow outputs
workflow_id = workflow.id
workflow_workflow = workflow.workflow
```

---


### Data_quality_ruleset

DataQualityRuleset resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | HashMap<String, String> |  | <p>A list of tags applied to the data quality ruleset.</p> |
| `name` | String | ✅ | <p>A unique name for the data quality ruleset.</p> |
| `client_token` | String |  | <p>Used for idempotency and is recommended to be set to a random ID (such as a UUID) to avoid creating or starting multiple instances of the same resource.</p> |
| `description` | String |  | <p>A description of the data quality ruleset.</p> |
| `ruleset` | String | ✅ | <p>A Data Quality Definition Language (DQDL) ruleset. For more information, see the Glue developer guide.</p> |
| `target_table` | String |  | <p>A target table associated with the data quality ruleset.</p> |
| `data_quality_security_configuration` | String |  | <p>The name of the security configuration created with the data quality encryption option.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `target_table` | String | <p>The name and database name of the target table.</p> |
| `last_modified_on` | String | <p>A timestamp. The last point in time when this data quality ruleset was modified.</p> |
| `recommendation_run_id` | String | <p>When a ruleset was created from a recommendation run, this run ID is generated to link the two together.</p> |
| `data_quality_security_configuration` | String | <p>The name of the security configuration created with the data quality encryption option.</p> |
| `ruleset` | String | <p>A Data Quality Definition Language (DQDL) ruleset. For more information, see the Glue developer guide.</p> |
| `created_on` | String | <p>A timestamp. The time and date that this data quality ruleset was created.</p> |
| `name` | String | <p>The name of the ruleset.</p> |
| `description` | String | <p>A description of the ruleset.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create data_quality_ruleset
data_quality_ruleset = provider.glue.Data_quality_ruleset {
    name = "value"  # <p>A unique name for the data quality ruleset.</p>
    ruleset = "value"  # <p>A Data Quality Definition Language (DQDL) ruleset. For more information, see the Glue developer guide.</p>
}

# Access data_quality_ruleset outputs
data_quality_ruleset_id = data_quality_ruleset.id
data_quality_ruleset_target_table = data_quality_ruleset.target_table
data_quality_ruleset_last_modified_on = data_quality_ruleset.last_modified_on
data_quality_ruleset_recommendation_run_id = data_quality_ruleset.recommendation_run_id
data_quality_ruleset_data_quality_security_configuration = data_quality_ruleset.data_quality_security_configuration
data_quality_ruleset_ruleset = data_quality_ruleset.ruleset
data_quality_ruleset_created_on = data_quality_ruleset.created_on
data_quality_ruleset_name = data_quality_ruleset.name
data_quality_ruleset_description = data_quality_ruleset.description
```

---


### Tags

Tags resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `tags` | HashMap<String, String> | <p>The requested tags.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access tags outputs
tags_id = tags.id
tags_tags = tags.tags
```

---


### Schema_versions

SchemaVersions resource

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


### User_defined_functions

UserDefinedFunctions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>A continuation token, if the list of functions returned does
      not include the last requested function.</p> |
| `user_defined_functions` | Vec<String> | <p>A list of requested function definitions.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access user_defined_functions outputs
user_defined_functions_id = user_defined_functions.id
user_defined_functions_next_token = user_defined_functions.next_token
user_defined_functions_user_defined_functions = user_defined_functions.user_defined_functions
```

---


### Workflow_run

WorkflowRun resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `run` | String | <p>The requested workflow run metadata.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access workflow_run outputs
workflow_run_id = workflow_run.id
workflow_run_run = workflow_run.run
```

---


### Column_statistics_task_settings

ColumnStatisticsTaskSettings resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `security_configuration` | String |  | <p>Name of the security configuration that is used to encrypt CloudWatch logs.</p> |
| `tags` | HashMap<String, String> |  | <p>A map of tags.</p> |
| `sample_size` | f64 |  | <p>The percentage of data to sample.</p> |
| `table_name` | String | ✅ | <p>The name of the table for which to generate column statistics.</p> |
| `role` | String | ✅ | <p>The role used for running the column statistics.</p> |
| `database_name` | String | ✅ | <p>The name of the database where the table resides.</p> |
| `column_name_list` | Vec<String> |  | <p>A list of column names for which to run statistics.</p> |
| `catalog_id` | String |  | <p>The ID of the Data Catalog in which the database resides.</p> |
| `schedule` | String |  | <p>A schedule for running the column statistics, specified in CRON syntax.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `column_statistics_task_settings` | String | <p>A <code>ColumnStatisticsTaskSettings</code> object representing the settings for the column statistics task.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create column_statistics_task_settings
column_statistics_task_settings = provider.glue.Column_statistics_task_settings {
    table_name = "value"  # <p>The name of the table for which to generate column statistics.</p>
    role = "value"  # <p>The role used for running the column statistics.</p>
    database_name = "value"  # <p>The name of the database where the table resides.</p>
}

# Access column_statistics_task_settings outputs
column_statistics_task_settings_id = column_statistics_task_settings.id
column_statistics_task_settings_column_statistics_task_settings = column_statistics_task_settings.column_statistics_task_settings
```

---


### Schema_versions_diff

SchemaVersionsDiff resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `diff` | String | <p>The difference between schemas as a string in JsonPatch format.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access schema_versions_diff outputs
schema_versions_diff_id = schema_versions_diff.id
schema_versions_diff_diff = schema_versions_diff.diff
```

---


### Data_quality_result

DataQualityResult resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `score` | f64 | <p>An aggregate data quality score. Represents the ratio of rules that passed to the total number of rules.</p> |
| `ruleset_evaluation_run_id` | String | <p>The unique run ID associated with the ruleset evaluation.</p> |
| `aggregated_metrics` | String | <p> A summary of <code>DataQualityAggregatedMetrics</code> objects showing the total counts of processed rows and rules, including their pass/fail statistics based on row-level results. </p> |
| `evaluation_context` | String | <p>In the context of a job in Glue Studio, each node in the canvas is typically assigned some sort of name and data quality nodes will have names. In the case of multiple nodes, the <code>evaluationContext</code> can differentiate the nodes.</p> |
| `result_id` | String | <p>A unique result ID for the data quality result.</p> |
| `profile_id` | String | <p>The Profile ID for the data quality result.</p> |
| `ruleset_name` | String | <p>The name of the ruleset associated with the data quality result.</p> |
| `analyzer_results` | Vec<String> | <p>A list of <code>DataQualityAnalyzerResult</code> objects representing the results for each analyzer. </p> |
| `data_source` | String | <p>The table associated with the data quality result, if any.</p> |
| `completed_on` | String | <p>The date and time when the run for this data quality result was completed.</p> |
| `observations` | Vec<String> | <p>A list of <code>DataQualityObservation</code> objects representing the observations generated after evaluating the rules and analyzers. </p> |
| `job_name` | String | <p>The job name associated with the data quality result, if any.</p> |
| `started_on` | String | <p>The date and time when the run for this data quality result started.</p> |
| `rule_results` | Vec<String> | <p>A list of <code>DataQualityRuleResult</code> objects representing the results for each rule. </p> |
| `job_run_id` | String | <p>The job run ID associated with the data quality result, if any.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access data_quality_result outputs
data_quality_result_id = data_quality_result.id
data_quality_result_score = data_quality_result.score
data_quality_result_ruleset_evaluation_run_id = data_quality_result.ruleset_evaluation_run_id
data_quality_result_aggregated_metrics = data_quality_result.aggregated_metrics
data_quality_result_evaluation_context = data_quality_result.evaluation_context
data_quality_result_result_id = data_quality_result.result_id
data_quality_result_profile_id = data_quality_result.profile_id
data_quality_result_ruleset_name = data_quality_result.ruleset_name
data_quality_result_analyzer_results = data_quality_result.analyzer_results
data_quality_result_data_source = data_quality_result.data_source
data_quality_result_completed_on = data_quality_result.completed_on
data_quality_result_observations = data_quality_result.observations
data_quality_result_job_name = data_quality_result.job_name
data_quality_result_started_on = data_quality_result.started_on
data_quality_result_rule_results = data_quality_result.rule_results
data_quality_result_job_run_id = data_quality_result.job_run_id
```

---


### Data_quality_ruleset_evaluation_run

DataQualityRulesetEvaluationRun resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `ruleset_names` | Vec<String> | <p>A list of ruleset names for the run. Currently, this parameter takes only one Ruleset name.</p> |
| `last_modified_on` | String | <p>A timestamp. The last point in time when this data quality rule recommendation run was modified.</p> |
| `data_source` | String | <p>The data source (an Glue table) associated with this evaluation run.</p> |
| `status` | String | <p>The status for this run.</p> |
| `additional_run_options` | String | <p>Additional run options you can specify for an evaluation run.</p> |
| `timeout` | i64 | <p>The timeout for a run in minutes. This is the maximum time that a run can consume resources before it is terminated and enters <code>TIMEOUT</code> status. The default is 2,880 minutes (48 hours).</p> |
| `error_string` | String | <p>The error strings that are associated with the run.</p> |
| `run_id` | String | <p>The unique run identifier associated with this run.</p> |
| `completed_on` | String | <p>The date and time when this run was completed.</p> |
| `result_ids` | Vec<String> | <p>A list of result IDs for the data quality results for the run.</p> |
| `execution_time` | i64 | <p>The amount of time (in seconds) that the run consumed resources.</p> |
| `number_of_workers` | i64 | <p>The number of <code>G.1X</code> workers to be used in the run. The default is 5.</p> |
| `started_on` | String | <p>The date and time when this run started.</p> |
| `role` | String | <p>An IAM role supplied to encrypt the results of the run.</p> |
| `additional_data_sources` | HashMap<String, String> | <p>A map of reference strings to additional data sources you can specify for an evaluation run.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access data_quality_ruleset_evaluation_run outputs
data_quality_ruleset_evaluation_run_id = data_quality_ruleset_evaluation_run.id
data_quality_ruleset_evaluation_run_ruleset_names = data_quality_ruleset_evaluation_run.ruleset_names
data_quality_ruleset_evaluation_run_last_modified_on = data_quality_ruleset_evaluation_run.last_modified_on
data_quality_ruleset_evaluation_run_data_source = data_quality_ruleset_evaluation_run.data_source
data_quality_ruleset_evaluation_run_status = data_quality_ruleset_evaluation_run.status
data_quality_ruleset_evaluation_run_additional_run_options = data_quality_ruleset_evaluation_run.additional_run_options
data_quality_ruleset_evaluation_run_timeout = data_quality_ruleset_evaluation_run.timeout
data_quality_ruleset_evaluation_run_error_string = data_quality_ruleset_evaluation_run.error_string
data_quality_ruleset_evaluation_run_run_id = data_quality_ruleset_evaluation_run.run_id
data_quality_ruleset_evaluation_run_completed_on = data_quality_ruleset_evaluation_run.completed_on
data_quality_ruleset_evaluation_run_result_ids = data_quality_ruleset_evaluation_run.result_ids
data_quality_ruleset_evaluation_run_execution_time = data_quality_ruleset_evaluation_run.execution_time
data_quality_ruleset_evaluation_run_number_of_workers = data_quality_ruleset_evaluation_run.number_of_workers
data_quality_ruleset_evaluation_run_started_on = data_quality_ruleset_evaluation_run.started_on
data_quality_ruleset_evaluation_run_role = data_quality_ruleset_evaluation_run.role
data_quality_ruleset_evaluation_run_additional_data_sources = data_quality_ruleset_evaluation_run.additional_data_sources
```

---


### Classifiers

Classifiers resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `classifiers` | Vec<String> | <p>The requested list of classifier
      objects.</p> |
| `next_token` | String | <p>A continuation token.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access classifiers outputs
classifiers_id = classifiers.id
classifiers_classifiers = classifiers.classifiers
classifiers_next_token = classifiers.next_token
```

---


### Workflow_runs

WorkflowRuns resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `runs` | Vec<String> | <p>A list of workflow run metadata objects.</p> |
| `next_token` | String | <p>A continuation token, if not all requested workflow runs have been returned.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access workflow_runs outputs
workflow_runs_id = workflow_runs.id
workflow_runs_runs = workflow_runs.runs
workflow_runs_next_token = workflow_runs.next_token
```

---


### Job_runs

JobRuns resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>A continuation token, if not all requested job runs have been returned.</p> |
| `job_runs` | Vec<String> | <p>A list of job-run metadata objects.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access job_runs outputs
job_runs_id = job_runs.id
job_runs_next_token = job_runs.next_token
job_runs_job_runs = job_runs.job_runs
```

---


### Blueprint

Blueprint resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>The name of the blueprint.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags to be applied to this blueprint.</p> |
| `blueprint_location` | String | ✅ | <p>Specifies a path in Amazon S3 where the blueprint is published.</p> |
| `description` | String |  | <p>A description of the blueprint.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `blueprint` | String | <p>Returns a <code>Blueprint</code> object.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create blueprint
blueprint = provider.glue.Blueprint {
    name = "value"  # <p>The name of the blueprint.</p>
    blueprint_location = "value"  # <p>Specifies a path in Amazon S3 where the blueprint is published.</p>
}

# Access blueprint outputs
blueprint_id = blueprint.id
blueprint_blueprint = blueprint.blueprint
```

---


### Data_quality_profile_annotation

DataQualityProfileAnnotation resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `inclusion_annotation` | String | ✅ | <p>The inclusion annotation value to apply to the profile.</p> |
| `profile_id` | String | ✅ | <p>The ID of the data quality monitoring profile to annotate.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create data_quality_profile_annotation
data_quality_profile_annotation = provider.glue.Data_quality_profile_annotation {
    inclusion_annotation = "value"  # <p>The inclusion annotation value to apply to the profile.</p>
    profile_id = "value"  # <p>The ID of the data quality monitoring profile to annotate.</p>
}

```

---


### Crawler_metrics

CrawlerMetrics resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `crawler_metrics_list` | Vec<String> | <p>A list of metrics for the specified crawler.</p> |
| `next_token` | String | <p>A continuation token, if the returned list does not contain the
      last metric available.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access crawler_metrics outputs
crawler_metrics_id = crawler_metrics.id
crawler_metrics_crawler_metrics_list = crawler_metrics.crawler_metrics_list
crawler_metrics_next_token = crawler_metrics.next_token
```

---


### Source_control_from_job

SourceControlFromJob resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `commit_id` | String |  | <p>A commit ID for a commit in the remote repository.</p> |
| `branch_name` | String |  | <p>An optional branch in the remote repository.</p> |
| `auth_token` | String |  | <p>The value of the authorization token.</p> |
| `folder` | String |  | <p>An optional folder in the remote repository.</p> |
| `job_name` | String |  | <p>The name of the Glue job to be synchronized to or from the remote repository.</p> |
| `auth_strategy` | String |  | <p>The type of authentication, which can be an authentication token stored in Amazon Web Services Secrets Manager, or a personal access token.</p> |
| `repository_name` | String |  | <p>The name of the remote repository that contains the job artifacts. 
      For BitBucket providers, <code>RepositoryName</code> should include <code>WorkspaceName</code>.
      Use the format <code><WorkspaceName>/<RepositoryName></code>. 
    </p> |
| `provider` | String |  | <p>
      The provider for the remote repository. Possible values: GITHUB, AWS_CODE_COMMIT, GITLAB, BITBUCKET.
    </p> |
| `repository_owner` | String |  | <p>The owner of the remote repository that contains the job artifacts.</p> |



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


### Plan

Plan resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `scala_code` | String | <p>The Scala code to perform the mapping.</p> |
| `python_script` | String | <p>A Python script to perform the mapping.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access plan outputs
plan_id = plan.id
plan_scala_code = plan.scala_code
plan_python_script = plan.python_script
```

---


### Connections

Connections resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>A continuation token, if the list of connections returned does not
      include the last of the filtered connections.</p> |
| `connection_list` | Vec<String> | <p>A list of requested connection definitions.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access connections outputs
connections_id = connections.id
connections_next_token = connections.next_token
connections_connection_list = connections.connection_list
```

---


### Inbound_integrations

InboundIntegrations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `inbound_integrations` | Vec<String> | <p>A list of inbound integrations.</p> |
| `marker` | String | <p>A value that indicates the starting point for the next set of response records in a subsequent request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access inbound_integrations outputs
inbound_integrations_id = inbound_integrations.id
inbound_integrations_inbound_integrations = inbound_integrations.inbound_integrations
inbound_integrations_marker = inbound_integrations.marker
```

---


### Column_statistics_task_run

ColumnStatisticsTaskRun resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `column_statistics_task_run` | String | <p>A <code>ColumnStatisticsTaskRun</code> object representing the details of the column stats run.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access column_statistics_task_run outputs
column_statistics_task_run_id = column_statistics_task_run.id
column_statistics_task_run_column_statistics_task_run = column_statistics_task_run.column_statistics_task_run
```

---


### Usage_profile

UsageProfile resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | HashMap<String, String> |  | <p>A list of tags applied to the usage profile.</p> |
| `configuration` | String | ✅ | <p>A <code>ProfileConfiguration</code> object specifying the job and session values for the profile.</p> |
| `name` | String | ✅ | <p>The name of the usage profile.</p> |
| `description` | String |  | <p>A description of the usage profile.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `created_on` | String | <p>The date and time when the usage profile was created.</p> |
| `name` | String | <p>The name of the usage profile.</p> |
| `last_modified_on` | String | <p>The date and time when the usage profile was last modified.</p> |
| `description` | String | <p>A description of the usage profile.</p> |
| `configuration` | String | <p>A <code>ProfileConfiguration</code> object specifying the job and session values for the profile.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create usage_profile
usage_profile = provider.glue.Usage_profile {
    configuration = "value"  # <p>A <code>ProfileConfiguration</code> object specifying the job and session values for the profile.</p>
    name = "value"  # <p>The name of the usage profile.</p>
}

# Access usage_profile outputs
usage_profile_id = usage_profile.id
usage_profile_created_on = usage_profile.created_on
usage_profile_name = usage_profile.name
usage_profile_last_modified_on = usage_profile.last_modified_on
usage_profile_description = usage_profile.description
usage_profile_configuration = usage_profile.configuration
```

---


### Integrations

Integrations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `marker` | String | <p>A value that indicates the starting point for the next set of response records in a subsequent request.</p> |
| `integrations` | Vec<String> | <p>A list of zero-ETL integrations.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access integrations outputs
integrations_id = integrations.id
integrations_marker = integrations.marker
integrations_integrations = integrations.integrations
```

---


### Blueprint_run

BlueprintRun resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `blueprint_run` | String | <p>Returns a <code>BlueprintRun</code> object.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access blueprint_run outputs
blueprint_run_id = blueprint_run.id
blueprint_run_blueprint_run = blueprint_run.blueprint_run
```

---


### User_defined_function

UserDefinedFunction resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `database_name` | String | ✅ | <p>The name of the catalog database in which to create the function.</p> |
| `function_input` | String | ✅ | <p>A <code>FunctionInput</code> object that defines the function
      to create in the Data Catalog.</p> |
| `catalog_id` | String |  | <p>The ID of the Data Catalog in which to create the function. If none is provided, the Amazon Web Services
      account ID is used by default.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `user_defined_function` | String | <p>The requested function definition.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create user_defined_function
user_defined_function = provider.glue.User_defined_function {
    database_name = "value"  # <p>The name of the catalog database in which to create the function.</p>
    function_input = "value"  # <p>A <code>FunctionInput</code> object that defines the function
      to create in the Data Catalog.</p>
}

# Access user_defined_function outputs
user_defined_function_id = user_defined_function.id
user_defined_function_user_defined_function = user_defined_function.user_defined_function
```

---


### Registry

Registry resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | HashMap<String, String> |  | <p>Amazon Web Services tags that contain a key value pair and may be searched by console, command line, or API.</p> |
| `description` | String |  | <p>A description of the registry. If description is not provided, there will not be any default value for this.</p> |
| `registry_name` | String | ✅ | <p>Name of the registry to be created of max length of 255, and may only contain letters, numbers, hyphen, underscore, dollar sign, or hash mark.  No whitespace.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `registry_arn` | String | <p>The Amazon Resource Name (ARN) of the registry.</p> |
| `registry_name` | String | <p>The name of the registry.</p> |
| `status` | String | <p>The status of the registry.</p> |
| `description` | String | <p>A description of the registry.</p> |
| `created_time` | String | <p>The date and time the registry was created.</p> |
| `updated_time` | String | <p>The date and time the registry was updated.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create registry
registry = provider.glue.Registry {
    registry_name = "value"  # <p>Name of the registry to be created of max length of 255, and may only contain letters, numbers, hyphen, underscore, dollar sign, or hash mark.  No whitespace.</p>
}

# Access registry outputs
registry_id = registry.id
registry_registry_arn = registry.registry_arn
registry_registry_name = registry.registry_name
registry_status = registry.status
registry_description = registry.description
registry_created_time = registry.created_time
registry_updated_time = registry.updated_time
```

---


### Ml_task_run

MLTaskRun resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `started_on` | String | <p>The date and time when this task run started.</p> |
| `transform_id` | String | <p>The unique identifier of the task run.</p> |
| `status` | String | <p>The status for this task run.</p> |
| `execution_time` | i64 | <p>The amount of time (in seconds) that the task run consumed resources.</p> |
| `error_string` | String | <p>The error strings that are associated with the task run.</p> |
| `properties` | String | <p>The list of properties that are associated with the task run.</p> |
| `log_group_name` | String | <p>The names of the log groups that are associated with the task run.</p> |
| `last_modified_on` | String | <p>The date and time when this task run was last modified.</p> |
| `completed_on` | String | <p>The date and time when this task run was completed.</p> |
| `task_run_id` | String | <p>The unique run identifier associated with this run.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access ml_task_run outputs
ml_task_run_id = ml_task_run.id
ml_task_run_started_on = ml_task_run.started_on
ml_task_run_transform_id = ml_task_run.transform_id
ml_task_run_status = ml_task_run.status
ml_task_run_execution_time = ml_task_run.execution_time
ml_task_run_error_string = ml_task_run.error_string
ml_task_run_properties = ml_task_run.properties
ml_task_run_log_group_name = ml_task_run.log_group_name
ml_task_run_last_modified_on = ml_task_run.last_modified_on
ml_task_run_completed_on = ml_task_run.completed_on
ml_task_run_task_run_id = ml_task_run.task_run_id
```

---


### Statement

Statement resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `statement` | String | <p>Returns the statement.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access statement outputs
statement_id = statement.id
statement_statement = statement.statement
```

---


### Schema_version_metadata

SchemaVersionMetadata resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `schema_version_id` | String |  | <p>The unique version ID of the schema version.</p> |
| `schema_version_number` | String |  | <p>The version number of the schema.</p> |
| `metadata_key_value` | String | ✅ | <p>The metadata key's corresponding value.</p> |
| `schema_id` | String |  | <p>The unique ID for the schema.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create schema_version_metadata
schema_version_metadata = provider.glue.Schema_version_metadata {
    metadata_key_value = "value"  # <p>The metadata key's corresponding value.</p>
}

```

---


### Integration_table_properties

IntegrationTableProperties resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `target_table_config` | String |  | <p>A structure for the target table configuration.</p> |
| `resource_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the target table for which to create integration table properties. Currently, this API only supports creating 
      integration table properties for target tables, and the provided ARN should be the ARN of the target table in the Glue Data Catalog. Support for 
      creating integration table properties for source connections (using the connection ARN) is not yet implemented and will be added in a future release.
    </p> |
| `table_name` | String | ✅ | <p>The name of the table to be replicated.</p> |
| `source_table_config` | String |  | <p>A structure for the source table configuration. See the <code>SourceTableConfig</code> structure to see list of supported source properties.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `table_name` | String | <p>The name of the table to be replicated.</p> |
| `target_table_config` | String | <p>A structure for the target table configuration.</p> |
| `resource_arn` | String | <p>The Amazon Resource Name (ARN) of the target table for which to retrieve integration table properties. Currently, this API only supports retrieving 
      properties for target tables, and the provided ARN should be the ARN of the target table in the Glue Data Catalog. Support for retrieving integration 
      table properties for source connections (using the connection ARN) is not yet implemented and will be added in a future release.
    </p> |
| `source_table_config` | String | <p>A structure for the source table configuration.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create integration_table_properties
integration_table_properties = provider.glue.Integration_table_properties {
    resource_arn = "value"  # <p>The Amazon Resource Name (ARN) of the target table for which to create integration table properties. Currently, this API only supports creating 
      integration table properties for target tables, and the provided ARN should be the ARN of the target table in the Glue Data Catalog. Support for 
      creating integration table properties for source connections (using the connection ARN) is not yet implemented and will be added in a future release.
    </p>
    table_name = "value"  # <p>The name of the table to be replicated.</p>
}

# Access integration_table_properties outputs
integration_table_properties_id = integration_table_properties.id
integration_table_properties_table_name = integration_table_properties.table_name
integration_table_properties_target_table_config = integration_table_properties.target_table_config
integration_table_properties_resource_arn = integration_table_properties.resource_arn
integration_table_properties_source_table_config = integration_table_properties.source_table_config
```

---


### Data_quality_model

DataQualityModel resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | String | <p>The training status of the data quality model.</p> |
| `failure_reason` | String | <p>The training failure reason.</p> |
| `completed_on` | String | <p>The timestamp when the data quality model training completed.</p> |
| `started_on` | String | <p>The timestamp when the data quality model training started.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access data_quality_model outputs
data_quality_model_id = data_quality_model.id
data_quality_model_status = data_quality_model.status
data_quality_model_failure_reason = data_quality_model.failure_reason
data_quality_model_completed_on = data_quality_model.completed_on
data_quality_model_started_on = data_quality_model.started_on
```

---


### Trigger

Trigger resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>The name of the trigger.</p> |
| `type` | String | ✅ | <p>The type of the new trigger.</p> |
| `predicate` | String |  | <p>A predicate to specify when the new trigger should fire.</p>
         <p>This field is required when the trigger type is <code>CONDITIONAL</code>.</p> |
| `actions` | Vec<String> | ✅ | <p>The actions initiated by this trigger when it fires.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags to use with this trigger. You may use tags to limit access to the trigger.
      For more information about tags in Glue, see
      <a href="https://docs.aws.amazon.com/glue/latest/dg/monitor-tags.html">Amazon Web Services Tags in Glue</a> in the developer guide. </p> |
| `event_batching_condition` | String |  | <p>Batch condition that must be met (specified number of events received or batch time window expired)
      before EventBridge event trigger fires.</p> |
| `workflow_name` | String |  | <p>The name of the workflow associated with the trigger.</p> |
| `description` | String |  | <p>A description of the new trigger.</p> |
| `start_on_creation` | bool |  | <p>Set to <code>true</code> to start <code>SCHEDULED</code> and <code>CONDITIONAL</code>
      triggers when created. True is not supported for <code>ON_DEMAND</code> triggers.</p> |
| `schedule` | String |  | <p>A <code>cron</code> expression used to specify the schedule (see <a href="https://docs.aws.amazon.com/glue/latest/dg/monitor-data-warehouse-schedule.html">Time-Based Schedules for Jobs and Crawlers</a>. For example, to run
      something every day at 12:15 UTC, you would specify:
      <code>cron(15 12 * * ? *)</code>.</p>
         <p>This field is required when the trigger type is SCHEDULED.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `trigger` | String | <p>The requested trigger definition.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create trigger
trigger = provider.glue.Trigger {
    name = "value"  # <p>The name of the trigger.</p>
    type = "value"  # <p>The type of the new trigger.</p>
    actions = "value"  # <p>The actions initiated by this trigger when it fires.</p>
}

# Access trigger outputs
trigger_id = trigger.id
trigger_trigger = trigger.trigger
```

---


### Entity_records

EntityRecords resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>A continuation token, present if the current segment is not the last.</p> |
| `records` | Vec<String> | <p>A list of the requested objects.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access entity_records outputs
entity_records_id = entity_records.id
entity_records_next_token = entity_records.next_token
entity_records_records = entity_records.records
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
| `get_resource_policies_response_list` | Vec<String> | <p>A list of the individual resource policies and the account-level resource policy.</p> |
| `next_token` | String | <p>A continuation token, if the returned list does not contain the last resource policy available.</p> |


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
resource_policies_get_resource_policies_response_list = resource_policies.get_resource_policies_response_list
resource_policies_next_token = resource_policies.next_token
```

---


### Schema_version

SchemaVersion resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | String | <p>The status of the schema version. </p> |
| `created_time` | String | <p>The date and time the schema version was created.</p> |
| `schema_definition` | String | <p>The schema definition for the schema ID.</p> |
| `schema_arn` | String | <p>The Amazon Resource Name (ARN) of the schema.</p> |
| `schema_version_id` | String | <p>The <code>SchemaVersionId</code> of the schema version.</p> |
| `version_number` | i64 | <p>The version number of the schema.</p> |
| `data_format` | String | <p>The data format of the schema definition. Currently <code>AVRO</code>, <code>JSON</code> and <code>PROTOBUF</code> are supported.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access schema_version outputs
schema_version_id = schema_version.id
schema_version_status = schema_version.status
schema_version_created_time = schema_version.created_time
schema_version_schema_definition = schema_version.schema_definition
schema_version_schema_arn = schema_version.schema_arn
schema_version_schema_version_id = schema_version.schema_version_id
schema_version_version_number = schema_version.version_number
schema_version_data_format = schema_version.data_format
```

---


### Table

Table resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `database_name` | String | ✅ | <p>The catalog database in which to create the new table. For Hive
      compatibility, this name is entirely lowercase.</p> |
| `catalog_id` | String |  | <p>The ID of the Data Catalog in which to create the <code>Table</code>.
      If none is supplied, the Amazon Web Services account ID is used by default.</p> |
| `partition_indexes` | Vec<String> |  | <p>A list of partition indexes, <code>PartitionIndex</code> structures, to create in the table.</p> |
| `open_table_format_input` | String |  | <p>Specifies an <code>OpenTableFormatInput</code> structure when creating an open format table.</p> |
| `name` | String |  | <p>The unique identifier for the table within the specified database that will be 
      created in the Glue Data Catalog.</p> |
| `transaction_id` | String |  | <p>The ID of the transaction.</p> |
| `table_input` | String |  | <p>The <code>TableInput</code> object that defines the metadata table
      to create in the catalog.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `table` | String | <p>The <code>Table</code> object that defines the specified table.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create table
table = provider.glue.Table {
    database_name = "value"  # <p>The catalog database in which to create the new table. For Hive
      compatibility, this name is entirely lowercase.</p>
}

# Access table outputs
table_id = table.id
table_table = table.table
```

---


### Catalogs

Catalogs resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>A continuation token for paginating the returned list of tokens, returned if the current segment of the list is not the last.</p> |
| `catalog_list` | Vec<String> | <p>An array of <code>Catalog</code> objects. A list of <code>Catalog</code> objects from the specified parent catalog.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access catalogs outputs
catalogs_id = catalogs.id
catalogs_next_token = catalogs.next_token
catalogs_catalog_list = catalogs.catalog_list
```

---


### Data_quality_rule_recommendation_run

DataQualityRuleRecommendationRun resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `number_of_workers` | i64 | <p>The number of <code>G.1X</code> workers to be used in the run. The default is 5.</p> |
| `execution_time` | i64 | <p>The amount of time (in seconds) that the run consumed resources.</p> |
| `data_quality_security_configuration` | String | <p>The name of the security configuration created with the data quality encryption option.</p> |
| `completed_on` | String | <p>The date and time when this run was completed.</p> |
| `status` | String | <p>The status for this run.</p> |
| `recommended_ruleset` | String | <p>When a start rule recommendation run completes, it creates a recommended ruleset (a set of rules). This member has those rules in Data Quality Definition Language (DQDL) format.</p> |
| `created_ruleset_name` | String | <p>The name of the ruleset that was created by the run.</p> |
| `last_modified_on` | String | <p>A timestamp. The last point in time when this data quality rule recommendation run was modified.</p> |
| `timeout` | i64 | <p>The timeout for a run in minutes. This is the maximum time that a run can consume resources before it is terminated and enters <code>TIMEOUT</code> status. The default is 2,880 minutes (48 hours).</p> |
| `error_string` | String | <p>The error strings that are associated with the run.</p> |
| `started_on` | String | <p>The date and time when this run started.</p> |
| `role` | String | <p>An IAM role supplied to encrypt the results of the run.</p> |
| `data_source` | String | <p>The data source (an Glue table) associated with this run.</p> |
| `run_id` | String | <p>The unique run identifier associated with this run.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access data_quality_rule_recommendation_run outputs
data_quality_rule_recommendation_run_id = data_quality_rule_recommendation_run.id
data_quality_rule_recommendation_run_number_of_workers = data_quality_rule_recommendation_run.number_of_workers
data_quality_rule_recommendation_run_execution_time = data_quality_rule_recommendation_run.execution_time
data_quality_rule_recommendation_run_data_quality_security_configuration = data_quality_rule_recommendation_run.data_quality_security_configuration
data_quality_rule_recommendation_run_completed_on = data_quality_rule_recommendation_run.completed_on
data_quality_rule_recommendation_run_status = data_quality_rule_recommendation_run.status
data_quality_rule_recommendation_run_recommended_ruleset = data_quality_rule_recommendation_run.recommended_ruleset
data_quality_rule_recommendation_run_created_ruleset_name = data_quality_rule_recommendation_run.created_ruleset_name
data_quality_rule_recommendation_run_last_modified_on = data_quality_rule_recommendation_run.last_modified_on
data_quality_rule_recommendation_run_timeout = data_quality_rule_recommendation_run.timeout
data_quality_rule_recommendation_run_error_string = data_quality_rule_recommendation_run.error_string
data_quality_rule_recommendation_run_started_on = data_quality_rule_recommendation_run.started_on
data_quality_rule_recommendation_run_role = data_quality_rule_recommendation_run.role
data_quality_rule_recommendation_run_data_source = data_quality_rule_recommendation_run.data_source
data_quality_rule_recommendation_run_run_id = data_quality_rule_recommendation_run.run_id
```

---


### Unfiltered_partitions_metadata

UnfilteredPartitionsMetadata resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>A continuation token, if the returned list of partitions does not include the last
      one.</p> |
| `unfiltered_partitions` | Vec<String> | <p>A list of requested partitions.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access unfiltered_partitions_metadata outputs
unfiltered_partitions_metadata_id = unfiltered_partitions_metadata.id
unfiltered_partitions_metadata_next_token = unfiltered_partitions_metadata.next_token
unfiltered_partitions_metadata_unfiltered_partitions = unfiltered_partitions_metadata.unfiltered_partitions
```

---


### Catalog_import_status

CatalogImportStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `import_status` | String | <p>The status of the specified catalog migration.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access catalog_import_status outputs
catalog_import_status_id = catalog_import_status.id
catalog_import_status_import_status = catalog_import_status.import_status
```

---


### Custom_entity_type

CustomEntityType resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | HashMap<String, String> |  | <p>A list of tags applied to the custom entity type.</p> |
| `context_words` | Vec<String> |  | <p>A list of context words. If none of these context words are found within the vicinity of the regular expression the data will not be detected as sensitive data.</p>
         <p>If no context words are passed only a regular expression is checked.</p> |
| `name` | String | ✅ | <p>A name for the custom pattern that allows it to be retrieved or deleted later. This name must be unique per Amazon Web Services account.</p> |
| `regex_string` | String | ✅ | <p>A regular expression string that is used for detecting sensitive data in a custom pattern.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `context_words` | Vec<String> | <p>A list of context words if specified when you created the custom pattern. If none of these context words are found within the vicinity of the regular expression the data will not be detected as sensitive data.</p> |
| `name` | String | <p>The name of the custom pattern that you retrieved.</p> |
| `regex_string` | String | <p>A regular expression string that is used for detecting sensitive data in a custom pattern.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create custom_entity_type
custom_entity_type = provider.glue.Custom_entity_type {
    name = "value"  # <p>A name for the custom pattern that allows it to be retrieved or deleted later. This name must be unique per Amazon Web Services account.</p>
    regex_string = "value"  # <p>A regular expression string that is used for detecting sensitive data in a custom pattern.</p>
}

# Access custom_entity_type outputs
custom_entity_type_id = custom_entity_type.id
custom_entity_type_context_words = custom_entity_type.context_words
custom_entity_type_name = custom_entity_type.name
custom_entity_type_regex_string = custom_entity_type.regex_string
```

---


### Tables

Tables resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>A continuation token, present if the current list segment is not the last.</p> |
| `table_list` | Vec<String> | <p>A list of the requested <code>Table</code> objects.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access tables outputs
tables_id = tables.id
tables_next_token = tables.next_token
tables_table_list = tables.table_list
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple schema resources
schema_0 = provider.glue.Schema {
    data_format = "value-0"
    schema_name = "value-0"
}
schema_1 = provider.glue.Schema {
    data_format = "value-1"
    schema_name = "value-1"
}
schema_2 = provider.glue.Schema {
    data_format = "value-2"
    schema_name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    schema = provider.glue.Schema {
        data_format = "production-value"
        schema_name = "production-value"
    }
```

---

## Related Documentation

- [AWS Glue Documentation](https://docs.aws.amazon.com/glue/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
