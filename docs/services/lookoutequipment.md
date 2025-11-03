# Lookoutequipment Service



**Resources**: 10

---

## Overview

The lookoutequipment service provides access to 10 resource types:

- [Resource_policy](#resource_policy) [CRD]
- [Active_model_version](#active_model_version) [U]
- [Model](#model) [CRUD]
- [Inference_scheduler](#inference_scheduler) [CRUD]
- [Dataset](#dataset) [CRD]
- [Retraining_scheduler](#retraining_scheduler) [CRUD]
- [Data_ingestion_job](#data_ingestion_job) [R]
- [Label_group](#label_group) [CRUD]
- [Model_version](#model_version) [R]
- [Label](#label) [CRD]

---

## Resources


### Resource_policy

ResourcePolicy resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `policy_revision_id` | String |  | <p>A unique identifier for a revision of the resource policy.</p> |
| `resource_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the resource for which the policy is being
         created.</p> |
| `client_token` | String | ✅ | <p>A unique identifier for the request. If you do not set the client request token,
         Amazon Lookout for Equipment generates one. </p> |
| `resource_policy` | String | ✅ | <p>The JSON-formatted resource policy to create.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `resource_policy` | String | <p>The resource policy in a JSON-formatted string.</p> |
| `last_modified_time` | String | <p>The time when the resource policy was last modified.</p> |
| `policy_revision_id` | String | <p>A unique identifier for a revision of the resource policy.</p> |
| `creation_time` | String | <p>The time when the resource policy was created.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create resource_policy
resource_policy = provider.lookoutequipment.Resource_policy {
    resource_arn = "value"  # <p>The Amazon Resource Name (ARN) of the resource for which the policy is being
         created.</p>
    client_token = "value"  # <p>A unique identifier for the request. If you do not set the client request token,
         Amazon Lookout for Equipment generates one. </p>
    resource_policy = "value"  # <p>The JSON-formatted resource policy to create.</p>
}

# Access resource_policy outputs
resource_policy_id = resource_policy.id
resource_policy_resource_policy = resource_policy.resource_policy
resource_policy_last_modified_time = resource_policy.last_modified_time
resource_policy_policy_revision_id = resource_policy.policy_revision_id
resource_policy_creation_time = resource_policy.creation_time
```

---


### Active_model_version

ActiveModelVersion resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `model_name` | String | ✅ | <p>The name of the machine learning model for which the active model version is being
         set.</p> |
| `model_version` | i64 | ✅ | <p>The version of the machine learning model for which the active model version is being
         set.</p> |



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


### Model

Model resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `data_pre_processing_configuration` | String |  | <p>The configuration is the <code>TargetSamplingRate</code>, which is the sampling rate of
         the data after post processing by Amazon Lookout for Equipment. For example, if you provide data that has been
         collected at a 1 second level and you want the system to resample the data at a 1 minute
         rate before training, the <code>TargetSamplingRate</code> is 1 minute.</p>
         <p>When providing a value for the <code>TargetSamplingRate</code>, you must attach the
         prefix "PT" to the rate you want. The value for a 1 second rate is therefore
            <i>PT1S</i>, the value for a 15 minute rate is <i>PT15M</i>,
         and the value for a 1 hour rate is <i>PT1H</i>
         </p> |
| `off_condition` | String |  | <p>Indicates that the asset associated with this sensor has been shut off. As long as this
         condition is met, Lookout for Equipment will not use data from this asset for training,
         evaluation, or inference.</p> |
| `training_data_end_time` | String |  | <p>Indicates the time reference in the dataset that should be used to end the subset of
         training data for the machine learning model. </p> |
| `role_arn` | String |  | <p> The Amazon Resource Name (ARN) of a role with permission to access the data source
         being used to create the machine learning model. </p> |
| `training_data_start_time` | String |  | <p>Indicates the time reference in the dataset that should be used to begin the subset of
         training data for the machine learning model. </p> |
| `labels_input_configuration` | String |  | <p>The input configuration for the labels being used for the machine learning model that's
         being created. </p> |
| `client_token` | String | ✅ | <p>A unique identifier for the request. If you do not set the client request token, Amazon
         Lookout for Equipment generates one. </p> |
| `model_diagnostics_output_configuration` | String |  | <p>The Amazon S3 location where you want Amazon Lookout for Equipment to save the pointwise model diagnostics.
        
         
      You must also specify the <code>RoleArn</code> request parameter.</p> |
| `dataset_name` | String | ✅ | <p>The name of the dataset for the machine learning model being created. </p> |
| `model_name` | String | ✅ | <p>The name for the machine learning model to be created.</p> |
| `server_side_kms_key_id` | String |  | <p>Provides the identifier of the KMS key used to encrypt model data by Amazon Lookout
         for Equipment. </p> |
| `dataset_schema` | String |  | <p>The data schema for the machine learning model being created. </p> |
| `evaluation_data_start_time` | String |  | <p>Indicates the time reference in the dataset that should be used to begin the subset of
         evaluation data for the machine learning model. </p> |
| `evaluation_data_end_time` | String |  | <p> Indicates the time reference in the dataset that should be used to end the subset of
         evaluation data for the machine learning model. </p> |
| `tags` | Vec<String> |  | <p> Any tags associated with the machine learning model being created. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `model_arn` | String | <p>The Amazon Resource Name (ARN) of the machine learning model being described. </p> |
| `model_name` | String | <p>The name of the machine learning model being described. </p> |
| `source_model_version_arn` | String | <p>The Amazon Resource Name (ARN) of the source model version. This field appears if the
         active model version was imported.</p> |
| `created_at` | String | <p>Indicates the time and date at which the machine learning model was created. </p> |
| `active_model_version` | i64 | <p>The name of the model version used by the inference schedular when running a scheduled
         inference execution.</p> |
| `next_scheduled_retraining_start_date` | String | <p>Indicates the date and time that the next scheduled retraining run will start on. Lookout for Equipment
         truncates the time you provide to the nearest UTC day.</p> |
| `retraining_scheduler_status` | String | <p>Indicates the status of the retraining scheduler. </p> |
| `model_version_activated_at` | String | <p>The date the active model version was activated.</p> |
| `previous_model_version_activated_at` | String | <p>The date and time when the previous active model version was activated.</p> |
| `model_quality` | String | <p>Provides a quality assessment for a model that uses labels. If Lookout for Equipment determines that the
         model quality is poor based on training metrics, the value is
         <code>POOR_QUALITY_DETECTED</code>. Otherwise, the value is
         <code>QUALITY_THRESHOLD_MET</code>.</p>
         <p>If the model is unlabeled, the model quality can't
         be assessed and the value of <code>ModelQuality</code> is
         <code>CANNOT_DETERMINE_QUALITY</code>. In this situation, you can get a model quality
         assessment by adding labels to the input dataset and retraining the model.</p>
         <p>For information about using labels with your models, see <a href="https://docs.aws.amazon.com/lookout-for-equipment/latest/ug/understanding-labeling.html">Understanding labeling</a>.</p>
         <p>For information about improving the quality of a model, see <a href="https://docs.aws.amazon.com/lookout-for-equipment/latest/ug/best-practices.html">Best practices with
         Amazon Lookout for Equipment</a>.</p> |
| `schema` | String | <p>A JSON description of the data that is in each time series dataset, including names,
         column names, and data types. </p> |
| `import_job_end_time` | String | <p>The date and time when the import job was completed. This field appears if the active
         model version was imported.</p> |
| `accumulated_inference_data_start_time` | String | <p>Indicates the start time of the inference data that has been accumulated. </p> |
| `last_updated_time` | String | <p>Indicates the last time the machine learning model was updated. The type of update is
         not specified. </p> |
| `dataset_name` | String | <p>The name of the dataset being used by the machine learning being described. </p> |
| `import_job_start_time` | String | <p>The date and time when the import job was started. This field appears if the active
         model version was imported.</p> |
| `status` | String | <p>Specifies the current status of the model being described. Status describes the status
         of the most recent action of the model. </p> |
| `prior_model_metrics` | String | <p>If the model version was retrained, this field shows a summary of the performance of the
         prior model on the new training range. You can use the information in this JSON-formatted
         object to compare the new model version and the prior model version.</p> |
| `training_data_end_time` | String | <p> Indicates the time reference in the dataset that was used to end the subset of training
         data for the machine learning model. </p> |
| `evaluation_data_start_time` | String | <p> Indicates the time reference in the dataset that was used to begin the subset of
         evaluation data for the machine learning model. </p> |
| `latest_scheduled_retraining_available_data_in_days` | i64 | <p>Indicates the number of days of data used in the most recent scheduled retraining run.
      </p> |
| `previous_active_model_version_arn` | String | <p>The ARN of the model version that was set as the active model version prior to the
         current active model version.</p> |
| `data_pre_processing_configuration` | String | <p>The configuration is the <code>TargetSamplingRate</code>, which is the sampling rate of
         the data after post processing by Amazon Lookout for Equipment. For example, if you provide data that has been
         collected at a 1 second level and you want the system to resample the data at a 1 minute
         rate before training, the <code>TargetSamplingRate</code> is 1 minute.</p>
         <p>When providing a value for the <code>TargetSamplingRate</code>, you must attach the
         prefix "PT" to the rate you want. The value for a 1 second rate is therefore
            <i>PT1S</i>, the value for a 15 minute rate is <i>PT15M</i>,
         and the value for a 1 hour rate is <i>PT1H</i>
         </p> |
| `training_data_start_time` | String | <p> Indicates the time reference in the dataset that was used to begin the subset of
         training data for the machine learning model. </p> |
| `role_arn` | String | <p> The Amazon Resource Name (ARN) of a role with permission to access the data source for
         the machine learning model being described. </p> |
| `model_metrics` | String | <p>The Model Metrics show an aggregated summary of the model's performance within the
         evaluation time range. This is the JSON content of the metrics created when evaluating the
         model. </p> |
| `training_execution_end_time` | String | <p>Indicates the time at which the training of the machine learning model was completed.
      </p> |
| `accumulated_inference_data_end_time` | String | <p>Indicates the end time of the inference data that has been accumulated. </p> |
| `latest_scheduled_retraining_status` | String | <p>Indicates the status of the most recent scheduled retraining run. </p> |
| `evaluation_data_end_time` | String | <p> Indicates the time reference in the dataset that was used to end the subset of
         evaluation data for the machine learning model. </p> |
| `labels_input_configuration` | String | <p>Specifies configuration information about the labels input, including its S3 location.
      </p> |
| `training_execution_start_time` | String | <p>Indicates the time at which the training of the machine learning model began. </p> |
| `dataset_arn` | String | <p>The Amazon Resouce Name (ARN) of the dataset used to create the machine learning model
         being described. </p> |
| `failed_reason` | String | <p>If the training of the machine learning model failed, this indicates the reason for that
         failure. </p> |
| `server_side_kms_key_id` | String | <p>Provides the identifier of the KMS key used to encrypt model data by Amazon Lookout
         for Equipment. </p> |
| `latest_scheduled_retraining_model_version` | i64 | <p>Indicates the most recent model version that was generated by retraining. </p> |
| `model_diagnostics_output_configuration` | String | <p>Configuration information for the model's pointwise model diagnostics.</p> |
| `latest_scheduled_retraining_failed_reason` | String | <p>If the model version was generated by retraining and the training failed, this indicates
         the reason for that failure. </p> |
| `off_condition` | String | <p>Indicates that the asset associated with this sensor has been shut off. As long as this
         condition is met, Lookout for Equipment will not use data from this asset for training, evaluation, or
         inference.</p> |
| `latest_scheduled_retraining_start_time` | String | <p>Indicates the start time of the most recent scheduled retraining run. </p> |
| `active_model_version_arn` | String | <p>The Amazon Resource Name (ARN) of the model version used by the inference scheduler when
         running a scheduled inference execution.</p> |
| `previous_active_model_version` | i64 | <p>The model version that was set as the active model version prior to the current active
         model version.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create model
model = provider.lookoutequipment.Model {
    client_token = "value"  # <p>A unique identifier for the request. If you do not set the client request token, Amazon
         Lookout for Equipment generates one. </p>
    dataset_name = "value"  # <p>The name of the dataset for the machine learning model being created. </p>
    model_name = "value"  # <p>The name for the machine learning model to be created.</p>
}

# Access model outputs
model_id = model.id
model_model_arn = model.model_arn
model_model_name = model.model_name
model_source_model_version_arn = model.source_model_version_arn
model_created_at = model.created_at
model_active_model_version = model.active_model_version
model_next_scheduled_retraining_start_date = model.next_scheduled_retraining_start_date
model_retraining_scheduler_status = model.retraining_scheduler_status
model_model_version_activated_at = model.model_version_activated_at
model_previous_model_version_activated_at = model.previous_model_version_activated_at
model_model_quality = model.model_quality
model_schema = model.schema
model_import_job_end_time = model.import_job_end_time
model_accumulated_inference_data_start_time = model.accumulated_inference_data_start_time
model_last_updated_time = model.last_updated_time
model_dataset_name = model.dataset_name
model_import_job_start_time = model.import_job_start_time
model_status = model.status
model_prior_model_metrics = model.prior_model_metrics
model_training_data_end_time = model.training_data_end_time
model_evaluation_data_start_time = model.evaluation_data_start_time
model_latest_scheduled_retraining_available_data_in_days = model.latest_scheduled_retraining_available_data_in_days
model_previous_active_model_version_arn = model.previous_active_model_version_arn
model_data_pre_processing_configuration = model.data_pre_processing_configuration
model_training_data_start_time = model.training_data_start_time
model_role_arn = model.role_arn
model_model_metrics = model.model_metrics
model_training_execution_end_time = model.training_execution_end_time
model_accumulated_inference_data_end_time = model.accumulated_inference_data_end_time
model_latest_scheduled_retraining_status = model.latest_scheduled_retraining_status
model_evaluation_data_end_time = model.evaluation_data_end_time
model_labels_input_configuration = model.labels_input_configuration
model_training_execution_start_time = model.training_execution_start_time
model_dataset_arn = model.dataset_arn
model_failed_reason = model.failed_reason
model_server_side_kms_key_id = model.server_side_kms_key_id
model_latest_scheduled_retraining_model_version = model.latest_scheduled_retraining_model_version
model_model_diagnostics_output_configuration = model.model_diagnostics_output_configuration
model_latest_scheduled_retraining_failed_reason = model.latest_scheduled_retraining_failed_reason
model_off_condition = model.off_condition
model_latest_scheduled_retraining_start_time = model.latest_scheduled_retraining_start_time
model_active_model_version_arn = model.active_model_version_arn
model_previous_active_model_version = model.previous_active_model_version
```

---


### Inference_scheduler

InferenceScheduler resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `data_output_configuration` | String | ✅ | <p>Specifies configuration information for the output results for the inference scheduler,
         including the S3 location for the output. </p> |
| `data_delay_offset_in_minutes` | i64 |  | <p>The interval (in minutes) of planned delay at the start of each inference segment. For
         example, if inference is set to run every ten minutes, the delay is set to five minutes and
         the time is 09:08. The inference scheduler will wake up at the configured interval (which,
         without a delay configured, would be 09:10) plus the additional five minute delay time (so
         09:15) to check your Amazon S3 bucket. The delay provides a buffer for you to upload data at the
         same frequency, so that you don't have to stop and restart the scheduler when uploading new
         data.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/lookout-for-equipment/latest/ug/understanding-inference-process.html">Understanding
            the inference process</a>.</p> |
| `role_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of a role with permission to access the data source being
         used for the inference. </p> |
| `tags` | Vec<String> |  | <p>Any tags associated with the inference scheduler. </p> |
| `client_token` | String | ✅ | <p> A unique identifier for the request. If you do not set the client request token, Amazon
         Lookout for Equipment generates one. </p> |
| `data_input_configuration` | String | ✅ | <p>Specifies configuration information for the input data for the inference scheduler,
         including delimiter, format, and dataset location. </p> |
| `inference_scheduler_name` | String | ✅ | <p>The name of the inference scheduler being created. </p> |
| `model_name` | String | ✅ | <p>The name of the previously trained machine learning model being used to create the
         inference scheduler. </p> |
| `data_upload_frequency` | String | ✅ | <p> How often data is uploaded to the source Amazon S3 bucket for the input data. The value
         chosen is the length of time between data uploads. For instance, if you select 5 minutes,
         Amazon Lookout for Equipment will upload the real-time data to the source bucket once every 5 minutes. This
         frequency also determines how often Amazon Lookout for Equipment runs inference on your data.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/lookout-for-equipment/latest/ug/understanding-inference-process.html">Understanding
            the inference process</a>.</p> |
| `server_side_kms_key_id` | String |  | <p>Provides the identifier of the KMS key used to encrypt inference scheduler data by
         Amazon Lookout for Equipment. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | String | <p>Indicates the status of the inference scheduler. </p> |
| `data_delay_offset_in_minutes` | i64 | <p> A period of time (in minutes) by which inference on the data is delayed after the data
         starts. For instance, if you select an offset delay time of five minutes, inference will
         not begin on the data until the first data measurement after the five minute mark. For
         example, if five minutes is selected, the inference scheduler will wake up at the
         configured frequency with the additional five minute delay time to check the customer S3
         bucket. The customer can upload data at the same frequency and they don't need to stop and
         restart the scheduler when uploading new data.</p> |
| `data_upload_frequency` | String | <p>Specifies how often data is uploaded to the source S3 bucket for the input data. This
         value is the length of time between data uploads. For instance, if you select 5 minutes,
         Amazon Lookout for Equipment will upload the real-time data to the source bucket once every 5 minutes. This
         frequency also determines how often Amazon Lookout for Equipment starts a scheduled inference on your data. In
         this example, it starts once every 5 minutes. </p> |
| `created_at` | String | <p>Specifies the time at which the inference scheduler was created. </p> |
| `data_input_configuration` | String | <p> Specifies configuration information for the input data for the inference scheduler,
         including delimiter, format, and dataset location. </p> |
| `model_arn` | String | <p>The Amazon Resource Name (ARN) of the machine learning model of the inference scheduler
         being described. </p> |
| `role_arn` | String | <p> The Amazon Resource Name (ARN) of a role with permission to access the data source for
         the inference scheduler being described. </p> |
| `inference_scheduler_name` | String | <p>The name of the inference scheduler being described. </p> |
| `updated_at` | String | <p>Specifies the time at which the inference scheduler was last updated, if it was. </p> |
| `data_output_configuration` | String | <p> Specifies information for the output results for the inference scheduler, including
         the output S3 location. </p> |
| `inference_scheduler_arn` | String | <p>The Amazon Resource Name (ARN) of the inference scheduler being described. </p> |
| `latest_inference_result` | String | <p>Indicates whether the latest execution for the inference scheduler was Anomalous
         (anomalous events found) or Normal (no anomalous events found).</p> |
| `server_side_kms_key_id` | String | <p>Provides the identifier of the KMS key used to encrypt inference scheduler data by
         Amazon Lookout for Equipment. </p> |
| `model_name` | String | <p>The name of the machine learning model of the inference scheduler being described.
      </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create inference_scheduler
inference_scheduler = provider.lookoutequipment.Inference_scheduler {
    data_output_configuration = "value"  # <p>Specifies configuration information for the output results for the inference scheduler,
         including the S3 location for the output. </p>
    role_arn = "value"  # <p>The Amazon Resource Name (ARN) of a role with permission to access the data source being
         used for the inference. </p>
    client_token = "value"  # <p> A unique identifier for the request. If you do not set the client request token, Amazon
         Lookout for Equipment generates one. </p>
    data_input_configuration = "value"  # <p>Specifies configuration information for the input data for the inference scheduler,
         including delimiter, format, and dataset location. </p>
    inference_scheduler_name = "value"  # <p>The name of the inference scheduler being created. </p>
    model_name = "value"  # <p>The name of the previously trained machine learning model being used to create the
         inference scheduler. </p>
    data_upload_frequency = "value"  # <p> How often data is uploaded to the source Amazon S3 bucket for the input data. The value
         chosen is the length of time between data uploads. For instance, if you select 5 minutes,
         Amazon Lookout for Equipment will upload the real-time data to the source bucket once every 5 minutes. This
         frequency also determines how often Amazon Lookout for Equipment runs inference on your data.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/lookout-for-equipment/latest/ug/understanding-inference-process.html">Understanding
            the inference process</a>.</p>
}

# Access inference_scheduler outputs
inference_scheduler_id = inference_scheduler.id
inference_scheduler_status = inference_scheduler.status
inference_scheduler_data_delay_offset_in_minutes = inference_scheduler.data_delay_offset_in_minutes
inference_scheduler_data_upload_frequency = inference_scheduler.data_upload_frequency
inference_scheduler_created_at = inference_scheduler.created_at
inference_scheduler_data_input_configuration = inference_scheduler.data_input_configuration
inference_scheduler_model_arn = inference_scheduler.model_arn
inference_scheduler_role_arn = inference_scheduler.role_arn
inference_scheduler_inference_scheduler_name = inference_scheduler.inference_scheduler_name
inference_scheduler_updated_at = inference_scheduler.updated_at
inference_scheduler_data_output_configuration = inference_scheduler.data_output_configuration
inference_scheduler_inference_scheduler_arn = inference_scheduler.inference_scheduler_arn
inference_scheduler_latest_inference_result = inference_scheduler.latest_inference_result
inference_scheduler_server_side_kms_key_id = inference_scheduler.server_side_kms_key_id
inference_scheduler_model_name = inference_scheduler.model_name
```

---


### Dataset

Dataset resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `server_side_kms_key_id` | String |  | <p>Provides the identifier of the KMS key used to encrypt dataset data by Amazon Lookout
         for Equipment. </p> |
| `client_token` | String | ✅ | <p> A unique identifier for the request. If you do not set the client request token, Amazon
         Lookout for Equipment generates one. </p> |
| `dataset_name` | String | ✅ | <p>The name of the dataset being created. </p> |
| `tags` | Vec<String> |  | <p>Any tags associated with the ingested data described in the dataset. </p> |
| `dataset_schema` | String |  | <p>A JSON description of the data that is in each time series dataset, including names,
         column names, and data types. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `dataset_name` | String | <p>The name of the dataset being described. </p> |
| `data_start_time` | String | <p> Indicates the earliest timestamp corresponding to data that was successfully ingested
         during the most recent ingestion of this particular dataset. </p> |
| `schema` | String | <p>A JSON description of the data that is in each time series dataset, including names,
         column names, and data types. </p> |
| `ingestion_input_configuration` | String | <p>Specifies the S3 location configuration for the data input for the data ingestion job.
      </p> |
| `ingested_files_summary` | String | <p>IngestedFilesSummary associated with the given dataset for the latest successful
         associated ingestion job id. </p> |
| `data_quality_summary` | String | <p> Gives statistics associated with the given dataset for the latest successful associated
         ingestion job id. These statistics primarily relate to quantifying incorrect data such as
         MissingCompleteSensorData, MissingSensorData, UnsupportedDateFormats,
         InsufficientSensorData, and DuplicateTimeStamps. </p> |
| `created_at` | String | <p>Specifies the time the dataset was created in Lookout for Equipment. </p> |
| `data_end_time` | String | <p> Indicates the latest timestamp corresponding to data that was successfully ingested
         during the most recent ingestion of this particular dataset. </p> |
| `source_dataset_arn` | String | <p>The Amazon Resource Name (ARN) of the source dataset from which the current data being
         described was imported from.</p> |
| `last_updated_at` | String | <p>Specifies the time the dataset was last updated, if it was. </p> |
| `server_side_kms_key_id` | String | <p>Provides the identifier of the KMS key used to encrypt dataset data by Amazon Lookout
         for Equipment. </p> |
| `role_arn` | String | <p> The Amazon Resource Name (ARN) of the IAM role that you are using for this the data
         ingestion job. </p> |
| `dataset_arn` | String | <p>The Amazon Resource Name (ARN) of the dataset being described. </p> |
| `status` | String | <p>Indicates the status of the dataset. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create dataset
dataset = provider.lookoutequipment.Dataset {
    client_token = "value"  # <p> A unique identifier for the request. If you do not set the client request token, Amazon
         Lookout for Equipment generates one. </p>
    dataset_name = "value"  # <p>The name of the dataset being created. </p>
}

# Access dataset outputs
dataset_id = dataset.id
dataset_dataset_name = dataset.dataset_name
dataset_data_start_time = dataset.data_start_time
dataset_schema = dataset.schema
dataset_ingestion_input_configuration = dataset.ingestion_input_configuration
dataset_ingested_files_summary = dataset.ingested_files_summary
dataset_data_quality_summary = dataset.data_quality_summary
dataset_created_at = dataset.created_at
dataset_data_end_time = dataset.data_end_time
dataset_source_dataset_arn = dataset.source_dataset_arn
dataset_last_updated_at = dataset.last_updated_at
dataset_server_side_kms_key_id = dataset.server_side_kms_key_id
dataset_role_arn = dataset.role_arn
dataset_dataset_arn = dataset.dataset_arn
dataset_status = dataset.status
```

---


### Retraining_scheduler

RetrainingScheduler resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `retraining_frequency` | String | ✅ | <p>This parameter uses the <a href="https://en.wikipedia.org/wiki/ISO_8601#Durations">ISO 8601</a> standard to set the frequency at which you want retraining to occur in
         terms of Years, Months, and/or Days (note: other parameters like Time are not currently
         supported). The minimum value is 30 days (P30D) and the maximum value is 1 year (P1Y). For
         example, the following values are valid:</p>
         <ul>
            <li>
               <p>P3M15D – Every 3 months and 15 days</p>
            </li>
            <li>
               <p>P2M – Every 2 months</p>
            </li>
            <li>
               <p>P150D – Every 150 days</p>
            </li>
         </ul> |
| `promote_mode` | String |  | <p>Indicates how the service will use new models. In <code>MANAGED</code> mode, new models
         will automatically be used for inference if they have better performance than the current
         model. In <code>MANUAL</code> mode, the new models will not be used <a href="https://docs.aws.amazon.com/lookout-for-equipment/latest/ug/versioning-model.html#model-activation">until they
            are manually activated</a>.</p> |
| `lookback_window` | String | ✅ | <p>The number of past days of data that will be used for retraining.</p> |
| `retraining_start_date` | String |  | <p>The start date for the retraining scheduler. Lookout for Equipment truncates the time you provide to the
         nearest UTC day.</p> |
| `model_name` | String | ✅ | <p>The name of the model to add the retraining scheduler to. </p> |
| `client_token` | String | ✅ | <p>A unique identifier for the request. If you do not set the client request token, Amazon
         Lookout for Equipment generates one. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `created_at` | String | <p>Indicates the time and date at which the retraining scheduler was created. </p> |
| `retraining_start_date` | String | <p>The start date for the retraining scheduler. Lookout for Equipment truncates the time you provide to the
         nearest UTC day.</p> |
| `lookback_window` | String | <p>The number of past days of data used for retraining.</p> |
| `model_name` | String | <p>The name of the model that the retraining scheduler is attached to. </p> |
| `status` | String | <p>The status of the retraining scheduler. </p> |
| `model_arn` | String | <p>The ARN of the model that the retraining scheduler is attached to. </p> |
| `promote_mode` | String | <p>Indicates how the service uses new models. In <code>MANAGED</code> mode, new models are
         used for inference if they have better performance than the current model. In
            <code>MANUAL</code> mode, the new models are not used until they are <a href="https://docs.aws.amazon.com/lookout-for-equipment/latest/ug/versioning-model.html#model-activation">manually
            activated</a>.</p> |
| `retraining_frequency` | String | <p>The frequency at which the model retraining is set. This follows the <a href="https://en.wikipedia.org/wiki/ISO_8601#Durations">ISO 8601</a>
         guidelines.</p> |
| `updated_at` | String | <p>Indicates the time and date at which the retraining scheduler was updated. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create retraining_scheduler
retraining_scheduler = provider.lookoutequipment.Retraining_scheduler {
    retraining_frequency = "value"  # <p>This parameter uses the <a href="https://en.wikipedia.org/wiki/ISO_8601#Durations">ISO 8601</a> standard to set the frequency at which you want retraining to occur in
         terms of Years, Months, and/or Days (note: other parameters like Time are not currently
         supported). The minimum value is 30 days (P30D) and the maximum value is 1 year (P1Y). For
         example, the following values are valid:</p>
         <ul>
            <li>
               <p>P3M15D – Every 3 months and 15 days</p>
            </li>
            <li>
               <p>P2M – Every 2 months</p>
            </li>
            <li>
               <p>P150D – Every 150 days</p>
            </li>
         </ul>
    lookback_window = "value"  # <p>The number of past days of data that will be used for retraining.</p>
    model_name = "value"  # <p>The name of the model to add the retraining scheduler to. </p>
    client_token = "value"  # <p>A unique identifier for the request. If you do not set the client request token, Amazon
         Lookout for Equipment generates one. </p>
}

# Access retraining_scheduler outputs
retraining_scheduler_id = retraining_scheduler.id
retraining_scheduler_created_at = retraining_scheduler.created_at
retraining_scheduler_retraining_start_date = retraining_scheduler.retraining_start_date
retraining_scheduler_lookback_window = retraining_scheduler.lookback_window
retraining_scheduler_model_name = retraining_scheduler.model_name
retraining_scheduler_status = retraining_scheduler.status
retraining_scheduler_model_arn = retraining_scheduler.model_arn
retraining_scheduler_promote_mode = retraining_scheduler.promote_mode
retraining_scheduler_retraining_frequency = retraining_scheduler.retraining_frequency
retraining_scheduler_updated_at = retraining_scheduler.updated_at
```

---


### Data_ingestion_job

DataIngestionJob resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `data_quality_summary` | String | <p> Gives statistics about a completed ingestion job. These statistics primarily relate to
         quantifying incorrect data such as MissingCompleteSensorData, MissingSensorData,
         UnsupportedDateFormats, InsufficientSensorData, and DuplicateTimeStamps. </p> |
| `role_arn` | String | <p>The Amazon Resource Name (ARN) of an IAM role with permission to access the data source
         being ingested. </p> |
| `failed_reason` | String | <p>Specifies the reason for failure when a data ingestion job has failed. </p> |
| `source_dataset_arn` | String | <p>The Amazon Resource Name (ARN) of the source dataset from which the data used for the
         data ingestion job was imported from.</p> |
| `ingested_files_summary` | String |  |
| `status` | String | <p>Indicates the status of the <code>DataIngestionJob</code> operation. </p> |
| `ingestion_input_configuration` | String | <p>Specifies the S3 location configuration for the data input for the data ingestion job.
      </p> |
| `data_start_time` | String | <p> Indicates the earliest timestamp corresponding to data that was successfully ingested
         during this specific ingestion job. </p> |
| `dataset_arn` | String | <p>The Amazon Resource Name (ARN) of the dataset being used in the data ingestion job.
      </p> |
| `job_id` | String | <p>Indicates the job ID of the data ingestion job. </p> |
| `created_at` | String | <p>The time at which the data ingestion job was created. </p> |
| `status_detail` | String | <p> Provides details about status of the ingestion job that is currently in progress.
      </p> |
| `ingested_data_size` | i64 | <p> Indicates the size of the ingested dataset. </p> |
| `data_end_time` | String | <p> Indicates the latest timestamp corresponding to data that was successfully ingested
         during this specific ingestion job. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access data_ingestion_job outputs
data_ingestion_job_id = data_ingestion_job.id
data_ingestion_job_data_quality_summary = data_ingestion_job.data_quality_summary
data_ingestion_job_role_arn = data_ingestion_job.role_arn
data_ingestion_job_failed_reason = data_ingestion_job.failed_reason
data_ingestion_job_source_dataset_arn = data_ingestion_job.source_dataset_arn
data_ingestion_job_ingested_files_summary = data_ingestion_job.ingested_files_summary
data_ingestion_job_status = data_ingestion_job.status
data_ingestion_job_ingestion_input_configuration = data_ingestion_job.ingestion_input_configuration
data_ingestion_job_data_start_time = data_ingestion_job.data_start_time
data_ingestion_job_dataset_arn = data_ingestion_job.dataset_arn
data_ingestion_job_job_id = data_ingestion_job.job_id
data_ingestion_job_created_at = data_ingestion_job.created_at
data_ingestion_job_status_detail = data_ingestion_job.status_detail
data_ingestion_job_ingested_data_size = data_ingestion_job.ingested_data_size
data_ingestion_job_data_end_time = data_ingestion_job.data_end_time
```

---


### Label_group

LabelGroup resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p> Tags that provide metadata about the label group you are creating. </p>
         <p>Data in this field will be retained for service usage. Follow best practices for the
         security of your data.</p> |
| `client_token` | String | ✅ | <p> A unique identifier for the request to create a label group. If you do not set the
         client request token, Lookout for Equipment generates one. </p> |
| `label_group_name` | String | ✅ | <p> Names a group of labels.</p>
         <p>Data in this field will be retained for service usage. Follow best practices for the
         security of your data. </p> |
| `fault_codes` | Vec<String> |  | <p> The acceptable fault codes (indicating the type of anomaly associated with the label)
         that can be used with this label group.</p>
         <p>Data in this field will be retained for service usage. Follow best practices for the
         security of your data.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `label_group_name` | String | <p> The name of the label group. </p> |
| `label_group_arn` | String | <p> The Amazon Resource Name (ARN) of the label group. </p> |
| `fault_codes` | Vec<String> | <p> Codes indicating the type of anomaly associated with the labels in the lagbel group.
      </p> |
| `created_at` | String | <p> The time at which the label group was created. </p> |
| `updated_at` | String | <p> The time at which the label group was updated. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create label_group
label_group = provider.lookoutequipment.Label_group {
    client_token = "value"  # <p> A unique identifier for the request to create a label group. If you do not set the
         client request token, Lookout for Equipment generates one. </p>
    label_group_name = "value"  # <p> Names a group of labels.</p>
         <p>Data in this field will be retained for service usage. Follow best practices for the
         security of your data. </p>
}

# Access label_group outputs
label_group_id = label_group.id
label_group_label_group_name = label_group.label_group_name
label_group_label_group_arn = label_group.label_group_arn
label_group_fault_codes = label_group.fault_codes
label_group_created_at = label_group.created_at
label_group_updated_at = label_group.updated_at
```

---


### Model_version

ModelVersion resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `model_version_arn` | String | <p>The Amazon Resource Name (ARN) of the model version.</p> |
| `data_pre_processing_configuration` | String |  |
| `evaluation_data_end_time` | String | <p>The date on which the data in the evaluation set began being gathered. If you imported
         the version, this is the date that the evaluation set data in the source version finished
         being gathered.</p> |
| `source_model_version_arn` | String | <p>If model version was imported, then this field is the arn of the source model
         version.</p> |
| `dataset_name` | String | <p>The name of the dataset used to train the model version.</p> |
| `model_version` | i64 | <p>The version of the machine learning model.</p> |
| `training_data_end_time` | String | <p>The date on which the training data finished being gathered. If you imported the
         version, this is the date that the training data in the source version finished being
         gathered.</p> |
| `role_arn` | String | <p>The Amazon Resource Name (ARN) of the role that was used to train the model
         version.</p> |
| `off_condition` | String | <p>Indicates that the asset associated with this sensor has been shut off. As long as this
         condition is met, Lookout for Equipment will not use data from this asset for training,
         evaluation, or inference.</p> |
| `status` | String | <p>The current status of the model version.</p> |
| `model_arn` | String | <p>The Amazon Resource Name (ARN) of the parent machine learning model that this version
         belong to.</p> |
| `source_type` | String | <p>Indicates whether this model version was created by training or by importing.</p> |
| `training_execution_start_time` | String | <p>The time when the training of the version began.</p> |
| `created_at` | String | <p>Indicates the time and date at which the machine learning model version was
         created.</p> |
| `server_side_kms_key_id` | String | <p>The identifier of the KMS key key used to encrypt model version data by
         Amazon Lookout for Equipment.</p> |
| `prior_model_metrics` | String | <p>If the model version was retrained, this field shows a summary of the performance of the
         prior model on the new training range. You can use the information in this JSON-formatted
         object to compare the new model version and the prior model version.</p> |
| `import_job_end_time` | String | <p>The date and time when the import job completed. This field appears if the model version
         was imported.</p> |
| `training_data_start_time` | String | <p>The date on which the training data began being gathered. If you imported the version,
         this is the date that the training data in the source version began being gathered.</p> |
| `model_metrics` | String | <p>Shows an aggregated summary, in JSON format, of the model's performance within the
         evaluation time range. These metrics are created when evaluating the model.</p> |
| `retraining_available_data_in_days` | i64 | <p>Indicates the number of days of data used in the most recent scheduled retraining run.
      </p> |
| `model_name` | String | <p>The name of the machine learning model that this version belongs to.</p> |
| `training_execution_end_time` | String | <p>The time when the training of the version completed.</p> |
| `model_diagnostics_results_object` | String | <p>The Amazon S3 output prefix for where Lookout for Equipment saves the pointwise model diagnostics for the model version.</p> |
| `model_quality` | String | <p>Provides a quality assessment for a model that uses labels. If Lookout for Equipment determines that the
         model quality is poor based on training metrics, the value is
         <code>POOR_QUALITY_DETECTED</code>. Otherwise, the value is
         <code>QUALITY_THRESHOLD_MET</code>.</p>
         <p>If the model is unlabeled, the model quality can't
         be assessed and the value of <code>ModelQuality</code> is
         <code>CANNOT_DETERMINE_QUALITY</code>. In this situation, you can get a model quality
         assessment by adding labels to the input dataset and retraining the model.</p>
         <p>For information about using labels with your models, see <a href="https://docs.aws.amazon.com/lookout-for-equipment/latest/ug/understanding-labeling.html">Understanding labeling</a>.</p>
         <p>For information about improving the quality of a model, see <a href="https://docs.aws.amazon.com/lookout-for-equipment/latest/ug/best-practices.html">Best practices with
         Amazon Lookout for Equipment</a>.</p> |
| `import_job_start_time` | String | <p>The date and time when the import job began. This field appears if the model version was
         imported.</p> |
| `failed_reason` | String | <p>The failure message if the training of the model version failed.</p> |
| `model_diagnostics_output_configuration` | String | <p>The Amazon S3 location where Amazon Lookout for Equipment saves the pointwise model diagnostics for the model version.</p> |
| `auto_promotion_result_reason` | String | <p>Indicates the reason for the <code>AutoPromotionResult</code>. For example, a model
         might not be promoted if its performance was worse than the active version, if there was an
         error during training, or if the retraining scheduler was using <code>MANUAL</code> promote
         mode. The model will be promoted in <code>MANAGED</code> promote mode if the performance is
         better than the previous model. </p> |
| `dataset_arn` | String | <p>The Amazon Resource Name (ARN) of the dataset used to train the model version.</p> |
| `labels_input_configuration` | String |  |
| `imported_data_size_in_bytes` | i64 | <p>The size in bytes of the imported data. This field appears if the model version was
         imported.</p> |
| `evaluation_data_start_time` | String | <p>The date on which the data in the evaluation set began being gathered. If you imported
         the version, this is the date that the evaluation set data in the source version began
         being gathered.</p> |
| `auto_promotion_result` | String | <p>Indicates whether the model version was promoted to be the active version after
         retraining or if there was an error with or cancellation of the retraining. </p> |
| `schema` | String | <p>The schema of the data used to train the model version.</p> |
| `last_updated_time` | String | <p>Indicates the last time the machine learning model version was updated.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access model_version outputs
model_version_id = model_version.id
model_version_model_version_arn = model_version.model_version_arn
model_version_data_pre_processing_configuration = model_version.data_pre_processing_configuration
model_version_evaluation_data_end_time = model_version.evaluation_data_end_time
model_version_source_model_version_arn = model_version.source_model_version_arn
model_version_dataset_name = model_version.dataset_name
model_version_model_version = model_version.model_version
model_version_training_data_end_time = model_version.training_data_end_time
model_version_role_arn = model_version.role_arn
model_version_off_condition = model_version.off_condition
model_version_status = model_version.status
model_version_model_arn = model_version.model_arn
model_version_source_type = model_version.source_type
model_version_training_execution_start_time = model_version.training_execution_start_time
model_version_created_at = model_version.created_at
model_version_server_side_kms_key_id = model_version.server_side_kms_key_id
model_version_prior_model_metrics = model_version.prior_model_metrics
model_version_import_job_end_time = model_version.import_job_end_time
model_version_training_data_start_time = model_version.training_data_start_time
model_version_model_metrics = model_version.model_metrics
model_version_retraining_available_data_in_days = model_version.retraining_available_data_in_days
model_version_model_name = model_version.model_name
model_version_training_execution_end_time = model_version.training_execution_end_time
model_version_model_diagnostics_results_object = model_version.model_diagnostics_results_object
model_version_model_quality = model_version.model_quality
model_version_import_job_start_time = model_version.import_job_start_time
model_version_failed_reason = model_version.failed_reason
model_version_model_diagnostics_output_configuration = model_version.model_diagnostics_output_configuration
model_version_auto_promotion_result_reason = model_version.auto_promotion_result_reason
model_version_dataset_arn = model_version.dataset_arn
model_version_labels_input_configuration = model_version.labels_input_configuration
model_version_imported_data_size_in_bytes = model_version.imported_data_size_in_bytes
model_version_evaluation_data_start_time = model_version.evaluation_data_start_time
model_version_auto_promotion_result = model_version.auto_promotion_result
model_version_schema = model_version.schema
model_version_last_updated_time = model_version.last_updated_time
```

---


### Label

Label resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `end_time` | String | ✅ | <p> The end time of the labeled event. </p> |
| `equipment` | String |  | <p> Indicates that a label pertains to a particular piece of equipment. </p>
         <p>Data in this field will be retained for service usage. Follow best practices for the
         security of your data.</p> |
| `client_token` | String | ✅ | <p> A unique identifier for the request to create a label. If you do not set the client
         request token, Lookout for Equipment generates one. </p> |
| `label_group_name` | String | ✅ | <p> The name of a group of labels. </p>
         <p>Data in this field will be retained for service usage. Follow best practices for the
         security of your data. </p> |
| `notes` | String |  | <p> Metadata providing additional information about the label. </p>
         <p>Data in this field will be retained for service usage. Follow best practices for the
         security of your data.</p> |
| `fault_code` | String |  | <p> Provides additional information about the label. The fault code must be defined in the
         FaultCodes attribute of the label group.</p>
         <p>Data in this field will be retained for service usage. Follow best practices for the
         security of your data. </p> |
| `start_time` | String | ✅ | <p> The start time of the labeled event. </p> |
| `rating` | String | ✅ | <p> Indicates whether a labeled event represents an anomaly. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `label_group_name` | String | <p> The name of the requested label group. </p> |
| `rating` | String | <p> Indicates whether a labeled event represents an anomaly. </p> |
| `label_group_arn` | String | <p> The Amazon Resource Name (ARN) of the requested label group. </p> |
| `label_id` | String | <p> The ID of the requested label. </p> |
| `fault_code` | String | <p> Indicates the type of anomaly associated with the label. </p>
         <p>Data in this field will be retained for service usage. Follow best practices for the
         security of your data.</p> |
| `start_time` | String | <p> The start time of the requested label. </p> |
| `notes` | String | <p>Metadata providing additional information about the label.</p>
         <p>Data in this field will be retained for service usage. Follow best practices for the
         security of your data.</p> |
| `created_at` | String | <p> The time at which the label was created. </p> |
| `end_time` | String | <p> The end time of the requested label. </p> |
| `equipment` | String | <p> Indicates that a label pertains to a particular piece of equipment. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create label
label = provider.lookoutequipment.Label {
    end_time = "value"  # <p> The end time of the labeled event. </p>
    client_token = "value"  # <p> A unique identifier for the request to create a label. If you do not set the client
         request token, Lookout for Equipment generates one. </p>
    label_group_name = "value"  # <p> The name of a group of labels. </p>
         <p>Data in this field will be retained for service usage. Follow best practices for the
         security of your data. </p>
    start_time = "value"  # <p> The start time of the labeled event. </p>
    rating = "value"  # <p> Indicates whether a labeled event represents an anomaly. </p>
}

# Access label outputs
label_id = label.id
label_label_group_name = label.label_group_name
label_rating = label.rating
label_label_group_arn = label.label_group_arn
label_label_id = label.label_id
label_fault_code = label.fault_code
label_start_time = label.start_time
label_notes = label.notes
label_created_at = label.created_at
label_end_time = label.end_time
label_equipment = label.equipment
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple resource_policy resources
resource_policy_0 = provider.lookoutequipment.Resource_policy {
    resource_arn = "value-0"
    client_token = "value-0"
    resource_policy = "value-0"
}
resource_policy_1 = provider.lookoutequipment.Resource_policy {
    resource_arn = "value-1"
    client_token = "value-1"
    resource_policy = "value-1"
}
resource_policy_2 = provider.lookoutequipment.Resource_policy {
    resource_arn = "value-2"
    client_token = "value-2"
    resource_policy = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    resource_policy = provider.lookoutequipment.Resource_policy {
        resource_arn = "production-value"
        client_token = "production-value"
        resource_policy = "production-value"
    }
```

---

## Related Documentation

- [AWS Lookoutequipment Documentation](https://docs.aws.amazon.com/lookoutequipment/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
