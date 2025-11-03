# Sagemaker Service



**Resources**: 90

---

## Overview

The sagemaker service provides access to 90 resource types:

- [Edge_packaging_job](#edge_packaging_job) [CR]
- [Reserved_capacity](#reserved_capacity) [R]
- [Inference_recommendations_job](#inference_recommendations_job) [CR]
- [Device_fleet_report](#device_fleet_report) [R]
- [Hub_content](#hub_content) [RUD]
- [Trial_component](#trial_component) [CRUD]
- [Artifact](#artifact) [CRUD]
- [Device_fleet](#device_fleet) [CRUD]
- [Lineage_group](#lineage_group) [R]
- [Subscribed_workteam](#subscribed_workteam) [R]
- [Inference_experiment](#inference_experiment) [CRUD]
- [Workforce](#workforce) [CRUD]
- [Model_package_group_policy](#model_package_group_policy) [CRD]
- [Human_task_ui](#human_task_ui) [CRD]
- [Pipeline_version](#pipeline_version) [U]
- [Cluster_scheduler_config](#cluster_scheduler_config) [CRUD]
- [Hub_content_reference](#hub_content_reference) [CUD]
- [Partner_app](#partner_app) [CRUD]
- [Feature_metadata](#feature_metadata) [RU]
- [Sagemaker_servicecatalog_portfolio_status](#sagemaker_servicecatalog_portfolio_status) [R]
- [User_profile](#user_profile) [CRUD]
- [App](#app) [CRD]
- [Labeling_job](#labeling_job) [CR]
- [Model_bias_job_definition](#model_bias_job_definition) [CRD]
- [Notebook_instance](#notebook_instance) [CRUD]
- [Data_quality_job_definition](#data_quality_job_definition) [CRD]
- [Presigned_domain_url](#presigned_domain_url) [C]
- [Cluster_event](#cluster_event) [R]
- [Compute_quota](#compute_quota) [CRUD]
- [Hub](#hub) [CRUD]
- [Device](#device) [R]
- [Endpoint](#endpoint) [CRUD]
- [Pipeline_execution](#pipeline_execution) [RU]
- [Processing_job](#processing_job) [CRD]
- [Pipeline_definition_for_execution](#pipeline_definition_for_execution) [R]
- [Monitoring_alert](#monitoring_alert) [U]
- [Compilation_job](#compilation_job) [CRD]
- [Auto_ml_job](#auto_ml_job) [CR]
- [Studio_lifecycle_config](#studio_lifecycle_config) [CRD]
- [Training_plan](#training_plan) [CR]
- [Optimization_job](#optimization_job) [CRD]
- [Model_explainability_job_definition](#model_explainability_job_definition) [CRD]
- [Cluster](#cluster) [CRUD]
- [Context](#context) [CRUD]
- [Cluster_software](#cluster_software) [U]
- [Algorithm](#algorithm) [CRD]
- [Inference_component_runtime_config](#inference_component_runtime_config) [U]
- [Experiment](#experiment) [CRUD]
- [App_image_config](#app_image_config) [CRUD]
- [Domain](#domain) [CRUD]
- [Flow_definition](#flow_definition) [CRD]
- [Hub_content_presigned_urls](#hub_content_presigned_urls) [C]
- [Edge_deployment_stage](#edge_deployment_stage) [CD]
- [Inference_component](#inference_component) [CRUD]
- [Model_card_export_job](#model_card_export_job) [CR]
- [Model_package_group](#model_package_group) [CRD]
- [Monitoring_schedule](#monitoring_schedule) [CRUD]
- [Partner_app_presigned_url](#partner_app_presigned_url) [C]
- [Project](#project) [CRUD]
- [Space](#space) [CRUD]
- [Transform_job](#transform_job) [CR]
- [Workteam](#workteam) [CRUD]
- [Tags](#tags) [D]
- [Lineage_group_policy](#lineage_group_policy) [R]
- [Pipeline](#pipeline) [CRUD]
- [Scaling_configuration_recommendation](#scaling_configuration_recommendation) [R]
- [Endpoint_config](#endpoint_config) [CRD]
- [Mlflow_tracking_server](#mlflow_tracking_server) [CRUD]
- [Training_job](#training_job) [CRUD]
- [Model_package](#model_package) [CRUD]
- [Auto_ml_job_v2](#auto_ml_job_v2) [CR]
- [Trial](#trial) [CRUD]
- [Search_suggestions](#search_suggestions) [R]
- [Presigned_notebook_instance_url](#presigned_notebook_instance_url) [C]
- [Image](#image) [CRUD]
- [Code_repository](#code_repository) [CRUD]
- [Feature_group](#feature_group) [CRUD]
- [Devices](#devices) [U]
- [Model_quality_job_definition](#model_quality_job_definition) [CRD]
- [Model](#model) [CRD]
- [Model_card](#model_card) [CRUD]
- [Cluster_node](#cluster_node) [R]
- [Endpoint_weights_and_capacities](#endpoint_weights_and_capacities) [U]
- [Edge_deployment_plan](#edge_deployment_plan) [CRD]
- [Hyper_parameter_tuning_job](#hyper_parameter_tuning_job) [CRD]
- [Presigned_mlflow_tracking_server_url](#presigned_mlflow_tracking_server_url) [C]
- [Notebook_instance_lifecycle_config](#notebook_instance_lifecycle_config) [CRUD]
- [Action](#action) [CRUD]
- [Image_version](#image_version) [CRUD]
- [Association](#association) [D]

---

## Resources


### Edge_packaging_job

EdgePackagingJob resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `model_version` | String | ✅ | <p>The version of the model.</p> |
| `compilation_job_name` | String | ✅ | <p>The name of the SageMaker Neo compilation job that will be used to locate model artifacts for packaging.</p> |
| `edge_packaging_job_name` | String | ✅ | <p>The name of the edge packaging job.</p> |
| `model_name` | String | ✅ | <p>The name of the model.</p> |
| `output_config` | String | ✅ | <p>Provides information about the output location for the packaged model.</p> |
| `role_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of an IAM role that enables Amazon SageMaker to download and upload the model, and to contact SageMaker Neo.</p> |
| `resource_key` | String |  | <p>The Amazon Web Services KMS key to use when encrypting the EBS volume the edge packaging job runs on.</p> |
| `tags` | Vec<String> |  | <p>Creates tags for the packaging job.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `model_signature` | String | <p>The signature document of files in the model artifact.</p> |
| `role_arn` | String | <p>The Amazon Resource Name (ARN) of an IAM role that enables Amazon SageMaker to download and upload the model, and to contact Neo.</p> |
| `edge_packaging_job_status` | String | <p>The current status of the packaging job.</p> |
| `output_config` | String | <p>The output configuration for the edge packaging job.</p> |
| `model_version` | String | <p>The version of the model.</p> |
| `model_artifact` | String | <p>The Amazon Simple Storage (S3) URI where model artifacts ares stored.</p> |
| `edge_packaging_job_name` | String | <p>The name of the edge packaging job.</p> |
| `creation_time` | String | <p>The timestamp of when the packaging job was created.</p> |
| `edge_packaging_job_arn` | String | <p>The Amazon Resource Name (ARN) of the edge packaging job.</p> |
| `resource_key` | String | <p>The Amazon Web Services KMS key to use when encrypting the EBS volume the job run on.</p> |
| `model_name` | String | <p>The name of the model.</p> |
| `preset_deployment_output` | String | <p>The output of a SageMaker Edge Manager deployable resource.</p> |
| `last_modified_time` | String | <p>The timestamp of when the job was last updated.</p> |
| `compilation_job_name` | String | <p>The name of the SageMaker Neo compilation job that is used to locate model artifacts that are being packaged.</p> |
| `edge_packaging_job_status_message` | String | <p>Returns a message describing the job status and error messages.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create edge_packaging_job
edge_packaging_job = provider.sagemaker.Edge_packaging_job {
    model_version = "value"  # <p>The version of the model.</p>
    compilation_job_name = "value"  # <p>The name of the SageMaker Neo compilation job that will be used to locate model artifacts for packaging.</p>
    edge_packaging_job_name = "value"  # <p>The name of the edge packaging job.</p>
    model_name = "value"  # <p>The name of the model.</p>
    output_config = "value"  # <p>Provides information about the output location for the packaged model.</p>
    role_arn = "value"  # <p>The Amazon Resource Name (ARN) of an IAM role that enables Amazon SageMaker to download and upload the model, and to contact SageMaker Neo.</p>
}

# Access edge_packaging_job outputs
edge_packaging_job_id = edge_packaging_job.id
edge_packaging_job_model_signature = edge_packaging_job.model_signature
edge_packaging_job_role_arn = edge_packaging_job.role_arn
edge_packaging_job_edge_packaging_job_status = edge_packaging_job.edge_packaging_job_status
edge_packaging_job_output_config = edge_packaging_job.output_config
edge_packaging_job_model_version = edge_packaging_job.model_version
edge_packaging_job_model_artifact = edge_packaging_job.model_artifact
edge_packaging_job_edge_packaging_job_name = edge_packaging_job.edge_packaging_job_name
edge_packaging_job_creation_time = edge_packaging_job.creation_time
edge_packaging_job_edge_packaging_job_arn = edge_packaging_job.edge_packaging_job_arn
edge_packaging_job_resource_key = edge_packaging_job.resource_key
edge_packaging_job_model_name = edge_packaging_job.model_name
edge_packaging_job_preset_deployment_output = edge_packaging_job.preset_deployment_output
edge_packaging_job_last_modified_time = edge_packaging_job.last_modified_time
edge_packaging_job_compilation_job_name = edge_packaging_job.compilation_job_name
edge_packaging_job_edge_packaging_job_status_message = edge_packaging_job.edge_packaging_job_status_message
```

---


### Reserved_capacity

ReservedCapacity resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `total_instance_count` | i64 | <p>The total number of instances allocated to this reserved capacity.</p> |
| `start_time` | String | <p>The timestamp when the reserved capacity becomes active.</p> |
| `ultra_server_summary` | String | <p>A summary of the UltraServer associated with this reserved capacity.</p> |
| `reserved_capacity_arn` | String | <p>ARN of the reserved capacity.</p> |
| `duration_minutes` | i64 | <p>The number of minutes for the duration of the reserved capacity. For example, if a reserved capacity starts at 08:55 and ends at 11:30, the minutes field would be 35.</p> |
| `available_instance_count` | i64 | <p>The number of instances currently available for use in this reserved capacity.</p> |
| `status` | String | <p>The current status of the reserved capacity.</p> |
| `availability_zone` | String | <p>The Availability Zone where the reserved capacity is provisioned.</p> |
| `end_time` | String | <p>The timestamp when the reserved capacity expires.</p> |
| `instance_type` | String | <p>The Amazon EC2 instance type used in the reserved capacity.</p> |
| `in_use_instance_count` | i64 | <p>The number of instances currently in use from this reserved capacity.</p> |
| `reserved_capacity_type` | String | <p>The type of reserved capacity.</p> |
| `duration_hours` | i64 | <p>The total duration of the reserved capacity in hours.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access reserved_capacity outputs
reserved_capacity_id = reserved_capacity.id
reserved_capacity_total_instance_count = reserved_capacity.total_instance_count
reserved_capacity_start_time = reserved_capacity.start_time
reserved_capacity_ultra_server_summary = reserved_capacity.ultra_server_summary
reserved_capacity_reserved_capacity_arn = reserved_capacity.reserved_capacity_arn
reserved_capacity_duration_minutes = reserved_capacity.duration_minutes
reserved_capacity_available_instance_count = reserved_capacity.available_instance_count
reserved_capacity_status = reserved_capacity.status
reserved_capacity_availability_zone = reserved_capacity.availability_zone
reserved_capacity_end_time = reserved_capacity.end_time
reserved_capacity_instance_type = reserved_capacity.instance_type
reserved_capacity_in_use_instance_count = reserved_capacity.in_use_instance_count
reserved_capacity_reserved_capacity_type = reserved_capacity.reserved_capacity_type
reserved_capacity_duration_hours = reserved_capacity.duration_hours
```

---


### Inference_recommendations_job

InferenceRecommendationsJob resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `job_description` | String |  | <p>Description of the recommendation job.</p> |
| `job_name` | String | ✅ | <p>A name for the recommendation job. The name must be unique within the Amazon Web Services Region and within your Amazon Web Services account. The job name is passed down to the resources created by the recommendation job. The names of resources (such as the model, endpoint configuration, endpoint, and compilation) that are prefixed with the job name are truncated at 40 characters.</p> |
| `role_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of an IAM role that enables Amazon SageMaker to perform tasks on your behalf.</p> |
| `stopping_conditions` | String |  | <p>A set of conditions for stopping a recommendation job. If any of the conditions are met, the job is automatically stopped.</p> |
| `input_config` | String | ✅ | <p>Provides information about the versioned model package Amazon Resource Name (ARN), the traffic pattern, and endpoint configurations.</p> |
| `tags` | Vec<String> |  | <p>The metadata that you apply to Amazon Web Services resources to help you categorize and organize them. Each tag consists of a key and a value, both of which you define. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services Resources</a> in the Amazon Web Services General Reference.</p> |
| `job_type` | String | ✅ | <p>Defines the type of recommendation job. Specify <code>Default</code> to initiate an instance recommendation and <code>Advanced</code> to initiate a load test. If left unspecified, Amazon SageMaker Inference Recommender will run an instance recommendation (<code>DEFAULT</code>) job.</p> |
| `output_config` | String |  | <p>Provides information about the output artifacts and the KMS key to use for Amazon S3 server-side encryption.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `inference_recommendations` | Vec<String> | <p>The recommendations made by Inference Recommender.</p> |
| `endpoint_performances` | Vec<String> | <p>The performance results from running an Inference Recommender job on an existing endpoint.</p> |
| `completion_time` | String | <p>A timestamp that shows when the job completed.</p> |
| `stopping_conditions` | String | <p>The stopping conditions that you provided when you initiated the job.</p> |
| `job_type` | String | <p>The job type that you provided when you initiated the job.</p> |
| `job_description` | String | <p>The job description that you provided when you initiated the job.</p> |
| `job_arn` | String | <p>The Amazon Resource Name (ARN) of the job.</p> |
| `creation_time` | String | <p>A timestamp that shows when the job was created.</p> |
| `last_modified_time` | String | <p>A timestamp that shows when the job was last modified.</p> |
| `failure_reason` | String | <p>If the job fails, provides information why the job failed.</p> |
| `status` | String | <p>The status of the job.</p> |
| `job_name` | String | <p>The name of the job. The name must be unique within an Amazon Web Services Region in the Amazon Web Services account.</p> |
| `role_arn` | String | <p>The Amazon Resource Name (ARN) of the Amazon Web Services Identity and Access Management (IAM) role you provided when you initiated the job.</p> |
| `input_config` | String | <p>Returns information about the versioned model package Amazon Resource Name (ARN), the traffic pattern, and endpoint configurations you provided when you initiated the job.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create inference_recommendations_job
inference_recommendations_job = provider.sagemaker.Inference_recommendations_job {
    job_name = "value"  # <p>A name for the recommendation job. The name must be unique within the Amazon Web Services Region and within your Amazon Web Services account. The job name is passed down to the resources created by the recommendation job. The names of resources (such as the model, endpoint configuration, endpoint, and compilation) that are prefixed with the job name are truncated at 40 characters.</p>
    role_arn = "value"  # <p>The Amazon Resource Name (ARN) of an IAM role that enables Amazon SageMaker to perform tasks on your behalf.</p>
    input_config = "value"  # <p>Provides information about the versioned model package Amazon Resource Name (ARN), the traffic pattern, and endpoint configurations.</p>
    job_type = "value"  # <p>Defines the type of recommendation job. Specify <code>Default</code> to initiate an instance recommendation and <code>Advanced</code> to initiate a load test. If left unspecified, Amazon SageMaker Inference Recommender will run an instance recommendation (<code>DEFAULT</code>) job.</p>
}

# Access inference_recommendations_job outputs
inference_recommendations_job_id = inference_recommendations_job.id
inference_recommendations_job_inference_recommendations = inference_recommendations_job.inference_recommendations
inference_recommendations_job_endpoint_performances = inference_recommendations_job.endpoint_performances
inference_recommendations_job_completion_time = inference_recommendations_job.completion_time
inference_recommendations_job_stopping_conditions = inference_recommendations_job.stopping_conditions
inference_recommendations_job_job_type = inference_recommendations_job.job_type
inference_recommendations_job_job_description = inference_recommendations_job.job_description
inference_recommendations_job_job_arn = inference_recommendations_job.job_arn
inference_recommendations_job_creation_time = inference_recommendations_job.creation_time
inference_recommendations_job_last_modified_time = inference_recommendations_job.last_modified_time
inference_recommendations_job_failure_reason = inference_recommendations_job.failure_reason
inference_recommendations_job_status = inference_recommendations_job.status
inference_recommendations_job_job_name = inference_recommendations_job.job_name
inference_recommendations_job_role_arn = inference_recommendations_job.role_arn
inference_recommendations_job_input_config = inference_recommendations_job.input_config
```

---


### Device_fleet_report

DeviceFleetReport resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `report_generated` | String | <p>Timestamp of when the report was generated.</p> |
| `device_fleet_arn` | String | <p>The Amazon Resource Name (ARN) of the device.</p> |
| `agent_versions` | Vec<String> | <p>The versions of Edge Manager agent deployed on the fleet.</p> |
| `device_stats` | String | <p>Status of devices.</p> |
| `model_stats` | Vec<String> | <p>Status of model on device.</p> |
| `output_config` | String | <p>The output configuration for storing sample data collected by the fleet.</p> |
| `device_fleet_name` | String | <p>The name of the fleet.</p> |
| `description` | String | <p>Description of the fleet.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access device_fleet_report outputs
device_fleet_report_id = device_fleet_report.id
device_fleet_report_report_generated = device_fleet_report.report_generated
device_fleet_report_device_fleet_arn = device_fleet_report.device_fleet_arn
device_fleet_report_agent_versions = device_fleet_report.agent_versions
device_fleet_report_device_stats = device_fleet_report.device_stats
device_fleet_report_model_stats = device_fleet_report.model_stats
device_fleet_report_output_config = device_fleet_report.output_config
device_fleet_report_device_fleet_name = device_fleet_report.device_fleet_name
device_fleet_report_description = device_fleet_report.description
```

---


### Hub_content

HubContent resource

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `hub_name` | String | ✅ | <p>The name of the SageMaker hub that contains the hub content you want to update. You can optionally use the hub ARN instead.</p> |
| `hub_content_markdown` | String |  | <p>A string that provides a description of the hub content. This string can include links, tables, and standard markdown formatting.</p> |
| `hub_content_display_name` | String |  | <p>The display name of the hub content.</p> |
| `hub_content_type` | String | ✅ | <p>The content type of the resource that you want to update. Only specify a <code>Model</code> or <code>Notebook</code> resource for this API. To update a <code>ModelReference</code>, use the <code>UpdateHubContentReference</code> API instead.</p> |
| `hub_content_description` | String |  | <p>The description of the hub content.</p> |
| `hub_content_search_keywords` | Vec<String> |  | <p>The searchable keywords of the hub content.</p> |
| `support_status` | String |  | <p>Indicates the current status of the hub content resource.</p> |
| `hub_content_version` | String | ✅ | <p>The hub content version that you want to update. For example, if you have two versions of a resource in your hub, you can update the second version.</p> |
| `hub_content_name` | String | ✅ | <p>The name of the hub content resource that you want to update.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `hub_name` | String | <p>The name of the hub that contains the content.</p> |
| `document_schema_version` | String | <p>The document schema version for the hub content.</p> |
| `hub_content_document` | String | <p>The hub content document that describes information about the hub content such as type, associated containers, scripts, and more.</p> |
| `hub_content_search_keywords` | Vec<String> | <p>The searchable keywords for the hub content.</p> |
| `sage_maker_public_hub_content_arn` | String | <p>The ARN of the public hub content.</p> |
| `hub_content_arn` | String | <p>The Amazon Resource Name (ARN) of the hub content.</p> |
| `failure_reason` | String | <p>The failure reason if importing hub content failed.</p> |
| `last_modified_time` | String | <p>The last modified time of the hub content.</p> |
| `hub_content_name` | String | <p>The name of the hub content.</p> |
| `hub_arn` | String | <p>The Amazon Resource Name (ARN) of the hub that contains the content. </p> |
| `hub_content_markdown` | String | <p>A string that provides a description of the hub content. This string can include links, tables, and standard markdown formating.</p> |
| `creation_time` | String | <p>The date and time that hub content was created.</p> |
| `hub_content_display_name` | String | <p>The display name of the hub content.</p> |
| `hub_content_version` | String | <p>The version of the hub content.</p> |
| `hub_content_dependencies` | Vec<String> | <p>The location of any dependencies that the hub content has, such as scripts, model artifacts, datasets, or notebooks.</p> |
| `hub_content_status` | String | <p>The status of the hub content.</p> |
| `hub_content_description` | String | <p>A description of the hub content.</p> |
| `reference_min_version` | String | <p>The minimum version of the hub content.</p> |
| `support_status` | String | <p>The support status of the hub content.</p> |
| `hub_content_type` | String | <p>The type of hub content.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access hub_content outputs
hub_content_id = hub_content.id
hub_content_hub_name = hub_content.hub_name
hub_content_document_schema_version = hub_content.document_schema_version
hub_content_hub_content_document = hub_content.hub_content_document
hub_content_hub_content_search_keywords = hub_content.hub_content_search_keywords
hub_content_sage_maker_public_hub_content_arn = hub_content.sage_maker_public_hub_content_arn
hub_content_hub_content_arn = hub_content.hub_content_arn
hub_content_failure_reason = hub_content.failure_reason
hub_content_last_modified_time = hub_content.last_modified_time
hub_content_hub_content_name = hub_content.hub_content_name
hub_content_hub_arn = hub_content.hub_arn
hub_content_hub_content_markdown = hub_content.hub_content_markdown
hub_content_creation_time = hub_content.creation_time
hub_content_hub_content_display_name = hub_content.hub_content_display_name
hub_content_hub_content_version = hub_content.hub_content_version
hub_content_hub_content_dependencies = hub_content.hub_content_dependencies
hub_content_hub_content_status = hub_content.hub_content_status
hub_content_hub_content_description = hub_content.hub_content_description
hub_content_reference_min_version = hub_content.reference_min_version
hub_content_support_status = hub_content.support_status
hub_content_hub_content_type = hub_content.hub_content_type
```

---


### Trial_component

TrialComponent resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `parameters` | HashMap<String, String> |  | <p>The hyperparameters for the component.</p> |
| `status` | String |  | <p>The status of the component. States include:</p> <ul> <li> <p>InProgress</p> </li> <li> <p>Completed</p> </li> <li> <p>Failed</p> </li> </ul> |
| `end_time` | String |  | <p>When the component ended.</p> |
| `output_artifacts` | HashMap<String, String> |  | <p>The output artifacts for the component. Examples of output artifacts are metrics, snapshots, logs, and images.</p> |
| `metadata_properties` | String |  |  |
| `display_name` | String |  | <p>The name of the component as displayed. The name doesn't need to be unique. If <code>DisplayName</code> isn't specified, <code>TrialComponentName</code> is displayed.</p> |
| `input_artifacts` | HashMap<String, String> |  | <p>The input artifacts for the component. Examples of input artifacts are datasets, algorithms, hyperparameters, source code, and instance types.</p> |
| `tags` | Vec<String> |  | <p>A list of tags to associate with the component. You can use <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_Search.html">Search</a> API to search on the tags.</p> |
| `start_time` | String |  | <p>When the component started.</p> |
| `trial_component_name` | String | ✅ | <p>The name of the component. The name must be unique in your Amazon Web Services account and is not case-sensitive.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `creation_time` | String | <p>When the component was created.</p> |
| `source` | String | <p>The Amazon Resource Name (ARN) of the source and, optionally, the job type.</p> |
| `output_artifacts` | HashMap<String, String> | <p>The output artifacts of the component.</p> |
| `created_by` | String | <p>Who created the trial component.</p> |
| `trial_component_arn` | String | <p>The Amazon Resource Name (ARN) of the trial component.</p> |
| `display_name` | String | <p>The name of the component as displayed. If <code>DisplayName</code> isn't specified, <code>TrialComponentName</code> is displayed.</p> |
| `status` | String | <p>The status of the component. States include:</p> <ul> <li> <p>InProgress</p> </li> <li> <p>Completed</p> </li> <li> <p>Failed</p> </li> </ul> |
| `parameters` | HashMap<String, String> | <p>The hyperparameters of the component.</p> |
| `start_time` | String | <p>When the component started.</p> |
| `trial_component_name` | String | <p>The name of the trial component.</p> |
| `last_modified_by` | String | <p>Who last modified the component.</p> |
| `lineage_group_arn` | String | <p>The Amazon Resource Name (ARN) of the lineage group.</p> |
| `end_time` | String | <p>When the component ended.</p> |
| `metrics` | Vec<String> | <p>The metrics for the component.</p> |
| `sources` | Vec<String> | <p>A list of ARNs and, if applicable, job types for multiple sources of an experiment run.</p> |
| `last_modified_time` | String | <p>When the component was last modified.</p> |
| `input_artifacts` | HashMap<String, String> | <p>The input artifacts of the component.</p> |
| `metadata_properties` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create trial_component
trial_component = provider.sagemaker.Trial_component {
    trial_component_name = "value"  # <p>The name of the component. The name must be unique in your Amazon Web Services account and is not case-sensitive.</p>
}

# Access trial_component outputs
trial_component_id = trial_component.id
trial_component_creation_time = trial_component.creation_time
trial_component_source = trial_component.source
trial_component_output_artifacts = trial_component.output_artifacts
trial_component_created_by = trial_component.created_by
trial_component_trial_component_arn = trial_component.trial_component_arn
trial_component_display_name = trial_component.display_name
trial_component_status = trial_component.status
trial_component_parameters = trial_component.parameters
trial_component_start_time = trial_component.start_time
trial_component_trial_component_name = trial_component.trial_component_name
trial_component_last_modified_by = trial_component.last_modified_by
trial_component_lineage_group_arn = trial_component.lineage_group_arn
trial_component_end_time = trial_component.end_time
trial_component_metrics = trial_component.metrics
trial_component_sources = trial_component.sources
trial_component_last_modified_time = trial_component.last_modified_time
trial_component_input_artifacts = trial_component.input_artifacts
trial_component_metadata_properties = trial_component.metadata_properties
```

---


### Artifact

Artifact resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `artifact_name` | String |  | <p>The name of the artifact. Must be unique to your account in an Amazon Web Services Region.</p> |
| `metadata_properties` | String |  |  |
| `artifact_type` | String | ✅ | <p>The artifact type.</p> |
| `source` | String | ✅ | <p>The ID, ID type, and URI of the source.</p> |
| `properties` | HashMap<String, String> |  | <p>A list of properties to add to the artifact.</p> |
| `tags` | Vec<String> |  | <p>A list of tags to apply to the artifact.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `artifact_arn` | String | <p>The Amazon Resource Name (ARN) of the artifact.</p> |
| `properties` | HashMap<String, String> | <p>A list of the artifact's properties.</p> |
| `creation_time` | String | <p>When the artifact was created.</p> |
| `artifact_name` | String | <p>The name of the artifact.</p> |
| `source` | String | <p>The source of the artifact.</p> |
| `lineage_group_arn` | String | <p>The Amazon Resource Name (ARN) of the lineage group.</p> |
| `last_modified_time` | String | <p>When the artifact was last modified.</p> |
| `created_by` | String |  |
| `last_modified_by` | String |  |
| `metadata_properties` | String |  |
| `artifact_type` | String | <p>The type of the artifact.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create artifact
artifact = provider.sagemaker.Artifact {
    artifact_type = "value"  # <p>The artifact type.</p>
    source = "value"  # <p>The ID, ID type, and URI of the source.</p>
}

# Access artifact outputs
artifact_id = artifact.id
artifact_artifact_arn = artifact.artifact_arn
artifact_properties = artifact.properties
artifact_creation_time = artifact.creation_time
artifact_artifact_name = artifact.artifact_name
artifact_source = artifact.source
artifact_lineage_group_arn = artifact.lineage_group_arn
artifact_last_modified_time = artifact.last_modified_time
artifact_created_by = artifact.created_by
artifact_last_modified_by = artifact.last_modified_by
artifact_metadata_properties = artifact.metadata_properties
artifact_artifact_type = artifact.artifact_type
```

---


### Device_fleet

DeviceFleet resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>Creates tags for the specified fleet.</p> |
| `enable_iot_role_alias` | bool |  | <p>Whether to create an Amazon Web Services IoT Role Alias during device fleet creation. The name of the role alias generated will match this pattern: "SageMakerEdge-{DeviceFleetName}".</p> <p>For example, if your device fleet is called "demo-fleet", the name of the role alias will be "SageMakerEdge-demo-fleet".</p> |
| `device_fleet_name` | String | ✅ | <p>The name of the fleet that the device belongs to.</p> |
| `role_arn` | String |  | <p>The Amazon Resource Name (ARN) that has access to Amazon Web Services Internet of Things (IoT).</p> |
| `description` | String |  | <p>A description of the fleet.</p> |
| `output_config` | String | ✅ | <p>The output configuration for storing sample data collected by the fleet.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `creation_time` | String | <p>Timestamp of when the device fleet was created.</p> |
| `device_fleet_name` | String | <p>The name of the fleet.</p> |
| `iot_role_alias` | String | <p>The Amazon Resource Name (ARN) alias created in Amazon Web Services Internet of Things (IoT).</p> |
| `last_modified_time` | String | <p>Timestamp of when the device fleet was last updated.</p> |
| `role_arn` | String | <p>The Amazon Resource Name (ARN) that has access to Amazon Web Services Internet of Things (IoT).</p> |
| `output_config` | String | <p>The output configuration for storing sampled data.</p> |
| `device_fleet_arn` | String | <p>The The Amazon Resource Name (ARN) of the fleet.</p> |
| `description` | String | <p>A description of the fleet.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create device_fleet
device_fleet = provider.sagemaker.Device_fleet {
    device_fleet_name = "value"  # <p>The name of the fleet that the device belongs to.</p>
    output_config = "value"  # <p>The output configuration for storing sample data collected by the fleet.</p>
}

# Access device_fleet outputs
device_fleet_id = device_fleet.id
device_fleet_creation_time = device_fleet.creation_time
device_fleet_device_fleet_name = device_fleet.device_fleet_name
device_fleet_iot_role_alias = device_fleet.iot_role_alias
device_fleet_last_modified_time = device_fleet.last_modified_time
device_fleet_role_arn = device_fleet.role_arn
device_fleet_output_config = device_fleet.output_config
device_fleet_device_fleet_arn = device_fleet.device_fleet_arn
device_fleet_description = device_fleet.description
```

---


### Lineage_group

LineageGroup resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `last_modified_by` | String |  |
| `lineage_group_name` | String | <p>The name of the lineage group.</p> |
| `lineage_group_arn` | String | <p>The Amazon Resource Name (ARN) of the lineage group.</p> |
| `creation_time` | String | <p>The creation time of lineage group.</p> |
| `last_modified_time` | String | <p>The last modified time of the lineage group.</p> |
| `description` | String | <p>The description of the lineage group.</p> |
| `created_by` | String |  |
| `display_name` | String | <p>The display name of the lineage group.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access lineage_group outputs
lineage_group_id = lineage_group.id
lineage_group_last_modified_by = lineage_group.last_modified_by
lineage_group_lineage_group_name = lineage_group.lineage_group_name
lineage_group_lineage_group_arn = lineage_group.lineage_group_arn
lineage_group_creation_time = lineage_group.creation_time
lineage_group_last_modified_time = lineage_group.last_modified_time
lineage_group_description = lineage_group.description
lineage_group_created_by = lineage_group.created_by
lineage_group_display_name = lineage_group.display_name
```

---


### Subscribed_workteam

SubscribedWorkteam resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `subscribed_workteam` | String | <p>A <code>Workteam</code> instance that contains information about the work team.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access subscribed_workteam outputs
subscribed_workteam_id = subscribed_workteam.id
subscribed_workteam_subscribed_workteam = subscribed_workteam.subscribed_workteam
```

---


### Inference_experiment

InferenceExperiment resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p> Array of key-value pairs. You can use tags to categorize your Amazon Web Services resources in different ways, for example, by purpose, owner, or environment. For more information, see <a href="https://docs.aws.amazon.com/ARG/latest/userguide/tagging.html">Tagging your Amazon Web Services Resources</a>. </p> |
| `type` | String | ✅ | <p> The type of the inference experiment that you want to run. The following types of experiments are possible: </p> <ul> <li> <p> <code>ShadowMode</code>: You can use this type to validate a shadow variant. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/shadow-tests.html">Shadow tests</a>. </p> </li> </ul> |
| `model_variants` | Vec<String> | ✅ | <p> An array of <code>ModelVariantConfig</code> objects. There is one for each variant in the inference experiment. Each <code>ModelVariantConfig</code> object in the array describes the infrastructure configuration for the corresponding variant. </p> |
| `name` | String | ✅ | <p>The name for the inference experiment.</p> |
| `endpoint_name` | String | ✅ | <p> The name of the Amazon SageMaker endpoint on which you want to run the inference experiment. </p> |
| `data_storage_config` | String |  | <p> The Amazon S3 location and configuration for storing inference request and response data. </p> <p> This is an optional parameter that you can use for data capture. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/model-monitor-data-capture.html">Capture data</a>. </p> |
| `schedule` | String |  | <p> The duration for which you want the inference experiment to run. If you don't specify this field, the experiment automatically starts immediately upon creation and concludes after 7 days. </p> |
| `description` | String |  | <p>A description for the inference experiment.</p> |
| `shadow_mode_config` | String | ✅ | <p> The configuration of <code>ShadowMode</code> inference experiment type. Use this field to specify a production variant which takes all the inference requests, and a shadow variant to which Amazon SageMaker replicates a percentage of the inference requests. For the shadow variant also specify the percentage of requests that Amazon SageMaker replicates. </p> |
| `kms_key` | String |  | <p> The Amazon Web Services Key Management Service (Amazon Web Services KMS) key that Amazon SageMaker uses to encrypt data on the storage volume attached to the ML compute instance that hosts the endpoint. The <code>KmsKey</code> can be any of the following formats: </p> <ul> <li> <p>KMS key ID</p> <p> <code>"1234abcd-12ab-34cd-56ef-1234567890ab"</code> </p> </li> <li> <p>Amazon Resource Name (ARN) of a KMS key</p> <p> <code>"arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab"</code> </p> </li> <li> <p>KMS key Alias</p> <p> <code>"alias/ExampleAlias"</code> </p> </li> <li> <p>Amazon Resource Name (ARN) of a KMS key Alias</p> <p> <code>"arn:aws:kms:us-west-2:111122223333:alias/ExampleAlias"</code> </p> </li> </ul> <p> If you use a KMS key ID or an alias of your KMS key, the Amazon SageMaker execution role must include permissions to call <code>kms:Encrypt</code>. If you don't provide a KMS key ID, Amazon SageMaker uses the default KMS key for Amazon S3 for your role's account. Amazon SageMaker uses server-side encryption with KMS managed keys for <code>OutputDataConfig</code>. If you use a bucket policy with an <code>s3:PutObject</code> permission that only allows objects with server-side encryption, set the condition key of <code>s3:x-amz-server-side-encryption</code> to <code>"aws:kms"</code>. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/UsingKMSEncryption.html">KMS managed Encryption Keys</a> in the <i>Amazon Simple Storage Service Developer Guide.</i> </p> <p> The KMS key policy must grant permission to the IAM role that you specify in your <code>CreateEndpoint</code> and <code>UpdateEndpoint</code> requests. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html">Using Key Policies in Amazon Web Services KMS</a> in the <i>Amazon Web Services Key Management Service Developer Guide</i>. </p> |
| `role_arn` | String | ✅ | <p> The ARN of the IAM role that Amazon SageMaker can assume to access model artifacts and container images, and manage Amazon SageMaker Inference endpoints for model deployment. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `creation_time` | String | <p>The timestamp at which you created the inference experiment.</p> |
| `description` | String | <p>The description of the inference experiment.</p> |
| `role_arn` | String | <p> The ARN of the IAM role that Amazon SageMaker can assume to access model artifacts and container images, and manage Amazon SageMaker Inference endpoints for model deployment. </p> |
| `arn` | String | <p>The ARN of the inference experiment being described.</p> |
| `endpoint_metadata` | String | <p>The metadata of the endpoint on which the inference experiment ran.</p> |
| `shadow_mode_config` | String | <p> The configuration of <code>ShadowMode</code> inference experiment type, which shows the production variant that takes all the inference requests, and the shadow variant to which Amazon SageMaker replicates a percentage of the inference requests. For the shadow variant it also shows the percentage of requests that Amazon SageMaker replicates. </p> |
| `schedule` | String | <p>The duration for which the inference experiment ran or will run.</p> |
| `type` | String | <p>The type of the inference experiment.</p> |
| `last_modified_time` | String | <p>The timestamp at which you last modified the inference experiment.</p> |
| `name` | String | <p>The name of the inference experiment.</p> |
| `status` | String | <p> The status of the inference experiment. The following are the possible statuses for an inference experiment: </p> <ul> <li> <p> <code>Creating</code> - Amazon SageMaker is creating your experiment. </p> </li> <li> <p> <code>Created</code> - Amazon SageMaker has finished the creation of your experiment and will begin the experiment at the scheduled time. </p> </li> <li> <p> <code>Updating</code> - When you make changes to your experiment, your experiment shows as updating. </p> </li> <li> <p> <code>Starting</code> - Amazon SageMaker is beginning your experiment. </p> </li> <li> <p> <code>Running</code> - Your experiment is in progress. </p> </li> <li> <p> <code>Stopping</code> - Amazon SageMaker is stopping your experiment. </p> </li> <li> <p> <code>Completed</code> - Your experiment has completed. </p> </li> <li> <p> <code>Cancelled</code> - When you conclude your experiment early using the <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_StopInferenceExperiment.html">StopInferenceExperiment</a> API, or if any operation fails with an unexpected error, it shows as cancelled. </p> </li> </ul> |
| `completion_time` | String | <p> The timestamp at which the inference experiment was completed. </p> |
| `data_storage_config` | String | <p>The Amazon S3 location and configuration for storing inference request and response data.</p> |
| `status_reason` | String | <p> The error message or client-specified <code>Reason</code> from the <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_StopInferenceExperiment.html">StopInferenceExperiment</a> API, that explains the status of the inference experiment. </p> |
| `kms_key` | String | <p> The Amazon Web Services Key Management Service (Amazon Web Services KMS) key that Amazon SageMaker uses to encrypt data on the storage volume attached to the ML compute instance that hosts the endpoint. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_CreateInferenceExperiment.html">CreateInferenceExperiment</a>. </p> |
| `model_variants` | Vec<String> | <p> An array of <code>ModelVariantConfigSummary</code> objects. There is one for each variant in the inference experiment. Each <code>ModelVariantConfigSummary</code> object in the array describes the infrastructure configuration for deploying the corresponding variant. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create inference_experiment
inference_experiment = provider.sagemaker.Inference_experiment {
    type = "value"  # <p> The type of the inference experiment that you want to run. The following types of experiments are possible: </p> <ul> <li> <p> <code>ShadowMode</code>: You can use this type to validate a shadow variant. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/shadow-tests.html">Shadow tests</a>. </p> </li> </ul>
    model_variants = "value"  # <p> An array of <code>ModelVariantConfig</code> objects. There is one for each variant in the inference experiment. Each <code>ModelVariantConfig</code> object in the array describes the infrastructure configuration for the corresponding variant. </p>
    name = "value"  # <p>The name for the inference experiment.</p>
    endpoint_name = "value"  # <p> The name of the Amazon SageMaker endpoint on which you want to run the inference experiment. </p>
    shadow_mode_config = "value"  # <p> The configuration of <code>ShadowMode</code> inference experiment type. Use this field to specify a production variant which takes all the inference requests, and a shadow variant to which Amazon SageMaker replicates a percentage of the inference requests. For the shadow variant also specify the percentage of requests that Amazon SageMaker replicates. </p>
    role_arn = "value"  # <p> The ARN of the IAM role that Amazon SageMaker can assume to access model artifacts and container images, and manage Amazon SageMaker Inference endpoints for model deployment. </p>
}

# Access inference_experiment outputs
inference_experiment_id = inference_experiment.id
inference_experiment_creation_time = inference_experiment.creation_time
inference_experiment_description = inference_experiment.description
inference_experiment_role_arn = inference_experiment.role_arn
inference_experiment_arn = inference_experiment.arn
inference_experiment_endpoint_metadata = inference_experiment.endpoint_metadata
inference_experiment_shadow_mode_config = inference_experiment.shadow_mode_config
inference_experiment_schedule = inference_experiment.schedule
inference_experiment_type = inference_experiment.type
inference_experiment_last_modified_time = inference_experiment.last_modified_time
inference_experiment_name = inference_experiment.name
inference_experiment_status = inference_experiment.status
inference_experiment_completion_time = inference_experiment.completion_time
inference_experiment_data_storage_config = inference_experiment.data_storage_config
inference_experiment_status_reason = inference_experiment.status_reason
inference_experiment_kms_key = inference_experiment.kms_key
inference_experiment_model_variants = inference_experiment.model_variants
```

---


### Workforce

Workforce resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `workforce_name` | String | ✅ | <p>The name of the private workforce.</p> |
| `cognito_config` | String |  | <p>Use this parameter to configure an Amazon Cognito private workforce. A single Cognito workforce is created using and corresponds to a single <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-identity-pools.html"> Amazon Cognito user pool</a>.</p> <p>Do not use <code>OidcConfig</code> if you specify values for <code>CognitoConfig</code>.</p> |
| `oidc_config` | String |  | <p>Use this parameter to configure a private workforce using your own OIDC Identity Provider.</p> <p>Do not use <code>CognitoConfig</code> if you specify values for <code>OidcConfig</code>.</p> |
| `source_ip_config` | String |  |  |
| `ip_address_type` | String |  | <p>Use this parameter to specify whether you want <code>IPv4</code> only or <code>dualstack</code> (<code>IPv4</code> and <code>IPv6</code>) to support your labeling workforce.</p> |
| `tags` | Vec<String> |  | <p>An array of key-value pairs that contain metadata to help you categorize and organize our workforce. Each tag consists of a key and a value, both of which you define.</p> |
| `workforce_vpc_config` | String |  | <p>Use this parameter to configure a workforce using VPC.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `workforce` | String | <p>A single private workforce, which is automatically created when you create your first private work team. You can create one private work force in each Amazon Web Services Region. By default, any workforce-related API operation used in a specific region will apply to the workforce created in that region. To learn how to create a private workforce, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/sms-workforce-create-private.html">Create a Private Workforce</a>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create workforce
workforce = provider.sagemaker.Workforce {
    workforce_name = "value"  # <p>The name of the private workforce.</p>
}

# Access workforce outputs
workforce_id = workforce.id
workforce_workforce = workforce.workforce
```

---


### Model_package_group_policy

ModelPackageGroupPolicy resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resource_policy` | String | ✅ | <p>The resource policy for the model group.</p> |
| `model_package_group_name` | String | ✅ | <p>The name of the model group to add a resource policy to.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `resource_policy` | String | <p>The resource policy for the model group.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create model_package_group_policy
model_package_group_policy = provider.sagemaker.Model_package_group_policy {
    resource_policy = "value"  # <p>The resource policy for the model group.</p>
    model_package_group_name = "value"  # <p>The name of the model group to add a resource policy to.</p>
}

# Access model_package_group_policy outputs
model_package_group_policy_id = model_package_group_policy.id
model_package_group_policy_resource_policy = model_package_group_policy.resource_policy
```

---


### Human_task_ui

HumanTaskUi resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `human_task_ui_name` | String | ✅ | <p>The name of the user interface you are creating.</p> |
| `ui_template` | String | ✅ |  |
| `tags` | Vec<String> |  | <p>An array of key-value pairs that contain metadata to help you categorize and organize a human review workflow user interface. Each tag consists of a key and a value, both of which you define.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `ui_template` | String |  |
| `human_task_ui_name` | String | <p>The name of the human task user interface (worker task template).</p> |
| `human_task_ui_status` | String | <p>The status of the human task user interface (worker task template). Valid values are listed below.</p> |
| `human_task_ui_arn` | String | <p>The Amazon Resource Name (ARN) of the human task user interface (worker task template).</p> |
| `creation_time` | String | <p>The timestamp when the human task user interface was created.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create human_task_ui
human_task_ui = provider.sagemaker.Human_task_ui {
    human_task_ui_name = "value"  # <p>The name of the user interface you are creating.</p>
    ui_template = "value"  # Required field
}

# Access human_task_ui outputs
human_task_ui_id = human_task_ui.id
human_task_ui_ui_template = human_task_ui.ui_template
human_task_ui_human_task_ui_name = human_task_ui.human_task_ui_name
human_task_ui_human_task_ui_status = human_task_ui.human_task_ui_status
human_task_ui_human_task_ui_arn = human_task_ui.human_task_ui_arn
human_task_ui_creation_time = human_task_ui.creation_time
```

---


### Pipeline_version

PipelineVersion resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `pipeline_version_display_name` | String |  | <p>The display name of the pipeline version.</p> |
| `pipeline_version_description` | String |  | <p>The description of the pipeline version.</p> |
| `pipeline_version_id` | i64 | ✅ | <p>The pipeline version ID to update.</p> |
| `pipeline_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the pipeline.</p> |



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


### Cluster_scheduler_config

ClusterSchedulerConfig resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `scheduler_config` | String | ✅ | <p>Configuration about the monitoring schedule.</p> |
| `tags` | Vec<String> |  | <p>Tags of the cluster policy.</p> |
| `cluster_arn` | String | ✅ | <p>ARN of the cluster.</p> |
| `description` | String |  | <p>Description of the cluster policy.</p> |
| `name` | String | ✅ | <p>Name for the cluster policy.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `created_by` | String |  |
| `cluster_scheduler_config_id` | String | <p>ID of the cluster policy.</p> |
| `cluster_scheduler_config_arn` | String | <p>ARN of the cluster policy.</p> |
| `cluster_scheduler_config_version` | i64 | <p>Version of the cluster policy.</p> |
| `creation_time` | String | <p>Creation time of the cluster policy.</p> |
| `scheduler_config` | String | <p>Cluster policy configuration. This policy is used for task prioritization and fair-share allocation. This helps prioritize critical workloads and distributes idle compute across entities.</p> |
| `name` | String | <p>Name of the cluster policy.</p> |
| `failure_reason` | String | <p>Failure reason of the cluster policy.</p> |
| `cluster_arn` | String | <p>ARN of the cluster where the cluster policy is applied.</p> |
| `description` | String | <p>Description of the cluster policy.</p> |
| `last_modified_by` | String |  |
| `last_modified_time` | String | <p>Last modified time of the cluster policy.</p> |
| `status` | String | <p>Status of the cluster policy.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create cluster_scheduler_config
cluster_scheduler_config = provider.sagemaker.Cluster_scheduler_config {
    scheduler_config = "value"  # <p>Configuration about the monitoring schedule.</p>
    cluster_arn = "value"  # <p>ARN of the cluster.</p>
    name = "value"  # <p>Name for the cluster policy.</p>
}

# Access cluster_scheduler_config outputs
cluster_scheduler_config_id = cluster_scheduler_config.id
cluster_scheduler_config_created_by = cluster_scheduler_config.created_by
cluster_scheduler_config_cluster_scheduler_config_id = cluster_scheduler_config.cluster_scheduler_config_id
cluster_scheduler_config_cluster_scheduler_config_arn = cluster_scheduler_config.cluster_scheduler_config_arn
cluster_scheduler_config_cluster_scheduler_config_version = cluster_scheduler_config.cluster_scheduler_config_version
cluster_scheduler_config_creation_time = cluster_scheduler_config.creation_time
cluster_scheduler_config_scheduler_config = cluster_scheduler_config.scheduler_config
cluster_scheduler_config_name = cluster_scheduler_config.name
cluster_scheduler_config_failure_reason = cluster_scheduler_config.failure_reason
cluster_scheduler_config_cluster_arn = cluster_scheduler_config.cluster_arn
cluster_scheduler_config_description = cluster_scheduler_config.description
cluster_scheduler_config_last_modified_by = cluster_scheduler_config.last_modified_by
cluster_scheduler_config_last_modified_time = cluster_scheduler_config.last_modified_time
cluster_scheduler_config_status = cluster_scheduler_config.status
```

---


### Hub_content_reference

HubContentReference resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `hub_name` | String | ✅ | <p>The name of the hub to add the hub content reference to.</p> |
| `sage_maker_public_hub_content_arn` | String | ✅ | <p>The ARN of the public hub content to reference.</p> |
| `hub_content_name` | String |  | <p>The name of the hub content to reference.</p> |
| `min_version` | String |  | <p>The minimum version of the hub content to reference.</p> |
| `tags` | Vec<String> |  | <p>Any tags associated with the hub content to reference.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create hub_content_reference
hub_content_reference = provider.sagemaker.Hub_content_reference {
    hub_name = "value"  # <p>The name of the hub to add the hub content reference to.</p>
    sage_maker_public_hub_content_arn = "value"  # <p>The ARN of the public hub content to reference.</p>
}

```

---


### Partner_app

PartnerApp resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `application_config` | String |  | <p>Configuration settings for the SageMaker Partner AI App.</p> |
| `maintenance_config` | String |  | <p>Maintenance configuration settings for the SageMaker Partner AI App.</p> |
| `tier` | String | ✅ | <p>Indicates the instance type and size of the cluster attached to the SageMaker Partner AI App.</p> |
| `client_token` | String |  | <p>A unique token that guarantees that the call to this API is idempotent.</p> |
| `kms_key_id` | String |  | <p>SageMaker Partner AI Apps uses Amazon Web Services KMS to encrypt data at rest using an Amazon Web Services managed key by default. For more control, specify a customer managed key.</p> |
| `name` | String | ✅ | <p>The name to give the SageMaker Partner AI App.</p> |
| `auth_type` | String | ✅ | <p>The authorization type that users use to access the SageMaker Partner AI App.</p> |
| `enable_iam_session_based_identity` | bool |  | <p>When set to <code>TRUE</code>, the SageMaker Partner AI App sets the Amazon Web Services IAM session name or the authenticated IAM user as the identity of the SageMaker Partner AI App user.</p> |
| `execution_role_arn` | String | ✅ | <p>The ARN of the IAM role that the partner application uses.</p> |
| `type` | String | ✅ | <p>The type of SageMaker Partner AI App to create. Must be one of the following: <code>lakera-guard</code>, <code>comet</code>, <code>deepchecks-llm-evaluation</code>, or <code>fiddler</code>.</p> |
| `tags` | Vec<String> |  | <p>Each tag consists of a key and an optional value. Tag keys must be unique per resource.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `arn` | String | <p>The ARN of the SageMaker Partner AI App that was described.</p> |
| `kms_key_id` | String | <p>The Amazon Web Services KMS customer managed key used to encrypt the data at rest associated with SageMaker Partner AI Apps.</p> |
| `type` | String | <p>The type of SageMaker Partner AI App. Must be one of the following: <code>lakera-guard</code>, <code>comet</code>, <code>deepchecks-llm-evaluation</code>, or <code>fiddler</code>.</p> |
| `last_modified_time` | String | <p>The time that the SageMaker Partner AI App was last modified.</p> |
| `execution_role_arn` | String | <p>The ARN of the IAM role associated with the SageMaker Partner AI App.</p> |
| `tier` | String | <p>The instance type and size of the cluster attached to the SageMaker Partner AI App.</p> |
| `base_url` | String | <p>The URL of the SageMaker Partner AI App that the Application SDK uses to support in-app calls for the user.</p> |
| `version` | String | <p>The version of the SageMaker Partner AI App.</p> |
| `application_config` | String | <p>Configuration settings for the SageMaker Partner AI App.</p> |
| `creation_time` | String | <p>The time that the SageMaker Partner AI App was created.</p> |
| `name` | String | <p>The name of the SageMaker Partner AI App.</p> |
| `maintenance_config` | String | <p>Maintenance configuration settings for the SageMaker Partner AI App.</p> |
| `auth_type` | String | <p>The authorization type that users use to access the SageMaker Partner AI App.</p> |
| `enable_iam_session_based_identity` | bool | <p>When set to <code>TRUE</code>, the SageMaker Partner AI App sets the Amazon Web Services IAM session name or the authenticated IAM user as the identity of the SageMaker Partner AI App user.</p> |
| `status` | String | <p>The status of the SageMaker Partner AI App.</p> <ul> <li> <p>Creating: SageMaker AI is creating the partner AI app. The partner AI app is not available during creation.</p> </li> <li> <p>Updating: SageMaker AI is updating the partner AI app. The partner AI app is not available when updating.</p> </li> <li> <p>Deleting: SageMaker AI is deleting the partner AI app. The partner AI app is not available during deletion.</p> </li> <li> <p>Available: The partner AI app is provisioned and accessible.</p> </li> <li> <p>Failed: The partner AI app is in a failed state and isn't available. SageMaker AI is investigating the issue. For further guidance, contact Amazon Web Services Support.</p> </li> <li> <p>UpdateFailed: The partner AI app couldn't be updated but is available.</p> </li> <li> <p>Deleted: The partner AI app is permanently deleted and not available.</p> </li> </ul> |
| `error` | String | <p>This is an error field object that contains the error code and the reason for an operation failure.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create partner_app
partner_app = provider.sagemaker.Partner_app {
    tier = "value"  # <p>Indicates the instance type and size of the cluster attached to the SageMaker Partner AI App.</p>
    name = "value"  # <p>The name to give the SageMaker Partner AI App.</p>
    auth_type = "value"  # <p>The authorization type that users use to access the SageMaker Partner AI App.</p>
    execution_role_arn = "value"  # <p>The ARN of the IAM role that the partner application uses.</p>
    type = "value"  # <p>The type of SageMaker Partner AI App to create. Must be one of the following: <code>lakera-guard</code>, <code>comet</code>, <code>deepchecks-llm-evaluation</code>, or <code>fiddler</code>.</p>
}

# Access partner_app outputs
partner_app_id = partner_app.id
partner_app_arn = partner_app.arn
partner_app_kms_key_id = partner_app.kms_key_id
partner_app_type = partner_app.type
partner_app_last_modified_time = partner_app.last_modified_time
partner_app_execution_role_arn = partner_app.execution_role_arn
partner_app_tier = partner_app.tier
partner_app_base_url = partner_app.base_url
partner_app_version = partner_app.version
partner_app_application_config = partner_app.application_config
partner_app_creation_time = partner_app.creation_time
partner_app_name = partner_app.name
partner_app_maintenance_config = partner_app.maintenance_config
partner_app_auth_type = partner_app.auth_type
partner_app_enable_iam_session_based_identity = partner_app.enable_iam_session_based_identity
partner_app_status = partner_app.status
partner_app_error = partner_app.error
```

---


### Feature_metadata

FeatureMetadata resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `feature_name` | String | ✅ | <p>The name of the feature that you're updating.</p> |
| `parameter_removals` | Vec<String> |  | <p>A list of parameter keys that you can specify to remove parameters that describe your feature.</p> |
| `feature_group_name` | String | ✅ | <p>The name or Amazon Resource Name (ARN) of the feature group containing the feature that you're updating.</p> |
| `description` | String |  | <p>A description that you can write to better describe the feature.</p> |
| `parameter_additions` | Vec<String> |  | <p>A list of key-value pairs that you can add to better describe the feature.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `feature_group_arn` | String | <p>The Amazon Resource Number (ARN) of the feature group that contains the feature.</p> |
| `feature_type` | String | <p>The data type of the feature.</p> |
| `feature_name` | String | <p>The name of the feature that you've specified.</p> |
| `feature_group_name` | String | <p>The name of the feature group that you've specified.</p> |
| `parameters` | Vec<String> | <p>The key-value pairs that you added to describe the feature.</p> |
| `description` | String | <p>The description you added to describe the feature.</p> |
| `last_modified_time` | String | <p>A timestamp indicating when the metadata for the feature group was modified. For example, if you add a parameter describing the feature, the timestamp changes to reflect the last time you </p> |
| `creation_time` | String | <p>A timestamp indicating when the feature was created.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access feature_metadata outputs
feature_metadata_id = feature_metadata.id
feature_metadata_feature_group_arn = feature_metadata.feature_group_arn
feature_metadata_feature_type = feature_metadata.feature_type
feature_metadata_feature_name = feature_metadata.feature_name
feature_metadata_feature_group_name = feature_metadata.feature_group_name
feature_metadata_parameters = feature_metadata.parameters
feature_metadata_description = feature_metadata.description
feature_metadata_last_modified_time = feature_metadata.last_modified_time
feature_metadata_creation_time = feature_metadata.creation_time
```

---


### Sagemaker_servicecatalog_portfolio_status

SagemakerServicecatalogPortfolioStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | String | <p>Whether Service Catalog is enabled or disabled in SageMaker.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access sagemaker_servicecatalog_portfolio_status outputs
sagemaker_servicecatalog_portfolio_status_id = sagemaker_servicecatalog_portfolio_status.id
sagemaker_servicecatalog_portfolio_status_status = sagemaker_servicecatalog_portfolio_status.status
```

---


### User_profile

UserProfile resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>Each tag consists of a key and an optional value. Tag keys must be unique per resource.</p> <p>Tags that you specify for the User Profile are also added to all Apps that the User Profile launches.</p> |
| `user_settings` | String |  | <p>A collection of settings.</p> |
| `domain_id` | String | ✅ | <p>The ID of the associated Domain.</p> |
| `user_profile_name` | String | ✅ | <p>A name for the UserProfile. This value is not case sensitive.</p> |
| `single_sign_on_user_identifier` | String |  | <p>A specifier for the type of value specified in SingleSignOnUserValue. Currently, the only supported value is "UserName". If the Domain's AuthMode is IAM Identity Center, this field is required. If the Domain's AuthMode is not IAM Identity Center, this field cannot be specified. </p> |
| `single_sign_on_user_value` | String |  | <p>The username of the associated Amazon Web Services Single Sign-On User for this UserProfile. If the Domain's AuthMode is IAM Identity Center, this field is required, and must match a valid username of a user in your directory. If the Domain's AuthMode is not IAM Identity Center, this field cannot be specified. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `single_sign_on_user_value` | String | <p>The IAM Identity Center user value.</p> |
| `last_modified_time` | String | <p>The last modified time.</p> |
| `home_efs_file_system_uid` | String | <p>The ID of the user's profile in the Amazon Elastic File System volume.</p> |
| `user_profile_arn` | String | <p>The user profile Amazon Resource Name (ARN).</p> |
| `status` | String | <p>The status.</p> |
| `user_profile_name` | String | <p>The user profile name.</p> |
| `creation_time` | String | <p>The creation time.</p> |
| `single_sign_on_user_identifier` | String | <p>The IAM Identity Center user identifier.</p> |
| `domain_id` | String | <p>The ID of the domain that contains the profile.</p> |
| `user_settings` | String | <p>A collection of settings.</p> |
| `failure_reason` | String | <p>The failure reason.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create user_profile
user_profile = provider.sagemaker.User_profile {
    domain_id = "value"  # <p>The ID of the associated Domain.</p>
    user_profile_name = "value"  # <p>A name for the UserProfile. This value is not case sensitive.</p>
}

# Access user_profile outputs
user_profile_id = user_profile.id
user_profile_single_sign_on_user_value = user_profile.single_sign_on_user_value
user_profile_last_modified_time = user_profile.last_modified_time
user_profile_home_efs_file_system_uid = user_profile.home_efs_file_system_uid
user_profile_user_profile_arn = user_profile.user_profile_arn
user_profile_status = user_profile.status
user_profile_user_profile_name = user_profile.user_profile_name
user_profile_creation_time = user_profile.creation_time
user_profile_single_sign_on_user_identifier = user_profile.single_sign_on_user_identifier
user_profile_domain_id = user_profile.domain_id
user_profile_user_settings = user_profile.user_settings
user_profile_failure_reason = user_profile.failure_reason
```

---


### App

App resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resource_spec` | String |  | <p>The instance type and the Amazon Resource Name (ARN) of the SageMaker AI image created on the instance.</p> <note> <p>The value of <code>InstanceType</code> passed as part of the <code>ResourceSpec</code> in the <code>CreateApp</code> call overrides the value passed as part of the <code>ResourceSpec</code> configured for the user profile or the domain. If <code>InstanceType</code> is not specified in any of those three <code>ResourceSpec</code> values for a <code>KernelGateway</code> app, the <code>CreateApp</code> call fails with a request validation error.</p> </note> |
| `tags` | Vec<String> |  | <p>Each tag consists of a key and an optional value. Tag keys must be unique per resource.</p> |
| `app_name` | String | ✅ | <p>The name of the app.</p> |
| `user_profile_name` | String |  | <p>The user profile name. If this value is not set, then <code>SpaceName</code> must be set.</p> |
| `space_name` | String |  | <p>The name of the space. If this value is not set, then <code>UserProfileName</code> must be set.</p> |
| `app_type` | String | ✅ | <p>The type of app.</p> |
| `recovery_mode` | bool |  | <p> Indicates whether the application is launched in recovery mode. </p> |
| `domain_id` | String | ✅ | <p>The domain ID.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `creation_time` | String | <p>The creation time of the application.</p> <note> <p>After an application has been shut down for 24 hours, SageMaker AI deletes all metadata for the application. To be considered an update and retain application metadata, applications must be restarted within 24 hours after the previous application has been shut down. After this time window, creation of an application is considered a new application rather than an update of the previous application.</p> </note> |
| `failure_reason` | String | <p>The failure reason.</p> |
| `last_health_check_timestamp` | String | <p>The timestamp of the last health check.</p> |
| `last_user_activity_timestamp` | String | <p>The timestamp of the last user's activity. <code>LastUserActivityTimestamp</code> is also updated when SageMaker AI performs health checks without user activity. As a result, this value is set to the same value as <code>LastHealthCheckTimestamp</code>.</p> |
| `resource_spec` | String | <p>The instance type and the Amazon Resource Name (ARN) of the SageMaker AI image created on the instance.</p> |
| `built_in_lifecycle_config_arn` | String | <p>The lifecycle configuration that runs before the default lifecycle configuration</p> |
| `effective_trusted_identity_propagation_status` | String | <p>The effective status of Trusted Identity Propagation (TIP) for this application. When enabled, user identities from IAM Identity Center are being propagated through the application to TIP enabled Amazon Web Services services. When disabled, standard IAM role-based access is used. </p> |
| `space_name` | String | <p>The name of the space. If this value is not set, then <code>UserProfileName</code> must be set.</p> |
| `domain_id` | String | <p>The domain ID.</p> |
| `user_profile_name` | String | <p>The user profile name.</p> |
| `app_name` | String | <p>The name of the app.</p> |
| `app_type` | String | <p>The type of app.</p> |
| `app_arn` | String | <p>The Amazon Resource Name (ARN) of the app.</p> |
| `status` | String | <p>The status.</p> |
| `recovery_mode` | bool | <p> Indicates whether the application is launched in recovery mode. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create app
app = provider.sagemaker.App {
    app_name = "value"  # <p>The name of the app.</p>
    app_type = "value"  # <p>The type of app.</p>
    domain_id = "value"  # <p>The domain ID.</p>
}

# Access app outputs
app_id = app.id
app_creation_time = app.creation_time
app_failure_reason = app.failure_reason
app_last_health_check_timestamp = app.last_health_check_timestamp
app_last_user_activity_timestamp = app.last_user_activity_timestamp
app_resource_spec = app.resource_spec
app_built_in_lifecycle_config_arn = app.built_in_lifecycle_config_arn
app_effective_trusted_identity_propagation_status = app.effective_trusted_identity_propagation_status
app_space_name = app.space_name
app_domain_id = app.domain_id
app_user_profile_name = app.user_profile_name
app_app_name = app.app_name
app_app_type = app.app_type
app_app_arn = app.app_arn
app_status = app.status
app_recovery_mode = app.recovery_mode
```

---


### Labeling_job

LabelingJob resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `labeling_job_name` | String | ✅ | <p>The name of the labeling job. This name is used to identify the job in a list of labeling jobs. Labeling job names must be unique within an Amazon Web Services account and region. <code>LabelingJobName</code> is not case sensitive. For example, Example-job and example-job are considered the same labeling job name by Ground Truth.</p> |
| `output_config` | String | ✅ | <p>The location of the output data and the Amazon Web Services Key Management Service key ID for the key used to encrypt the output data, if any.</p> |
| `labeling_job_algorithms_config` | String |  | <p>Configures the information required to perform automated data labeling.</p> |
| `label_category_config_s3_uri` | String |  | <p>The S3 URI of the file, referred to as a <i>label category configuration file</i>, that defines the categories used to label the data objects.</p> <p>For 3D point cloud and video frame task types, you can add label category attributes and frame attributes to your label category configuration file. To learn how, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/sms-point-cloud-label-category-config.html">Create a Labeling Category Configuration File for 3D Point Cloud Labeling Jobs</a>. </p> <p>For named entity recognition jobs, in addition to <code>"labels"</code>, you must provide worker instructions in the label category configuration file using the <code>"instructions"</code> parameter: <code>"instructions": {"shortInstruction":"&lt;h1&gt;Add header&lt;/h1&gt;&lt;p&gt;Add Instructions&lt;/p&gt;", "fullInstruction":"&lt;p&gt;Add additional instructions.&lt;/p&gt;"}</code>. For details and an example, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/sms-named-entity-recg.html#sms-creating-ner-api">Create a Named Entity Recognition Labeling Job (API) </a>.</p> <p>For all other <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/sms-task-types.html">built-in task types</a> and <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/sms-custom-templates.html">custom tasks</a>, your label category configuration file must be a JSON file in the following format. Identify the labels you want to use by replacing <code>label_1</code>, <code>label_2</code>,<code>...</code>,<code>label_n</code> with your label categories.</p> <p> <code>{ </code> </p> <p> <code>"document-version": "2018-11-28",</code> </p> <p> <code>"labels": [{"label": "label_1"},{"label": "label_2"},...{"label": "label_n"}]</code> </p> <p> <code>}</code> </p> <p>Note the following about the label category configuration file:</p> <ul> <li> <p>For image classification and text classification (single and multi-label) you must specify at least two label categories. For all other task types, the minimum number of label categories required is one. </p> </li> <li> <p>Each label category must be unique, you cannot specify duplicate label categories.</p> </li> <li> <p>If you create a 3D point cloud or video frame adjustment or verification labeling job, you must include <code>auditLabelAttributeName</code> in the label category configuration. Use this parameter to enter the <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_CreateLabelingJob.html#sagemaker-CreateLabelingJob-request-LabelAttributeName"> <code>LabelAttributeName</code> </a> of the labeling job you want to adjust or verify annotations of.</p> </li> </ul> |
| `tags` | Vec<String> |  | <p>An array of key/value pairs. For more information, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html#allocation-what">Using Cost Allocation Tags</a> in the <i>Amazon Web Services Billing and Cost Management User Guide</i>.</p> |
| `human_task_config` | String | ✅ | <p>Configures the labeling task and how it is presented to workers; including, but not limited to price, keywords, and batch size (task count).</p> |
| `label_attribute_name` | String | ✅ | <p>The attribute name to use for the label in the output manifest file. This is the key for the key/value pair formed with the label that a worker assigns to the object. The <code>LabelAttributeName</code> must meet the following requirements.</p> <ul> <li> <p>The name can't end with "-metadata". </p> </li> <li> <p>If you are using one of the <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/sms-task-types.html">built-in task types</a> or one of the following, the attribute name <i>must</i> end with "-ref".</p> <ul> <li> <p>Image semantic segmentation (<code>SemanticSegmentation)</code> and adjustment (<code>AdjustmentSemanticSegmentation</code>) labeling jobs for this task type. One exception is that verification (<code>VerificationSemanticSegmentation</code>) <i>must not</i> end with -"ref".</p> </li> <li> <p>Video frame object detection (<code>VideoObjectDetection</code>), and adjustment and verification (<code>AdjustmentVideoObjectDetection</code>) labeling jobs for this task type.</p> </li> <li> <p>Video frame object tracking (<code>VideoObjectTracking</code>), and adjustment and verification (<code>AdjustmentVideoObjectTracking</code>) labeling jobs for this task type.</p> </li> <li> <p>3D point cloud semantic segmentation (<code>3DPointCloudSemanticSegmentation</code>), and adjustment and verification (<code>Adjustment3DPointCloudSemanticSegmentation</code>) labeling jobs for this task type. </p> </li> <li> <p>3D point cloud object tracking (<code>3DPointCloudObjectTracking</code>), and adjustment and verification (<code>Adjustment3DPointCloudObjectTracking</code>) labeling jobs for this task type. </p> </li> </ul> </li> </ul> <p/> <important> <p>If you are creating an adjustment or verification labeling job, you must use a <i>different</i> <code>LabelAttributeName</code> than the one used in the original labeling job. The original labeling job is the Ground Truth labeling job that produced the labels that you want verified or adjusted. To learn more about adjustment and verification labeling jobs, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/sms-verification-data.html">Verify and Adjust Labels</a>.</p> </important> |
| `input_config` | String | ✅ | <p>Input data for the labeling job, such as the Amazon S3 location of the data objects and the location of the manifest file that describes the data objects.</p> <p>You must specify at least one of the following: <code>S3DataSource</code> or <code>SnsDataSource</code>. </p> <ul> <li> <p>Use <code>SnsDataSource</code> to specify an SNS input topic for a streaming labeling job. If you do not specify and SNS input topic ARN, Ground Truth will create a one-time labeling job that stops after all data objects in the input manifest file have been labeled.</p> </li> <li> <p>Use <code>S3DataSource</code> to specify an input manifest file for both streaming and one-time labeling jobs. Adding an <code>S3DataSource</code> is optional if you use <code>SnsDataSource</code> to create a streaming labeling job.</p> </li> </ul> <p>If you use the Amazon Mechanical Turk workforce, your input data should not include confidential information, personal information or protected health information. Use <code>ContentClassifiers</code> to specify that your data is free of personally identifiable information and adult content.</p> |
| `role_arn` | String | ✅ | <p>The Amazon Resource Number (ARN) that Amazon SageMaker assumes to perform tasks on your behalf during data labeling. You must grant this role the necessary permissions so that Amazon SageMaker can successfully complete data labeling.</p> |
| `stopping_conditions` | String |  | <p>A set of conditions for stopping the labeling job. If any of the conditions are met, the job is automatically stopped. You can use these conditions to control the cost of data labeling.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `labeling_job_status` | String | <p>The processing status of the labeling job. </p> |
| `label_counters` | String | <p>Provides a breakdown of the number of data objects labeled by humans, the number of objects labeled by machine, the number of objects than couldn't be labeled, and the total number of objects labeled. </p> |
| `creation_time` | String | <p>The date and time that the labeling job was created.</p> |
| `labeling_job_arn` | String | <p>The Amazon Resource Name (ARN) of the labeling job.</p> |
| `label_category_config_s3_uri` | String | <p>The S3 location of the JSON file that defines the categories used to label data objects. Please note the following label-category limits:</p> <ul> <li> <p>Semantic segmentation labeling jobs using automated labeling: 20 labels</p> </li> <li> <p>Box bounding labeling jobs (all): 10 labels</p> </li> </ul> <p>The file is a JSON structure in the following format:</p> <p> <code>{</code> </p> <p> <code> "document-version": "2018-11-28"</code> </p> <p> <code> "labels": [</code> </p> <p> <code> {</code> </p> <p> <code> "label": "<i>label 1</i>"</code> </p> <p> <code> },</code> </p> <p> <code> {</code> </p> <p> <code> "label": "<i>label 2</i>"</code> </p> <p> <code> },</code> </p> <p> <code> ...</code> </p> <p> <code> {</code> </p> <p> <code> "label": "<i>label n</i>"</code> </p> <p> <code> }</code> </p> <p> <code> ]</code> </p> <p> <code>}</code> </p> |
| `tags` | Vec<String> | <p>An array of key-value pairs. You can use tags to categorize your Amazon Web Services resources in different ways, for example, by purpose, owner, or environment. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services Resources</a>.</p> |
| `role_arn` | String | <p>The Amazon Resource Name (ARN) that SageMaker assumes to perform tasks on your behalf during data labeling.</p> |
| `input_config` | String | <p>Input configuration information for the labeling job, such as the Amazon S3 location of the data objects and the location of the manifest file that describes the data objects.</p> |
| `labeling_job_output` | String | <p>The location of the output produced by the labeling job.</p> |
| `labeling_job_algorithms_config` | String | <p>Configuration information for automated data labeling.</p> |
| `labeling_job_name` | String | <p>The name assigned to the labeling job when it was created.</p> |
| `last_modified_time` | String | <p>The date and time that the labeling job was last updated.</p> |
| `failure_reason` | String | <p>If the job failed, the reason that it failed. </p> |
| `job_reference_code` | String | <p>A unique identifier for work done as part of a labeling job.</p> |
| `label_attribute_name` | String | <p>The attribute used as the label in the output manifest file.</p> |
| `stopping_conditions` | String | <p>A set of conditions for stopping a labeling job. If any of the conditions are met, the job is automatically stopped.</p> |
| `human_task_config` | String | <p>Configuration information required for human workers to complete a labeling task.</p> |
| `output_config` | String | <p>The location of the job's output data and the Amazon Web Services Key Management Service key ID for the key used to encrypt the output data, if any.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create labeling_job
labeling_job = provider.sagemaker.Labeling_job {
    labeling_job_name = "value"  # <p>The name of the labeling job. This name is used to identify the job in a list of labeling jobs. Labeling job names must be unique within an Amazon Web Services account and region. <code>LabelingJobName</code> is not case sensitive. For example, Example-job and example-job are considered the same labeling job name by Ground Truth.</p>
    output_config = "value"  # <p>The location of the output data and the Amazon Web Services Key Management Service key ID for the key used to encrypt the output data, if any.</p>
    human_task_config = "value"  # <p>Configures the labeling task and how it is presented to workers; including, but not limited to price, keywords, and batch size (task count).</p>
    label_attribute_name = "value"  # <p>The attribute name to use for the label in the output manifest file. This is the key for the key/value pair formed with the label that a worker assigns to the object. The <code>LabelAttributeName</code> must meet the following requirements.</p> <ul> <li> <p>The name can't end with "-metadata". </p> </li> <li> <p>If you are using one of the <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/sms-task-types.html">built-in task types</a> or one of the following, the attribute name <i>must</i> end with "-ref".</p> <ul> <li> <p>Image semantic segmentation (<code>SemanticSegmentation)</code> and adjustment (<code>AdjustmentSemanticSegmentation</code>) labeling jobs for this task type. One exception is that verification (<code>VerificationSemanticSegmentation</code>) <i>must not</i> end with -"ref".</p> </li> <li> <p>Video frame object detection (<code>VideoObjectDetection</code>), and adjustment and verification (<code>AdjustmentVideoObjectDetection</code>) labeling jobs for this task type.</p> </li> <li> <p>Video frame object tracking (<code>VideoObjectTracking</code>), and adjustment and verification (<code>AdjustmentVideoObjectTracking</code>) labeling jobs for this task type.</p> </li> <li> <p>3D point cloud semantic segmentation (<code>3DPointCloudSemanticSegmentation</code>), and adjustment and verification (<code>Adjustment3DPointCloudSemanticSegmentation</code>) labeling jobs for this task type. </p> </li> <li> <p>3D point cloud object tracking (<code>3DPointCloudObjectTracking</code>), and adjustment and verification (<code>Adjustment3DPointCloudObjectTracking</code>) labeling jobs for this task type. </p> </li> </ul> </li> </ul> <p/> <important> <p>If you are creating an adjustment or verification labeling job, you must use a <i>different</i> <code>LabelAttributeName</code> than the one used in the original labeling job. The original labeling job is the Ground Truth labeling job that produced the labels that you want verified or adjusted. To learn more about adjustment and verification labeling jobs, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/sms-verification-data.html">Verify and Adjust Labels</a>.</p> </important>
    input_config = "value"  # <p>Input data for the labeling job, such as the Amazon S3 location of the data objects and the location of the manifest file that describes the data objects.</p> <p>You must specify at least one of the following: <code>S3DataSource</code> or <code>SnsDataSource</code>. </p> <ul> <li> <p>Use <code>SnsDataSource</code> to specify an SNS input topic for a streaming labeling job. If you do not specify and SNS input topic ARN, Ground Truth will create a one-time labeling job that stops after all data objects in the input manifest file have been labeled.</p> </li> <li> <p>Use <code>S3DataSource</code> to specify an input manifest file for both streaming and one-time labeling jobs. Adding an <code>S3DataSource</code> is optional if you use <code>SnsDataSource</code> to create a streaming labeling job.</p> </li> </ul> <p>If you use the Amazon Mechanical Turk workforce, your input data should not include confidential information, personal information or protected health information. Use <code>ContentClassifiers</code> to specify that your data is free of personally identifiable information and adult content.</p>
    role_arn = "value"  # <p>The Amazon Resource Number (ARN) that Amazon SageMaker assumes to perform tasks on your behalf during data labeling. You must grant this role the necessary permissions so that Amazon SageMaker can successfully complete data labeling.</p>
}

# Access labeling_job outputs
labeling_job_id = labeling_job.id
labeling_job_labeling_job_status = labeling_job.labeling_job_status
labeling_job_label_counters = labeling_job.label_counters
labeling_job_creation_time = labeling_job.creation_time
labeling_job_labeling_job_arn = labeling_job.labeling_job_arn
labeling_job_label_category_config_s3_uri = labeling_job.label_category_config_s3_uri
labeling_job_tags = labeling_job.tags
labeling_job_role_arn = labeling_job.role_arn
labeling_job_input_config = labeling_job.input_config
labeling_job_labeling_job_output = labeling_job.labeling_job_output
labeling_job_labeling_job_algorithms_config = labeling_job.labeling_job_algorithms_config
labeling_job_labeling_job_name = labeling_job.labeling_job_name
labeling_job_last_modified_time = labeling_job.last_modified_time
labeling_job_failure_reason = labeling_job.failure_reason
labeling_job_job_reference_code = labeling_job.job_reference_code
labeling_job_label_attribute_name = labeling_job.label_attribute_name
labeling_job_stopping_conditions = labeling_job.stopping_conditions
labeling_job_human_task_config = labeling_job.human_task_config
labeling_job_output_config = labeling_job.output_config
```

---


### Model_bias_job_definition

ModelBiasJobDefinition resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `model_bias_job_output_config` | String | ✅ |  |
| `model_bias_baseline_config` | String |  | <p>The baseline configuration for a model bias job.</p> |
| `network_config` | String |  | <p>Networking options for a model bias job.</p> |
| `job_definition_name` | String | ✅ | <p>The name of the bias job definition. The name must be unique within an Amazon Web Services Region in the Amazon Web Services account.</p> |
| `tags` | Vec<String> |  | <p>(Optional) An array of key-value pairs. For more information, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html#allocation-whatURL"> Using Cost Allocation Tags</a> in the <i>Amazon Web Services Billing and Cost Management User Guide</i>.</p> |
| `model_bias_app_specification` | String | ✅ | <p>Configures the model bias job to run a specified Docker container image.</p> |
| `job_resources` | String | ✅ |  |
| `stopping_condition` | String |  |  |
| `role_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of an IAM role that Amazon SageMaker AI can assume to perform tasks on your behalf.</p> |
| `model_bias_job_input` | String | ✅ | <p>Inputs for the model bias job.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `model_bias_baseline_config` | String | <p>The baseline configuration for a model bias job.</p> |
| `model_bias_job_output_config` | String |  |
| `job_resources` | String |  |
| `network_config` | String | <p>Networking options for a model bias job.</p> |
| `stopping_condition` | String |  |
| `creation_time` | String | <p>The time at which the model bias job was created.</p> |
| `job_definition_arn` | String | <p>The Amazon Resource Name (ARN) of the model bias job.</p> |
| `role_arn` | String | <p>The Amazon Resource Name (ARN) of the IAM role that has read permission to the input data location and write permission to the output data location in Amazon S3.</p> |
| `job_definition_name` | String | <p>The name of the bias job definition. The name must be unique within an Amazon Web Services Region in the Amazon Web Services account.</p> |
| `model_bias_app_specification` | String | <p>Configures the model bias job to run a specified Docker container image.</p> |
| `model_bias_job_input` | String | <p>Inputs for the model bias job.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create model_bias_job_definition
model_bias_job_definition = provider.sagemaker.Model_bias_job_definition {
    model_bias_job_output_config = "value"  # Required field
    job_definition_name = "value"  # <p>The name of the bias job definition. The name must be unique within an Amazon Web Services Region in the Amazon Web Services account.</p>
    model_bias_app_specification = "value"  # <p>Configures the model bias job to run a specified Docker container image.</p>
    job_resources = "value"  # Required field
    role_arn = "value"  # <p>The Amazon Resource Name (ARN) of an IAM role that Amazon SageMaker AI can assume to perform tasks on your behalf.</p>
    model_bias_job_input = "value"  # <p>Inputs for the model bias job.</p>
}

# Access model_bias_job_definition outputs
model_bias_job_definition_id = model_bias_job_definition.id
model_bias_job_definition_model_bias_baseline_config = model_bias_job_definition.model_bias_baseline_config
model_bias_job_definition_model_bias_job_output_config = model_bias_job_definition.model_bias_job_output_config
model_bias_job_definition_job_resources = model_bias_job_definition.job_resources
model_bias_job_definition_network_config = model_bias_job_definition.network_config
model_bias_job_definition_stopping_condition = model_bias_job_definition.stopping_condition
model_bias_job_definition_creation_time = model_bias_job_definition.creation_time
model_bias_job_definition_job_definition_arn = model_bias_job_definition.job_definition_arn
model_bias_job_definition_role_arn = model_bias_job_definition.role_arn
model_bias_job_definition_job_definition_name = model_bias_job_definition.job_definition_name
model_bias_job_definition_model_bias_app_specification = model_bias_job_definition.model_bias_app_specification
model_bias_job_definition_model_bias_job_input = model_bias_job_definition.model_bias_job_input
```

---


### Notebook_instance

NotebookInstance resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `accelerator_types` | Vec<String> |  | <p>This parameter is no longer supported. Elastic Inference (EI) is no longer available.</p> <p>This parameter was used to specify a list of EI instance types to associate with this notebook instance.</p> |
| `platform_identifier` | String |  | <p>The platform identifier of the notebook instance runtime environment. The default value is <code>notebook-al2-v2</code>.</p> |
| `instance_type` | String | ✅ | <p>The type of ML compute instance to launch for the notebook instance.</p> |
| `kms_key_id` | String |  | <p>The Amazon Resource Name (ARN) of a Amazon Web Services Key Management Service key that SageMaker AI uses to encrypt data on the storage volume attached to your notebook instance. The KMS key you provide must be enabled. For information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/enabling-keys.html">Enabling and Disabling Keys</a> in the <i>Amazon Web Services Key Management Service Developer Guide</i>.</p> |
| `tags` | Vec<String> |  | <p>An array of key-value pairs. You can use tags to categorize your Amazon Web Services resources in different ways, for example, by purpose, owner, or environment. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services Resources</a>.</p> |
| `root_access` | String |  | <p>Whether root access is enabled or disabled for users of the notebook instance. The default value is <code>Enabled</code>.</p> <note> <p>Lifecycle configurations need root access to be able to set up a notebook instance. Because of this, lifecycle configurations associated with a notebook instance always run with root access even if you disable root access for users.</p> </note> |
| `default_code_repository` | String |  | <p>A Git repository to associate with the notebook instance as its default code repository. This can be either the name of a Git repository stored as a resource in your account, or the URL of a Git repository in <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/welcome.html">Amazon Web Services CodeCommit</a> or in any other Git repository. When you open a notebook instance, it opens in the directory that contains this repository. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/nbi-git-repo.html">Associating Git Repositories with SageMaker AI Notebook Instances</a>.</p> |
| `security_group_ids` | Vec<String> |  | <p>The VPC security group IDs, in the form sg-xxxxxxxx. The security groups must be for the same VPC as specified in the subnet. </p> |
| `instance_metadata_service_configuration` | String |  | <p>Information on the IMDS configuration of the notebook instance</p> |
| `additional_code_repositories` | Vec<String> |  | <p>An array of up to three Git repositories to associate with the notebook instance. These can be either the names of Git repositories stored as resources in your account, or the URL of Git repositories in <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/welcome.html">Amazon Web Services CodeCommit</a> or in any other Git repository. These repositories are cloned at the same level as the default repository of your notebook instance. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/nbi-git-repo.html">Associating Git Repositories with SageMaker AI Notebook Instances</a>.</p> |
| `direct_internet_access` | String |  | <p>Sets whether SageMaker AI provides internet access to the notebook instance. If you set this to <code>Disabled</code> this notebook instance is able to access resources only in your VPC, and is not be able to connect to SageMaker AI training and endpoint services unless you configure a NAT Gateway in your VPC.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/appendix-additional-considerations.html#appendix-notebook-and-internet-access">Notebook Instances Are Internet-Enabled by Default</a>. You can set the value of this parameter to <code>Disabled</code> only if you set a value for the <code>SubnetId</code> parameter.</p> |
| `subnet_id` | String |  | <p>The ID of the subnet in a VPC to which you would like to have a connectivity from your ML compute instance. </p> |
| `ip_address_type` | String |  | <p>The IP address type for the notebook instance. Specify <code>ipv4</code> for IPv4-only connectivity or <code>dualstack</code> for both IPv4 and IPv6 connectivity. When you specify <code>dualstack</code>, the subnet must support IPv6 CIDR blocks. If not specified, defaults to <code>ipv4</code>.</p> |
| `role_arn` | String | ✅ | <p> When you send any requests to Amazon Web Services resources from the notebook instance, SageMaker AI assumes this role to perform tasks on your behalf. You must grant this role necessary permissions so SageMaker AI can perform these tasks. The policy must allow the SageMaker AI service principal (sagemaker.amazonaws.com) permissions to assume this role. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/sagemaker-roles.html">SageMaker AI Roles</a>. </p> <note> <p>To be able to pass this role to SageMaker AI, the caller of this API must have the <code>iam:PassRole</code> permission.</p> </note> |
| `volume_size_in_gb` | i64 |  | <p>The size, in GB, of the ML storage volume to attach to the notebook instance. The default value is 5 GB.</p> |
| `lifecycle_config_name` | String |  | <p>The name of a lifecycle configuration to associate with the notebook instance. For information about lifestyle configurations, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/notebook-lifecycle-config.html">Step 2.1: (Optional) Customize a Notebook Instance</a>.</p> |
| `notebook_instance_name` | String | ✅ | <p>The name of the new notebook instance.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `creation_time` | String | <p>A timestamp. Use this parameter to return the time when the notebook instance was created</p> |
| `role_arn` | String | <p>The Amazon Resource Name (ARN) of the IAM role associated with the instance. </p> |
| `security_groups` | Vec<String> | <p>The IDs of the VPC security groups.</p> |
| `instance_metadata_service_configuration` | String | <p>Information on the IMDS configuration of the notebook instance</p> |
| `platform_identifier` | String | <p>The platform identifier of the notebook instance runtime environment.</p> |
| `kms_key_id` | String | <p>The Amazon Web Services KMS key ID SageMaker AI uses to encrypt data when storing it on the ML storage volume attached to the instance. </p> |
| `instance_type` | String | <p>The type of ML compute instance running on the notebook instance.</p> |
| `failure_reason` | String | <p>If status is <code>Failed</code>, the reason it failed.</p> |
| `url` | String | <p>The URL that you use to connect to the Jupyter notebook that is running in your notebook instance. </p> |
| `volume_size_in_gb` | i64 | <p>The size, in GB, of the ML storage volume attached to the notebook instance.</p> |
| `notebook_instance_lifecycle_config_name` | String | <p>Returns the name of a notebook instance lifecycle configuration.</p> <p>For information about notebook instance lifestyle configurations, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/notebook-lifecycle-config.html">Step 2.1: (Optional) Customize a Notebook Instance</a> </p> |
| `accelerator_types` | Vec<String> | <p>This parameter is no longer supported. Elastic Inference (EI) is no longer available.</p> <p>This parameter was used to specify a list of the EI instance types associated with this notebook instance.</p> |
| `ip_address_type` | String | <p>The IP address type configured for the notebook instance. Returns <code>ipv4</code> for IPv4-only connectivity or <code>dualstack</code> for both IPv4 and IPv6 connectivity.</p> |
| `network_interface_id` | String | <p>The network interface IDs that SageMaker AI created at the time of creating the instance. </p> |
| `last_modified_time` | String | <p>A timestamp. Use this parameter to retrieve the time when the notebook instance was last modified. </p> |
| `notebook_instance_name` | String | <p>The name of the SageMaker AI notebook instance. </p> |
| `notebook_instance_arn` | String | <p>The Amazon Resource Name (ARN) of the notebook instance.</p> |
| `direct_internet_access` | String | <p>Describes whether SageMaker AI provides internet access to the notebook instance. If this value is set to <i>Disabled</i>, the notebook instance does not have internet access, and cannot connect to SageMaker AI training and endpoint services.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/appendix-additional-considerations.html#appendix-notebook-and-internet-access">Notebook Instances Are Internet-Enabled by Default</a>.</p> |
| `subnet_id` | String | <p>The ID of the VPC subnet.</p> |
| `additional_code_repositories` | Vec<String> | <p>An array of up to three Git repositories associated with the notebook instance. These can be either the names of Git repositories stored as resources in your account, or the URL of Git repositories in <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/welcome.html">Amazon Web Services CodeCommit</a> or in any other Git repository. These repositories are cloned at the same level as the default repository of your notebook instance. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/nbi-git-repo.html">Associating Git Repositories with SageMaker AI Notebook Instances</a>.</p> |
| `root_access` | String | <p>Whether root access is enabled or disabled for users of the notebook instance.</p> <note> <p>Lifecycle configurations need root access to be able to set up a notebook instance. Because of this, lifecycle configurations associated with a notebook instance always run with root access even if you disable root access for users.</p> </note> |
| `default_code_repository` | String | <p>The Git repository associated with the notebook instance as its default code repository. This can be either the name of a Git repository stored as a resource in your account, or the URL of a Git repository in <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/welcome.html">Amazon Web Services CodeCommit</a> or in any other Git repository. When you open a notebook instance, it opens in the directory that contains this repository. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/nbi-git-repo.html">Associating Git Repositories with SageMaker AI Notebook Instances</a>.</p> |
| `notebook_instance_status` | String | <p>The status of the notebook instance.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create notebook_instance
notebook_instance = provider.sagemaker.Notebook_instance {
    instance_type = "value"  # <p>The type of ML compute instance to launch for the notebook instance.</p>
    role_arn = "value"  # <p> When you send any requests to Amazon Web Services resources from the notebook instance, SageMaker AI assumes this role to perform tasks on your behalf. You must grant this role necessary permissions so SageMaker AI can perform these tasks. The policy must allow the SageMaker AI service principal (sagemaker.amazonaws.com) permissions to assume this role. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/sagemaker-roles.html">SageMaker AI Roles</a>. </p> <note> <p>To be able to pass this role to SageMaker AI, the caller of this API must have the <code>iam:PassRole</code> permission.</p> </note>
    notebook_instance_name = "value"  # <p>The name of the new notebook instance.</p>
}

# Access notebook_instance outputs
notebook_instance_id = notebook_instance.id
notebook_instance_creation_time = notebook_instance.creation_time
notebook_instance_role_arn = notebook_instance.role_arn
notebook_instance_security_groups = notebook_instance.security_groups
notebook_instance_instance_metadata_service_configuration = notebook_instance.instance_metadata_service_configuration
notebook_instance_platform_identifier = notebook_instance.platform_identifier
notebook_instance_kms_key_id = notebook_instance.kms_key_id
notebook_instance_instance_type = notebook_instance.instance_type
notebook_instance_failure_reason = notebook_instance.failure_reason
notebook_instance_url = notebook_instance.url
notebook_instance_volume_size_in_gb = notebook_instance.volume_size_in_gb
notebook_instance_notebook_instance_lifecycle_config_name = notebook_instance.notebook_instance_lifecycle_config_name
notebook_instance_accelerator_types = notebook_instance.accelerator_types
notebook_instance_ip_address_type = notebook_instance.ip_address_type
notebook_instance_network_interface_id = notebook_instance.network_interface_id
notebook_instance_last_modified_time = notebook_instance.last_modified_time
notebook_instance_notebook_instance_name = notebook_instance.notebook_instance_name
notebook_instance_notebook_instance_arn = notebook_instance.notebook_instance_arn
notebook_instance_direct_internet_access = notebook_instance.direct_internet_access
notebook_instance_subnet_id = notebook_instance.subnet_id
notebook_instance_additional_code_repositories = notebook_instance.additional_code_repositories
notebook_instance_root_access = notebook_instance.root_access
notebook_instance_default_code_repository = notebook_instance.default_code_repository
notebook_instance_notebook_instance_status = notebook_instance.notebook_instance_status
```

---


### Data_quality_job_definition

DataQualityJobDefinition resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `job_resources` | String | ✅ |  |
| `stopping_condition` | String |  |  |
| `data_quality_job_output_config` | String | ✅ |  |
| `data_quality_baseline_config` | String |  | <p>Configures the constraints and baselines for the monitoring job.</p> |
| `role_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of an IAM role that Amazon SageMaker AI can assume to perform tasks on your behalf.</p> |
| `data_quality_job_input` | String | ✅ | <p>A list of inputs for the monitoring job. Currently endpoints are supported as monitoring inputs.</p> |
| `job_definition_name` | String | ✅ | <p>The name for the monitoring job definition.</p> |
| `network_config` | String |  | <p>Specifies networking configuration for the monitoring job.</p> |
| `tags` | Vec<String> |  | <p>(Optional) An array of key-value pairs. For more information, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html#allocation-whatURL"> Using Cost Allocation Tags</a> in the <i>Amazon Web Services Billing and Cost Management User Guide</i>.</p> |
| `data_quality_app_specification` | String | ✅ | <p>Specifies the container that runs the monitoring job.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `job_definition_name` | String | <p>The name of the data quality monitoring job definition.</p> |
| `job_definition_arn` | String | <p>The Amazon Resource Name (ARN) of the data quality monitoring job definition.</p> |
| `data_quality_baseline_config` | String | <p>The constraints and baselines for the data quality monitoring job definition.</p> |
| `role_arn` | String | <p>The Amazon Resource Name (ARN) of an IAM role that Amazon SageMaker AI can assume to perform tasks on your behalf.</p> |
| `creation_time` | String | <p>The time that the data quality monitoring job definition was created.</p> |
| `data_quality_job_output_config` | String |  |
| `network_config` | String | <p>The networking configuration for the data quality monitoring job.</p> |
| `stopping_condition` | String |  |
| `data_quality_app_specification` | String | <p>Information about the container that runs the data quality monitoring job.</p> |
| `data_quality_job_input` | String | <p>The list of inputs for the data quality monitoring job. Currently endpoints are supported.</p> |
| `job_resources` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create data_quality_job_definition
data_quality_job_definition = provider.sagemaker.Data_quality_job_definition {
    job_resources = "value"  # Required field
    data_quality_job_output_config = "value"  # Required field
    role_arn = "value"  # <p>The Amazon Resource Name (ARN) of an IAM role that Amazon SageMaker AI can assume to perform tasks on your behalf.</p>
    data_quality_job_input = "value"  # <p>A list of inputs for the monitoring job. Currently endpoints are supported as monitoring inputs.</p>
    job_definition_name = "value"  # <p>The name for the monitoring job definition.</p>
    data_quality_app_specification = "value"  # <p>Specifies the container that runs the monitoring job.</p>
}

# Access data_quality_job_definition outputs
data_quality_job_definition_id = data_quality_job_definition.id
data_quality_job_definition_job_definition_name = data_quality_job_definition.job_definition_name
data_quality_job_definition_job_definition_arn = data_quality_job_definition.job_definition_arn
data_quality_job_definition_data_quality_baseline_config = data_quality_job_definition.data_quality_baseline_config
data_quality_job_definition_role_arn = data_quality_job_definition.role_arn
data_quality_job_definition_creation_time = data_quality_job_definition.creation_time
data_quality_job_definition_data_quality_job_output_config = data_quality_job_definition.data_quality_job_output_config
data_quality_job_definition_network_config = data_quality_job_definition.network_config
data_quality_job_definition_stopping_condition = data_quality_job_definition.stopping_condition
data_quality_job_definition_data_quality_app_specification = data_quality_job_definition.data_quality_app_specification
data_quality_job_definition_data_quality_job_input = data_quality_job_definition.data_quality_job_input
data_quality_job_definition_job_resources = data_quality_job_definition.job_resources
```

---


### Presigned_domain_url

PresignedDomainUrl resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `session_expiration_duration_in_seconds` | i64 |  | <p>The session expiration duration in seconds. This value defaults to 43200.</p> |
| `domain_id` | String | ✅ | <p>The domain ID.</p> |
| `expires_in_seconds` | i64 |  | <p>The number of seconds until the pre-signed URL expires. This value defaults to 300.</p> |
| `landing_uri` | String |  | <p>The landing page that the user is directed to when accessing the presigned URL. Using this value, users can access Studio or Studio Classic, even if it is not the default experience for the domain. The supported values are:</p> <ul> <li> <p> <code>studio::relative/path</code>: Directs users to the relative path in Studio.</p> </li> <li> <p> <code>app:JupyterServer:relative/path</code>: Directs users to the relative path in the Studio Classic application.</p> </li> <li> <p> <code>app:JupyterLab:relative/path</code>: Directs users to the relative path in the JupyterLab application.</p> </li> <li> <p> <code>app:RStudioServerPro:relative/path</code>: Directs users to the relative path in the RStudio application.</p> </li> <li> <p> <code>app:CodeEditor:relative/path</code>: Directs users to the relative path in the Code Editor, based on Code-OSS, Visual Studio Code - Open Source application.</p> </li> <li> <p> <code>app:Canvas:relative/path</code>: Directs users to the relative path in the Canvas application.</p> </li> </ul> |
| `space_name` | String |  | <p>The name of the space.</p> |
| `user_profile_name` | String | ✅ | <p>The name of the UserProfile to sign-in as.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create presigned_domain_url
presigned_domain_url = provider.sagemaker.Presigned_domain_url {
    domain_id = "value"  # <p>The domain ID.</p>
    user_profile_name = "value"  # <p>The name of the UserProfile to sign-in as.</p>
}

```

---


### Cluster_event

ClusterEvent resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `event_details` | String | <p>Detailed information about the requested cluster event, including event metadata for various resource types such as <code>Cluster</code>, <code>InstanceGroup</code>, <code>Instance</code>, and their associated attributes.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access cluster_event outputs
cluster_event_id = cluster_event.id
cluster_event_event_details = cluster_event.event_details
```

---


### Compute_quota

ComputeQuota resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `compute_quota_config` | String | ✅ | <p>Configuration of the compute allocation definition. This includes the resource sharing option, and the setting to preempt low priority tasks.</p> |
| `cluster_arn` | String | ✅ | <p>ARN of the cluster.</p> |
| `compute_quota_target` | String | ✅ | <p>The target entity to allocate compute resources to.</p> |
| `activation_state` | String |  | <p>The state of the compute allocation being described. Use to enable or disable compute allocation.</p> <p>Default is <code>Enabled</code>.</p> |
| `description` | String |  | <p>Description of the compute allocation definition.</p> |
| `tags` | Vec<String> |  | <p>Tags of the compute allocation definition.</p> |
| `name` | String | ✅ | <p>Name to the compute allocation definition.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `activation_state` | String | <p>The state of the compute allocation being described. Use to enable or disable compute allocation.</p> <p>Default is <code>Enabled</code>.</p> |
| `status` | String | <p>Status of the compute allocation definition.</p> |
| `last_modified_by` | String |  |
| `description` | String | <p>Description of the compute allocation definition.</p> |
| `failure_reason` | String | <p>Failure reason of the compute allocation definition.</p> |
| `name` | String | <p>Name of the compute allocation definition.</p> |
| `compute_quota_target` | String | <p>The target entity to allocate compute resources to.</p> |
| `created_by` | String |  |
| `last_modified_time` | String | <p>Last modified time of the compute allocation configuration.</p> |
| `creation_time` | String | <p>Creation time of the compute allocation configuration.</p> |
| `compute_quota_arn` | String | <p>ARN of the compute allocation definition.</p> |
| `cluster_arn` | String | <p>ARN of the cluster.</p> |
| `compute_quota_version` | i64 | <p>Version of the compute allocation definition.</p> |
| `compute_quota_id` | String | <p>ID of the compute allocation definition.</p> |
| `compute_quota_config` | String | <p>Configuration of the compute allocation definition. This includes the resource sharing option, and the setting to preempt low priority tasks.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create compute_quota
compute_quota = provider.sagemaker.Compute_quota {
    compute_quota_config = "value"  # <p>Configuration of the compute allocation definition. This includes the resource sharing option, and the setting to preempt low priority tasks.</p>
    cluster_arn = "value"  # <p>ARN of the cluster.</p>
    compute_quota_target = "value"  # <p>The target entity to allocate compute resources to.</p>
    name = "value"  # <p>Name to the compute allocation definition.</p>
}

# Access compute_quota outputs
compute_quota_id = compute_quota.id
compute_quota_activation_state = compute_quota.activation_state
compute_quota_status = compute_quota.status
compute_quota_last_modified_by = compute_quota.last_modified_by
compute_quota_description = compute_quota.description
compute_quota_failure_reason = compute_quota.failure_reason
compute_quota_name = compute_quota.name
compute_quota_compute_quota_target = compute_quota.compute_quota_target
compute_quota_created_by = compute_quota.created_by
compute_quota_last_modified_time = compute_quota.last_modified_time
compute_quota_creation_time = compute_quota.creation_time
compute_quota_compute_quota_arn = compute_quota.compute_quota_arn
compute_quota_cluster_arn = compute_quota.cluster_arn
compute_quota_compute_quota_version = compute_quota.compute_quota_version
compute_quota_compute_quota_id = compute_quota.compute_quota_id
compute_quota_compute_quota_config = compute_quota.compute_quota_config
```

---


### Hub

Hub resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `hub_search_keywords` | Vec<String> |  | <p>The searchable keywords for the hub.</p> |
| `hub_description` | String | ✅ | <p>A description of the hub.</p> |
| `s3_storage_config` | String |  | <p>The Amazon S3 storage configuration for the hub.</p> |
| `hub_name` | String | ✅ | <p>The name of the hub to create.</p> |
| `hub_display_name` | String |  | <p>The display name of the hub.</p> |
| `tags` | Vec<String> |  | <p>Any tags to associate with the hub.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `hub_display_name` | String | <p>The display name of the hub.</p> |
| `hub_arn` | String | <p>The Amazon Resource Name (ARN) of the hub.</p> |
| `hub_description` | String | <p>A description of the hub.</p> |
| `hub_name` | String | <p>The name of the hub.</p> |
| `creation_time` | String | <p>The date and time that the hub was created.</p> |
| `hub_search_keywords` | Vec<String> | <p>The searchable keywords for the hub.</p> |
| `s3_storage_config` | String | <p>The Amazon S3 storage configuration for the hub.</p> |
| `failure_reason` | String | <p>The failure reason if importing hub content failed.</p> |
| `last_modified_time` | String | <p>The date and time that the hub was last modified.</p> |
| `hub_status` | String | <p>The status of the hub.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create hub
hub = provider.sagemaker.Hub {
    hub_description = "value"  # <p>A description of the hub.</p>
    hub_name = "value"  # <p>The name of the hub to create.</p>
}

# Access hub outputs
hub_id = hub.id
hub_hub_display_name = hub.hub_display_name
hub_hub_arn = hub.hub_arn
hub_hub_description = hub.hub_description
hub_hub_name = hub.hub_name
hub_creation_time = hub.creation_time
hub_hub_search_keywords = hub.hub_search_keywords
hub_s3_storage_config = hub.s3_storage_config
hub_failure_reason = hub.failure_reason
hub_last_modified_time = hub.last_modified_time
hub_hub_status = hub.hub_status
```

---


### Device

Device resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `device_fleet_name` | String | <p>The name of the fleet the device belongs to.</p> |
| `device_arn` | String | <p>The Amazon Resource Name (ARN) of the device.</p> |
| `models` | Vec<String> | <p>Models on the device.</p> |
| `description` | String | <p>A description of the device.</p> |
| `agent_version` | String | <p>Edge Manager agent version.</p> |
| `latest_heartbeat` | String | <p>The last heartbeat received from the device.</p> |
| `next_token` | String | <p>The response from the last list when returning a list large enough to need tokening.</p> |
| `registration_time` | String | <p>The timestamp of the last registration or de-reregistration.</p> |
| `iot_thing_name` | String | <p>The Amazon Web Services Internet of Things (IoT) object thing name associated with the device.</p> |
| `device_name` | String | <p>The unique identifier of the device.</p> |
| `max_models` | i64 | <p>The maximum number of models.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access device outputs
device_id = device.id
device_device_fleet_name = device.device_fleet_name
device_device_arn = device.device_arn
device_models = device.models
device_description = device.description
device_agent_version = device.agent_version
device_latest_heartbeat = device.latest_heartbeat
device_next_token = device.next_token
device_registration_time = device.registration_time
device_iot_thing_name = device.iot_thing_name
device_device_name = device.device_name
device_max_models = device.max_models
```

---


### Endpoint

Endpoint resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `endpoint_name` | String | ✅ | <p>The name of the endpoint.The name must be unique within an Amazon Web Services Region in your Amazon Web Services account. The name is case-insensitive in <code>CreateEndpoint</code>, but the case is preserved and must be matched in <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_runtime_InvokeEndpoint.html">InvokeEndpoint</a>.</p> |
| `endpoint_config_name` | String | ✅ | <p>The name of an endpoint configuration. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_CreateEndpointConfig.html">CreateEndpointConfig</a>. </p> |
| `tags` | Vec<String> |  | <p>An array of key-value pairs. You can use tags to categorize your Amazon Web Services resources in different ways, for example, by purpose, owner, or environment. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services Resources</a>.</p> |
| `deployment_config` | String |  |  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `endpoint_arn` | String | <p>The Amazon Resource Name (ARN) of the endpoint.</p> |
| `explainer_config` | String | <p>The configuration parameters for an explainer.</p> |
| `production_variants` | Vec<String> | <p>An array of <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_ProductionVariantSummary.html">ProductionVariantSummary</a> objects, one for each model hosted behind this endpoint.</p> |
| `endpoint_status` | String | <p>The status of the endpoint.</p> <ul> <li> <p> <code>OutOfService</code>: Endpoint is not available to take incoming requests.</p> </li> <li> <p> <code>Creating</code>: <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_CreateEndpoint.html">CreateEndpoint</a> is executing.</p> </li> <li> <p> <code>Updating</code>: <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_UpdateEndpoint.html">UpdateEndpoint</a> or <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_UpdateEndpointWeightsAndCapacities.html">UpdateEndpointWeightsAndCapacities</a> is executing.</p> </li> <li> <p> <code>SystemUpdating</code>: Endpoint is undergoing maintenance and cannot be updated or deleted or re-scaled until it has completed. This maintenance operation does not change any customer-specified values such as VPC config, KMS encryption, model, instance type, or instance count.</p> </li> <li> <p> <code>RollingBack</code>: Endpoint fails to scale up or down or change its variant weight and is in the process of rolling back to its previous configuration. Once the rollback completes, endpoint returns to an <code>InService</code> status. This transitional status only applies to an endpoint that has autoscaling enabled and is undergoing variant weight or capacity changes as part of an <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_UpdateEndpointWeightsAndCapacities.html">UpdateEndpointWeightsAndCapacities</a> call or when the <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_UpdateEndpointWeightsAndCapacities.html">UpdateEndpointWeightsAndCapacities</a> operation is called explicitly.</p> </li> <li> <p> <code>InService</code>: Endpoint is available to process incoming requests.</p> </li> <li> <p> <code>Deleting</code>: <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_DeleteEndpoint.html">DeleteEndpoint</a> is executing.</p> </li> <li> <p> <code>Failed</code>: Endpoint could not be created, updated, or re-scaled. Use the <code>FailureReason</code> value returned by <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_DescribeEndpoint.html">DescribeEndpoint</a> for information about the failure. <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_DeleteEndpoint.html">DeleteEndpoint</a> is the only operation that can be performed on a failed endpoint.</p> </li> <li> <p> <code>UpdateRollbackFailed</code>: Both the rolling deployment and auto-rollback failed. Your endpoint is in service with a mix of the old and new endpoint configurations. For information about how to remedy this issue and restore the endpoint's status to <code>InService</code>, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/deployment-guardrails-rolling.html">Rolling Deployments</a>.</p> </li> </ul> |
| `endpoint_config_name` | String | <p>The name of the endpoint configuration associated with this endpoint.</p> |
| `last_deployment_config` | String | <p>The most recent deployment configuration for the endpoint.</p> |
| `pending_deployment_summary` | String | <p>Returns the summary of an in-progress deployment. This field is only returned when the endpoint is creating or updating with a new endpoint configuration.</p> |
| `endpoint_name` | String | <p>Name of the endpoint.</p> |
| `async_inference_config` | String | <p>Returns the description of an endpoint configuration created using the <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_CreateEndpointConfig.html"> <code>CreateEndpointConfig</code> </a> API.</p> |
| `failure_reason` | String | <p>If the status of the endpoint is <code>Failed</code>, the reason why it failed. </p> |
| `data_capture_config` | String |  |
| `last_modified_time` | String | <p>A timestamp that shows when the endpoint was last modified.</p> |
| `shadow_production_variants` | Vec<String> | <p>An array of <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_ProductionVariantSummary.html">ProductionVariantSummary</a> objects, one for each model that you want to host at this endpoint in shadow mode with production traffic replicated from the model specified on <code>ProductionVariants</code>.</p> |
| `creation_time` | String | <p>A timestamp that shows when the endpoint was created.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create endpoint
endpoint = provider.sagemaker.Endpoint {
    endpoint_name = "value"  # <p>The name of the endpoint.The name must be unique within an Amazon Web Services Region in your Amazon Web Services account. The name is case-insensitive in <code>CreateEndpoint</code>, but the case is preserved and must be matched in <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_runtime_InvokeEndpoint.html">InvokeEndpoint</a>.</p>
    endpoint_config_name = "value"  # <p>The name of an endpoint configuration. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_CreateEndpointConfig.html">CreateEndpointConfig</a>. </p>
}

# Access endpoint outputs
endpoint_id = endpoint.id
endpoint_endpoint_arn = endpoint.endpoint_arn
endpoint_explainer_config = endpoint.explainer_config
endpoint_production_variants = endpoint.production_variants
endpoint_endpoint_status = endpoint.endpoint_status
endpoint_endpoint_config_name = endpoint.endpoint_config_name
endpoint_last_deployment_config = endpoint.last_deployment_config
endpoint_pending_deployment_summary = endpoint.pending_deployment_summary
endpoint_endpoint_name = endpoint.endpoint_name
endpoint_async_inference_config = endpoint.async_inference_config
endpoint_failure_reason = endpoint.failure_reason
endpoint_data_capture_config = endpoint.data_capture_config
endpoint_last_modified_time = endpoint.last_modified_time
endpoint_shadow_production_variants = endpoint.shadow_production_variants
endpoint_creation_time = endpoint.creation_time
```

---


### Pipeline_execution

PipelineExecution resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `parallelism_configuration` | String |  | <p>This configuration, if specified, overrides the parallelism configuration of the parent pipeline for this specific run.</p> |
| `pipeline_execution_description` | String |  | <p>The description of the pipeline execution.</p> |
| `pipeline_execution_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the pipeline execution.</p> |
| `pipeline_execution_display_name` | String |  | <p>The display name of the pipeline execution.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `pipeline_experiment_config` | String |  |
| `pipeline_execution_description` | String | <p>The description of the pipeline execution.</p> |
| `created_by` | String |  |
| `pipeline_execution_arn` | String | <p>The Amazon Resource Name (ARN) of the pipeline execution.</p> |
| `pipeline_arn` | String | <p>The Amazon Resource Name (ARN) of the pipeline.</p> |
| `last_modified_by` | String |  |
| `pipeline_version_id` | i64 | <p>The ID of the pipeline version.</p> |
| `last_modified_time` | String | <p>The time when the pipeline execution was modified last.</p> |
| `parallelism_configuration` | String | <p>The parallelism configuration applied to the pipeline.</p> |
| `selective_execution_config` | String | <p>The selective execution configuration applied to the pipeline run.</p> |
| `pipeline_execution_display_name` | String | <p>The display name of the pipeline execution.</p> |
| `creation_time` | String | <p>The time when the pipeline execution was created.</p> |
| `failure_reason` | String | <p>If the execution failed, a message describing why.</p> |
| `pipeline_execution_status` | String | <p>The status of the pipeline execution.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access pipeline_execution outputs
pipeline_execution_id = pipeline_execution.id
pipeline_execution_pipeline_experiment_config = pipeline_execution.pipeline_experiment_config
pipeline_execution_pipeline_execution_description = pipeline_execution.pipeline_execution_description
pipeline_execution_created_by = pipeline_execution.created_by
pipeline_execution_pipeline_execution_arn = pipeline_execution.pipeline_execution_arn
pipeline_execution_pipeline_arn = pipeline_execution.pipeline_arn
pipeline_execution_last_modified_by = pipeline_execution.last_modified_by
pipeline_execution_pipeline_version_id = pipeline_execution.pipeline_version_id
pipeline_execution_last_modified_time = pipeline_execution.last_modified_time
pipeline_execution_parallelism_configuration = pipeline_execution.parallelism_configuration
pipeline_execution_selective_execution_config = pipeline_execution.selective_execution_config
pipeline_execution_pipeline_execution_display_name = pipeline_execution.pipeline_execution_display_name
pipeline_execution_creation_time = pipeline_execution.creation_time
pipeline_execution_failure_reason = pipeline_execution.failure_reason
pipeline_execution_pipeline_execution_status = pipeline_execution.pipeline_execution_status
```

---


### Processing_job

ProcessingJob resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `experiment_config` | String |  |  |
| `processing_inputs` | Vec<String> |  | <p>An array of inputs configuring the data to download into the processing container.</p> |
| `role_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of an IAM role that Amazon SageMaker can assume to perform tasks on your behalf.</p> |
| `processing_output_config` | String |  | <p>Output configuration for the processing job.</p> |
| `processing_job_name` | String | ✅ | <p> The name of the processing job. The name must be unique within an Amazon Web Services Region in the Amazon Web Services account.</p> |
| `stopping_condition` | String |  | <p>The time limit for how long the processing job is allowed to run.</p> |
| `app_specification` | String | ✅ | <p>Configures the processing job to run a specified Docker container image.</p> |
| `tags` | Vec<String> |  | <p>(Optional) An array of key-value pairs. For more information, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html#allocation-whatURL">Using Cost Allocation Tags</a> in the <i>Amazon Web Services Billing and Cost Management User Guide</i>.</p> <important> <p>Do not include any security-sensitive information including account access IDs, secrets, or tokens in any tags. As part of the shared responsibility model, you are responsible for any potential exposure, unauthorized access, or compromise of your sensitive data if caused by security-sensitive information included in the request tag variable or plain text fields.</p> </important> |
| `processing_resources` | String | ✅ | <p>Identifies the resources, ML compute instances, and ML storage volumes to deploy for a processing job. In distributed training, you specify more than one instance.</p> |
| `environment` | HashMap<String, String> |  | <p>The environment variables to set in the Docker container. Up to 100 key and values entries in the map are supported.</p> <important> <p>Do not include any security-sensitive information including account access IDs, secrets, or tokens in any environment fields. As part of the shared responsibility model, you are responsible for any potential exposure, unauthorized access, or compromise of your sensitive data if caused by security-sensitive information included in the request environment variable or plain text fields.</p> </important> |
| `network_config` | String |  | <p>Networking options for a processing job, such as whether to allow inbound and outbound network calls to and from processing containers, and the VPC subnets and security groups to use for VPC-enabled processing jobs.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `processing_job_name` | String | <p>The name of the processing job. The name must be unique within an Amazon Web Services Region in the Amazon Web Services account.</p> |
| `experiment_config` | String | <p>The configuration information used to create an experiment.</p> |
| `processing_output_config` | String | <p>Output configuration for the processing job.</p> |
| `processing_end_time` | String | <p>The time at which the processing job completed.</p> |
| `auto_ml_job_arn` | String | <p>The ARN of an AutoML job associated with this processing job.</p> |
| `creation_time` | String | <p>The time at which the processing job was created.</p> |
| `app_specification` | String | <p>Configures the processing job to run a specified container image.</p> |
| `last_modified_time` | String | <p>The time at which the processing job was last modified.</p> |
| `stopping_condition` | String | <p>The time limit for how long the processing job is allowed to run.</p> |
| `network_config` | String | <p>Networking options for a processing job.</p> |
| `role_arn` | String | <p>The Amazon Resource Name (ARN) of an IAM role that Amazon SageMaker can assume to perform tasks on your behalf.</p> |
| `processing_inputs` | Vec<String> | <p>The inputs for a processing job.</p> |
| `failure_reason` | String | <p>A string, up to one KB in size, that contains the reason a processing job failed, if it failed.</p> |
| `environment` | HashMap<String, String> | <p>The environment variables set in the Docker container.</p> |
| `monitoring_schedule_arn` | String | <p>The ARN of a monitoring schedule for an endpoint associated with this processing job.</p> |
| `training_job_arn` | String | <p>The ARN of a training job associated with this processing job.</p> |
| `exit_message` | String | <p>An optional string, up to one KB in size, that contains metadata from the processing container when the processing job exits.</p> |
| `processing_start_time` | String | <p>The time at which the processing job started.</p> |
| `processing_resources` | String | <p>Identifies the resources, ML compute instances, and ML storage volumes to deploy for a processing job. In distributed training, you specify more than one instance.</p> |
| `processing_job_arn` | String | <p>The Amazon Resource Name (ARN) of the processing job.</p> |
| `processing_job_status` | String | <p>Provides the status of a processing job.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create processing_job
processing_job = provider.sagemaker.Processing_job {
    role_arn = "value"  # <p>The Amazon Resource Name (ARN) of an IAM role that Amazon SageMaker can assume to perform tasks on your behalf.</p>
    processing_job_name = "value"  # <p> The name of the processing job. The name must be unique within an Amazon Web Services Region in the Amazon Web Services account.</p>
    app_specification = "value"  # <p>Configures the processing job to run a specified Docker container image.</p>
    processing_resources = "value"  # <p>Identifies the resources, ML compute instances, and ML storage volumes to deploy for a processing job. In distributed training, you specify more than one instance.</p>
}

# Access processing_job outputs
processing_job_id = processing_job.id
processing_job_processing_job_name = processing_job.processing_job_name
processing_job_experiment_config = processing_job.experiment_config
processing_job_processing_output_config = processing_job.processing_output_config
processing_job_processing_end_time = processing_job.processing_end_time
processing_job_auto_ml_job_arn = processing_job.auto_ml_job_arn
processing_job_creation_time = processing_job.creation_time
processing_job_app_specification = processing_job.app_specification
processing_job_last_modified_time = processing_job.last_modified_time
processing_job_stopping_condition = processing_job.stopping_condition
processing_job_network_config = processing_job.network_config
processing_job_role_arn = processing_job.role_arn
processing_job_processing_inputs = processing_job.processing_inputs
processing_job_failure_reason = processing_job.failure_reason
processing_job_environment = processing_job.environment
processing_job_monitoring_schedule_arn = processing_job.monitoring_schedule_arn
processing_job_training_job_arn = processing_job.training_job_arn
processing_job_exit_message = processing_job.exit_message
processing_job_processing_start_time = processing_job.processing_start_time
processing_job_processing_resources = processing_job.processing_resources
processing_job_processing_job_arn = processing_job.processing_job_arn
processing_job_processing_job_status = processing_job.processing_job_status
```

---


### Pipeline_definition_for_execution

PipelineDefinitionForExecution resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `creation_time` | String | <p>The time when the pipeline was created.</p> |
| `pipeline_definition` | String | <p>The JSON pipeline definition.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access pipeline_definition_for_execution outputs
pipeline_definition_for_execution_id = pipeline_definition_for_execution.id
pipeline_definition_for_execution_creation_time = pipeline_definition_for_execution.creation_time
pipeline_definition_for_execution_pipeline_definition = pipeline_definition_for_execution.pipeline_definition
```

---


### Monitoring_alert

MonitoringAlert resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `monitoring_schedule_name` | String | ✅ | <p>The name of a monitoring schedule.</p> |
| `monitoring_alert_name` | String | ✅ | <p>The name of a monitoring alert.</p> |
| `datapoints_to_alert` | i64 | ✅ | <p>Within <code>EvaluationPeriod</code>, how many execution failures will raise an alert.</p> |
| `evaluation_period` | i64 | ✅ | <p>The number of most recent monitoring executions to consider when evaluating alert status.</p> |



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


### Compilation_job

CompilationJob resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `output_config` | String | ✅ | <p>Provides information about the output location for the compiled model and the target device the model runs on.</p> |
| `model_package_version_arn` | String |  | <p>The Amazon Resource Name (ARN) of a versioned model package. Provide either a <code>ModelPackageVersionArn</code> or an <code>InputConfig</code> object in the request syntax. The presence of both objects in the <code>CreateCompilationJob</code> request will return an exception.</p> |
| `vpc_config` | String |  | <p>A <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_VpcConfig.html">VpcConfig</a> object that specifies the VPC that you want your compilation job to connect to. Control access to your models by configuring the VPC. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/neo-vpc.html">Protect Compilation Jobs by Using an Amazon Virtual Private Cloud</a>.</p> |
| `stopping_condition` | String | ✅ | <p>Specifies a limit to how long a model compilation job can run. When the job reaches the time limit, Amazon SageMaker AI ends the compilation job. Use this API to cap model training costs.</p> |
| `input_config` | String |  | <p>Provides information about the location of input model artifacts, the name and shape of the expected data inputs, and the framework in which the model was trained.</p> |
| `tags` | Vec<String> |  | <p>An array of key-value pairs. You can use tags to categorize your Amazon Web Services resources in different ways, for example, by purpose, owner, or environment. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services Resources</a>.</p> |
| `compilation_job_name` | String | ✅ | <p>A name for the model compilation job. The name must be unique within the Amazon Web Services Region and within your Amazon Web Services account. </p> |
| `role_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of an IAM role that enables Amazon SageMaker AI to perform tasks on your behalf. </p> <p>During model compilation, Amazon SageMaker AI needs your permission to:</p> <ul> <li> <p>Read input data from an S3 bucket</p> </li> <li> <p>Write model artifacts to an S3 bucket</p> </li> <li> <p>Write logs to Amazon CloudWatch Logs</p> </li> <li> <p>Publish metrics to Amazon CloudWatch</p> </li> </ul> <p>You grant permissions for all of these tasks to an IAM role. To pass this role to Amazon SageMaker AI, the caller of this API must have the <code>iam:PassRole</code> permission. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/sagemaker-roles.html">Amazon SageMaker AI Roles.</a> </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `inference_image` | String | <p>The inference image to use when compiling a model. Specify an image only if the target device is a cloud instance.</p> |
| `input_config` | String | <p>Information about the location in Amazon S3 of the input model artifacts, the name and shape of the expected data inputs, and the framework in which the model was trained.</p> |
| `model_artifacts` | String | <p>Information about the location in Amazon S3 that has been configured for storing the model artifacts used in the compilation job.</p> |
| `compilation_job_status` | String | <p>The status of the model compilation job.</p> |
| `model_digests` | String | <p>Provides a BLAKE2 hash value that identifies the compiled model artifacts in Amazon S3.</p> |
| `failure_reason` | String | <p>If a model compilation job failed, the reason it failed. </p> |
| `compilation_job_arn` | String | <p>The Amazon Resource Name (ARN) of the model compilation job.</p> |
| `role_arn` | String | <p>The Amazon Resource Name (ARN) of an IAM role that Amazon SageMaker AI assumes to perform the model compilation job.</p> |
| `stopping_condition` | String | <p>Specifies a limit to how long a model compilation job can run. When the job reaches the time limit, Amazon SageMaker AI ends the compilation job. Use this API to cap model training costs.</p> |
| `model_package_version_arn` | String | <p>The Amazon Resource Name (ARN) of the versioned model package that was provided to SageMaker Neo when you initiated a compilation job.</p> |
| `compilation_job_name` | String | <p>The name of the model compilation job.</p> |
| `creation_time` | String | <p>The time that the model compilation job was created.</p> |
| `derived_information` | String | <p>Information that SageMaker Neo automatically derived about the model.</p> |
| `output_config` | String | <p>Information about the output location for the compiled model and the target device that the model runs on.</p> |
| `vpc_config` | String | <p>A <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_VpcConfig.html">VpcConfig</a> object that specifies the VPC that you want your compilation job to connect to. Control access to your models by configuring the VPC. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/neo-vpc.html">Protect Compilation Jobs by Using an Amazon Virtual Private Cloud</a>.</p> |
| `compilation_end_time` | String | <p>The time when the model compilation job on a compilation job instance ended. For a successful or stopped job, this is when the job's model artifacts have finished uploading. For a failed job, this is when Amazon SageMaker AI detected that the job failed. </p> |
| `compilation_start_time` | String | <p>The time when the model compilation job started the <code>CompilationJob</code> instances. </p> <p>You are billed for the time between this timestamp and the timestamp in the <code>CompilationEndTime</code> field. In Amazon CloudWatch Logs, the start time might be later than this time. That's because it takes time to download the compilation job, which depends on the size of the compilation job container. </p> |
| `last_modified_time` | String | <p>The time that the status of the model compilation job was last modified.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create compilation_job
compilation_job = provider.sagemaker.Compilation_job {
    output_config = "value"  # <p>Provides information about the output location for the compiled model and the target device the model runs on.</p>
    stopping_condition = "value"  # <p>Specifies a limit to how long a model compilation job can run. When the job reaches the time limit, Amazon SageMaker AI ends the compilation job. Use this API to cap model training costs.</p>
    compilation_job_name = "value"  # <p>A name for the model compilation job. The name must be unique within the Amazon Web Services Region and within your Amazon Web Services account. </p>
    role_arn = "value"  # <p>The Amazon Resource Name (ARN) of an IAM role that enables Amazon SageMaker AI to perform tasks on your behalf. </p> <p>During model compilation, Amazon SageMaker AI needs your permission to:</p> <ul> <li> <p>Read input data from an S3 bucket</p> </li> <li> <p>Write model artifacts to an S3 bucket</p> </li> <li> <p>Write logs to Amazon CloudWatch Logs</p> </li> <li> <p>Publish metrics to Amazon CloudWatch</p> </li> </ul> <p>You grant permissions for all of these tasks to an IAM role. To pass this role to Amazon SageMaker AI, the caller of this API must have the <code>iam:PassRole</code> permission. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/sagemaker-roles.html">Amazon SageMaker AI Roles.</a> </p>
}

# Access compilation_job outputs
compilation_job_id = compilation_job.id
compilation_job_inference_image = compilation_job.inference_image
compilation_job_input_config = compilation_job.input_config
compilation_job_model_artifacts = compilation_job.model_artifacts
compilation_job_compilation_job_status = compilation_job.compilation_job_status
compilation_job_model_digests = compilation_job.model_digests
compilation_job_failure_reason = compilation_job.failure_reason
compilation_job_compilation_job_arn = compilation_job.compilation_job_arn
compilation_job_role_arn = compilation_job.role_arn
compilation_job_stopping_condition = compilation_job.stopping_condition
compilation_job_model_package_version_arn = compilation_job.model_package_version_arn
compilation_job_compilation_job_name = compilation_job.compilation_job_name
compilation_job_creation_time = compilation_job.creation_time
compilation_job_derived_information = compilation_job.derived_information
compilation_job_output_config = compilation_job.output_config
compilation_job_vpc_config = compilation_job.vpc_config
compilation_job_compilation_end_time = compilation_job.compilation_end_time
compilation_job_compilation_start_time = compilation_job.compilation_start_time
compilation_job_last_modified_time = compilation_job.last_modified_time
```

---


### Auto_ml_job

AutoMLJob resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `input_data_config` | Vec<String> | ✅ | <p>An array of channel objects that describes the input data and its location. Each channel is a named input source. Similar to <code>InputDataConfig</code> supported by <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_HyperParameterTrainingJobDefinition.html">HyperParameterTrainingJobDefinition</a>. Format(s) supported: CSV, Parquet. A minimum of 500 rows is required for the training dataset. There is not a minimum number of rows required for the validation dataset.</p> |
| `role_arn` | String | ✅ | <p>The ARN of the role that is used to access the data.</p> |
| `auto_ml_job_objective` | String |  | <p>Specifies a metric to minimize or maximize as the objective of a job. If not specified, the default objective metric depends on the problem type. See <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_AutoMLJobObjective.html">AutoMLJobObjective</a> for the default values.</p> |
| `auto_ml_job_name` | String | ✅ | <p>Identifies an Autopilot job. The name must be unique to your account and is case insensitive.</p> |
| `generate_candidate_definitions_only` | bool |  | <p>Generates possible candidates without training the models. A candidate is a combination of data preprocessors, algorithms, and algorithm parameter settings.</p> |
| `tags` | Vec<String> |  | <p>An array of key-value pairs. You can use tags to categorize your Amazon Web Services resources in different ways, for example, by purpose, owner, or environment. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web ServicesResources</a>. Tag keys must be unique per resource.</p> |
| `model_deploy_config` | String |  | <p>Specifies how to generate the endpoint name for an automatic one-click Autopilot model deployment.</p> |
| `problem_type` | String |  | <p>Defines the type of supervised learning problem available for the candidates. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/autopilot-datasets-problem-types.html#autopilot-problem-types"> SageMaker Autopilot problem types</a>.</p> |
| `auto_ml_job_config` | String |  | <p>A collection of settings used to configure an AutoML job.</p> |
| `output_data_config` | String | ✅ | <p>Provides information about encryption and the Amazon S3 output path needed to store artifacts from an AutoML job. Format(s) supported: CSV.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `last_modified_time` | String | <p>Returns the job's last modified time.</p> |
| `auto_ml_job_status` | String | <p>Returns the status of the AutoML job.</p> |
| `role_arn` | String | <p>The ARN of the IAM role that has read permission to the input data location and write permission to the output data location in Amazon S3.</p> |
| `end_time` | String | <p>Returns the end time of the AutoML job.</p> |
| `resolved_attributes` | String | <p>Contains <code>ProblemType</code>, <code>AutoMLJobObjective</code>, and <code>CompletionCriteria</code>. If you do not provide these values, they are inferred.</p> |
| `model_deploy_config` | String | <p>Indicates whether the model was deployed automatically to an endpoint and the name of that endpoint if deployed automatically.</p> |
| `auto_ml_job_arn` | String | <p>Returns the ARN of the AutoML job.</p> |
| `auto_ml_job_objective` | String | <p>Returns the job's objective.</p> |
| `failure_reason` | String | <p>Returns the failure reason for an AutoML job, when applicable.</p> |
| `best_candidate` | String | <p>The best model candidate selected by SageMaker AI Autopilot using both the best objective metric and lowest <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/autopilot-metrics-validation.html">InferenceLatency</a> for an experiment.</p> |
| `output_data_config` | String | <p>Returns the job's output data config.</p> |
| `model_deploy_result` | String | <p>Provides information about endpoint for the model deployment.</p> |
| `auto_ml_job_name` | String | <p>Returns the name of the AutoML job.</p> |
| `creation_time` | String | <p>Returns the creation time of the AutoML job.</p> |
| `auto_ml_job_artifacts` | String | <p>Returns information on the job's artifacts found in <code>AutoMLJobArtifacts</code>.</p> |
| `input_data_config` | Vec<String> | <p>Returns the input data configuration for the AutoML job.</p> |
| `problem_type` | String | <p>Returns the job's problem type.</p> |
| `partial_failure_reasons` | Vec<String> | <p>Returns a list of reasons for partial failures within an AutoML job.</p> |
| `auto_ml_job_secondary_status` | String | <p>Returns the secondary status of the AutoML job.</p> |
| `generate_candidate_definitions_only` | bool | <p>Indicates whether the output for an AutoML job generates candidate definitions only.</p> |
| `auto_ml_job_config` | String | <p>Returns the configuration for the AutoML job.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create auto_ml_job
auto_ml_job = provider.sagemaker.Auto_ml_job {
    input_data_config = "value"  # <p>An array of channel objects that describes the input data and its location. Each channel is a named input source. Similar to <code>InputDataConfig</code> supported by <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_HyperParameterTrainingJobDefinition.html">HyperParameterTrainingJobDefinition</a>. Format(s) supported: CSV, Parquet. A minimum of 500 rows is required for the training dataset. There is not a minimum number of rows required for the validation dataset.</p>
    role_arn = "value"  # <p>The ARN of the role that is used to access the data.</p>
    auto_ml_job_name = "value"  # <p>Identifies an Autopilot job. The name must be unique to your account and is case insensitive.</p>
    output_data_config = "value"  # <p>Provides information about encryption and the Amazon S3 output path needed to store artifacts from an AutoML job. Format(s) supported: CSV.</p>
}

# Access auto_ml_job outputs
auto_ml_job_id = auto_ml_job.id
auto_ml_job_last_modified_time = auto_ml_job.last_modified_time
auto_ml_job_auto_ml_job_status = auto_ml_job.auto_ml_job_status
auto_ml_job_role_arn = auto_ml_job.role_arn
auto_ml_job_end_time = auto_ml_job.end_time
auto_ml_job_resolved_attributes = auto_ml_job.resolved_attributes
auto_ml_job_model_deploy_config = auto_ml_job.model_deploy_config
auto_ml_job_auto_ml_job_arn = auto_ml_job.auto_ml_job_arn
auto_ml_job_auto_ml_job_objective = auto_ml_job.auto_ml_job_objective
auto_ml_job_failure_reason = auto_ml_job.failure_reason
auto_ml_job_best_candidate = auto_ml_job.best_candidate
auto_ml_job_output_data_config = auto_ml_job.output_data_config
auto_ml_job_model_deploy_result = auto_ml_job.model_deploy_result
auto_ml_job_auto_ml_job_name = auto_ml_job.auto_ml_job_name
auto_ml_job_creation_time = auto_ml_job.creation_time
auto_ml_job_auto_ml_job_artifacts = auto_ml_job.auto_ml_job_artifacts
auto_ml_job_input_data_config = auto_ml_job.input_data_config
auto_ml_job_problem_type = auto_ml_job.problem_type
auto_ml_job_partial_failure_reasons = auto_ml_job.partial_failure_reasons
auto_ml_job_auto_ml_job_secondary_status = auto_ml_job.auto_ml_job_secondary_status
auto_ml_job_generate_candidate_definitions_only = auto_ml_job.generate_candidate_definitions_only
auto_ml_job_auto_ml_job_config = auto_ml_job.auto_ml_job_config
```

---


### Studio_lifecycle_config

StudioLifecycleConfig resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `studio_lifecycle_config_name` | String | ✅ | <p>The name of the Amazon SageMaker AI Studio Lifecycle Configuration to create.</p> |
| `studio_lifecycle_config_content` | String | ✅ | <p>The content of your Amazon SageMaker AI Studio Lifecycle Configuration script. This content must be base64 encoded.</p> |
| `tags` | Vec<String> |  | <p>Tags to be associated with the Lifecycle Configuration. Each tag consists of a key and an optional value. Tag keys must be unique per resource. Tags are searchable using the Search API. </p> |
| `studio_lifecycle_config_app_type` | String | ✅ | <p>The App type that the Lifecycle Configuration is attached to.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `last_modified_time` | String | <p>This value is equivalent to CreationTime because Amazon SageMaker AI Studio Lifecycle Configurations are immutable.</p> |
| `creation_time` | String | <p>The creation time of the Amazon SageMaker AI Studio Lifecycle Configuration.</p> |
| `studio_lifecycle_config_content` | String | <p>The content of your Amazon SageMaker AI Studio Lifecycle Configuration script.</p> |
| `studio_lifecycle_config_app_type` | String | <p>The App type that the Lifecycle Configuration is attached to.</p> |
| `studio_lifecycle_config_name` | String | <p>The name of the Amazon SageMaker AI Studio Lifecycle Configuration that is described.</p> |
| `studio_lifecycle_config_arn` | String | <p>The ARN of the Lifecycle Configuration to describe.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create studio_lifecycle_config
studio_lifecycle_config = provider.sagemaker.Studio_lifecycle_config {
    studio_lifecycle_config_name = "value"  # <p>The name of the Amazon SageMaker AI Studio Lifecycle Configuration to create.</p>
    studio_lifecycle_config_content = "value"  # <p>The content of your Amazon SageMaker AI Studio Lifecycle Configuration script. This content must be base64 encoded.</p>
    studio_lifecycle_config_app_type = "value"  # <p>The App type that the Lifecycle Configuration is attached to.</p>
}

# Access studio_lifecycle_config outputs
studio_lifecycle_config_id = studio_lifecycle_config.id
studio_lifecycle_config_last_modified_time = studio_lifecycle_config.last_modified_time
studio_lifecycle_config_creation_time = studio_lifecycle_config.creation_time
studio_lifecycle_config_studio_lifecycle_config_content = studio_lifecycle_config.studio_lifecycle_config_content
studio_lifecycle_config_studio_lifecycle_config_app_type = studio_lifecycle_config.studio_lifecycle_config_app_type
studio_lifecycle_config_studio_lifecycle_config_name = studio_lifecycle_config.studio_lifecycle_config_name
studio_lifecycle_config_studio_lifecycle_config_arn = studio_lifecycle_config.studio_lifecycle_config_arn
```

---


### Training_plan

TrainingPlan resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `training_plan_offering_id` | String | ✅ | <p>The unique identifier of the training plan offering to use for creating this plan.</p> |
| `spare_instance_count_per_ultra_server` | i64 |  | <p>Number of spare instances to reserve per UltraServer for enhanced resiliency. Default is 1.</p> |
| `tags` | Vec<String> |  | <p>An array of key-value pairs to apply to this training plan.</p> |
| `training_plan_name` | String | ✅ | <p>The name of the training plan to create.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `duration_minutes` | i64 | <p>The additional minutes beyond whole hours in the total duration for this training plan.</p> |
| `available_instance_count` | i64 | <p>The number of instances currently available for use in this training plan.</p> |
| `total_instance_count` | i64 | <p>The total number of instances reserved in this training plan.</p> |
| `start_time` | String | <p>The start time of the training plan.</p> |
| `target_resources` | Vec<String> | <p>The target resources (e.g., SageMaker Training Jobs, SageMaker HyperPod) that can use this training plan.</p> <p>Training plans are specific to their target resource.</p> <ul> <li> <p>A training plan designed for SageMaker training jobs can only be used to schedule and run training jobs.</p> </li> <li> <p>A training plan for HyperPod clusters can be used exclusively to provide compute resources to a cluster's instance group.</p> </li> </ul> |
| `status_message` | String | <p>A message providing additional information about the current status of the training plan.</p> |
| `in_use_instance_count` | i64 | <p>The number of instances currently in use from this training plan.</p> |
| `duration_hours` | i64 | <p>The number of whole hours in the total duration for this training plan.</p> |
| `training_plan_name` | String | <p>The name of the training plan.</p> |
| `upfront_fee` | String | <p>The upfront fee for the training plan.</p> |
| `unhealthy_instance_count` | i64 | <p>The number of instances in the training plan that are currently in an unhealthy state.</p> |
| `available_spare_instance_count` | i64 | <p>The number of available spare instances in the training plan.</p> |
| `end_time` | String | <p>The end time of the training plan.</p> |
| `training_plan_arn` | String | <p>The Amazon Resource Name (ARN); of the training plan.</p> |
| `total_ultra_server_count` | i64 | <p>The total number of UltraServers reserved to this training plan.</p> |
| `status` | String | <p>The current status of the training plan (e.g., Pending, Active, Expired). To see the complete list of status values available for a training plan, refer to the <code>Status</code> attribute within the <code> <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_TrainingPlanSummary.html">TrainingPlanSummary</a> </code> object.</p> |
| `currency_code` | String | <p>The currency code for the upfront fee (e.g., USD).</p> |
| `reserved_capacity_summaries` | Vec<String> | <p>The list of Reserved Capacity providing the underlying compute resources of the plan. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create training_plan
training_plan = provider.sagemaker.Training_plan {
    training_plan_offering_id = "value"  # <p>The unique identifier of the training plan offering to use for creating this plan.</p>
    training_plan_name = "value"  # <p>The name of the training plan to create.</p>
}

# Access training_plan outputs
training_plan_id = training_plan.id
training_plan_duration_minutes = training_plan.duration_minutes
training_plan_available_instance_count = training_plan.available_instance_count
training_plan_total_instance_count = training_plan.total_instance_count
training_plan_start_time = training_plan.start_time
training_plan_target_resources = training_plan.target_resources
training_plan_status_message = training_plan.status_message
training_plan_in_use_instance_count = training_plan.in_use_instance_count
training_plan_duration_hours = training_plan.duration_hours
training_plan_training_plan_name = training_plan.training_plan_name
training_plan_upfront_fee = training_plan.upfront_fee
training_plan_unhealthy_instance_count = training_plan.unhealthy_instance_count
training_plan_available_spare_instance_count = training_plan.available_spare_instance_count
training_plan_end_time = training_plan.end_time
training_plan_training_plan_arn = training_plan.training_plan_arn
training_plan_total_ultra_server_count = training_plan.total_ultra_server_count
training_plan_status = training_plan.status
training_plan_currency_code = training_plan.currency_code
training_plan_reserved_capacity_summaries = training_plan.reserved_capacity_summaries
```

---


### Optimization_job

OptimizationJob resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `vpc_config` | String |  | <p>A VPC in Amazon VPC that your optimized model has access to.</p> |
| `output_config` | String | ✅ | <p>Details for where to store the optimized model that you create with the optimization job.</p> |
| `deployment_instance_type` | String | ✅ | <p>The type of instance that hosts the optimized model that you create with the optimization job.</p> |
| `tags` | Vec<String> |  | <p>A list of key-value pairs associated with the optimization job. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services resources</a> in the <i>Amazon Web Services General Reference Guide</i>.</p> |
| `model_source` | String | ✅ | <p>The location of the source model to optimize with an optimization job.</p> |
| `optimization_configs` | Vec<String> | ✅ | <p>Settings for each of the optimization techniques that the job applies.</p> |
| `role_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of an IAM role that enables Amazon SageMaker AI to perform tasks on your behalf. </p> <p>During model optimization, Amazon SageMaker AI needs your permission to:</p> <ul> <li> <p>Read input data from an S3 bucket</p> </li> <li> <p>Write model artifacts to an S3 bucket</p> </li> <li> <p>Write logs to Amazon CloudWatch Logs</p> </li> <li> <p>Publish metrics to Amazon CloudWatch</p> </li> </ul> <p>You grant permissions for all of these tasks to an IAM role. To pass this role to Amazon SageMaker AI, the caller of this API must have the <code>iam:PassRole</code> permission. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/sagemaker-roles.html">Amazon SageMaker AI Roles.</a> </p> |
| `optimization_environment` | HashMap<String, String> |  | <p>The environment variables to set in the model container.</p> |
| `optimization_job_name` | String | ✅ | <p>A custom name for the new optimization job.</p> |
| `stopping_condition` | String | ✅ |  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `model_source` | String | <p>The location of the source model to optimize with an optimization job.</p> |
| `deployment_instance_type` | String | <p>The type of instance that hosts the optimized model that you create with the optimization job.</p> |
| `optimization_job_arn` | String | <p>The Amazon Resource Name (ARN) of the optimization job.</p> |
| `optimization_start_time` | String | <p>The time when the optimization job started.</p> |
| `last_modified_time` | String | <p>The time when the optimization job was last updated.</p> |
| `role_arn` | String | <p>The ARN of the IAM role that you assigned to the optimization job.</p> |
| `vpc_config` | String | <p>A VPC in Amazon VPC that your optimized model has access to.</p> |
| `failure_reason` | String | <p>If the optimization job status is <code>FAILED</code>, the reason for the failure.</p> |
| `stopping_condition` | String |  |
| `optimization_end_time` | String | <p>The time when the optimization job finished processing.</p> |
| `optimization_job_status` | String | <p>The current status of the optimization job.</p> |
| `optimization_environment` | HashMap<String, String> | <p>The environment variables to set in the model container.</p> |
| `optimization_output` | String | <p>Output values produced by an optimization job.</p> |
| `output_config` | String | <p>Details for where to store the optimized model that you create with the optimization job.</p> |
| `optimization_job_name` | String | <p>The name that you assigned to the optimization job.</p> |
| `optimization_configs` | Vec<String> | <p>Settings for each of the optimization techniques that the job applies.</p> |
| `creation_time` | String | <p>The time when you created the optimization job.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create optimization_job
optimization_job = provider.sagemaker.Optimization_job {
    output_config = "value"  # <p>Details for where to store the optimized model that you create with the optimization job.</p>
    deployment_instance_type = "value"  # <p>The type of instance that hosts the optimized model that you create with the optimization job.</p>
    model_source = "value"  # <p>The location of the source model to optimize with an optimization job.</p>
    optimization_configs = "value"  # <p>Settings for each of the optimization techniques that the job applies.</p>
    role_arn = "value"  # <p>The Amazon Resource Name (ARN) of an IAM role that enables Amazon SageMaker AI to perform tasks on your behalf. </p> <p>During model optimization, Amazon SageMaker AI needs your permission to:</p> <ul> <li> <p>Read input data from an S3 bucket</p> </li> <li> <p>Write model artifacts to an S3 bucket</p> </li> <li> <p>Write logs to Amazon CloudWatch Logs</p> </li> <li> <p>Publish metrics to Amazon CloudWatch</p> </li> </ul> <p>You grant permissions for all of these tasks to an IAM role. To pass this role to Amazon SageMaker AI, the caller of this API must have the <code>iam:PassRole</code> permission. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/sagemaker-roles.html">Amazon SageMaker AI Roles.</a> </p>
    optimization_job_name = "value"  # <p>A custom name for the new optimization job.</p>
    stopping_condition = "value"  # Required field
}

# Access optimization_job outputs
optimization_job_id = optimization_job.id
optimization_job_model_source = optimization_job.model_source
optimization_job_deployment_instance_type = optimization_job.deployment_instance_type
optimization_job_optimization_job_arn = optimization_job.optimization_job_arn
optimization_job_optimization_start_time = optimization_job.optimization_start_time
optimization_job_last_modified_time = optimization_job.last_modified_time
optimization_job_role_arn = optimization_job.role_arn
optimization_job_vpc_config = optimization_job.vpc_config
optimization_job_failure_reason = optimization_job.failure_reason
optimization_job_stopping_condition = optimization_job.stopping_condition
optimization_job_optimization_end_time = optimization_job.optimization_end_time
optimization_job_optimization_job_status = optimization_job.optimization_job_status
optimization_job_optimization_environment = optimization_job.optimization_environment
optimization_job_optimization_output = optimization_job.optimization_output
optimization_job_output_config = optimization_job.output_config
optimization_job_optimization_job_name = optimization_job.optimization_job_name
optimization_job_optimization_configs = optimization_job.optimization_configs
optimization_job_creation_time = optimization_job.creation_time
```

---


### Model_explainability_job_definition

ModelExplainabilityJobDefinition resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `job_definition_name` | String | ✅ | <p> The name of the model explainability job definition. The name must be unique within an Amazon Web Services Region in the Amazon Web Services account.</p> |
| `network_config` | String |  | <p>Networking options for a model explainability job.</p> |
| `model_explainability_job_input` | String | ✅ | <p>Inputs for the model explainability job.</p> |
| `model_explainability_baseline_config` | String |  | <p>The baseline configuration for a model explainability job.</p> |
| `tags` | Vec<String> |  | <p>(Optional) An array of key-value pairs. For more information, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html#allocation-whatURL"> Using Cost Allocation Tags</a> in the <i>Amazon Web Services Billing and Cost Management User Guide</i>.</p> |
| `model_explainability_app_specification` | String | ✅ | <p>Configures the model explainability job to run a specified Docker container image.</p> |
| `stopping_condition` | String |  |  |
| `job_resources` | String | ✅ |  |
| `role_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of an IAM role that Amazon SageMaker AI can assume to perform tasks on your behalf.</p> |
| `model_explainability_job_output_config` | String | ✅ |  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `model_explainability_job_input` | String | <p>Inputs for the model explainability job.</p> |
| `stopping_condition` | String |  |
| `model_explainability_baseline_config` | String | <p>The baseline configuration for a model explainability job.</p> |
| `job_definition_name` | String | <p>The name of the explainability job definition. The name must be unique within an Amazon Web Services Region in the Amazon Web Services account.</p> |
| `job_resources` | String |  |
| `role_arn` | String | <p>The Amazon Resource Name (ARN) of the IAM role that has read permission to the input data location and write permission to the output data location in Amazon S3.</p> |
| `network_config` | String | <p>Networking options for a model explainability job.</p> |
| `model_explainability_app_specification` | String | <p>Configures the model explainability job to run a specified Docker container image.</p> |
| `model_explainability_job_output_config` | String |  |
| `job_definition_arn` | String | <p>The Amazon Resource Name (ARN) of the model explainability job.</p> |
| `creation_time` | String | <p>The time at which the model explainability job was created.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create model_explainability_job_definition
model_explainability_job_definition = provider.sagemaker.Model_explainability_job_definition {
    job_definition_name = "value"  # <p> The name of the model explainability job definition. The name must be unique within an Amazon Web Services Region in the Amazon Web Services account.</p>
    model_explainability_job_input = "value"  # <p>Inputs for the model explainability job.</p>
    model_explainability_app_specification = "value"  # <p>Configures the model explainability job to run a specified Docker container image.</p>
    job_resources = "value"  # Required field
    role_arn = "value"  # <p>The Amazon Resource Name (ARN) of an IAM role that Amazon SageMaker AI can assume to perform tasks on your behalf.</p>
    model_explainability_job_output_config = "value"  # Required field
}

# Access model_explainability_job_definition outputs
model_explainability_job_definition_id = model_explainability_job_definition.id
model_explainability_job_definition_model_explainability_job_input = model_explainability_job_definition.model_explainability_job_input
model_explainability_job_definition_stopping_condition = model_explainability_job_definition.stopping_condition
model_explainability_job_definition_model_explainability_baseline_config = model_explainability_job_definition.model_explainability_baseline_config
model_explainability_job_definition_job_definition_name = model_explainability_job_definition.job_definition_name
model_explainability_job_definition_job_resources = model_explainability_job_definition.job_resources
model_explainability_job_definition_role_arn = model_explainability_job_definition.role_arn
model_explainability_job_definition_network_config = model_explainability_job_definition.network_config
model_explainability_job_definition_model_explainability_app_specification = model_explainability_job_definition.model_explainability_app_specification
model_explainability_job_definition_model_explainability_job_output_config = model_explainability_job_definition.model_explainability_job_output_config
model_explainability_job_definition_job_definition_arn = model_explainability_job_definition.job_definition_arn
model_explainability_job_definition_creation_time = model_explainability_job_definition.creation_time
```

---


### Cluster

Cluster resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tiered_storage_config` | String |  | <p>The configuration for managed tier checkpointing on the HyperPod cluster. When enabled, this feature uses a multi-tier storage approach for storing model checkpoints, providing faster checkpoint operations and improved fault tolerance across cluster nodes.</p> |
| `auto_scaling` | String |  | <p>The autoscaling configuration for the cluster. Enables automatic scaling of cluster nodes based on workload demand using a Karpenter-based system.</p> |
| `node_recovery` | String |  | <p>The node recovery mode for the SageMaker HyperPod cluster. When set to <code>Automatic</code>, SageMaker HyperPod will automatically reboot or replace faulty nodes when issues are detected. When set to <code>None</code>, cluster administrators will need to manually manage any faulty cluster instances.</p> |
| `cluster_role` | String |  | <p>The Amazon Resource Name (ARN) of the IAM role that HyperPod assumes to perform cluster autoscaling operations. This role must have permissions for <code>sagemaker:BatchAddClusterNodes</code> and <code>sagemaker:BatchDeleteClusterNodes</code>. This is only required when autoscaling is enabled and when HyperPod is performing autoscaling operations.</p> |
| `instance_groups` | Vec<String> |  | <p>The instance groups to be created in the SageMaker HyperPod cluster.</p> |
| `node_provisioning_mode` | String |  | <p>The mode for provisioning nodes in the cluster. You can specify the following modes:</p> <ul> <li> <p> <b>Continuous</b>: Scaling behavior that enables 1) concurrent operation execution within instance groups, 2) continuous retry mechanisms for failed operations, 3) enhanced customer visibility into cluster events through detailed event streams, 4) partial provisioning capabilities. Your clusters and instance groups remain <code>InService</code> while scaling. This mode is only supported for EKS orchestrated clusters.</p> </li> </ul> |
| `restricted_instance_groups` | Vec<String> |  | <p>The specialized instance groups for training models like Amazon Nova to be created in the SageMaker HyperPod cluster.</p> |
| `tags` | Vec<String> |  | <p>Custom tags for managing the SageMaker HyperPod cluster as an Amazon Web Services resource. You can add tags to your cluster in the same way you add them in other Amazon Web Services services that support tagging. To learn more about tagging Amazon Web Services resources in general, see <a href="https://docs.aws.amazon.com/tag-editor/latest/userguide/tagging.html">Tagging Amazon Web Services Resources User Guide</a>.</p> |
| `orchestrator` | String |  | <p>The type of orchestrator to use for the SageMaker HyperPod cluster. Currently, the only supported value is <code>"eks"</code>, which is to use an Amazon Elastic Kubernetes Service cluster as the orchestrator.</p> |
| `cluster_name` | String | ✅ | <p>The name for the new SageMaker HyperPod cluster.</p> |
| `vpc_config` | String |  | <p>Specifies the Amazon Virtual Private Cloud (VPC) that is associated with the Amazon SageMaker HyperPod cluster. You can control access to and from your resources by configuring your VPC. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/infrastructure-give-access.html">Give SageMaker access to resources in your Amazon VPC</a>.</p> <note> <p>When your Amazon VPC and subnets support IPv6, network communications differ based on the cluster orchestration platform:</p> <ul> <li> <p>Slurm-orchestrated clusters automatically configure nodes with dual IPv6 and IPv4 addresses, allowing immediate IPv6 network communications.</p> </li> <li> <p>In Amazon EKS-orchestrated clusters, nodes receive dual-stack addressing, but pods can only use IPv6 when the Amazon EKS cluster is explicitly IPv6-enabled. For information about deploying an IPv6 Amazon EKS cluster, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/deploy-ipv6-cluster.html#_deploy_an_ipv6_cluster_with_eksctl">Amazon EKS IPv6 Cluster Deployment</a>.</p> </li> </ul> <p>Additional resources for IPv6 configuration:</p> <ul> <li> <p>For information about adding IPv6 support to your VPC, see to <a href="https://docs.aws.amazon.com/vpc/latest/userguide/vpc-migrate-ipv6.html">IPv6 Support for VPC</a>.</p> </li> <li> <p>For information about creating a new IPv6-compatible VPC, see <a href="https://docs.aws.amazon.com/vpc/latest/userguide/create-vpc.html">Amazon VPC Creation Guide</a>.</p> </li> <li> <p>To configure SageMaker HyperPod with a custom Amazon VPC, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/sagemaker-hyperpod-prerequisites.html#sagemaker-hyperpod-prerequisites-optional-vpc">Custom Amazon VPC Setup for SageMaker HyperPod</a>.</p> </li> </ul> </note> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `creation_time` | String | <p>The time when the SageMaker Cluster is created.</p> |
| `restricted_instance_groups` | Vec<String> | <p>The specialized instance groups for training models like Amazon Nova to be created in the SageMaker HyperPod cluster.</p> |
| `orchestrator` | String | <p>The type of orchestrator used for the SageMaker HyperPod cluster. </p> |
| `auto_scaling` | String | <p>The current autoscaling configuration and status for the autoscaler.</p> |
| `failure_message` | String | <p>The failure message of the SageMaker HyperPod cluster.</p> |
| `tiered_storage_config` | String | <p>The current configuration for managed tier checkpointing on the HyperPod cluster. For example, this shows whether the feature is enabled and the percentage of cluster memory allocated for checkpoint storage.</p> |
| `cluster_name` | String | <p>The name of the SageMaker HyperPod cluster.</p> |
| `node_recovery` | String | <p>The node recovery mode configured for the SageMaker HyperPod cluster.</p> |
| `cluster_arn` | String | <p>The Amazon Resource Name (ARN) of the SageMaker HyperPod cluster.</p> |
| `cluster_role` | String | <p>The Amazon Resource Name (ARN) of the IAM role that HyperPod uses for cluster autoscaling operations.</p> |
| `node_provisioning_mode` | String | <p>The mode used for provisioning nodes in the cluster.</p> |
| `cluster_status` | String | <p>The status of the SageMaker HyperPod cluster.</p> |
| `instance_groups` | Vec<String> | <p>The instance groups of the SageMaker HyperPod cluster.</p> |
| `vpc_config` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create cluster
cluster = provider.sagemaker.Cluster {
    cluster_name = "value"  # <p>The name for the new SageMaker HyperPod cluster.</p>
}

# Access cluster outputs
cluster_id = cluster.id
cluster_creation_time = cluster.creation_time
cluster_restricted_instance_groups = cluster.restricted_instance_groups
cluster_orchestrator = cluster.orchestrator
cluster_auto_scaling = cluster.auto_scaling
cluster_failure_message = cluster.failure_message
cluster_tiered_storage_config = cluster.tiered_storage_config
cluster_cluster_name = cluster.cluster_name
cluster_node_recovery = cluster.node_recovery
cluster_cluster_arn = cluster.cluster_arn
cluster_cluster_role = cluster.cluster_role
cluster_node_provisioning_mode = cluster.node_provisioning_mode
cluster_cluster_status = cluster.cluster_status
cluster_instance_groups = cluster.instance_groups
cluster_vpc_config = cluster.vpc_config
```

---


### Context

Context resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `context_name` | String | ✅ | <p>The name of the context. Must be unique to your account in an Amazon Web Services Region.</p> |
| `properties` | HashMap<String, String> |  | <p>A list of properties to add to the context.</p> |
| `tags` | Vec<String> |  | <p>A list of tags to apply to the context.</p> |
| `context_type` | String | ✅ | <p>The context type.</p> |
| `source` | String | ✅ | <p>The source type, ID, and URI.</p> |
| `description` | String |  | <p>The description of the context.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `context_type` | String | <p>The type of the context.</p> |
| `creation_time` | String | <p>When the context was created.</p> |
| `last_modified_time` | String | <p>When the context was last modified.</p> |
| `context_name` | String | <p>The name of the context.</p> |
| `context_arn` | String | <p>The Amazon Resource Name (ARN) of the context.</p> |
| `source` | String | <p>The source of the context.</p> |
| `description` | String | <p>The description of the context.</p> |
| `properties` | HashMap<String, String> | <p>A list of the context's properties.</p> |
| `created_by` | String |  |
| `last_modified_by` | String |  |
| `lineage_group_arn` | String | <p>The Amazon Resource Name (ARN) of the lineage group.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create context
context = provider.sagemaker.Context {
    context_name = "value"  # <p>The name of the context. Must be unique to your account in an Amazon Web Services Region.</p>
    context_type = "value"  # <p>The context type.</p>
    source = "value"  # <p>The source type, ID, and URI.</p>
}

# Access context outputs
context_id = context.id
context_context_type = context.context_type
context_creation_time = context.creation_time
context_last_modified_time = context.last_modified_time
context_context_name = context.context_name
context_context_arn = context.context_arn
context_source = context.source
context_description = context.description
context_properties = context.properties
context_created_by = context.created_by
context_last_modified_by = context.last_modified_by
context_lineage_group_arn = context.lineage_group_arn
```

---


### Cluster_software

ClusterSoftware resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `instance_groups` | Vec<String> |  | <p>The array of instance groups for which to update AMI versions.</p> |
| `image_id` | String |  | <p>When configuring your HyperPod cluster, you can specify an image ID using one of the following options:</p> <ul> <li> <p> <code>HyperPodPublicAmiId</code>: Use a HyperPod public AMI</p> </li> <li> <p> <code>CustomAmiId</code>: Use your custom AMI</p> </li> <li> <p> <code>default</code>: Use the default latest system image</p> </li> </ul> <p>If you choose to use a custom AMI (<code>CustomAmiId</code>), ensure it meets the following requirements:</p> <ul> <li> <p>Encryption: The custom AMI must be unencrypted.</p> </li> <li> <p>Ownership: The custom AMI must be owned by the same Amazon Web Services account that is creating the HyperPod cluster.</p> </li> <li> <p>Volume support: Only the primary AMI snapshot volume is supported; additional AMI volumes are not supported.</p> </li> </ul> <p>When updating the instance group's AMI through the <code>UpdateClusterSoftware</code> operation, if an instance group uses a custom AMI, you must provide an <code>ImageId</code> or use the default as input. Note that if you don't specify an instance group in your <code>UpdateClusterSoftware</code> request, then all of the instance groups are patched with the specified image.</p> |
| `deployment_config` | String |  | <p>The configuration to use when updating the AMI versions.</p> |
| `cluster_name` | String | ✅ | <p>Specify the name or the Amazon Resource Name (ARN) of the SageMaker HyperPod cluster you want to update for security patching.</p> |



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


### Algorithm

Algorithm resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `algorithm_name` | String | ✅ | <p>The name of the algorithm.</p> |
| `training_specification` | String | ✅ | <p>Specifies details about training jobs run by this algorithm, including the following:</p> <ul> <li> <p>The Amazon ECR path of the container and the version digest of the algorithm.</p> </li> <li> <p>The hyperparameters that the algorithm supports.</p> </li> <li> <p>The instance types that the algorithm supports for training.</p> </li> <li> <p>Whether the algorithm supports distributed training.</p> </li> <li> <p>The metrics that the algorithm emits to Amazon CloudWatch.</p> </li> <li> <p>Which metrics that the algorithm emits can be used as the objective metric for hyperparameter tuning jobs.</p> </li> <li> <p>The input channels that the algorithm supports for training data. For example, an algorithm might support <code>train</code>, <code>validation</code>, and <code>test</code> channels.</p> </li> </ul> |
| `inference_specification` | String |  | <p>Specifies details about inference jobs that the algorithm runs, including the following:</p> <ul> <li> <p>The Amazon ECR paths of containers that contain the inference code and model artifacts.</p> </li> <li> <p>The instance types that the algorithm supports for transform jobs and real-time endpoints used for inference.</p> </li> <li> <p>The input and output content formats that the algorithm supports for inference.</p> </li> </ul> |
| `algorithm_description` | String |  | <p>A description of the algorithm.</p> |
| `certify_for_marketplace` | bool |  | <p>Whether to certify the algorithm so that it can be listed in Amazon Web Services Marketplace.</p> |
| `validation_specification` | String |  | <p>Specifies configurations for one or more training jobs and that SageMaker runs to test the algorithm's training code and, optionally, one or more batch transform jobs that SageMaker runs to test the algorithm's inference code.</p> |
| `tags` | Vec<String> |  | <p>An array of key-value pairs. You can use tags to categorize your Amazon Web Services resources in different ways, for example, by purpose, owner, or environment. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services Resources</a>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `certify_for_marketplace` | bool | <p>Whether the algorithm is certified to be listed in Amazon Web Services Marketplace.</p> |
| `algorithm_arn` | String | <p>The Amazon Resource Name (ARN) of the algorithm.</p> |
| `algorithm_name` | String | <p>The name of the algorithm being described.</p> |
| `creation_time` | String | <p>A timestamp specifying when the algorithm was created.</p> |
| `algorithm_status` | String | <p>The current status of the algorithm.</p> |
| `inference_specification` | String | <p>Details about inference jobs that the algorithm runs.</p> |
| `training_specification` | String | <p>Details about training jobs run by this algorithm.</p> |
| `algorithm_status_details` | String | <p>Details about the current status of the algorithm.</p> |
| `algorithm_description` | String | <p>A brief summary about the algorithm.</p> |
| `validation_specification` | String | <p>Details about configurations for one or more training jobs that SageMaker runs to test the algorithm.</p> |
| `product_id` | String | <p>The product identifier of the algorithm.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create algorithm
algorithm = provider.sagemaker.Algorithm {
    algorithm_name = "value"  # <p>The name of the algorithm.</p>
    training_specification = "value"  # <p>Specifies details about training jobs run by this algorithm, including the following:</p> <ul> <li> <p>The Amazon ECR path of the container and the version digest of the algorithm.</p> </li> <li> <p>The hyperparameters that the algorithm supports.</p> </li> <li> <p>The instance types that the algorithm supports for training.</p> </li> <li> <p>Whether the algorithm supports distributed training.</p> </li> <li> <p>The metrics that the algorithm emits to Amazon CloudWatch.</p> </li> <li> <p>Which metrics that the algorithm emits can be used as the objective metric for hyperparameter tuning jobs.</p> </li> <li> <p>The input channels that the algorithm supports for training data. For example, an algorithm might support <code>train</code>, <code>validation</code>, and <code>test</code> channels.</p> </li> </ul>
}

# Access algorithm outputs
algorithm_id = algorithm.id
algorithm_certify_for_marketplace = algorithm.certify_for_marketplace
algorithm_algorithm_arn = algorithm.algorithm_arn
algorithm_algorithm_name = algorithm.algorithm_name
algorithm_creation_time = algorithm.creation_time
algorithm_algorithm_status = algorithm.algorithm_status
algorithm_inference_specification = algorithm.inference_specification
algorithm_training_specification = algorithm.training_specification
algorithm_algorithm_status_details = algorithm.algorithm_status_details
algorithm_algorithm_description = algorithm.algorithm_description
algorithm_validation_specification = algorithm.validation_specification
algorithm_product_id = algorithm.product_id
```

---


### Inference_component_runtime_config

InferenceComponentRuntimeConfig resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `inference_component_name` | String | ✅ | <p>The name of the inference component to update.</p> |
| `desired_runtime_config` | String | ✅ | <p>Runtime settings for a model that is deployed with an inference component.</p> |



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


### Experiment

Experiment resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | <p>The name of the experiment as displayed. The name doesn't need to be unique. If you don't specify <code>DisplayName</code>, the value in <code>ExperimentName</code> is displayed.</p> |
| `tags` | Vec<String> |  | <p>A list of tags to associate with the experiment. You can use <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_Search.html">Search</a> API to search on the tags.</p> |
| `experiment_name` | String | ✅ | <p>The name of the experiment. The name must be unique in your Amazon Web Services account and is not case-sensitive.</p> |
| `description` | String |  | <p>The description of the experiment.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `last_modified_time` | String | <p>When the experiment was last modified.</p> |
| `experiment_arn` | String | <p>The Amazon Resource Name (ARN) of the experiment.</p> |
| `last_modified_by` | String | <p>Who last modified the experiment.</p> |
| `source` | String | <p>The Amazon Resource Name (ARN) of the source and, optionally, the type.</p> |
| `created_by` | String | <p>Who created the experiment.</p> |
| `description` | String | <p>The description of the experiment.</p> |
| `display_name` | String | <p>The name of the experiment as displayed. If <code>DisplayName</code> isn't specified, <code>ExperimentName</code> is displayed.</p> |
| `experiment_name` | String | <p>The name of the experiment.</p> |
| `creation_time` | String | <p>When the experiment was created.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create experiment
experiment = provider.sagemaker.Experiment {
    experiment_name = "value"  # <p>The name of the experiment. The name must be unique in your Amazon Web Services account and is not case-sensitive.</p>
}

# Access experiment outputs
experiment_id = experiment.id
experiment_last_modified_time = experiment.last_modified_time
experiment_experiment_arn = experiment.experiment_arn
experiment_last_modified_by = experiment.last_modified_by
experiment_source = experiment.source
experiment_created_by = experiment.created_by
experiment_description = experiment.description
experiment_display_name = experiment.display_name
experiment_experiment_name = experiment.experiment_name
experiment_creation_time = experiment.creation_time
```

---


### App_image_config

AppImageConfig resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `app_image_config_name` | String | ✅ | <p>The name of the AppImageConfig. Must be unique to your account.</p> |
| `code_editor_app_image_config` | String |  | <p>The <code>CodeEditorAppImageConfig</code>. You can only specify one image kernel in the AppImageConfig API. This kernel is shown to users before the image starts. After the image runs, all kernels are visible in Code Editor.</p> |
| `jupyter_lab_app_image_config` | String |  | <p>The <code>JupyterLabAppImageConfig</code>. You can only specify one image kernel in the <code>AppImageConfig</code> API. This kernel is shown to users before the image starts. After the image runs, all kernels are visible in JupyterLab.</p> |
| `tags` | Vec<String> |  | <p>A list of tags to apply to the AppImageConfig.</p> |
| `kernel_gateway_image_config` | String |  | <p>The KernelGatewayImageConfig. You can only specify one image kernel in the AppImageConfig API. This kernel will be shown to users before the image starts. Once the image runs, all kernels are visible in JupyterLab.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `creation_time` | String | <p>When the AppImageConfig was created.</p> |
| `last_modified_time` | String | <p>When the AppImageConfig was last modified.</p> |
| `code_editor_app_image_config` | String | <p>The configuration of the Code Editor app.</p> |
| `kernel_gateway_image_config` | String | <p>The configuration of a KernelGateway app.</p> |
| `app_image_config_arn` | String | <p>The ARN of the AppImageConfig.</p> |
| `app_image_config_name` | String | <p>The name of the AppImageConfig.</p> |
| `jupyter_lab_app_image_config` | String | <p>The configuration of the JupyterLab app.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create app_image_config
app_image_config = provider.sagemaker.App_image_config {
    app_image_config_name = "value"  # <p>The name of the AppImageConfig. Must be unique to your account.</p>
}

# Access app_image_config outputs
app_image_config_id = app_image_config.id
app_image_config_creation_time = app_image_config.creation_time
app_image_config_last_modified_time = app_image_config.last_modified_time
app_image_config_code_editor_app_image_config = app_image_config.code_editor_app_image_config
app_image_config_kernel_gateway_image_config = app_image_config.kernel_gateway_image_config
app_image_config_app_image_config_arn = app_image_config.app_image_config_arn
app_image_config_app_image_config_name = app_image_config.app_image_config_name
app_image_config_jupyter_lab_app_image_config = app_image_config.jupyter_lab_app_image_config
```

---


### Domain

Domain resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>Tags to associated with the Domain. Each tag consists of a key and an optional value. Tag keys must be unique per resource. Tags are searchable using the <code>Search</code> API.</p> <p>Tags that you specify for the Domain are also added to all Apps that the Domain launches.</p> |
| `home_efs_file_system_kms_key_id` | String |  | <p>Use <code>KmsKeyId</code>.</p> |
| `kms_key_id` | String |  | <p>SageMaker AI uses Amazon Web Services KMS to encrypt EFS and EBS volumes attached to the domain with an Amazon Web Services managed key by default. For more control, specify a customer managed key.</p> |
| `app_network_access_type` | String |  | <p>Specifies the VPC used for non-EFS traffic. The default value is <code>PublicInternetOnly</code>.</p> <ul> <li> <p> <code>PublicInternetOnly</code> - Non-EFS traffic is through a VPC managed by Amazon SageMaker AI, which allows direct internet access</p> </li> <li> <p> <code>VpcOnly</code> - All traffic is through the specified VPC and subnets</p> </li> </ul> |
| `subnet_ids` | Vec<String> |  | <p>The VPC subnets that the domain uses for communication.</p> <p>The field is optional when the <code>AppNetworkAccessType</code> parameter is set to <code>PublicInternetOnly</code> for domains created from Amazon SageMaker Unified Studio.</p> |
| `auth_mode` | String | ✅ | <p>The mode of authentication that members use to access the domain.</p> |
| `domain_settings` | String |  | <p>A collection of <code>Domain</code> settings.</p> |
| `app_security_group_management` | String |  | <p>The entity that creates and manages the required security groups for inter-app communication in <code>VPCOnly</code> mode. Required when <code>CreateDomain.AppNetworkAccessType</code> is <code>VPCOnly</code> and <code>DomainSettings.RStudioServerProDomainSettings.DomainExecutionRoleArn</code> is provided. If setting up the domain for use with RStudio, this value must be set to <code>Service</code>.</p> |
| `default_user_settings` | String | ✅ | <p>The default settings to use to create a user profile when <code>UserSettings</code> isn't specified in the call to the <code>CreateUserProfile</code> API.</p> <p> <code>SecurityGroups</code> is aggregated when specified in both calls. For all other settings in <code>UserSettings</code>, the values specified in <code>CreateUserProfile</code> take precedence over those specified in <code>CreateDomain</code>.</p> |
| `domain_name` | String | ✅ | <p>A name for the domain.</p> |
| `vpc_id` | String |  | <p>The ID of the Amazon Virtual Private Cloud (VPC) that the domain uses for communication.</p> <p>The field is optional when the <code>AppNetworkAccessType</code> parameter is set to <code>PublicInternetOnly</code> for domains created from Amazon SageMaker Unified Studio.</p> |
| `tag_propagation` | String |  | <p>Indicates whether custom tag propagation is supported for the domain. Defaults to <code>DISABLED</code>.</p> |
| `default_space_settings` | String |  | <p>The default settings for shared spaces that users create in the domain.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `vpc_id` | String | <p>The ID of the Amazon Virtual Private Cloud (VPC) that the domain uses for communication.</p> |
| `app_security_group_management` | String | <p>The entity that creates and manages the required security groups for inter-app communication in <code>VPCOnly</code> mode. Required when <code>CreateDomain.AppNetworkAccessType</code> is <code>VPCOnly</code> and <code>DomainSettings.RStudioServerProDomainSettings.DomainExecutionRoleArn</code> is provided.</p> |
| `status` | String | <p>The status.</p> |
| `domain_name` | String | <p>The domain name.</p> |
| `security_group_id_for_domain_boundary` | String | <p>The ID of the security group that authorizes traffic between the <code>RSessionGateway</code> apps and the <code>RStudioServerPro</code> app.</p> |
| `failure_reason` | String | <p>The failure reason.</p> |
| `url` | String | <p>The domain's URL.</p> |
| `kms_key_id` | String | <p>The Amazon Web Services KMS customer managed key used to encrypt the EFS volume attached to the domain.</p> |
| `domain_settings` | String | <p>A collection of <code>Domain</code> settings.</p> |
| `single_sign_on_application_arn` | String | <p>The ARN of the application managed by SageMaker AI in IAM Identity Center. This value is only returned for domains created after October 1, 2023.</p> |
| `subnet_ids` | Vec<String> | <p>The VPC subnets that the domain uses for communication.</p> |
| `tag_propagation` | String | <p>Indicates whether custom tag propagation is supported for the domain.</p> |
| `auth_mode` | String | <p>The domain's authentication mode.</p> |
| `default_space_settings` | String | <p>The default settings for shared spaces that users create in the domain.</p> |
| `last_modified_time` | String | <p>The last modified time.</p> |
| `domain_arn` | String | <p>The domain's Amazon Resource Name (ARN).</p> |
| `single_sign_on_managed_application_instance_id` | String | <p>The IAM Identity Center managed application instance ID.</p> |
| `creation_time` | String | <p>The creation time.</p> |
| `app_network_access_type` | String | <p>Specifies the VPC used for non-EFS traffic. The default value is <code>PublicInternetOnly</code>.</p> <ul> <li> <p> <code>PublicInternetOnly</code> - Non-EFS traffic is through a VPC managed by Amazon SageMaker AI, which allows direct internet access</p> </li> <li> <p> <code>VpcOnly</code> - All traffic is through the specified VPC and subnets</p> </li> </ul> |
| `domain_id` | String | <p>The domain ID.</p> |
| `default_user_settings` | String | <p>Settings which are applied to UserProfiles in this domain if settings are not explicitly specified in a given UserProfile. </p> |
| `home_efs_file_system_id` | String | <p>The ID of the Amazon Elastic File System managed by this Domain.</p> |
| `home_efs_file_system_kms_key_id` | String | <p>Use <code>KmsKeyId</code>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create domain
domain = provider.sagemaker.Domain {
    auth_mode = "value"  # <p>The mode of authentication that members use to access the domain.</p>
    default_user_settings = "value"  # <p>The default settings to use to create a user profile when <code>UserSettings</code> isn't specified in the call to the <code>CreateUserProfile</code> API.</p> <p> <code>SecurityGroups</code> is aggregated when specified in both calls. For all other settings in <code>UserSettings</code>, the values specified in <code>CreateUserProfile</code> take precedence over those specified in <code>CreateDomain</code>.</p>
    domain_name = "value"  # <p>A name for the domain.</p>
}

# Access domain outputs
domain_id = domain.id
domain_vpc_id = domain.vpc_id
domain_app_security_group_management = domain.app_security_group_management
domain_status = domain.status
domain_domain_name = domain.domain_name
domain_security_group_id_for_domain_boundary = domain.security_group_id_for_domain_boundary
domain_failure_reason = domain.failure_reason
domain_url = domain.url
domain_kms_key_id = domain.kms_key_id
domain_domain_settings = domain.domain_settings
domain_single_sign_on_application_arn = domain.single_sign_on_application_arn
domain_subnet_ids = domain.subnet_ids
domain_tag_propagation = domain.tag_propagation
domain_auth_mode = domain.auth_mode
domain_default_space_settings = domain.default_space_settings
domain_last_modified_time = domain.last_modified_time
domain_domain_arn = domain.domain_arn
domain_single_sign_on_managed_application_instance_id = domain.single_sign_on_managed_application_instance_id
domain_creation_time = domain.creation_time
domain_app_network_access_type = domain.app_network_access_type
domain_domain_id = domain.domain_id
domain_default_user_settings = domain.default_user_settings
domain_home_efs_file_system_id = domain.home_efs_file_system_id
domain_home_efs_file_system_kms_key_id = domain.home_efs_file_system_kms_key_id
```

---


### Flow_definition

FlowDefinition resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>An array of key-value pairs that contain metadata to help you categorize and organize a flow definition. Each tag consists of a key and a value, both of which you define.</p> |
| `human_loop_request_source` | String |  | <p>Container for configuring the source of human task requests. Use to specify if Amazon Rekognition or Amazon Textract is used as an integration source.</p> |
| `human_loop_activation_config` | String |  | <p>An object containing information about the events that trigger a human workflow.</p> |
| `human_loop_config` | String |  | <p>An object containing information about the tasks the human reviewers will perform.</p> |
| `output_config` | String | ✅ | <p>An object containing information about where the human review results will be uploaded.</p> |
| `flow_definition_name` | String | ✅ | <p>The name of your flow definition.</p> |
| `role_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the role needed to call other services on your behalf. For example, <code>arn:aws:iam::1234567890:role/service-role/AmazonSageMaker-ExecutionRole-20180111T151298</code>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `failure_reason` | String | <p>The reason your flow definition failed.</p> |
| `flow_definition_status` | String | <p>The status of the flow definition. Valid values are listed below.</p> |
| `flow_definition_name` | String | <p>The Amazon Resource Name (ARN) of the flow definition.</p> |
| `creation_time` | String | <p>The timestamp when the flow definition was created.</p> |
| `role_arn` | String | <p>The Amazon Resource Name (ARN) of the Amazon Web Services Identity and Access Management (IAM) execution role for the flow definition.</p> |
| `human_loop_activation_config` | String | <p>An object containing information about what triggers a human review workflow.</p> |
| `human_loop_request_source` | String | <p>Container for configuring the source of human task requests. Used to specify if Amazon Rekognition or Amazon Textract is used as an integration source.</p> |
| `flow_definition_arn` | String | <p>The Amazon Resource Name (ARN) of the flow defintion.</p> |
| `human_loop_config` | String | <p>An object containing information about who works on the task, the workforce task price, and other task details.</p> |
| `output_config` | String | <p>An object containing information about the output file.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create flow_definition
flow_definition = provider.sagemaker.Flow_definition {
    output_config = "value"  # <p>An object containing information about where the human review results will be uploaded.</p>
    flow_definition_name = "value"  # <p>The name of your flow definition.</p>
    role_arn = "value"  # <p>The Amazon Resource Name (ARN) of the role needed to call other services on your behalf. For example, <code>arn:aws:iam::1234567890:role/service-role/AmazonSageMaker-ExecutionRole-20180111T151298</code>.</p>
}

# Access flow_definition outputs
flow_definition_id = flow_definition.id
flow_definition_failure_reason = flow_definition.failure_reason
flow_definition_flow_definition_status = flow_definition.flow_definition_status
flow_definition_flow_definition_name = flow_definition.flow_definition_name
flow_definition_creation_time = flow_definition.creation_time
flow_definition_role_arn = flow_definition.role_arn
flow_definition_human_loop_activation_config = flow_definition.human_loop_activation_config
flow_definition_human_loop_request_source = flow_definition.human_loop_request_source
flow_definition_flow_definition_arn = flow_definition.flow_definition_arn
flow_definition_human_loop_config = flow_definition.human_loop_config
flow_definition_output_config = flow_definition.output_config
```

---


### Hub_content_presigned_urls

HubContentPresignedUrls resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `hub_content_name` | String | ✅ | <p>The name of the hub content for which to generate presigned URLs. This identifies the specific model or content within the hub.</p> |
| `access_config` | String |  | <p>Configuration settings for accessing the hub content, including end-user license agreement acceptance for gated models and expected S3 URL validation.</p> |
| `max_results` | i64 |  | <p>The maximum number of presigned URLs to return in the response. Default value is 100. Large models may contain hundreds of files, requiring pagination to retrieve all URLs.</p> |
| `hub_content_version` | String |  | <p>The version of the hub content. If not specified, the latest version is used.</p> |
| `next_token` | String |  | <p> A token for pagination. Use this token to retrieve the next set of presigned URLs when the response is truncated.</p> |
| `hub_content_type` | String | ✅ | <p>The type of hub content to access. Valid values include <code>Model</code>, <code>Notebook</code>, and <code>ModelReference</code>.</p> |
| `hub_name` | String | ✅ | <p>The name or Amazon Resource Name (ARN) of the hub that contains the content. For public content, use <code>SageMakerPublicHub</code>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create hub_content_presigned_urls
hub_content_presigned_urls = provider.sagemaker.Hub_content_presigned_urls {
    hub_content_name = "value"  # <p>The name of the hub content for which to generate presigned URLs. This identifies the specific model or content within the hub.</p>
    hub_content_type = "value"  # <p>The type of hub content to access. Valid values include <code>Model</code>, <code>Notebook</code>, and <code>ModelReference</code>.</p>
    hub_name = "value"  # <p>The name or Amazon Resource Name (ARN) of the hub that contains the content. For public content, use <code>SageMakerPublicHub</code>.</p>
}

```

---


### Edge_deployment_stage

EdgeDeploymentStage resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `stages` | Vec<String> | ✅ | <p>List of stages to be added to the edge deployment plan.</p> |
| `edge_deployment_plan_name` | String | ✅ | <p>The name of the edge deployment plan.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create edge_deployment_stage
edge_deployment_stage = provider.sagemaker.Edge_deployment_stage {
    stages = "value"  # <p>List of stages to be added to the edge deployment plan.</p>
    edge_deployment_plan_name = "value"  # <p>The name of the edge deployment plan.</p>
}

```

---


### Inference_component

InferenceComponent resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `inference_component_name` | String | ✅ | <p>A unique name to assign to the inference component.</p> |
| `variant_name` | String |  | <p>The name of an existing production variant where you host the inference component.</p> |
| `runtime_config` | String |  | <p>Runtime settings for a model that is deployed with an inference component.</p> |
| `tags` | Vec<String> |  | <p>A list of key-value pairs associated with the model. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services resources</a> in the <i>Amazon Web Services General Reference</i>.</p> |
| `specification` | String | ✅ | <p>Details about the resources to deploy with this inference component, including the model, container, and compute resources.</p> |
| `endpoint_name` | String | ✅ | <p>The name of an existing endpoint where you host the inference component.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `creation_time` | String | <p>The time when the inference component was created.</p> |
| `endpoint_name` | String | <p>The name of the endpoint that hosts the inference component.</p> |
| `inference_component_status` | String | <p>The status of the inference component.</p> |
| `failure_reason` | String | <p>If the inference component status is <code>Failed</code>, the reason for the failure.</p> |
| `variant_name` | String | <p>The name of the production variant that hosts the inference component.</p> |
| `last_modified_time` | String | <p>The time when the inference component was last updated.</p> |
| `inference_component_name` | String | <p>The name of the inference component.</p> |
| `endpoint_arn` | String | <p>The Amazon Resource Name (ARN) of the endpoint that hosts the inference component.</p> |
| `inference_component_arn` | String | <p>The Amazon Resource Name (ARN) of the inference component.</p> |
| `runtime_config` | String | <p>Details about the runtime settings for the model that is deployed with the inference component.</p> |
| `last_deployment_config` | String | <p>The deployment and rollback settings that you assigned to the inference component.</p> |
| `specification` | String | <p>Details about the resources that are deployed with this inference component.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create inference_component
inference_component = provider.sagemaker.Inference_component {
    inference_component_name = "value"  # <p>A unique name to assign to the inference component.</p>
    specification = "value"  # <p>Details about the resources to deploy with this inference component, including the model, container, and compute resources.</p>
    endpoint_name = "value"  # <p>The name of an existing endpoint where you host the inference component.</p>
}

# Access inference_component outputs
inference_component_id = inference_component.id
inference_component_creation_time = inference_component.creation_time
inference_component_endpoint_name = inference_component.endpoint_name
inference_component_inference_component_status = inference_component.inference_component_status
inference_component_failure_reason = inference_component.failure_reason
inference_component_variant_name = inference_component.variant_name
inference_component_last_modified_time = inference_component.last_modified_time
inference_component_inference_component_name = inference_component.inference_component_name
inference_component_endpoint_arn = inference_component.endpoint_arn
inference_component_inference_component_arn = inference_component.inference_component_arn
inference_component_runtime_config = inference_component.runtime_config
inference_component_last_deployment_config = inference_component.last_deployment_config
inference_component_specification = inference_component.specification
```

---


### Model_card_export_job

ModelCardExportJob resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `model_card_name` | String | ✅ | <p>The name or Amazon Resource Name (ARN) of the model card to export.</p> |
| `model_card_version` | i64 |  | <p>The version of the model card to export. If a version is not provided, then the latest version of the model card is exported.</p> |
| `output_config` | String | ✅ | <p>The model card output configuration that specifies the Amazon S3 path for exporting.</p> |
| `model_card_export_job_name` | String | ✅ | <p>The name of the model card export job.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `model_card_export_job_arn` | String | <p>The Amazon Resource Name (ARN) of the model card export job.</p> |
| `model_card_export_job_name` | String | <p>The name of the model card export job to describe.</p> |
| `status` | String | <p>The completion status of the model card export job.</p> <ul> <li> <p> <code>InProgress</code>: The model card export job is in progress.</p> </li> <li> <p> <code>Completed</code>: The model card export job is complete.</p> </li> <li> <p> <code>Failed</code>: The model card export job failed. To see the reason for the failure, see the <code>FailureReason</code> field in the response to a <code>DescribeModelCardExportJob</code> call.</p> </li> </ul> |
| `created_at` | String | <p>The date and time that the model export job was created.</p> |
| `failure_reason` | String | <p>The failure reason if the model export job fails.</p> |
| `export_artifacts` | String | <p>The exported model card artifacts.</p> |
| `output_config` | String | <p>The export output details for the model card.</p> |
| `model_card_version` | i64 | <p>The version of the model card that the model export job exports.</p> |
| `model_card_name` | String | <p>The name or Amazon Resource Name (ARN) of the model card that the model export job exports.</p> |
| `last_modified_at` | String | <p>The date and time that the model export job was last modified.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create model_card_export_job
model_card_export_job = provider.sagemaker.Model_card_export_job {
    model_card_name = "value"  # <p>The name or Amazon Resource Name (ARN) of the model card to export.</p>
    output_config = "value"  # <p>The model card output configuration that specifies the Amazon S3 path for exporting.</p>
    model_card_export_job_name = "value"  # <p>The name of the model card export job.</p>
}

# Access model_card_export_job outputs
model_card_export_job_id = model_card_export_job.id
model_card_export_job_model_card_export_job_arn = model_card_export_job.model_card_export_job_arn
model_card_export_job_model_card_export_job_name = model_card_export_job.model_card_export_job_name
model_card_export_job_status = model_card_export_job.status
model_card_export_job_created_at = model_card_export_job.created_at
model_card_export_job_failure_reason = model_card_export_job.failure_reason
model_card_export_job_export_artifacts = model_card_export_job.export_artifacts
model_card_export_job_output_config = model_card_export_job.output_config
model_card_export_job_model_card_version = model_card_export_job.model_card_version
model_card_export_job_model_card_name = model_card_export_job.model_card_name
model_card_export_job_last_modified_at = model_card_export_job.last_modified_at
```

---


### Model_package_group

ModelPackageGroup resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `model_package_group_name` | String | ✅ | <p>The name of the model group.</p> |
| `tags` | Vec<String> |  | <p>A list of key value pairs associated with the model group. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services resources</a> in the <i>Amazon Web Services General Reference Guide</i>.</p> |
| `model_package_group_description` | String |  | <p>A description for the model group.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `model_package_group_name` | String | <p>The name of the model group.</p> |
| `model_package_group_arn` | String | <p>The Amazon Resource Name (ARN) of the model group.</p> |
| `creation_time` | String | <p>The time that the model group was created.</p> |
| `model_package_group_description` | String | <p>A description of the model group.</p> |
| `created_by` | String |  |
| `model_package_group_status` | String | <p>The status of the model group.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create model_package_group
model_package_group = provider.sagemaker.Model_package_group {
    model_package_group_name = "value"  # <p>The name of the model group.</p>
}

# Access model_package_group outputs
model_package_group_id = model_package_group.id
model_package_group_model_package_group_name = model_package_group.model_package_group_name
model_package_group_model_package_group_arn = model_package_group.model_package_group_arn
model_package_group_creation_time = model_package_group.creation_time
model_package_group_model_package_group_description = model_package_group.model_package_group_description
model_package_group_created_by = model_package_group.created_by
model_package_group_model_package_group_status = model_package_group.model_package_group_status
```

---


### Monitoring_schedule

MonitoringSchedule resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>(Optional) An array of key-value pairs. For more information, see <a href=" https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html#allocation-whatURL">Using Cost Allocation Tags</a> in the <i>Amazon Web Services Billing and Cost Management User Guide</i>.</p> |
| `monitoring_schedule_config` | String | ✅ | <p>The configuration object that specifies the monitoring schedule and defines the monitoring job.</p> |
| `monitoring_schedule_name` | String | ✅ | <p>The name of the monitoring schedule. The name must be unique within an Amazon Web Services Region within an Amazon Web Services account.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `monitoring_schedule_arn` | String | <p>The Amazon Resource Name (ARN) of the monitoring schedule.</p> |
| `endpoint_name` | String | <p> The name of the endpoint for the monitoring job.</p> |
| `last_monitoring_execution_summary` | String | <p>Describes metadata on the last execution to run, if there was one.</p> |
| `failure_reason` | String | <p>A string, up to one KB in size, that contains the reason a monitoring job failed, if it failed.</p> |
| `last_modified_time` | String | <p>The time at which the monitoring job was last modified.</p> |
| `monitoring_schedule_config` | String | <p>The configuration object that specifies the monitoring schedule and defines the monitoring job.</p> |
| `monitoring_schedule_name` | String | <p>Name of the monitoring schedule.</p> |
| `monitoring_type` | String | <p>The type of the monitoring job that this schedule runs. This is one of the following values.</p> <ul> <li> <p> <code>DATA_QUALITY</code> - The schedule is for a data quality monitoring job.</p> </li> <li> <p> <code>MODEL_QUALITY</code> - The schedule is for a model quality monitoring job.</p> </li> <li> <p> <code>MODEL_BIAS</code> - The schedule is for a bias monitoring job.</p> </li> <li> <p> <code>MODEL_EXPLAINABILITY</code> - The schedule is for an explainability monitoring job.</p> </li> </ul> |
| `creation_time` | String | <p>The time at which the monitoring job was created.</p> |
| `monitoring_schedule_status` | String | <p>The status of an monitoring job.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create monitoring_schedule
monitoring_schedule = provider.sagemaker.Monitoring_schedule {
    monitoring_schedule_config = "value"  # <p>The configuration object that specifies the monitoring schedule and defines the monitoring job.</p>
    monitoring_schedule_name = "value"  # <p>The name of the monitoring schedule. The name must be unique within an Amazon Web Services Region within an Amazon Web Services account.</p>
}

# Access monitoring_schedule outputs
monitoring_schedule_id = monitoring_schedule.id
monitoring_schedule_monitoring_schedule_arn = monitoring_schedule.monitoring_schedule_arn
monitoring_schedule_endpoint_name = monitoring_schedule.endpoint_name
monitoring_schedule_last_monitoring_execution_summary = monitoring_schedule.last_monitoring_execution_summary
monitoring_schedule_failure_reason = monitoring_schedule.failure_reason
monitoring_schedule_last_modified_time = monitoring_schedule.last_modified_time
monitoring_schedule_monitoring_schedule_config = monitoring_schedule.monitoring_schedule_config
monitoring_schedule_monitoring_schedule_name = monitoring_schedule.monitoring_schedule_name
monitoring_schedule_monitoring_type = monitoring_schedule.monitoring_type
monitoring_schedule_creation_time = monitoring_schedule.creation_time
monitoring_schedule_monitoring_schedule_status = monitoring_schedule.monitoring_schedule_status
```

---


### Partner_app_presigned_url

PartnerAppPresignedUrl resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `arn` | String | ✅ | <p>The ARN of the SageMaker Partner AI App to create the presigned URL for.</p> |
| `expires_in_seconds` | i64 |  | <p>The time that will pass before the presigned URL expires.</p> |
| `session_expiration_duration_in_seconds` | i64 |  | <p>Indicates how long the Amazon SageMaker Partner AI App session can be accessed for after logging in.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create partner_app_presigned_url
partner_app_presigned_url = provider.sagemaker.Partner_app_presigned_url {
    arn = "value"  # <p>The ARN of the SageMaker Partner AI App to create the presigned URL for.</p>
}

```

---


### Project

Project resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>An array of key-value pairs that you want to use to organize and track your Amazon Web Services resource costs. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services resources</a> in the <i>Amazon Web Services General Reference Guide</i>.</p> |
| `service_catalog_provisioning_details` | String |  | <p>The product ID and provisioning artifact ID to provision a service catalog. The provisioning artifact ID will default to the latest provisioning artifact ID of the product, if you don't provide the provisioning artifact ID. For more information, see <a href="https://docs.aws.amazon.com/servicecatalog/latest/adminguide/introduction.html">What is Amazon Web Services Service Catalog</a>.</p> |
| `template_providers` | Vec<String> |  | <p> An array of template provider configurations for creating infrastructure resources for the project. </p> |
| `project_name` | String | ✅ | <p>The name of the project.</p> |
| `project_description` | String |  | <p>A description for the project.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `template_provider_details` | Vec<String> | <p> An array of template providers associated with the project. </p> |
| `created_by` | String |  |
| `last_modified_time` | String | <p>The timestamp when project was last modified.</p> |
| `project_description` | String | <p>The description of the project.</p> |
| `project_arn` | String | <p>The Amazon Resource Name (ARN) of the project.</p> |
| `service_catalog_provisioning_details` | String | <p>Information used to provision a service catalog product. For information, see <a href="https://docs.aws.amazon.com/servicecatalog/latest/adminguide/introduction.html">What is Amazon Web Services Service Catalog</a>.</p> |
| `project_name` | String | <p>The name of the project.</p> |
| `last_modified_by` | String |  |
| `creation_time` | String | <p>The time when the project was created.</p> |
| `service_catalog_provisioned_product_details` | String | <p>Information about a provisioned service catalog product.</p> |
| `project_status` | String | <p>The status of the project.</p> |
| `project_id` | String | <p>The ID of the project.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create project
project = provider.sagemaker.Project {
    project_name = "value"  # <p>The name of the project.</p>
}

# Access project outputs
project_id = project.id
project_template_provider_details = project.template_provider_details
project_created_by = project.created_by
project_last_modified_time = project.last_modified_time
project_project_description = project.project_description
project_project_arn = project.project_arn
project_service_catalog_provisioning_details = project.service_catalog_provisioning_details
project_project_name = project.project_name
project_last_modified_by = project.last_modified_by
project_creation_time = project.creation_time
project_service_catalog_provisioned_product_details = project.service_catalog_provisioned_product_details
project_project_status = project.project_status
project_project_id = project.project_id
```

---


### Space

Space resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `space_sharing_settings` | String |  | <p>A collection of space sharing settings.</p> |
| `space_display_name` | String |  | <p>The name of the space that appears in the SageMaker Studio UI.</p> |
| `space_settings` | String |  | <p>A collection of space settings.</p> |
| `space_name` | String | ✅ | <p>The name of the space.</p> |
| `domain_id` | String | ✅ | <p>The ID of the associated domain.</p> |
| `tags` | Vec<String> |  | <p>Tags to associated with the space. Each tag consists of a key and an optional value. Tag keys must be unique for each resource. Tags are searchable using the <code>Search</code> API.</p> |
| `ownership_settings` | String |  | <p>A collection of ownership settings.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `failure_reason` | String | <p>The failure reason.</p> |
| `domain_id` | String | <p>The ID of the associated domain.</p> |
| `space_arn` | String | <p>The space's Amazon Resource Name (ARN).</p> |
| `status` | String | <p>The status.</p> |
| `home_efs_file_system_uid` | String | <p>The ID of the space's profile in the Amazon EFS volume.</p> |
| `url` | String | <p>Returns the URL of the space. If the space is created with Amazon Web Services IAM Identity Center (Successor to Amazon Web Services Single Sign-On) authentication, users can navigate to the URL after appending the respective redirect parameter for the application type to be federated through Amazon Web Services IAM Identity Center.</p> <p>The following application types are supported:</p> <ul> <li> <p>Studio Classic: <code>&amp;redirect=JupyterServer</code> </p> </li> <li> <p>JupyterLab: <code>&amp;redirect=JupyterLab</code> </p> </li> <li> <p>Code Editor, based on Code-OSS, Visual Studio Code - Open Source: <code>&amp;redirect=CodeEditor</code> </p> </li> </ul> |
| `space_settings` | String | <p>A collection of space settings.</p> |
| `ownership_settings` | String | <p>The collection of ownership settings for a space.</p> |
| `last_modified_time` | String | <p>The last modified time.</p> |
| `creation_time` | String | <p>The creation time.</p> |
| `space_name` | String | <p>The name of the space.</p> |
| `space_display_name` | String | <p>The name of the space that appears in the Amazon SageMaker Studio UI.</p> |
| `space_sharing_settings` | String | <p>The collection of space sharing settings for a space.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create space
space = provider.sagemaker.Space {
    space_name = "value"  # <p>The name of the space.</p>
    domain_id = "value"  # <p>The ID of the associated domain.</p>
}

# Access space outputs
space_id = space.id
space_failure_reason = space.failure_reason
space_domain_id = space.domain_id
space_space_arn = space.space_arn
space_status = space.status
space_home_efs_file_system_uid = space.home_efs_file_system_uid
space_url = space.url
space_space_settings = space.space_settings
space_ownership_settings = space.ownership_settings
space_last_modified_time = space.last_modified_time
space_creation_time = space.creation_time
space_space_name = space.space_name
space_space_display_name = space.space_display_name
space_space_sharing_settings = space.space_sharing_settings
```

---


### Transform_job

TransformJob resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `batch_strategy` | String |  | <p>Specifies the number of records to include in a mini-batch for an HTTP inference request. A <i>record</i> <i/> is a single unit of input data that inference can be made on. For example, a single line in a CSV file is a record. </p> <p>To enable the batch strategy, you must set the <code>SplitType</code> property to <code>Line</code>, <code>RecordIO</code>, or <code>TFRecord</code>.</p> <p>To use only one record when making an HTTP invocation request to a container, set <code>BatchStrategy</code> to <code>SingleRecord</code> and <code>SplitType</code> to <code>Line</code>.</p> <p>To fit as many records in a mini-batch as can fit within the <code>MaxPayloadInMB</code> limit, set <code>BatchStrategy</code> to <code>MultiRecord</code> and <code>SplitType</code> to <code>Line</code>.</p> |
| `transform_job_name` | String | ✅ | <p>The name of the transform job. The name must be unique within an Amazon Web Services Region in an Amazon Web Services account. </p> |
| `transform_output` | String | ✅ | <p>Describes the results of the transform job.</p> |
| `model_client_config` | String |  | <p>Configures the timeout and maximum number of retries for processing a transform job invocation.</p> |
| `environment` | HashMap<String, String> |  | <p>The environment variables to set in the Docker container. Don't include any sensitive data in your environment variables. We support up to 16 key and values entries in the map.</p> |
| `transform_input` | String | ✅ | <p>Describes the input source and the way the transform job consumes it.</p> |
| `transform_resources` | String | ✅ | <p>Describes the resources, including ML instance types and ML instance count, to use for the transform job.</p> |
| `max_payload_in_mb` | i64 |  | <p>The maximum allowed size of the payload, in MB. A <i>payload</i> is the data portion of a record (without metadata). The value in <code>MaxPayloadInMB</code> must be greater than, or equal to, the size of a single record. To estimate the size of a record in MB, divide the size of your dataset by the number of records. To ensure that the records fit within the maximum payload size, we recommend using a slightly larger value. The default value is <code>6</code> MB. </p> <p>The value of <code>MaxPayloadInMB</code> cannot be greater than 100 MB. If you specify the <code>MaxConcurrentTransforms</code> parameter, the value of <code>(MaxConcurrentTransforms * MaxPayloadInMB)</code> also cannot exceed 100 MB.</p> <p>For cases where the payload might be arbitrarily large and is transmitted using HTTP chunked encoding, set the value to <code>0</code>. This feature works only in supported algorithms. Currently, Amazon SageMaker built-in algorithms do not support HTTP chunked encoding.</p> |
| `max_concurrent_transforms` | i64 |  | <p>The maximum number of parallel requests that can be sent to each instance in a transform job. If <code>MaxConcurrentTransforms</code> is set to <code>0</code> or left unset, Amazon SageMaker checks the optional execution-parameters to determine the settings for your chosen algorithm. If the execution-parameters endpoint is not enabled, the default value is <code>1</code>. For more information on execution-parameters, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/your-algorithms-batch-code.html#your-algorithms-batch-code-how-containe-serves-requests">How Containers Serve Requests</a>. For built-in algorithms, you don't need to set a value for <code>MaxConcurrentTransforms</code>.</p> |
| `data_processing` | String |  | <p>The data structure used to specify the data to be used for inference in a batch transform job and to associate the data that is relevant to the prediction results in the output. The input filter provided allows you to exclude input data that is not needed for inference in a batch transform job. The output filter provided allows you to include input data relevant to interpreting the predictions in the output from the job. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/batch-transform-data-processing.html">Associate Prediction Results with their Corresponding Input Records</a>.</p> |
| `experiment_config` | String |  |  |
| `tags` | Vec<String> |  | <p>(Optional) An array of key-value pairs. For more information, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html#allocation-what">Using Cost Allocation Tags</a> in the <i>Amazon Web Services Billing and Cost Management User Guide</i>.</p> |
| `model_name` | String | ✅ | <p>The name of the model that you want to use for the transform job. <code>ModelName</code> must be the name of an existing Amazon SageMaker model within an Amazon Web Services Region in an Amazon Web Services account.</p> |
| `data_capture_config` | String |  | <p>Configuration to control how SageMaker captures inference data.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `failure_reason` | String | <p>If the transform job failed, <code>FailureReason</code> describes why it failed. A transform job creates a log file, which includes error messages, and stores it as an Amazon S3 object. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/logging-cloudwatch.html">Log Amazon SageMaker Events with Amazon CloudWatch</a>.</p> |
| `transform_job_name` | String | <p>The name of the transform job.</p> |
| `batch_strategy` | String | <p>Specifies the number of records to include in a mini-batch for an HTTP inference request. A <i>record</i> <i/> is a single unit of input data that inference can be made on. For example, a single line in a CSV file is a record. </p> <p>To enable the batch strategy, you must set <code>SplitType</code> to <code>Line</code>, <code>RecordIO</code>, or <code>TFRecord</code>.</p> |
| `transform_start_time` | String | <p>Indicates when the transform job starts on ML instances. You are billed for the time interval between this time and the value of <code>TransformEndTime</code>.</p> |
| `data_processing` | String |  |
| `model_name` | String | <p>The name of the model used in the transform job.</p> |
| `environment` | HashMap<String, String> | <p>The environment variables to set in the Docker container. We support up to 16 key and values entries in the map.</p> |
| `max_payload_in_mb` | i64 | <p>The maximum payload size, in MB, used in the transform job.</p> |
| `data_capture_config` | String | <p>Configuration to control how SageMaker captures inference data.</p> |
| `transform_output` | String | <p>Identifies the Amazon S3 location where you want Amazon SageMaker to save the results from the transform job.</p> |
| `labeling_job_arn` | String | <p>The Amazon Resource Name (ARN) of the Amazon SageMaker Ground Truth labeling job that created the transform or training job.</p> |
| `transform_job_status` | String | <p>The status of the transform job. If the transform job failed, the reason is returned in the <code>FailureReason</code> field.</p> |
| `model_client_config` | String | <p>The timeout and maximum number of retries for processing a transform job invocation.</p> |
| `creation_time` | String | <p>A timestamp that shows when the transform Job was created.</p> |
| `transform_job_arn` | String | <p>The Amazon Resource Name (ARN) of the transform job.</p> |
| `max_concurrent_transforms` | i64 | <p>The maximum number of parallel requests on each instance node that can be launched in a transform job. The default value is 1.</p> |
| `transform_input` | String | <p>Describes the dataset to be transformed and the Amazon S3 location where it is stored.</p> |
| `transform_resources` | String | <p>Describes the resources, including ML instance types and ML instance count, to use for the transform job.</p> |
| `transform_end_time` | String | <p>Indicates when the transform job has been completed, or has stopped or failed. You are billed for the time interval between this time and the value of <code>TransformStartTime</code>.</p> |
| `auto_ml_job_arn` | String | <p>The Amazon Resource Name (ARN) of the AutoML transform job.</p> |
| `experiment_config` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create transform_job
transform_job = provider.sagemaker.Transform_job {
    transform_job_name = "value"  # <p>The name of the transform job. The name must be unique within an Amazon Web Services Region in an Amazon Web Services account. </p>
    transform_output = "value"  # <p>Describes the results of the transform job.</p>
    transform_input = "value"  # <p>Describes the input source and the way the transform job consumes it.</p>
    transform_resources = "value"  # <p>Describes the resources, including ML instance types and ML instance count, to use for the transform job.</p>
    model_name = "value"  # <p>The name of the model that you want to use for the transform job. <code>ModelName</code> must be the name of an existing Amazon SageMaker model within an Amazon Web Services Region in an Amazon Web Services account.</p>
}

# Access transform_job outputs
transform_job_id = transform_job.id
transform_job_failure_reason = transform_job.failure_reason
transform_job_transform_job_name = transform_job.transform_job_name
transform_job_batch_strategy = transform_job.batch_strategy
transform_job_transform_start_time = transform_job.transform_start_time
transform_job_data_processing = transform_job.data_processing
transform_job_model_name = transform_job.model_name
transform_job_environment = transform_job.environment
transform_job_max_payload_in_mb = transform_job.max_payload_in_mb
transform_job_data_capture_config = transform_job.data_capture_config
transform_job_transform_output = transform_job.transform_output
transform_job_labeling_job_arn = transform_job.labeling_job_arn
transform_job_transform_job_status = transform_job.transform_job_status
transform_job_model_client_config = transform_job.model_client_config
transform_job_creation_time = transform_job.creation_time
transform_job_transform_job_arn = transform_job.transform_job_arn
transform_job_max_concurrent_transforms = transform_job.max_concurrent_transforms
transform_job_transform_input = transform_job.transform_input
transform_job_transform_resources = transform_job.transform_resources
transform_job_transform_end_time = transform_job.transform_end_time
transform_job_auto_ml_job_arn = transform_job.auto_ml_job_arn
transform_job_experiment_config = transform_job.experiment_config
```

---


### Workteam

Workteam resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `notification_configuration` | String |  | <p>Configures notification of workers regarding available or expiring work items.</p> |
| `workteam_name` | String | ✅ | <p>The name of the work team. Use this name to identify the work team.</p> |
| `tags` | Vec<String> |  | <p>An array of key-value pairs.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-resource-tags.html">Resource Tag</a> and <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html#allocation-what">Using Cost Allocation Tags</a> in the <i> Amazon Web Services Billing and Cost Management User Guide</i>.</p> |
| `workforce_name` | String |  | <p>The name of the workforce.</p> |
| `member_definitions` | Vec<String> | ✅ | <p>A list of <code>MemberDefinition</code> objects that contains objects that identify the workers that make up the work team. </p> <p>Workforces can be created using Amazon Cognito or your own OIDC Identity Provider (IdP). For private workforces created using Amazon Cognito use <code>CognitoMemberDefinition</code>. For workforces created using your own OIDC identity provider (IdP) use <code>OidcMemberDefinition</code>. Do not provide input for both of these parameters in a single request.</p> <p>For workforces created using Amazon Cognito, private work teams correspond to Amazon Cognito <i>user groups</i> within the user pool used to create a workforce. All of the <code>CognitoMemberDefinition</code> objects that make up the member definition must have the same <code>ClientId</code> and <code>UserPool</code> values. To add a Amazon Cognito user group to an existing worker pool, see <a href="">Adding groups to a User Pool</a>. For more information about user pools, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-identity-pools.html">Amazon Cognito User Pools</a>.</p> <p>For workforces created using your own OIDC IdP, specify the user groups that you want to include in your private work team in <code>OidcMemberDefinition</code> by listing those groups in <code>Groups</code>.</p> |
| `description` | String | ✅ | <p>A description of the work team.</p> |
| `worker_access_configuration` | String |  | <p>Use this optional parameter to constrain access to an Amazon S3 resource based on the IP address using supported IAM global condition keys. The Amazon S3 resource is accessed in the worker portal using a Amazon S3 presigned URL.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `workteam` | String | <p>A <code>Workteam</code> instance that contains information about the work team. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create workteam
workteam = provider.sagemaker.Workteam {
    workteam_name = "value"  # <p>The name of the work team. Use this name to identify the work team.</p>
    member_definitions = "value"  # <p>A list of <code>MemberDefinition</code> objects that contains objects that identify the workers that make up the work team. </p> <p>Workforces can be created using Amazon Cognito or your own OIDC Identity Provider (IdP). For private workforces created using Amazon Cognito use <code>CognitoMemberDefinition</code>. For workforces created using your own OIDC identity provider (IdP) use <code>OidcMemberDefinition</code>. Do not provide input for both of these parameters in a single request.</p> <p>For workforces created using Amazon Cognito, private work teams correspond to Amazon Cognito <i>user groups</i> within the user pool used to create a workforce. All of the <code>CognitoMemberDefinition</code> objects that make up the member definition must have the same <code>ClientId</code> and <code>UserPool</code> values. To add a Amazon Cognito user group to an existing worker pool, see <a href="">Adding groups to a User Pool</a>. For more information about user pools, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-identity-pools.html">Amazon Cognito User Pools</a>.</p> <p>For workforces created using your own OIDC IdP, specify the user groups that you want to include in your private work team in <code>OidcMemberDefinition</code> by listing those groups in <code>Groups</code>.</p>
    description = "value"  # <p>A description of the work team.</p>
}

# Access workteam outputs
workteam_id = workteam.id
workteam_workteam = workteam.workteam
```

---


### Tags

Tags resource

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


### Lineage_group_policy

LineageGroupPolicy resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `resource_policy` | String | <p>The resource policy that gives access to the lineage group in another account.</p> |
| `lineage_group_arn` | String | <p>The Amazon Resource Name (ARN) of the lineage group.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access lineage_group_policy outputs
lineage_group_policy_id = lineage_group_policy.id
lineage_group_policy_resource_policy = lineage_group_policy.resource_policy
lineage_group_policy_lineage_group_arn = lineage_group_policy.lineage_group_arn
```

---


### Pipeline

Pipeline resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `pipeline_description` | String |  | <p>A description of the pipeline.</p> |
| `role_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the role used by the pipeline to access and create resources.</p> |
| `parallelism_configuration` | String |  | <p>This is the configuration that controls the parallelism of the pipeline. If specified, it applies to all runs of this pipeline by default.</p> |
| `pipeline_display_name` | String |  | <p>The display name of the pipeline.</p> |
| `tags` | Vec<String> |  | <p>A list of tags to apply to the created pipeline.</p> |
| `pipeline_definition_s3_location` | String |  | <p>The location of the pipeline definition stored in Amazon S3. If specified, SageMaker will retrieve the pipeline definition from this location.</p> |
| `client_request_token` | String | ✅ | <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the operation. An idempotent operation completes no more than one time.</p> |
| `pipeline_definition` | String |  | <p>The <a href="https://aws-sagemaker-mlops.github.io/sagemaker-model-building-pipeline-definition-JSON-schema/">JSON pipeline definition</a> of the pipeline.</p> |
| `pipeline_name` | String | ✅ | <p>The name of the pipeline.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `parallelism_configuration` | String | <p>Lists the parallelism configuration applied to the pipeline.</p> |
| `pipeline_status` | String | <p>The status of the pipeline execution.</p> |
| `pipeline_version_description` | String | <p>The description of the pipeline version.</p> |
| `role_arn` | String | <p>The Amazon Resource Name (ARN) that the pipeline uses to execute.</p> |
| `pipeline_version_display_name` | String | <p>The display name of the pipeline version.</p> |
| `last_run_time` | String | <p>The time when the pipeline was last run.</p> |
| `pipeline_definition` | String | <p>The JSON pipeline definition.</p> |
| `pipeline_description` | String | <p>The description of the pipeline.</p> |
| `pipeline_arn` | String | <p>The Amazon Resource Name (ARN) of the pipeline.</p> |
| `creation_time` | String | <p>The time when the pipeline was created.</p> |
| `created_by` | String |  |
| `last_modified_by` | String |  |
| `pipeline_name` | String | <p>The name of the pipeline.</p> |
| `last_modified_time` | String | <p>The time when the pipeline was last modified.</p> |
| `pipeline_display_name` | String | <p>The display name of the pipeline.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create pipeline
pipeline = provider.sagemaker.Pipeline {
    role_arn = "value"  # <p>The Amazon Resource Name (ARN) of the role used by the pipeline to access and create resources.</p>
    client_request_token = "value"  # <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the operation. An idempotent operation completes no more than one time.</p>
    pipeline_name = "value"  # <p>The name of the pipeline.</p>
}

# Access pipeline outputs
pipeline_id = pipeline.id
pipeline_parallelism_configuration = pipeline.parallelism_configuration
pipeline_pipeline_status = pipeline.pipeline_status
pipeline_pipeline_version_description = pipeline.pipeline_version_description
pipeline_role_arn = pipeline.role_arn
pipeline_pipeline_version_display_name = pipeline.pipeline_version_display_name
pipeline_last_run_time = pipeline.last_run_time
pipeline_pipeline_definition = pipeline.pipeline_definition
pipeline_pipeline_description = pipeline.pipeline_description
pipeline_pipeline_arn = pipeline.pipeline_arn
pipeline_creation_time = pipeline.creation_time
pipeline_created_by = pipeline.created_by
pipeline_last_modified_by = pipeline.last_modified_by
pipeline_pipeline_name = pipeline.pipeline_name
pipeline_last_modified_time = pipeline.last_modified_time
pipeline_pipeline_display_name = pipeline.pipeline_display_name
```

---


### Scaling_configuration_recommendation

ScalingConfigurationRecommendation resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `dynamic_scaling_configuration` | String | <p>An object with the recommended values for you to specify when creating an autoscaling policy.</p> |
| `endpoint_name` | String | <p>The name of an endpoint benchmarked during a previously completed Inference Recommender job.</p> |
| `scaling_policy_objective` | String | <p>An object representing the anticipated traffic pattern for an endpoint that you specified in the request.</p> |
| `metric` | String | <p>An object with a list of metrics that were benchmarked during the previously completed Inference Recommender job.</p> |
| `inference_recommendations_job_name` | String | <p>The name of a previously completed Inference Recommender job.</p> |
| `recommendation_id` | String | <p>The recommendation ID of a previously completed inference recommendation.</p> |
| `target_cpu_utilization_per_core` | i64 | <p>The percentage of how much utilization you want an instance to use before autoscaling, which you specified in the request. The default value is 50%.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access scaling_configuration_recommendation outputs
scaling_configuration_recommendation_id = scaling_configuration_recommendation.id
scaling_configuration_recommendation_dynamic_scaling_configuration = scaling_configuration_recommendation.dynamic_scaling_configuration
scaling_configuration_recommendation_endpoint_name = scaling_configuration_recommendation.endpoint_name
scaling_configuration_recommendation_scaling_policy_objective = scaling_configuration_recommendation.scaling_policy_objective
scaling_configuration_recommendation_metric = scaling_configuration_recommendation.metric
scaling_configuration_recommendation_inference_recommendations_job_name = scaling_configuration_recommendation.inference_recommendations_job_name
scaling_configuration_recommendation_recommendation_id = scaling_configuration_recommendation.recommendation_id
scaling_configuration_recommendation_target_cpu_utilization_per_core = scaling_configuration_recommendation.target_cpu_utilization_per_core
```

---


### Endpoint_config

EndpointConfig resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `async_inference_config` | String |  | <p>Specifies configuration for how an endpoint performs asynchronous inference. This is a required field in order for your Endpoint to be invoked using <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_runtime_InvokeEndpointAsync.html">InvokeEndpointAsync</a>.</p> |
| `production_variants` | Vec<String> | ✅ | <p>An array of <code>ProductionVariant</code> objects, one for each model that you want to host at this endpoint.</p> |
| `kms_key_id` | String |  | <p>The Amazon Resource Name (ARN) of a Amazon Web Services Key Management Service key that SageMaker uses to encrypt data on the storage volume attached to the ML compute instance that hosts the endpoint.</p> <p>The KmsKeyId can be any of the following formats: </p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Alias name: <code>alias/ExampleAlias</code> </p> </li> <li> <p>Alias name ARN: <code>arn:aws:kms:us-west-2:111122223333:alias/ExampleAlias</code> </p> </li> </ul> <p>The KMS key policy must grant permission to the IAM role that you specify in your <code>CreateEndpoint</code>, <code>UpdateEndpoint</code> requests. For more information, refer to the Amazon Web Services Key Management Service section<a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html"> Using Key Policies in Amazon Web Services KMS </a> </p> <note> <p>Certain Nitro-based instances include local storage, dependent on the instance type. Local storage volumes are encrypted using a hardware module on the instance. You can't request a <code>KmsKeyId</code> when using an instance type with local storage. If any of the models that you specify in the <code>ProductionVariants</code> parameter use nitro-based instances with local storage, do not specify a value for the <code>KmsKeyId</code> parameter. If you specify a value for <code>KmsKeyId</code> when using any nitro-based instances with local storage, the call to <code>CreateEndpointConfig</code> fails.</p> <p>For a list of instance types that support local instance storage, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/InstanceStorage.html#instance-store-volumes">Instance Store Volumes</a>.</p> <p>For more information about local instance storage encryption, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ssd-instance-store.html">SSD Instance Store Volumes</a>.</p> </note> |
| `tags` | Vec<String> |  | <p>An array of key-value pairs. You can use tags to categorize your Amazon Web Services resources in different ways, for example, by purpose, owner, or environment. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services Resources</a>.</p> |
| `shadow_production_variants` | Vec<String> |  | <p>An array of <code>ProductionVariant</code> objects, one for each model that you want to host at this endpoint in shadow mode with production traffic replicated from the model specified on <code>ProductionVariants</code>. If you use this field, you can only specify one variant for <code>ProductionVariants</code> and one variant for <code>ShadowProductionVariants</code>.</p> |
| `endpoint_config_name` | String | ✅ | <p>The name of the endpoint configuration. You specify this name in a <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_CreateEndpoint.html">CreateEndpoint</a> request. </p> |
| `data_capture_config` | String |  |  |
| `explainer_config` | String |  | <p>A member of <code>CreateEndpointConfig</code> that enables explainers.</p> |
| `execution_role_arn` | String |  | <p>The Amazon Resource Name (ARN) of an IAM role that Amazon SageMaker AI can assume to perform actions on your behalf. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/sagemaker-roles.html">SageMaker AI Roles</a>. </p> <note> <p>To be able to pass this role to Amazon SageMaker AI, the caller of this action must have the <code>iam:PassRole</code> permission.</p> </note> |
| `vpc_config` | String |  |  |
| `enable_network_isolation` | bool |  | <p>Sets whether all model containers deployed to the endpoint are isolated. If they are, no inbound or outbound network calls can be made to or from the model containers.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `endpoint_config_name` | String | <p>Name of the SageMaker endpoint configuration.</p> |
| `endpoint_config_arn` | String | <p>The Amazon Resource Name (ARN) of the endpoint configuration.</p> |
| `creation_time` | String | <p>A timestamp that shows when the endpoint configuration was created.</p> |
| `async_inference_config` | String | <p>Returns the description of an endpoint configuration created using the <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_CreateEndpointConfig.html"> <code>CreateEndpointConfig</code> </a> API.</p> |
| `execution_role_arn` | String | <p>The Amazon Resource Name (ARN) of the IAM role that you assigned to the endpoint configuration.</p> |
| `data_capture_config` | String |  |
| `shadow_production_variants` | Vec<String> | <p>An array of <code>ProductionVariant</code> objects, one for each model that you want to host at this endpoint in shadow mode with production traffic replicated from the model specified on <code>ProductionVariants</code>.</p> |
| `production_variants` | Vec<String> | <p>An array of <code>ProductionVariant</code> objects, one for each model that you want to host at this endpoint.</p> |
| `vpc_config` | String |  |
| `kms_key_id` | String | <p>Amazon Web Services KMS key ID Amazon SageMaker uses to encrypt data when storing it on the ML storage volume attached to the instance.</p> |
| `explainer_config` | String | <p>The configuration parameters for an explainer.</p> |
| `enable_network_isolation` | bool | <p>Indicates whether all model containers deployed to the endpoint are isolated. If they are, no inbound or outbound network calls can be made to or from the model containers.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create endpoint_config
endpoint_config = provider.sagemaker.Endpoint_config {
    production_variants = "value"  # <p>An array of <code>ProductionVariant</code> objects, one for each model that you want to host at this endpoint.</p>
    endpoint_config_name = "value"  # <p>The name of the endpoint configuration. You specify this name in a <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_CreateEndpoint.html">CreateEndpoint</a> request. </p>
}

# Access endpoint_config outputs
endpoint_config_id = endpoint_config.id
endpoint_config_endpoint_config_name = endpoint_config.endpoint_config_name
endpoint_config_endpoint_config_arn = endpoint_config.endpoint_config_arn
endpoint_config_creation_time = endpoint_config.creation_time
endpoint_config_async_inference_config = endpoint_config.async_inference_config
endpoint_config_execution_role_arn = endpoint_config.execution_role_arn
endpoint_config_data_capture_config = endpoint_config.data_capture_config
endpoint_config_shadow_production_variants = endpoint_config.shadow_production_variants
endpoint_config_production_variants = endpoint_config.production_variants
endpoint_config_vpc_config = endpoint_config.vpc_config
endpoint_config_kms_key_id = endpoint_config.kms_key_id
endpoint_config_explainer_config = endpoint_config.explainer_config
endpoint_config_enable_network_isolation = endpoint_config.enable_network_isolation
```

---


### Mlflow_tracking_server

MlflowTrackingServer resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tracking_server_size` | String |  | <p>The size of the tracking server you want to create. You can choose between <code>"Small"</code>, <code>"Medium"</code>, and <code>"Large"</code>. The default MLflow Tracking Server configuration size is <code>"Small"</code>. You can choose a size depending on the projected use of the tracking server such as the volume of data logged, number of users, and frequency of use. </p> <p>We recommend using a small tracking server for teams of up to 25 users, a medium tracking server for teams of up to 50 users, and a large tracking server for teams of up to 100 users. </p> |
| `artifact_store_uri` | String | ✅ | <p>The S3 URI for a general purpose bucket to use as the MLflow Tracking Server artifact store.</p> |
| `tracking_server_name` | String | ✅ | <p>A unique string identifying the tracking server name. This string is part of the tracking server ARN.</p> |
| `automatic_model_registration` | bool |  | <p>Whether to enable or disable automatic registration of new MLflow models to the SageMaker Model Registry. To enable automatic model registration, set this value to <code>True</code>. To disable automatic model registration, set this value to <code>False</code>. If not specified, <code>AutomaticModelRegistration</code> defaults to <code>False</code>.</p> |
| `weekly_maintenance_window_start` | String |  | <p>The day and time of the week in Coordinated Universal Time (UTC) 24-hour standard time that weekly maintenance updates are scheduled. For example: TUE:03:30.</p> |
| `role_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) for an IAM role in your account that the MLflow Tracking Server uses to access the artifact store in Amazon S3. The role should have <code>AmazonS3FullAccess</code> permissions. For more information on IAM permissions for tracking server creation, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/mlflow-create-tracking-server-iam.html">Set up IAM permissions for MLflow</a>.</p> |
| `tags` | Vec<String> |  | <p>Tags consisting of key-value pairs used to manage metadata for the tracking server.</p> |
| `mlflow_version` | String |  | <p>The version of MLflow that the tracking server uses. To see which MLflow versions are available to use, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/mlflow.html#mlflow-create-tracking-server-how-it-works">How it works</a>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `mlflow_version` | String | <p>The MLflow version used for the described tracking server.</p> |
| `automatic_model_registration` | bool | <p>Whether automatic registration of new MLflow models to the SageMaker Model Registry is enabled.</p> |
| `artifact_store_uri` | String | <p>The S3 URI of the general purpose bucket used as the MLflow Tracking Server artifact store.</p> |
| `tracking_server_name` | String | <p>The name of the described tracking server.</p> |
| `last_modified_time` | String | <p>The timestamp of when the described MLflow Tracking Server was last modified.</p> |
| `role_arn` | String | <p>The Amazon Resource Name (ARN) for an IAM role in your account that the described MLflow Tracking Server uses to access the artifact store in Amazon S3.</p> |
| `is_active` | String | <p>Whether the described MLflow Tracking Server is currently active.</p> |
| `tracking_server_maintenance_status` | String | <p> The current maintenance status of the described MLflow Tracking Server. </p> |
| `creation_time` | String | <p>The timestamp of when the described MLflow Tracking Server was created.</p> |
| `weekly_maintenance_window_start` | String | <p>The day and time of the week when weekly maintenance occurs on the described tracking server.</p> |
| `tracking_server_url` | String | <p>The URL to connect to the MLflow user interface for the described tracking server.</p> |
| `created_by` | String |  |
| `tracking_server_arn` | String | <p>The ARN of the described tracking server.</p> |
| `tracking_server_size` | String | <p>The size of the described tracking server.</p> |
| `tracking_server_status` | String | <p>The current creation status of the described MLflow Tracking Server.</p> |
| `last_modified_by` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create mlflow_tracking_server
mlflow_tracking_server = provider.sagemaker.Mlflow_tracking_server {
    artifact_store_uri = "value"  # <p>The S3 URI for a general purpose bucket to use as the MLflow Tracking Server artifact store.</p>
    tracking_server_name = "value"  # <p>A unique string identifying the tracking server name. This string is part of the tracking server ARN.</p>
    role_arn = "value"  # <p>The Amazon Resource Name (ARN) for an IAM role in your account that the MLflow Tracking Server uses to access the artifact store in Amazon S3. The role should have <code>AmazonS3FullAccess</code> permissions. For more information on IAM permissions for tracking server creation, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/mlflow-create-tracking-server-iam.html">Set up IAM permissions for MLflow</a>.</p>
}

# Access mlflow_tracking_server outputs
mlflow_tracking_server_id = mlflow_tracking_server.id
mlflow_tracking_server_mlflow_version = mlflow_tracking_server.mlflow_version
mlflow_tracking_server_automatic_model_registration = mlflow_tracking_server.automatic_model_registration
mlflow_tracking_server_artifact_store_uri = mlflow_tracking_server.artifact_store_uri
mlflow_tracking_server_tracking_server_name = mlflow_tracking_server.tracking_server_name
mlflow_tracking_server_last_modified_time = mlflow_tracking_server.last_modified_time
mlflow_tracking_server_role_arn = mlflow_tracking_server.role_arn
mlflow_tracking_server_is_active = mlflow_tracking_server.is_active
mlflow_tracking_server_tracking_server_maintenance_status = mlflow_tracking_server.tracking_server_maintenance_status
mlflow_tracking_server_creation_time = mlflow_tracking_server.creation_time
mlflow_tracking_server_weekly_maintenance_window_start = mlflow_tracking_server.weekly_maintenance_window_start
mlflow_tracking_server_tracking_server_url = mlflow_tracking_server.tracking_server_url
mlflow_tracking_server_created_by = mlflow_tracking_server.created_by
mlflow_tracking_server_tracking_server_arn = mlflow_tracking_server.tracking_server_arn
mlflow_tracking_server_tracking_server_size = mlflow_tracking_server.tracking_server_size
mlflow_tracking_server_tracking_server_status = mlflow_tracking_server.tracking_server_status
mlflow_tracking_server_last_modified_by = mlflow_tracking_server.last_modified_by
```

---


### Training_job

TrainingJob resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `environment` | HashMap<String, String> |  | <p>The environment variables to set in the Docker container.</p> <important> <p>Do not include any security-sensitive information including account access IDs, secrets, or tokens in any environment fields. As part of the shared responsibility model, you are responsible for any potential exposure, unauthorized access, or compromise of your sensitive data if caused by security-sensitive information included in the request environment variable or plain text fields.</p> </important> |
| `enable_inter_container_traffic_encryption` | bool |  | <p>To encrypt all communications between ML compute instances in distributed training, choose <code>True</code>. Encryption provides greater security for distributed training, but training might take longer. How long it takes depends on the amount of communication between compute instances, especially if you use a deep learning algorithm in distributed training. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/train-encrypt.html">Protect Communications Between ML Compute Instances in a Distributed Training Job</a>.</p> |
| `infra_check_config` | String |  | <p>Contains information about the infrastructure health check configuration for the training job.</p> |
| `stopping_condition` | String | ✅ | <p>Specifies a limit to how long a model training job can run. It also specifies how long a managed Spot training job has to complete. When the job reaches the time limit, SageMaker ends the training job. Use this API to cap model training costs.</p> <p>To stop a job, SageMaker sends the algorithm the <code>SIGTERM</code> signal, which delays job termination for 120 seconds. Algorithms can use this 120-second window to save the model artifacts, so the results of training are not lost. </p> |
| `debug_rule_configurations` | Vec<String> |  | <p>Configuration information for Amazon SageMaker Debugger rules for debugging output tensors.</p> |
| `role_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of an IAM role that SageMaker can assume to perform tasks on your behalf. </p> <p>During model training, SageMaker needs your permission to read input data from an S3 bucket, download a Docker image that contains training code, write model artifacts to an S3 bucket, write logs to Amazon CloudWatch Logs, and publish metrics to Amazon CloudWatch. You grant permissions for all of these tasks to an IAM role. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/sagemaker-roles.html">SageMaker Roles</a>. </p> <note> <p>To be able to pass this role to SageMaker, the caller of this API must have the <code>iam:PassRole</code> permission.</p> </note> |
| `resource_config` | String | ✅ | <p>The resources, including the ML compute instances and ML storage volumes, to use for model training. </p> <p>ML storage volumes store model artifacts and incremental states. Training algorithms might also use ML storage volumes for scratch space. If you want SageMaker to use the ML storage volume to store the training data, choose <code>File</code> as the <code>TrainingInputMode</code> in the algorithm specification. For distributed training algorithms, specify an instance count greater than 1.</p> |
| `profiler_config` | String |  |  |
| `algorithm_specification` | String | ✅ | <p>The registry path of the Docker image that contains the training algorithm and algorithm-specific metadata, including the input mode. For more information about algorithms provided by SageMaker, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/algos.html">Algorithms</a>. For information about providing your own algorithms, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/your-algorithms.html">Using Your Own Algorithms with Amazon SageMaker</a>. </p> |
| `session_chaining_config` | String |  | <p>Contains information about attribute-based access control (ABAC) for the training job.</p> |
| `training_job_name` | String | ✅ | <p>The name of the training job. The name must be unique within an Amazon Web Services Region in an Amazon Web Services account. </p> |
| `tags` | Vec<String> |  | <p>An array of key-value pairs. You can use tags to categorize your Amazon Web Services resources in different ways, for example, by purpose, owner, or environment. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services Resources</a>.</p> <important> <p>Do not include any security-sensitive information including account access IDs, secrets, or tokens in any tags. As part of the shared responsibility model, you are responsible for any potential exposure, unauthorized access, or compromise of your sensitive data if caused by any security-sensitive information included in the request tag variable or plain text fields.</p> </important> |
| `tensor_board_output_config` | String |  |  |
| `profiler_rule_configurations` | Vec<String> |  | <p>Configuration information for Amazon SageMaker Debugger rules for profiling system and framework metrics.</p> |
| `retry_strategy` | String |  | <p>The number of times to retry the job when the job fails due to an <code>InternalServerError</code>.</p> |
| `output_data_config` | String | ✅ | <p>Specifies the path to the S3 location where you want to store model artifacts. SageMaker creates subfolders for the artifacts. </p> |
| `hyper_parameters` | HashMap<String, String> |  | <p>Algorithm-specific parameters that influence the quality of the model. You set hyperparameters before you start the learning process. For a list of hyperparameters for each training algorithm provided by SageMaker, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/algos.html">Algorithms</a>. </p> <p>You can specify a maximum of 100 hyperparameters. Each hyperparameter is a key-value pair. Each key and value is limited to 256 characters, as specified by the <code>Length Constraint</code>. </p> <important> <p>Do not include any security-sensitive information including account access IDs, secrets, or tokens in any hyperparameter fields. As part of the shared responsibility model, you are responsible for any potential exposure, unauthorized access, or compromise of your sensitive data if caused by any security-sensitive information included in the request hyperparameter variable or plain text fields.</p> </important> |
| `enable_network_isolation` | bool |  | <p>Isolates the training container. No inbound or outbound network calls can be made, except for calls between peers within a training cluster for distributed training. If you enable network isolation for training jobs that are configured to use a VPC, SageMaker downloads and uploads customer data and model artifacts through the specified VPC, but the training container does not have network access.</p> |
| `debug_hook_config` | String |  |  |
| `enable_managed_spot_training` | bool |  | <p>To train models using managed spot training, choose <code>True</code>. Managed spot training provides a fully managed and scalable infrastructure for training machine learning models. this option is useful when training jobs can be interrupted and when there is flexibility when the training job is run. </p> <p>The complete and intermediate results of jobs are stored in an Amazon S3 bucket, and can be used as a starting point to train models incrementally. Amazon SageMaker provides metrics and logs in CloudWatch. They can be used to see when managed spot training jobs are running, interrupted, resumed, or completed. </p> |
| `vpc_config` | String |  | <p>A <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_VpcConfig.html">VpcConfig</a> object that specifies the VPC that you want your training job to connect to. Control access to and from your training container by configuring the VPC. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/train-vpc.html">Protect Training Jobs by Using an Amazon Virtual Private Cloud</a>.</p> |
| `input_data_config` | Vec<String> |  | <p>An array of <code>Channel</code> objects. Each channel is a named input source. <code>InputDataConfig</code> describes the input data and its location. </p> <p>Algorithms can accept input data from one or more channels. For example, an algorithm might have two channels of input data, <code>training_data</code> and <code>validation_data</code>. The configuration for each channel provides the S3, EFS, or FSx location where the input data is stored. It also provides information about the stored data: the MIME type, compression method, and whether the data is wrapped in RecordIO format. </p> <p>Depending on the input mode that the algorithm supports, SageMaker either copies input data files from an S3 bucket to a local directory in the Docker container, or makes it available as input streams. For example, if you specify an EFS location, input data files are available as input streams. They do not need to be downloaded.</p> <p>Your input must be in the same Amazon Web Services region as your training job.</p> |
| `experiment_config` | String |  |  |
| `remote_debug_config` | String |  | <p>Configuration for remote debugging. To learn more about the remote debugging functionality of SageMaker, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/train-remote-debugging.html">Access a training container through Amazon Web Services Systems Manager (SSM) for remote debugging</a>.</p> |
| `checkpoint_config` | String |  | <p>Contains information about the output location for managed spot training checkpoint data.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `creation_time` | String | <p>A timestamp that indicates when the training job was created.</p> |
| `tuning_job_arn` | String | <p>The Amazon Resource Name (ARN) of the associated hyperparameter tuning job if the training job was launched by a hyperparameter tuning job.</p> |
| `vpc_config` | String | <p>A <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_VpcConfig.html">VpcConfig</a> object that specifies the VPC that this training job has access to. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/train-vpc.html">Protect Training Jobs by Using an Amazon Virtual Private Cloud</a>.</p> |
| `billable_time_in_seconds` | i64 | <p>The billable time in seconds. Billable time refers to the absolute wall-clock time.</p> <p>Multiply <code>BillableTimeInSeconds</code> by the number of instances (<code>InstanceCount</code>) in your training cluster to get the total compute time SageMaker bills you if you run distributed training. The formula is as follows: <code>BillableTimeInSeconds * InstanceCount</code> .</p> <p>You can calculate the savings from using managed spot training using the formula <code>(1 - BillableTimeInSeconds / TrainingTimeInSeconds) * 100</code>. For example, if <code>BillableTimeInSeconds</code> is 100 and <code>TrainingTimeInSeconds</code> is 500, the savings is 80%.</p> |
| `training_job_arn` | String | <p>The Amazon Resource Name (ARN) of the training job.</p> |
| `secondary_status` | String | <p> Provides detailed information about the state of the training job. For detailed information on the secondary status of the training job, see <code>StatusMessage</code> under <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_SecondaryStatusTransition.html">SecondaryStatusTransition</a>.</p> <p>SageMaker provides primary statuses and secondary statuses that apply to each of them:</p> <dl> <dt>InProgress</dt> <dd> <ul> <li> <p> <code>Starting</code> - Starting the training job.</p> </li> <li> <p> <code>Downloading</code> - An optional stage for algorithms that support <code>File</code> training input mode. It indicates that data is being downloaded to the ML storage volumes.</p> </li> <li> <p> <code>Training</code> - Training is in progress.</p> </li> <li> <p> <code>Interrupted</code> - The job stopped because the managed spot training instances were interrupted. </p> </li> <li> <p> <code>Uploading</code> - Training is complete and the model artifacts are being uploaded to the S3 location.</p> </li> </ul> </dd> <dt>Completed</dt> <dd> <ul> <li> <p> <code>Completed</code> - The training job has completed.</p> </li> </ul> </dd> <dt>Failed</dt> <dd> <ul> <li> <p> <code>Failed</code> - The training job has failed. The reason for the failure is returned in the <code>FailureReason</code> field of <code>DescribeTrainingJobResponse</code>.</p> </li> </ul> </dd> <dt>Stopped</dt> <dd> <ul> <li> <p> <code>MaxRuntimeExceeded</code> - The job stopped because it exceeded the maximum allowed runtime.</p> </li> <li> <p> <code>MaxWaitTimeExceeded</code> - The job stopped because it exceeded the maximum allowed wait time.</p> </li> <li> <p> <code>Stopped</code> - The training job has stopped.</p> </li> </ul> </dd> <dt>Stopping</dt> <dd> <ul> <li> <p> <code>Stopping</code> - Stopping the training job.</p> </li> </ul> </dd> </dl> <important> <p>Valid values for <code>SecondaryStatus</code> are subject to change. </p> </important> <p>We no longer support the following secondary statuses:</p> <ul> <li> <p> <code>LaunchingMLInstances</code> </p> </li> <li> <p> <code>PreparingTraining</code> </p> </li> <li> <p> <code>DownloadingTrainingImage</code> </p> </li> </ul> |
| `hyper_parameters` | HashMap<String, String> | <p>Algorithm-specific parameters. </p> |
| `profiling_status` | String | <p>Profiling status of a training job.</p> |
| `role_arn` | String | <p>The Amazon Web Services Identity and Access Management (IAM) role configured for the training job. </p> |
| `secondary_status_transitions` | Vec<String> | <p>A history of all of the secondary statuses that the training job has transitioned through.</p> |
| `resource_config` | String | <p>Resources, including ML compute instances and ML storage volumes, that are configured for model training. </p> |
| `profiler_rule_configurations` | Vec<String> | <p>Configuration information for Amazon SageMaker Debugger rules for profiling system and framework metrics.</p> |
| `stopping_condition` | String | <p>Specifies a limit to how long a model training job can run. It also specifies how long a managed Spot training job has to complete. When the job reaches the time limit, SageMaker ends the training job. Use this API to cap model training costs.</p> <p>To stop a job, SageMaker sends the algorithm the <code>SIGTERM</code> signal, which delays job termination for 120 seconds. Algorithms can use this 120-second window to save the model artifacts, so the results of training are not lost. </p> |
| `enable_managed_spot_training` | bool | <p>A Boolean indicating whether managed spot training is enabled (<code>True</code>) or not (<code>False</code>).</p> |
| `debug_rule_configurations` | Vec<String> | <p>Configuration information for Amazon SageMaker Debugger rules for debugging output tensors.</p> |
| `experiment_config` | String |  |
| `debug_rule_evaluation_statuses` | Vec<String> | <p>Evaluation status of Amazon SageMaker Debugger rules for debugging on a training job.</p> |
| `tensor_board_output_config` | String |  |
| `training_end_time` | String | <p>Indicates the time when the training job ends on training instances. You are billed for the time interval between the value of <code>TrainingStartTime</code> and this time. For successful jobs and stopped jobs, this is the time after model artifacts are uploaded. For failed jobs, this is the time when SageMaker detects a job failure.</p> |
| `checkpoint_config` | String |  |
| `input_data_config` | Vec<String> | <p>An array of <code>Channel</code> objects that describes each data input channel. </p> |
| `failure_reason` | String | <p>If the training job failed, the reason it failed. </p> |
| `training_job_name` | String | <p> Name of the model training job. </p> |
| `last_modified_time` | String | <p>A timestamp that indicates when the status of the training job was last modified.</p> |
| `training_start_time` | String | <p>Indicates the time when the training job starts on training instances. You are billed for the time interval between this time and the value of <code>TrainingEndTime</code>. The start time in CloudWatch Logs might be later than this time. The difference is due to the time it takes to download the training data and to the size of the training container.</p> |
| `enable_network_isolation` | bool | <p>If you want to allow inbound or outbound network calls, except for calls between peers within a training cluster for distributed training, choose <code>True</code>. If you enable network isolation for training jobs that are configured to use a VPC, SageMaker downloads and uploads customer data and model artifacts through the specified VPC, but the training container does not have network access.</p> |
| `enable_inter_container_traffic_encryption` | bool | <p>To encrypt all communications between ML compute instances in distributed training, choose <code>True</code>. Encryption provides greater security for distributed training, but training might take longer. How long it takes depends on the amount of communication between compute instances, especially if you use a deep learning algorithms in distributed training.</p> |
| `warm_pool_status` | String | <p>The status of the warm pool associated with the training job.</p> |
| `debug_hook_config` | String |  |
| `profiler_config` | String |  |
| `algorithm_specification` | String | <p>Information about the algorithm used for training, and algorithm metadata. </p> |
| `final_metric_data_list` | Vec<String> | <p>A collection of <code>MetricData</code> objects that specify the names, values, and dates and times that the training algorithm emitted to Amazon CloudWatch.</p> |
| `retry_strategy` | String | <p>The number of times to retry the job when the job fails due to an <code>InternalServerError</code>.</p> |
| `remote_debug_config` | String | <p>Configuration for remote debugging. To learn more about the remote debugging functionality of SageMaker, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/train-remote-debugging.html">Access a training container through Amazon Web Services Systems Manager (SSM) for remote debugging</a>.</p> |
| `infra_check_config` | String | <p>Contains information about the infrastructure health check configuration for the training job.</p> |
| `environment` | HashMap<String, String> | <p>The environment variables to set in the Docker container.</p> <important> <p>Do not include any security-sensitive information including account access IDs, secrets, or tokens in any environment fields. As part of the shared responsibility model, you are responsible for any potential exposure, unauthorized access, or compromise of your sensitive data if caused by security-sensitive information included in the request environment variable or plain text fields.</p> </important> |
| `labeling_job_arn` | String | <p>The Amazon Resource Name (ARN) of the SageMaker Ground Truth labeling job that created the transform or training job.</p> |
| `auto_ml_job_arn` | String | <p>The Amazon Resource Name (ARN) of an AutoML job.</p> |
| `training_job_status` | String | <p>The status of the training job.</p> <p>SageMaker provides the following training job statuses:</p> <ul> <li> <p> <code>InProgress</code> - The training is in progress.</p> </li> <li> <p> <code>Completed</code> - The training job has completed.</p> </li> <li> <p> <code>Failed</code> - The training job has failed. To see the reason for the failure, see the <code>FailureReason</code> field in the response to a <code>DescribeTrainingJobResponse</code> call.</p> </li> <li> <p> <code>Stopping</code> - The training job is stopping.</p> </li> <li> <p> <code>Stopped</code> - The training job has stopped.</p> </li> </ul> <p>For more detailed information, see <code>SecondaryStatus</code>. </p> |
| `model_artifacts` | String | <p>Information about the Amazon S3 location that is configured for storing model artifacts. </p> |
| `output_data_config` | String | <p>The S3 path where model artifacts that you configured when creating the job are stored. SageMaker creates subfolders for model artifacts. </p> |
| `profiler_rule_evaluation_statuses` | Vec<String> | <p>Evaluation status of Amazon SageMaker Debugger rules for profiling on a training job.</p> |
| `training_time_in_seconds` | i64 | <p>The training time in seconds.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create training_job
training_job = provider.sagemaker.Training_job {
    stopping_condition = "value"  # <p>Specifies a limit to how long a model training job can run. It also specifies how long a managed Spot training job has to complete. When the job reaches the time limit, SageMaker ends the training job. Use this API to cap model training costs.</p> <p>To stop a job, SageMaker sends the algorithm the <code>SIGTERM</code> signal, which delays job termination for 120 seconds. Algorithms can use this 120-second window to save the model artifacts, so the results of training are not lost. </p>
    role_arn = "value"  # <p>The Amazon Resource Name (ARN) of an IAM role that SageMaker can assume to perform tasks on your behalf. </p> <p>During model training, SageMaker needs your permission to read input data from an S3 bucket, download a Docker image that contains training code, write model artifacts to an S3 bucket, write logs to Amazon CloudWatch Logs, and publish metrics to Amazon CloudWatch. You grant permissions for all of these tasks to an IAM role. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/sagemaker-roles.html">SageMaker Roles</a>. </p> <note> <p>To be able to pass this role to SageMaker, the caller of this API must have the <code>iam:PassRole</code> permission.</p> </note>
    resource_config = "value"  # <p>The resources, including the ML compute instances and ML storage volumes, to use for model training. </p> <p>ML storage volumes store model artifacts and incremental states. Training algorithms might also use ML storage volumes for scratch space. If you want SageMaker to use the ML storage volume to store the training data, choose <code>File</code> as the <code>TrainingInputMode</code> in the algorithm specification. For distributed training algorithms, specify an instance count greater than 1.</p>
    algorithm_specification = "value"  # <p>The registry path of the Docker image that contains the training algorithm and algorithm-specific metadata, including the input mode. For more information about algorithms provided by SageMaker, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/algos.html">Algorithms</a>. For information about providing your own algorithms, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/your-algorithms.html">Using Your Own Algorithms with Amazon SageMaker</a>. </p>
    training_job_name = "value"  # <p>The name of the training job. The name must be unique within an Amazon Web Services Region in an Amazon Web Services account. </p>
    output_data_config = "value"  # <p>Specifies the path to the S3 location where you want to store model artifacts. SageMaker creates subfolders for the artifacts. </p>
}

# Access training_job outputs
training_job_id = training_job.id
training_job_creation_time = training_job.creation_time
training_job_tuning_job_arn = training_job.tuning_job_arn
training_job_vpc_config = training_job.vpc_config
training_job_billable_time_in_seconds = training_job.billable_time_in_seconds
training_job_training_job_arn = training_job.training_job_arn
training_job_secondary_status = training_job.secondary_status
training_job_hyper_parameters = training_job.hyper_parameters
training_job_profiling_status = training_job.profiling_status
training_job_role_arn = training_job.role_arn
training_job_secondary_status_transitions = training_job.secondary_status_transitions
training_job_resource_config = training_job.resource_config
training_job_profiler_rule_configurations = training_job.profiler_rule_configurations
training_job_stopping_condition = training_job.stopping_condition
training_job_enable_managed_spot_training = training_job.enable_managed_spot_training
training_job_debug_rule_configurations = training_job.debug_rule_configurations
training_job_experiment_config = training_job.experiment_config
training_job_debug_rule_evaluation_statuses = training_job.debug_rule_evaluation_statuses
training_job_tensor_board_output_config = training_job.tensor_board_output_config
training_job_training_end_time = training_job.training_end_time
training_job_checkpoint_config = training_job.checkpoint_config
training_job_input_data_config = training_job.input_data_config
training_job_failure_reason = training_job.failure_reason
training_job_training_job_name = training_job.training_job_name
training_job_last_modified_time = training_job.last_modified_time
training_job_training_start_time = training_job.training_start_time
training_job_enable_network_isolation = training_job.enable_network_isolation
training_job_enable_inter_container_traffic_encryption = training_job.enable_inter_container_traffic_encryption
training_job_warm_pool_status = training_job.warm_pool_status
training_job_debug_hook_config = training_job.debug_hook_config
training_job_profiler_config = training_job.profiler_config
training_job_algorithm_specification = training_job.algorithm_specification
training_job_final_metric_data_list = training_job.final_metric_data_list
training_job_retry_strategy = training_job.retry_strategy
training_job_remote_debug_config = training_job.remote_debug_config
training_job_infra_check_config = training_job.infra_check_config
training_job_environment = training_job.environment
training_job_labeling_job_arn = training_job.labeling_job_arn
training_job_auto_ml_job_arn = training_job.auto_ml_job_arn
training_job_training_job_status = training_job.training_job_status
training_job_model_artifacts = training_job.model_artifacts
training_job_output_data_config = training_job.output_data_config
training_job_profiler_rule_evaluation_statuses = training_job.profiler_rule_evaluation_statuses
training_job_training_time_in_seconds = training_job.training_time_in_seconds
```

---


### Model_package

ModelPackage resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `skip_model_validation` | String |  | <p>Indicates if you want to skip model validation.</p> |
| `model_package_group_name` | String |  | <p>The name or Amazon Resource Name (ARN) of the model package group that this model version belongs to.</p> <p>This parameter is required for versioned models, and does not apply to unversioned models.</p> |
| `task` | String |  | <p>The machine learning task your model package accomplishes. Common machine learning tasks include object detection and image classification. The following tasks are supported by Inference Recommender: <code>"IMAGE_CLASSIFICATION"</code> | <code>"OBJECT_DETECTION"</code> | <code>"TEXT_GENERATION"</code> |<code>"IMAGE_SEGMENTATION"</code> | <code>"FILL_MASK"</code> | <code>"CLASSIFICATION"</code> | <code>"REGRESSION"</code> | <code>"OTHER"</code>.</p> <p>Specify "OTHER" if none of the tasks listed fit your use case.</p> |
| `security_config` | String |  | <p>The KMS Key ID (<code>KMSKeyId</code>) used for encryption of model package information.</p> |
| `domain` | String |  | <p>The machine learning domain of your model package and its components. Common machine learning domains include computer vision and natural language processing.</p> |
| `model_package_name` | String |  | <p>The name of the model package. The name must have 1 to 63 characters. Valid characters are a-z, A-Z, 0-9, and - (hyphen).</p> <p>This parameter is required for unversioned models. It is not applicable to versioned models.</p> |
| `metadata_properties` | String |  |  |
| `model_metrics` | String |  | <p>A structure that contains model metrics reports.</p> |
| `model_life_cycle` | String |  | <p> A structure describing the current state of the model in its life cycle. </p> |
| `certify_for_marketplace` | bool |  | <p>Whether to certify the model package for listing on Amazon Web Services Marketplace.</p> <p>This parameter is optional for unversioned models, and does not apply to versioned models.</p> |
| `source_algorithm_specification` | String |  | <p>Details about the algorithm that was used to create the model package.</p> |
| `model_card` | String |  | <p>The model card associated with the model package. Since <code>ModelPackageModelCard</code> is tied to a model package, it is a specific usage of a model card and its schema is simplified compared to the schema of <code>ModelCard</code>. The <code>ModelPackageModelCard</code> schema does not include <code>model_package_details</code>, and <code>model_overview</code> is composed of the <code>model_creator</code> and <code>model_artifact</code> properties. For more information about the model package model card schema, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/model-registry-details.html#model-card-schema">Model package model card schema</a>. For more information about the model card associated with the model package, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/model-registry-details.html">View the Details of a Model Version</a>.</p> |
| `drift_check_baselines` | String |  | <p>Represents the drift check baselines that can be used when the model monitor is set using the model package. For more information, see the topic on <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/pipelines-quality-clarify-baseline-lifecycle.html#pipelines-quality-clarify-baseline-drift-detection">Drift Detection against Previous Baselines in SageMaker Pipelines</a> in the <i>Amazon SageMaker Developer Guide</i>. </p> |
| `model_package_description` | String |  | <p>A description of the model package.</p> |
| `validation_specification` | String |  | <p>Specifies configurations for one or more transform jobs that SageMaker runs to test the model package.</p> |
| `inference_specification` | String |  | <p>Specifies details about inference jobs that you can run with models based on this model package, including the following information:</p> <ul> <li> <p>The Amazon ECR paths of containers that contain the inference code and model artifacts.</p> </li> <li> <p>The instance types that the model package supports for transform jobs and real-time endpoints used for inference.</p> </li> <li> <p>The input and output content formats that the model package supports for inference.</p> </li> </ul> |
| `additional_inference_specifications` | Vec<String> |  | <p>An array of additional Inference Specification objects. Each additional Inference Specification specifies artifacts based on this model package that can be used on inference endpoints. Generally used with SageMaker Neo to store the compiled artifacts. </p> |
| `model_approval_status` | String |  | <p>Whether the model is approved for deployment.</p> <p>This parameter is optional for versioned models, and does not apply to unversioned models.</p> <p>For versioned models, the value of this parameter must be set to <code>Approved</code> to deploy the model.</p> |
| `sample_payload_url` | String |  | <p>The Amazon Simple Storage Service (Amazon S3) path where the sample payload is stored. This path must point to a single gzip compressed tar archive (.tar.gz suffix). This archive can hold multiple files that are all equally used in the load test. Each file in the archive must satisfy the size constraints of the <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_runtime_InvokeEndpoint.html#API_runtime_InvokeEndpoint_RequestSyntax">InvokeEndpoint</a> call.</p> |
| `client_token` | String |  | <p>A unique token that guarantees that the call to this API is idempotent.</p> |
| `customer_metadata_properties` | HashMap<String, String> |  | <p>The metadata properties associated with the model package versions.</p> |
| `source_uri` | String |  | <p>The URI of the source for the model package. If you want to clone a model package, set it to the model package Amazon Resource Name (ARN). If you want to register a model, set it to the model ARN.</p> |
| `tags` | Vec<String> |  | <p>A list of key value pairs associated with the model. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services resources</a> in the <i>Amazon Web Services General Reference Guide</i>.</p> <p>If you supply <code>ModelPackageGroupName</code>, your model package belongs to the model group you specify and uses the tags associated with the model group. In this case, you cannot supply a <code>tag</code> argument. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `model_life_cycle` | String | <p> A structure describing the current state of the model in its life cycle. </p> |
| `model_package_description` | String | <p>A brief summary of the model package.</p> |
| `model_package_group_name` | String | <p>If the model is a versioned model, the name of the model group that the versioned model belongs to.</p> |
| `task` | String | <p>The machine learning task you specified that your model package accomplishes. Common machine learning tasks include object detection and image classification.</p> |
| `source_algorithm_specification` | String | <p>Details about the algorithm that was used to create the model package.</p> |
| `model_package_status_details` | String | <p>Details about the current status of the model package.</p> |
| `additional_inference_specifications` | Vec<String> | <p>An array of additional Inference Specification objects. Each additional Inference Specification specifies artifacts based on this model package that can be used on inference endpoints. Generally used with SageMaker Neo to store the compiled artifacts.</p> |
| `drift_check_baselines` | String | <p>Represents the drift check baselines that can be used when the model monitor is set using the model package. For more information, see the topic on <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/pipelines-quality-clarify-baseline-lifecycle.html#pipelines-quality-clarify-baseline-drift-detection">Drift Detection against Previous Baselines in SageMaker Pipelines</a> in the <i>Amazon SageMaker Developer Guide</i>. </p> |
| `approval_description` | String | <p>A description provided for the model approval.</p> |
| `model_card` | String | <p>The model card associated with the model package. Since <code>ModelPackageModelCard</code> is tied to a model package, it is a specific usage of a model card and its schema is simplified compared to the schema of <code>ModelCard</code>. The <code>ModelPackageModelCard</code> schema does not include <code>model_package_details</code>, and <code>model_overview</code> is composed of the <code>model_creator</code> and <code>model_artifact</code> properties. For more information about the model package model card schema, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/model-registry-details.html#model-card-schema">Model package model card schema</a>. For more information about the model card associated with the model package, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/model-registry-details.html">View the Details of a Model Version</a>.</p> |
| `inference_specification` | String | <p>Details about inference jobs that you can run with models based on this model package.</p> |
| `model_package_status` | String | <p>The current status of the model package.</p> |
| `model_package_arn` | String | <p>The Amazon Resource Name (ARN) of the model package.</p> |
| `model_package_version` | i64 | <p>The version of the model package.</p> |
| `creation_time` | String | <p>A timestamp specifying when the model package was created.</p> |
| `certify_for_marketplace` | bool | <p>Whether the model package is certified for listing on Amazon Web Services Marketplace.</p> |
| `validation_specification` | String | <p>Configurations for one or more transform jobs that SageMaker runs to test the model package.</p> |
| `model_metrics` | String | <p>Metrics for the model.</p> |
| `last_modified_by` | String |  |
| `skip_model_validation` | String | <p>Indicates if you want to skip model validation.</p> |
| `security_config` | String | <p>The KMS Key ID (<code>KMSKeyId</code>) used for encryption of model package information.</p> |
| `source_uri` | String | <p>The URI of the source for the model package.</p> |
| `metadata_properties` | String |  |
| `domain` | String | <p>The machine learning domain of the model package you specified. Common machine learning domains include computer vision and natural language processing.</p> |
| `created_by` | String |  |
| `model_approval_status` | String | <p>The approval status of the model package.</p> |
| `sample_payload_url` | String | <p>The Amazon Simple Storage Service (Amazon S3) path where the sample payload are stored. This path points to a single gzip compressed tar archive (.tar.gz suffix).</p> |
| `customer_metadata_properties` | HashMap<String, String> | <p>The metadata properties associated with the model package versions.</p> |
| `last_modified_time` | String | <p>The last time that the model package was modified.</p> |
| `model_package_name` | String | <p>The name of the model package being described.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create model_package
model_package = provider.sagemaker.Model_package {
}

# Access model_package outputs
model_package_id = model_package.id
model_package_model_life_cycle = model_package.model_life_cycle
model_package_model_package_description = model_package.model_package_description
model_package_model_package_group_name = model_package.model_package_group_name
model_package_task = model_package.task
model_package_source_algorithm_specification = model_package.source_algorithm_specification
model_package_model_package_status_details = model_package.model_package_status_details
model_package_additional_inference_specifications = model_package.additional_inference_specifications
model_package_drift_check_baselines = model_package.drift_check_baselines
model_package_approval_description = model_package.approval_description
model_package_model_card = model_package.model_card
model_package_inference_specification = model_package.inference_specification
model_package_model_package_status = model_package.model_package_status
model_package_model_package_arn = model_package.model_package_arn
model_package_model_package_version = model_package.model_package_version
model_package_creation_time = model_package.creation_time
model_package_certify_for_marketplace = model_package.certify_for_marketplace
model_package_validation_specification = model_package.validation_specification
model_package_model_metrics = model_package.model_metrics
model_package_last_modified_by = model_package.last_modified_by
model_package_skip_model_validation = model_package.skip_model_validation
model_package_security_config = model_package.security_config
model_package_source_uri = model_package.source_uri
model_package_metadata_properties = model_package.metadata_properties
model_package_domain = model_package.domain
model_package_created_by = model_package.created_by
model_package_model_approval_status = model_package.model_approval_status
model_package_sample_payload_url = model_package.sample_payload_url
model_package_customer_metadata_properties = model_package.customer_metadata_properties
model_package_last_modified_time = model_package.last_modified_time
model_package_model_package_name = model_package.model_package_name
```

---


### Auto_ml_job_v2

AutoMLJobV2 resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `output_data_config` | String | ✅ | <p>Provides information about encryption and the Amazon S3 output path needed to store artifacts from an AutoML job.</p> |
| `auto_ml_job_objective` | String |  | <p>Specifies a metric to minimize or maximize as the objective of a job. If not specified, the default objective metric depends on the problem type. For the list of default values per problem type, see <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_AutoMLJobObjective.html">AutoMLJobObjective</a>.</p> <note> <ul> <li> <p>For tabular problem types: You must either provide both the <code>AutoMLJobObjective</code> and indicate the type of supervised learning problem in <code>AutoMLProblemTypeConfig</code> (<code>TabularJobConfig.ProblemType</code>), or none at all.</p> </li> <li> <p>For text generation problem types (LLMs fine-tuning): Fine-tuning language models in Autopilot does not require setting the <code>AutoMLJobObjective</code> field. Autopilot fine-tunes LLMs without requiring multiple candidates to be trained and evaluated. Instead, using your dataset, Autopilot directly fine-tunes your target model to enhance a default objective metric, the cross-entropy loss. After fine-tuning a language model, you can evaluate the quality of its generated text using different metrics. For a list of the available metrics, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/autopilot-llms-finetuning-metrics.html">Metrics for fine-tuning LLMs in Autopilot</a>.</p> </li> </ul> </note> |
| `tags` | Vec<String> |  | <p>An array of key-value pairs. You can use tags to categorize your Amazon Web Services resources in different ways, such as by purpose, owner, or environment. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web ServicesResources</a>. Tag keys must be unique per resource.</p> |
| `model_deploy_config` | String |  | <p>Specifies how to generate the endpoint name for an automatic one-click Autopilot model deployment.</p> |
| `role_arn` | String | ✅ | <p>The ARN of the role that is used to access the data.</p> |
| `auto_ml_job_input_data_config` | Vec<String> | ✅ | <p>An array of channel objects describing the input data and their location. Each channel is a named input source. Similar to the <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_CreateAutoMLJob.html#sagemaker-CreateAutoMLJob-request-InputDataConfig">InputDataConfig</a> attribute in the <code>CreateAutoMLJob</code> input parameters. The supported formats depend on the problem type:</p> <ul> <li> <p>For tabular problem types: <code>S3Prefix</code>, <code>ManifestFile</code>.</p> </li> <li> <p>For image classification: <code>S3Prefix</code>, <code>ManifestFile</code>, <code>AugmentedManifestFile</code>.</p> </li> <li> <p>For text classification: <code>S3Prefix</code>.</p> </li> <li> <p>For time-series forecasting: <code>S3Prefix</code>.</p> </li> <li> <p>For text generation (LLMs fine-tuning): <code>S3Prefix</code>.</p> </li> </ul> |
| `auto_ml_problem_type_config` | String | ✅ | <p>Defines the configuration settings of one of the supported problem types.</p> |
| `security_config` | String |  | <p>The security configuration for traffic encryption or Amazon VPC settings.</p> |
| `auto_ml_job_name` | String | ✅ | <p>Identifies an Autopilot job. The name must be unique to your account and is case insensitive.</p> |
| `auto_ml_compute_config` | String |  | <p>Specifies the compute configuration for the AutoML job V2.</p> |
| `data_split_config` | String |  | <p>This structure specifies how to split the data into train and validation datasets.</p> <p>The validation and training datasets must contain the same headers. For jobs created by calling <code>CreateAutoMLJob</code>, the validation dataset must be less than 2 GB in size.</p> <note> <p>This attribute must not be set for the time-series forecasting problem type, as Autopilot automatically splits the input dataset into training and validation sets.</p> </note> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `model_deploy_result` | String | <p>Provides information about endpoint for the model deployment.</p> |
| `last_modified_time` | String | <p>Returns the job's last modified time.</p> |
| `auto_ml_job_objective` | String | <p>Returns the job's objective.</p> |
| `creation_time` | String | <p>Returns the creation time of the AutoML job V2.</p> |
| `auto_ml_problem_type_config_name` | String | <p>Returns the name of the problem type configuration set for the AutoML job V2.</p> |
| `auto_ml_job_status` | String | <p>Returns the status of the AutoML job V2.</p> |
| `output_data_config` | String | <p>Returns the job's output data config.</p> |
| `model_deploy_config` | String | <p>Indicates whether the model was deployed automatically to an endpoint and the name of that endpoint if deployed automatically.</p> |
| `auto_ml_problem_type_config` | String | <p>Returns the configuration settings of the problem type set for the AutoML job V2.</p> |
| `auto_ml_compute_config` | String | <p>The compute configuration used for the AutoML job V2.</p> |
| `partial_failure_reasons` | Vec<String> | <p>Returns a list of reasons for partial failures within an AutoML job V2.</p> |
| `security_config` | String | <p>Returns the security configuration for traffic encryption or Amazon VPC settings.</p> |
| `data_split_config` | String | <p>Returns the configuration settings of how the data are split into train and validation datasets.</p> |
| `auto_ml_job_secondary_status` | String | <p>Returns the secondary status of the AutoML job V2.</p> |
| `role_arn` | String | <p>The ARN of the IAM role that has read permission to the input data location and write permission to the output data location in Amazon S3.</p> |
| `best_candidate` | String | <p>Information about the candidate produced by an AutoML training job V2, including its status, steps, and other properties.</p> |
| `auto_ml_job_name` | String | <p>Returns the name of the AutoML job V2.</p> |
| `failure_reason` | String | <p>Returns the reason for the failure of the AutoML job V2, when applicable.</p> |
| `resolved_attributes` | String | <p>Returns the resolved attributes used by the AutoML job V2.</p> |
| `end_time` | String | <p>Returns the end time of the AutoML job V2.</p> |
| `auto_ml_job_arn` | String | <p>Returns the Amazon Resource Name (ARN) of the AutoML job V2.</p> |
| `auto_ml_job_input_data_config` | Vec<String> | <p>Returns an array of channel objects describing the input data and their location.</p> |
| `auto_ml_job_artifacts` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create auto_ml_job_v2
auto_ml_job_v2 = provider.sagemaker.Auto_ml_job_v2 {
    output_data_config = "value"  # <p>Provides information about encryption and the Amazon S3 output path needed to store artifacts from an AutoML job.</p>
    role_arn = "value"  # <p>The ARN of the role that is used to access the data.</p>
    auto_ml_job_input_data_config = "value"  # <p>An array of channel objects describing the input data and their location. Each channel is a named input source. Similar to the <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_CreateAutoMLJob.html#sagemaker-CreateAutoMLJob-request-InputDataConfig">InputDataConfig</a> attribute in the <code>CreateAutoMLJob</code> input parameters. The supported formats depend on the problem type:</p> <ul> <li> <p>For tabular problem types: <code>S3Prefix</code>, <code>ManifestFile</code>.</p> </li> <li> <p>For image classification: <code>S3Prefix</code>, <code>ManifestFile</code>, <code>AugmentedManifestFile</code>.</p> </li> <li> <p>For text classification: <code>S3Prefix</code>.</p> </li> <li> <p>For time-series forecasting: <code>S3Prefix</code>.</p> </li> <li> <p>For text generation (LLMs fine-tuning): <code>S3Prefix</code>.</p> </li> </ul>
    auto_ml_problem_type_config = "value"  # <p>Defines the configuration settings of one of the supported problem types.</p>
    auto_ml_job_name = "value"  # <p>Identifies an Autopilot job. The name must be unique to your account and is case insensitive.</p>
}

# Access auto_ml_job_v2 outputs
auto_ml_job_v2_id = auto_ml_job_v2.id
auto_ml_job_v2_model_deploy_result = auto_ml_job_v2.model_deploy_result
auto_ml_job_v2_last_modified_time = auto_ml_job_v2.last_modified_time
auto_ml_job_v2_auto_ml_job_objective = auto_ml_job_v2.auto_ml_job_objective
auto_ml_job_v2_creation_time = auto_ml_job_v2.creation_time
auto_ml_job_v2_auto_ml_problem_type_config_name = auto_ml_job_v2.auto_ml_problem_type_config_name
auto_ml_job_v2_auto_ml_job_status = auto_ml_job_v2.auto_ml_job_status
auto_ml_job_v2_output_data_config = auto_ml_job_v2.output_data_config
auto_ml_job_v2_model_deploy_config = auto_ml_job_v2.model_deploy_config
auto_ml_job_v2_auto_ml_problem_type_config = auto_ml_job_v2.auto_ml_problem_type_config
auto_ml_job_v2_auto_ml_compute_config = auto_ml_job_v2.auto_ml_compute_config
auto_ml_job_v2_partial_failure_reasons = auto_ml_job_v2.partial_failure_reasons
auto_ml_job_v2_security_config = auto_ml_job_v2.security_config
auto_ml_job_v2_data_split_config = auto_ml_job_v2.data_split_config
auto_ml_job_v2_auto_ml_job_secondary_status = auto_ml_job_v2.auto_ml_job_secondary_status
auto_ml_job_v2_role_arn = auto_ml_job_v2.role_arn
auto_ml_job_v2_best_candidate = auto_ml_job_v2.best_candidate
auto_ml_job_v2_auto_ml_job_name = auto_ml_job_v2.auto_ml_job_name
auto_ml_job_v2_failure_reason = auto_ml_job_v2.failure_reason
auto_ml_job_v2_resolved_attributes = auto_ml_job_v2.resolved_attributes
auto_ml_job_v2_end_time = auto_ml_job_v2.end_time
auto_ml_job_v2_auto_ml_job_arn = auto_ml_job_v2.auto_ml_job_arn
auto_ml_job_v2_auto_ml_job_input_data_config = auto_ml_job_v2.auto_ml_job_input_data_config
auto_ml_job_v2_auto_ml_job_artifacts = auto_ml_job_v2.auto_ml_job_artifacts
```

---


### Trial

Trial resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `experiment_name` | String | ✅ | <p>The name of the experiment to associate the trial with.</p> |
| `metadata_properties` | String |  |  |
| `tags` | Vec<String> |  | <p>A list of tags to associate with the trial. You can use <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_Search.html">Search</a> API to search on the tags.</p> |
| `trial_name` | String | ✅ | <p>The name of the trial. The name must be unique in your Amazon Web Services account and is not case-sensitive.</p> |
| `display_name` | String |  | <p>The name of the trial as displayed. The name doesn't need to be unique. If <code>DisplayName</code> isn't specified, <code>TrialName</code> is displayed.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `source` | String | <p>The Amazon Resource Name (ARN) of the source and, optionally, the job type.</p> |
| `display_name` | String | <p>The name of the trial as displayed. If <code>DisplayName</code> isn't specified, <code>TrialName</code> is displayed.</p> |
| `metadata_properties` | String |  |
| `trial_name` | String | <p>The name of the trial.</p> |
| `trial_arn` | String | <p>The Amazon Resource Name (ARN) of the trial.</p> |
| `experiment_name` | String | <p>The name of the experiment the trial is part of.</p> |
| `creation_time` | String | <p>When the trial was created.</p> |
| `last_modified_time` | String | <p>When the trial was last modified.</p> |
| `created_by` | String | <p>Who created the trial.</p> |
| `last_modified_by` | String | <p>Who last modified the trial.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create trial
trial = provider.sagemaker.Trial {
    experiment_name = "value"  # <p>The name of the experiment to associate the trial with.</p>
    trial_name = "value"  # <p>The name of the trial. The name must be unique in your Amazon Web Services account and is not case-sensitive.</p>
}

# Access trial outputs
trial_id = trial.id
trial_source = trial.source
trial_display_name = trial.display_name
trial_metadata_properties = trial.metadata_properties
trial_trial_name = trial.trial_name
trial_trial_arn = trial.trial_arn
trial_experiment_name = trial.experiment_name
trial_creation_time = trial.creation_time
trial_last_modified_time = trial.last_modified_time
trial_created_by = trial.created_by
trial_last_modified_by = trial.last_modified_by
```

---


### Search_suggestions

SearchSuggestions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `property_name_suggestions` | Vec<String> | <p>A list of property names for a <code>Resource</code> that match a <code>SuggestionQuery</code>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access search_suggestions outputs
search_suggestions_id = search_suggestions.id
search_suggestions_property_name_suggestions = search_suggestions.property_name_suggestions
```

---


### Presigned_notebook_instance_url

PresignedNotebookInstanceUrl resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `session_expiration_duration_in_seconds` | i64 |  | <p>The duration of the session, in seconds. The default is 12 hours.</p> |
| `notebook_instance_name` | String | ✅ | <p>The name of the notebook instance.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create presigned_notebook_instance_url
presigned_notebook_instance_url = provider.sagemaker.Presigned_notebook_instance_url {
    notebook_instance_name = "value"  # <p>The name of the notebook instance.</p>
}

```

---


### Image

Image resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | <p>The display name of the image. If not provided, <code>ImageName</code> is displayed.</p> |
| `role_arn` | String | ✅ | <p>The ARN of an IAM role that enables Amazon SageMaker AI to perform tasks on your behalf.</p> |
| `description` | String |  | <p>The description of the image.</p> |
| `tags` | Vec<String> |  | <p>A list of tags to apply to the image.</p> |
| `image_name` | String | ✅ | <p>The name of the image. Must be unique to your account.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | <p>The name of the image as displayed.</p> |
| `image_arn` | String | <p>The ARN of the image.</p> |
| `last_modified_time` | String | <p>When the image was last modified.</p> |
| `description` | String | <p>The description of the image.</p> |
| `image_status` | String | <p>The status of the image.</p> |
| `role_arn` | String | <p>The ARN of the IAM role that enables Amazon SageMaker AI to perform tasks on your behalf.</p> |
| `creation_time` | String | <p>When the image was created.</p> |
| `image_name` | String | <p>The name of the image.</p> |
| `failure_reason` | String | <p>When a create, update, or delete operation fails, the reason for the failure.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create image
image = provider.sagemaker.Image {
    role_arn = "value"  # <p>The ARN of an IAM role that enables Amazon SageMaker AI to perform tasks on your behalf.</p>
    image_name = "value"  # <p>The name of the image. Must be unique to your account.</p>
}

# Access image outputs
image_id = image.id
image_display_name = image.display_name
image_image_arn = image.image_arn
image_last_modified_time = image.last_modified_time
image_description = image.description
image_image_status = image.image_status
image_role_arn = image.role_arn
image_creation_time = image.creation_time
image_image_name = image.image_name
image_failure_reason = image.failure_reason
```

---


### Code_repository

CodeRepository resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>An array of key-value pairs. You can use tags to categorize your Amazon Web Services resources in different ways, for example, by purpose, owner, or environment. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services Resources</a>.</p> |
| `code_repository_name` | String | ✅ | <p>The name of the Git repository. The name must have 1 to 63 characters. Valid characters are a-z, A-Z, 0-9, and - (hyphen).</p> |
| `git_config` | String | ✅ | <p>Specifies details about the repository, including the URL where the repository is located, the default branch, and credentials to use to access the repository.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `last_modified_time` | String | <p>The date and time that the repository was last changed.</p> |
| `git_config` | String | <p>Configuration details about the repository, including the URL where the repository is located, the default branch, and the Amazon Resource Name (ARN) of the Amazon Web Services Secrets Manager secret that contains the credentials used to access the repository.</p> |
| `code_repository_arn` | String | <p>The Amazon Resource Name (ARN) of the Git repository.</p> |
| `code_repository_name` | String | <p>The name of the Git repository.</p> |
| `creation_time` | String | <p>The date and time that the repository was created.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create code_repository
code_repository = provider.sagemaker.Code_repository {
    code_repository_name = "value"  # <p>The name of the Git repository. The name must have 1 to 63 characters. Valid characters are a-z, A-Z, 0-9, and - (hyphen).</p>
    git_config = "value"  # <p>Specifies details about the repository, including the URL where the repository is located, the default branch, and credentials to use to access the repository.</p>
}

# Access code_repository outputs
code_repository_id = code_repository.id
code_repository_last_modified_time = code_repository.last_modified_time
code_repository_git_config = code_repository.git_config
code_repository_code_repository_arn = code_repository.code_repository_arn
code_repository_code_repository_name = code_repository.code_repository_name
code_repository_creation_time = code_repository.creation_time
```

---


### Feature_group

FeatureGroup resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `event_time_feature_name` | String | ✅ | <p>The name of the feature that stores the <code>EventTime</code> of a <code>Record</code> in a <code>FeatureGroup</code>.</p> <p>An <code>EventTime</code> is a point in time when a new event occurs that corresponds to the creation or update of a <code>Record</code> in a <code>FeatureGroup</code>. All <code>Records</code> in the <code>FeatureGroup</code> must have a corresponding <code>EventTime</code>.</p> <p>An <code>EventTime</code> can be a <code>String</code> or <code>Fractional</code>. </p> <ul> <li> <p> <code>Fractional</code>: <code>EventTime</code> feature values must be a Unix timestamp in seconds.</p> </li> <li> <p> <code>String</code>: <code>EventTime</code> feature values must be an ISO-8601 string in the format. The following formats are supported <code>yyyy-MM-dd'T'HH:mm:ssZ</code> and <code>yyyy-MM-dd'T'HH:mm:ss.SSSZ</code> where <code>yyyy</code>, <code>MM</code>, and <code>dd</code> represent the year, month, and day respectively and <code>HH</code>, <code>mm</code>, <code>ss</code>, and if applicable, <code>SSS</code> represent the hour, month, second and milliseconds respsectively. <code>'T'</code> and <code>Z</code> are constants.</p> </li> </ul> |
| `tags` | Vec<String> |  | <p>Tags used to identify <code>Features</code> in each <code>FeatureGroup</code>.</p> |
| `feature_definitions` | Vec<String> | ✅ | <p>A list of <code>Feature</code> names and types. <code>Name</code> and <code>Type</code> is compulsory per <code>Feature</code>. </p> <p>Valid feature <code>FeatureType</code>s are <code>Integral</code>, <code>Fractional</code> and <code>String</code>.</p> <p> <code>FeatureName</code>s cannot be any of the following: <code>is_deleted</code>, <code>write_time</code>, <code>api_invocation_time</code> </p> <p>You can create up to 2,500 <code>FeatureDefinition</code>s per <code>FeatureGroup</code>.</p> |
| `online_store_config` | String |  | <p>You can turn the <code>OnlineStore</code> on or off by specifying <code>True</code> for the <code>EnableOnlineStore</code> flag in <code>OnlineStoreConfig</code>.</p> <p>You can also include an Amazon Web Services KMS key ID (<code>KMSKeyId</code>) for at-rest encryption of the <code>OnlineStore</code>.</p> <p>The default value is <code>False</code>.</p> |
| `description` | String |  | <p>A free-form description of a <code>FeatureGroup</code>.</p> |
| `feature_group_name` | String | ✅ | <p>The name of the <code>FeatureGroup</code>. The name must be unique within an Amazon Web Services Region in an Amazon Web Services account.</p> <p>The name:</p> <ul> <li> <p>Must start with an alphanumeric character.</p> </li> <li> <p>Can only include alphanumeric characters, underscores, and hyphens. Spaces are not allowed.</p> </li> </ul> |
| `throughput_config` | String |  |  |
| `offline_store_config` | String |  | <p>Use this to configure an <code>OfflineFeatureStore</code>. This parameter allows you to specify:</p> <ul> <li> <p>The Amazon Simple Storage Service (Amazon S3) location of an <code>OfflineStore</code>.</p> </li> <li> <p>A configuration for an Amazon Web Services Glue or Amazon Web Services Hive data catalog. </p> </li> <li> <p>An KMS encryption key to encrypt the Amazon S3 location used for <code>OfflineStore</code>. If KMS encryption key is not specified, by default we encrypt all data at rest using Amazon Web Services KMS key. By defining your <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/bucket-key.html">bucket-level key</a> for SSE, you can reduce Amazon Web Services KMS requests costs by up to 99 percent.</p> </li> <li> <p>Format for the offline store table. Supported formats are Glue (Default) and <a href="https://iceberg.apache.org/">Apache Iceberg</a>.</p> </li> </ul> <p>To learn more about this parameter, see <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_OfflineStoreConfig.html">OfflineStoreConfig</a>.</p> |
| `record_identifier_feature_name` | String | ✅ | <p>The name of the <code>Feature</code> whose value uniquely identifies a <code>Record</code> defined in the <code>FeatureStore</code>. Only the latest record per identifier value will be stored in the <code>OnlineStore</code>. <code>RecordIdentifierFeatureName</code> must be one of feature definitions' names.</p> <p>You use the <code>RecordIdentifierFeatureName</code> to access data in a <code>FeatureStore</code>.</p> <p>This name:</p> <ul> <li> <p>Must start with an alphanumeric character.</p> </li> <li> <p>Can only contains alphanumeric characters, hyphens, underscores. Spaces are not allowed. </p> </li> </ul> |
| `role_arn` | String |  | <p>The Amazon Resource Name (ARN) of the IAM execution role used to persist data into the <code>OfflineStore</code> if an <code>OfflineStoreConfig</code> is provided.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `creation_time` | String | <p>A timestamp indicating when SageMaker created the <code>FeatureGroup</code>.</p> |
| `last_modified_time` | String | <p>A timestamp indicating when the feature group was last updated.</p> |
| `throughput_config` | String |  |
| `last_update_status` | String | <p>A value indicating whether the update made to the feature group was successful.</p> |
| `description` | String | <p>A free form description of the feature group.</p> |
| `failure_reason` | String | <p>The reason that the <code>FeatureGroup</code> failed to be replicated in the <code>OfflineStore</code>. This is failure can occur because:</p> <ul> <li> <p>The <code>FeatureGroup</code> could not be created in the <code>OfflineStore</code>.</p> </li> <li> <p>The <code>FeatureGroup</code> could not be deleted from the <code>OfflineStore</code>.</p> </li> </ul> |
| `feature_group_name` | String | <p>he name of the <code>FeatureGroup</code>.</p> |
| `next_token` | String | <p>A token to resume pagination of the list of <code>Features</code> (<code>FeatureDefinitions</code>).</p> |
| `online_store_total_size_bytes` | i64 | <p>The size of the <code>OnlineStore</code> in bytes.</p> |
| `record_identifier_feature_name` | String | <p>The name of the <code>Feature</code> used for <code>RecordIdentifier</code>, whose value uniquely identifies a record stored in the feature store.</p> |
| `feature_definitions` | Vec<String> | <p>A list of the <code>Features</code> in the <code>FeatureGroup</code>. Each feature is defined by a <code>FeatureName</code> and <code>FeatureType</code>.</p> |
| `feature_group_status` | String | <p>The status of the feature group.</p> |
| `online_store_config` | String | <p>The configuration for the <code>OnlineStore</code>.</p> |
| `event_time_feature_name` | String | <p>The name of the feature that stores the <code>EventTime</code> of a Record in a <code>FeatureGroup</code>.</p> <p> An <code>EventTime</code> is a point in time when a new event occurs that corresponds to the creation or update of a <code>Record</code> in a <code>FeatureGroup</code>. All <code>Records</code> in the <code>FeatureGroup</code> have a corresponding <code>EventTime</code>.</p> |
| `role_arn` | String | <p>The Amazon Resource Name (ARN) of the IAM execution role used to persist data into the OfflineStore if an OfflineStoreConfig is provided.</p> |
| `feature_group_arn` | String | <p>The Amazon Resource Name (ARN) of the <code>FeatureGroup</code>. </p> |
| `offline_store_status` | String | <p>The status of the <code>OfflineStore</code>. Notifies you if replicating data into the <code>OfflineStore</code> has failed. Returns either: <code>Active</code> or <code>Blocked</code> </p> |
| `offline_store_config` | String | <p>The configuration of the offline store. It includes the following configurations:</p> <ul> <li> <p>Amazon S3 location of the offline store.</p> </li> <li> <p>Configuration of the Glue data catalog.</p> </li> <li> <p>Table format of the offline store.</p> </li> <li> <p>Option to disable the automatic creation of a Glue table for the offline store.</p> </li> <li> <p>Encryption configuration.</p> </li> </ul> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create feature_group
feature_group = provider.sagemaker.Feature_group {
    event_time_feature_name = "value"  # <p>The name of the feature that stores the <code>EventTime</code> of a <code>Record</code> in a <code>FeatureGroup</code>.</p> <p>An <code>EventTime</code> is a point in time when a new event occurs that corresponds to the creation or update of a <code>Record</code> in a <code>FeatureGroup</code>. All <code>Records</code> in the <code>FeatureGroup</code> must have a corresponding <code>EventTime</code>.</p> <p>An <code>EventTime</code> can be a <code>String</code> or <code>Fractional</code>. </p> <ul> <li> <p> <code>Fractional</code>: <code>EventTime</code> feature values must be a Unix timestamp in seconds.</p> </li> <li> <p> <code>String</code>: <code>EventTime</code> feature values must be an ISO-8601 string in the format. The following formats are supported <code>yyyy-MM-dd'T'HH:mm:ssZ</code> and <code>yyyy-MM-dd'T'HH:mm:ss.SSSZ</code> where <code>yyyy</code>, <code>MM</code>, and <code>dd</code> represent the year, month, and day respectively and <code>HH</code>, <code>mm</code>, <code>ss</code>, and if applicable, <code>SSS</code> represent the hour, month, second and milliseconds respsectively. <code>'T'</code> and <code>Z</code> are constants.</p> </li> </ul>
    feature_definitions = "value"  # <p>A list of <code>Feature</code> names and types. <code>Name</code> and <code>Type</code> is compulsory per <code>Feature</code>. </p> <p>Valid feature <code>FeatureType</code>s are <code>Integral</code>, <code>Fractional</code> and <code>String</code>.</p> <p> <code>FeatureName</code>s cannot be any of the following: <code>is_deleted</code>, <code>write_time</code>, <code>api_invocation_time</code> </p> <p>You can create up to 2,500 <code>FeatureDefinition</code>s per <code>FeatureGroup</code>.</p>
    feature_group_name = "value"  # <p>The name of the <code>FeatureGroup</code>. The name must be unique within an Amazon Web Services Region in an Amazon Web Services account.</p> <p>The name:</p> <ul> <li> <p>Must start with an alphanumeric character.</p> </li> <li> <p>Can only include alphanumeric characters, underscores, and hyphens. Spaces are not allowed.</p> </li> </ul>
    record_identifier_feature_name = "value"  # <p>The name of the <code>Feature</code> whose value uniquely identifies a <code>Record</code> defined in the <code>FeatureStore</code>. Only the latest record per identifier value will be stored in the <code>OnlineStore</code>. <code>RecordIdentifierFeatureName</code> must be one of feature definitions' names.</p> <p>You use the <code>RecordIdentifierFeatureName</code> to access data in a <code>FeatureStore</code>.</p> <p>This name:</p> <ul> <li> <p>Must start with an alphanumeric character.</p> </li> <li> <p>Can only contains alphanumeric characters, hyphens, underscores. Spaces are not allowed. </p> </li> </ul>
}

# Access feature_group outputs
feature_group_id = feature_group.id
feature_group_creation_time = feature_group.creation_time
feature_group_last_modified_time = feature_group.last_modified_time
feature_group_throughput_config = feature_group.throughput_config
feature_group_last_update_status = feature_group.last_update_status
feature_group_description = feature_group.description
feature_group_failure_reason = feature_group.failure_reason
feature_group_feature_group_name = feature_group.feature_group_name
feature_group_next_token = feature_group.next_token
feature_group_online_store_total_size_bytes = feature_group.online_store_total_size_bytes
feature_group_record_identifier_feature_name = feature_group.record_identifier_feature_name
feature_group_feature_definitions = feature_group.feature_definitions
feature_group_feature_group_status = feature_group.feature_group_status
feature_group_online_store_config = feature_group.online_store_config
feature_group_event_time_feature_name = feature_group.event_time_feature_name
feature_group_role_arn = feature_group.role_arn
feature_group_feature_group_arn = feature_group.feature_group_arn
feature_group_offline_store_status = feature_group.offline_store_status
feature_group_offline_store_config = feature_group.offline_store_config
```

---


### Devices

Devices resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `devices` | Vec<String> | ✅ | <p>List of devices to register with Edge Manager agent.</p> |
| `device_fleet_name` | String | ✅ | <p>The name of the fleet the devices belong to.</p> |



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


### Model_quality_job_definition

ModelQualityJobDefinition resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `model_quality_baseline_config` | String |  | <p>Specifies the constraints and baselines for the monitoring job.</p> |
| `job_resources` | String | ✅ |  |
| `tags` | Vec<String> |  | <p>(Optional) An array of key-value pairs. For more information, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html#allocation-whatURL"> Using Cost Allocation Tags</a> in the <i>Amazon Web Services Billing and Cost Management User Guide</i>.</p> |
| `model_quality_job_input` | String | ✅ | <p>A list of the inputs that are monitored. Currently endpoints are supported.</p> |
| `job_definition_name` | String | ✅ | <p>The name of the monitoring job definition.</p> |
| `network_config` | String |  | <p>Specifies the network configuration for the monitoring job.</p> |
| `stopping_condition` | String |  |  |
| `model_quality_app_specification` | String | ✅ | <p>The container that runs the monitoring job.</p> |
| `model_quality_job_output_config` | String | ✅ |  |
| `role_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of an IAM role that Amazon SageMaker AI can assume to perform tasks on your behalf.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `model_quality_job_input` | String | <p>Inputs for the model quality job.</p> |
| `model_quality_job_output_config` | String |  |
| `job_definition_name` | String | <p>The name of the quality job definition. The name must be unique within an Amazon Web Services Region in the Amazon Web Services account.</p> |
| `job_definition_arn` | String | <p>The Amazon Resource Name (ARN) of the model quality job.</p> |
| `stopping_condition` | String |  |
| `creation_time` | String | <p>The time at which the model quality job was created.</p> |
| `model_quality_app_specification` | String | <p>Configures the model quality job to run a specified Docker container image.</p> |
| `role_arn` | String | <p>The Amazon Resource Name (ARN) of an IAM role that Amazon SageMaker AI can assume to perform tasks on your behalf.</p> |
| `model_quality_baseline_config` | String | <p>The baseline configuration for a model quality job.</p> |
| `job_resources` | String |  |
| `network_config` | String | <p>Networking options for a model quality job.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create model_quality_job_definition
model_quality_job_definition = provider.sagemaker.Model_quality_job_definition {
    job_resources = "value"  # Required field
    model_quality_job_input = "value"  # <p>A list of the inputs that are monitored. Currently endpoints are supported.</p>
    job_definition_name = "value"  # <p>The name of the monitoring job definition.</p>
    model_quality_app_specification = "value"  # <p>The container that runs the monitoring job.</p>
    model_quality_job_output_config = "value"  # Required field
    role_arn = "value"  # <p>The Amazon Resource Name (ARN) of an IAM role that Amazon SageMaker AI can assume to perform tasks on your behalf.</p>
}

# Access model_quality_job_definition outputs
model_quality_job_definition_id = model_quality_job_definition.id
model_quality_job_definition_model_quality_job_input = model_quality_job_definition.model_quality_job_input
model_quality_job_definition_model_quality_job_output_config = model_quality_job_definition.model_quality_job_output_config
model_quality_job_definition_job_definition_name = model_quality_job_definition.job_definition_name
model_quality_job_definition_job_definition_arn = model_quality_job_definition.job_definition_arn
model_quality_job_definition_stopping_condition = model_quality_job_definition.stopping_condition
model_quality_job_definition_creation_time = model_quality_job_definition.creation_time
model_quality_job_definition_model_quality_app_specification = model_quality_job_definition.model_quality_app_specification
model_quality_job_definition_role_arn = model_quality_job_definition.role_arn
model_quality_job_definition_model_quality_baseline_config = model_quality_job_definition.model_quality_baseline_config
model_quality_job_definition_job_resources = model_quality_job_definition.job_resources
model_quality_job_definition_network_config = model_quality_job_definition.network_config
```

---


### Model

Model resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `inference_execution_config` | String |  | <p>Specifies details of how containers in a multi-container endpoint are called.</p> |
| `containers` | Vec<String> |  | <p>Specifies the containers in the inference pipeline.</p> |
| `vpc_config` | String |  | <p>A <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_VpcConfig.html">VpcConfig</a> object that specifies the VPC that you want your model to connect to. Control access to and from your model container by configuring the VPC. <code>VpcConfig</code> is used in hosting services and in batch transform. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/host-vpc.html">Protect Endpoints by Using an Amazon Virtual Private Cloud</a> and <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/batch-vpc.html">Protect Data in Batch Transform Jobs by Using an Amazon Virtual Private Cloud</a>.</p> |
| `enable_network_isolation` | bool |  | <p>Isolates the model container. No inbound or outbound network calls can be made to or from the model container.</p> |
| `execution_role_arn` | String |  | <p>The Amazon Resource Name (ARN) of the IAM role that SageMaker can assume to access model artifacts and docker image for deployment on ML compute instances or for batch transform jobs. Deploying on ML compute instances is part of model hosting. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/sagemaker-roles.html">SageMaker Roles</a>. </p> <note> <p>To be able to pass this role to SageMaker, the caller of this API must have the <code>iam:PassRole</code> permission.</p> </note> |
| `tags` | Vec<String> |  | <p>An array of key-value pairs. You can use tags to categorize your Amazon Web Services resources in different ways, for example, by purpose, owner, or environment. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services Resources</a>.</p> |
| `model_name` | String | ✅ | <p>The name of the new model.</p> |
| `primary_container` | String |  | <p>The location of the primary docker image containing inference code, associated artifacts, and custom environment map that the inference code uses when the model is deployed for predictions. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `containers` | Vec<String> | <p>The containers in the inference pipeline.</p> |
| `deployment_recommendation` | String | <p>A set of recommended deployment configurations for the model.</p> |
| `vpc_config` | String | <p>A <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_VpcConfig.html">VpcConfig</a> object that specifies the VPC that this model has access to. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/host-vpc.html">Protect Endpoints by Using an Amazon Virtual Private Cloud</a> </p> |
| `enable_network_isolation` | bool | <p>If <code>True</code>, no inbound or outbound network calls can be made to or from the model container.</p> |
| `execution_role_arn` | String | <p>The Amazon Resource Name (ARN) of the IAM role that you specified for the model.</p> |
| `model_name` | String | <p>Name of the SageMaker model.</p> |
| `creation_time` | String | <p>A timestamp that shows when the model was created.</p> |
| `model_arn` | String | <p>The Amazon Resource Name (ARN) of the model.</p> |
| `primary_container` | String | <p>The location of the primary inference code, associated artifacts, and custom environment map that the inference code uses when it is deployed in production. </p> |
| `inference_execution_config` | String | <p>Specifies details of how containers in a multi-container endpoint are called.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create model
model = provider.sagemaker.Model {
    model_name = "value"  # <p>The name of the new model.</p>
}

# Access model outputs
model_id = model.id
model_containers = model.containers
model_deployment_recommendation = model.deployment_recommendation
model_vpc_config = model.vpc_config
model_enable_network_isolation = model.enable_network_isolation
model_execution_role_arn = model.execution_role_arn
model_model_name = model.model_name
model_creation_time = model.creation_time
model_model_arn = model.model_arn
model_primary_container = model.primary_container
model_inference_execution_config = model.inference_execution_config
```

---


### Model_card

ModelCard resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `model_card_name` | String | ✅ | <p>The unique name of the model card.</p> |
| `security_config` | String |  | <p>An optional Key Management Service key to encrypt, decrypt, and re-encrypt model card content for regulated workloads with highly sensitive data.</p> |
| `content` | String | ✅ | <p>The content of the model card. Content must be in <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/model-cards.html#model-cards-json-schema">model card JSON schema</a> and provided as a string.</p> |
| `model_card_status` | String | ✅ | <p>The approval status of the model card within your organization. Different organizations might have different criteria for model card review and approval.</p> <ul> <li> <p> <code>Draft</code>: The model card is a work in progress.</p> </li> <li> <p> <code>PendingReview</code>: The model card is pending review.</p> </li> <li> <p> <code>Approved</code>: The model card is approved.</p> </li> <li> <p> <code>Archived</code>: The model card is archived. No more updates should be made to the model card, but it can still be exported.</p> </li> </ul> |
| `tags` | Vec<String> |  | <p>Key-value pairs used to manage metadata for model cards.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `created_by` | String |  |
| `model_card_status` | String | <p>The approval status of the model card within your organization. Different organizations might have different criteria for model card review and approval.</p> <ul> <li> <p> <code>Draft</code>: The model card is a work in progress.</p> </li> <li> <p> <code>PendingReview</code>: The model card is pending review.</p> </li> <li> <p> <code>Approved</code>: The model card is approved.</p> </li> <li> <p> <code>Archived</code>: The model card is archived. No more updates should be made to the model card, but it can still be exported.</p> </li> </ul> |
| `last_modified_time` | String | <p>The date and time the model card was last modified.</p> |
| `last_modified_by` | String |  |
| `content` | String | <p>The content of the model card.</p> |
| `security_config` | String | <p>The security configuration used to protect model card content.</p> |
| `creation_time` | String | <p>The date and time the model card was created.</p> |
| `model_card_name` | String | <p>The name of the model card.</p> |
| `model_card_processing_status` | String | <p>The processing status of model card deletion. The <code>ModelCardProcessingStatus</code> updates throughout the different deletion steps.</p> <ul> <li> <p> <code>DeletePending</code>: Model card deletion request received.</p> </li> <li> <p> <code>DeleteInProgress</code>: Model card deletion is in progress.</p> </li> <li> <p> <code>ContentDeleted</code>: Deleted model card content.</p> </li> <li> <p> <code>ExportJobsDeleted</code>: Deleted all export jobs associated with the model card.</p> </li> <li> <p> <code>DeleteCompleted</code>: Successfully deleted the model card.</p> </li> <li> <p> <code>DeleteFailed</code>: The model card failed to delete.</p> </li> </ul> |
| `model_card_version` | i64 | <p>The version of the model card.</p> |
| `model_card_arn` | String | <p>The Amazon Resource Name (ARN) of the model card.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create model_card
model_card = provider.sagemaker.Model_card {
    model_card_name = "value"  # <p>The unique name of the model card.</p>
    content = "value"  # <p>The content of the model card. Content must be in <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/model-cards.html#model-cards-json-schema">model card JSON schema</a> and provided as a string.</p>
    model_card_status = "value"  # <p>The approval status of the model card within your organization. Different organizations might have different criteria for model card review and approval.</p> <ul> <li> <p> <code>Draft</code>: The model card is a work in progress.</p> </li> <li> <p> <code>PendingReview</code>: The model card is pending review.</p> </li> <li> <p> <code>Approved</code>: The model card is approved.</p> </li> <li> <p> <code>Archived</code>: The model card is archived. No more updates should be made to the model card, but it can still be exported.</p> </li> </ul>
}

# Access model_card outputs
model_card_id = model_card.id
model_card_created_by = model_card.created_by
model_card_model_card_status = model_card.model_card_status
model_card_last_modified_time = model_card.last_modified_time
model_card_last_modified_by = model_card.last_modified_by
model_card_content = model_card.content
model_card_security_config = model_card.security_config
model_card_creation_time = model_card.creation_time
model_card_model_card_name = model_card.model_card_name
model_card_model_card_processing_status = model_card.model_card_processing_status
model_card_model_card_version = model_card.model_card_version
model_card_model_card_arn = model_card.model_card_arn
```

---


### Cluster_node

ClusterNode resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `node_details` | String | <p>The details of the SageMaker HyperPod cluster node.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access cluster_node outputs
cluster_node_id = cluster_node.id
cluster_node_node_details = cluster_node.node_details
```

---


### Endpoint_weights_and_capacities

EndpointWeightsAndCapacities resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `desired_weights_and_capacities` | Vec<String> | ✅ | <p>An object that provides new capacity and weight values for a variant.</p> |
| `endpoint_name` | String | ✅ | <p>The name of an existing SageMaker endpoint.</p> |



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


### Edge_deployment_plan

EdgeDeploymentPlan resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `edge_deployment_plan_name` | String | ✅ | <p>The name of the edge deployment plan.</p> |
| `stages` | Vec<String> |  | <p>List of stages of the edge deployment plan. The number of stages is limited to 10 per deployment.</p> |
| `model_configs` | Vec<String> | ✅ | <p>List of models associated with the edge deployment plan.</p> |
| `tags` | Vec<String> |  | <p>List of tags with which to tag the edge deployment plan.</p> |
| `device_fleet_name` | String | ✅ | <p>The device fleet used for this edge deployment plan.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `last_modified_time` | String | <p>The time when the edge deployment plan was last updated.</p> |
| `edge_deployment_plan_arn` | String | <p>The ARN of edge deployment plan.</p> |
| `edge_deployment_plan_name` | String | <p>The name of the edge deployment plan.</p> |
| `next_token` | String | <p>Token to use when calling the next set of stages in the edge deployment plan.</p> |
| `stages` | Vec<String> | <p>List of stages in the edge deployment plan.</p> |
| `device_fleet_name` | String | <p>The device fleet used for this edge deployment plan.</p> |
| `edge_deployment_pending` | i64 | <p>The number of edge devices yet to pick up deployment, or in progress.</p> |
| `model_configs` | Vec<String> | <p>List of models associated with the edge deployment plan.</p> |
| `edge_deployment_success` | i64 | <p>The number of edge devices with the successful deployment.</p> |
| `creation_time` | String | <p>The time when the edge deployment plan was created.</p> |
| `edge_deployment_failed` | i64 | <p>The number of edge devices that failed the deployment.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create edge_deployment_plan
edge_deployment_plan = provider.sagemaker.Edge_deployment_plan {
    edge_deployment_plan_name = "value"  # <p>The name of the edge deployment plan.</p>
    model_configs = "value"  # <p>List of models associated with the edge deployment plan.</p>
    device_fleet_name = "value"  # <p>The device fleet used for this edge deployment plan.</p>
}

# Access edge_deployment_plan outputs
edge_deployment_plan_id = edge_deployment_plan.id
edge_deployment_plan_last_modified_time = edge_deployment_plan.last_modified_time
edge_deployment_plan_edge_deployment_plan_arn = edge_deployment_plan.edge_deployment_plan_arn
edge_deployment_plan_edge_deployment_plan_name = edge_deployment_plan.edge_deployment_plan_name
edge_deployment_plan_next_token = edge_deployment_plan.next_token
edge_deployment_plan_stages = edge_deployment_plan.stages
edge_deployment_plan_device_fleet_name = edge_deployment_plan.device_fleet_name
edge_deployment_plan_edge_deployment_pending = edge_deployment_plan.edge_deployment_pending
edge_deployment_plan_model_configs = edge_deployment_plan.model_configs
edge_deployment_plan_edge_deployment_success = edge_deployment_plan.edge_deployment_success
edge_deployment_plan_creation_time = edge_deployment_plan.creation_time
edge_deployment_plan_edge_deployment_failed = edge_deployment_plan.edge_deployment_failed
```

---


### Hyper_parameter_tuning_job

HyperParameterTuningJob resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `warm_start_config` | String |  | <p>Specifies the configuration for starting the hyperparameter tuning job using one or more previous tuning jobs as a starting point. The results of previous tuning jobs are used to inform which combinations of hyperparameters to search over in the new tuning job.</p> <p>All training jobs launched by the new hyperparameter tuning job are evaluated by using the objective metric. If you specify <code>IDENTICAL_DATA_AND_ALGORITHM</code> as the <code>WarmStartType</code> value for the warm start configuration, the training job that performs the best in the new tuning job is compared to the best training jobs from the parent tuning jobs. From these, the training job that performs the best as measured by the objective metric is returned as the overall best training job.</p> <note> <p>All training jobs launched by parent hyperparameter tuning jobs and the new hyperparameter tuning jobs count against the limit of training jobs for the tuning job.</p> </note> |
| `autotune` | String |  | <p>Configures SageMaker Automatic model tuning (AMT) to automatically find optimal parameters for the following fields:</p> <ul> <li> <p> <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_HyperParameterTuningJobConfig.html#sagemaker-Type-HyperParameterTuningJobConfig-ParameterRanges">ParameterRanges</a>: The names and ranges of parameters that a hyperparameter tuning job can optimize.</p> </li> <li> <p> <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_ResourceLimits.html">ResourceLimits</a>: The maximum resources that can be used for a training job. These resources include the maximum number of training jobs, the maximum runtime of a tuning job, and the maximum number of training jobs to run at the same time.</p> </li> <li> <p> <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_HyperParameterTuningJobConfig.html#sagemaker-Type-HyperParameterTuningJobConfig-TrainingJobEarlyStoppingType">TrainingJobEarlyStoppingType</a>: A flag that specifies whether or not to use early stopping for training jobs launched by a hyperparameter tuning job.</p> </li> <li> <p> <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_HyperParameterTrainingJobDefinition.html#sagemaker-Type-HyperParameterTrainingJobDefinition-RetryStrategy">RetryStrategy</a>: The number of times to retry a training job.</p> </li> <li> <p> <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_HyperParameterTuningJobConfig.html">Strategy</a>: Specifies how hyperparameter tuning chooses the combinations of hyperparameter values to use for the training jobs that it launches.</p> </li> <li> <p> <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_ConvergenceDetected.html">ConvergenceDetected</a>: A flag to indicate that Automatic model tuning (AMT) has detected model convergence.</p> </li> </ul> |
| `hyper_parameter_tuning_job_name` | String | ✅ | <p>The name of the tuning job. This name is the prefix for the names of all training jobs that this tuning job launches. The name must be unique within the same Amazon Web Services account and Amazon Web Services Region. The name must have 1 to 32 characters. Valid characters are a-z, A-Z, 0-9, and : + = @ _ % - (hyphen). The name is not case sensitive.</p> |
| `tags` | Vec<String> |  | <p>An array of key-value pairs. You can use tags to categorize your Amazon Web Services resources in different ways, for example, by purpose, owner, or environment. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services Resources</a>.</p> <p>Tags that you specify for the tuning job are also added to all training jobs that the tuning job launches.</p> |
| `training_job_definitions` | Vec<String> |  | <p>A list of the <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_HyperParameterTrainingJobDefinition.html">HyperParameterTrainingJobDefinition</a> objects launched for this tuning job.</p> |
| `training_job_definition` | String |  | <p>The <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_HyperParameterTrainingJobDefinition.html">HyperParameterTrainingJobDefinition</a> object that describes the training jobs that this tuning job launches, including static hyperparameters, input data configuration, output data configuration, resource configuration, and stopping condition.</p> |
| `hyper_parameter_tuning_job_config` | String | ✅ | <p>The <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_HyperParameterTuningJobConfig.html">HyperParameterTuningJobConfig</a> object that describes the tuning job, including the search strategy, the objective metric used to evaluate training jobs, ranges of parameters to search, and resource limits for the tuning job. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/automatic-model-tuning-how-it-works.html">How Hyperparameter Tuning Works</a>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `hyper_parameter_tuning_job_name` | String | <p>The name of the hyperparameter tuning job.</p> |
| `hyper_parameter_tuning_job_status` | String | <p>The status of the tuning job.</p> |
| `overall_best_training_job` | String | <p>If the hyperparameter tuning job is an warm start tuning job with a <code>WarmStartType</code> of <code>IDENTICAL_DATA_AND_ALGORITHM</code>, this is the <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_TrainingJobSummary.html">TrainingJobSummary</a> for the training job with the best objective metric value of all training jobs launched by this tuning job and all parent jobs specified for the warm start tuning job.</p> |
| `training_job_definitions` | Vec<String> | <p>A list of the <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_HyperParameterTrainingJobDefinition.html">HyperParameterTrainingJobDefinition</a> objects launched for this tuning job.</p> |
| `creation_time` | String | <p>The date and time that the tuning job started.</p> |
| `failure_reason` | String | <p>If the tuning job failed, the reason it failed.</p> |
| `tuning_job_completion_details` | String | <p>Tuning job completion information returned as the response from a hyperparameter tuning job. This information tells if your tuning job has or has not converged. It also includes the number of training jobs that have not improved model performance as evaluated against the objective function.</p> |
| `training_job_status_counters` | String | <p>The <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_TrainingJobStatusCounters.html">TrainingJobStatusCounters</a> object that specifies the number of training jobs, categorized by status, that this tuning job launched.</p> |
| `autotune` | String | <p>A flag to indicate if autotune is enabled for the hyperparameter tuning job.</p> |
| `last_modified_time` | String | <p>The date and time that the status of the tuning job was modified. </p> |
| `hyper_parameter_tuning_job_arn` | String | <p>The Amazon Resource Name (ARN) of the tuning job.</p> |
| `consumed_resources` | String |  |
| `hyper_parameter_tuning_end_time` | String | <p>The date and time that the tuning job ended.</p> |
| `hyper_parameter_tuning_job_config` | String | <p>The <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_HyperParameterTuningJobConfig.html">HyperParameterTuningJobConfig</a> object that specifies the configuration of the tuning job.</p> |
| `best_training_job` | String | <p>A <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_TrainingJobSummary.html">TrainingJobSummary</a> object that describes the training job that completed with the best current <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_HyperParameterTuningJobObjective.html">HyperParameterTuningJobObjective</a>.</p> |
| `training_job_definition` | String | <p>The <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_HyperParameterTrainingJobDefinition.html">HyperParameterTrainingJobDefinition</a> object that specifies the definition of the training jobs that this tuning job launches.</p> |
| `objective_status_counters` | String | <p>The <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_ObjectiveStatusCounters.html">ObjectiveStatusCounters</a> object that specifies the number of training jobs, categorized by the status of their final objective metric, that this tuning job launched.</p> |
| `warm_start_config` | String | <p>The configuration for starting the hyperparameter parameter tuning job using one or more previous tuning jobs as a starting point. The results of previous tuning jobs are used to inform which combinations of hyperparameters to search over in the new tuning job.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create hyper_parameter_tuning_job
hyper_parameter_tuning_job = provider.sagemaker.Hyper_parameter_tuning_job {
    hyper_parameter_tuning_job_name = "value"  # <p>The name of the tuning job. This name is the prefix for the names of all training jobs that this tuning job launches. The name must be unique within the same Amazon Web Services account and Amazon Web Services Region. The name must have 1 to 32 characters. Valid characters are a-z, A-Z, 0-9, and : + = @ _ % - (hyphen). The name is not case sensitive.</p>
    hyper_parameter_tuning_job_config = "value"  # <p>The <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_HyperParameterTuningJobConfig.html">HyperParameterTuningJobConfig</a> object that describes the tuning job, including the search strategy, the objective metric used to evaluate training jobs, ranges of parameters to search, and resource limits for the tuning job. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/automatic-model-tuning-how-it-works.html">How Hyperparameter Tuning Works</a>.</p>
}

# Access hyper_parameter_tuning_job outputs
hyper_parameter_tuning_job_id = hyper_parameter_tuning_job.id
hyper_parameter_tuning_job_hyper_parameter_tuning_job_name = hyper_parameter_tuning_job.hyper_parameter_tuning_job_name
hyper_parameter_tuning_job_hyper_parameter_tuning_job_status = hyper_parameter_tuning_job.hyper_parameter_tuning_job_status
hyper_parameter_tuning_job_overall_best_training_job = hyper_parameter_tuning_job.overall_best_training_job
hyper_parameter_tuning_job_training_job_definitions = hyper_parameter_tuning_job.training_job_definitions
hyper_parameter_tuning_job_creation_time = hyper_parameter_tuning_job.creation_time
hyper_parameter_tuning_job_failure_reason = hyper_parameter_tuning_job.failure_reason
hyper_parameter_tuning_job_tuning_job_completion_details = hyper_parameter_tuning_job.tuning_job_completion_details
hyper_parameter_tuning_job_training_job_status_counters = hyper_parameter_tuning_job.training_job_status_counters
hyper_parameter_tuning_job_autotune = hyper_parameter_tuning_job.autotune
hyper_parameter_tuning_job_last_modified_time = hyper_parameter_tuning_job.last_modified_time
hyper_parameter_tuning_job_hyper_parameter_tuning_job_arn = hyper_parameter_tuning_job.hyper_parameter_tuning_job_arn
hyper_parameter_tuning_job_consumed_resources = hyper_parameter_tuning_job.consumed_resources
hyper_parameter_tuning_job_hyper_parameter_tuning_end_time = hyper_parameter_tuning_job.hyper_parameter_tuning_end_time
hyper_parameter_tuning_job_hyper_parameter_tuning_job_config = hyper_parameter_tuning_job.hyper_parameter_tuning_job_config
hyper_parameter_tuning_job_best_training_job = hyper_parameter_tuning_job.best_training_job
hyper_parameter_tuning_job_training_job_definition = hyper_parameter_tuning_job.training_job_definition
hyper_parameter_tuning_job_objective_status_counters = hyper_parameter_tuning_job.objective_status_counters
hyper_parameter_tuning_job_warm_start_config = hyper_parameter_tuning_job.warm_start_config
```

---


### Presigned_mlflow_tracking_server_url

PresignedMlflowTrackingServerUrl resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `session_expiration_duration_in_seconds` | i64 |  | <p>The duration in seconds that your MLflow UI session is valid.</p> |
| `expires_in_seconds` | i64 |  | <p>The duration in seconds that your presigned URL is valid. The presigned URL can be used only once.</p> |
| `tracking_server_name` | String | ✅ | <p>The name of the tracking server to connect to your MLflow UI.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create presigned_mlflow_tracking_server_url
presigned_mlflow_tracking_server_url = provider.sagemaker.Presigned_mlflow_tracking_server_url {
    tracking_server_name = "value"  # <p>The name of the tracking server to connect to your MLflow UI.</p>
}

```

---


### Notebook_instance_lifecycle_config

NotebookInstanceLifecycleConfig resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `notebook_instance_lifecycle_config_name` | String | ✅ | <p>The name of the lifecycle configuration.</p> |
| `on_create` | Vec<String> |  | <p>A shell script that runs only once, when you create a notebook instance. The shell script must be a base64-encoded string.</p> |
| `tags` | Vec<String> |  | <p>An array of key-value pairs. You can use tags to categorize your Amazon Web Services resources in different ways, for example, by purpose, owner, or environment. For more information, see <a href="https://docs.aws.amazon.com/tag-editor/latest/userguide/tagging.html">Tagging Amazon Web Services Resources</a>.</p> |
| `on_start` | Vec<String> |  | <p>A shell script that runs every time you start a notebook instance, including when you create the notebook instance. The shell script must be a base64-encoded string.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `creation_time` | String | <p>A timestamp that tells when the lifecycle configuration was created.</p> |
| `on_start` | Vec<String> | <p>The shell script that runs every time you start a notebook instance, including when you create the notebook instance.</p> |
| `notebook_instance_lifecycle_config_name` | String | <p>The name of the lifecycle configuration.</p> |
| `on_create` | Vec<String> | <p>The shell script that runs only once, when you create a notebook instance.</p> |
| `notebook_instance_lifecycle_config_arn` | String | <p>The Amazon Resource Name (ARN) of the lifecycle configuration.</p> |
| `last_modified_time` | String | <p>A timestamp that tells when the lifecycle configuration was last modified.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create notebook_instance_lifecycle_config
notebook_instance_lifecycle_config = provider.sagemaker.Notebook_instance_lifecycle_config {
    notebook_instance_lifecycle_config_name = "value"  # <p>The name of the lifecycle configuration.</p>
}

# Access notebook_instance_lifecycle_config outputs
notebook_instance_lifecycle_config_id = notebook_instance_lifecycle_config.id
notebook_instance_lifecycle_config_creation_time = notebook_instance_lifecycle_config.creation_time
notebook_instance_lifecycle_config_on_start = notebook_instance_lifecycle_config.on_start
notebook_instance_lifecycle_config_notebook_instance_lifecycle_config_name = notebook_instance_lifecycle_config.notebook_instance_lifecycle_config_name
notebook_instance_lifecycle_config_on_create = notebook_instance_lifecycle_config.on_create
notebook_instance_lifecycle_config_notebook_instance_lifecycle_config_arn = notebook_instance_lifecycle_config.notebook_instance_lifecycle_config_arn
notebook_instance_lifecycle_config_last_modified_time = notebook_instance_lifecycle_config.last_modified_time
```

---


### Action

Action resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | <p>The description of the action.</p> |
| `metadata_properties` | String |  |  |
| `tags` | Vec<String> |  | <p>A list of tags to apply to the action.</p> |
| `source` | String | ✅ | <p>The source type, ID, and URI.</p> |
| `status` | String |  | <p>The status of the action.</p> |
| `properties` | HashMap<String, String> |  | <p>A list of properties to add to the action.</p> |
| `action_name` | String | ✅ | <p>The name of the action. Must be unique to your account in an Amazon Web Services Region.</p> |
| `action_type` | String | ✅ | <p>The action type.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `action_type` | String | <p>The type of the action.</p> |
| `last_modified_time` | String | <p>When the action was last modified.</p> |
| `description` | String | <p>The description of the action.</p> |
| `metadata_properties` | String |  |
| `properties` | HashMap<String, String> | <p>A list of the action's properties.</p> |
| `action_arn` | String | <p>The Amazon Resource Name (ARN) of the action.</p> |
| `status` | String | <p>The status of the action.</p> |
| `lineage_group_arn` | String | <p>The Amazon Resource Name (ARN) of the lineage group.</p> |
| `action_name` | String | <p>The name of the action.</p> |
| `source` | String | <p>The source of the action.</p> |
| `created_by` | String |  |
| `last_modified_by` | String |  |
| `creation_time` | String | <p>When the action was created.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create action
action = provider.sagemaker.Action {
    source = "value"  # <p>The source type, ID, and URI.</p>
    action_name = "value"  # <p>The name of the action. Must be unique to your account in an Amazon Web Services Region.</p>
    action_type = "value"  # <p>The action type.</p>
}

# Access action outputs
action_id = action.id
action_action_type = action.action_type
action_last_modified_time = action.last_modified_time
action_description = action.description
action_metadata_properties = action.metadata_properties
action_properties = action.properties
action_action_arn = action.action_arn
action_status = action.status
action_lineage_group_arn = action.lineage_group_arn
action_action_name = action.action_name
action_source = action.source
action_created_by = action.created_by
action_last_modified_by = action.last_modified_by
action_creation_time = action.creation_time
```

---


### Image_version

ImageVersion resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `ml_framework` | String |  | <p>The machine learning framework vended in the image version.</p> |
| `image_name` | String | ✅ | <p>The <code>ImageName</code> of the <code>Image</code> to create a version of.</p> |
| `aliases` | Vec<String> |  | <p>A list of aliases created with the image version.</p> |
| `vendor_guidance` | String |  | <p>The stability of the image version, specified by the maintainer.</p> <ul> <li> <p> <code>NOT_PROVIDED</code>: The maintainers did not provide a status for image version stability.</p> </li> <li> <p> <code>STABLE</code>: The image version is stable.</p> </li> <li> <p> <code>TO_BE_ARCHIVED</code>: The image version is set to be archived. Custom image versions that are set to be archived are automatically archived after three months.</p> </li> <li> <p> <code>ARCHIVED</code>: The image version is archived. Archived image versions are not searchable and are no longer actively supported. </p> </li> </ul> |
| `job_type` | String |  | <p>Indicates SageMaker AI job type compatibility.</p> <ul> <li> <p> <code>TRAINING</code>: The image version is compatible with SageMaker AI training jobs.</p> </li> <li> <p> <code>INFERENCE</code>: The image version is compatible with SageMaker AI inference jobs.</p> </li> <li> <p> <code>NOTEBOOK_KERNEL</code>: The image version is compatible with SageMaker AI notebook kernels.</p> </li> </ul> |
| `programming_lang` | String |  | <p>The supported programming language and its version.</p> |
| `horovod` | bool |  | <p>Indicates Horovod compatibility.</p> |
| `release_notes` | String |  | <p>The maintainer description of the image version.</p> |
| `client_token` | String | ✅ | <p>A unique ID. If not specified, the Amazon Web Services CLI and Amazon Web Services SDKs, such as the SDK for Python (Boto3), add a unique value to the call.</p> |
| `base_image` | String | ✅ | <p>The registry path of the container image to use as the starting point for this version. The path is an Amazon ECR URI in the following format:</p> <p> <code>&lt;acct-id&gt;.dkr.ecr.&lt;region&gt;.amazonaws.com/&lt;repo-name[:tag] or [@digest]&gt;</code> </p> |
| `processor` | String |  | <p>Indicates CPU or GPU compatibility.</p> <ul> <li> <p> <code>CPU</code>: The image version is compatible with CPU.</p> </li> <li> <p> <code>GPU</code>: The image version is compatible with GPU.</p> </li> </ul> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `container_image` | String | <p>The registry path of the container image that contains this image version.</p> |
| `creation_time` | String | <p>When the version was created.</p> |
| `processor` | String | <p>Indicates CPU or GPU compatibility.</p> <ul> <li> <p> <code>CPU</code>: The image version is compatible with CPU.</p> </li> <li> <p> <code>GPU</code>: The image version is compatible with GPU.</p> </li> </ul> |
| `image_version_arn` | String | <p>The ARN of the version.</p> |
| `ml_framework` | String | <p>The machine learning framework vended in the image version.</p> |
| `image_version_status` | String | <p>The status of the version.</p> |
| `base_image` | String | <p>The registry path of the container image on which this image version is based.</p> |
| `release_notes` | String | <p>The maintainer description of the image version.</p> |
| `last_modified_time` | String | <p>When the version was last modified.</p> |
| `image_arn` | String | <p>The ARN of the image the version is based on.</p> |
| `version` | i64 | <p>The version number.</p> |
| `programming_lang` | String | <p>The supported programming language and its version.</p> |
| `vendor_guidance` | String | <p>The stability of the image version specified by the maintainer.</p> <ul> <li> <p> <code>NOT_PROVIDED</code>: The maintainers did not provide a status for image version stability.</p> </li> <li> <p> <code>STABLE</code>: The image version is stable.</p> </li> <li> <p> <code>TO_BE_ARCHIVED</code>: The image version is set to be archived. Custom image versions that are set to be archived are automatically archived after three months.</p> </li> <li> <p> <code>ARCHIVED</code>: The image version is archived. Archived image versions are not searchable and are no longer actively supported. </p> </li> </ul> |
| `job_type` | String | <p>Indicates SageMaker AI job type compatibility.</p> <ul> <li> <p> <code>TRAINING</code>: The image version is compatible with SageMaker AI training jobs.</p> </li> <li> <p> <code>INFERENCE</code>: The image version is compatible with SageMaker AI inference jobs.</p> </li> <li> <p> <code>NOTEBOOK_KERNEL</code>: The image version is compatible with SageMaker AI notebook kernels.</p> </li> </ul> |
| `horovod` | bool | <p>Indicates Horovod compatibility.</p> |
| `failure_reason` | String | <p>When a create or delete operation fails, the reason for the failure.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create image_version
image_version = provider.sagemaker.Image_version {
    image_name = "value"  # <p>The <code>ImageName</code> of the <code>Image</code> to create a version of.</p>
    client_token = "value"  # <p>A unique ID. If not specified, the Amazon Web Services CLI and Amazon Web Services SDKs, such as the SDK for Python (Boto3), add a unique value to the call.</p>
    base_image = "value"  # <p>The registry path of the container image to use as the starting point for this version. The path is an Amazon ECR URI in the following format:</p> <p> <code>&lt;acct-id&gt;.dkr.ecr.&lt;region&gt;.amazonaws.com/&lt;repo-name[:tag] or [@digest]&gt;</code> </p>
}

# Access image_version outputs
image_version_id = image_version.id
image_version_container_image = image_version.container_image
image_version_creation_time = image_version.creation_time
image_version_processor = image_version.processor
image_version_image_version_arn = image_version.image_version_arn
image_version_ml_framework = image_version.ml_framework
image_version_image_version_status = image_version.image_version_status
image_version_base_image = image_version.base_image
image_version_release_notes = image_version.release_notes
image_version_last_modified_time = image_version.last_modified_time
image_version_image_arn = image_version.image_arn
image_version_version = image_version.version
image_version_programming_lang = image_version.programming_lang
image_version_vendor_guidance = image_version.vendor_guidance
image_version_job_type = image_version.job_type
image_version_horovod = image_version.horovod
image_version_failure_reason = image_version.failure_reason
```

---


### Association

Association resource

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



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple edge_packaging_job resources
edge_packaging_job_0 = provider.sagemaker.Edge_packaging_job {
    model_version = "value-0"
    compilation_job_name = "value-0"
    edge_packaging_job_name = "value-0"
    model_name = "value-0"
    output_config = "value-0"
    role_arn = "value-0"
}
edge_packaging_job_1 = provider.sagemaker.Edge_packaging_job {
    model_version = "value-1"
    compilation_job_name = "value-1"
    edge_packaging_job_name = "value-1"
    model_name = "value-1"
    output_config = "value-1"
    role_arn = "value-1"
}
edge_packaging_job_2 = provider.sagemaker.Edge_packaging_job {
    model_version = "value-2"
    compilation_job_name = "value-2"
    edge_packaging_job_name = "value-2"
    model_name = "value-2"
    output_config = "value-2"
    role_arn = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    edge_packaging_job = provider.sagemaker.Edge_packaging_job {
        model_version = "production-value"
        compilation_job_name = "production-value"
        edge_packaging_job_name = "production-value"
        model_name = "production-value"
        output_config = "production-value"
        role_arn = "production-value"
    }
```

---

## Related Documentation

- [AWS Sagemaker Documentation](https://docs.aws.amazon.com/sagemaker/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
