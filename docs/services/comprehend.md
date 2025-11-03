# Comprehend Service



**Resources**: 16

---

## Overview

The comprehend service provides access to 16 resource types:

- [Document_classification_job](#document_classification_job) [R]
- [Entity_recognizer](#entity_recognizer) [CRD]
- [Events_detection_job](#events_detection_job) [R]
- [Dominant_language_detection_job](#dominant_language_detection_job) [R]
- [Sentiment_detection_job](#sentiment_detection_job) [R]
- [Endpoint](#endpoint) [CRUD]
- [Flywheel](#flywheel) [CRUD]
- [Topics_detection_job](#topics_detection_job) [R]
- [Dataset](#dataset) [CR]
- [Entities_detection_job](#entities_detection_job) [R]
- [Document_classifier](#document_classifier) [CRD]
- [Pii_entities_detection_job](#pii_entities_detection_job) [R]
- [Key_phrases_detection_job](#key_phrases_detection_job) [R]
- [Resource_policy](#resource_policy) [CRD]
- [Targeted_sentiment_detection_job](#targeted_sentiment_detection_job) [R]
- [Flywheel_iteration](#flywheel_iteration) [R]

---

## Resources


### Document_classification_job

DocumentClassificationJob resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `document_classification_job_properties` | String | <p>An object that describes the properties associated with the document classification
      job.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access document_classification_job outputs
document_classification_job_id = document_classification_job.id
document_classification_job_document_classification_job_properties = document_classification_job.document_classification_job_properties
```

---


### Entity_recognizer

EntityRecognizer resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `data_access_role_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the IAM role that
      grants Amazon Comprehend read access to your input data.</p> |
| `recognizer_name` | String | ✅ | <p>The name given to the newly created recognizer. Recognizer names can be a maximum of 256
      characters. Alphanumeric characters, hyphens (-) and underscores (_) are allowed. The name
      must be unique in the account/Region.</p> |
| `model_kms_key_id` | String |  | <p>ID for the KMS key that Amazon Comprehend uses to encrypt
      trained custom models. The ModelKmsKeyId can be either of the following formats:</p>
         <ul>
            <li>
               <p>KMS Key ID: <code>"1234abcd-12ab-34cd-56ef-1234567890ab"</code>
               </p>
            </li>
            <li>
               <p>Amazon Resource Name (ARN) of a KMS Key:
            <code>"arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab"</code>
               </p>
            </li>
         </ul> |
| `client_request_token` | String |  | <p> A unique identifier for the request. If you don't set the client request token, Amazon
      Comprehend generates one.</p> |
| `vpc_config` | String |  | <p>Configuration parameters for an optional private Virtual Private Cloud (VPC) containing
      the resources you are using for your custom entity recognizer. For more information, see
      <a href="https://docs.aws.amazon.com/vpc/latest/userguide/what-is-amazon-vpc.html">Amazon
        VPC</a>. </p> |
| `volume_kms_key_id` | String |  | <p>ID for the Amazon Web Services Key Management Service (KMS) key that Amazon Comprehend uses to encrypt
      data on the storage volume attached to the ML compute instance(s) that process the analysis
      job. The VolumeKmsKeyId can be either of the following formats:</p>
         <ul>
            <li>
               <p>KMS Key ID: <code>"1234abcd-12ab-34cd-56ef-1234567890ab"</code>
               </p>
            </li>
            <li>
               <p>Amazon Resource Name (ARN) of a KMS Key:
            <code>"arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab"</code>
               </p>
            </li>
         </ul> |
| `input_data_config` | String | ✅ | <p>Specifies the format and location of the input data. The S3 bucket containing the input
      data must be located in the same Region as the entity recognizer being created. </p> |
| `model_policy` | String |  | <p>The JSON resource-based policy to attach to your custom entity recognizer model. You can
      use this policy to allow another Amazon Web Services account to import your custom model.</p>
         <p>Provide your JSON as a UTF-8 encoded string without line breaks. To provide valid JSON for
      your policy, enclose the attribute names and values in double quotes. If the JSON body is also
      enclosed in double quotes, then you must escape the double quotes that are inside the
      policy:</p>
         <p>
            <code>"{\"attribute\": \"value\", \"attribute\": [\"value\"]}"</code>
         </p>
         <p>To avoid escaping quotes, you can use single quotes to enclose the policy and double
      quotes to enclose the JSON names and values:</p>
         <p>
            <code>'{"attribute": "value", "attribute": ["value"]}'</code>
         </p> |
| `version_name` | String |  | <p>The version name given to the newly created recognizer. Version names can be a maximum of
      256 characters. Alphanumeric characters, hyphens (-) and underscores (_) are allowed. The
      version name must be unique among all models with the same recognizer name in the account/Region.</p> |
| `tags` | Vec<String> |  | <p>Tags to associate with the entity recognizer. A tag is a key-value pair
      that adds as a metadata to a resource used by Amazon Comprehend. For example, a tag with
      "Sales" as the key might be added to a resource to indicate its use by the sales department.
    </p> |
| `language_code` | String | ✅ | <p> You can specify any of the following languages: English
      ("en"), Spanish ("es"), French ("fr"), Italian ("it"), German ("de"), or Portuguese ("pt").
      If you plan to use this entity recognizer with PDF, Word, or image input files, you must
      specify English as the language.
      All training documents must be in the same language.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `entity_recognizer_properties` | String | <p>Describes information associated with an entity recognizer.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create entity_recognizer
entity_recognizer = provider.comprehend.Entity_recognizer {
    data_access_role_arn = "value"  # <p>The Amazon Resource Name (ARN) of the IAM role that
      grants Amazon Comprehend read access to your input data.</p>
    recognizer_name = "value"  # <p>The name given to the newly created recognizer. Recognizer names can be a maximum of 256
      characters. Alphanumeric characters, hyphens (-) and underscores (_) are allowed. The name
      must be unique in the account/Region.</p>
    input_data_config = "value"  # <p>Specifies the format and location of the input data. The S3 bucket containing the input
      data must be located in the same Region as the entity recognizer being created. </p>
    language_code = "value"  # <p> You can specify any of the following languages: English
      ("en"), Spanish ("es"), French ("fr"), Italian ("it"), German ("de"), or Portuguese ("pt").
      If you plan to use this entity recognizer with PDF, Word, or image input files, you must
      specify English as the language.
      All training documents must be in the same language.</p>
}

# Access entity_recognizer outputs
entity_recognizer_id = entity_recognizer.id
entity_recognizer_entity_recognizer_properties = entity_recognizer.entity_recognizer_properties
```

---


### Events_detection_job

EventsDetectionJob resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `events_detection_job_properties` | String | <p>An object that contains the properties associated with an event detection job.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access events_detection_job outputs
events_detection_job_id = events_detection_job.id
events_detection_job_events_detection_job_properties = events_detection_job.events_detection_job_properties
```

---


### Dominant_language_detection_job

DominantLanguageDetectionJob resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `dominant_language_detection_job_properties` | String | <p>An object that contains the properties associated with a dominant language detection
      job.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access dominant_language_detection_job outputs
dominant_language_detection_job_id = dominant_language_detection_job.id
dominant_language_detection_job_dominant_language_detection_job_properties = dominant_language_detection_job.dominant_language_detection_job_properties
```

---


### Sentiment_detection_job

SentimentDetectionJob resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `sentiment_detection_job_properties` | String | <p>An object that contains the properties associated with a sentiment detection job.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access sentiment_detection_job outputs
sentiment_detection_job_id = sentiment_detection_job.id
sentiment_detection_job_sentiment_detection_job_properties = sentiment_detection_job.sentiment_detection_job_properties
```

---


### Endpoint

Endpoint resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `desired_inference_units` | i64 | ✅ | <p> The desired number of inference units to be used by the model using this endpoint.
      
      Each inference unit represents of a throughput of 100 characters per second.</p> |
| `client_request_token` | String |  | <p>An idempotency token provided by the customer. If this token matches a previous endpoint
      creation request, Amazon Comprehend will not return a <code>ResourceInUseException</code>.
    </p> |
| `data_access_role_arn` | String |  | <p>The Amazon Resource Name (ARN) of the IAM role that
      grants Amazon Comprehend read access to trained custom models encrypted with a customer
      managed key (ModelKmsKeyId).</p> |
| `flywheel_arn` | String |  | <p>The Amazon Resource Number (ARN) of the flywheel to which the endpoint will be
      attached.</p> |
| `model_arn` | String |  | <p>The Amazon Resource Number (ARN) of the model to which the endpoint will be
      attached.</p> |
| `endpoint_name` | String | ✅ | <p>This is the descriptive suffix that becomes part of the <code>EndpointArn</code> used for
      all subsequent requests to this resource. </p> |
| `tags` | Vec<String> |  | <p>Tags to associate with the endpoint. A tag is a key-value pair that adds
      metadata to the endpoint. For example, a tag with "Sales" as the key might be added to an
      endpoint to indicate its use by the sales department. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `endpoint_properties` | String | <p>Describes information associated with the specific endpoint.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create endpoint
endpoint = provider.comprehend.Endpoint {
    desired_inference_units = "value"  # <p> The desired number of inference units to be used by the model using this endpoint.
      
      Each inference unit represents of a throughput of 100 characters per second.</p>
    endpoint_name = "value"  # <p>This is the descriptive suffix that becomes part of the <code>EndpointArn</code> used for
      all subsequent requests to this resource. </p>
}

# Access endpoint outputs
endpoint_id = endpoint.id
endpoint_endpoint_properties = endpoint.endpoint_properties
```

---


### Flywheel

Flywheel resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `active_model_arn` | String |  | <p>To associate an existing model with the flywheel, specify the Amazon Resource Number (ARN) of the model version.
      Do not set <code>TaskConfig</code> or <code>ModelType</code> if you specify an <code>ActiveModelArn</code>.</p> |
| `client_request_token` | String |  | <p>A unique identifier for the request. If you don't set the client request token, Amazon
      Comprehend generates one.</p> |
| `data_access_role_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the IAM role that
      grants Amazon Comprehend the permissions required to access the flywheel data in the data lake.</p> |
| `data_lake_s3_uri` | String | ✅ | <p>Enter the S3 location for the data lake. You can specify a new S3 bucket or a new folder of an
    existing S3 bucket. The flywheel creates the data lake at this location.</p> |
| `tags` | Vec<String> |  | <p>The tags to associate with this flywheel.</p> |
| `flywheel_name` | String | ✅ | <p>Name for the flywheel.</p> |
| `task_config` | String |  | <p>Configuration about the model associated with the flywheel.
      You need to set <code>TaskConfig</code> if you are creating a flywheel for a new model.</p> |
| `data_security_config` | String |  | <p>Data security configurations.</p> |
| `model_type` | String |  | <p>The model type. You need to set <code>ModelType</code> if you are creating a flywheel for a new model.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `flywheel_properties` | String | <p>The flywheel properties.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create flywheel
flywheel = provider.comprehend.Flywheel {
    data_access_role_arn = "value"  # <p>The Amazon Resource Name (ARN) of the IAM role that
      grants Amazon Comprehend the permissions required to access the flywheel data in the data lake.</p>
    data_lake_s3_uri = "value"  # <p>Enter the S3 location for the data lake. You can specify a new S3 bucket or a new folder of an
    existing S3 bucket. The flywheel creates the data lake at this location.</p>
    flywheel_name = "value"  # <p>Name for the flywheel.</p>
}

# Access flywheel outputs
flywheel_id = flywheel.id
flywheel_flywheel_properties = flywheel.flywheel_properties
```

---


### Topics_detection_job

TopicsDetectionJob resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `topics_detection_job_properties` | String | <p>The list of properties for the requested job.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access topics_detection_job outputs
topics_detection_job_id = topics_detection_job.id
topics_detection_job_topics_detection_job_properties = topics_detection_job.topics_detection_job_properties
```

---


### Dataset

Dataset resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>Tags for the dataset.</p> |
| `dataset_name` | String | ✅ | <p>Name of the dataset.</p> |
| `dataset_type` | String |  | <p>The dataset type. You can specify that the data in a dataset is for training
      the model or for testing the model.</p> |
| `description` | String |  | <p>Description of the dataset.</p> |
| `input_data_config` | String | ✅ | <p>Information about the input data configuration. The type of input data varies based
      on the format of the input and whether the data is for a classifier model or an entity recognition model.</p> |
| `flywheel_arn` | String | ✅ | <p>The Amazon Resource Number (ARN) of the flywheel of the flywheel to receive the data.</p> |
| `client_request_token` | String |  | <p>A unique identifier for the request. If you don't set the client request token, Amazon
      Comprehend generates one.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `dataset_properties` | String | <p>The dataset properties.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create dataset
dataset = provider.comprehend.Dataset {
    dataset_name = "value"  # <p>Name of the dataset.</p>
    input_data_config = "value"  # <p>Information about the input data configuration. The type of input data varies based
      on the format of the input and whether the data is for a classifier model or an entity recognition model.</p>
    flywheel_arn = "value"  # <p>The Amazon Resource Number (ARN) of the flywheel of the flywheel to receive the data.</p>
}

# Access dataset outputs
dataset_id = dataset.id
dataset_dataset_properties = dataset.dataset_properties
```

---


### Entities_detection_job

EntitiesDetectionJob resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `entities_detection_job_properties` | String | <p>An object that contains the properties associated with an entities detection job.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access entities_detection_job outputs
entities_detection_job_id = entities_detection_job.id
entities_detection_job_entities_detection_job_properties = entities_detection_job.entities_detection_job_properties
```

---


### Document_classifier

DocumentClassifier resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `vpc_config` | String |  | <p>Configuration parameters for an optional private Virtual Private Cloud (VPC) containing
      the resources you are using for your custom classifier. For more information, see <a href="https://docs.aws.amazon.com/vpc/latest/userguide/what-is-amazon-vpc.html">Amazon
        VPC</a>. </p> |
| `mode` | String |  | <p>Indicates the mode in which the classifier will be trained. The classifier can be trained
      in multi-class (single-label) mode or multi-label mode. 
      Multi-class mode identifies a single class label for each document and
      multi-label mode identifies one or more class labels for each document. Multiple
      labels for an individual document are separated by a delimiter. The default delimiter between
      labels is a pipe (|).</p> |
| `input_data_config` | String | ✅ | <p>Specifies the format and location of the input data for the job.</p> |
| `model_kms_key_id` | String |  | <p>ID for the KMS key that Amazon Comprehend uses to encrypt
      trained custom models. The ModelKmsKeyId can be either of the following formats:</p>
         <ul>
            <li>
               <p>KMS Key ID: <code>"1234abcd-12ab-34cd-56ef-1234567890ab"</code>
               </p>
            </li>
            <li>
               <p>Amazon Resource Name (ARN) of a KMS Key:
            <code>"arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab"</code>
               </p>
            </li>
         </ul> |
| `data_access_role_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the IAM role that
      grants Amazon Comprehend read access to your input data.</p> |
| `language_code` | String | ✅ | <p>The language of the input documents. You can specify any of the languages
      supported by Amazon Comprehend. All documents must be in the same language.</p> |
| `model_policy` | String |  | <p>The resource-based policy to attach to your custom document classifier model. You can use
      this policy to allow another Amazon Web Services account to import your custom model.</p>
         <p>Provide your policy as a JSON body that you enter as a UTF-8 encoded string without line
      breaks. To provide valid JSON, enclose the attribute names and values in double quotes. If the
      JSON body is also enclosed in double quotes, then you must escape the double quotes that are
      inside the policy:</p>
         <p>
            <code>"{\"attribute\": \"value\", \"attribute\": [\"value\"]}"</code>
         </p>
         <p>To avoid escaping quotes, you can use single quotes to enclose the policy and double
      quotes to enclose the JSON names and values:</p>
         <p>
            <code>'{"attribute": "value", "attribute": ["value"]}'</code>
         </p> |
| `output_data_config` | String |  | <p>Specifies the location for the output files from a custom classifier job.
      This parameter is required for a request that creates a native document model.</p> |
| `tags` | Vec<String> |  | <p>Tags to associate with the document classifier. A tag is a key-value
      pair that adds as a metadata to a resource used by Amazon Comprehend. For example, a tag with
      "Sales" as the key might be added to a resource to indicate its use by the sales department.
    </p> |
| `version_name` | String |  | <p>The version name given to the newly created classifier. Version names can have a maximum
      of 256 characters. Alphanumeric characters, hyphens (-) and underscores (_) are allowed. The
      version name must be unique among all models with the same classifier name in the Amazon Web Services account/Amazon Web Services Region.</p> |
| `volume_kms_key_id` | String |  | <p>ID for the Amazon Web Services Key Management Service (KMS) key that Amazon Comprehend uses to encrypt
      data on the storage volume attached to the ML compute instance(s) that process the analysis
      job. The VolumeKmsKeyId can be either of the following formats:</p>
         <ul>
            <li>
               <p>KMS Key ID: <code>"1234abcd-12ab-34cd-56ef-1234567890ab"</code>
               </p>
            </li>
            <li>
               <p>Amazon Resource Name (ARN) of a KMS Key:
            <code>"arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab"</code>
               </p>
            </li>
         </ul> |
| `document_classifier_name` | String | ✅ | <p>The name of the document classifier.</p> |
| `client_request_token` | String |  | <p>A unique identifier for the request. If you don't set the client request token, Amazon
      Comprehend generates one.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `document_classifier_properties` | String | <p>An object that contains the properties associated with a document classifier.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create document_classifier
document_classifier = provider.comprehend.Document_classifier {
    input_data_config = "value"  # <p>Specifies the format and location of the input data for the job.</p>
    data_access_role_arn = "value"  # <p>The Amazon Resource Name (ARN) of the IAM role that
      grants Amazon Comprehend read access to your input data.</p>
    language_code = "value"  # <p>The language of the input documents. You can specify any of the languages
      supported by Amazon Comprehend. All documents must be in the same language.</p>
    document_classifier_name = "value"  # <p>The name of the document classifier.</p>
}

# Access document_classifier outputs
document_classifier_id = document_classifier.id
document_classifier_document_classifier_properties = document_classifier.document_classifier_properties
```

---


### Pii_entities_detection_job

PiiEntitiesDetectionJob resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `pii_entities_detection_job_properties` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access pii_entities_detection_job outputs
pii_entities_detection_job_id = pii_entities_detection_job.id
pii_entities_detection_job_pii_entities_detection_job_properties = pii_entities_detection_job.pii_entities_detection_job_properties
```

---


### Key_phrases_detection_job

KeyPhrasesDetectionJob resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `key_phrases_detection_job_properties` | String | <p>An object that contains the properties associated with a key phrases detection job.
    </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access key_phrases_detection_job outputs
key_phrases_detection_job_id = key_phrases_detection_job.id
key_phrases_detection_job_key_phrases_detection_job_properties = key_phrases_detection_job.key_phrases_detection_job_properties
```

---


### Resource_policy

ResourcePolicy resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resource_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the custom model to attach the policy to.</p> |
| `resource_policy` | String | ✅ | <p>The JSON resource-based policy to attach to your custom model. Provide your JSON as a
      UTF-8 encoded string without line breaks. To provide valid JSON for your policy, enclose the
      attribute names and values in double quotes. If the JSON body is also enclosed in double
      quotes, then you must escape the double quotes that are inside the policy:</p>
         <p>
            <code>"{\"attribute\": \"value\", \"attribute\": [\"value\"]}"</code>
         </p>
         <p>To avoid escaping quotes, you can use single quotes to enclose the policy and double
      quotes to enclose the JSON names and values:</p>
         <p>
            <code>'{"attribute": "value", "attribute": ["value"]}'</code>
         </p> |
| `policy_revision_id` | String |  | <p>The revision ID that Amazon Comprehend assigned to the policy that you are updating. If
      you are creating a new policy that has no prior version, don't use this parameter. Amazon
      Comprehend creates the revision ID for you.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `last_modified_time` | String | <p>The time at which the policy was last modified.</p> |
| `resource_policy` | String | <p>The JSON body of the resource-based policy.</p> |
| `creation_time` | String | <p>The time at which the policy was created.</p> |
| `policy_revision_id` | String | <p>The revision ID of the policy. Each time you modify a policy, Amazon Comprehend assigns a
      new revision ID, and it deletes the prior version of the policy.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create resource_policy
resource_policy = provider.comprehend.Resource_policy {
    resource_arn = "value"  # <p>The Amazon Resource Name (ARN) of the custom model to attach the policy to.</p>
    resource_policy = "value"  # <p>The JSON resource-based policy to attach to your custom model. Provide your JSON as a
      UTF-8 encoded string without line breaks. To provide valid JSON for your policy, enclose the
      attribute names and values in double quotes. If the JSON body is also enclosed in double
      quotes, then you must escape the double quotes that are inside the policy:</p>
         <p>
            <code>"{\"attribute\": \"value\", \"attribute\": [\"value\"]}"</code>
         </p>
         <p>To avoid escaping quotes, you can use single quotes to enclose the policy and double
      quotes to enclose the JSON names and values:</p>
         <p>
            <code>'{"attribute": "value", "attribute": ["value"]}'</code>
         </p>
}

# Access resource_policy outputs
resource_policy_id = resource_policy.id
resource_policy_last_modified_time = resource_policy.last_modified_time
resource_policy_resource_policy = resource_policy.resource_policy
resource_policy_creation_time = resource_policy.creation_time
resource_policy_policy_revision_id = resource_policy.policy_revision_id
```

---


### Targeted_sentiment_detection_job

TargetedSentimentDetectionJob resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `targeted_sentiment_detection_job_properties` | String | <p>An object that contains the properties associated with a targeted sentiment detection job.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access targeted_sentiment_detection_job outputs
targeted_sentiment_detection_job_id = targeted_sentiment_detection_job.id
targeted_sentiment_detection_job_targeted_sentiment_detection_job_properties = targeted_sentiment_detection_job.targeted_sentiment_detection_job_properties
```

---


### Flywheel_iteration

FlywheelIteration resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `flywheel_iteration_properties` | String | <p>The configuration properties of a flywheel iteration.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access flywheel_iteration outputs
flywheel_iteration_id = flywheel_iteration.id
flywheel_iteration_flywheel_iteration_properties = flywheel_iteration.flywheel_iteration_properties
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple document_classification_job resources
document_classification_job_0 = provider.comprehend.Document_classification_job {
}
document_classification_job_1 = provider.comprehend.Document_classification_job {
}
document_classification_job_2 = provider.comprehend.Document_classification_job {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    document_classification_job = provider.comprehend.Document_classification_job {
    }
```

---

## Related Documentation

- [AWS Comprehend Documentation](https://docs.aws.amazon.com/comprehend/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
