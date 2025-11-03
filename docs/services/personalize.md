# Personalize Service



**Resources**: 19

---

## Overview

The personalize service provides access to 19 resource types:

- [Solution_metrics](#solution_metrics) [R]
- [Schema](#schema) [CRD]
- [Solution_version](#solution_version) [CR]
- [Dataset](#dataset) [CRUD]
- [Data_deletion_job](#data_deletion_job) [CR]
- [Algorithm](#algorithm) [R]
- [Batch_inference_job](#batch_inference_job) [CR]
- [Recommender](#recommender) [CRUD]
- [Feature_transformation](#feature_transformation) [R]
- [Dataset_export_job](#dataset_export_job) [CR]
- [Dataset_group](#dataset_group) [CRD]
- [Metric_attribution](#metric_attribution) [CRUD]
- [Solution](#solution) [CRUD]
- [Dataset_import_job](#dataset_import_job) [CR]
- [Campaign](#campaign) [CRUD]
- [Filter](#filter) [CRD]
- [Recipe](#recipe) [R]
- [Batch_segment_job](#batch_segment_job) [CR]
- [Event_tracker](#event_tracker) [CRD]

---

## Resources


### Solution_metrics

SolutionMetrics resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `solution_version_arn` | String | <p>The same solution version ARN as specified in the request.</p> |
| `metrics` | HashMap<String, f64> | <p>The metrics for the solution version. For more information, see
      <a href="https://docs.aws.amazon.com/personalize/latest/dg/working-with-training-metrics.html">
        Evaluating a solution version with metrics
      </a>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access solution_metrics outputs
solution_metrics_id = solution_metrics.id
solution_metrics_solution_version_arn = solution_metrics.solution_version_arn
solution_metrics_metrics = solution_metrics.metrics
```

---


### Schema

Schema resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `domain` | String |  | <p>The domain for the schema. If you are creating a schema for a dataset in a Domain dataset group, specify
    the domain you chose when you created the Domain dataset group.</p> |
| `schema` | String | ✅ | <p>A schema in Avro JSON format.</p> |
| `name` | String | ✅ | <p>The name for the schema.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `schema` | String | <p>The requested schema.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create schema
schema = provider.personalize.Schema {
    schema = "value"  # <p>A schema in Avro JSON format.</p>
    name = "value"  # <p>The name for the schema.</p>
}

# Access schema outputs
schema_id = schema.id
schema_schema = schema.schema
```

---


### Solution_version

SolutionVersion resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `solution_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the solution containing the training configuration
      information.</p> |
| `name` | String |  | <p>The name of the solution version.</p> |
| `tags` | Vec<String> |  | <p>A list of <a href="https://docs.aws.amazon.com/personalize/latest/dg/tagging-resources.html">tags</a> to apply to the solution version.</p> |
| `training_mode` | String |  | <p>The scope of training to be performed when creating the solution version. 
      The default is <code>FULL</code>. This creates a completely new model based on the entirety 
      of the training data from the datasets in your dataset group.
    </p>
         <p>If you use
      <a href="https://docs.aws.amazon.com/personalize/latest/dg/native-recipe-new-item-USER_PERSONALIZATION.html">User-Personalization</a>,
      you can specify a training mode of <code>UPDATE</code>. This updates the model to consider new items for recommendations. It is not a full
        retraining. You should still complete a full retraining weekly.
        If you specify <code>UPDATE</code>, Amazon Personalize will stop automatic updates for the solution version. To resume updates, create a new solution with training mode set to <code>FULL</code>
        and deploy it in a campaign. 
        For more information about automatic updates, see 
        <a href="https://docs.aws.amazon.com/personalize/latest/dg/use-case-recipe-features.html#maintaining-with-automatic-updates">Automatic updates</a>.
      </p>
         <p>The <code>UPDATE</code> option can only be used when you already have an active solution
        version created from the input solution using the <code>FULL</code> option and the input
        solution was trained with the 
        <a href="https://docs.aws.amazon.com/personalize/latest/dg/native-recipe-new-item-USER_PERSONALIZATION.html">User-Personalization</a>
        recipe or the legacy
        <a href="https://docs.aws.amazon.com/personalize/latest/dg/native-recipe-hrnn-coldstart.html">HRNN-Coldstart</a> recipe.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `solution_version` | String | <p>The solution version.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create solution_version
solution_version = provider.personalize.Solution_version {
    solution_arn = "value"  # <p>The Amazon Resource Name (ARN) of the solution containing the training configuration
      information.</p>
}

# Access solution_version outputs
solution_version_id = solution_version.id
solution_version_solution_version = solution_version.solution_version
```

---


### Dataset

Dataset resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `schema_arn` | String | ✅ | <p>The ARN of the schema to associate with the dataset. The schema
      defines the dataset fields.</p> |
| `dataset_type` | String | ✅ | <p>The type of dataset.</p>
         <p>One of the following (case insensitive) values:</p>
         <ul>
            <li>
               <p>Interactions</p>
            </li>
            <li>
               <p>Items</p>
            </li>
            <li>
               <p>Users</p>
            </li>
            <li>
               <p>Actions</p>
            </li>
            <li>
               <p>Action_Interactions</p>
            </li>
         </ul> |
| `dataset_group_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the dataset group to add the dataset
      to.</p> |
| `name` | String | ✅ | <p>The name for the dataset.</p> |
| `tags` | Vec<String> |  | <p>A list of <a href="https://docs.aws.amazon.com/personalize/latest/dg/tagging-resources.html">tags</a> to apply to the dataset.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `dataset` | String | <p>A listing of the dataset's properties.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create dataset
dataset = provider.personalize.Dataset {
    schema_arn = "value"  # <p>The ARN of the schema to associate with the dataset. The schema
      defines the dataset fields.</p>
    dataset_type = "value"  # <p>The type of dataset.</p>
         <p>One of the following (case insensitive) values:</p>
         <ul>
            <li>
               <p>Interactions</p>
            </li>
            <li>
               <p>Items</p>
            </li>
            <li>
               <p>Users</p>
            </li>
            <li>
               <p>Actions</p>
            </li>
            <li>
               <p>Action_Interactions</p>
            </li>
         </ul>
    dataset_group_arn = "value"  # <p>The Amazon Resource Name (ARN) of the dataset group to add the dataset
      to.</p>
    name = "value"  # <p>The name for the dataset.</p>
}

# Access dataset outputs
dataset_id = dataset.id
dataset_dataset = dataset.dataset
```

---


### Data_deletion_job

DataDeletionJob resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `role_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the IAM role that has permissions to read from the Amazon S3
      data source.</p> |
| `data_source` | String | ✅ | <p>The Amazon S3 bucket that contains the list of userIds of the users to delete.</p> |
| `dataset_group_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the dataset group that has the datasets you want to
    delete records from.</p> |
| `tags` | Vec<String> |  | <p>A list of <a href="https://docs.aws.amazon.com/personalize/latest/dg/tagging-resources.html">tags</a> to apply to the data deletion job.</p> |
| `job_name` | String | ✅ | <p>The name for the data deletion job.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `data_deletion_job` | String | <p>Information about the data deletion job, including the status.</p>
         <p>The status is one of the following values:</p>
         <ul>
            <li>
               <p>PENDING</p>
            </li>
            <li>
               <p>IN_PROGRESS</p>
            </li>
            <li>
               <p>COMPLETED</p>
            </li>
            <li>
               <p>FAILED</p>
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

# Create data_deletion_job
data_deletion_job = provider.personalize.Data_deletion_job {
    role_arn = "value"  # <p>The Amazon Resource Name (ARN) of the IAM role that has permissions to read from the Amazon S3
      data source.</p>
    data_source = "value"  # <p>The Amazon S3 bucket that contains the list of userIds of the users to delete.</p>
    dataset_group_arn = "value"  # <p>The Amazon Resource Name (ARN) of the dataset group that has the datasets you want to
    delete records from.</p>
    job_name = "value"  # <p>The name for the data deletion job.</p>
}

# Access data_deletion_job outputs
data_deletion_job_id = data_deletion_job.id
data_deletion_job_data_deletion_job = data_deletion_job.data_deletion_job
```

---


### Algorithm

Algorithm resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `algorithm` | String | <p>A listing of the properties of the algorithm.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access algorithm outputs
algorithm_id = algorithm.id
algorithm_algorithm = algorithm.algorithm
```

---


### Batch_inference_job

BatchInferenceJob resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `batch_inference_job_config` | String |  | <p>The configuration details of a batch inference job.</p> |
| `tags` | Vec<String> |  | <p>A list of <a href="https://docs.aws.amazon.com/personalize/latest/dg/tagging-resources.html">tags</a> to apply to the batch inference job.</p> |
| `batch_inference_job_mode` | String |  | <p>The mode of the batch inference job. To generate descriptive themes for groups of similar items, set the
      job mode to <code>THEME_GENERATION</code>. If you don't want to generate themes, use the default <code>BATCH_INFERENCE</code>.</p>
         <p>
      When you get batch recommendations with themes, you will incur additional costs. For more information, see <a href="https://aws.amazon.com/personalize/pricing/">Amazon Personalize pricing</a>.
    </p> |
| `job_name` | String | ✅ | <p>The name of the batch inference job to create.</p> |
| `role_arn` | String | ✅ | <p>The ARN of the Amazon Identity and Access Management role that has permissions to read and write to your input and output
      Amazon S3 buckets respectively.</p> |
| `filter_arn` | String |  | <p>The ARN of the filter to apply to the batch inference job. For more information on using
      filters, see
      <a href="https://docs.aws.amazon.com/personalize/latest/dg/filter-batch.html">Filtering batch recommendations</a>.</p> |
| `job_input` | String | ✅ | <p>The Amazon S3 path that leads to the input file to base your recommendations on. The input
      material must be in JSON format.</p> |
| `job_output` | String | ✅ | <p>The path to the Amazon S3 bucket where the job's output will be stored.</p> |
| `solution_version_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the solution version that will be used to generate the
      batch inference recommendations.</p> |
| `theme_generation_config` | String |  | <p>For theme generation jobs, specify the name of the column in your Items
      dataset that contains each item's name.</p> |
| `num_results` | i64 |  | <p>The number of recommendations to retrieve.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `batch_inference_job` | String | <p>Information on the specified batch inference job.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create batch_inference_job
batch_inference_job = provider.personalize.Batch_inference_job {
    job_name = "value"  # <p>The name of the batch inference job to create.</p>
    role_arn = "value"  # <p>The ARN of the Amazon Identity and Access Management role that has permissions to read and write to your input and output
      Amazon S3 buckets respectively.</p>
    job_input = "value"  # <p>The Amazon S3 path that leads to the input file to base your recommendations on. The input
      material must be in JSON format.</p>
    job_output = "value"  # <p>The path to the Amazon S3 bucket where the job's output will be stored.</p>
    solution_version_arn = "value"  # <p>The Amazon Resource Name (ARN) of the solution version that will be used to generate the
      batch inference recommendations.</p>
}

# Access batch_inference_job outputs
batch_inference_job_id = batch_inference_job.id
batch_inference_job_batch_inference_job = batch_inference_job.batch_inference_job
```

---


### Recommender

Recommender resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>The name of the recommender.</p> |
| `recommender_config` | String |  | <p>The configuration details of the recommender.</p> |
| `tags` | Vec<String> |  | <p>A list of <a href="https://docs.aws.amazon.com/personalize/latest/dg/tagging-resources.html">tags</a> to apply to the recommender.</p> |
| `dataset_group_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the destination domain dataset group for the recommender.</p> |
| `recipe_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the recipe that the recommender will use. For a recommender, a recipe is a Domain dataset group
      use case. Only Domain dataset group use cases can be used to create a recommender. For information about use cases see <a href="https://docs.aws.amazon.com/personalize/latest/dg/domain-use-cases.html">Choosing recommender use cases</a>.
    </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `recommender` | String | <p>The properties of the recommender.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create recommender
recommender = provider.personalize.Recommender {
    name = "value"  # <p>The name of the recommender.</p>
    dataset_group_arn = "value"  # <p>The Amazon Resource Name (ARN) of the destination domain dataset group for the recommender.</p>
    recipe_arn = "value"  # <p>The Amazon Resource Name (ARN) of the recipe that the recommender will use. For a recommender, a recipe is a Domain dataset group
      use case. Only Domain dataset group use cases can be used to create a recommender. For information about use cases see <a href="https://docs.aws.amazon.com/personalize/latest/dg/domain-use-cases.html">Choosing recommender use cases</a>.
    </p>
}

# Access recommender outputs
recommender_id = recommender.id
recommender_recommender = recommender.recommender
```

---


### Feature_transformation

FeatureTransformation resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `feature_transformation` | String | <p>A listing of the FeatureTransformation properties.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access feature_transformation outputs
feature_transformation_id = feature_transformation.id
feature_transformation_feature_transformation = feature_transformation.feature_transformation
```

---


### Dataset_export_job

DatasetExportJob resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `ingestion_mode` | String |  | <p>The data to export, based on how you imported the data. You can choose
      to export only <code>BULK</code> data that you imported using a dataset
      import job, only <code>PUT</code> data that you imported incrementally
      (using the console, PutEvents, PutUsers and PutItems operations), or
        <code>ALL</code> for both types. The default value is <code>PUT</code>.
    </p> |
| `job_output` | String | ✅ | <p>The path to the Amazon S3 bucket where the job's output is stored.</p> |
| `role_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the IAM service role that has
      permissions to add data to your output Amazon S3 bucket.</p> |
| `tags` | Vec<String> |  | <p>A list of <a href="https://docs.aws.amazon.com/personalize/latest/dg/tagging-resources.html">tags</a> to apply to the dataset export job.</p> |
| `job_name` | String | ✅ | <p>The name for the dataset export job.</p> |
| `dataset_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the dataset that contains the data
      to export.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `dataset_export_job` | String | <p>Information about the dataset export job, including the status.</p>
         <p>The status is one of the following values:</p>
         <ul>
            <li>
               <p>CREATE PENDING</p>
            </li>
            <li>
               <p>CREATE IN_PROGRESS</p>
            </li>
            <li>
               <p>ACTIVE</p>
            </li>
            <li>
               <p>CREATE FAILED</p>
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

# Create dataset_export_job
dataset_export_job = provider.personalize.Dataset_export_job {
    job_output = "value"  # <p>The path to the Amazon S3 bucket where the job's output is stored.</p>
    role_arn = "value"  # <p>The Amazon Resource Name (ARN) of the IAM service role that has
      permissions to add data to your output Amazon S3 bucket.</p>
    job_name = "value"  # <p>The name for the dataset export job.</p>
    dataset_arn = "value"  # <p>The Amazon Resource Name (ARN) of the dataset that contains the data
      to export.</p>
}

# Access dataset_export_job outputs
dataset_export_job_id = dataset_export_job.id
dataset_export_job_dataset_export_job = dataset_export_job.dataset_export_job
```

---


### Dataset_group

DatasetGroup resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>A list of <a href="https://docs.aws.amazon.com/personalize/latest/dg/tagging-resources.html">tags</a> to apply to the dataset group.</p> |
| `role_arn` | String |  | <p>The ARN of the Identity and Access Management (IAM) role that has permissions to access
      the Key Management Service (KMS) key. Supplying an IAM role is only valid when also
      specifying a KMS key.</p> |
| `kms_key_arn` | String |  | <p>The Amazon Resource Name (ARN) of a Key Management Service (KMS) key used to
      encrypt the datasets.</p> |
| `domain` | String |  | <p>The domain of the dataset group. Specify a domain to create a
      Domain dataset group. The domain you specify determines the default
      schemas for datasets and the use cases available for recommenders. If you
      don't specify a domain, you create a Custom dataset group with solution
      versions that you deploy with a campaign. </p> |
| `name` | String | ✅ | <p>The name for the new dataset group.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `dataset_group` | String | <p>A listing of the dataset group's properties.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create dataset_group
dataset_group = provider.personalize.Dataset_group {
    name = "value"  # <p>The name for the new dataset group.</p>
}

# Access dataset_group outputs
dataset_group_id = dataset_group.id
dataset_group_dataset_group = dataset_group.dataset_group
```

---


### Metric_attribution

MetricAttribution resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `dataset_group_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the destination dataset group for the metric attribution.</p> |
| `metrics` | Vec<String> | ✅ | <p>A list of metric attributes for the metric attribution. Each metric attribute specifies an event type to track and a function.
      Available functions are <code>SUM()</code> or <code>SAMPLECOUNT()</code>. For SUM() functions, provide the 
      dataset type (either Interactions or Items) and column to sum as a parameter. For example SUM(Items.PRICE).</p> |
| `metrics_output_config` | String | ✅ | <p>The output configuration details for the metric attribution.</p> |
| `name` | String | ✅ | <p>A name for the metric attribution.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `metric_attribution` | String | <p>The details of the metric attribution.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create metric_attribution
metric_attribution = provider.personalize.Metric_attribution {
    dataset_group_arn = "value"  # <p>The Amazon Resource Name (ARN) of the destination dataset group for the metric attribution.</p>
    metrics = "value"  # <p>A list of metric attributes for the metric attribution. Each metric attribute specifies an event type to track and a function.
      Available functions are <code>SUM()</code> or <code>SAMPLECOUNT()</code>. For SUM() functions, provide the 
      dataset type (either Interactions or Items) and column to sum as a parameter. For example SUM(Items.PRICE).</p>
    metrics_output_config = "value"  # <p>The output configuration details for the metric attribution.</p>
    name = "value"  # <p>A name for the metric attribution.</p>
}

# Access metric_attribution outputs
metric_attribution_id = metric_attribution.id
metric_attribution_metric_attribution = metric_attribution.metric_attribution
```

---


### Solution

Solution resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>A list of <a href="https://docs.aws.amazon.com/personalize/latest/dg/tagging-resources.html">tags</a> to apply to the solution.</p> |
| `name` | String | ✅ | <p>The name for the solution.</p> |
| `perform_auto_ml` | bool |  | <important>
            <p>We don't recommend enabling automated machine learning. Instead, match your use case to the available Amazon Personalize 
        recipes. For more information, see <a href="https://docs.aws.amazon.com/personalize/latest/dg/working-with-predefined-recipes.html">Choosing a recipe</a>.</p>
         </important>
         <p>Whether to perform automated machine learning (AutoML). The default is <code>false</code>.
      For this case, you must specify <code>recipeArn</code>.</p>
         <p>When set to <code>true</code>, Amazon Personalize analyzes your training data and selects
      the optimal USER_PERSONALIZATION recipe and hyperparameters. In this case, you must omit
      <code>recipeArn</code>. Amazon Personalize determines the optimal recipe by running tests with
      different values for the hyperparameters.
      AutoML lengthens the training process as compared to selecting a specific recipe.</p> |
| `solution_config` | String |  | <p>The configuration properties for the solution. When <code>performAutoML</code> is set to
      true, Amazon Personalize only evaluates the <code>autoMLConfig</code> section
      of the solution configuration.</p>
         <note>
            <p>Amazon Personalize doesn't support configuring the <code>hpoObjective</code> 
        at this time.</p>
         </note> |
| `perform_hpo` | bool |  | <p>Whether to perform hyperparameter optimization (HPO) on the specified or selected recipe.
      The default is <code>false</code>.</p>
         <p>When performing AutoML, this parameter is always <code>true</code> and you
      should not set it to <code>false</code>.</p> |
| `perform_auto_training` | bool |  | <p>Whether the solution uses automatic training to create new solution versions (trained models). The default is
        <code>True</code> and the solution automatically creates new solution versions every 7 days. You can change the training
      frequency by specifying a <code>schedulingExpression</code> in the <code>AutoTrainingConfig</code> as part of solution
      configuration. For more information about automatic training,
      see <a href="https://docs.aws.amazon.com/personalize/latest/dg/solution-config-auto-training.html">Configuring automatic training</a>.</p>
         <p>
      Automatic solution version creation starts within one hour after the solution is ACTIVE. If you manually create a solution version within
      the hour, the solution skips the first automatic training.
    </p>
         <p>
      After training starts, you can
      get the solution version's Amazon Resource Name (ARN) with the <a href="https://docs.aws.amazon.com/personalize/latest/dg/API_ListSolutionVersions.html">ListSolutionVersions</a> API operation. 
      To get its status, use the <a href="https://docs.aws.amazon.com/personalize/latest/dg/API_DescribeSolutionVersion.html">DescribeSolutionVersion</a>.
    </p> |
| `dataset_group_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the dataset group that provides the training data.</p> |
| `recipe_arn` | String |  | <p>The Amazon Resource Name (ARN) of the recipe to use for model training. This is required when
      <code>performAutoML</code> is false. For information about different Amazon Personalize recipes and their ARNs, 
      see <a href="https://docs.aws.amazon.com/personalize/latest/dg/working-with-predefined-recipes.html">Choosing a recipe</a>.
    </p> |
| `event_type` | String |  | <p>When your have multiple event types (using an <code>EVENT_TYPE</code> schema field),
      this parameter specifies which event type (for example, 'click' or 'like') is used for
      training the model.</p>
         <p>If you do not provide an <code>eventType</code>, Amazon Personalize will use all interactions for training with
       equal weight regardless of type.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `solution` | String | <p>An object that describes the solution.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create solution
solution = provider.personalize.Solution {
    name = "value"  # <p>The name for the solution.</p>
    dataset_group_arn = "value"  # <p>The Amazon Resource Name (ARN) of the dataset group that provides the training data.</p>
}

# Access solution outputs
solution_id = solution.id
solution_solution = solution.solution
```

---


### Dataset_import_job

DatasetImportJob resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `data_source` | String | ✅ | <p>The Amazon S3 bucket that contains the training data to import.</p> |
| `publish_attribution_metrics_to_s3` | bool |  | <p>If you created a metric attribution, specify whether to publish metrics for this import job to Amazon S3</p> |
| `job_name` | String | ✅ | <p>The name for the dataset import job.</p> |
| `role_arn` | String | ✅ | <p>The ARN of the IAM role that has permissions to read from the Amazon S3
      data source.</p> |
| `import_mode` | String |  | <p>Specify how to add the new records to an existing dataset. The default
      import mode is <code>FULL</code>. If you haven't imported bulk records into the dataset previously, you
      can only specify <code>FULL</code>.</p>
         <ul>
            <li>
               <p>Specify <code>FULL</code> to overwrite all existing bulk data in
          your dataset. Data you imported individually is not replaced.</p>
            </li>
            <li>
               <p>Specify <code>INCREMENTAL</code> to append the new records to the
          existing data in your dataset. Amazon Personalize replaces any record with the
          same ID with the new one.</p>
            </li>
         </ul> |
| `tags` | Vec<String> |  | <p>A list of <a href="https://docs.aws.amazon.com/personalize/latest/dg/tagging-resources.html">tags</a> to apply to the dataset import job.</p> |
| `dataset_arn` | String | ✅ | <p>The ARN of the dataset that receives the imported data.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `dataset_import_job` | String | <p>Information about the dataset import job, including the status.</p>
         <p>The status is one of the following values:</p>
         <ul>
            <li>
               <p>CREATE PENDING</p>
            </li>
            <li>
               <p>CREATE IN_PROGRESS</p>
            </li>
            <li>
               <p>ACTIVE</p>
            </li>
            <li>
               <p>CREATE FAILED</p>
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
dataset_import_job = provider.personalize.Dataset_import_job {
    data_source = "value"  # <p>The Amazon S3 bucket that contains the training data to import.</p>
    job_name = "value"  # <p>The name for the dataset import job.</p>
    role_arn = "value"  # <p>The ARN of the IAM role that has permissions to read from the Amazon S3
      data source.</p>
    dataset_arn = "value"  # <p>The ARN of the dataset that receives the imported data.</p>
}

# Access dataset_import_job outputs
dataset_import_job_id = dataset_import_job.id
dataset_import_job_dataset_import_job = dataset_import_job.dataset_import_job
```

---


### Campaign

Campaign resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>A list of <a href="https://docs.aws.amazon.com/personalize/latest/dg/tagging-resources.html">tags</a> to apply to the campaign.</p> |
| `campaign_config` | String |  | <p>The configuration details of a campaign.</p> |
| `solution_version_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the trained model to deploy with the campaign. To specify the latest solution version of your solution, 
      specify the ARN of your <i>solution</i> in <code>SolutionArn/$LATEST</code> format.
      You must use this format if you set <code>syncWithLatestSolutionVersion</code> to <code>True</code> in the 
      <a href="https://docs.aws.amazon.com/personalize/latest/dg/API_CampaignConfig.html">CampaignConfig</a>.
    </p>
         <p>
      To deploy a model that isn't the latest solution version of your solution, specify the ARN of the solution version.
    </p>
         <p>
      For more information about automatic campaign updates, see 
      <a href="https://docs.aws.amazon.com/personalize/latest/dg/campaigns.html#create-campaign-automatic-latest-sv-update">Enabling automatic campaign updates</a>.
    </p> |
| `name` | String | ✅ | <p>A name for the new campaign. The campaign name must be unique within your account.</p> |
| `min_provisioned_tps` | i64 |  | <p>Specifies the requested minimum provisioned transactions (recommendations) per second that
      Amazon Personalize will support.  A high <code>minProvisionedTPS</code> will increase your bill. We recommend starting with 1 for <code>minProvisionedTPS</code> (the default). Track
      your usage using Amazon CloudWatch metrics, and increase the <code>minProvisionedTPS</code> as necessary.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `campaign` | String | <p>The properties of the campaign.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create campaign
campaign = provider.personalize.Campaign {
    solution_version_arn = "value"  # <p>The Amazon Resource Name (ARN) of the trained model to deploy with the campaign. To specify the latest solution version of your solution, 
      specify the ARN of your <i>solution</i> in <code>SolutionArn/$LATEST</code> format.
      You must use this format if you set <code>syncWithLatestSolutionVersion</code> to <code>True</code> in the 
      <a href="https://docs.aws.amazon.com/personalize/latest/dg/API_CampaignConfig.html">CampaignConfig</a>.
    </p>
         <p>
      To deploy a model that isn't the latest solution version of your solution, specify the ARN of the solution version.
    </p>
         <p>
      For more information about automatic campaign updates, see 
      <a href="https://docs.aws.amazon.com/personalize/latest/dg/campaigns.html#create-campaign-automatic-latest-sv-update">Enabling automatic campaign updates</a>.
    </p>
    name = "value"  # <p>A name for the new campaign. The campaign name must be unique within your account.</p>
}

# Access campaign outputs
campaign_id = campaign.id
campaign_campaign = campaign.campaign
```

---


### Filter

Filter resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>The name of the filter to create.</p> |
| `dataset_group_arn` | String | ✅ | <p>The ARN of the dataset group that the filter will belong to.</p> |
| `tags` | Vec<String> |  | <p>A list of <a href="https://docs.aws.amazon.com/personalize/latest/dg/tagging-resources.html">tags</a> to apply to the filter.</p> |
| `filter_expression` | String | ✅ | <p>The filter expression defines which items are included or excluded from recommendations. Filter expression must follow specific format rules. 
            For information about filter expression structure and syntax, see
            <a href="https://docs.aws.amazon.com/personalize/latest/dg/filter-expressions.html">Filter expressions</a>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `filter` | String | <p>The filter's details.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create filter
filter = provider.personalize.Filter {
    name = "value"  # <p>The name of the filter to create.</p>
    dataset_group_arn = "value"  # <p>The ARN of the dataset group that the filter will belong to.</p>
    filter_expression = "value"  # <p>The filter expression defines which items are included or excluded from recommendations. Filter expression must follow specific format rules. 
            For information about filter expression structure and syntax, see
            <a href="https://docs.aws.amazon.com/personalize/latest/dg/filter-expressions.html">Filter expressions</a>.</p>
}

# Access filter outputs
filter_id = filter.id
filter_filter = filter.filter
```

---


### Recipe

Recipe resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `recipe` | String | <p>An object that describes the recipe.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access recipe outputs
recipe_id = recipe.id
recipe_recipe = recipe.recipe
```

---


### Batch_segment_job

BatchSegmentJob resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `filter_arn` | String |  | <p>The ARN of the filter to apply to the batch segment job. For more information on using
      filters, see <a href="https://docs.aws.amazon.com/personalize/latest/dg/filter-batch.html">Filtering batch recommendations</a>.</p> |
| `solution_version_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the solution version you want the batch segment job to use to generate
      batch segments.</p> |
| `num_results` | i64 |  | <p>The number of predicted users generated by the batch segment job for each line of input data. The maximum number of users per segment is 5 million.</p> |
| `job_name` | String | ✅ | <p>The name of the batch segment job to create.</p> |
| `job_input` | String | ✅ | <p>The Amazon S3 path for the input data used to generate the batch segment job.</p> |
| `role_arn` | String | ✅ | <p>The ARN of the Amazon Identity and Access Management role that has permissions to read and write to your input and output
      Amazon S3 buckets respectively.</p> |
| `tags` | Vec<String> |  | <p>A list of <a href="https://docs.aws.amazon.com/personalize/latest/dg/tagging-resources.html">tags</a> to apply to the batch segment job.</p> |
| `job_output` | String | ✅ | <p>The Amazon S3 path for the bucket where the job's output will be stored.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `batch_segment_job` | String | <p>Information on the specified batch segment job.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create batch_segment_job
batch_segment_job = provider.personalize.Batch_segment_job {
    solution_version_arn = "value"  # <p>The Amazon Resource Name (ARN) of the solution version you want the batch segment job to use to generate
      batch segments.</p>
    job_name = "value"  # <p>The name of the batch segment job to create.</p>
    job_input = "value"  # <p>The Amazon S3 path for the input data used to generate the batch segment job.</p>
    role_arn = "value"  # <p>The ARN of the Amazon Identity and Access Management role that has permissions to read and write to your input and output
      Amazon S3 buckets respectively.</p>
    job_output = "value"  # <p>The Amazon S3 path for the bucket where the job's output will be stored.</p>
}

# Access batch_segment_job outputs
batch_segment_job_id = batch_segment_job.id
batch_segment_job_batch_segment_job = batch_segment_job.batch_segment_job
```

---


### Event_tracker

EventTracker resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>A list of <a href="https://docs.aws.amazon.com/personalize/latest/dg/tagging-resources.html">tags</a> to apply to the event tracker.</p> |
| `name` | String | ✅ | <p>The name for the event tracker.</p> |
| `dataset_group_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the dataset group that receives the event data.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `event_tracker` | String | <p>An object that describes the event tracker.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create event_tracker
event_tracker = provider.personalize.Event_tracker {
    name = "value"  # <p>The name for the event tracker.</p>
    dataset_group_arn = "value"  # <p>The Amazon Resource Name (ARN) of the dataset group that receives the event data.</p>
}

# Access event_tracker outputs
event_tracker_id = event_tracker.id
event_tracker_event_tracker = event_tracker.event_tracker
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple solution_metrics resources
solution_metrics_0 = provider.personalize.Solution_metrics {
}
solution_metrics_1 = provider.personalize.Solution_metrics {
}
solution_metrics_2 = provider.personalize.Solution_metrics {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    solution_metrics = provider.personalize.Solution_metrics {
    }
```

---

## Related Documentation

- [AWS Personalize Documentation](https://docs.aws.amazon.com/personalize/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
