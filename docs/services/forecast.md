# Forecast Service



**Resources**: 16

---

## Overview

The forecast service provides access to 16 resource types:

- [Dataset](#dataset) [CRD]
- [Explainability_export](#explainability_export) [CRD]
- [Predictor](#predictor) [CRD]
- [Explainability](#explainability) [CRD]
- [Monitor](#monitor) [CRD]
- [What_if_analysis](#what_if_analysis) [CRD]
- [Forecast](#forecast) [CRD]
- [What_if_forecast](#what_if_forecast) [CRD]
- [Auto_predictor](#auto_predictor) [CR]
- [Predictor_backtest_export_job](#predictor_backtest_export_job) [CRD]
- [What_if_forecast_export](#what_if_forecast_export) [CRD]
- [Resource_tree](#resource_tree) [D]
- [Dataset_import_job](#dataset_import_job) [CRD]
- [Dataset_group](#dataset_group) [CRUD]
- [Accuracy_metrics](#accuracy_metrics) [R]
- [Forecast_export_job](#forecast_export_job) [CRD]

---

## Resources


### Dataset

Dataset resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `encryption_config` | String |  | <p>An Key Management Service (KMS) key and the Identity and Access Management (IAM) role that Amazon Forecast can assume to access
      the key.</p> |
| `data_frequency` | String |  | <p>The frequency of data collection. This parameter is required for RELATED_TIME_SERIES
      datasets.</p>
         <p>Valid intervals are an integer followed by Y (Year), M (Month), W (Week), D (Day), H (Hour), and min (Minute). For example,
      "1D" indicates every day and "15min" indicates every 15 minutes. You cannot specify a value that would overlap with the next larger frequency. That means, for example, you cannot specify a frequency of 60 minutes, because that is equivalent to 1 hour. The valid values for each frequency are the following:</p>
         <ul>
            <li>
               <p>Minute - 1-59</p>
            </li>
            <li>
               <p>Hour - 1-23</p>
            </li>
            <li>
               <p>Day - 1-6</p>
            </li>
            <li>
               <p>Week - 1-4</p>
            </li>
            <li>
               <p>Month - 1-11</p>
            </li>
            <li>
               <p>Year - 1</p>
            </li>
         </ul>
         <p>Thus, if you want every other week forecasts, specify "2W". Or, if you want quarterly forecasts, you specify "3M".</p> |
| `dataset_name` | String | ✅ | <p>A name for the dataset.</p> |
| `domain` | String | ✅ | <p>The domain associated with the dataset. When you add a dataset to a dataset group, this
      value and the value specified for the <code>Domain</code> parameter of the <a href="https://docs.aws.amazon.com/forecast/latest/dg/API_CreateDatasetGroup.html">CreateDatasetGroup</a> operation must match.</p>
         <p>The <code>Domain</code> and <code>DatasetType</code> that you choose determine the fields
      that must be present in the training data that you import to the dataset. For example, if you
      choose the <code>RETAIL</code> domain and <code>TARGET_TIME_SERIES</code> as the
        <code>DatasetType</code>, Amazon Forecast requires <code>item_id</code>, <code>timestamp</code>,
      and <code>demand</code> fields to be present in your data. For more information, see <a href="https://docs.aws.amazon.com/forecast/latest/dg/howitworks-datasets-groups.html">Importing
        datasets</a>.</p> |
| `tags` | Vec<String> |  | <p>The optional metadata that you apply to the dataset to help you categorize and organize
      them. Each tag consists of a key and an optional value, both of which you define.</p>
         <p>The following basic restrictions apply to tags:</p>
         <ul>
            <li>
               <p>Maximum number of tags per resource - 50.</p>
            </li>
            <li>
               <p>For each resource, each tag key must be unique, and each tag key can have only one
          value.</p>
            </li>
            <li>
               <p>Maximum key length - 128 Unicode characters in UTF-8.</p>
            </li>
            <li>
               <p>Maximum value length - 256 Unicode characters in UTF-8.</p>
            </li>
            <li>
               <p>If your tagging schema is used across multiple services and resources, remember that
          other services may have restrictions on allowed characters. Generally allowed characters
          are: letters, numbers, and spaces representable in UTF-8, and the following characters: +
          - = . _ : / @.</p>
            </li>
            <li>
               <p>Tag keys and values are case sensitive.</p>
            </li>
            <li>
               <p>Do not use <code>aws:</code>, <code>AWS:</code>, or any upper or lowercase combination
          of such as a prefix for keys as it is reserved for Amazon Web Services use. You cannot edit or delete tag
          keys with this prefix. Values can have this prefix. If a tag value has <code>aws</code> as
          its prefix but the key does not, then Forecast considers it to be a user tag and will
          count against the limit of 50 tags. Tags with only the key prefix of <code>aws</code> do
          not count against your tags per resource limit.</p>
            </li>
         </ul> |
| `dataset_type` | String | ✅ | <p>The dataset type. Valid values depend on the chosen <code>Domain</code>.</p> |
| `schema` | String | ✅ | <p>The schema for the dataset. The schema attributes and their order must match the fields in
      your data. The dataset <code>Domain</code> and <code>DatasetType</code> that you choose
      determine the minimum required fields in your training data. For information about the
      required fields for a specific dataset domain and type, see <a href="https://docs.aws.amazon.com/forecast/latest/dg/howitworks-domains-ds-types.html">Dataset Domains and Dataset
        Types</a>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `creation_time` | String | <p>When the dataset was created.</p> |
| `dataset_name` | String | <p>The name of the dataset.</p> |
| `dataset_type` | String | <p>The dataset type.</p> |
| `data_frequency` | String | <p>The frequency of data collection.</p>
         <p>Valid intervals are Y (Year), M (Month), W (Week), D (Day), H (Hour), 30min (30 minutes),
      15min (15 minutes), 10min (10 minutes), 5min (5 minutes), and 1min (1 minute). For example,
      "M" indicates every month and "30min" indicates every 30 minutes.</p> |
| `dataset_arn` | String | <p>The Amazon Resource Name (ARN) of the dataset.</p> |
| `last_modification_time` | String | <p>When you create a dataset, <code>LastModificationTime</code> is the same as
        <code>CreationTime</code>. While data is being imported to the dataset,
        <code>LastModificationTime</code> is the current time of the <code>DescribeDataset</code>
      call. After a <a href="https://docs.aws.amazon.com/forecast/latest/dg/API_CreateDatasetImportJob.html">CreateDatasetImportJob</a>
      operation has finished, <code>LastModificationTime</code> is when the import job completed or
      failed.</p> |
| `status` | String | <p>The status of the dataset. States include:</p>
         <ul>
            <li>
               <p>
                  <code>ACTIVE</code>
               </p>
            </li>
            <li>
               <p>
                  <code>CREATE_PENDING</code>, <code>CREATE_IN_PROGRESS</code>,
            <code>CREATE_FAILED</code>
               </p>
            </li>
            <li>
               <p>
                  <code>DELETE_PENDING</code>, <code>DELETE_IN_PROGRESS</code>,
            <code>DELETE_FAILED</code>
               </p>
            </li>
            <li>
               <p>
                  <code>UPDATE_PENDING</code>, <code>UPDATE_IN_PROGRESS</code>,
            <code>UPDATE_FAILED</code>
               </p>
            </li>
         </ul>
         <p>The <code>UPDATE</code> states apply while data is imported to the dataset from a call to
      the <a href="https://docs.aws.amazon.com/forecast/latest/dg/API_CreateDatasetImportJob.html">CreateDatasetImportJob</a> operation and reflect the status of the dataset import job.
      For example, when the import job status is <code>CREATE_IN_PROGRESS</code>, the status of the
      dataset is <code>UPDATE_IN_PROGRESS</code>.</p>
         <note>
            <p>The <code>Status</code> of the dataset must be <code>ACTIVE</code> before you can import
        training data.</p>
         </note> |
| `domain` | String | <p>The domain associated with the dataset.</p> |
| `encryption_config` | String | <p>The Key Management Service (KMS) key and the Identity and Access Management (IAM) role that Amazon Forecast can assume to access
      the key.</p> |
| `schema` | String | <p>An array of <code>SchemaAttribute</code> objects that specify the dataset fields. Each
        <code>SchemaAttribute</code> specifies the name and data type of a field.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create dataset
dataset = provider.forecast.Dataset {
    dataset_name = "value"  # <p>A name for the dataset.</p>
    domain = "value"  # <p>The domain associated with the dataset. When you add a dataset to a dataset group, this
      value and the value specified for the <code>Domain</code> parameter of the <a href="https://docs.aws.amazon.com/forecast/latest/dg/API_CreateDatasetGroup.html">CreateDatasetGroup</a> operation must match.</p>
         <p>The <code>Domain</code> and <code>DatasetType</code> that you choose determine the fields
      that must be present in the training data that you import to the dataset. For example, if you
      choose the <code>RETAIL</code> domain and <code>TARGET_TIME_SERIES</code> as the
        <code>DatasetType</code>, Amazon Forecast requires <code>item_id</code>, <code>timestamp</code>,
      and <code>demand</code> fields to be present in your data. For more information, see <a href="https://docs.aws.amazon.com/forecast/latest/dg/howitworks-datasets-groups.html">Importing
        datasets</a>.</p>
    dataset_type = "value"  # <p>The dataset type. Valid values depend on the chosen <code>Domain</code>.</p>
    schema = "value"  # <p>The schema for the dataset. The schema attributes and their order must match the fields in
      your data. The dataset <code>Domain</code> and <code>DatasetType</code> that you choose
      determine the minimum required fields in your training data. For information about the
      required fields for a specific dataset domain and type, see <a href="https://docs.aws.amazon.com/forecast/latest/dg/howitworks-domains-ds-types.html">Dataset Domains and Dataset
        Types</a>.</p>
}

# Access dataset outputs
dataset_id = dataset.id
dataset_creation_time = dataset.creation_time
dataset_dataset_name = dataset.dataset_name
dataset_dataset_type = dataset.dataset_type
dataset_data_frequency = dataset.data_frequency
dataset_dataset_arn = dataset.dataset_arn
dataset_last_modification_time = dataset.last_modification_time
dataset_status = dataset.status
dataset_domain = dataset.domain
dataset_encryption_config = dataset.encryption_config
dataset_schema = dataset.schema
```

---


### Explainability_export

ExplainabilityExport resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `destination` | String | ✅ |  |
| `format` | String |  | <p>The format of the exported data, CSV or PARQUET.</p> |
| `explainability_export_name` | String | ✅ | <p>A unique name for the Explainability export.</p> |
| `explainability_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the Explainability to export.</p> |
| `tags` | Vec<String> |  | <p>Optional metadata to help you categorize and organize your resources. Each tag
            consists of a key and an optional value, both of which you define. Tag keys and values
            are case sensitive.</p>
         <p>The following restrictions apply to tags:</p>
         <ul>
            <li>
               <p>For each resource, each tag key must be unique and each tag key must have one
                    value.</p>
            </li>
            <li>
               <p>Maximum number of tags per resource: 50.</p>
            </li>
            <li>
               <p>Maximum key length: 128 Unicode characters in UTF-8.</p>
            </li>
            <li>
               <p>Maximum value length: 256 Unicode characters in UTF-8.</p>
            </li>
            <li>
               <p>Accepted characters: all letters and numbers, spaces representable in UTF-8,
                    and + - = . _ : / @. If your tagging schema is used across other services and
                    resources, the character restrictions of those services also apply. </p>
            </li>
            <li>
               <p>Key prefixes cannot include any upper or lowercase combination of
                        <code>aws:</code> or <code>AWS:</code>. Values can have this prefix. If a
                    tag value has <code>aws</code> as its prefix but the key does not, Forecast
                    considers it to be a user tag and will count against the limit of 50 tags. Tags
                    with only the key prefix of <code>aws</code> do not count against your tags per
                    resource limit. You cannot edit or delete tag keys with this prefix.</p>
            </li>
         </ul> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `explainability_export_arn` | String | <p>The Amazon Resource Name (ARN) of the Explainability export.</p> |
| `format` | String | <p>The format of the exported data, CSV or PARQUET.</p> |
| `status` | String | <p>The status of the Explainability export. States include: </p>
         <ul>
            <li>
               <p>
                  <code>ACTIVE</code>
               </p>
            </li>
            <li>
               <p>
                  <code>CREATE_PENDING</code>, <code>CREATE_IN_PROGRESS</code>,
                        <code>CREATE_FAILED</code>
               </p>
            </li>
            <li>
               <p>
                  <code>CREATE_STOPPING</code>, <code>CREATE_STOPPED</code>
               </p>
            </li>
            <li>
               <p>
                  <code>DELETE_PENDING</code>, <code>DELETE_IN_PROGRESS</code>,
                        <code>DELETE_FAILED</code>
               </p>
            </li>
         </ul> |
| `explainability_export_name` | String | <p>The name of the Explainability export.</p> |
| `creation_time` | String | <p>When the Explainability export was created.</p> |
| `message` | String | <p>Information about any errors that occurred during the export.</p> |
| `destination` | String |  |
| `explainability_arn` | String | <p>The Amazon Resource Name (ARN) of the Explainability export.</p> |
| `last_modification_time` | String | <p>The last time the resource was modified. The timestamp depends on the status of the
            job:</p>
         <ul>
            <li>
               <p>
                  <code>CREATE_PENDING</code> - The <code>CreationTime</code>.</p>
            </li>
            <li>
               <p>
                  <code>CREATE_IN_PROGRESS</code> - The current timestamp.</p>
            </li>
            <li>
               <p>
                  <code>CREATE_STOPPING</code> - The current timestamp.</p>
            </li>
            <li>
               <p>
                  <code>CREATE_STOPPED</code> - When the job stopped.</p>
            </li>
            <li>
               <p>
                  <code>ACTIVE</code> or <code>CREATE_FAILED</code> - When the job finished or
                    failed.</p>
            </li>
         </ul> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create explainability_export
explainability_export = provider.forecast.Explainability_export {
    destination = "value"  # Required field
    explainability_export_name = "value"  # <p>A unique name for the Explainability export.</p>
    explainability_arn = "value"  # <p>The Amazon Resource Name (ARN) of the Explainability to export.</p>
}

# Access explainability_export outputs
explainability_export_id = explainability_export.id
explainability_export_explainability_export_arn = explainability_export.explainability_export_arn
explainability_export_format = explainability_export.format
explainability_export_status = explainability_export.status
explainability_export_explainability_export_name = explainability_export.explainability_export_name
explainability_export_creation_time = explainability_export.creation_time
explainability_export_message = explainability_export.message
explainability_export_destination = explainability_export.destination
explainability_export_explainability_arn = explainability_export.explainability_arn
explainability_export_last_modification_time = explainability_export.last_modification_time
```

---


### Predictor

Predictor resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `forecast_types` | Vec<String> |  | <p>Specifies the forecast types used to train a predictor. You can specify up to five
      forecast types. Forecast types can be quantiles from 0.01 to 0.99, by increments of 0.01 or
      higher. You can also specify the mean forecast with <code>mean</code>. </p>
         <p>The default value is <code>["0.10", "0.50", "0.9"]</code>.</p> |
| `training_parameters` | HashMap<String, String> |  | <p>The hyperparameters to override for model training. The hyperparameters that you can
      override are listed in the individual algorithms. For the list of supported algorithms, see
        <a>aws-forecast-choosing-recipes</a>.</p> |
| `auto_ml_override_strategy` | String |  | <note>
            <p> The <code>LatencyOptimized</code> AutoML override strategy is only available in private beta.
                Contact Amazon Web Services Support or your account manager to learn more about access privileges.
            </p>
         </note>
         <p>Used to overide the default AutoML strategy, which is to optimize predictor accuracy.
            To apply an AutoML strategy that minimizes training time, use
                <code>LatencyOptimized</code>.</p>
         <p>This parameter is only valid for predictors trained using AutoML.</p> |
| `encryption_config` | String |  | <p>An Key Management Service (KMS) key and the Identity and Access Management (IAM) role that Amazon Forecast can assume to access
      the key.</p> |
| `algorithm_arn` | String |  | <p>The Amazon Resource Name (ARN) of the algorithm to use for model training. Required if
        <code>PerformAutoML</code> is not set to <code>true</code>.</p>
         <p class="title">
            <b>Supported algorithms:</b>
         </p>
         <ul>
            <li>
               <p>
                  <code>arn:aws:forecast:::algorithm/ARIMA</code>
               </p>
            </li>
            <li>
               <p>
                  <code>arn:aws:forecast:::algorithm/CNN-QR</code>
               </p>
            </li>
            <li>
               <p>
                  <code>arn:aws:forecast:::algorithm/Deep_AR_Plus</code>
               </p>
            </li>
            <li>
               <p>
                  <code>arn:aws:forecast:::algorithm/ETS</code>
               </p>
            </li>
            <li>
               <p>
                  <code>arn:aws:forecast:::algorithm/NPTS</code>
               </p>
            </li>
            <li>
               <p>
                  <code>arn:aws:forecast:::algorithm/Prophet</code>
               </p>
            </li>
         </ul> |
| `forecast_horizon` | i64 | ✅ | <p>Specifies the number of time-steps that the model is trained to predict. The forecast
      horizon is also called the prediction length.</p>
         <p>For example, if you configure a dataset for daily data collection (using the
        <code>DataFrequency</code> parameter of the <a>CreateDataset</a> operation) and
      set the forecast horizon to 10, the model returns predictions for 10 days.</p>
         <p>The maximum forecast horizon is the lesser of 500 time-steps or 1/3 of the
      TARGET_TIME_SERIES dataset length.</p> |
| `evaluation_parameters` | String |  | <p>Used to override the default evaluation parameters of the specified algorithm. Amazon Forecast
      evaluates a predictor by splitting a dataset into training data and testing data. The
      evaluation parameters define how to perform the split and the number of iterations.</p> |
| `tags` | Vec<String> |  | <p>The optional metadata that you apply to the predictor to help you categorize and organize
      them. Each tag consists of a key and an optional value, both of which you define.</p>
         <p>The following basic restrictions apply to tags:</p>
         <ul>
            <li>
               <p>Maximum number of tags per resource - 50.</p>
            </li>
            <li>
               <p>For each resource, each tag key must be unique, and each tag key can have only one
          value.</p>
            </li>
            <li>
               <p>Maximum key length - 128 Unicode characters in UTF-8.</p>
            </li>
            <li>
               <p>Maximum value length - 256 Unicode characters in UTF-8.</p>
            </li>
            <li>
               <p>If your tagging schema is used across multiple services and resources, remember that
          other services may have restrictions on allowed characters. Generally allowed characters
          are: letters, numbers, and spaces representable in UTF-8, and the following characters: +
          - = . _ : / @.</p>
            </li>
            <li>
               <p>Tag keys and values are case sensitive.</p>
            </li>
            <li>
               <p>Do not use <code>aws:</code>, <code>AWS:</code>, or any upper or lowercase combination
          of such as a prefix for keys as it is reserved for Amazon Web Services use. You cannot edit or delete tag
          keys with this prefix. Values can have this prefix. If a tag value has <code>aws</code> as
          its prefix but the key does not, then Forecast considers it to be a user tag and will
          count against the limit of 50 tags. Tags with only the key prefix of <code>aws</code> do
          not count against your tags per resource limit.</p>
            </li>
         </ul> |
| `input_data_config` | String | ✅ | <p>Describes the dataset group that contains the data to use to train the predictor.</p> |
| `featurization_config` | String | ✅ | <p>The featurization configuration.</p> |
| `perform_auto_ml` | bool |  | <p>Whether to perform AutoML. When Amazon Forecast performs AutoML, it evaluates the algorithms it
      provides and chooses the best algorithm and configuration for your training dataset.</p>
         <p>The default value is <code>false</code>. In this case, you are required to specify an
      algorithm.</p>
         <p>Set <code>PerformAutoML</code> to <code>true</code> to have Amazon Forecast perform AutoML. This
      is a good option if you aren't sure which algorithm is suitable for your training data. In
      this case, <code>PerformHPO</code> must be false.</p> |
| `hpo_config` | String |  | <p>Provides hyperparameter override values for the algorithm. If you don't provide this
      parameter, Amazon Forecast uses default values. The individual algorithms specify which
      hyperparameters support hyperparameter optimization (HPO). For more information, see <a>aws-forecast-choosing-recipes</a>.</p>
         <p>If you included the <code>HPOConfig</code> object, you must set <code>PerformHPO</code> to
      true.</p> |
| `optimization_metric` | String |  | <p>The accuracy metric used to optimize the predictor.</p> |
| `predictor_name` | String | ✅ | <p>A name for the predictor.</p> |
| `perform_hpo` | bool |  | <p>Whether to perform hyperparameter optimization (HPO). HPO finds optimal hyperparameter
      values for your training data. The process of performing HPO is known as running a
      hyperparameter tuning job.</p>
         <p>The default value is <code>false</code>. In this case, Amazon Forecast uses default
      hyperparameter values from the chosen algorithm.</p>
         <p>To override the default values, set <code>PerformHPO</code> to <code>true</code> and,
      optionally, supply the <a>HyperParameterTuningJobConfig</a> object. The tuning job
      specifies a metric to optimize, which hyperparameters participate in tuning, and the valid
      range for each tunable hyperparameter. In this case, you are required to specify an algorithm
      and <code>PerformAutoML</code> must be false.</p>
         <p>The following algorithms support HPO:</p>
         <ul>
            <li>
               <p>DeepAR+</p>
            </li>
            <li>
               <p>CNN-QR</p>
            </li>
         </ul> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `estimated_time_remaining_in_minutes` | i64 | <p>The estimated time remaining in minutes for the predictor training job to complete.</p> |
| `hpo_config` | String | <p>The hyperparameter override values for the algorithm.</p> |
| `evaluation_parameters` | String | <p>Used to override the default evaluation parameters of the specified algorithm. Amazon Forecast
      evaluates a predictor by splitting a dataset into training data and testing data. The
      evaluation parameters define how to perform the split and the number of iterations.</p> |
| `forecast_horizon` | i64 | <p>The number of time-steps of the forecast. The forecast horizon is also called the
      prediction length.</p> |
| `input_data_config` | String | <p>Describes the dataset group that contains the data to use to train the predictor.</p> |
| `perform_hpo` | bool | <p>Whether the predictor is set to perform hyperparameter optimization (HPO).</p> |
| `encryption_config` | String | <p>An Key Management Service (KMS) key and the Identity and Access Management (IAM) role that Amazon Forecast can assume to access
      the key.</p> |
| `is_auto_predictor` | bool | <p>Whether the predictor was created with <a>CreateAutoPredictor</a>.</p> |
| `auto_ml_override_strategy` | String | <note>
            <p> The <code>LatencyOptimized</code> AutoML override strategy is only available in private beta.
                Contact Amazon Web Services Support or your account manager to learn more about access privileges.
            </p>
         </note>
         <p>The AutoML strategy used to train the predictor. Unless <code>LatencyOptimized</code>
            is specified, the AutoML strategy optimizes predictor accuracy.</p>
         <p>This parameter is only valid for predictors trained using AutoML.</p> |
| `dataset_import_job_arns` | Vec<String> | <p>An array of the ARNs of the dataset import jobs used to import training data for the
      predictor.</p> |
| `last_modification_time` | String | <p>The last time the resource was modified. The timestamp depends on the status of the
      job:</p>
         <ul>
            <li>
               <p>
                  <code>CREATE_PENDING</code> - The <code>CreationTime</code>.</p>
            </li>
            <li>
               <p>
                  <code>CREATE_IN_PROGRESS</code> - The current timestamp.</p>
            </li>
            <li>
               <p>
                  <code>CREATE_STOPPING</code> - The current timestamp.</p>
            </li>
            <li>
               <p>
                  <code>CREATE_STOPPED</code> - When the job stopped.</p>
            </li>
            <li>
               <p>
                  <code>ACTIVE</code> or <code>CREATE_FAILED</code> - When the job finished or
          failed.</p>
            </li>
         </ul> |
| `featurization_config` | String | <p>The featurization configuration.</p> |
| `predictor_execution_details` | String | <p>Details on the the status and results of the backtests performed to evaluate the accuracy
      of the predictor. You specify the number of backtests to perform when you call the  operation.</p> |
| `optimization_metric` | String | <p>The accuracy metric used to optimize the predictor.</p> |
| `algorithm_arn` | String | <p>The Amazon Resource Name (ARN) of the algorithm used for model training.</p> |
| `message` | String | <p>If an error occurred, an informational message about the error.</p> |
| `forecast_types` | Vec<String> | <p>The forecast types used during predictor training. Default value is
        <code>["0.1","0.5","0.9"]</code>
         </p> |
| `auto_ml_algorithm_arns` | Vec<String> | <p>When <code>PerformAutoML</code> is specified, the ARN of the chosen algorithm.</p> |
| `predictor_arn` | String | <p>The ARN of the predictor.</p> |
| `perform_auto_ml` | bool | <p>Whether the predictor is set to perform AutoML.</p> |
| `predictor_name` | String | <p>The name of the predictor.</p> |
| `status` | String | <p>The status of the predictor. States include:</p>
         <ul>
            <li>
               <p>
                  <code>ACTIVE</code>
               </p>
            </li>
            <li>
               <p>
                  <code>CREATE_PENDING</code>, <code>CREATE_IN_PROGRESS</code>,
            <code>CREATE_FAILED</code>
               </p>
            </li>
            <li>
               <p>
                  <code>DELETE_PENDING</code>, <code>DELETE_IN_PROGRESS</code>,
            <code>DELETE_FAILED</code>
               </p>
            </li>
            <li>
               <p>
                  <code>CREATE_STOPPING</code>, <code>CREATE_STOPPED</code>
               </p>
            </li>
         </ul>
         <note>
            <p>The <code>Status</code> of the predictor must be <code>ACTIVE</code> before you can use
        the predictor to create a forecast.</p>
         </note> |
| `training_parameters` | HashMap<String, String> | <p>The default training parameters or overrides selected during model training. When running
      AutoML or choosing HPO with CNN-QR or DeepAR+, the optimized values for the chosen
      hyperparameters are returned. For more information, see <a>aws-forecast-choosing-recipes</a>.</p> |
| `creation_time` | String | <p>When the model training task was created.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create predictor
predictor = provider.forecast.Predictor {
    forecast_horizon = "value"  # <p>Specifies the number of time-steps that the model is trained to predict. The forecast
      horizon is also called the prediction length.</p>
         <p>For example, if you configure a dataset for daily data collection (using the
        <code>DataFrequency</code> parameter of the <a>CreateDataset</a> operation) and
      set the forecast horizon to 10, the model returns predictions for 10 days.</p>
         <p>The maximum forecast horizon is the lesser of 500 time-steps or 1/3 of the
      TARGET_TIME_SERIES dataset length.</p>
    input_data_config = "value"  # <p>Describes the dataset group that contains the data to use to train the predictor.</p>
    featurization_config = "value"  # <p>The featurization configuration.</p>
    predictor_name = "value"  # <p>A name for the predictor.</p>
}

# Access predictor outputs
predictor_id = predictor.id
predictor_estimated_time_remaining_in_minutes = predictor.estimated_time_remaining_in_minutes
predictor_hpo_config = predictor.hpo_config
predictor_evaluation_parameters = predictor.evaluation_parameters
predictor_forecast_horizon = predictor.forecast_horizon
predictor_input_data_config = predictor.input_data_config
predictor_perform_hpo = predictor.perform_hpo
predictor_encryption_config = predictor.encryption_config
predictor_is_auto_predictor = predictor.is_auto_predictor
predictor_auto_ml_override_strategy = predictor.auto_ml_override_strategy
predictor_dataset_import_job_arns = predictor.dataset_import_job_arns
predictor_last_modification_time = predictor.last_modification_time
predictor_featurization_config = predictor.featurization_config
predictor_predictor_execution_details = predictor.predictor_execution_details
predictor_optimization_metric = predictor.optimization_metric
predictor_algorithm_arn = predictor.algorithm_arn
predictor_message = predictor.message
predictor_forecast_types = predictor.forecast_types
predictor_auto_ml_algorithm_arns = predictor.auto_ml_algorithm_arns
predictor_predictor_arn = predictor.predictor_arn
predictor_perform_auto_ml = predictor.perform_auto_ml
predictor_predictor_name = predictor.predictor_name
predictor_status = predictor.status
predictor_training_parameters = predictor.training_parameters
predictor_creation_time = predictor.creation_time
```

---


### Explainability

Explainability resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `start_date_time` | String |  | <p>If <code>TimePointGranularity</code> is set to <code>SPECIFIC</code>, define the first
            point for the Explainability.</p>
         <p>Use the following timestamp format: yyyy-MM-ddTHH:mm:ss (example:
            2015-01-01T20:00:00)</p> |
| `tags` | Vec<String> |  | <p>Optional metadata to help you categorize and organize your resources. Each tag
            consists of a key and an optional value, both of which you define. Tag keys and values
            are case sensitive.</p>
         <p>The following restrictions apply to tags:</p>
         <ul>
            <li>
               <p>For each resource, each tag key must be unique and each tag key must have one
                    value.</p>
            </li>
            <li>
               <p>Maximum number of tags per resource: 50.</p>
            </li>
            <li>
               <p>Maximum key length: 128 Unicode characters in UTF-8.</p>
            </li>
            <li>
               <p>Maximum value length: 256 Unicode characters in UTF-8.</p>
            </li>
            <li>
               <p>Accepted characters: all letters and numbers, spaces representable in UTF-8,
                    and + - = . _ : / @. If your tagging schema is used across other services and
                    resources, the character restrictions of those services also apply. </p>
            </li>
            <li>
               <p>Key prefixes cannot include any upper or lowercase combination of
                        <code>aws:</code> or <code>AWS:</code>. Values can have this prefix. If a
                    tag value has <code>aws</code> as its prefix but the key does not, Forecast
                    considers it to be a user tag and will count against the limit of 50 tags. Tags
                    with only the key prefix of <code>aws</code> do not count against your tags per
                    resource limit. You cannot edit or delete tag keys with this prefix.</p>
            </li>
         </ul> |
| `explainability_config` | String | ✅ | <p>The configuration settings that define the granularity of time series and time points
            for the Explainability.</p> |
| `resource_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the Predictor or Forecast used to create the
            Explainability.</p> |
| `schema` | String |  |  |
| `data_source` | String |  |  |
| `explainability_name` | String | ✅ | <p>A unique name for the Explainability.</p> |
| `enable_visualization` | bool |  | <p>Create an Explainability visualization that is viewable within the Amazon Web Services console.</p> |
| `end_date_time` | String |  | <p>If <code>TimePointGranularity</code> is set to <code>SPECIFIC</code>, define the last
            time point for the Explainability.</p>
         <p>Use the following timestamp format: yyyy-MM-ddTHH:mm:ss (example:
            2015-01-01T20:00:00)</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `end_date_time` | String | <p>If <code>TimePointGranularity</code> is set to <code>SPECIFIC</code>, the last time
            point in the Explainability.</p> |
| `last_modification_time` | String | <p>The last time the resource was modified. The timestamp depends on the status of the
            job:</p>
         <ul>
            <li>
               <p>
                  <code>CREATE_PENDING</code> - The <code>CreationTime</code>.</p>
            </li>
            <li>
               <p>
                  <code>CREATE_IN_PROGRESS</code> - The current timestamp.</p>
            </li>
            <li>
               <p>
                  <code>CREATE_STOPPING</code> - The current timestamp.</p>
            </li>
            <li>
               <p>
                  <code>CREATE_STOPPED</code> - When the job stopped.</p>
            </li>
            <li>
               <p>
                  <code>ACTIVE</code> or <code>CREATE_FAILED</code> - When the job finished or
                    failed.</p>
            </li>
         </ul> |
| `data_source` | String |  |
| `start_date_time` | String | <p>If <code>TimePointGranularity</code> is set to <code>SPECIFIC</code>, the first time
            point in the Explainability.</p> |
| `creation_time` | String | <p>When the Explainability resource was created.</p> |
| `status` | String | <p>The status of the Explainability resource. States include: </p>
         <ul>
            <li>
               <p>
                  <code>ACTIVE</code>
               </p>
            </li>
            <li>
               <p>
                  <code>CREATE_PENDING</code>, <code>CREATE_IN_PROGRESS</code>,
                        <code>CREATE_FAILED</code>
               </p>
            </li>
            <li>
               <p>
                  <code>CREATE_STOPPING</code>, <code>CREATE_STOPPED</code>
               </p>
            </li>
            <li>
               <p>
                  <code>DELETE_PENDING</code>, <code>DELETE_IN_PROGRESS</code>,
                        <code>DELETE_FAILED</code>
               </p>
            </li>
         </ul> |
| `explainability_name` | String | <p>The name of the Explainability.</p> |
| `explainability_config` | String | <p>The configuration settings that define the granularity of time series and time points
            for the Explainability.</p> |
| `estimated_time_remaining_in_minutes` | i64 | <p>The estimated time remaining in minutes for the <a>CreateExplainability</a>
            job to complete.</p> |
| `message` | String | <p>If an error occurred, a message about the error.</p> |
| `explainability_arn` | String | <p>The Amazon Resource Name (ARN) of the Explainability.</p> |
| `resource_arn` | String | <p>The Amazon Resource Name (ARN) of the Predictor or Forecast used to create the
            Explainability resource.</p> |
| `enable_visualization` | bool | <p>Whether the visualization was enabled for the Explainability resource.</p> |
| `schema` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create explainability
explainability = provider.forecast.Explainability {
    explainability_config = "value"  # <p>The configuration settings that define the granularity of time series and time points
            for the Explainability.</p>
    resource_arn = "value"  # <p>The Amazon Resource Name (ARN) of the Predictor or Forecast used to create the
            Explainability.</p>
    explainability_name = "value"  # <p>A unique name for the Explainability.</p>
}

# Access explainability outputs
explainability_id = explainability.id
explainability_end_date_time = explainability.end_date_time
explainability_last_modification_time = explainability.last_modification_time
explainability_data_source = explainability.data_source
explainability_start_date_time = explainability.start_date_time
explainability_creation_time = explainability.creation_time
explainability_status = explainability.status
explainability_explainability_name = explainability.explainability_name
explainability_explainability_config = explainability.explainability_config
explainability_estimated_time_remaining_in_minutes = explainability.estimated_time_remaining_in_minutes
explainability_message = explainability.message
explainability_explainability_arn = explainability.explainability_arn
explainability_resource_arn = explainability.resource_arn
explainability_enable_visualization = explainability.enable_visualization
explainability_schema = explainability.schema
```

---


### Monitor

Monitor resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `monitor_name` | String | ✅ | <p>The name of the monitor resource.</p> |
| `resource_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the predictor to monitor.</p> |
| `tags` | Vec<String> |  | <p>A list of <a href="https://docs.aws.amazon.com/forecast/latest/dg/tagging-forecast-resources.html">tags</a> to apply to the monitor resource.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `last_modification_time` | String | <p>The timestamp of the latest modification to the monitor.</p> |
| `estimated_evaluation_time_remaining_in_minutes` | i64 | <p>The estimated number of minutes remaining before the monitor resource finishes its current evaluation.</p> |
| `creation_time` | String | <p>The timestamp for when the monitor resource was created.</p> |
| `status` | String | <p>The status of the monitor resource.</p> |
| `last_evaluation_time` | String | <p>The timestamp of the latest evaluation completed by the monitor.</p> |
| `message` | String | <p>An error message, if any, for the monitor.</p> |
| `baseline` | String | <p>Metrics you can use as a baseline for comparison purposes. Use these values you interpret monitoring results for an auto predictor.</p> |
| `monitor_name` | String | <p>The name of the monitor.</p> |
| `last_evaluation_state` | String | <p>The state of the monitor's latest evaluation.</p> |
| `monitor_arn` | String | <p>The Amazon Resource Name (ARN) of the monitor resource described.</p> |
| `resource_arn` | String | <p>The Amazon Resource Name (ARN) of the auto predictor being monitored.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create monitor
monitor = provider.forecast.Monitor {
    monitor_name = "value"  # <p>The name of the monitor resource.</p>
    resource_arn = "value"  # <p>The Amazon Resource Name (ARN) of the predictor to monitor.</p>
}

# Access monitor outputs
monitor_id = monitor.id
monitor_last_modification_time = monitor.last_modification_time
monitor_estimated_evaluation_time_remaining_in_minutes = monitor.estimated_evaluation_time_remaining_in_minutes
monitor_creation_time = monitor.creation_time
monitor_status = monitor.status
monitor_last_evaluation_time = monitor.last_evaluation_time
monitor_message = monitor.message
monitor_baseline = monitor.baseline
monitor_monitor_name = monitor.monitor_name
monitor_last_evaluation_state = monitor.last_evaluation_state
monitor_monitor_arn = monitor.monitor_arn
monitor_resource_arn = monitor.resource_arn
```

---


### What_if_analysis

WhatIfAnalysis resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `what_if_analysis_name` | String | ✅ | <p>The name of the what-if analysis. Each name must be unique.</p> |
| `time_series_selector` | String |  | <p>Defines the set of time series that are used in the what-if analysis with a <code>TimeSeriesIdentifiers</code>
      object. What-if analyses are performed only for the time series in this object.</p>
         <p>The <code>TimeSeriesIdentifiers</code> object needs the following information:</p>
         <ul>
            <li>
               <p>
                  <code>DataSource</code>
               </p>
            </li>
            <li>
               <p>
                  <code>Format</code>
               </p>
            </li>
            <li>
               <p>
                  <code>Schema</code>
               </p>
            </li>
         </ul> |
| `tags` | Vec<String> |  | <p>A list of <a href="https://docs.aws.amazon.com/forecast/latest/dg/tagging-forecast-resources.html">tags</a> to apply to the what if forecast.</p> |
| `forecast_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the baseline forecast.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `what_if_analysis_arn` | String | <p>The Amazon Resource Name (ARN) of the what-if analysis.</p> |
| `last_modification_time` | String | <p>The last time the resource was modified. The timestamp depends on the status of the job:</p>
         <ul>
            <li>
               <p>
                  <code>CREATE_PENDING</code> - The <code>CreationTime</code>.</p>
            </li>
            <li>
               <p>
                  <code>CREATE_IN_PROGRESS</code> - The current timestamp.</p>
            </li>
            <li>
               <p>
                  <code>CREATE_STOPPING</code> - The current timestamp.</p>
            </li>
            <li>
               <p>
                  <code>CREATE_STOPPED</code> - When the job stopped.</p>
            </li>
            <li>
               <p>
                  <code>ACTIVE</code> or <code>CREATE_FAILED</code> - When the job finished or
          failed.</p>
            </li>
         </ul> |
| `message` | String | <p>If an error occurred, an informational message about the error.</p> |
| `estimated_time_remaining_in_minutes` | i64 | <p>The approximate time remaining to complete the what-if analysis, in minutes.</p> |
| `creation_time` | String | <p>When the what-if analysis was created.</p> |
| `time_series_selector` | String |  |
| `what_if_analysis_name` | String | <p>The name of the what-if analysis.</p> |
| `forecast_arn` | String | <p>The Amazon Resource Name (ARN) of the what-if forecast.</p> |
| `status` | String | <p>The status of the what-if analysis. States include:</p>
         <ul>
            <li>
               <p>
                  <code>ACTIVE</code>
               </p>
            </li>
            <li>
               <p>
                  <code>CREATE_PENDING</code>, <code>CREATE_IN_PROGRESS</code>,
          <code>CREATE_FAILED</code>
               </p>
            </li>
            <li>
               <p>
                  <code>CREATE_STOPPING</code>, <code>CREATE_STOPPED</code>
               </p>
            </li>
            <li>
               <p>
                  <code>DELETE_PENDING</code>, <code>DELETE_IN_PROGRESS</code>,
          <code>DELETE_FAILED</code>
               </p>
            </li>
         </ul>
         <note>
            <p>The <code>Status</code> of the what-if analysis must be <code>ACTIVE</code> before you can access the
        analysis.</p>
         </note> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create what_if_analysis
what_if_analysis = provider.forecast.What_if_analysis {
    what_if_analysis_name = "value"  # <p>The name of the what-if analysis. Each name must be unique.</p>
    forecast_arn = "value"  # <p>The Amazon Resource Name (ARN) of the baseline forecast.</p>
}

# Access what_if_analysis outputs
what_if_analysis_id = what_if_analysis.id
what_if_analysis_what_if_analysis_arn = what_if_analysis.what_if_analysis_arn
what_if_analysis_last_modification_time = what_if_analysis.last_modification_time
what_if_analysis_message = what_if_analysis.message
what_if_analysis_estimated_time_remaining_in_minutes = what_if_analysis.estimated_time_remaining_in_minutes
what_if_analysis_creation_time = what_if_analysis.creation_time
what_if_analysis_time_series_selector = what_if_analysis.time_series_selector
what_if_analysis_what_if_analysis_name = what_if_analysis.what_if_analysis_name
what_if_analysis_forecast_arn = what_if_analysis.forecast_arn
what_if_analysis_status = what_if_analysis.status
```

---


### Forecast

Forecast resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `predictor_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the predictor to use to generate the forecast.</p> |
| `forecast_types` | Vec<String> |  | <p>The quantiles at which probabilistic forecasts are generated. <b>You
        can currently specify up to 5 quantiles per forecast</b>. Accepted values include
        <code>0.01 to 0.99</code> (increments of .01 only) and <code>mean</code>. The mean forecast
      is different from the median (0.50) when the distribution is not symmetric (for example, Beta
      and Negative Binomial).
      </p>
         <p>The default quantiles are the quantiles you specified during predictor creation.
      If you didn't specify quantiles, the default values are <code>["0.1", "0.5", "0.9"]</code>.
    </p> |
| `forecast_name` | String | ✅ | <p>A name for the forecast.</p> |
| `tags` | Vec<String> |  | <p>The optional metadata that you apply to the forecast to help you categorize and organize
      them. Each tag consists of a key and an optional value, both of which you define.</p>
         <p>The following basic restrictions apply to tags:</p>
         <ul>
            <li>
               <p>Maximum number of tags per resource - 50.</p>
            </li>
            <li>
               <p>For each resource, each tag key must be unique, and each tag key can have only one
          value.</p>
            </li>
            <li>
               <p>Maximum key length - 128 Unicode characters in UTF-8.</p>
            </li>
            <li>
               <p>Maximum value length - 256 Unicode characters in UTF-8.</p>
            </li>
            <li>
               <p>If your tagging schema is used across multiple services and resources, remember that
          other services may have restrictions on allowed characters. Generally allowed characters
          are: letters, numbers, and spaces representable in UTF-8, and the following characters: +
          - = . _ : / @.</p>
            </li>
            <li>
               <p>Tag keys and values are case sensitive.</p>
            </li>
            <li>
               <p>Do not use <code>aws:</code>, <code>AWS:</code>, or any upper or lowercase combination
          of such as a prefix for keys as it is reserved for Amazon Web Services use. You cannot edit or delete tag
          keys with this prefix. Values can have this prefix. If a tag value has <code>aws</code> as
          its prefix but the key does not, then Forecast considers it to be a user tag and will
          count against the limit of 50 tags. Tags with only the key prefix of <code>aws</code> do
          not count against your tags per resource limit.</p>
            </li>
         </ul> |
| `time_series_selector` | String |  | <p>Defines the set of time series that are used to create the forecasts in a <code>TimeSeriesIdentifiers</code> object.</p>
         <p>The <code>TimeSeriesIdentifiers</code> object needs the following information:</p>
         <ul>
            <li>
               <p>
                  <code>DataSource</code>
               </p>
            </li>
            <li>
               <p>
                  <code>Format</code>
               </p>
            </li>
            <li>
               <p>
                  <code>Schema</code>
               </p>
            </li>
         </ul> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `forecast_types` | Vec<String> | <p>The quantiles at which probabilistic forecasts were generated.</p> |
| `creation_time` | String | <p>When the forecast creation task was created.</p> |
| `forecast_arn` | String | <p>The forecast ARN as specified in the request.</p> |
| `forecast_name` | String | <p>The name of the forecast.</p> |
| `predictor_arn` | String | <p>The ARN of the predictor used to generate the forecast.</p> |
| `dataset_group_arn` | String | <p>The ARN of the dataset group that provided the data used to train the predictor.</p> |
| `estimated_time_remaining_in_minutes` | i64 | <p>The estimated time remaining in minutes for the forecast job to complete.</p> |
| `status` | String | <p>The status of the forecast. States include:</p>
         <ul>
            <li>
               <p>
                  <code>ACTIVE</code>
               </p>
            </li>
            <li>
               <p>
                  <code>CREATE_PENDING</code>, <code>CREATE_IN_PROGRESS</code>,
          <code>CREATE_FAILED</code>
               </p>
            </li>
            <li>
               <p>
                  <code>CREATE_STOPPING</code>, <code>CREATE_STOPPED</code>
               </p>
            </li>
            <li>
               <p>
                  <code>DELETE_PENDING</code>, <code>DELETE_IN_PROGRESS</code>,
          <code>DELETE_FAILED</code>
               </p>
            </li>
         </ul>
         <note>
            <p>The <code>Status</code> of the forecast must be <code>ACTIVE</code> before you can query
        or export the forecast.</p>
         </note> |
| `last_modification_time` | String | <p>The last time the resource was modified. The timestamp depends on the status of the job:</p>
         <ul>
            <li>
               <p>
                  <code>CREATE_PENDING</code> - The <code>CreationTime</code>.</p>
            </li>
            <li>
               <p>
                  <code>CREATE_IN_PROGRESS</code> - The current timestamp.</p>
            </li>
            <li>
               <p>
                  <code>CREATE_STOPPING</code> - The current timestamp.</p>
            </li>
            <li>
               <p>
                  <code>CREATE_STOPPED</code> - When the job stopped.</p>
            </li>
            <li>
               <p>
                  <code>ACTIVE</code> or <code>CREATE_FAILED</code> - When the job finished or
          failed.</p>
            </li>
         </ul> |
| `time_series_selector` | String | <p>The time series to include in the forecast.</p> |
| `message` | String | <p>If an error occurred, an informational message about the error.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create forecast
forecast = provider.forecast.Forecast {
    predictor_arn = "value"  # <p>The Amazon Resource Name (ARN) of the predictor to use to generate the forecast.</p>
    forecast_name = "value"  # <p>A name for the forecast.</p>
}

# Access forecast outputs
forecast_id = forecast.id
forecast_forecast_types = forecast.forecast_types
forecast_creation_time = forecast.creation_time
forecast_forecast_arn = forecast.forecast_arn
forecast_forecast_name = forecast.forecast_name
forecast_predictor_arn = forecast.predictor_arn
forecast_dataset_group_arn = forecast.dataset_group_arn
forecast_estimated_time_remaining_in_minutes = forecast.estimated_time_remaining_in_minutes
forecast_status = forecast.status
forecast_last_modification_time = forecast.last_modification_time
forecast_time_series_selector = forecast.time_series_selector
forecast_message = forecast.message
```

---


### What_if_forecast

WhatIfForecast resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `what_if_forecast_name` | String | ✅ | <p>The name of the what-if forecast. Names must be unique within each what-if analysis.</p> |
| `time_series_replacements_data_source` | String |  | <p>The replacement time series dataset, which contains the rows that you want to change in the related time
      series dataset. A replacement time series does not need to contain all rows that are in the baseline related time
      series. Include only the rows (measure-dimension combinations) that you want to include in the what-if
      forecast.</p>
         <p>This dataset is merged with the
      original time series to create a transformed dataset that is used for the what-if analysis.</p>
         <p>This dataset should contain the items to modify (such as item_id or workforce_type), any relevant dimensions, the timestamp column, and at least one of the related time series columns. This file should not contain duplicate timestamps for the same time series.</p>
         <p>Timestamps and item_ids not included in this dataset are not included in the what-if analysis. </p> |
| `what_if_analysis_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the what-if analysis.</p> |
| `tags` | Vec<String> |  | <p>A list of <a href="https://docs.aws.amazon.com/forecast/latest/dg/tagging-forecast-resources.html">tags</a> to apply to the what if forecast.</p> |
| `time_series_transformations` | Vec<String> |  | <p>The transformations that are applied to the baseline time series. Each transformation contains an action and a set of conditions. An action is applied only when all conditions are met. If no conditions are provided, the action is applied to all items.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `creation_time` | String | <p>When the what-if forecast was created.</p> |
| `message` | String | <p>If an error occurred, an informational message about the error.</p> |
| `status` | String | <p>The status of the what-if forecast. States include:</p>
         <ul>
            <li>
               <p>
                  <code>ACTIVE</code>
               </p>
            </li>
            <li>
               <p>
                  <code>CREATE_PENDING</code>, <code>CREATE_IN_PROGRESS</code>,
          <code>CREATE_FAILED</code>
               </p>
            </li>
            <li>
               <p>
                  <code>CREATE_STOPPING</code>, <code>CREATE_STOPPED</code>
               </p>
            </li>
            <li>
               <p>
                  <code>DELETE_PENDING</code>, <code>DELETE_IN_PROGRESS</code>,
          <code>DELETE_FAILED</code>
               </p>
            </li>
         </ul>
         <note>
            <p>The <code>Status</code> of the what-if forecast must be <code>ACTIVE</code> before you can access the
        forecast.</p>
         </note> |
| `what_if_analysis_arn` | String | <p>The Amazon Resource Name (ARN) of the what-if analysis that contains this forecast.</p> |
| `what_if_forecast_name` | String | <p>The name of the what-if forecast.</p> |
| `last_modification_time` | String | <p>The last time the resource was modified. The timestamp depends on the status of the job:</p>
         <ul>
            <li>
               <p>
                  <code>CREATE_PENDING</code> - The <code>CreationTime</code>.</p>
            </li>
            <li>
               <p>
                  <code>CREATE_IN_PROGRESS</code> - The current timestamp.</p>
            </li>
            <li>
               <p>
                  <code>CREATE_STOPPING</code> - The current timestamp.</p>
            </li>
            <li>
               <p>
                  <code>CREATE_STOPPED</code> - When the job stopped.</p>
            </li>
            <li>
               <p>
                  <code>ACTIVE</code> or <code>CREATE_FAILED</code> - When the job finished or
          failed.</p>
            </li>
         </ul> |
| `time_series_replacements_data_source` | String | <p>An array of <code>S3Config</code>, <code>Schema</code>, and <code>Format</code> elements that describe the replacement time series.</p> |
| `estimated_time_remaining_in_minutes` | i64 | <p>The approximate time remaining to complete the what-if forecast, in minutes.</p> |
| `time_series_transformations` | Vec<String> | <p>An array of <code>Action</code> and <code>TimeSeriesConditions</code> elements that describe what transformations were applied to which time series.</p> |
| `what_if_forecast_arn` | i64 | <p>The Amazon Resource Name (ARN) of the what-if forecast.</p> |
| `forecast_types` | Vec<String> | <p>The quantiles at which probabilistic forecasts are generated. You can specify up to five quantiles per what-if
      forecast in the <a>CreateWhatIfForecast</a> operation. If you didn't specify quantiles, the default
      values are <code>["0.1", "0.5", "0.9"]</code>. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create what_if_forecast
what_if_forecast = provider.forecast.What_if_forecast {
    what_if_forecast_name = "value"  # <p>The name of the what-if forecast. Names must be unique within each what-if analysis.</p>
    what_if_analysis_arn = "value"  # <p>The Amazon Resource Name (ARN) of the what-if analysis.</p>
}

# Access what_if_forecast outputs
what_if_forecast_id = what_if_forecast.id
what_if_forecast_creation_time = what_if_forecast.creation_time
what_if_forecast_message = what_if_forecast.message
what_if_forecast_status = what_if_forecast.status
what_if_forecast_what_if_analysis_arn = what_if_forecast.what_if_analysis_arn
what_if_forecast_what_if_forecast_name = what_if_forecast.what_if_forecast_name
what_if_forecast_last_modification_time = what_if_forecast.last_modification_time
what_if_forecast_time_series_replacements_data_source = what_if_forecast.time_series_replacements_data_source
what_if_forecast_estimated_time_remaining_in_minutes = what_if_forecast.estimated_time_remaining_in_minutes
what_if_forecast_time_series_transformations = what_if_forecast.time_series_transformations
what_if_forecast_what_if_forecast_arn = what_if_forecast.what_if_forecast_arn
what_if_forecast_forecast_types = what_if_forecast.forecast_types
```

---


### Auto_predictor

AutoPredictor resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `predictor_name` | String | ✅ | <p>A unique name for the predictor</p> |
| `forecast_dimensions` | Vec<String> |  | <p>An array of dimension (field) names that specify how to group the generated
            forecast.</p>
         <p>For example, if you are generating forecasts for item sales across all your stores,
            and your dataset contains a <code>store_id</code> field, you would specify
                <code>store_id</code> as a dimension to group sales forecasts for each store.</p> |
| `forecast_horizon` | i64 |  | <p>The number of time-steps that the model predicts. The forecast horizon is also called
            the prediction length.</p>
         <p>The maximum forecast horizon is the lesser of 500 time-steps or 1/4 of the
            TARGET_TIME_SERIES dataset length. If you are retraining an existing AutoPredictor, then
            the maximum forecast horizon is the lesser of 500 time-steps or 1/3 of the
            TARGET_TIME_SERIES dataset length.</p>
         <p>If you are upgrading to an AutoPredictor or retraining an existing AutoPredictor, you
            cannot update the forecast horizon parameter. You can meet this requirement by providing
            longer time-series in the dataset.</p> |
| `monitor_config` | String |  | <p>The configuration details for predictor monitoring. Provide a name for the monitor resource to enable predictor monitoring.</p>
         <p>Predictor monitoring allows you to see how your predictor's performance changes over time.
         For more information, see <a href="https://docs.aws.amazon.com/forecast/latest/dg/predictor-monitoring.html">Predictor Monitoring</a>.</p> |
| `data_config` | String |  | <p>The data configuration for your dataset group and any additional datasets.</p> |
| `encryption_config` | String |  |  |
| `reference_predictor_arn` | String |  | <p>The ARN of the predictor to retrain or upgrade. This parameter is only used when
            retraining or upgrading a predictor. When creating a new predictor, do not specify a
            value for this parameter.</p>
         <p>When upgrading or retraining a predictor, only specify values for the
                <code>ReferencePredictorArn</code> and <code>PredictorName</code>. The value for
                <code>PredictorName</code> must be a unique predictor name.</p> |
| `forecast_frequency` | String |  | <p>The frequency of predictions in a forecast.</p>
         <p>Valid intervals are an integer followed by Y (Year), M (Month), W (Week), D (Day), H (Hour), and min (Minute). For example,
        "1D" indicates every day and "15min" indicates every 15 minutes. You cannot specify a value that would overlap with the next larger frequency. That means, for example, you cannot specify a frequency of 60 minutes, because that is equivalent to 1 hour. The valid values for each frequency are the following:</p>
         <ul>
            <li>
               <p>Minute - 1-59</p>
            </li>
            <li>
               <p>Hour - 1-23</p>
            </li>
            <li>
               <p>Day - 1-6</p>
            </li>
            <li>
               <p>Week - 1-4</p>
            </li>
            <li>
               <p>Month - 1-11</p>
            </li>
            <li>
               <p>Year - 1</p>
            </li>
         </ul>
         <p>Thus, if you want every other week forecasts, specify "2W". Or, if you want quarterly forecasts, you specify "3M".</p>
         <p>The frequency must be greater than or equal to the TARGET_TIME_SERIES dataset
            frequency.</p>
         <p>When a RELATED_TIME_SERIES dataset is provided, the frequency must be equal to the
            RELATED_TIME_SERIES dataset frequency.</p> |
| `forecast_types` | Vec<String> |  | <p>The forecast types used to train a predictor. You can specify up to five forecast
            types. Forecast types can be quantiles from 0.01 to 0.99, by increments of 0.01 or
            higher. You can also specify the mean forecast with <code>mean</code>.</p> |
| `explain_predictor` | bool |  | <p>Create an Explainability resource for the predictor.</p> |
| `optimization_metric` | String |  | <p>The accuracy metric used to optimize the predictor.</p> |
| `tags` | Vec<String> |  | <p>Optional metadata to help you categorize and organize your predictors. Each tag
            consists of a key and an optional value, both of which you define. Tag keys and values
            are case sensitive.</p>
         <p>The following restrictions apply to tags:</p>
         <ul>
            <li>
               <p>For each resource, each tag key must be unique and each tag key must have one
                    value.</p>
            </li>
            <li>
               <p>Maximum number of tags per resource: 50.</p>
            </li>
            <li>
               <p>Maximum key length: 128 Unicode characters in UTF-8.</p>
            </li>
            <li>
               <p>Maximum value length: 256 Unicode characters in UTF-8.</p>
            </li>
            <li>
               <p>Accepted characters: all letters and numbers, spaces representable in UTF-8,
                    and + - = . _ : / @. If your tagging schema is used across other services and
                    resources, the character restrictions of those services also apply. </p>
            </li>
            <li>
               <p>Key prefixes cannot include any upper or lowercase combination of
                        <code>aws:</code> or <code>AWS:</code>. Values can have this prefix. If a
                    tag value has <code>aws</code> as its prefix but the key does not, Forecast
                    considers it to be a user tag and will count against the limit of 50 tags. Tags
                    with only the key prefix of <code>aws</code> do not count against your tags per
                    resource limit. You cannot edit or delete tag keys with this prefix.</p>
            </li>
         </ul> |
| `time_alignment_boundary` | String |  | <p>The time boundary Forecast uses to align and aggregate any data that doesn't align with your forecast frequency. Provide the unit of time and the time boundary as a key value pair. 
            For more information on specifying a time boundary, see <a href="https://docs.aws.amazon.com/forecast/latest/dg/data-aggregation.html#specifying-time-boundary">Specifying a Time Boundary</a>.
            If you
            don't provide a time boundary, Forecast uses a set of <a href="https://docs.aws.amazon.com/forecast/latest/dg/data-aggregation.html#default-time-boundaries">Default Time Boundaries</a>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `forecast_horizon` | i64 | <p>The number of time-steps that the model predicts. The forecast horizon is also called
            the prediction length.</p> |
| `forecast_frequency` | String | <p>The frequency of predictions in a forecast.</p>
         <p>Valid intervals are Y (Year), M (Month), W (Week), D (Day), H (Hour), 30min (30
            minutes), 15min (15 minutes), 10min (10 minutes), 5min (5 minutes), and 1min (1 minute).
            For example, "Y" indicates every year and "5min" indicates every five minutes.</p> |
| `estimated_time_remaining_in_minutes` | i64 | <p>The estimated time remaining in minutes for the predictor training job to
            complete.</p> |
| `creation_time` | String | <p>The timestamp of the CreateAutoPredictor request.</p> |
| `predictor_name` | String | <p>The name of the predictor.</p> |
| `encryption_config` | String |  |
| `explainability_info` | String | <p>Provides the status and ARN of the Predictor Explainability.</p> |
| `monitor_info` | String | <p>A  object with the Amazon Resource Name (ARN) and status of the monitor resource.</p> |
| `message` | String | <p>In the event of an error, a message detailing the cause of the error.</p> |
| `reference_predictor_summary` | String | <p>The ARN and state of the reference predictor. This parameter is only valid for
            retrained or upgraded predictors.</p> |
| `status` | String | <p>The status of the predictor. States include: </p>
         <ul>
            <li>
               <p>
                  <code>ACTIVE</code>
               </p>
            </li>
            <li>
               <p>
                  <code>CREATE_PENDING</code>, <code>CREATE_IN_PROGRESS</code>,
                        <code>CREATE_FAILED</code>
               </p>
            </li>
            <li>
               <p>
                  <code>CREATE_STOPPING</code>, <code>CREATE_STOPPED</code>
               </p>
            </li>
            <li>
               <p>
                  <code>DELETE_PENDING</code>, <code>DELETE_IN_PROGRESS</code>,
                        <code>DELETE_FAILED</code>
               </p>
            </li>
         </ul> |
| `predictor_arn` | String | <p>The Amazon Resource Name (ARN) of the predictor</p> |
| `dataset_import_job_arns` | Vec<String> | <p>An array of the ARNs of the dataset import jobs used to import training data for the
            predictor.</p> |
| `forecast_dimensions` | Vec<String> | <p>An array of dimension (field) names that specify the attributes used to group your
            time series.</p> |
| `last_modification_time` | String | <p>The last time the resource was modified. The timestamp depends on the status of the
            job:</p>
         <ul>
            <li>
               <p>
                  <code>CREATE_PENDING</code> - The <code>CreationTime</code>.</p>
            </li>
            <li>
               <p>
                  <code>CREATE_IN_PROGRESS</code> - The current timestamp.</p>
            </li>
            <li>
               <p>
                  <code>CREATE_STOPPING</code> - The current timestamp.</p>
            </li>
            <li>
               <p>
                  <code>CREATE_STOPPED</code> - When the job stopped.</p>
            </li>
            <li>
               <p>
                  <code>ACTIVE</code> or <code>CREATE_FAILED</code> - When the job finished or
                    failed.</p>
            </li>
         </ul> |
| `optimization_metric` | String | <p>The accuracy metric used to optimize the predictor.</p> |
| `time_alignment_boundary` | String | <p>The time boundary Forecast uses when aggregating data.</p> |
| `forecast_types` | Vec<String> | <p>The forecast types used during predictor training. Default value is
            ["0.1","0.5","0.9"].</p> |
| `data_config` | String | <p>The data configuration for your dataset group and any additional datasets.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create auto_predictor
auto_predictor = provider.forecast.Auto_predictor {
    predictor_name = "value"  # <p>A unique name for the predictor</p>
}

# Access auto_predictor outputs
auto_predictor_id = auto_predictor.id
auto_predictor_forecast_horizon = auto_predictor.forecast_horizon
auto_predictor_forecast_frequency = auto_predictor.forecast_frequency
auto_predictor_estimated_time_remaining_in_minutes = auto_predictor.estimated_time_remaining_in_minutes
auto_predictor_creation_time = auto_predictor.creation_time
auto_predictor_predictor_name = auto_predictor.predictor_name
auto_predictor_encryption_config = auto_predictor.encryption_config
auto_predictor_explainability_info = auto_predictor.explainability_info
auto_predictor_monitor_info = auto_predictor.monitor_info
auto_predictor_message = auto_predictor.message
auto_predictor_reference_predictor_summary = auto_predictor.reference_predictor_summary
auto_predictor_status = auto_predictor.status
auto_predictor_predictor_arn = auto_predictor.predictor_arn
auto_predictor_dataset_import_job_arns = auto_predictor.dataset_import_job_arns
auto_predictor_forecast_dimensions = auto_predictor.forecast_dimensions
auto_predictor_last_modification_time = auto_predictor.last_modification_time
auto_predictor_optimization_metric = auto_predictor.optimization_metric
auto_predictor_time_alignment_boundary = auto_predictor.time_alignment_boundary
auto_predictor_forecast_types = auto_predictor.forecast_types
auto_predictor_data_config = auto_predictor.data_config
```

---


### Predictor_backtest_export_job

PredictorBacktestExportJob resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `format` | String |  | <p>The format of the exported data, CSV or PARQUET. The default value is CSV.</p> |
| `predictor_backtest_export_job_name` | String | ✅ | <p>The name for the backtest export job.</p> |
| `destination` | String | ✅ |  |
| `predictor_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the predictor that you want to export.</p> |
| `tags` | Vec<String> |  | <p>Optional metadata to help you categorize and organize your backtests. Each tag
            consists of a key and an optional value, both of which you define. Tag keys and values
            are case sensitive.</p>
         <p>The following restrictions apply to tags:</p>
         <ul>
            <li>
               <p>For each resource, each tag key must be unique and each tag key must have one
                    value.</p>
            </li>
            <li>
               <p>Maximum number of tags per resource: 50.</p>
            </li>
            <li>
               <p>Maximum key length: 128 Unicode characters in UTF-8.</p>
            </li>
            <li>
               <p>Maximum value length: 256 Unicode characters in UTF-8.</p>
            </li>
            <li>
               <p>Accepted characters: all letters and numbers, spaces representable in UTF-8,
                    and + - = . _ : / @. If your tagging schema is used across other services and
                    resources, the character restrictions of those services also apply. </p>
            </li>
            <li>
               <p>Key prefixes cannot include any upper or lowercase combination of
                        <code>aws:</code> or <code>AWS:</code>. Values can have this prefix. If a
                    tag value has <code>aws</code> as its prefix but the key does not, Forecast
                    considers it to be a user tag and will count against the limit of 50 tags. Tags
                    with only the key prefix of <code>aws</code> do not count against your tags per
                    resource limit. You cannot edit or delete tag keys with this prefix.</p>
            </li>
         </ul> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `format` | String | <p>The format of the exported data, CSV or PARQUET.</p> |
| `creation_time` | String | <p>When the predictor backtest export job was created.</p> |
| `predictor_backtest_export_job_arn` | String | <p>The Amazon Resource Name (ARN) of the predictor backtest export job.</p> |
| `predictor_arn` | String | <p>The Amazon Resource Name (ARN) of the predictor.</p> |
| `destination` | String |  |
| `message` | String | <p>Information about any errors that may have occurred during the backtest export.</p> |
| `status` | String | <p>The status of the predictor backtest export job. States include: </p>
         <ul>
            <li>
               <p>
                  <code>ACTIVE</code>
               </p>
            </li>
            <li>
               <p>
                  <code>CREATE_PENDING</code>, <code>CREATE_IN_PROGRESS</code>,
                        <code>CREATE_FAILED</code>
               </p>
            </li>
            <li>
               <p>
                  <code>CREATE_STOPPING</code>, <code>CREATE_STOPPED</code>
               </p>
            </li>
            <li>
               <p>
                  <code>DELETE_PENDING</code>, <code>DELETE_IN_PROGRESS</code>,
                        <code>DELETE_FAILED</code>
               </p>
            </li>
         </ul> |
| `last_modification_time` | String | <p>The last time the resource was modified. The timestamp depends on the status of the
            job:</p>
         <ul>
            <li>
               <p>
                  <code>CREATE_PENDING</code> - The <code>CreationTime</code>.</p>
            </li>
            <li>
               <p>
                  <code>CREATE_IN_PROGRESS</code> - The current timestamp.</p>
            </li>
            <li>
               <p>
                  <code>CREATE_STOPPING</code> - The current timestamp.</p>
            </li>
            <li>
               <p>
                  <code>CREATE_STOPPED</code> - When the job stopped.</p>
            </li>
            <li>
               <p>
                  <code>ACTIVE</code> or <code>CREATE_FAILED</code> - When the job finished or
                    failed.</p>
            </li>
         </ul> |
| `predictor_backtest_export_job_name` | String | <p>The name of the predictor backtest export job.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create predictor_backtest_export_job
predictor_backtest_export_job = provider.forecast.Predictor_backtest_export_job {
    predictor_backtest_export_job_name = "value"  # <p>The name for the backtest export job.</p>
    destination = "value"  # Required field
    predictor_arn = "value"  # <p>The Amazon Resource Name (ARN) of the predictor that you want to export.</p>
}

# Access predictor_backtest_export_job outputs
predictor_backtest_export_job_id = predictor_backtest_export_job.id
predictor_backtest_export_job_format = predictor_backtest_export_job.format
predictor_backtest_export_job_creation_time = predictor_backtest_export_job.creation_time
predictor_backtest_export_job_predictor_backtest_export_job_arn = predictor_backtest_export_job.predictor_backtest_export_job_arn
predictor_backtest_export_job_predictor_arn = predictor_backtest_export_job.predictor_arn
predictor_backtest_export_job_destination = predictor_backtest_export_job.destination
predictor_backtest_export_job_message = predictor_backtest_export_job.message
predictor_backtest_export_job_status = predictor_backtest_export_job.status
predictor_backtest_export_job_last_modification_time = predictor_backtest_export_job.last_modification_time
predictor_backtest_export_job_predictor_backtest_export_job_name = predictor_backtest_export_job.predictor_backtest_export_job_name
```

---


### What_if_forecast_export

WhatIfForecastExport resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `destination` | String | ✅ | <p>The location where you want to save the forecast and an Identity and Access Management (IAM) role that
      Amazon Forecast can assume to access the location. The forecast must be exported to an Amazon S3
      bucket.</p>
         <p>If encryption is used, <code>Destination</code> must include an Key Management Service (KMS) key. The
      IAM role must allow Amazon Forecast permission to access the key.</p> |
| `what_if_forecast_export_name` | String | ✅ | <p>The name of the what-if forecast to export.</p> |
| `tags` | Vec<String> |  | <p>A list of <a href="https://docs.aws.amazon.com/forecast/latest/dg/tagging-forecast-resources.html">tags</a> to apply to the what if forecast.</p> |
| `what_if_forecast_arns` | Vec<i64> | ✅ | <p>The list of what-if forecast Amazon Resource Names (ARNs) to export.</p> |
| `format` | String |  | <p>The format of the exported data, CSV or PARQUET.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `destination` | String |  |
| `last_modification_time` | String | <p>The last time the resource was modified. The timestamp depends on the status of the job:</p>
         <ul>
            <li>
               <p>
                  <code>CREATE_PENDING</code> - The <code>CreationTime</code>.</p>
            </li>
            <li>
               <p>
                  <code>CREATE_IN_PROGRESS</code> - The current timestamp.</p>
            </li>
            <li>
               <p>
                  <code>CREATE_STOPPING</code> - The current timestamp.</p>
            </li>
            <li>
               <p>
                  <code>CREATE_STOPPED</code> - When the job stopped.</p>
            </li>
            <li>
               <p>
                  <code>ACTIVE</code> or <code>CREATE_FAILED</code> - When the job finished or
          failed.</p>
            </li>
         </ul> |
| `format` | String | <p>The format of the exported data, CSV or PARQUET.</p> |
| `what_if_forecast_arns` | i64 | <p>An array of Amazon Resource Names (ARNs) that represent all of the what-if forecasts exported in this
      resource.</p> |
| `message` | String | <p>If an error occurred, an informational message about the error.</p> |
| `creation_time` | String | <p>When the what-if forecast export was created.</p> |
| `what_if_forecast_export_arn` | i64 | <p>The Amazon Resource Name (ARN) of the what-if forecast export.</p> |
| `what_if_forecast_export_name` | String | <p>The name of the what-if forecast export.</p> |
| `status` | String | <p>The status of the what-if forecast. States include:</p>
         <ul>
            <li>
               <p>
                  <code>ACTIVE</code>
               </p>
            </li>
            <li>
               <p>
                  <code>CREATE_PENDING</code>, <code>CREATE_IN_PROGRESS</code>,
          <code>CREATE_FAILED</code>
               </p>
            </li>
            <li>
               <p>
                  <code>CREATE_STOPPING</code>, <code>CREATE_STOPPED</code>
               </p>
            </li>
            <li>
               <p>
                  <code>DELETE_PENDING</code>, <code>DELETE_IN_PROGRESS</code>,
          <code>DELETE_FAILED</code>
               </p>
            </li>
         </ul>
         <note>
            <p>The <code>Status</code> of the what-if forecast export must be <code>ACTIVE</code> before you can access the
        forecast export.</p>
         </note> |
| `estimated_time_remaining_in_minutes` | i64 | <p>The approximate time remaining to complete the what-if forecast export, in minutes.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create what_if_forecast_export
what_if_forecast_export = provider.forecast.What_if_forecast_export {
    destination = "value"  # <p>The location where you want to save the forecast and an Identity and Access Management (IAM) role that
      Amazon Forecast can assume to access the location. The forecast must be exported to an Amazon S3
      bucket.</p>
         <p>If encryption is used, <code>Destination</code> must include an Key Management Service (KMS) key. The
      IAM role must allow Amazon Forecast permission to access the key.</p>
    what_if_forecast_export_name = "value"  # <p>The name of the what-if forecast to export.</p>
    what_if_forecast_arns = "value"  # <p>The list of what-if forecast Amazon Resource Names (ARNs) to export.</p>
}

# Access what_if_forecast_export outputs
what_if_forecast_export_id = what_if_forecast_export.id
what_if_forecast_export_destination = what_if_forecast_export.destination
what_if_forecast_export_last_modification_time = what_if_forecast_export.last_modification_time
what_if_forecast_export_format = what_if_forecast_export.format
what_if_forecast_export_what_if_forecast_arns = what_if_forecast_export.what_if_forecast_arns
what_if_forecast_export_message = what_if_forecast_export.message
what_if_forecast_export_creation_time = what_if_forecast_export.creation_time
what_if_forecast_export_what_if_forecast_export_arn = what_if_forecast_export.what_if_forecast_export_arn
what_if_forecast_export_what_if_forecast_export_name = what_if_forecast_export.what_if_forecast_export_name
what_if_forecast_export_status = what_if_forecast_export.status
what_if_forecast_export_estimated_time_remaining_in_minutes = what_if_forecast_export.estimated_time_remaining_in_minutes
```

---


### Resource_tree

ResourceTree resource

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


### Dataset_import_job

DatasetImportJob resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `data_source` | String | ✅ | <p>The location of the training data to import and an Identity and Access Management (IAM) role that Amazon Forecast
      can assume to access the data. The training data must be stored in an Amazon S3 bucket.</p>
         <p>If encryption is used, <code>DataSource</code> must include an Key Management Service (KMS) key and the
      IAM role must allow Amazon Forecast permission to access the key. The KMS key and IAM role must
      match those specified in the <code>EncryptionConfig</code> parameter of the <a href="https://docs.aws.amazon.com/forecast/latest/dg/API_CreateDataset.html">CreateDataset</a>
      operation.</p> |
| `use_geolocation_for_time_zone` | bool |  | <p>Automatically derive time zone information from the geolocation attribute. This option
            is ideal for datasets that contain timestamps in multiple time zones and those
            timestamps are expressed in local time.</p> |
| `tags` | Vec<String> |  | <p>The optional metadata that you apply to the dataset import job to help you categorize and
      organize them. Each tag consists of a key and an optional value, both of which you
      define.</p>
         <p>The following basic restrictions apply to tags:</p>
         <ul>
            <li>
               <p>Maximum number of tags per resource - 50.</p>
            </li>
            <li>
               <p>For each resource, each tag key must be unique, and each tag key can have only one
          value.</p>
            </li>
            <li>
               <p>Maximum key length - 128 Unicode characters in UTF-8.</p>
            </li>
            <li>
               <p>Maximum value length - 256 Unicode characters in UTF-8.</p>
            </li>
            <li>
               <p>If your tagging schema is used across multiple services and resources, remember that
          other services may have restrictions on allowed characters. Generally allowed characters
          are: letters, numbers, and spaces representable in UTF-8, and the following characters: +
          - = . _ : / @.</p>
            </li>
            <li>
               <p>Tag keys and values are case sensitive.</p>
            </li>
            <li>
               <p>Do not use <code>aws:</code>, <code>AWS:</code>, or any upper or lowercase combination
          of such as a prefix for keys as it is reserved for Amazon Web Services use. You cannot edit or delete tag
          keys with this prefix. Values can have this prefix. If a tag value has <code>aws</code> as
          its prefix but the key does not, then Forecast considers it to be a user tag and will
          count against the limit of 50 tags. Tags with only the key prefix of <code>aws</code> do
          not count against your tags per resource limit.</p>
            </li>
         </ul> |
| `timestamp_format` | String |  | <p>The format of timestamps in the dataset. The format that you specify depends on the
        <code>DataFrequency</code> specified when the dataset was created. The following formats are
      supported</p>
         <ul>
            <li>
               <p>"yyyy-MM-dd"</p>
               <p>For the following data frequencies: Y, M, W, and D</p>
            </li>
            <li>
               <p>"yyyy-MM-dd HH:mm:ss"</p>
               <p>For the following data frequencies: H, 30min, 15min, and 1min; and optionally, for: Y,
          M, W, and D</p>
            </li>
         </ul>
         <p>If the format isn't specified, Amazon Forecast expects the format to be "yyyy-MM-dd
      HH:mm:ss".</p> |
| `time_zone` | String |  | <p>A single time zone for every item in your dataset. This option is ideal for datasets
            with all timestamps within a single time zone, or if all timestamps are normalized to a
            single time zone. </p>
         <p>Refer to the <a href="http://joda-time.sourceforge.net/timezones.html">Joda-Time
                API</a> for a complete list of valid time zone names.</p> |
| `geolocation_format` | String |  | <p>The format of the geolocation attribute. The geolocation attribute can be formatted in
            one of two ways:</p>
         <ul>
            <li>
               <p>
                  <code>LAT_LONG</code> - the latitude and longitude in decimal format (Example: 47.61_-122.33).</p>
            </li>
            <li>
               <p>
                  <code>CC_POSTALCODE</code> (US Only) - the country code (US), followed by the 5-digit ZIP code (Example: US_98121).</p>
            </li>
         </ul> |
| `dataset_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the Amazon Forecast dataset that you want to import data
      to.</p> |
| `import_mode` | String |  | <p>Specifies whether the dataset import job is a <code>FULL</code> or <code>INCREMENTAL</code> import. A <code>FULL</code> dataset import replaces all of the existing data with the newly imported data. An <code>INCREMENTAL</code> import appends the imported data to the existing data.</p> |
| `format` | String |  | <p>The format of the imported data, CSV or PARQUET. The default value is CSV.</p> |
| `dataset_import_job_name` | String | ✅ | <p>The name for the dataset import job. We recommend including the current timestamp in the
      name, for example, <code>20190721DatasetImport</code>. This can help you avoid getting a
        <code>ResourceAlreadyExistsException</code> exception.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `dataset_import_job_name` | String | <p>The name of the dataset import job.</p> |
| `format` | String | <p>The format of the imported data, CSV or PARQUET.</p> |
| `dataset_arn` | String | <p>The Amazon Resource Name (ARN) of the dataset that the training data was imported
      to.</p> |
| `use_geolocation_for_time_zone` | bool | <p>Whether <code>TimeZone</code> is automatically derived from the geolocation
            attribute.</p> |
| `time_zone` | String | <p>The single time zone applied to every item in the dataset</p> |
| `dataset_import_job_arn` | String | <p>The ARN of the dataset import job.</p> |
| `geolocation_format` | String | <p>The format of the geolocation attribute. Valid Values:<code>"LAT_LONG"</code> and
                <code>"CC_POSTALCODE"</code>.</p> |
| `field_statistics` | HashMap<String, String> | <p>Statistical information about each field in the input data.</p> |
| `timestamp_format` | String | <p>The format of timestamps in the dataset. The format that you specify depends on the
        <code>DataFrequency</code> specified when the dataset was created. The following formats are
      supported</p>
         <ul>
            <li>
               <p>"yyyy-MM-dd"</p>
               <p>For the following data frequencies: Y, M, W, and D</p>
            </li>
            <li>
               <p>"yyyy-MM-dd HH:mm:ss"</p>
               <p>For the following data frequencies: H, 30min, 15min, and 1min; and optionally, for: Y,
          M, W, and D</p>
            </li>
         </ul> |
| `data_source` | String | <p>The location of the training data to import and an Identity and Access Management (IAM) role that Amazon Forecast
      can assume to access the data.</p>
         <p>If encryption is used, <code>DataSource</code> includes an Key Management Service (KMS) key.</p> |
| `data_size` | f64 | <p>The size of the dataset in gigabytes (GB) after the import job has finished.</p> |
| `message` | String | <p>If an error occurred, an informational message about the error.</p> |
| `estimated_time_remaining_in_minutes` | i64 | <p>The estimated time remaining in minutes for the dataset import job to complete.</p> |
| `creation_time` | String | <p>When the dataset import job was created.</p> |
| `import_mode` | String | <p>The import mode of the dataset import job, FULL or INCREMENTAL.</p> |
| `status` | String | <p>The status of the dataset import job. States include:</p>
         <ul>
            <li>
               <p>
                  <code>ACTIVE</code>
               </p>
            </li>
            <li>
               <p>
                  <code>CREATE_PENDING</code>, <code>CREATE_IN_PROGRESS</code>,
            <code>CREATE_FAILED</code>
               </p>
            </li>
            <li>
               <p>
                  <code>DELETE_PENDING</code>, <code>DELETE_IN_PROGRESS</code>,
            <code>DELETE_FAILED</code>
               </p>
            </li>
            <li>
               <p>
                  <code>CREATE_STOPPING</code>, <code>CREATE_STOPPED</code>
               </p>
            </li>
         </ul> |
| `last_modification_time` | String | <p>The last time the resource was modified. The timestamp depends on the status of the
      job:</p>
         <ul>
            <li>
               <p>
                  <code>CREATE_PENDING</code> - The <code>CreationTime</code>.</p>
            </li>
            <li>
               <p>
                  <code>CREATE_IN_PROGRESS</code> - The current timestamp.</p>
            </li>
            <li>
               <p>
                  <code>CREATE_STOPPING</code> - The current timestamp.</p>
            </li>
            <li>
               <p>
                  <code>CREATE_STOPPED</code> - When the job stopped.</p>
            </li>
            <li>
               <p>
                  <code>ACTIVE</code> or <code>CREATE_FAILED</code> - When the job finished or
          failed.</p>
            </li>
         </ul> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create dataset_import_job
dataset_import_job = provider.forecast.Dataset_import_job {
    data_source = "value"  # <p>The location of the training data to import and an Identity and Access Management (IAM) role that Amazon Forecast
      can assume to access the data. The training data must be stored in an Amazon S3 bucket.</p>
         <p>If encryption is used, <code>DataSource</code> must include an Key Management Service (KMS) key and the
      IAM role must allow Amazon Forecast permission to access the key. The KMS key and IAM role must
      match those specified in the <code>EncryptionConfig</code> parameter of the <a href="https://docs.aws.amazon.com/forecast/latest/dg/API_CreateDataset.html">CreateDataset</a>
      operation.</p>
    dataset_arn = "value"  # <p>The Amazon Resource Name (ARN) of the Amazon Forecast dataset that you want to import data
      to.</p>
    dataset_import_job_name = "value"  # <p>The name for the dataset import job. We recommend including the current timestamp in the
      name, for example, <code>20190721DatasetImport</code>. This can help you avoid getting a
        <code>ResourceAlreadyExistsException</code> exception.</p>
}

# Access dataset_import_job outputs
dataset_import_job_id = dataset_import_job.id
dataset_import_job_dataset_import_job_name = dataset_import_job.dataset_import_job_name
dataset_import_job_format = dataset_import_job.format
dataset_import_job_dataset_arn = dataset_import_job.dataset_arn
dataset_import_job_use_geolocation_for_time_zone = dataset_import_job.use_geolocation_for_time_zone
dataset_import_job_time_zone = dataset_import_job.time_zone
dataset_import_job_dataset_import_job_arn = dataset_import_job.dataset_import_job_arn
dataset_import_job_geolocation_format = dataset_import_job.geolocation_format
dataset_import_job_field_statistics = dataset_import_job.field_statistics
dataset_import_job_timestamp_format = dataset_import_job.timestamp_format
dataset_import_job_data_source = dataset_import_job.data_source
dataset_import_job_data_size = dataset_import_job.data_size
dataset_import_job_message = dataset_import_job.message
dataset_import_job_estimated_time_remaining_in_minutes = dataset_import_job.estimated_time_remaining_in_minutes
dataset_import_job_creation_time = dataset_import_job.creation_time
dataset_import_job_import_mode = dataset_import_job.import_mode
dataset_import_job_status = dataset_import_job.status
dataset_import_job_last_modification_time = dataset_import_job.last_modification_time
```

---


### Dataset_group

DatasetGroup resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `dataset_group_name` | String | ✅ | <p>A name for the dataset group.</p> |
| `domain` | String | ✅ | <p>The domain associated with the dataset group. When you add a dataset to a dataset group,
      this value and the value specified for the <code>Domain</code> parameter of the <a href="https://docs.aws.amazon.com/forecast/latest/dg/API_CreateDataset.html">CreateDataset</a>
      operation must match.</p>
         <p>The <code>Domain</code> and <code>DatasetType</code> that you choose determine the fields
      that must be present in training data that you import to a dataset. For example, if you choose
      the <code>RETAIL</code> domain and <code>TARGET_TIME_SERIES</code> as the
        <code>DatasetType</code>, Amazon Forecast requires that <code>item_id</code>,
        <code>timestamp</code>, and <code>demand</code> fields are present in your data. For more
      information, see <a href="https://docs.aws.amazon.com/forecast/latest/dg/howitworks-datasets-groups.html">Dataset groups</a>.</p> |
| `dataset_arns` | Vec<String> |  | <p>An array of Amazon Resource Names (ARNs) of the datasets that you want to include in the
      dataset group.</p> |
| `tags` | Vec<String> |  | <p>The optional metadata that you apply to the dataset group to help you categorize and
      organize them. Each tag consists of a key and an optional value, both of which you
      define.</p>
         <p>The following basic restrictions apply to tags:</p>
         <ul>
            <li>
               <p>Maximum number of tags per resource - 50.</p>
            </li>
            <li>
               <p>For each resource, each tag key must be unique, and each tag key can have only one
          value.</p>
            </li>
            <li>
               <p>Maximum key length - 128 Unicode characters in UTF-8.</p>
            </li>
            <li>
               <p>Maximum value length - 256 Unicode characters in UTF-8.</p>
            </li>
            <li>
               <p>If your tagging schema is used across multiple services and resources, remember that
          other services may have restrictions on allowed characters. Generally allowed characters
          are: letters, numbers, and spaces representable in UTF-8, and the following characters: +
          - = . _ : / @.</p>
            </li>
            <li>
               <p>Tag keys and values are case sensitive.</p>
            </li>
            <li>
               <p>Do not use <code>aws:</code>, <code>AWS:</code>, or any upper or lowercase combination
          of such as a prefix for keys as it is reserved for Amazon Web Services use. You cannot edit or delete tag
          keys with this prefix. Values can have this prefix. If a tag value has <code>aws</code> as
          its prefix but the key does not, then Forecast considers it to be a user tag and will
          count against the limit of 50 tags. Tags with only the key prefix of <code>aws</code> do
          not count against your tags per resource limit.</p>
            </li>
         </ul> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `creation_time` | String | <p>When the dataset group was created.</p> |
| `last_modification_time` | String | <p>When the dataset group was created or last updated from a call to the <a href="https://docs.aws.amazon.com/forecast/latest/dg/API_UpdateDatasetGroup.html">UpdateDatasetGroup</a> operation. While the dataset group is being updated,
        <code>LastModificationTime</code> is the current time of the
        <code>DescribeDatasetGroup</code> call.</p> |
| `dataset_arns` | Vec<String> | <p>An array of Amazon Resource Names (ARNs) of the datasets contained in the dataset
      group.</p> |
| `dataset_group_name` | String | <p>The name of the dataset group.</p> |
| `domain` | String | <p>The domain associated with the dataset group.</p> |
| `status` | String | <p>The status of the dataset group. States include:</p>
         <ul>
            <li>
               <p>
                  <code>ACTIVE</code>
               </p>
            </li>
            <li>
               <p>
                  <code>CREATE_PENDING</code>, <code>CREATE_IN_PROGRESS</code>,
            <code>CREATE_FAILED</code>
               </p>
            </li>
            <li>
               <p>
                  <code>DELETE_PENDING</code>, <code>DELETE_IN_PROGRESS</code>,
            <code>DELETE_FAILED</code>
               </p>
            </li>
            <li>
               <p>
                  <code>UPDATE_PENDING</code>, <code>UPDATE_IN_PROGRESS</code>,
            <code>UPDATE_FAILED</code>
               </p>
            </li>
         </ul>
         <p>The <code>UPDATE</code> states apply when you call the <a href="https://docs.aws.amazon.com/forecast/latest/dg/API_UpdateDatasetGroup.html">UpdateDatasetGroup</a>
      operation.</p>
         <note>
            <p>The <code>Status</code> of the dataset group must be <code>ACTIVE</code> before you can
        use the dataset group to create a predictor.</p>
         </note> |
| `dataset_group_arn` | String | <p>The ARN of the dataset group.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create dataset_group
dataset_group = provider.forecast.Dataset_group {
    dataset_group_name = "value"  # <p>A name for the dataset group.</p>
    domain = "value"  # <p>The domain associated with the dataset group. When you add a dataset to a dataset group,
      this value and the value specified for the <code>Domain</code> parameter of the <a href="https://docs.aws.amazon.com/forecast/latest/dg/API_CreateDataset.html">CreateDataset</a>
      operation must match.</p>
         <p>The <code>Domain</code> and <code>DatasetType</code> that you choose determine the fields
      that must be present in training data that you import to a dataset. For example, if you choose
      the <code>RETAIL</code> domain and <code>TARGET_TIME_SERIES</code> as the
        <code>DatasetType</code>, Amazon Forecast requires that <code>item_id</code>,
        <code>timestamp</code>, and <code>demand</code> fields are present in your data. For more
      information, see <a href="https://docs.aws.amazon.com/forecast/latest/dg/howitworks-datasets-groups.html">Dataset groups</a>.</p>
}

# Access dataset_group outputs
dataset_group_id = dataset_group.id
dataset_group_creation_time = dataset_group.creation_time
dataset_group_last_modification_time = dataset_group.last_modification_time
dataset_group_dataset_arns = dataset_group.dataset_arns
dataset_group_dataset_group_name = dataset_group.dataset_group_name
dataset_group_domain = dataset_group.domain
dataset_group_status = dataset_group.status
dataset_group_dataset_group_arn = dataset_group.dataset_group_arn
```

---


### Accuracy_metrics

AccuracyMetrics resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `predictor_evaluation_results` | Vec<String> | <p>An array of results from evaluating the predictor.</p> |
| `is_auto_predictor` | bool | <p>Whether the predictor was created with <a>CreateAutoPredictor</a>.</p> |
| `optimization_metric` | String | <p>The accuracy metric used to optimize the predictor.</p> |
| `auto_ml_override_strategy` | String | <note>
            <p> The <code>LatencyOptimized</code> AutoML override strategy is only available in private beta.
                Contact Amazon Web Services Support or your account manager to learn more about access privileges.
            </p>
         </note>
         <p>The AutoML strategy used to train the predictor. Unless <code>LatencyOptimized</code>
            is specified, the AutoML strategy optimizes predictor accuracy.</p>
         <p>This parameter is only valid for predictors trained using AutoML.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access accuracy_metrics outputs
accuracy_metrics_id = accuracy_metrics.id
accuracy_metrics_predictor_evaluation_results = accuracy_metrics.predictor_evaluation_results
accuracy_metrics_is_auto_predictor = accuracy_metrics.is_auto_predictor
accuracy_metrics_optimization_metric = accuracy_metrics.optimization_metric
accuracy_metrics_auto_ml_override_strategy = accuracy_metrics.auto_ml_override_strategy
```

---


### Forecast_export_job

ForecastExportJob resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `forecast_export_job_name` | String | ✅ | <p>The name for the forecast export job.</p> |
| `tags` | Vec<String> |  | <p>The optional metadata that you apply to the forecast export job to help you categorize and
      organize them. Each tag consists of a key and an optional value, both of which you
      define.</p>
         <p>The following basic restrictions apply to tags:</p>
         <ul>
            <li>
               <p>Maximum number of tags per resource - 50.</p>
            </li>
            <li>
               <p>For each resource, each tag key must be unique, and each tag key can have only one
          value.</p>
            </li>
            <li>
               <p>Maximum key length - 128 Unicode characters in UTF-8.</p>
            </li>
            <li>
               <p>Maximum value length - 256 Unicode characters in UTF-8.</p>
            </li>
            <li>
               <p>If your tagging schema is used across multiple services and resources, remember that
          other services may have restrictions on allowed characters. Generally allowed characters
          are: letters, numbers, and spaces representable in UTF-8, and the following characters: +
          - = . _ : / @.</p>
            </li>
            <li>
               <p>Tag keys and values are case sensitive.</p>
            </li>
            <li>
               <p>Do not use <code>aws:</code>, <code>AWS:</code>, or any upper or lowercase combination
          of such as a prefix for keys as it is reserved for Amazon Web Services use. You cannot edit or delete tag
          keys with this prefix. Values can have this prefix. If a tag value has <code>aws</code> as
          its prefix but the key does not, then Forecast considers it to be a user tag and will
          count against the limit of 50 tags. Tags with only the key prefix of <code>aws</code> do
          not count against your tags per resource limit.</p>
            </li>
         </ul> |
| `forecast_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the forecast that you want to export.</p> |
| `format` | String |  | <p>The format of the exported data, CSV or PARQUET. The default value is CSV.</p> |
| `destination` | String | ✅ | <p>The location where you want to save the forecast and an Identity and Access Management (IAM) role that
      Amazon Forecast can assume to access the location. The forecast must be exported to an Amazon S3
      bucket.</p>
         <p>If encryption is used, <code>Destination</code> must include an Key Management Service (KMS) key. The
      IAM role must allow Amazon Forecast permission to access the key.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `forecast_arn` | String | <p>The Amazon Resource Name (ARN) of the exported forecast.</p> |
| `forecast_export_job_arn` | String | <p>The ARN of the forecast export job.</p> |
| `forecast_export_job_name` | String | <p>The name of the forecast export job.</p> |
| `last_modification_time` | String | <p>The last time the resource was modified. The timestamp depends on the status of the job:</p>
         <ul>
            <li>
               <p>
                  <code>CREATE_PENDING</code> - The <code>CreationTime</code>.</p>
            </li>
            <li>
               <p>
                  <code>CREATE_IN_PROGRESS</code> - The current timestamp.</p>
            </li>
            <li>
               <p>
                  <code>CREATE_STOPPING</code> - The current timestamp.</p>
            </li>
            <li>
               <p>
                  <code>CREATE_STOPPED</code> - When the job stopped.</p>
            </li>
            <li>
               <p>
                  <code>ACTIVE</code> or <code>CREATE_FAILED</code> - When the job finished or
          failed.</p>
            </li>
         </ul> |
| `format` | String | <p>The format of the exported data, CSV or PARQUET.</p> |
| `message` | String | <p>If an error occurred, an informational message about the error.</p> |
| `status` | String | <p>The status of the forecast export job. States include:</p>
         <ul>
            <li>
               <p>
                  <code>ACTIVE</code>
               </p>
            </li>
            <li>
               <p>
                  <code>CREATE_PENDING</code>, <code>CREATE_IN_PROGRESS</code>,
          <code>CREATE_FAILED</code>
               </p>
            </li>
            <li>
               <p>
                  <code>CREATE_STOPPING</code>, <code>CREATE_STOPPED</code>
               </p>
            </li>
            <li>
               <p>
                  <code>DELETE_PENDING</code>, <code>DELETE_IN_PROGRESS</code>,
          <code>DELETE_FAILED</code>
               </p>
            </li>
         </ul>
         <note>
            <p>The <code>Status</code> of the forecast export job must be <code>ACTIVE</code> before
        you can access the forecast in your S3 bucket.</p>
         </note> |
| `destination` | String | <p>The path to the Amazon Simple Storage Service (Amazon S3) bucket where the forecast is exported.</p> |
| `creation_time` | String | <p>When the forecast export job was created.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create forecast_export_job
forecast_export_job = provider.forecast.Forecast_export_job {
    forecast_export_job_name = "value"  # <p>The name for the forecast export job.</p>
    forecast_arn = "value"  # <p>The Amazon Resource Name (ARN) of the forecast that you want to export.</p>
    destination = "value"  # <p>The location where you want to save the forecast and an Identity and Access Management (IAM) role that
      Amazon Forecast can assume to access the location. The forecast must be exported to an Amazon S3
      bucket.</p>
         <p>If encryption is used, <code>Destination</code> must include an Key Management Service (KMS) key. The
      IAM role must allow Amazon Forecast permission to access the key.</p>
}

# Access forecast_export_job outputs
forecast_export_job_id = forecast_export_job.id
forecast_export_job_forecast_arn = forecast_export_job.forecast_arn
forecast_export_job_forecast_export_job_arn = forecast_export_job.forecast_export_job_arn
forecast_export_job_forecast_export_job_name = forecast_export_job.forecast_export_job_name
forecast_export_job_last_modification_time = forecast_export_job.last_modification_time
forecast_export_job_format = forecast_export_job.format
forecast_export_job_message = forecast_export_job.message
forecast_export_job_status = forecast_export_job.status
forecast_export_job_destination = forecast_export_job.destination
forecast_export_job_creation_time = forecast_export_job.creation_time
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple dataset resources
dataset_0 = provider.forecast.Dataset {
    dataset_name = "value-0"
    domain = "value-0"
    dataset_type = "value-0"
    schema = "value-0"
}
dataset_1 = provider.forecast.Dataset {
    dataset_name = "value-1"
    domain = "value-1"
    dataset_type = "value-1"
    schema = "value-1"
}
dataset_2 = provider.forecast.Dataset {
    dataset_name = "value-2"
    domain = "value-2"
    dataset_type = "value-2"
    schema = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    dataset = provider.forecast.Dataset {
        dataset_name = "production-value"
        domain = "production-value"
        dataset_type = "production-value"
        schema = "production-value"
    }
```

---

## Related Documentation

- [AWS Forecast Documentation](https://docs.aws.amazon.com/forecast/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
