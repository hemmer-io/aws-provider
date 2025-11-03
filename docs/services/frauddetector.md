# Frauddetector Service



**Resources**: 40

---

## Overview

The frauddetector service provides access to 40 resource types:

- [Events_by_event_type](#events_by_event_type) [D]
- [Model_version_status](#model_version_status) [U]
- [Label](#label) [CD]
- [Labels](#labels) [R]
- [Event_prediction_metadata](#event_prediction_metadata) [R]
- [List](#list) [CUD]
- [Event_type](#event_type) [CD]
- [Batch_import_jobs](#batch_import_jobs) [R]
- [Event_types](#event_types) [R]
- [Event_label](#event_label) [U]
- [Models](#models) [R]
- [Model_versions](#model_versions) [R]
- [Model](#model) [CUD]
- [Batch_prediction_job](#batch_prediction_job) [CD]
- [Outcome](#outcome) [CD]
- [Rule_metadata](#rule_metadata) [U]
- [Batch_import_job](#batch_import_job) [CD]
- [Kms_encryption_key](#kms_encryption_key) [CR]
- [Detector_version](#detector_version) [CRUD]
- [Variable](#variable) [CUD]
- [Variables](#variables) [R]
- [Detectors](#detectors) [R]
- [Rule](#rule) [CD]
- [Detector](#detector) [CRD]
- [External_model](#external_model) [CD]
- [External_models](#external_models) [R]
- [List_elements](#list_elements) [R]
- [Rules](#rules) [R]
- [Outcomes](#outcomes) [R]
- [Entity_type](#entity_type) [CD]
- [Event_prediction](#event_prediction) [R]
- [Rule_version](#rule_version) [U]
- [Entity_types](#entity_types) [R]
- [Detector_version_status](#detector_version_status) [U]
- [Event](#event) [RD]
- [Detector_version_metadata](#detector_version_metadata) [U]
- [Delete_events_by_event_type_status](#delete_events_by_event_type_status) [R]
- [Batch_prediction_jobs](#batch_prediction_jobs) [R]
- [Lists_metadata](#lists_metadata) [R]
- [Model_version](#model_version) [CRUD]

---

## Resources


### Events_by_event_type

EventsByEventType resource

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


### Model_version_status

ModelVersionStatus resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `status` | String | ✅ | <p>The model version status.</p> |
| `model_id` | String | ✅ | <p>The model ID of the model version to update.</p> |
| `model_type` | String | ✅ | <p>The model type.</p> |
| `model_version_number` | String | ✅ | <p>The model version number.</p> |



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


### Label

Label resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | <p>The label description.</p> |
| `tags` | Vec<String> |  | <p>A collection of key and value pairs.</p> |
| `name` | String | ✅ | <p>The label name.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create label
label = provider.frauddetector.Label {
    name = "value"  # <p>The label name.</p>
}

```

---


### Labels

Labels resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The next page token.</p> |
| `labels` | Vec<String> | <p>An array of labels.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access labels outputs
labels_id = labels.id
labels_next_token = labels.next_token
labels_labels = labels.labels
```

---


### Event_prediction_metadata

EventPredictionMetadata resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `event_id` | String | <p>
The event ID.
</p> |
| `event_timestamp` | String | <p>
The timestamp for when the prediction was generated for the associated event ID.
</p> |
| `evaluated_model_versions` | Vec<String> | <p>
Model versions that were evaluated for generating predictions.
</p> |
| `entity_type` | String | <p>
The entity type.
</p> |
| `prediction_timestamp` | String | <p>The timestamp that defines when the prediction was generated. </p> |
| `rules` | Vec<String> | <p>
List of rules associated with the detector version that were used for evaluating variable values.
</p> |
| `rule_execution_mode` | String | <p>
The execution mode of the rule used for evaluating variable values.
</p> |
| `detector_version_status` | String | <p>
The status of the detector version.
</p> |
| `entity_id` | String | <p>
The entity ID.
</p> |
| `outcomes` | Vec<String> | <p>
The outcomes of the matched rule, based on the rule execution mode.
</p> |
| `event_type_name` | String | <p>
The event type associated with the detector specified for this prediction.
</p> |
| `detector_version_id` | String | <p>
The detector version ID.
</p> |
| `evaluated_external_models` | Vec<String> | <p>
External (Amazon SageMaker) models that were evaluated for generating predictions.
</p> |
| `event_variables` | Vec<String> | <p>
A list of event variables that influenced the prediction scores.
</p> |
| `detector_id` | String | <p>
The detector ID.
</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access event_prediction_metadata outputs
event_prediction_metadata_id = event_prediction_metadata.id
event_prediction_metadata_event_id = event_prediction_metadata.event_id
event_prediction_metadata_event_timestamp = event_prediction_metadata.event_timestamp
event_prediction_metadata_evaluated_model_versions = event_prediction_metadata.evaluated_model_versions
event_prediction_metadata_entity_type = event_prediction_metadata.entity_type
event_prediction_metadata_prediction_timestamp = event_prediction_metadata.prediction_timestamp
event_prediction_metadata_rules = event_prediction_metadata.rules
event_prediction_metadata_rule_execution_mode = event_prediction_metadata.rule_execution_mode
event_prediction_metadata_detector_version_status = event_prediction_metadata.detector_version_status
event_prediction_metadata_entity_id = event_prediction_metadata.entity_id
event_prediction_metadata_outcomes = event_prediction_metadata.outcomes
event_prediction_metadata_event_type_name = event_prediction_metadata.event_type_name
event_prediction_metadata_detector_version_id = event_prediction_metadata.detector_version_id
event_prediction_metadata_evaluated_external_models = event_prediction_metadata.evaluated_external_models
event_prediction_metadata_event_variables = event_prediction_metadata.event_variables
event_prediction_metadata_detector_id = event_prediction_metadata.detector_id
```

---


### List

List resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `variable_type` | String |  | <p>
            The variable type of the list. You can only assign the variable type with String data type.  For more information, see 
            <a href="https://docs.aws.amazon.com/frauddetector/latest/ug/create-a-variable.html#variable-types">Variable types</a>.
        </p> |
| `name` | String | ✅ | <p>
            The name of the list.
        </p> |
| `elements` | Vec<String> |  | <p>
            The names of the elements, if providing.  You can also create an empty list and add elements later using the <a href="https://docs.aws.amazon.com/frauddetector/latest/api/API_Updatelist.html">UpdateList</a> API.
        </p> |
| `tags` | Vec<String> |  | <p>
            A collection of the key and value pairs.
        </p> |
| `description` | String |  | <p>
            The description of the list.
        </p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create list
list = provider.frauddetector.List {
    name = "value"  # <p>
            The name of the list.
        </p>
}

```

---


### Event_type

EventType resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `event_variables` | Vec<String> | ✅ | <p>The event type variables.</p> |
| `description` | String |  | <p>The description of the event type.</p> |
| `event_ingestion` | String |  | <p>Specifies if ingestion is enabled or disabled.</p> |
| `name` | String | ✅ | <p>The name.</p> |
| `event_orchestration` | String |  | <p>Enables or disables event orchestration. If enabled, you can send event predictions to select AWS services for downstream processing of the events.</p> |
| `labels` | Vec<String> |  | <p>The event type labels.</p> |
| `tags` | Vec<String> |  | <p>A collection of key and value pairs.</p> |
| `entity_types` | Vec<String> | ✅ | <p>The entity type for the event type. Example entity types: customer, merchant, account.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create event_type
event_type = provider.frauddetector.Event_type {
    event_variables = "value"  # <p>The event type variables.</p>
    name = "value"  # <p>The name.</p>
    entity_types = "value"  # <p>The entity type for the event type. Example entity types: customer, merchant, account.</p>
}

```

---


### Batch_import_jobs

BatchImportJobs resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The next token for the subsequent resquest.</p> |
| `batch_imports` | Vec<String> | <p>An array containing the details of each batch import job.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access batch_import_jobs outputs
batch_import_jobs_id = batch_import_jobs.id
batch_import_jobs_next_token = batch_import_jobs.next_token
batch_import_jobs_batch_imports = batch_import_jobs.batch_imports
```

---


### Event_types

EventTypes resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `event_types` | Vec<String> | <p>An array of event types.</p> |
| `next_token` | String | <p>The next page token.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access event_types outputs
event_types_id = event_types.id
event_types_event_types = event_types.event_types
event_types_next_token = event_types.next_token
```

---


### Event_label

EventLabel resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `label_timestamp` | String | ✅ | <p>The timestamp associated with the label. The timestamp must be specified using ISO 8601 standard in UTC. </p> |
| `assigned_label` | String | ✅ | <p>The new label to assign to the event.</p> |
| `event_type_name` | String | ✅ | <p>The event type of the event associated with the label to update.</p> |
| `event_id` | String | ✅ | <p>The ID of the event associated with the label to update.</p> |



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


### Models

Models resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The next page token to be used in subsequent requests.</p> |
| `models` | Vec<String> | <p>The array of models.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access models outputs
models_id = models.id
models_next_token = models.next_token
models_models = models.models
```

---


### Model_versions

ModelVersions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The next token.</p> |
| `model_version_details` | Vec<String> | <p>The model version details.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access model_versions outputs
model_versions_id = model_versions.id
model_versions_next_token = model_versions.next_token
model_versions_model_version_details = model_versions.model_version_details
```

---


### Model

Model resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>A collection of key and value pairs.</p> |
| `model_id` | String | ✅ | <p>The model ID.</p> |
| `description` | String |  | <p>The model description. </p> |
| `model_type` | String | ✅ | <p>The model type. </p> |
| `event_type_name` | String | ✅ | <p>The name of the event type.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create model
model = provider.frauddetector.Model {
    model_id = "value"  # <p>The model ID.</p>
    model_type = "value"  # <p>The model type. </p>
    event_type_name = "value"  # <p>The name of the event type.</p>
}

```

---


### Batch_prediction_job

BatchPredictionJob resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `iam_role_arn` | String | ✅ | <p>The ARN of the IAM role to use for this job request.</p>
         <p>The IAM Role must have read permissions to your input S3 bucket and write permissions to your output S3 bucket.
         For more information about bucket permissions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/example-policies-s3.html">User policy examples</a> in the 
         <i>Amazon S3 User Guide</i>.</p> |
| `detector_name` | String | ✅ | <p>The name of the detector.</p> |
| `output_path` | String | ✅ | <p>The Amazon S3 location of your output file.</p> |
| `detector_version` | String |  | <p>The detector version.</p> |
| `input_path` | String | ✅ | <p>The Amazon S3 location of your training file.</p> |
| `job_id` | String | ✅ | <p>The ID of the batch prediction job.</p> |
| `tags` | Vec<String> |  | <p>A collection of key and value pairs.</p> |
| `event_type_name` | String | ✅ | <p>The name of the event type.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create batch_prediction_job
batch_prediction_job = provider.frauddetector.Batch_prediction_job {
    iam_role_arn = "value"  # <p>The ARN of the IAM role to use for this job request.</p>
         <p>The IAM Role must have read permissions to your input S3 bucket and write permissions to your output S3 bucket.
         For more information about bucket permissions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/example-policies-s3.html">User policy examples</a> in the 
         <i>Amazon S3 User Guide</i>.</p>
    detector_name = "value"  # <p>The name of the detector.</p>
    output_path = "value"  # <p>The Amazon S3 location of your output file.</p>
    input_path = "value"  # <p>The Amazon S3 location of your training file.</p>
    job_id = "value"  # <p>The ID of the batch prediction job.</p>
    event_type_name = "value"  # <p>The name of the event type.</p>
}

```

---


### Outcome

Outcome resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>A collection of key and value pairs.</p> |
| `description` | String |  | <p>The outcome description.</p> |
| `name` | String | ✅ | <p>The name of the outcome.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create outcome
outcome = provider.frauddetector.Outcome {
    name = "value"  # <p>The name of the outcome.</p>
}

```

---


### Rule_metadata

RuleMetadata resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String | ✅ | <p>The rule description.</p> |
| `rule` | String | ✅ | <p>The rule to update.</p> |



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


### Batch_import_job

BatchImportJob resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `job_id` | String | ✅ | <p>The ID of the batch import job. The ID cannot be of a past job, unless the job exists in <code>CREATE_FAILED</code> state.</p> |
| `tags` | Vec<String> |  | <p>A collection of key-value pairs associated with this request.  </p> |
| `event_type_name` | String | ✅ | <p>The name of the event type.</p> |
| `output_path` | String | ✅ | <p>The URI that points to the Amazon S3 location for storing your results. </p> |
| `iam_role_arn` | String | ✅ | <p>The ARN of the IAM role created for Amazon S3 bucket that holds your data file.</p>
         <p>The IAM role must have read permissions to your input S3 bucket and write permissions to your output S3 bucket.
         For more information about bucket permissions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/example-policies-s3.html">User policy examples</a> in the 
         <i>Amazon S3 User Guide</i>.</p> |
| `input_path` | String | ✅ | <p>The URI that points to the Amazon S3 location of your data file.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create batch_import_job
batch_import_job = provider.frauddetector.Batch_import_job {
    job_id = "value"  # <p>The ID of the batch import job. The ID cannot be of a past job, unless the job exists in <code>CREATE_FAILED</code> state.</p>
    event_type_name = "value"  # <p>The name of the event type.</p>
    output_path = "value"  # <p>The URI that points to the Amazon S3 location for storing your results. </p>
    iam_role_arn = "value"  # <p>The ARN of the IAM role created for Amazon S3 bucket that holds your data file.</p>
         <p>The IAM role must have read permissions to your input S3 bucket and write permissions to your output S3 bucket.
         For more information about bucket permissions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/example-policies-s3.html">User policy examples</a> in the 
         <i>Amazon S3 User Guide</i>.</p>
    input_path = "value"  # <p>The URI that points to the Amazon S3 location of your data file.</p>
}

```

---


### Kms_encryption_key

KMSEncryptionKey resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `kms_encryption_key_arn` | String | ✅ | <p>The KMS encryption key ARN.</p>
         <p>The KMS key must be single-Region key. Amazon Fraud Detector does not support multi-Region KMS key.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kms_key` | String | <p>The KMS encryption key.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create kms_encryption_key
kms_encryption_key = provider.frauddetector.Kms_encryption_key {
    kms_encryption_key_arn = "value"  # <p>The KMS encryption key ARN.</p>
         <p>The KMS key must be single-Region key. Amazon Fraud Detector does not support multi-Region KMS key.</p>
}

# Access kms_encryption_key outputs
kms_encryption_key_id = kms_encryption_key.id
kms_encryption_key_kms_key = kms_encryption_key.kms_key
```

---


### Detector_version

DetectorVersion resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `rule_execution_mode` | String |  | <p>The rule execution mode for the rules included in the detector version.</p>
         <p>You can define and edit the rule mode at the detector version level, when it is in draft status.</p>
         <p>If you specify <code>FIRST_MATCHED</code>, Amazon Fraud Detector evaluates rules sequentially, first to last, stopping at the first matched rule. Amazon Fraud dectector then provides the outcomes for that single rule.</p>
         <p>If you specifiy <code>ALL_MATCHED</code>, Amazon Fraud Detector evaluates all rules and returns the outcomes for all matched rules. </p>
         <p>The default behavior is <code>FIRST_MATCHED</code>.</p> |
| `detector_id` | String | ✅ | <p>The ID of the detector under which you want to create a new version.</p> |
| `description` | String |  | <p>The description of the detector version.</p> |
| `rules` | Vec<String> | ✅ | <p>The rules to include in the detector version.</p> |
| `external_model_endpoints` | Vec<String> |  | <p>The Amazon Sagemaker model endpoints to include in the detector version.</p> |
| `model_versions` | Vec<String> |  | <p>The model versions to include in the detector version.</p> |
| `tags` | Vec<String> |  | <p>A collection of key and value pairs.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `created_time` | String | <p>The timestamp when the detector version was created. </p> |
| `external_model_endpoints` | Vec<String> | <p>The Amazon SageMaker model endpoints included in the detector version.</p> |
| `description` | String | <p>The detector version description.</p> |
| `rules` | Vec<String> | <p>The rules included in the detector version.</p> |
| `arn` | String | <p>The detector version ARN.</p> |
| `last_updated_time` | String | <p>The timestamp when the detector version was last updated.
        </p> |
| `detector_id` | String | <p>The detector ID.</p> |
| `detector_version_id` | String | <p>The detector version ID.</p> |
| `rule_execution_mode` | String | <p>The execution mode of the rule in the dectector</p>
         <p>
            <code>FIRST_MATCHED</code> indicates that Amazon Fraud Detector evaluates rules sequentially, first to last, stopping at the first matched rule. Amazon Fraud dectector then provides the outcomes for that single rule.</p>
         <p>
            <code>ALL_MATCHED</code> indicates that Amazon Fraud Detector evaluates all rules and returns the outcomes for all matched rules. You can define and edit the rule mode at the detector version level, when it is in draft status.</p> |
| `model_versions` | Vec<String> | <p>The model versions included in the detector version. </p> |
| `status` | String | <p>The status of the detector version.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create detector_version
detector_version = provider.frauddetector.Detector_version {
    detector_id = "value"  # <p>The ID of the detector under which you want to create a new version.</p>
    rules = "value"  # <p>The rules to include in the detector version.</p>
}

# Access detector_version outputs
detector_version_id = detector_version.id
detector_version_created_time = detector_version.created_time
detector_version_external_model_endpoints = detector_version.external_model_endpoints
detector_version_description = detector_version.description
detector_version_rules = detector_version.rules
detector_version_arn = detector_version.arn
detector_version_last_updated_time = detector_version.last_updated_time
detector_version_detector_id = detector_version.detector_id
detector_version_detector_version_id = detector_version.detector_version_id
detector_version_rule_execution_mode = detector_version.rule_execution_mode
detector_version_model_versions = detector_version.model_versions
detector_version_status = detector_version.status
```

---


### Variable

Variable resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `data_source` | String | ✅ | <p>The source of the data.</p> |
| `tags` | Vec<String> |  | <p>A collection of key and value pairs.</p> |
| `name` | String | ✅ | <p>The name of the variable.</p> |
| `data_type` | String | ✅ | <p>The data type of the variable.</p> |
| `description` | String |  | <p>The description.</p> |
| `variable_type` | String |  | <p>The variable type. For more information see <a href="https://docs.aws.amazon.com/frauddetector/latest/ug/create-a-variable.html#variable-types">Variable types</a>.
				</p>
         <p>Valid Values: <code>AUTH_CODE | AVS | BILLING_ADDRESS_L1 | BILLING_ADDRESS_L2 | BILLING_CITY | BILLING_COUNTRY | BILLING_NAME | BILLING_PHONE | BILLING_STATE | BILLING_ZIP | CARD_BIN | CATEGORICAL | CURRENCY_CODE | EMAIL_ADDRESS | FINGERPRINT | FRAUD_LABEL | FREE_FORM_TEXT | IP_ADDRESS | NUMERIC | ORDER_ID | PAYMENT_TYPE | PHONE_NUMBER | PRICE | PRODUCT_CATEGORY | SHIPPING_ADDRESS_L1 | SHIPPING_ADDRESS_L2 | SHIPPING_CITY | SHIPPING_COUNTRY | SHIPPING_NAME | SHIPPING_PHONE | SHIPPING_STATE | SHIPPING_ZIP | USERAGENT</code>
         </p> |
| `default_value` | String | ✅ | <p>The default value for the variable when no value is received.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create variable
variable = provider.frauddetector.Variable {
    data_source = "value"  # <p>The source of the data.</p>
    name = "value"  # <p>The name of the variable.</p>
    data_type = "value"  # <p>The data type of the variable.</p>
    default_value = "value"  # <p>The default value for the variable when no value is received.</p>
}

```

---


### Variables

Variables resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The next page token to be used in subsequent requests. </p> |
| `variables` | Vec<String> | <p>The names of the variables returned. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access variables outputs
variables_id = variables.id
variables_next_token = variables.next_token
variables_variables = variables.variables
```

---


### Detectors

Detectors resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `detectors` | Vec<String> | <p>The detectors.</p> |
| `next_token` | String | <p>The next page token.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access detectors outputs
detectors_id = detectors.id
detectors_detectors = detectors.detectors
detectors_next_token = detectors.next_token
```

---


### Rule

Rule resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>A collection of key and value pairs.</p> |
| `detector_id` | String | ✅ | <p>The detector ID for the rule's parent detector.</p> |
| `expression` | String | ✅ | <p>The rule expression.</p> |
| `language` | String | ✅ | <p>The language of the rule.</p> |
| `description` | String |  | <p>The rule description.</p> |
| `outcomes` | Vec<String> | ✅ | <p>The outcome or outcomes returned when the rule expression matches.</p> |
| `rule_id` | String | ✅ | <p>The rule ID.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create rule
rule = provider.frauddetector.Rule {
    detector_id = "value"  # <p>The detector ID for the rule's parent detector.</p>
    expression = "value"  # <p>The rule expression.</p>
    language = "value"  # <p>The language of the rule.</p>
    outcomes = "value"  # <p>The outcome or outcomes returned when the rule expression matches.</p>
    rule_id = "value"  # <p>The rule ID.</p>
}

```

---


### Detector

Detector resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `detector_id` | String | ✅ | <p>The detector ID. </p> |
| `event_type_name` | String | ✅ | <p>The name of the event type.</p> |
| `tags` | Vec<String> |  | <p>A collection of key and value pairs.</p> |
| `description` | String |  | <p>The description of the detector.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `detector_version_summaries` | Vec<String> | <p>The status and description for each detector version.</p> |
| `next_token` | String | <p>The next token to be used for subsequent requests.</p> |
| `detector_id` | String | <p>The detector ID.</p> |
| `arn` | String | <p>The detector ARN.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create detector
detector = provider.frauddetector.Detector {
    detector_id = "value"  # <p>The detector ID. </p>
    event_type_name = "value"  # <p>The name of the event type.</p>
}

# Access detector outputs
detector_id = detector.id
detector_detector_version_summaries = detector.detector_version_summaries
detector_next_token = detector.next_token
detector_detector_id = detector.detector_id
detector_arn = detector.arn
```

---


### External_model

ExternalModel resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `invoke_model_endpoint_role_arn` | String | ✅ | <p>The IAM role used to invoke the model endpoint.</p> |
| `tags` | Vec<String> |  | <p>A collection of key and value pairs.</p> |
| `output_configuration` | String | ✅ | <p>The model endpoint output configuration.</p> |
| `model_endpoint_status` | String | ✅ | <p>The model endpoint’s status in Amazon Fraud Detector.</p> |
| `model_endpoint` | String | ✅ | <p>The model endpoints name.</p> |
| `model_source` | String | ✅ | <p>The source of the model.</p> |
| `input_configuration` | String | ✅ | <p>The model endpoint input configuration.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create external_model
external_model = provider.frauddetector.External_model {
    invoke_model_endpoint_role_arn = "value"  # <p>The IAM role used to invoke the model endpoint.</p>
    output_configuration = "value"  # <p>The model endpoint output configuration.</p>
    model_endpoint_status = "value"  # <p>The model endpoint’s status in Amazon Fraud Detector.</p>
    model_endpoint = "value"  # <p>The model endpoints name.</p>
    model_source = "value"  # <p>The source of the model.</p>
    input_configuration = "value"  # <p>The model endpoint input configuration.</p>
}

```

---


### External_models

ExternalModels resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `external_models` | Vec<String> | <p>Gets the Amazon SageMaker models.</p> |
| `next_token` | String | <p>The next page token to be used in subsequent requests.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access external_models outputs
external_models_id = external_models.id
external_models_external_models = external_models.external_models
external_models_next_token = external_models.next_token
```

---


### List_elements

ListElements resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>
            The next page token.
        </p> |
| `elements` | Vec<String> | <p>
            The list elements.
        </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access list_elements outputs
list_elements_id = list_elements.id
list_elements_next_token = list_elements.next_token
list_elements_elements = list_elements.elements
```

---


### Rules

Rules resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `rule_details` | Vec<String> | <p>The details of the requested rule.</p> |
| `next_token` | String | <p>The next page token to be used in subsequent requests.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access rules outputs
rules_id = rules.id
rules_rule_details = rules.rule_details
rules_next_token = rules.next_token
```

---


### Outcomes

Outcomes resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The next page token for subsequent requests.</p> |
| `outcomes` | Vec<String> | <p>The outcomes. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access outcomes outputs
outcomes_id = outcomes.id
outcomes_next_token = outcomes.next_token
outcomes_outcomes = outcomes.outcomes
```

---


### Entity_type

EntityType resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | <p>The description.</p> |
| `tags` | Vec<String> |  | <p>A collection of key and value pairs.</p> |
| `name` | String | ✅ | <p>The name of the entity type.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create entity_type
entity_type = provider.frauddetector.Entity_type {
    name = "value"  # <p>The name of the entity type.</p>
}

```

---


### Event_prediction

EventPrediction resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `rule_results` | Vec<String> | <p>The results from the rules.</p> |
| `model_scores` | Vec<String> | <p>The model scores. Amazon Fraud Detector generates model scores between 0 and 1000, where 0 is low fraud risk and 1000 is high fraud risk. Model scores are directly related to the false positive rate (FPR). For example, a score of 600 corresponds to an estimated 10% false positive rate whereas a score of 900 corresponds to an estimated 2% false positive rate.</p> |
| `external_model_outputs` | Vec<String> | <p>The model scores for Amazon SageMaker models.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access event_prediction outputs
event_prediction_id = event_prediction.id
event_prediction_rule_results = event_prediction.rule_results
event_prediction_model_scores = event_prediction.model_scores
event_prediction_external_model_outputs = event_prediction.external_model_outputs
```

---


### Rule_version

RuleVersion resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `rule` | String | ✅ | <p>The rule to update.</p> |
| `outcomes` | Vec<String> | ✅ | <p>The outcomes.</p> |
| `language` | String | ✅ | <p>The language.</p> |
| `expression` | String | ✅ | <p>The rule expression.</p> |
| `description` | String |  | <p>The description.</p> |
| `tags` | Vec<String> |  | <p>The tags to assign to the rule version.</p> |



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


### Entity_types

EntityTypes resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `entity_types` | Vec<String> | <p>An array of entity types.</p> |
| `next_token` | String | <p>The next page token.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access entity_types outputs
entity_types_id = entity_types.id
entity_types_entity_types = entity_types.entity_types
entity_types_next_token = entity_types.next_token
```

---


### Detector_version_status

DetectorVersionStatus resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `detector_id` | String | ✅ | <p>The detector ID. </p> |
| `detector_version_id` | String | ✅ | <p>The detector version ID. </p> |
| `status` | String | ✅ | <p>The new status.</p>
         <p>The only supported values are <code>ACTIVE</code> and <code>INACTIVE</code>
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


### Event

Event resource

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `event` | String | <p>The details of the event.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access event outputs
event_id = event.id
event_event = event.event
```

---


### Detector_version_metadata

DetectorVersionMetadata resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `detector_id` | String | ✅ | <p>The detector ID.</p> |
| `detector_version_id` | String | ✅ | <p>The detector version ID. </p> |
| `description` | String | ✅ | <p>The description.</p> |



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


### Delete_events_by_event_type_status

DeleteEventsByEventTypeStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `event_type_name` | String | <p>The event type name.</p> |
| `events_deletion_status` | String | <p>The deletion status.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access delete_events_by_event_type_status outputs
delete_events_by_event_type_status_id = delete_events_by_event_type_status.id
delete_events_by_event_type_status_event_type_name = delete_events_by_event_type_status.event_type_name
delete_events_by_event_type_status_events_deletion_status = delete_events_by_event_type_status.events_deletion_status
```

---


### Batch_prediction_jobs

BatchPredictionJobs resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The next token for the subsequent request.</p> |
| `batch_predictions` | Vec<String> | <p>An array containing the details of each batch prediction job.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access batch_prediction_jobs outputs
batch_prediction_jobs_id = batch_prediction_jobs.id
batch_prediction_jobs_next_token = batch_prediction_jobs.next_token
batch_prediction_jobs_batch_predictions = batch_prediction_jobs.batch_predictions
```

---


### Lists_metadata

ListsMetadata resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>
            The next page token.
        </p> |
| `lists` | Vec<String> | <p>
            The metadata of the specified list or all lists under the account.
        </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access lists_metadata outputs
lists_metadata_id = lists_metadata.id
lists_metadata_next_token = lists_metadata.next_token
lists_metadata_lists = lists_metadata.lists
```

---


### Model_version

ModelVersion resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `training_data_source` | String | ✅ | <p>The training data source location in Amazon S3. </p> |
| `training_data_schema` | String | ✅ | <p>The training data schema.</p> |
| `external_events_detail` | String |  | <p>Details of the external events data used for model version training. Required if <code>trainingDataSource</code> is <code>EXTERNAL_EVENTS</code>.</p> |
| `ingested_events_detail` | String |  | <p>Details of the ingested events data used for model version training. Required if <code>trainingDataSource</code> is <code>INGESTED_EVENTS</code>.</p> |
| `tags` | Vec<String> |  | <p>A collection of key and value pairs.</p> |
| `model_type` | String | ✅ | <p>The model type.</p> |
| `model_id` | String | ✅ | <p>The model ID. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `training_data_schema` | String | <p>The training data schema.</p> |
| `external_events_detail` | String | <p>The details of the external events data used for training the model version. 
         This will be populated if the <code>trainingDataSource</code> is <code>EXTERNAL_EVENTS</code>
         </p> |
| `model_id` | String | <p>The model ID.</p> |
| `training_data_source` | String | <p>The training data source.</p> |
| `status` | String | <p>The model version status.</p>
         <p>Possible values are:</p>
         <ul>
            <li>
               <p>
                  <code>TRAINING_IN_PROGRESS</code>
               </p>
            </li>
            <li>
               <p>
                  <code>TRAINING_COMPLETE</code>
               </p>
            </li>
            <li>
               <p>
                  <code>ACTIVATE_REQUESTED</code>
               </p>
            </li>
            <li>
               <p>
                  <code>ACTIVATE_IN_PROGRESS</code>
               </p>
            </li>
            <li>
               <p>
                  <code>ACTIVE</code>
               </p>
            </li>
            <li>
               <p>
                  <code>INACTIVATE_REQUESTED</code>
               </p>
            </li>
            <li>
               <p>
                  <code>INACTIVATE_IN_PROGRESS</code>
               </p>
            </li>
            <li>
               <p>
                  <code>INACTIVE</code>
               </p>
            </li>
            <li>
               <p>
                  <code>ERROR</code>
               </p>
            </li>
         </ul> |
| `model_version_number` | String | <p>The model version number.</p> |
| `arn` | String | <p>The model version ARN.</p> |
| `model_type` | String | <p>The model type.</p> |
| `ingested_events_detail` | String | <p>The details of the ingested events data used for training the model version. 
         This will be populated if the <code>trainingDataSource</code> is <code>INGESTED_EVENTS</code>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create model_version
model_version = provider.frauddetector.Model_version {
    training_data_source = "value"  # <p>The training data source location in Amazon S3. </p>
    training_data_schema = "value"  # <p>The training data schema.</p>
    model_type = "value"  # <p>The model type.</p>
    model_id = "value"  # <p>The model ID. </p>
}

# Access model_version outputs
model_version_id = model_version.id
model_version_training_data_schema = model_version.training_data_schema
model_version_external_events_detail = model_version.external_events_detail
model_version_model_id = model_version.model_id
model_version_training_data_source = model_version.training_data_source
model_version_status = model_version.status
model_version_model_version_number = model_version.model_version_number
model_version_arn = model_version.arn
model_version_model_type = model_version.model_type
model_version_ingested_events_detail = model_version.ingested_events_detail
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple events_by_event_type resources
events_by_event_type_0 = provider.frauddetector.Events_by_event_type {
}
events_by_event_type_1 = provider.frauddetector.Events_by_event_type {
}
events_by_event_type_2 = provider.frauddetector.Events_by_event_type {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    events_by_event_type = provider.frauddetector.Events_by_event_type {
    }
```

---

## Related Documentation

- [AWS Frauddetector Documentation](https://docs.aws.amazon.com/frauddetector/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
