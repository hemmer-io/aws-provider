# Databrew Service



**Resources**: 10

---

## Overview

The databrew service provides access to 10 resource types:

- [Project](#project) [CRUD]
- [Job_run](#job_run) [R]
- [Job](#job) [RD]
- [Recipe_version](#recipe_version) [D]
- [Schedule](#schedule) [CRUD]
- [Profile_job](#profile_job) [CU]
- [Dataset](#dataset) [CRUD]
- [Recipe](#recipe) [CRU]
- [Recipe_job](#recipe_job) [CU]
- [Ruleset](#ruleset) [CRUD]

---

## Resources


### Project

Project resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `sample` | String |  |  |
| `role_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the Identity and Access Management (IAM) role to
            be assumed for this request.</p> |
| `tags` | HashMap<String, String> |  | <p>Metadata tags to apply to this project.</p> |
| `dataset_name` | String | ✅ | <p>The name of an existing dataset to associate this project with.</p> |
| `recipe_name` | String | ✅ | <p>The name of an existing recipe to associate with the project.</p> |
| `name` | String | ✅ | <p>A unique name for the new project. Valid characters are alphanumeric (A-Z, a-z, 0-9),
            hyphen (-), period (.), and space.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `dataset_name` | String | <p>The dataset associated with the project.</p> |
| `create_date` | String | <p>The date and time that the project was created.</p> |
| `last_modified_date` | String | <p>The date and time that the project was last modified.</p> |
| `last_modified_by` | String | <p>The identifier (user name) of the user who last modified the project.</p> |
| `resource_arn` | String | <p>The Amazon Resource Name (ARN) of the project.</p> |
| `sample` | String |  |
| `created_by` | String | <p>The identifier (user name) of the user who created the project.</p> |
| `role_arn` | String | <p>The ARN of the Identity and Access Management (IAM) role to be assumed when
            DataBrew runs the job.</p> |
| `recipe_name` | String | <p>The recipe associated with this job.</p> |
| `opened_by` | String | <p>The identifier (user name) of the user that opened the project for use. </p> |
| `name` | String | <p>The name of the project.</p> |
| `open_date` | String | <p>The date and time when the project was opened. </p> |
| `tags` | HashMap<String, String> | <p>Metadata tags associated with this project.</p> |
| `session_status` | String | <p>Describes the current state of the session:</p>
         <ul>
            <li>
               <p>
                  <code>PROVISIONING</code> - allocating resources for the session.</p>
            </li>
            <li>
               <p>
                  <code>INITIALIZING</code> - getting the session ready for first use.</p>
            </li>
            <li>
               <p>
                  <code>ASSIGNED</code> - the session is ready for use.</p>
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

# Create project
project = provider.databrew.Project {
    role_arn = "value"  # <p>The Amazon Resource Name (ARN) of the Identity and Access Management (IAM) role to
            be assumed for this request.</p>
    dataset_name = "value"  # <p>The name of an existing dataset to associate this project with.</p>
    recipe_name = "value"  # <p>The name of an existing recipe to associate with the project.</p>
    name = "value"  # <p>A unique name for the new project. Valid characters are alphanumeric (A-Z, a-z, 0-9),
            hyphen (-), period (.), and space.</p>
}

# Access project outputs
project_id = project.id
project_dataset_name = project.dataset_name
project_create_date = project.create_date
project_last_modified_date = project.last_modified_date
project_last_modified_by = project.last_modified_by
project_resource_arn = project.resource_arn
project_sample = project.sample
project_created_by = project.created_by
project_role_arn = project.role_arn
project_recipe_name = project.recipe_name
project_opened_by = project.opened_by
project_name = project.name
project_open_date = project.open_date
project_tags = project.tags
project_session_status = project.session_status
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
| `execution_time` | i64 | <p>The amount of time, in seconds, during which the job run consumed resources.</p> |
| `recipe_reference` | String |  |
| `error_message` | String | <p>A message indicating an error (if any) that was encountered when the job ran.</p> |
| `started_by` | String | <p>The Amazon Resource Name (ARN) of the user who started the job run.</p> |
| `state` | String | <p>The current state of the job run entity itself.</p> |
| `profile_configuration` | String | <p>Configuration for profile jobs. Used to select columns, do evaluations, 
            and override default parameters of evaluations. When configuration is null, the
            profile job will run with default settings.</p> |
| `dataset_name` | String | <p>The name of the dataset for the job to process.</p> |
| `run_id` | String | <p>The unique identifier of the job run.</p> |
| `data_catalog_outputs` | Vec<String> | <p>One or more artifacts that represent the Glue Data Catalog output from running the job.</p> |
| `validation_configurations` | Vec<String> | <p>List of validation configurations that are applied to the profile job.</p> |
| `attempt` | i64 | <p>The number of times that DataBrew has attempted to run the job.</p> |
| `outputs` | Vec<String> | <p>One or more output artifacts from a job run.</p> |
| `database_outputs` | Vec<String> | <p>Represents a list of JDBC database output objects which defines the output 
            destination for a DataBrew recipe job to write into.</p> |
| `job_sample` | String | <p>Sample configuration for profile jobs only. Determines the number of rows on which the
            profile job will be executed. If a JobSample value is not provided, the default value
            will be used. The default value is CUSTOM_ROWS for the mode parameter and 20000 for the
            size parameter.</p> |
| `log_group_name` | String | <p>The name of an Amazon CloudWatch log group, where the job writes diagnostic messages
            when it runs.</p> |
| `job_name` | String | <p>The name of the job being processed during this run.</p> |
| `started_on` | String | <p>The date and time when the job run began.</p> |
| `completed_on` | String | <p>The date and time when the job completed processing.</p> |
| `log_subscription` | String | <p>The current status of Amazon CloudWatch logging for the job run.</p> |


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
job_run_execution_time = job_run.execution_time
job_run_recipe_reference = job_run.recipe_reference
job_run_error_message = job_run.error_message
job_run_started_by = job_run.started_by
job_run_state = job_run.state
job_run_profile_configuration = job_run.profile_configuration
job_run_dataset_name = job_run.dataset_name
job_run_run_id = job_run.run_id
job_run_data_catalog_outputs = job_run.data_catalog_outputs
job_run_validation_configurations = job_run.validation_configurations
job_run_attempt = job_run.attempt
job_run_outputs = job_run.outputs
job_run_database_outputs = job_run.database_outputs
job_run_job_sample = job_run.job_sample
job_run_log_group_name = job_run.log_group_name
job_run_job_name = job_run.job_name
job_run_started_on = job_run.started_on
job_run_completed_on = job_run.completed_on
job_run_log_subscription = job_run.log_subscription
```

---


### Job

Job resource

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `type` | String | <p>The job type, which must be one of the following:</p>
         <ul>
            <li>
               <p>
                  <code>PROFILE</code> - The job analyzes the dataset to determine its size,
                    data types, data distribution, and more.</p>
            </li>
            <li>
               <p>
                  <code>RECIPE</code> - The job applies one or more transformations to a
                    dataset.</p>
            </li>
         </ul> |
| `encryption_key_arn` | String | <p>The Amazon Resource Name (ARN) of an encryption key that is used to protect the
            job.</p> |
| `name` | String | <p>The name of the job.</p> |
| `project_name` | String | <p>The DataBrew project associated with this job.</p> |
| `dataset_name` | String | <p>The dataset that the job acts upon.</p> |
| `data_catalog_outputs` | Vec<String> | <p>One or more artifacts that represent the Glue Data Catalog output from running the job.</p> |
| `outputs` | Vec<String> | <p>One or more artifacts that represent the output from running the job.</p> |
| `tags` | HashMap<String, String> | <p>Metadata tags associated with this job.</p> |
| `last_modified_date` | String | <p>The date and time that the job was last modified.</p> |
| `resource_arn` | String | <p>The Amazon Resource Name (ARN) of the job.</p> |
| `role_arn` | String | <p>The ARN of the Identity and Access Management (IAM) role to be assumed when
            DataBrew runs the job.</p> |
| `timeout` | i64 | <p>The job's timeout in minutes. A job that attempts to run longer than this timeout
            period ends with a status of <code>TIMEOUT</code>.</p> |
| `encryption_mode` | String | <p>The encryption mode for the job, which can be one of the following:</p>
         <ul>
            <li>
               <p>
                  <code>SSE-KMS</code> - Server-side encryption with keys managed by KMS.</p>
            </li>
            <li>
               <p>
                  <code>SSE-S3</code> - Server-side encryption with keys managed by Amazon
                    S3.</p>
            </li>
         </ul> |
| `max_capacity` | i64 | <p>The maximum number of compute nodes that DataBrew can consume when the job processes
            data.</p> |
| `log_subscription` | String | <p>Indicates whether Amazon CloudWatch logging is enabled for this job.</p> |
| `create_date` | String | <p>The date and time that the job was created.</p> |
| `last_modified_by` | String | <p>The identifier (user name) of the user who last modified the job.</p> |
| `profile_configuration` | String | <p>Configuration for profile jobs. Used to select columns, do evaluations, 
            and override default parameters of evaluations. When configuration is null, the
            profile job will run with default settings.</p> |
| `created_by` | String | <p>The identifier (user name) of the user associated with the creation of the job.</p> |
| `validation_configurations` | Vec<String> | <p>List of validation configurations that are applied to the profile job.</p> |
| `database_outputs` | Vec<String> | <p>Represents a list of JDBC database output objects which defines the output 
            destination for a DataBrew recipe job to write into.</p> |
| `recipe_reference` | String |  |
| `job_sample` | String | <p>Sample configuration for profile jobs only. Determines the number of rows on which the
            profile job will be executed.</p> |
| `max_retries` | i64 | <p>The maximum number of times to retry the job after a job run fails.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access job outputs
job_id = job.id
job_type = job.type
job_encryption_key_arn = job.encryption_key_arn
job_name = job.name
job_project_name = job.project_name
job_dataset_name = job.dataset_name
job_data_catalog_outputs = job.data_catalog_outputs
job_outputs = job.outputs
job_tags = job.tags
job_last_modified_date = job.last_modified_date
job_resource_arn = job.resource_arn
job_role_arn = job.role_arn
job_timeout = job.timeout
job_encryption_mode = job.encryption_mode
job_max_capacity = job.max_capacity
job_log_subscription = job.log_subscription
job_create_date = job.create_date
job_last_modified_by = job.last_modified_by
job_profile_configuration = job.profile_configuration
job_created_by = job.created_by
job_validation_configurations = job.validation_configurations
job_database_outputs = job.database_outputs
job_recipe_reference = job.recipe_reference
job_job_sample = job.job_sample
job_max_retries = job.max_retries
```

---


### Recipe_version

RecipeVersion resource

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


### Schedule

Schedule resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `job_names` | Vec<String> |  | <p>The name or names of one or more jobs to be run.</p> |
| `name` | String | ✅ | <p>A unique name for the schedule. Valid characters are alphanumeric (A-Z, a-z, 0-9),
            hyphen (-), period (.), and space.</p> |
| `cron_expression` | String | ✅ | <p>The date or dates and time or times when the jobs are to be run. For more information,
            see <a href="https://docs.aws.amazon.com/databrew/latest/dg/jobs.cron.html">Cron
                expressions</a> in the <i>Glue DataBrew Developer
            Guide</i>.</p> |
| `tags` | HashMap<String, String> |  | <p>Metadata tags to apply to this schedule.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `job_names` | Vec<String> | <p>The name or names of one or more jobs to be run by using the schedule.</p> |
| `name` | String | <p>The name of the schedule.</p> |
| `cron_expression` | String | <p>The date or dates and time or times when the jobs are to be run for the schedule. For
            more information, see <a href="https://docs.aws.amazon.com/databrew/latest/dg/jobs.cron.html">Cron expressions</a> in the
            <i>Glue DataBrew Developer Guide</i>.</p> |
| `create_date` | String | <p>The date and time that the schedule was created.</p> |
| `last_modified_by` | String | <p>The identifier (user name) of the user who last modified the schedule.</p> |
| `last_modified_date` | String | <p>The date and time that the schedule was last modified.</p> |
| `created_by` | String | <p>The identifier (user name) of the user who created the schedule. </p> |
| `tags` | HashMap<String, String> | <p>Metadata tags associated with this schedule.</p> |
| `resource_arn` | String | <p>The Amazon Resource Name (ARN) of the schedule.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create schedule
schedule = provider.databrew.Schedule {
    name = "value"  # <p>A unique name for the schedule. Valid characters are alphanumeric (A-Z, a-z, 0-9),
            hyphen (-), period (.), and space.</p>
    cron_expression = "value"  # <p>The date or dates and time or times when the jobs are to be run. For more information,
            see <a href="https://docs.aws.amazon.com/databrew/latest/dg/jobs.cron.html">Cron
                expressions</a> in the <i>Glue DataBrew Developer
            Guide</i>.</p>
}

# Access schedule outputs
schedule_id = schedule.id
schedule_job_names = schedule.job_names
schedule_name = schedule.name
schedule_cron_expression = schedule.cron_expression
schedule_create_date = schedule.create_date
schedule_last_modified_by = schedule.last_modified_by
schedule_last_modified_date = schedule.last_modified_date
schedule_created_by = schedule.created_by
schedule_tags = schedule.tags
schedule_resource_arn = schedule.resource_arn
```

---


### Profile_job

ProfileJob resource

**Operations**: ✅ Create ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `job_sample` | String |  | <p>Sample configuration for profile jobs only. Determines the number of rows on which the
            profile job will be executed. If a JobSample value is not provided, the default value
            will be used. The default value is CUSTOM_ROWS for the mode parameter and 20000 for the
            size parameter.</p> |
| `name` | String | ✅ | <p>The name of the job to be created. Valid characters are alphanumeric (A-Z, a-z, 0-9),
            hyphen (-), period (.), and space.</p> |
| `encryption_mode` | String |  | <p>The encryption mode for the job, which can be one of the following:</p>
         <ul>
            <li>
               <p>
                  <code>SSE-KMS</code> - <code>SSE-KMS</code> - Server-side encryption with 
                    KMS-managed keys.</p>
            </li>
            <li>
               <p>
                  <code>SSE-S3</code> - Server-side encryption with keys managed by Amazon S3.</p>
            </li>
         </ul> |
| `tags` | HashMap<String, String> |  | <p>Metadata tags to apply to this job.</p> |
| `configuration` | String |  | <p>Configuration for profile jobs. Used to select columns, do evaluations, 
            and override default parameters of evaluations. When configuration is null, the
            profile job will run with default settings.</p> |
| `validation_configurations` | Vec<String> |  | <p>List of validation configurations that are applied to the profile job.</p> |
| `dataset_name` | String | ✅ | <p>The name of the dataset that this job is to act upon.</p> |
| `max_capacity` | i64 |  | <p>The maximum number of nodes that DataBrew can use when the job processes data.</p> |
| `timeout` | i64 |  | <p>The job's timeout in minutes. A job that attempts to run longer than this timeout
            period ends with a status of <code>TIMEOUT</code>.</p> |
| `log_subscription` | String |  | <p>Enables or disables Amazon CloudWatch logging for the job. If logging is enabled,
            CloudWatch writes one log stream for each job run.</p> |
| `max_retries` | i64 |  | <p>The maximum number of times to retry the job after a job run fails.</p> |
| `output_location` | String | ✅ |  |
| `encryption_key_arn` | String |  | <p>The Amazon Resource Name (ARN) of an encryption key that is used to protect the
            job.</p> |
| `role_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the Identity and Access Management (IAM) role to
            be assumed when DataBrew runs the job.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create profile_job
profile_job = provider.databrew.Profile_job {
    name = "value"  # <p>The name of the job to be created. Valid characters are alphanumeric (A-Z, a-z, 0-9),
            hyphen (-), period (.), and space.</p>
    dataset_name = "value"  # <p>The name of the dataset that this job is to act upon.</p>
    output_location = "value"  # Required field
    role_arn = "value"  # <p>The Amazon Resource Name (ARN) of the Identity and Access Management (IAM) role to
            be assumed when DataBrew runs the job.</p>
}

```

---


### Dataset

Dataset resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `format` | String |  | <p>The file format of a dataset that is created from an Amazon S3 file or folder.</p> |
| `name` | String | ✅ | <p>The name of the dataset to be created. Valid characters are alphanumeric (A-Z, a-z,
            0-9), hyphen (-), period (.), and space.</p> |
| `input` | String | ✅ |  |
| `tags` | HashMap<String, String> |  | <p>Metadata tags to apply to this dataset.</p> |
| `format_options` | String |  |  |
| `path_options` | String |  | <p>A set of options that defines how DataBrew interprets an Amazon S3 path of the dataset.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `path_options` | String | <p>A set of options that defines how DataBrew interprets an Amazon S3 
            path of the dataset.</p> |
| `input` | String |  |
| `last_modified_by` | String | <p>The identifier (user name) of the user who last modified the dataset.</p> |
| `created_by` | String | <p>The identifier (user name) of the user who created the dataset.</p> |
| `format` | String | <p>The file format of a dataset that is created from an Amazon S3 file 
            or folder.</p> |
| `source` | String | <p>The location of the data for this dataset, Amazon S3 or the 
            Glue Data Catalog.</p> |
| `create_date` | String | <p>The date and time that the dataset was created.</p> |
| `format_options` | String |  |
| `name` | String | <p>The name of the dataset.</p> |
| `last_modified_date` | String | <p>The date and time that the dataset was last modified.</p> |
| `tags` | HashMap<String, String> | <p>Metadata tags associated with this dataset.</p> |
| `resource_arn` | String | <p>The Amazon Resource Name (ARN) of the dataset.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create dataset
dataset = provider.databrew.Dataset {
    name = "value"  # <p>The name of the dataset to be created. Valid characters are alphanumeric (A-Z, a-z,
            0-9), hyphen (-), period (.), and space.</p>
    input = "value"  # Required field
}

# Access dataset outputs
dataset_id = dataset.id
dataset_path_options = dataset.path_options
dataset_input = dataset.input
dataset_last_modified_by = dataset.last_modified_by
dataset_created_by = dataset.created_by
dataset_format = dataset.format
dataset_source = dataset.source
dataset_create_date = dataset.create_date
dataset_format_options = dataset.format_options
dataset_name = dataset.name
dataset_last_modified_date = dataset.last_modified_date
dataset_tags = dataset.tags
dataset_resource_arn = dataset.resource_arn
```

---


### Recipe

Recipe resource

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `steps` | Vec<String> | ✅ | <p>An array containing the steps to be performed by the recipe. Each recipe step consists
            of one recipe action and (optionally) an array of condition expressions.</p> |
| `tags` | HashMap<String, String> |  | <p>Metadata tags to apply to this recipe.</p> |
| `description` | String |  | <p>A description for the recipe.</p> |
| `name` | String | ✅ | <p>A unique name for the recipe. Valid characters are alphanumeric (A-Z, a-z, 0-9),
            hyphen (-), period (.), and space.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `recipe_version` | String | <p>The recipe version identifier.</p> |
| `description` | String | <p>The description of the recipe.</p> |
| `tags` | HashMap<String, String> | <p>Metadata tags associated with this project.</p> |
| `project_name` | String | <p>The name of the project associated with this recipe.</p> |
| `created_by` | String | <p>The identifier (user name) of the user who created the recipe.</p> |
| `create_date` | String | <p>The date and time that the recipe was created.</p> |
| `published_by` | String | <p>The identifier (user name) of the user who last published the recipe.</p> |
| `last_modified_date` | String | <p>The date and time that the recipe was last modified.</p> |
| `published_date` | String | <p>The date and time when the recipe was last published.</p> |
| `last_modified_by` | String | <p>The identifier (user name) of the user who last modified the recipe.</p> |
| `steps` | Vec<String> | <p>One or more steps to be performed by the recipe. Each step consists of an action, and
            the conditions under which the action should succeed.</p> |
| `resource_arn` | String | <p>The ARN of the recipe.</p> |
| `name` | String | <p>The name of the recipe.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create recipe
recipe = provider.databrew.Recipe {
    steps = "value"  # <p>An array containing the steps to be performed by the recipe. Each recipe step consists
            of one recipe action and (optionally) an array of condition expressions.</p>
    name = "value"  # <p>A unique name for the recipe. Valid characters are alphanumeric (A-Z, a-z, 0-9),
            hyphen (-), period (.), and space.</p>
}

# Access recipe outputs
recipe_id = recipe.id
recipe_recipe_version = recipe.recipe_version
recipe_description = recipe.description
recipe_tags = recipe.tags
recipe_project_name = recipe.project_name
recipe_created_by = recipe.created_by
recipe_create_date = recipe.create_date
recipe_published_by = recipe.published_by
recipe_last_modified_date = recipe.last_modified_date
recipe_published_date = recipe.published_date
recipe_last_modified_by = recipe.last_modified_by
recipe_steps = recipe.steps
recipe_resource_arn = recipe.resource_arn
recipe_name = recipe.name
```

---


### Recipe_job

RecipeJob resource

**Operations**: ✅ Create ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>A unique name for the job. Valid characters are alphanumeric (A-Z, a-z, 0-9), hyphen
            (-), period (.), and space.</p> |
| `log_subscription` | String |  | <p>Enables or disables Amazon CloudWatch logging for the job. If logging is enabled,
            CloudWatch writes one log stream for each job run.</p> |
| `database_outputs` | Vec<String> |  | <p>Represents a list of JDBC database output objects which defines the output destination for 
            a DataBrew recipe job to write to. </p> |
| `encryption_mode` | String |  | <p>The encryption mode for the job, which can be one of the following:</p>
         <ul>
            <li>
               <p>
                  <code>SSE-KMS</code> - Server-side encryption with keys managed by KMS.</p>
            </li>
            <li>
               <p>
                  <code>SSE-S3</code> - Server-side encryption with keys managed by Amazon S3.</p>
            </li>
         </ul> |
| `role_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the Identity and Access Management (IAM) role to
            be assumed when DataBrew runs the job.</p> |
| `recipe_reference` | String |  |  |
| `data_catalog_outputs` | Vec<String> |  | <p>One or more artifacts that represent the Glue Data Catalog output from running the job.</p> |
| `project_name` | String |  | <p>Either the name of an existing project, or a combination of a recipe and a dataset to
            associate with the recipe.</p> |
| `tags` | HashMap<String, String> |  | <p>Metadata tags to apply to this job.</p> |
| `max_capacity` | i64 |  | <p>The maximum number of nodes that DataBrew can consume when the job processes
            data.</p> |
| `timeout` | i64 |  | <p>The job's timeout in minutes. A job that attempts to run longer than this timeout
            period ends with a status of <code>TIMEOUT</code>.</p> |
| `outputs` | Vec<String> |  | <p>One or more artifacts that represent the output from running the job.</p> |
| `encryption_key_arn` | String |  | <p>The Amazon Resource Name (ARN) of an encryption key that is used to protect the
            job.</p> |
| `dataset_name` | String |  | <p>The name of the dataset that this job processes.</p> |
| `max_retries` | i64 |  | <p>The maximum number of times to retry the job after a job run fails.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create recipe_job
recipe_job = provider.databrew.Recipe_job {
    name = "value"  # <p>A unique name for the job. Valid characters are alphanumeric (A-Z, a-z, 0-9), hyphen
            (-), period (.), and space.</p>
    role_arn = "value"  # <p>The Amazon Resource Name (ARN) of the Identity and Access Management (IAM) role to
            be assumed when DataBrew runs the job.</p>
}

```

---


### Ruleset

Ruleset resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>The name of the ruleset to be created. Valid characters are alphanumeric 
            (A-Z, a-z, 0-9), hyphen (-), period (.), and space.</p> |
| `rules` | Vec<String> | ✅ | <p>A list of rules that are defined with the ruleset. A rule includes 
            one or more checks to be validated on a DataBrew dataset.</p> |
| `description` | String |  | <p>The description of the ruleset.</p> |
| `tags` | HashMap<String, String> |  | <p>Metadata tags to apply to the ruleset.</p> |
| `target_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of a resource (dataset) that the 
            ruleset is associated with.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | <p>The name of the ruleset.</p> |
| `target_arn` | String | <p>The Amazon Resource Name (ARN) of a resource (dataset) that the ruleset is 
            associated with.</p> |
| `last_modified_date` | String | <p>The modification date and time of the ruleset.</p> |
| `rules` | Vec<String> | <p>A list of rules that are defined with the ruleset. A rule includes one 
            or more checks to be validated on a DataBrew dataset.</p> |
| `tags` | HashMap<String, String> | <p>Metadata tags that have been applied to the ruleset.</p> |
| `resource_arn` | String | <p>The Amazon Resource Name (ARN) for the ruleset.</p> |
| `description` | String | <p>The description of the ruleset.</p> |
| `created_by` | String | <p>The Amazon Resource Name (ARN) of the user who created the ruleset.</p> |
| `last_modified_by` | String | <p>The Amazon Resource Name (ARN) of the user who last modified the ruleset.</p> |
| `create_date` | String | <p>The date and time that the ruleset was created.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create ruleset
ruleset = provider.databrew.Ruleset {
    name = "value"  # <p>The name of the ruleset to be created. Valid characters are alphanumeric 
            (A-Z, a-z, 0-9), hyphen (-), period (.), and space.</p>
    rules = "value"  # <p>A list of rules that are defined with the ruleset. A rule includes 
            one or more checks to be validated on a DataBrew dataset.</p>
    target_arn = "value"  # <p>The Amazon Resource Name (ARN) of a resource (dataset) that the 
            ruleset is associated with.</p>
}

# Access ruleset outputs
ruleset_id = ruleset.id
ruleset_name = ruleset.name
ruleset_target_arn = ruleset.target_arn
ruleset_last_modified_date = ruleset.last_modified_date
ruleset_rules = ruleset.rules
ruleset_tags = ruleset.tags
ruleset_resource_arn = ruleset.resource_arn
ruleset_description = ruleset.description
ruleset_created_by = ruleset.created_by
ruleset_last_modified_by = ruleset.last_modified_by
ruleset_create_date = ruleset.create_date
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple project resources
project_0 = provider.databrew.Project {
    role_arn = "value-0"
    dataset_name = "value-0"
    recipe_name = "value-0"
    name = "value-0"
}
project_1 = provider.databrew.Project {
    role_arn = "value-1"
    dataset_name = "value-1"
    recipe_name = "value-1"
    name = "value-1"
}
project_2 = provider.databrew.Project {
    role_arn = "value-2"
    dataset_name = "value-2"
    recipe_name = "value-2"
    name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    project = provider.databrew.Project {
        role_arn = "production-value"
        dataset_name = "production-value"
        recipe_name = "production-value"
        name = "production-value"
    }
```

---

## Related Documentation

- [AWS Databrew Documentation](https://docs.aws.amazon.com/databrew/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
