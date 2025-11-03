# Textract Service



**Resources**: 7

---

## Overview

The textract service provides access to 7 resource types:

- [Adapter_version](#adapter_version) [CRD]
- [Lending_analysis_summary](#lending_analysis_summary) [R]
- [Lending_analysis](#lending_analysis) [R]
- [Document_text_detection](#document_text_detection) [R]
- [Adapter](#adapter) [CRUD]
- [Document_analysis](#document_analysis) [R]
- [Expense_analysis](#expense_analysis) [R]

---

## Resources


### Adapter_version

AdapterVersion resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_request_token` | String |  | <p>Idempotent token is used to recognize the request. If the same token is used with multiple 
         CreateAdapterVersion requests, the same session is returned. 
         This token is employed to avoid unintentionally creating the same session multiple times.</p> |
| `adapter_id` | String | ✅ | <p>A string containing a unique ID for the adapter that will receive a new version.</p> |
| `dataset_config` | String | ✅ | <p>Specifies a dataset used to train a new adapter version. Takes a ManifestS3Object as the
         value.</p> |
| `kms_key_id` | String |  | <p>The identifier for your AWS Key Management Service key (AWS KMS key). Used to encrypt your documents.</p> |
| `tags` | HashMap<String, String> |  | <p>A set of tags (key-value pairs) that you want to attach to the adapter version. </p> |
| `output_config` | String | ✅ |  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `adapter_id` | String | <p>A string containing a unique ID for the adapter version being retrieved.</p> |
| `tags` | HashMap<String, String> | <p>A set of tags (key-value pairs) that are associated with the adapter version.</p> |
| `kms_key_id` | String | <p>The identifier for your AWS Key Management Service key (AWS KMS key). Used to encrypt your documents.</p> |
| `output_config` | String |  |
| `status` | String | <p>The status of the adapter version that has been requested.</p> |
| `status_message` | String | <p>A message that describes the status of the requested adapter version.</p> |
| `dataset_config` | String | <p>Specifies a dataset used to train a new adapter version. Takes a ManifestS3Objec as the
         value.</p> |
| `adapter_version` | String | <p>A string containing the adapter version that has been retrieved.</p> |
| `evaluation_metrics` | Vec<String> | <p>The evaluation metrics (F1 score, Precision, and Recall) for the requested version, 
         grouped by baseline metrics and adapter version.</p> |
| `creation_time` | String | <p>The time that the adapter version was created.</p> |
| `feature_types` | Vec<String> | <p>List of the targeted feature types for the requested adapter version.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create adapter_version
adapter_version = provider.textract.Adapter_version {
    adapter_id = "value"  # <p>A string containing a unique ID for the adapter that will receive a new version.</p>
    dataset_config = "value"  # <p>Specifies a dataset used to train a new adapter version. Takes a ManifestS3Object as the
         value.</p>
    output_config = "value"  # Required field
}

# Access adapter_version outputs
adapter_version_id = adapter_version.id
adapter_version_adapter_id = adapter_version.adapter_id
adapter_version_tags = adapter_version.tags
adapter_version_kms_key_id = adapter_version.kms_key_id
adapter_version_output_config = adapter_version.output_config
adapter_version_status = adapter_version.status
adapter_version_status_message = adapter_version.status_message
adapter_version_dataset_config = adapter_version.dataset_config
adapter_version_adapter_version = adapter_version.adapter_version
adapter_version_evaluation_metrics = adapter_version.evaluation_metrics
adapter_version_creation_time = adapter_version.creation_time
adapter_version_feature_types = adapter_version.feature_types
```

---


### Lending_analysis_summary

LendingAnalysisSummary resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `summary` | String | <p> Contains summary information for documents grouped by type.</p> |
| `warnings` | Vec<String> | <p>A list of warnings that occurred during the lending analysis operation.</p> |
| `analyze_lending_model_version` | String | <p>The current model version of the Analyze Lending API.</p> |
| `job_status` | String | <p> The current status of the lending analysis job. </p> |
| `status_message` | String | <p>Returns if the lending analysis could not be completed. Contains explanation for what error
   occurred.</p> |
| `document_metadata` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access lending_analysis_summary outputs
lending_analysis_summary_id = lending_analysis_summary.id
lending_analysis_summary_summary = lending_analysis_summary.summary
lending_analysis_summary_warnings = lending_analysis_summary.warnings
lending_analysis_summary_analyze_lending_model_version = lending_analysis_summary.analyze_lending_model_version
lending_analysis_summary_job_status = lending_analysis_summary.job_status
lending_analysis_summary_status_message = lending_analysis_summary.status_message
lending_analysis_summary_document_metadata = lending_analysis_summary.document_metadata
```

---


### Lending_analysis

LendingAnalysis resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status_message` | String | <p>  Returns if the lending analysis job could not be completed. Contains explanation for
            what error occurred. </p> |
| `next_token` | String | <p>If the response is truncated, Amazon Textract returns this token. 
            You can use this token in the subsequent request to retrieve the next set of lending results.</p> |
| `document_metadata` | String |  |
| `job_status` | String | <p> The current status of the lending analysis job.</p> |
| `analyze_lending_model_version` | String | <p> The current model version of the Analyze Lending API.</p> |
| `warnings` | Vec<String> | <p> A list of warnings that occurred during the lending analysis operation. </p> |
| `results` | Vec<String> | <p> Holds the information returned by one of AmazonTextract's document analysis
            operations for the pinstripe.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access lending_analysis outputs
lending_analysis_id = lending_analysis.id
lending_analysis_status_message = lending_analysis.status_message
lending_analysis_next_token = lending_analysis.next_token
lending_analysis_document_metadata = lending_analysis.document_metadata
lending_analysis_job_status = lending_analysis.job_status
lending_analysis_analyze_lending_model_version = lending_analysis.analyze_lending_model_version
lending_analysis_warnings = lending_analysis.warnings
lending_analysis_results = lending_analysis.results
```

---


### Document_text_detection

DocumentTextDetection resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `blocks` | Vec<String> | <p>The results of the text-detection operation.</p> |
| `next_token` | String | <p>If the response is truncated, Amazon Textract returns this token. You can use this token in
         the subsequent request to retrieve the next set of text-detection results.</p> |
| `status_message` | String | <p>Returns if the detection job could not be completed. Contains explanation for what error occured. </p> |
| `warnings` | Vec<String> | <p>A list of warnings that occurred during the text-detection operation for the
         document.</p> |
| `document_metadata` | String | <p>Information about a document that Amazon Textract processed. <code>DocumentMetadata</code> is
         returned in every page of paginated responses from an Amazon Textract video operation.</p> |
| `job_status` | String | <p>The current status of the text detection job.</p> |
| `detect_document_text_model_version` | String | <p></p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access document_text_detection outputs
document_text_detection_id = document_text_detection.id
document_text_detection_blocks = document_text_detection.blocks
document_text_detection_next_token = document_text_detection.next_token
document_text_detection_status_message = document_text_detection.status_message
document_text_detection_warnings = document_text_detection.warnings
document_text_detection_document_metadata = document_text_detection.document_metadata
document_text_detection_job_status = document_text_detection.job_status
document_text_detection_detect_document_text_model_version = document_text_detection.detect_document_text_model_version
```

---


### Adapter

Adapter resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_request_token` | String |  | <p>Idempotent token is used to recognize the request. If the same token is used with multiple 
         CreateAdapter requests, the same session is returned. 
         This token is employed to avoid unintentionally creating the same session multiple times.</p> |
| `feature_types` | Vec<String> | ✅ | <p>The type of feature that the adapter is being trained on. Currrenly, supported feature
         types are: <code>QUERIES</code>
         </p> |
| `description` | String |  | <p>The description to be assigned to the adapter being created.</p> |
| `adapter_name` | String | ✅ | <p>The name to be assigned to the adapter being created.</p> |
| `tags` | HashMap<String, String> |  | <p>A list of tags to be added to the adapter.</p> |
| `auto_update` | String |  | <p>Controls whether or not the adapter should automatically update.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `auto_update` | String | <p>Binary value indicating if the adapter is being automatically updated or not.</p> |
| `description` | String | <p>The description for the requested adapter.</p> |
| `tags` | HashMap<String, String> | <p>A set of tags (key-value pairs) associated with the adapter that has been retrieved.</p> |
| `adapter_id` | String | <p>A string identifying the adapter that information has been retrieved for.</p> |
| `adapter_name` | String | <p>The name of the requested adapter.</p> |
| `creation_time` | String | <p>The date and time the requested adapter was created at.</p> |
| `feature_types` | Vec<String> | <p>List of the targeted feature types for the requested adapter.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create adapter
adapter = provider.textract.Adapter {
    feature_types = "value"  # <p>The type of feature that the adapter is being trained on. Currrenly, supported feature
         types are: <code>QUERIES</code>
         </p>
    adapter_name = "value"  # <p>The name to be assigned to the adapter being created.</p>
}

# Access adapter outputs
adapter_id = adapter.id
adapter_auto_update = adapter.auto_update
adapter_description = adapter.description
adapter_tags = adapter.tags
adapter_adapter_id = adapter.adapter_id
adapter_adapter_name = adapter.adapter_name
adapter_creation_time = adapter.creation_time
adapter_feature_types = adapter.feature_types
```

---


### Document_analysis

DocumentAnalysis resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `document_metadata` | String | <p>Information about a document that Amazon Textract processed.
            <code>DocumentMetadata</code> is returned in every page of paginated responses from an
         Amazon Textract video operation.</p> |
| `blocks` | Vec<String> | <p>The results of the text-analysis operation.</p> |
| `status_message` | String | <p>Returns if the detection job could not be completed. Contains explanation for what error
         occured.</p> |
| `next_token` | String | <p>If the response is truncated, Amazon Textract returns this token. You can use this token
         in the subsequent request to retrieve the next set of text detection results.</p> |
| `analyze_document_model_version` | String | <p></p> |
| `warnings` | Vec<String> | <p>A list of warnings that occurred during the document-analysis operation.</p> |
| `job_status` | String | <p>The current status of the text detection job.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access document_analysis outputs
document_analysis_id = document_analysis.id
document_analysis_document_metadata = document_analysis.document_metadata
document_analysis_blocks = document_analysis.blocks
document_analysis_status_message = document_analysis.status_message
document_analysis_next_token = document_analysis.next_token
document_analysis_analyze_document_model_version = document_analysis.analyze_document_model_version
document_analysis_warnings = document_analysis.warnings
document_analysis_job_status = document_analysis.job_status
```

---


### Expense_analysis

ExpenseAnalysis resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `expense_documents` | Vec<String> | <p>The expenses detected by Amazon Textract.</p> |
| `status_message` | String | <p>Returns if the detection job could not be completed. Contains explanation for what error occured. </p> |
| `job_status` | String | <p>The current status of the text detection job.</p> |
| `warnings` | Vec<String> | <p>A list of warnings that occurred during the text-detection operation for the
   document.</p> |
| `document_metadata` | String | <p>Information about a document that Amazon Textract processed. <code>DocumentMetadata</code> is
   returned in every page of paginated responses from an Amazon Textract operation.</p> |
| `next_token` | String | <p>If the response is truncated, Amazon Textract returns this token. You can use this token in
   the subsequent request to retrieve the next set of text-detection results.</p> |
| `analyze_expense_model_version` | String | <p>The current model version of AnalyzeExpense.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access expense_analysis outputs
expense_analysis_id = expense_analysis.id
expense_analysis_expense_documents = expense_analysis.expense_documents
expense_analysis_status_message = expense_analysis.status_message
expense_analysis_job_status = expense_analysis.job_status
expense_analysis_warnings = expense_analysis.warnings
expense_analysis_document_metadata = expense_analysis.document_metadata
expense_analysis_next_token = expense_analysis.next_token
expense_analysis_analyze_expense_model_version = expense_analysis.analyze_expense_model_version
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple adapter_version resources
adapter_version_0 = provider.textract.Adapter_version {
    adapter_id = "value-0"
    dataset_config = "value-0"
    output_config = "value-0"
}
adapter_version_1 = provider.textract.Adapter_version {
    adapter_id = "value-1"
    dataset_config = "value-1"
    output_config = "value-1"
}
adapter_version_2 = provider.textract.Adapter_version {
    adapter_id = "value-2"
    dataset_config = "value-2"
    output_config = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    adapter_version = provider.textract.Adapter_version {
        adapter_id = "production-value"
        dataset_config = "production-value"
        output_config = "production-value"
    }
```

---

## Related Documentation

- [AWS Textract Documentation](https://docs.aws.amazon.com/textract/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
