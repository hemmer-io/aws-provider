# Database_migration_service Service



**Resources**: 59

---

## Overview

The database_migration_service service provides access to 59 resource types:

- [Certificate](#certificate) [D]
- [Fleet_advisor_databases](#fleet_advisor_databases) [RD]
- [Account_attributes](#account_attributes) [R]
- [Fleet_advisor_lsa_analysis](#fleet_advisor_lsa_analysis) [R]
- [Replication_instance_task_logs](#replication_instance_task_logs) [R]
- [Recommendations](#recommendations) [R]
- [Replication_subnet_groups](#replication_subnet_groups) [R]
- [Replication_subnet_group](#replication_subnet_group) [CD]
- [Refresh_schemas_status](#refresh_schemas_status) [R]
- [Replication_config](#replication_config) [CD]
- [Replication_task_assessment_run](#replication_task_assessment_run) [D]
- [Data_migrations](#data_migrations) [R]
- [Recommendation_limitations](#recommendation_limitations) [R]
- [Data_providers](#data_providers) [R]
- [Metadata_model_imports](#metadata_model_imports) [R]
- [Replications](#replications) [R]
- [Fleet_advisor_collectors](#fleet_advisor_collectors) [R]
- [Metadata_model_exports_as_script](#metadata_model_exports_as_script) [R]
- [Extension_pack_associations](#extension_pack_associations) [R]
- [Endpoint_settings](#endpoint_settings) [R]
- [Endpoints](#endpoints) [R]
- [Metadata_model_conversions](#metadata_model_conversions) [R]
- [Orderable_replication_instances](#orderable_replication_instances) [R]
- [Pending_maintenance_actions](#pending_maintenance_actions) [R]
- [Metadata_model_assessments](#metadata_model_assessments) [R]
- [Connections](#connections) [R]
- [Schemas](#schemas) [R]
- [Events](#events) [R]
- [Instance_profiles](#instance_profiles) [R]
- [Endpoint](#endpoint) [CD]
- [Endpoint_types](#endpoint_types) [R]
- [Instance_profile](#instance_profile) [CD]
- [Connection](#connection) [D]
- [Replication_task_assessment_results](#replication_task_assessment_results) [R]
- [Fleet_advisor_schemas](#fleet_advisor_schemas) [R]
- [Replication_tasks](#replication_tasks) [R]
- [Event_subscription](#event_subscription) [CD]
- [Replication_instances](#replication_instances) [R]
- [Migration_projects](#migration_projects) [R]
- [Replication_table_statistics](#replication_table_statistics) [R]
- [Replication_task_assessment_runs](#replication_task_assessment_runs) [R]
- [Certificates](#certificates) [R]
- [Replication_task](#replication_task) [CD]
- [Replication_configs](#replication_configs) [R]
- [Migration_project](#migration_project) [CD]
- [Subscriptions_to_event_bridge](#subscriptions_to_event_bridge) [U]
- [Conversion_configuration](#conversion_configuration) [R]
- [Event_categories](#event_categories) [R]
- [Engine_versions](#engine_versions) [R]
- [Fleet_advisor_schema_object_summary](#fleet_advisor_schema_object_summary) [R]
- [Replication_task_individual_assessments](#replication_task_individual_assessments) [R]
- [Metadata_model_exports_to_target](#metadata_model_exports_to_target) [R]
- [Data_provider](#data_provider) [CD]
- [Fleet_advisor_collector](#fleet_advisor_collector) [CD]
- [Applicable_individual_assessments](#applicable_individual_assessments) [R]
- [Data_migration](#data_migration) [CD]
- [Table_statistics](#table_statistics) [R]
- [Replication_instance](#replication_instance) [CD]
- [Event_subscriptions](#event_subscriptions) [R]

---

## Resources


### Certificate

Certificate resource

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


### Fleet_advisor_databases

FleetAdvisorDatabases resource

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `databases` | Vec<String> | <p>Provides descriptions of the Fleet Advisor collector databases, including the database's collector, ID,
            and name.</p> |
| `next_token` | String | <p>If <code>NextToken</code> is returned, there are more results available. The value of
                <code>NextToken</code> is a unique pagination token for each page. Make the call
            again using the returned token to retrieve the next page. Keep all other arguments
            unchanged. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access fleet_advisor_databases outputs
fleet_advisor_databases_id = fleet_advisor_databases.id
fleet_advisor_databases_databases = fleet_advisor_databases.databases
fleet_advisor_databases_next_token = fleet_advisor_databases.next_token
```

---


### Account_attributes

AccountAttributes resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `account_quotas` | Vec<String> | <p>Account quota information.</p> |
| `unique_account_identifier` | String | <p>A unique DMS identifier for an account in a particular Amazon Web Services Region. The value of this
         identifier has the following format: <code>c99999999999</code>. DMS uses this identifier to
         name artifacts. For example, DMS uses this identifier to name the default Amazon S3 bucket
         for storing task assessment reports in a given Amazon Web Services Region. The format of this S3 bucket
         name is the following:
               <code>dms-<i>AccountNumber</i>-<i>UniqueAccountIdentifier</i>.</code>
         Here is an example name for this default S3 bucket:
            <code>dms-111122223333-c44445555666</code>.</p>
         <note>
            <p>DMS supports the <code>UniqueAccountIdentifier</code> parameter in versions 3.1.4
            and later.</p>
         </note> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access account_attributes outputs
account_attributes_id = account_attributes.id
account_attributes_account_quotas = account_attributes.account_quotas
account_attributes_unique_account_identifier = account_attributes.unique_account_identifier
```

---


### Fleet_advisor_lsa_analysis

FleetAdvisorLsaAnalysis resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>If <code>NextToken</code> is returned, there are more results available. The value of
                <code>NextToken</code> is a unique pagination token for each page. Make the call
            again using the returned token to retrieve the next page. Keep all other arguments
            unchanged. </p> |
| `analysis` | Vec<String> | <p>A list of <code>FleetAdvisorLsaAnalysisResponse</code> objects.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access fleet_advisor_lsa_analysis outputs
fleet_advisor_lsa_analysis_id = fleet_advisor_lsa_analysis.id
fleet_advisor_lsa_analysis_next_token = fleet_advisor_lsa_analysis.next_token
fleet_advisor_lsa_analysis_analysis = fleet_advisor_lsa_analysis.analysis
```

---


### Replication_instance_task_logs

ReplicationInstanceTaskLogs resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `replication_instance_task_logs` | Vec<String> | <p>An array of replication task log metadata. Each member of the array contains the
         replication task name, ARN, and task log size (in bytes). </p> |
| `marker` | String | <p> An optional pagination token provided by a previous request. If this parameter is
         specified, the response includes only records beyond the marker, up to the value specified
         by <code>MaxRecords</code>.</p> |
| `replication_instance_arn` | String | <p>The Amazon Resource Name (ARN) of the replication instance.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access replication_instance_task_logs outputs
replication_instance_task_logs_id = replication_instance_task_logs.id
replication_instance_task_logs_replication_instance_task_logs = replication_instance_task_logs.replication_instance_task_logs
replication_instance_task_logs_marker = replication_instance_task_logs.marker
replication_instance_task_logs_replication_instance_arn = replication_instance_task_logs.replication_instance_arn
```

---


### Recommendations

Recommendations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The unique pagination token returned for you to pass to a subsequent request. Fleet
            Advisor returns this token when the number of records in the response is greater than
            the <code>MaxRecords</code> value. To retrieve the next page, make the call again using
            the returned token and keeping all other arguments unchanged.</p> |
| `recommendations` | Vec<String> | <p>The list of recommendations of target engines that Fleet Advisor created for the
            source database.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access recommendations outputs
recommendations_id = recommendations.id
recommendations_next_token = recommendations.next_token
recommendations_recommendations = recommendations.recommendations
```

---


### Replication_subnet_groups

ReplicationSubnetGroups resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `replication_subnet_groups` | Vec<String> | <p>A description of the replication subnet groups.</p> |
| `marker` | String | <p> An optional pagination token provided by a previous request. If this parameter is
         specified, the response includes only records beyond the marker, up to the value specified
         by <code>MaxRecords</code>. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access replication_subnet_groups outputs
replication_subnet_groups_id = replication_subnet_groups.id
replication_subnet_groups_replication_subnet_groups = replication_subnet_groups.replication_subnet_groups
replication_subnet_groups_marker = replication_subnet_groups.marker
```

---


### Replication_subnet_group

ReplicationSubnetGroup resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `replication_subnet_group_description` | String | ✅ | <p>The description for the subnet group.
      </p>
         <p>Constraints: This parameter Must not contain non-printable control characters.</p> |
| `subnet_ids` | Vec<String> | ✅ | <p>Two or more subnet IDs to be assigned to the subnet group.</p> |
| `tags` | Vec<String> |  | <p>One or more tags to be assigned to the subnet group.</p> |
| `replication_subnet_group_identifier` | String | ✅ | <p>The name for the replication subnet group. This value is stored as a lowercase
         string.</p>
         <p>Constraints: Must contain no more than 255 alphanumeric characters, periods,
         underscores, or hyphens. Must not be "default".</p>
         <p>Example: <code>mySubnetgroup</code>
         </p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create replication_subnet_group
replication_subnet_group = provider.database_migration_service.Replication_subnet_group {
    replication_subnet_group_description = "value"  # <p>The description for the subnet group.
      </p>
         <p>Constraints: This parameter Must not contain non-printable control characters.</p>
    subnet_ids = "value"  # <p>Two or more subnet IDs to be assigned to the subnet group.</p>
    replication_subnet_group_identifier = "value"  # <p>The name for the replication subnet group. This value is stored as a lowercase
         string.</p>
         <p>Constraints: Must contain no more than 255 alphanumeric characters, periods,
         underscores, or hyphens. Must not be "default".</p>
         <p>Example: <code>mySubnetgroup</code>
         </p>
}

```

---


### Refresh_schemas_status

RefreshSchemasStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `refresh_schemas_status` | String | <p>The status of the schema.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access refresh_schemas_status outputs
refresh_schemas_status_id = refresh_schemas_status.id
refresh_schemas_status_refresh_schemas_status = refresh_schemas_status.refresh_schemas_status
```

---


### Replication_config

ReplicationConfig resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `replication_settings` | String |  | <p>Optional JSON settings for DMS Serverless replications that are provisioned using this
         replication configuration. For example, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Tasks.CustomizingTasks.TaskSettings.ChangeProcessingTuning.html"> Change processing tuning settings</a>.</p> |
| `tags` | Vec<String> |  | <p>One or more optional tags associated with resources used by the DMS Serverless
         replication. For more information, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Tagging.html"> Tagging resources in Database Migration Service</a>.</p> |
| `resource_identifier` | String |  | <p>Optional unique value or name that you set for a given resource that can be used to
         construct an Amazon Resource Name (ARN) for that resource. For more information, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Security.html#CHAP_Security.FineGrainedAccess"> Fine-grained access control using resource names and
         tags</a>.</p> |
| `table_mappings` | String | ✅ | <p>JSON table mappings for DMS Serverless replications that are provisioned using this
         replication configuration. For more information, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Tasks.CustomizingTasks.TableMapping.SelectionTransformation.html"> Specifying table selection and transformations rules using
            JSON</a>.</p> |
| `supplemental_settings` | String |  | <p>Optional JSON settings for specifying supplemental data. For more information, see
            <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Tasks.TaskData.html">
            Specifying supplemental data for task settings</a>.</p> |
| `target_endpoint_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the target endpoint for this DMS serverless
         replication configuration.</p> |
| `replication_type` | String | ✅ | <p>The type of DMS Serverless replication to provision using this replication
         configuration.</p>
         <p>Possible values:</p>
         <ul>
            <li>
               <p>
                  <code>"full-load"</code>
               </p>
            </li>
            <li>
               <p>
                  <code>"cdc"</code>
               </p>
            </li>
            <li>
               <p>
                  <code>"full-load-and-cdc"</code>
               </p>
            </li>
         </ul> |
| `source_endpoint_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the source endpoint for this DMS Serverless
         replication configuration.</p> |
| `compute_config` | String | ✅ | <p>Configuration parameters for provisioning an DMS Serverless replication.</p> |
| `replication_config_identifier` | String | ✅ | <p>A unique identifier that you want to use to create a <code>ReplicationConfigArn</code>
         that is returned as part of the output from this action. You can then pass this output
            <code>ReplicationConfigArn</code> as the value of the <code>ReplicationConfigArn</code>
         option for other actions to identify both DMS Serverless replications and replication
         configurations that you want those actions to operate on. For some actions, you can also
         use either this unique identifier or a corresponding ARN in action filters to identify the
         specific replication and replication configuration to operate on.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create replication_config
replication_config = provider.database_migration_service.Replication_config {
    table_mappings = "value"  # <p>JSON table mappings for DMS Serverless replications that are provisioned using this
         replication configuration. For more information, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Tasks.CustomizingTasks.TableMapping.SelectionTransformation.html"> Specifying table selection and transformations rules using
            JSON</a>.</p>
    target_endpoint_arn = "value"  # <p>The Amazon Resource Name (ARN) of the target endpoint for this DMS serverless
         replication configuration.</p>
    replication_type = "value"  # <p>The type of DMS Serverless replication to provision using this replication
         configuration.</p>
         <p>Possible values:</p>
         <ul>
            <li>
               <p>
                  <code>"full-load"</code>
               </p>
            </li>
            <li>
               <p>
                  <code>"cdc"</code>
               </p>
            </li>
            <li>
               <p>
                  <code>"full-load-and-cdc"</code>
               </p>
            </li>
         </ul>
    source_endpoint_arn = "value"  # <p>The Amazon Resource Name (ARN) of the source endpoint for this DMS Serverless
         replication configuration.</p>
    compute_config = "value"  # <p>Configuration parameters for provisioning an DMS Serverless replication.</p>
    replication_config_identifier = "value"  # <p>A unique identifier that you want to use to create a <code>ReplicationConfigArn</code>
         that is returned as part of the output from this action. You can then pass this output
            <code>ReplicationConfigArn</code> as the value of the <code>ReplicationConfigArn</code>
         option for other actions to identify both DMS Serverless replications and replication
         configurations that you want those actions to operate on. For some actions, you can also
         use either this unique identifier or a corresponding ARN in action filters to identify the
         specific replication and replication configuration to operate on.</p>
}

```

---


### Replication_task_assessment_run

ReplicationTaskAssessmentRun resource

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


### Data_migrations

DataMigrations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `data_migrations` | Vec<String> | <p>Returns information about the data migrations used in the project.</p> |
| `marker` | String | <p>An optional pagination token provided by a previous request. If this parameter is
         specified, the response includes only records beyond the marker, up to the value specified
         by <code>MaxRecords</code>. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access data_migrations outputs
data_migrations_id = data_migrations.id
data_migrations_data_migrations = data_migrations.data_migrations
data_migrations_marker = data_migrations.marker
```

---


### Recommendation_limitations

RecommendationLimitations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The unique pagination token returned for you to pass to a subsequent request. Fleet
            Advisor returns this token when the number of records in the response is greater than
            the <code>MaxRecords</code> value. To retrieve the next page, make the call again using
            the returned token and keeping all other arguments unchanged.</p> |
| `limitations` | Vec<String> | <p>The list of limitations for recommendations of target Amazon Web Services engines.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access recommendation_limitations outputs
recommendation_limitations_id = recommendation_limitations.id
recommendation_limitations_next_token = recommendation_limitations.next_token
recommendation_limitations_limitations = recommendation_limitations.limitations
```

---


### Data_providers

DataProviders resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `data_providers` | Vec<String> | <p>A description of data providers.</p> |
| `marker` | String | <p>Specifies the unique pagination token that makes it possible to display the next page of results. 
         If this parameter is specified, the response includes only records beyond the marker, up to the 
         value specified by <code>MaxRecords</code>.</p>
         <p>If <code>Marker</code> is returned by a previous response, there are more results available. 
         The value of <code>Marker</code> is a unique pagination token for each page. To retrieve the next page, 
         make the call again using the returned token and keeping all other arguments unchanged.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access data_providers outputs
data_providers_id = data_providers.id
data_providers_data_providers = data_providers.data_providers
data_providers_marker = data_providers.marker
```

---


### Metadata_model_imports

MetadataModelImports resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `marker` | String | <p>Specifies the unique pagination token that makes it possible to display the next page of results. 
         If this parameter is specified, the response includes only records beyond the marker, up to the 
         value specified by <code>MaxRecords</code>.</p>
         <p>If <code>Marker</code> is returned by a previous response, there are more results available. 
         The value of <code>Marker</code> is a unique pagination token for each page. To retrieve the next page, 
         make the call again using the returned token and keeping all other arguments unchanged.</p> |
| `requests` | Vec<String> | <p>A paginated list of metadata model imports.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access metadata_model_imports outputs
metadata_model_imports_id = metadata_model_imports.id
metadata_model_imports_marker = metadata_model_imports.marker
metadata_model_imports_requests = metadata_model_imports.requests
```

---


### Replications

Replications resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `replications` | Vec<String> | <p>The replication descriptions.</p> |
| `marker` | String | <p>An optional pagination token provided by a previous request. If this parameter is
         specified, the response includes only records beyond the marker, up to the value specified
         by <code>MaxRecords</code>. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access replications outputs
replications_id = replications.id
replications_replications = replications.replications
replications_marker = replications.marker
```

---


### Fleet_advisor_collectors

FleetAdvisorCollectors resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `collectors` | Vec<String> | <p>Provides descriptions of the Fleet Advisor collectors, including the collectors' name
            and ID, and the latest inventory data. </p> |
| `next_token` | String | <p>If <code>NextToken</code> is returned, there are more results available. The value of
                <code>NextToken</code> is a unique pagination token for each page. Make the call
            again using the returned token to retrieve the next page. Keep all other arguments
            unchanged. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access fleet_advisor_collectors outputs
fleet_advisor_collectors_id = fleet_advisor_collectors.id
fleet_advisor_collectors_collectors = fleet_advisor_collectors.collectors
fleet_advisor_collectors_next_token = fleet_advisor_collectors.next_token
```

---


### Metadata_model_exports_as_script

MetadataModelExportsAsScript resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `requests` | Vec<String> | <p>A paginated list of metadata model exports.</p> |
| `marker` | String | <p>Specifies the unique pagination token that makes it possible to display the next page of results. 
         If this parameter is specified, the response includes only records beyond the marker, up to the 
         value specified by <code>MaxRecords</code>.</p>
         <p>If <code>Marker</code> is returned by a previous response, there are more results available. 
         The value of <code>Marker</code> is a unique pagination token for each page. To retrieve the next page, 
         make the call again using the returned token and keeping all other arguments unchanged.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access metadata_model_exports_as_script outputs
metadata_model_exports_as_script_id = metadata_model_exports_as_script.id
metadata_model_exports_as_script_requests = metadata_model_exports_as_script.requests
metadata_model_exports_as_script_marker = metadata_model_exports_as_script.marker
```

---


### Extension_pack_associations

ExtensionPackAssociations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `requests` | Vec<String> | <p>A paginated list of extension pack associations for the specified migration project.</p> |
| `marker` | String | <p>Specifies the unique pagination token that makes it possible to display the next page of results. 
         If this parameter is specified, the response includes only records beyond the marker, up to the 
         value specified by <code>MaxRecords</code>.</p>
         <p>If <code>Marker</code> is returned by a previous response, there are more results available. 
         The value of <code>Marker</code> is a unique pagination token for each page. To retrieve the next page, 
         make the call again using the returned token and keeping all other arguments unchanged.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access extension_pack_associations outputs
extension_pack_associations_id = extension_pack_associations.id
extension_pack_associations_requests = extension_pack_associations.requests
extension_pack_associations_marker = extension_pack_associations.marker
```

---


### Endpoint_settings

EndpointSettings resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `endpoint_settings` | Vec<String> | <p>Descriptions of the endpoint settings available for your source or target database
         engine.</p> |
| `marker` | String | <p>An optional pagination token provided by a previous request. If this parameter is
         specified, the response includes only records beyond the marker, up to the value specified
         by <code>MaxRecords</code>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access endpoint_settings outputs
endpoint_settings_id = endpoint_settings.id
endpoint_settings_endpoint_settings = endpoint_settings.endpoint_settings
endpoint_settings_marker = endpoint_settings.marker
```

---


### Endpoints

Endpoints resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `endpoints` | Vec<String> | <p>Endpoint description.</p> |
| `marker` | String | <p> An optional pagination token provided by a previous request. If this parameter is
         specified, the response includes only records beyond the marker, up to the value specified
         by <code>MaxRecords</code>. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access endpoints outputs
endpoints_id = endpoints.id
endpoints_endpoints = endpoints.endpoints
endpoints_marker = endpoints.marker
```

---


### Metadata_model_conversions

MetadataModelConversions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `marker` | String | <p>Specifies the unique pagination token that makes it possible to display the next page of results. 
         If this parameter is specified, the response includes only records beyond the marker, up to the 
         value specified by <code>MaxRecords</code>.</p>
         <p>If <code>Marker</code> is returned by a previous response, there are more results available. 
         The value of <code>Marker</code> is a unique pagination token for each page. To retrieve the next page, 
         make the call again using the returned token and keeping all other arguments unchanged.</p> |
| `requests` | Vec<String> | <p>A paginated list of metadata model conversions.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access metadata_model_conversions outputs
metadata_model_conversions_id = metadata_model_conversions.id
metadata_model_conversions_marker = metadata_model_conversions.marker
metadata_model_conversions_requests = metadata_model_conversions.requests
```

---


### Orderable_replication_instances

OrderableReplicationInstances resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `marker` | String | <p> An optional pagination token provided by a previous request. If this parameter is
         specified, the response includes only records beyond the marker, up to the value specified
         by <code>MaxRecords</code>. </p> |
| `orderable_replication_instances` | Vec<String> | <p>The order-able replication instances available.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access orderable_replication_instances outputs
orderable_replication_instances_id = orderable_replication_instances.id
orderable_replication_instances_marker = orderable_replication_instances.marker
orderable_replication_instances_orderable_replication_instances = orderable_replication_instances.orderable_replication_instances
```

---


### Pending_maintenance_actions

PendingMaintenanceActions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `pending_maintenance_actions` | Vec<String> | <p>The pending maintenance action.</p> |
| `marker` | String | <p> An optional pagination token provided by a previous request. If this parameter is
         specified, the response includes only records beyond the marker, up to the value specified
         by <code>MaxRecords</code>. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access pending_maintenance_actions outputs
pending_maintenance_actions_id = pending_maintenance_actions.id
pending_maintenance_actions_pending_maintenance_actions = pending_maintenance_actions.pending_maintenance_actions
pending_maintenance_actions_marker = pending_maintenance_actions.marker
```

---


### Metadata_model_assessments

MetadataModelAssessments resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `marker` | String | <p>Specifies the unique pagination token that makes it possible to display the next page of results. 
         If this parameter is specified, the response includes only records beyond the marker, up to the 
         value specified by <code>MaxRecords</code>.</p>
         <p>If <code>Marker</code> is returned by a previous response, there are more results available. 
         The value of <code>Marker</code> is a unique pagination token for each page. To retrieve the next page, 
         make the call again using the returned token and keeping all other arguments unchanged.</p> |
| `requests` | Vec<String> | <p>A paginated list of metadata model assessments for the specified migration project.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access metadata_model_assessments outputs
metadata_model_assessments_id = metadata_model_assessments.id
metadata_model_assessments_marker = metadata_model_assessments.marker
metadata_model_assessments_requests = metadata_model_assessments.requests
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
| `marker` | String | <p> An optional pagination token provided by a previous request. If this parameter is
         specified, the response includes only records beyond the marker, up to the value specified
         by <code>MaxRecords</code>. </p> |
| `connections` | Vec<String> | <p>A description of the connections.</p> |


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
connections_marker = connections.marker
connections_connections = connections.connections
```

---


### Schemas

Schemas resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `marker` | String | <p> An optional pagination token provided by a previous request. If this parameter is
         specified, the response includes only records beyond the marker, up to the value specified
         by <code>MaxRecords</code>. </p> |
| `schemas` | Vec<String> | <p>The described schema.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access schemas outputs
schemas_id = schemas.id
schemas_marker = schemas.marker
schemas_schemas = schemas.schemas
```

---


### Events

Events resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `events` | Vec<String> | <p>The events described.</p> |
| `marker` | String | <p> An optional pagination token provided by a previous request. If this parameter is
         specified, the response includes only records beyond the marker, up to the value specified
         by <code>MaxRecords</code>. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access events outputs
events_id = events.id
events_events = events.events
events_marker = events.marker
```

---


### Instance_profiles

InstanceProfiles resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `instance_profiles` | Vec<String> | <p>A description of instance profiles.</p> |
| `marker` | String | <p>Specifies the unique pagination token that makes it possible to display the next page of results. 
         If this parameter is specified, the response includes only records beyond the marker, up to the 
         value specified by <code>MaxRecords</code>.</p>
         <p>If <code>Marker</code> is returned by a previous response, there are more results available. 
         The value of <code>Marker</code> is a unique pagination token for each page. To retrieve the next page, 
         make the call again using the returned token and keeping all other arguments unchanged.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access instance_profiles outputs
instance_profiles_id = instance_profiles.id
instance_profiles_instance_profiles = instance_profiles.instance_profiles
instance_profiles_marker = instance_profiles.marker
```

---


### Endpoint

Endpoint resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `extra_connection_attributes` | String |  | <p>Additional attributes associated with the connection. Each attribute is specified as a
         name-value pair associated by an equal sign (=). Multiple attributes are separated by a
         semicolon (;) with no additional white space. For information on the attributes available
         for connecting your source or target endpoint, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Endpoints.html">Working with DMS Endpoints</a> in
         the <i>Database Migration Service User Guide.</i>
         </p> |
| `redis_settings` | String |  | <p>Settings in JSON format for the target Redis endpoint.</p> |
| `external_table_definition` | String |  | <p>The external table definition. </p> |
| `kinesis_settings` | String |  | <p>Settings in JSON format for the target endpoint for Amazon Kinesis Data Streams. For
         more information about the available settings, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Target.Kinesis.html#CHAP_Target.Kinesis.ObjectMapping">Using object mapping to migrate data to a Kinesis data stream</a> in the
            <i>Database Migration Service User Guide.</i>
         </p> |
| `doc_db_settings` | String |  |  |
| `server_name` | String |  | <p>The name of the server where the endpoint database resides.</p> |
| `endpoint_identifier` | String | ✅ | <p>The database endpoint identifier. Identifiers must begin with a letter and must contain
         only ASCII letters, digits, and hyphens. They can't end with a hyphen, or contain two
         consecutive hyphens.</p> |
| `service_access_role_arn` | String |  | <p> The Amazon Resource Name (ARN) for the service access role that you want to use to
         create the endpoint. The role must allow the <code>iam:PassRole</code> action.</p> |
| `mongo_db_settings` | String |  | <p>Settings in JSON format for the source MongoDB endpoint. For more information about the
         available settings, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Source.MongoDB.html#CHAP_Source.MongoDB.Configuration">Endpoint configuration settings when using MongoDB as a source for Database Migration Service</a> in
         the <i>Database Migration Service User Guide.</i>
         </p> |
| `my_sql_settings` | String |  | <p>Settings in JSON format for the source and target MySQL endpoint. For information about
         other available settings, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Source.MySQL.html#CHAP_Source.MySQL.ConnectionAttrib">Extra connection attributes when using MySQL as a source for DMS</a> and <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Target.MySQL.html#CHAP_Target.MySQL.ConnectionAttrib">Extra connection attributes when using a MySQL-compatible database as a target for
            DMS</a> in the <i>Database Migration Service User Guide.</i>
         </p> |
| `microsoft_sql_server_settings` | String |  | <p>Settings in JSON format for the source and target Microsoft SQL Server endpoint. For
         information about other available settings, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Source.SQLServer.html#CHAP_Source.SQLServer.ConnectionAttrib">Extra connection attributes when using SQL Server as a source for DMS</a> and
            <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Target.SQLServer.html#CHAP_Target.SQLServer.ConnectionAttrib"> Extra connection attributes when using SQL Server as a target for DMS</a> in the
            <i>Database Migration Service User Guide.</i>
         </p> |
| `ibm_db2_settings` | String |  | <p>Settings in JSON format for the source IBM Db2 LUW endpoint. For information about other
         available settings, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Source.DB2.html#CHAP_Source.DB2.ConnectionAttrib">Extra
            connection attributes when using Db2 LUW as a source for DMS</a> in the
            <i>Database Migration Service User Guide.</i>
         </p> |
| `tags` | Vec<String> |  | <p>One or more tags to be assigned to the endpoint.</p> |
| `password` | String |  | <p>The password to be used to log in to the endpoint database.</p> |
| `resource_identifier` | String |  | <p>A friendly name for the resource identifier at the end of the <code>EndpointArn</code>
         response parameter that is returned in the created <code>Endpoint</code> object. The value
         for this parameter can have up to 31 characters. It can contain only ASCII letters, digits,
         and hyphen ('-'). Also, it can't end with a hyphen or contain two consecutive hyphens,
         and can only begin with a letter, such as <code>Example-App-ARN1</code>. For example, this
         value might result in the <code>EndpointArn</code> value
            <code>arn:aws:dms:eu-west-1:012345678901:rep:Example-App-ARN1</code>. If you don't
         specify a <code>ResourceIdentifier</code> value, DMS generates a default identifier value
         for the end of <code>EndpointArn</code>.</p> |
| `redshift_settings` | String |  |  |
| `neptune_settings` | String |  | <p>Settings in JSON format for the target Amazon Neptune endpoint. For more information
         about the available settings, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Target.Neptune.html#CHAP_Target.Neptune.EndpointSettings">Specifying graph-mapping rules using Gremlin and R2RML for Amazon
            Neptune as a target</a> in the <i>Database Migration Service User Guide.</i>
         </p> |
| `ssl_mode` | String |  | <p>The Secure Sockets Layer (SSL) mode to use for the SSL connection. The default is
            <code>none</code>
         </p> |
| `postgre_sql_settings` | String |  | <p>Settings in JSON format for the source and target PostgreSQL endpoint. For information
         about other available settings, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Source.PostgreSQL.html#CHAP_Source.PostgreSQL.ConnectionAttrib">Extra connection attributes when using PostgreSQL as a source for DMS</a> and
            <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Target.PostgreSQL.html#CHAP_Target.PostgreSQL.ConnectionAttrib"> Extra connection attributes when using PostgreSQL as a target for DMS</a> in the
            <i>Database Migration Service User Guide.</i>
         </p> |
| `certificate_arn` | String |  | <p>The Amazon Resource Name (ARN) for the certificate.</p> |
| `sybase_settings` | String |  | <p>Settings in JSON format for the source and target SAP ASE endpoint. For information
         about other available settings, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Source.SAP.html#CHAP_Source.SAP.ConnectionAttrib">Extra
            connection attributes when using SAP ASE as a source for DMS</a> and <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Target.SAP.html#CHAP_Target.SAP.ConnectionAttrib">Extra
            connection attributes when using SAP ASE as a target for DMS</a> in the
            <i>Database Migration Service User Guide.</i>
         </p> |
| `kms_key_id` | String |  | <p>An KMS key identifier that is used to encrypt the connection parameters for the
         endpoint.</p>
         <p>If you don't specify a value for the <code>KmsKeyId</code> parameter, then DMS uses
         your default encryption key.</p>
         <p>KMS creates the default encryption key for your Amazon Web Services account. Your Amazon Web Services account has
         a different default encryption key for each Amazon Web Services Region.</p> |
| `username` | String |  | <p>The user name to be used to log in to the endpoint database.</p> |
| `kafka_settings` | String |  | <p>Settings in JSON format for the target Apache Kafka endpoint. For more information about
         the available settings, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Target.Kafka.html#CHAP_Target.Kafka.ObjectMapping">Using
            object mapping to migrate data to a Kafka topic</a> in the <i>Database Migration Service User
            Guide.</i>
         </p> |
| `s3_settings` | String |  | <p>Settings in JSON format for the target Amazon S3 endpoint. For more information about
         the available settings, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Target.S3.html#CHAP_Target.S3.Configuring">Extra
            Connection Attributes When Using Amazon S3 as a Target for DMS</a> in the
            <i>Database Migration Service User Guide.</i>
         </p> |
| `gcp_my_sql_settings` | String |  | <p>Settings in JSON format for the source GCP MySQL endpoint.</p> |
| `engine_name` | String | ✅ | <p>The type of engine for the endpoint. Valid values, depending on the
            <code>EndpointType</code> value, include <code>"mysql"</code>, <code>"oracle"</code>,
            <code>"postgres"</code>, <code>"mariadb"</code>, <code>"aurora"</code>,
            <code>"aurora-postgresql"</code>, <code>"opensearch"</code>, <code>"redshift"</code>,
            <code>"s3"</code>, <code>"db2"</code>, <code>"db2-zos"</code>, <code>"azuredb"</code>,
            <code>"sybase"</code>, <code>"dynamodb"</code>, <code>"mongodb"</code>,
            <code>"kinesis"</code>, <code>"kafka"</code>, <code>"elasticsearch"</code>,
            <code>"docdb"</code>, <code>"sqlserver"</code>, <code>"neptune"</code>,
            <code>"babelfish"</code>, <code>redshift-serverless</code>,
            <code>aurora-serverless</code>, <code>aurora-postgresql-serverless</code>,
            <code>gcp-mysql</code>, <code>azure-sql-managed-instance</code>, <code>redis</code>,
            <code>dms-transfer</code>.</p> |
| `dms_transfer_settings` | String |  | <p>The settings in JSON format for the DMS transfer type of source endpoint. </p>
         <p>Possible settings include the following:</p>
         <ul>
            <li>
               <p>
                  <code>ServiceAccessRoleArn</code> - The Amazon Resource Name (ARN) used by the
               service access IAM role. The role must allow the <code>iam:PassRole</code>
               action.</p>
            </li>
            <li>
               <p>
                  <code>BucketName</code> - The name of the S3 bucket to use.</p>
            </li>
         </ul>
         <p>Shorthand syntax for these settings is as follows:
            <code>ServiceAccessRoleArn=string,BucketName=string</code>
         </p>
         <p>JSON syntax for these settings is as follows: <code>{ "ServiceAccessRoleArn": "string",
            "BucketName": "string", } </code>
         </p> |
| `port` | i64 |  | <p>The port used by the endpoint database.</p> |
| `database_name` | String |  | <p>The name of the endpoint database. For a MySQL source or target endpoint, do not specify
         DatabaseName. To migrate to a specific database, use this setting and
            <code>targetDbType</code>.</p> |
| `endpoint_type` | String | ✅ | <p>The type of endpoint. Valid values are <code>source</code> and
         <code>target</code>.</p> |
| `dynamo_db_settings` | String |  | <p>Settings in JSON format for the target Amazon DynamoDB endpoint. For information about
         other available settings, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Target.DynamoDB.html#CHAP_Target.DynamoDB.ObjectMapping">Using Object Mapping to Migrate Data to DynamoDB</a> in the <i>Database Migration Service
            User Guide.</i>
         </p> |
| `oracle_settings` | String |  | <p>Settings in JSON format for the source and target Oracle endpoint. For information about
         other available settings, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Source.Oracle.html#CHAP_Source.Oracle.ConnectionAttrib">Extra connection attributes when using Oracle as a source for DMS</a> and <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Target.Oracle.html#CHAP_Target.Oracle.ConnectionAttrib">
            Extra connection attributes when using Oracle as a target for DMS</a> in the
            <i>Database Migration Service User Guide.</i>
         </p> |
| `elasticsearch_settings` | String |  | <p>Settings in JSON format for the target OpenSearch endpoint. For more
         information about the available settings, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Target.Elasticsearch.html#CHAP_Target.Elasticsearch.Configuration">Extra Connection Attributes When Using OpenSearch as a Target for
            DMS</a> in the <i>Database Migration Service User Guide</i>.</p> |
| `timestream_settings` | String |  | <p>Settings in JSON format for the target Amazon Timestream endpoint.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create endpoint
endpoint = provider.database_migration_service.Endpoint {
    endpoint_identifier = "value"  # <p>The database endpoint identifier. Identifiers must begin with a letter and must contain
         only ASCII letters, digits, and hyphens. They can't end with a hyphen, or contain two
         consecutive hyphens.</p>
    engine_name = "value"  # <p>The type of engine for the endpoint. Valid values, depending on the
            <code>EndpointType</code> value, include <code>"mysql"</code>, <code>"oracle"</code>,
            <code>"postgres"</code>, <code>"mariadb"</code>, <code>"aurora"</code>,
            <code>"aurora-postgresql"</code>, <code>"opensearch"</code>, <code>"redshift"</code>,
            <code>"s3"</code>, <code>"db2"</code>, <code>"db2-zos"</code>, <code>"azuredb"</code>,
            <code>"sybase"</code>, <code>"dynamodb"</code>, <code>"mongodb"</code>,
            <code>"kinesis"</code>, <code>"kafka"</code>, <code>"elasticsearch"</code>,
            <code>"docdb"</code>, <code>"sqlserver"</code>, <code>"neptune"</code>,
            <code>"babelfish"</code>, <code>redshift-serverless</code>,
            <code>aurora-serverless</code>, <code>aurora-postgresql-serverless</code>,
            <code>gcp-mysql</code>, <code>azure-sql-managed-instance</code>, <code>redis</code>,
            <code>dms-transfer</code>.</p>
    endpoint_type = "value"  # <p>The type of endpoint. Valid values are <code>source</code> and
         <code>target</code>.</p>
}

```

---


### Endpoint_types

EndpointTypes resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `supported_endpoint_types` | Vec<String> | <p>The types of endpoints that are supported.</p> |
| `marker` | String | <p> An optional pagination token provided by a previous request. If this parameter is
         specified, the response includes only records beyond the marker, up to the value specified
         by <code>MaxRecords</code>. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access endpoint_types outputs
endpoint_types_id = endpoint_types.id
endpoint_types_supported_endpoint_types = endpoint_types.supported_endpoint_types
endpoint_types_marker = endpoint_types.marker
```

---


### Instance_profile

InstanceProfile resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `publicly_accessible` | bool |  | <p>Specifies the accessibility options for the instance profile. A value of
         <code>true</code> represents an instance profile with a public IP address. A value of
         <code>false</code> represents an instance profile with a private IP address. The default value
         is <code>true</code>.</p> |
| `network_type` | String |  | <p>Specifies the network type for the instance profile. A value of <code>IPV4</code> 
         represents an instance profile with IPv4 network type and only supports IPv4 addressing. 
         A value of <code>IPV6</code> represents an instance profile with IPv6 network type 
         and only supports IPv6 addressing. A value of <code>DUAL</code> represents an instance 
         profile with dual network type that supports IPv4 and IPv6 addressing.</p> |
| `kms_key_arn` | String |  | <p>The Amazon Resource Name (ARN) of the KMS key that is used to encrypt 
         the connection parameters for the instance profile.</p>
         <p>If you don't specify a value for the <code>KmsKeyArn</code> parameter, then
         DMS uses an Amazon Web Services owned encryption key to encrypt your resources.</p> |
| `instance_profile_name` | String |  | <p>A user-friendly name for the instance profile.</p> |
| `subnet_group_identifier` | String |  | <p>A subnet group to associate with the instance profile.</p> |
| `vpc_security_groups` | String |  | <p>Specifies the VPC security group names to be used with the instance profile. 
         The VPC security group must work with the VPC containing the instance profile.</p> |
| `availability_zone` | String |  | <p>The Availability Zone where the instance profile will be created. The default
         value is a random, system-chosen Availability Zone in the Amazon Web Services Region where your 
         data provider is created, for examplem <code>us-east-1d</code>.</p> |
| `tags` | Vec<String> |  | <p>One or more tags to be assigned to the instance profile.</p> |
| `description` | String |  | <p>A user-friendly description of the instance profile.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create instance_profile
instance_profile = provider.database_migration_service.Instance_profile {
}

```

---


### Connection

Connection resource

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


### Replication_task_assessment_results

ReplicationTaskAssessmentResults resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `bucket_name` | String | <p>- The Amazon S3 bucket where the task assessment report is located. </p> |
| `marker` | String | <p> An optional pagination token provided by a previous request. If this parameter is
         specified, the response includes only records beyond the marker, up to the value specified
         by <code>MaxRecords</code>. </p> |
| `replication_task_assessment_results` | Vec<String> | <p> The task assessment report. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access replication_task_assessment_results outputs
replication_task_assessment_results_id = replication_task_assessment_results.id
replication_task_assessment_results_bucket_name = replication_task_assessment_results.bucket_name
replication_task_assessment_results_marker = replication_task_assessment_results.marker
replication_task_assessment_results_replication_task_assessment_results = replication_task_assessment_results.replication_task_assessment_results
```

---


### Fleet_advisor_schemas

FleetAdvisorSchemas resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>If <code>NextToken</code> is returned, there are more results available. The value of
                <code>NextToken</code> is a unique pagination token for each page. Make the call
            again using the returned token to retrieve the next page. Keep all other arguments
            unchanged. </p> |
| `fleet_advisor_schemas` | Vec<String> | <p>A collection of <code>SchemaResponse</code> objects.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access fleet_advisor_schemas outputs
fleet_advisor_schemas_id = fleet_advisor_schemas.id
fleet_advisor_schemas_next_token = fleet_advisor_schemas.next_token
fleet_advisor_schemas_fleet_advisor_schemas = fleet_advisor_schemas.fleet_advisor_schemas
```

---


### Replication_tasks

ReplicationTasks resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `replication_tasks` | Vec<String> | <p>A description of the replication tasks.</p> |
| `marker` | String | <p> An optional pagination token provided by a previous request. If this parameter is
         specified, the response includes only records beyond the marker, up to the value specified
         by <code>MaxRecords</code>. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access replication_tasks outputs
replication_tasks_id = replication_tasks.id
replication_tasks_replication_tasks = replication_tasks.replication_tasks
replication_tasks_marker = replication_tasks.marker
```

---


### Event_subscription

EventSubscription resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>One or more tags to be assigned to the event subscription.</p> |
| `source_type` | String |  | <p> The type of DMS resource that generates the events. For example, if you want to be
         notified of events generated by a replication instance, you set this parameter to
            <code>replication-instance</code>. If this value isn't specified, all events are
         returned. </p>
         <p>Valid values: <code>replication-instance</code> | <code>replication-task</code>
         </p> |
| `event_categories` | Vec<String> |  | <p>A list of event categories for a source type that you want to subscribe to. For more
         information, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Events.html">Working with Events and Notifications</a> in the <i>Database Migration Service User
            Guide.</i>
         </p> |
| `source_ids` | Vec<String> |  | <p>A list of identifiers for which DMS provides notification events.</p>
         <p>If you don't specify a value, notifications are provided for all sources.</p>
         <p>If you specify multiple values, they must be of the same type. For example, if you
         specify a database instance ID, then all of the other values must be database instance
         IDs.</p> |
| `sns_topic_arn` | String | ✅ | <p> The Amazon Resource Name (ARN) of the Amazon SNS topic created for event notification.
         The ARN is created by Amazon SNS when you create a topic and subscribe to it. </p> |
| `enabled` | bool |  | <p> A Boolean value; set to <code>true</code> to activate the subscription, or set to
            <code>false</code> to create the subscription but not activate it. </p> |
| `subscription_name` | String | ✅ | <p>The name of the DMS event notification subscription. This name must be less than 255
         characters.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create event_subscription
event_subscription = provider.database_migration_service.Event_subscription {
    sns_topic_arn = "value"  # <p> The Amazon Resource Name (ARN) of the Amazon SNS topic created for event notification.
         The ARN is created by Amazon SNS when you create a topic and subscribe to it. </p>
    subscription_name = "value"  # <p>The name of the DMS event notification subscription. This name must be less than 255
         characters.</p>
}

```

---


### Replication_instances

ReplicationInstances resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `marker` | String | <p> An optional pagination token provided by a previous request. If this parameter is
         specified, the response includes only records beyond the marker, up to the value specified
         by <code>MaxRecords</code>. </p> |
| `replication_instances` | Vec<String> | <p>The replication instances described.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access replication_instances outputs
replication_instances_id = replication_instances.id
replication_instances_marker = replication_instances.marker
replication_instances_replication_instances = replication_instances.replication_instances
```

---


### Migration_projects

MigrationProjects resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `marker` | String | <p>Specifies the unique pagination token that makes it possible to display the next page of results. 
         If this parameter is specified, the response includes only records beyond the marker, up to the 
         value specified by <code>MaxRecords</code>.</p>
         <p>If <code>Marker</code> is returned by a previous response, there are more results available. 
         The value of <code>Marker</code> is a unique pagination token for each page. To retrieve the next page, 
         make the call again using the returned token and keeping all other arguments unchanged.</p> |
| `migration_projects` | Vec<String> | <p>A description of migration projects.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access migration_projects outputs
migration_projects_id = migration_projects.id
migration_projects_marker = migration_projects.marker
migration_projects_migration_projects = migration_projects.migration_projects
```

---


### Replication_table_statistics

ReplicationTableStatistics resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `replication_config_arn` | String | <p>The Amazon Resource Name of the replication config.</p> |
| `marker` | String | <p>An optional pagination token provided by a previous request. If this parameter is
         specified, the response includes only records beyond the marker, up to the value specified
         by <code>MaxRecords</code>. </p> |
| `replication_table_statistics` | Vec<String> | <p>Returns table statistics on the replication, including table name, rows inserted, rows
         updated, and rows deleted.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access replication_table_statistics outputs
replication_table_statistics_id = replication_table_statistics.id
replication_table_statistics_replication_config_arn = replication_table_statistics.replication_config_arn
replication_table_statistics_marker = replication_table_statistics.marker
replication_table_statistics_replication_table_statistics = replication_table_statistics.replication_table_statistics
```

---


### Replication_task_assessment_runs

ReplicationTaskAssessmentRuns resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `marker` | String | <p>A pagination token returned for you to pass to a subsequent request. If you pass this
         token as the <code>Marker</code> value in a subsequent request, the response includes only
         records beyond the marker, up to the value specified in the request by
            <code>MaxRecords</code>.</p> |
| `replication_task_assessment_runs` | Vec<String> | <p>One or more premigration assessment runs as specified by <code>Filters</code>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access replication_task_assessment_runs outputs
replication_task_assessment_runs_id = replication_task_assessment_runs.id
replication_task_assessment_runs_marker = replication_task_assessment_runs.marker
replication_task_assessment_runs_replication_task_assessment_runs = replication_task_assessment_runs.replication_task_assessment_runs
```

---


### Certificates

Certificates resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `certificates` | Vec<String> | <p>The Secure Sockets Layer (SSL) certificates associated with the replication
         instance.</p> |
| `marker` | String | <p>The pagination token.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access certificates outputs
certificates_id = certificates.id
certificates_certificates = certificates.certificates
certificates_marker = certificates.marker
```

---


### Replication_task

ReplicationTask resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `target_endpoint_arn` | String | ✅ | <p>An Amazon Resource Name (ARN) that uniquely identifies the target endpoint.</p> |
| `cdc_start_time` | String |  | <p>Indicates the start time for a change data capture (CDC) operation. Use either
         CdcStartTime or CdcStartPosition to specify when you want a CDC operation to start.
         Specifying both values results in an error.</p>
         <p>Timestamp Example: --cdc-start-time “2018-03-08T12:12:12”</p> |
| `replication_task_settings` | String |  | <p>Overall settings for the task, in JSON format. For more information, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Tasks.CustomizingTasks.TaskSettings.html">Specifying Task
            Settings for Database Migration Service Tasks</a> in the <i>Database Migration Service User
         Guide.</i>
         </p> |
| `cdc_start_position` | String |  | <p>Indicates when you want a change data capture (CDC) operation to start. Use either
         CdcStartPosition or CdcStartTime to specify when you want a CDC operation to start.
         Specifying both values results in an error.</p>
         <p> The value can be in date, checkpoint, or LSN/SCN format.</p>
         <p>Date Example: --cdc-start-position “2018-03-08T12:12:12”</p>
         <p>Checkpoint Example: --cdc-start-position
         "checkpoint:V1#27#mysql-bin-changelog.157832:1975:-1:2002:677883278264080:mysql-bin-changelog.157832:1876#0#0#*#0#93"</p>
         <p>LSN Example: --cdc-start-position “mysql-bin-changelog.000024:373”</p>
         <note>
            <p>When you use this task setting with a source PostgreSQL database, a logical
            replication slot should already be created and associated with the source endpoint. You
            can verify this by setting the <code>slotName</code> extra connection attribute to the
            name of this logical replication slot. For more information, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Source.PostgreSQL.html#CHAP_Source.PostgreSQL.ConnectionAttrib">Extra Connection Attributes When Using PostgreSQL as a Source
               for DMS</a>.</p>
         </note> |
| `cdc_stop_position` | String |  | <p>Indicates when you want a change data capture (CDC) operation to stop. The value can be
         either server time or commit time.</p>
         <p>Server time example: --cdc-stop-position “server_time:2018-02-09T12:12:12”</p>
         <p>Commit time example: --cdc-stop-position “commit_time:2018-02-09T12:12:12“</p> |
| `task_data` | String |  | <p>Supplemental information that the task requires to migrate the data for certain source
         and target endpoints. For more information, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Tasks.TaskData.html">Specifying Supplemental Data for
            Task Settings</a> in the <i>Database Migration Service User Guide.</i>
         </p> |
| `replication_task_identifier` | String | ✅ | <p>An identifier for the replication task.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must contain 1-255 alphanumeric characters or hyphens.</p>
            </li>
            <li>
               <p>First character must be a letter.</p>
            </li>
            <li>
               <p>Cannot end with a hyphen or contain two consecutive hyphens.</p>
            </li>
         </ul> |
| `replication_instance_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of a replication instance.</p> |
| `source_endpoint_arn` | String | ✅ | <p>An Amazon Resource Name (ARN) that uniquely identifies the source endpoint.</p> |
| `table_mappings` | String | ✅ | <p>The table mappings for the task, in JSON format. For more information, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Tasks.CustomizingTasks.TableMapping.html">Using Table
            Mapping to Specify Task Settings</a> in the <i>Database Migration Service User
            Guide.</i>
         </p> |
| `tags` | Vec<String> |  | <p>One or more tags to be assigned to the replication task.</p> |
| `resource_identifier` | String |  | <p>A friendly name for the resource identifier at the end of the <code>EndpointArn</code>
         response parameter that is returned in the created <code>Endpoint</code> object. The value
         for this parameter can have up to 31 characters. It can contain only ASCII letters, digits,
         and hyphen ('-'). Also, it can't end with a hyphen or contain two consecutive hyphens,
         and can only begin with a letter, such as <code>Example-App-ARN1</code>. For example, this
         value might result in the <code>EndpointArn</code> value
            <code>arn:aws:dms:eu-west-1:012345678901:rep:Example-App-ARN1</code>. If you don't
         specify a <code>ResourceIdentifier</code> value, DMS generates a default identifier value
         for the end of <code>EndpointArn</code>.</p> |
| `migration_type` | String | ✅ | <p>The migration type. Valid values: <code>full-load</code> | <code>cdc</code> |
            <code>full-load-and-cdc</code>
         </p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create replication_task
replication_task = provider.database_migration_service.Replication_task {
    target_endpoint_arn = "value"  # <p>An Amazon Resource Name (ARN) that uniquely identifies the target endpoint.</p>
    replication_task_identifier = "value"  # <p>An identifier for the replication task.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must contain 1-255 alphanumeric characters or hyphens.</p>
            </li>
            <li>
               <p>First character must be a letter.</p>
            </li>
            <li>
               <p>Cannot end with a hyphen or contain two consecutive hyphens.</p>
            </li>
         </ul>
    replication_instance_arn = "value"  # <p>The Amazon Resource Name (ARN) of a replication instance.</p>
    source_endpoint_arn = "value"  # <p>An Amazon Resource Name (ARN) that uniquely identifies the source endpoint.</p>
    table_mappings = "value"  # <p>The table mappings for the task, in JSON format. For more information, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Tasks.CustomizingTasks.TableMapping.html">Using Table
            Mapping to Specify Task Settings</a> in the <i>Database Migration Service User
            Guide.</i>
         </p>
    migration_type = "value"  # <p>The migration type. Valid values: <code>full-load</code> | <code>cdc</code> |
            <code>full-load-and-cdc</code>
         </p>
}

```

---


### Replication_configs

ReplicationConfigs resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `marker` | String | <p>An optional pagination token provided by a previous request. If this parameter is
         specified, the response includes only records beyond the marker, up to the value specified
         by <code>MaxRecords</code>. </p> |
| `replication_configs` | Vec<String> | <p>Returned configuration parameters that describe each provisioned DMS Serverless
         replication.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access replication_configs outputs
replication_configs_id = replication_configs.id
replication_configs_marker = replication_configs.marker
replication_configs_replication_configs = replication_configs.replication_configs
```

---


### Migration_project

MigrationProject resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `schema_conversion_application_attributes` | String |  | <p>The schema conversion application attributes, including the Amazon S3 bucket name and Amazon S3 role ARN.</p> |
| `transformation_rules` | String |  | <p>The settings in JSON format for migration rules. Migration rules make it possible for you to change 
         the object names according to the rules that you specify. For example, you can change an object name 
         to lowercase or uppercase, add or remove a prefix or suffix, or rename objects.</p> |
| `migration_project_name` | String |  | <p>A user-friendly name for the migration project.</p> |
| `source_data_provider_descriptors` | Vec<String> | ✅ | <p>Information about the source data provider, including the name, ARN, and Secrets Manager parameters.</p> |
| `description` | String |  | <p>A user-friendly description of the migration project.</p> |
| `instance_profile_identifier` | String | ✅ | <p>The identifier of the associated instance profile. Identifiers must begin with a letter 
         and must contain only ASCII letters, digits, and hyphens. They can't end with 
         a hyphen, or contain two consecutive hyphens.</p> |
| `target_data_provider_descriptors` | Vec<String> | ✅ | <p>Information about the target data provider, including the name, ARN, and Amazon Web Services Secrets Manager parameters.</p> |
| `tags` | Vec<String> |  | <p>One or more tags to be assigned to the migration project.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create migration_project
migration_project = provider.database_migration_service.Migration_project {
    source_data_provider_descriptors = "value"  # <p>Information about the source data provider, including the name, ARN, and Secrets Manager parameters.</p>
    instance_profile_identifier = "value"  # <p>The identifier of the associated instance profile. Identifiers must begin with a letter 
         and must contain only ASCII letters, digits, and hyphens. They can't end with 
         a hyphen, or contain two consecutive hyphens.</p>
    target_data_provider_descriptors = "value"  # <p>Information about the target data provider, including the name, ARN, and Amazon Web Services Secrets Manager parameters.</p>
}

```

---


### Subscriptions_to_event_bridge

SubscriptionsToEventBridge resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `force_move` | bool |  | <p>When set to true, this operation migrates DMS subscriptions for Amazon
         SNS notifications no matter what your replication instance version is. If not set or set to
         false, this operation runs only when all your replication instances are from DMS version 3.4.5 or higher. </p> |



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


### Conversion_configuration

ConversionConfiguration resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `conversion_configuration` | String | <p>The configuration parameters for the schema conversion project.</p> |
| `migration_project_identifier` | String | <p>The name or Amazon Resource Name (ARN) for the schema conversion project.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access conversion_configuration outputs
conversion_configuration_id = conversion_configuration.id
conversion_configuration_conversion_configuration = conversion_configuration.conversion_configuration
conversion_configuration_migration_project_identifier = conversion_configuration.migration_project_identifier
```

---


### Event_categories

EventCategories resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `event_category_group_list` | Vec<String> | <p>A list of event categories.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access event_categories outputs
event_categories_id = event_categories.id
event_categories_event_category_group_list = event_categories.event_category_group_list
```

---


### Engine_versions

EngineVersions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `marker` | String | <p>An optional pagination token provided by a previous request. If this parameter is
         specified, the response includes only records beyond the marker, up to the value specified
         by <code>MaxRecords</code>. </p> |
| `engine_versions` | Vec<String> | <p>Returned <code>EngineVersion</code> objects that describe the replication instance
         engine versions used in the project.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access engine_versions outputs
engine_versions_id = engine_versions.id
engine_versions_marker = engine_versions.marker
engine_versions_engine_versions = engine_versions.engine_versions
```

---


### Fleet_advisor_schema_object_summary

FleetAdvisorSchemaObjectSummary resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `fleet_advisor_schema_objects` | Vec<String> | <p>A collection of <code>FleetAdvisorSchemaObjectResponse</code> objects.</p> |
| `next_token` | String | <p>If <code>NextToken</code> is returned, there are more results available. The value of
                <code>NextToken</code> is a unique pagination token for each page. Make the call
            again using the returned token to retrieve the next page. Keep all other arguments
            unchanged. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access fleet_advisor_schema_object_summary outputs
fleet_advisor_schema_object_summary_id = fleet_advisor_schema_object_summary.id
fleet_advisor_schema_object_summary_fleet_advisor_schema_objects = fleet_advisor_schema_object_summary.fleet_advisor_schema_objects
fleet_advisor_schema_object_summary_next_token = fleet_advisor_schema_object_summary.next_token
```

---


### Replication_task_individual_assessments

ReplicationTaskIndividualAssessments resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `replication_task_individual_assessments` | Vec<String> | <p>One or more individual assessments as specified by <code>Filters</code>.</p> |
| `marker` | String | <p>A pagination token returned for you to pass to a subsequent request. If you pass this
         token as the <code>Marker</code> value in a subsequent request, the response includes only
         records beyond the marker, up to the value specified in the request by
            <code>MaxRecords</code>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access replication_task_individual_assessments outputs
replication_task_individual_assessments_id = replication_task_individual_assessments.id
replication_task_individual_assessments_replication_task_individual_assessments = replication_task_individual_assessments.replication_task_individual_assessments
replication_task_individual_assessments_marker = replication_task_individual_assessments.marker
```

---


### Metadata_model_exports_to_target

MetadataModelExportsToTarget resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `requests` | Vec<String> | <p>A paginated list of metadata model exports.</p> |
| `marker` | String | <p>Specifies the unique pagination token that makes it possible to display the next page of results. 
         If this parameter is specified, the response includes only records beyond the marker, up to the 
         value specified by <code>MaxRecords</code>.</p>
         <p>If <code>Marker</code> is returned by a previous response, there are more results available. 
         The value of <code>Marker</code> is a unique pagination token for each page. To retrieve the next page, 
         make the call again using the returned token and keeping all other arguments unchanged.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access metadata_model_exports_to_target outputs
metadata_model_exports_to_target_id = metadata_model_exports_to_target.id
metadata_model_exports_to_target_requests = metadata_model_exports_to_target.requests
metadata_model_exports_to_target_marker = metadata_model_exports_to_target.marker
```

---


### Data_provider

DataProvider resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `virtual` | bool |  | <p>Indicates whether the data provider is virtual.</p> |
| `settings` | String | ✅ | <p>The settings in JSON format for a data provider.</p> |
| `tags` | Vec<String> |  | <p>One or more tags to be assigned to the data provider.</p> |
| `description` | String |  | <p>A user-friendly description of the data provider.</p> |
| `engine` | String | ✅ | <p>The type of database engine for the data provider. Valid values include <code>"aurora"</code>, 
         <code>"aurora-postgresql"</code>, <code>"mysql"</code>, <code>"oracle"</code>, <code>"postgres"</code>, 
         <code>"sqlserver"</code>, <code>redshift</code>, <code>mariadb</code>, <code>mongodb</code>, <code>db2</code>, <code>db2-zos</code> and <code>docdb</code>. A value of <code>"aurora"</code> represents Amazon Aurora MySQL-Compatible Edition.</p> |
| `data_provider_name` | String |  | <p>A user-friendly name for the data provider.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create data_provider
data_provider = provider.database_migration_service.Data_provider {
    settings = "value"  # <p>The settings in JSON format for a data provider.</p>
    engine = "value"  # <p>The type of database engine for the data provider. Valid values include <code>"aurora"</code>, 
         <code>"aurora-postgresql"</code>, <code>"mysql"</code>, <code>"oracle"</code>, <code>"postgres"</code>, 
         <code>"sqlserver"</code>, <code>redshift</code>, <code>mariadb</code>, <code>mongodb</code>, <code>db2</code>, <code>db2-zos</code> and <code>docdb</code>. A value of <code>"aurora"</code> represents Amazon Aurora MySQL-Compatible Edition.</p>
}

```

---


### Fleet_advisor_collector

FleetAdvisorCollector resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `service_access_role_arn` | String | ✅ | <p>The IAM role that grants permissions to access the specified Amazon S3 bucket.</p> |
| `s3_bucket_name` | String | ✅ | <p>The Amazon S3 bucket that the Fleet Advisor collector uses to store inventory metadata.</p> |
| `collector_name` | String | ✅ | <p>The name of your Fleet Advisor collector (for example, <code>sample-collector</code>).</p> |
| `description` | String |  | <p>A summary description of your Fleet Advisor collector.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create fleet_advisor_collector
fleet_advisor_collector = provider.database_migration_service.Fleet_advisor_collector {
    service_access_role_arn = "value"  # <p>The IAM role that grants permissions to access the specified Amazon S3 bucket.</p>
    s3_bucket_name = "value"  # <p>The Amazon S3 bucket that the Fleet Advisor collector uses to store inventory metadata.</p>
    collector_name = "value"  # <p>The name of your Fleet Advisor collector (for example, <code>sample-collector</code>).</p>
}

```

---


### Applicable_individual_assessments

ApplicableIndividualAssessments resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `individual_assessment_names` | Vec<String> | <p>List of names for the individual assessments supported by the premigration assessment
         run that you start based on the specified request parameters. For more information on the
         available individual assessments, including compatibility with different migration task
         configurations, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Tasks.AssessmentReport.html">Working with premigration assessment runs</a> in the
            <i>Database Migration Service User Guide.</i>
         </p> |
| `marker` | String | <p>Pagination token returned for you to pass to a subsequent request. If you pass this
         token as the <code>Marker</code> value in a subsequent request, the response includes only
         records beyond the marker, up to the value specified in the request by
            <code>MaxRecords</code>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access applicable_individual_assessments outputs
applicable_individual_assessments_id = applicable_individual_assessments.id
applicable_individual_assessments_individual_assessment_names = applicable_individual_assessments.individual_assessment_names
applicable_individual_assessments_marker = applicable_individual_assessments.marker
```

---


### Data_migration

DataMigration resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `migration_project_identifier` | String | ✅ | <p>An identifier for the migration project.</p> |
| `data_migration_type` | String | ✅ | <p>Specifies if the data migration is full-load only, change data capture (CDC) only, or
         full-load and CDC.</p> |
| `service_access_role_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) for the service access role that you want to use to
         create the data migration.</p> |
| `target_data_settings` | Vec<String> |  | <p>Specifies information about the target data provider.</p> |
| `tags` | Vec<String> |  | <p>One or more tags to be assigned to the data migration.</p> |
| `data_migration_name` | String |  | <p>A user-friendly name for the data migration. Data migration names have the following
         constraints:</p>
         <ul>
            <li>
               <p>Must begin with a letter, and can only contain ASCII letters, digits, and hyphens.
            </p>
            </li>
            <li>
               <p>Can't end with a hyphen or contain two consecutive hyphens.</p>
            </li>
            <li>
               <p>Length must be from 1 to 255 characters.</p>
            </li>
         </ul> |
| `selection_rules` | String |  | <p>An optional JSON string specifying what tables, views, and schemas to include or exclude
         from the migration.</p> |
| `enable_cloudwatch_logs` | bool |  | <p>Specifies whether to enable CloudWatch logs for the data migration.</p> |
| `number_of_jobs` | i64 |  | <p>The number of parallel jobs that trigger parallel threads to unload the tables from the
         source, and then load them to the target.</p> |
| `source_data_settings` | Vec<String> |  | <p>Specifies information about the source data provider.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create data_migration
data_migration = provider.database_migration_service.Data_migration {
    migration_project_identifier = "value"  # <p>An identifier for the migration project.</p>
    data_migration_type = "value"  # <p>Specifies if the data migration is full-load only, change data capture (CDC) only, or
         full-load and CDC.</p>
    service_access_role_arn = "value"  # <p>The Amazon Resource Name (ARN) for the service access role that you want to use to
         create the data migration.</p>
}

```

---


### Table_statistics

TableStatistics resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `marker` | String | <p> An optional pagination token provided by a previous request. If this parameter is
         specified, the response includes only records beyond the marker, up to the value specified
         by <code>MaxRecords</code>. </p> |
| `table_statistics` | Vec<String> | <p>The table statistics.</p> |
| `replication_task_arn` | String | <p>The Amazon Resource Name (ARN) of the replication task.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access table_statistics outputs
table_statistics_id = table_statistics.id
table_statistics_marker = table_statistics.marker
table_statistics_table_statistics = table_statistics.table_statistics
table_statistics_replication_task_arn = table_statistics.replication_task_arn
```

---


### Replication_instance

ReplicationInstance resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `engine_version` | String |  | <p>The engine version number of the replication instance.</p>
         <p>If an engine version number is not specified when a replication instance is created, the
         default is the latest engine version available.</p> |
| `allocated_storage` | i64 |  | <p>The amount of storage (in gigabytes) to be initially allocated for the replication
         instance.</p> |
| `preferred_maintenance_window` | String |  | <p>The weekly time range during which system maintenance can occur, in Universal
         Coordinated Time (UTC).</p>
         <p> Format: <code>ddd:hh24:mi-ddd:hh24:mi</code>
         </p>
         <p>Default: A 30-minute window selected at random from an 8-hour block of time per
         Amazon Web Services Region, occurring on a random day of the week.</p>
         <p>Valid Days: Mon, Tue, Wed, Thu, Fri, Sat, Sun</p>
         <p>Constraints: Minimum 30-minute window.</p> |
| `kms_key_id` | String |  | <p>An KMS key identifier that is used to encrypt the data on the replication
         instance.</p>
         <p>If you don't specify a value for the <code>KmsKeyId</code> parameter, then DMS uses
         your default encryption key.</p>
         <p>KMS creates the default encryption key for your Amazon Web Services account. Your Amazon Web Services account has
         a different default encryption key for each Amazon Web Services Region.</p> |
| `replication_instance_class` | String | ✅ | <p>The compute and memory capacity of the replication instance as defined for the specified
         replication instance class. For example to specify the instance class dms.c4.large, set
         this parameter to <code>"dms.c4.large"</code>.</p>
         <p>For more information on the settings and capacities for the available replication
         instance classes, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_ReplicationInstance.Types.html ">
            Choosing the right DMS replication instance</a>; and, <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_BestPractices.SizingReplicationInstance.html">Selecting the best size for a replication instance</a>. </p> |
| `replication_subnet_group_identifier` | String |  | <p>A subnet group to associate with the replication instance.</p> |
| `network_type` | String |  | <p>The type of IP address protocol used by a replication instance, such as IPv4 only or
         Dual-stack that supports both IPv4 and IPv6 addressing. IPv6 only is not yet
         supported.</p> |
| `vpc_security_group_ids` | Vec<String> |  | <p> Specifies the VPC security group to be used with the replication instance. The VPC
         security group must work with the VPC containing the replication instance. </p> |
| `replication_instance_identifier` | String | ✅ | <p>The replication instance identifier. This parameter is stored as a lowercase
         string.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must contain 1-63 alphanumeric characters or hyphens.</p>
            </li>
            <li>
               <p>First character must be a letter.</p>
            </li>
            <li>
               <p>Can't end with a hyphen or contain two consecutive hyphens.</p>
            </li>
         </ul>
         <p>Example: <code>myrepinstance</code>
         </p> |
| `multi_az` | bool |  | <p> Specifies whether the replication instance is a Multi-AZ deployment. You can't set
         the <code>AvailabilityZone</code> parameter if the Multi-AZ parameter is set to
            <code>true</code>. </p> |
| `tags` | Vec<String> |  | <p>One or more tags to be assigned to the replication instance.</p> |
| `kerberos_authentication_settings` | String |  | <p>Specifies the settings required for kerberos authentication when creating the
         replication instance.</p> |
| `auto_minor_version_upgrade` | bool |  | <p>A value that indicates whether minor engine upgrades are applied automatically to the
         replication instance during the maintenance window. This parameter defaults to
            <code>true</code>.</p>
         <p>Default: <code>true</code>
         </p> |
| `dns_name_servers` | String |  | <p>A list of custom DNS name servers supported for the replication instance to access your
         on-premise source or target database. This list overrides the default name servers
         supported by the replication instance. You can specify a comma-separated list of internet
         addresses for up to four on-premise DNS name servers. For example:
            <code>"1.1.1.1,2.2.2.2,3.3.3.3,4.4.4.4"</code>
         </p> |
| `resource_identifier` | String |  | <p>A friendly name for the resource identifier at the end of the <code>EndpointArn</code>
         response parameter that is returned in the created <code>Endpoint</code> object. The value
         for this parameter can have up to 31 characters. It can contain only ASCII letters, digits,
         and hyphen ('-'). Also, it can't end with a hyphen or contain two consecutive hyphens,
         and can only begin with a letter, such as <code>Example-App-ARN1</code>. For example, this
         value might result in the <code>EndpointArn</code> value
            <code>arn:aws:dms:eu-west-1:012345678901:rep:Example-App-ARN1</code>. If you don't
         specify a <code>ResourceIdentifier</code> value, DMS generates a default identifier value
         for the end of <code>EndpointArn</code>.</p> |
| `publicly_accessible` | bool |  | <p> Specifies the accessibility options for the replication instance. A value of
            <code>true</code> represents an instance with a public IP address. A value of
            <code>false</code> represents an instance with a private IP address. The default value
         is <code>true</code>. </p> |
| `availability_zone` | String |  | <p>The Availability Zone where the replication instance will be created. The default value
         is a random, system-chosen Availability Zone in the endpoint's Amazon Web Services Region, for example:
            <code>us-east-1d</code>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create replication_instance
replication_instance = provider.database_migration_service.Replication_instance {
    replication_instance_class = "value"  # <p>The compute and memory capacity of the replication instance as defined for the specified
         replication instance class. For example to specify the instance class dms.c4.large, set
         this parameter to <code>"dms.c4.large"</code>.</p>
         <p>For more information on the settings and capacities for the available replication
         instance classes, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_ReplicationInstance.Types.html ">
            Choosing the right DMS replication instance</a>; and, <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_BestPractices.SizingReplicationInstance.html">Selecting the best size for a replication instance</a>. </p>
    replication_instance_identifier = "value"  # <p>The replication instance identifier. This parameter is stored as a lowercase
         string.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must contain 1-63 alphanumeric characters or hyphens.</p>
            </li>
            <li>
               <p>First character must be a letter.</p>
            </li>
            <li>
               <p>Can't end with a hyphen or contain two consecutive hyphens.</p>
            </li>
         </ul>
         <p>Example: <code>myrepinstance</code>
         </p>
}

```

---


### Event_subscriptions

EventSubscriptions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `marker` | String | <p> An optional pagination token provided by a previous request. If this parameter is
         specified, the response includes only records beyond the marker, up to the value specified
         by <code>MaxRecords</code>. </p> |
| `event_subscriptions_list` | Vec<String> | <p>A list of event subscriptions.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access event_subscriptions outputs
event_subscriptions_id = event_subscriptions.id
event_subscriptions_marker = event_subscriptions.marker
event_subscriptions_event_subscriptions_list = event_subscriptions.event_subscriptions_list
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple certificate resources
certificate_0 = provider.database_migration_service.Certificate {
}
certificate_1 = provider.database_migration_service.Certificate {
}
certificate_2 = provider.database_migration_service.Certificate {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    certificate = provider.database_migration_service.Certificate {
    }
```

---

## Related Documentation

- [AWS Database_migration_service Documentation](https://docs.aws.amazon.com/database_migration_service/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
