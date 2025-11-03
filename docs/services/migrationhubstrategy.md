# Migrationhubstrategy Service



**Resources**: 12

---

## Overview

The migrationhubstrategy service provides access to 12 resource types:

- [Recommendation_report_details](#recommendation_report_details) [R]
- [Application_component_details](#application_component_details) [R]
- [Server_strategies](#server_strategies) [R]
- [Portfolio_preferences](#portfolio_preferences) [CR]
- [Import_file_task](#import_file_task) [R]
- [Portfolio_summary](#portfolio_summary) [R]
- [Latest_assessment_id](#latest_assessment_id) [R]
- [Application_component_config](#application_component_config) [U]
- [Assessment](#assessment) [R]
- [Server_config](#server_config) [U]
- [Server_details](#server_details) [R]
- [Application_component_strategies](#application_component_strategies) [R]

---

## Resources


### Recommendation_report_details

RecommendationReportDetails resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `id` | String | <p> The ID of the recommendation report generation task. See the response of <a>StartRecommendationReportGeneration</a>. </p> |
| `recommendation_report_details` | String | <p> Detailed information about the recommendation report. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access recommendation_report_details outputs
recommendation_report_details_id = recommendation_report_details.id
recommendation_report_details_id = recommendation_report_details.id
recommendation_report_details_recommendation_report_details = recommendation_report_details.recommendation_report_details
```

---


### Application_component_details

ApplicationComponentDetails resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `more_application_resource` | bool | <p> Set to true if the application component belongs to more than one application group.
    </p> |
| `associated_applications` | Vec<String> | <p> The associated application group as defined in AWS Application Discovery Service. </p> |
| `application_component_detail` | String | <p> Detailed information about an application component. </p> |
| `associated_server_ids` | Vec<String> | <p> A list of the IDs of the servers on which the application component is running. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access application_component_details outputs
application_component_details_id = application_component_details.id
application_component_details_more_application_resource = application_component_details.more_application_resource
application_component_details_associated_applications = application_component_details.associated_applications
application_component_details_application_component_detail = application_component_details.application_component_detail
application_component_details_associated_server_ids = application_component_details.associated_server_ids
```

---


### Server_strategies

ServerStrategies resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `server_strategies` | Vec<String> | <p> A list of strategy recommendations for the server. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access server_strategies outputs
server_strategies_id = server_strategies.id
server_strategies_server_strategies = server_strategies.server_strategies
```

---


### Portfolio_preferences

PortfolioPreferences resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `database_preferences` | String |  | <p> The transformation preferences for database applications. </p> |
| `application_preferences` | String |  | <p> The transformation preferences for non-database applications. </p> |
| `application_mode` | String |  | <p>The classification for application component types.</p> |
| `prioritize_business_goals` | String |  | <p> The rank of the business goals based on priority. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `database_preferences` | String | <p> The transformation preferences for database applications. </p> |
| `application_mode` | String | <p>The classification for application component types.</p> |
| `prioritize_business_goals` | String | <p> The rank of business goals based on priority. </p> |
| `application_preferences` | String | <p> The transformation preferences for non-database applications. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create portfolio_preferences
portfolio_preferences = provider.migrationhubstrategy.Portfolio_preferences {
}

# Access portfolio_preferences outputs
portfolio_preferences_id = portfolio_preferences.id
portfolio_preferences_database_preferences = portfolio_preferences.database_preferences
portfolio_preferences_application_mode = portfolio_preferences.application_mode
portfolio_preferences_prioritize_business_goals = portfolio_preferences.prioritize_business_goals
portfolio_preferences_application_preferences = portfolio_preferences.application_preferences
```

---


### Import_file_task

ImportFileTask resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | String | <p> Status of import file task. </p> |
| `status_report_s3_bucket` | String | <p> The S3 bucket name for status report of import task. </p> |
| `input_s3_key` | String | <p> The Amazon S3 key name of the import file. </p> |
| `completion_time` | String | <p> The time that the import task completed. </p> |
| `number_of_records_failed` | i64 | <p> The number of records that failed to be imported. </p> |
| `import_name` | String | <p> The name of the import task given in <a>StartImportFileTask</a>. </p> |
| `number_of_records_success` | i64 | <p> The number of records successfully imported. </p> |
| `status_report_s3_key` | String | <p> The Amazon S3 key name for status report of import task. The report contains details about
      whether each record imported successfully or why it did not.</p> |
| `input_s3_bucket` | String | <p> The S3 bucket where import file is located. </p> |
| `id` | String | <p> The import file task <code>id</code> returned in the response of <a>StartImportFileTask</a>. </p> |
| `start_time` | String | <p> Start time of the import task. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access import_file_task outputs
import_file_task_id = import_file_task.id
import_file_task_status = import_file_task.status
import_file_task_status_report_s3_bucket = import_file_task.status_report_s3_bucket
import_file_task_input_s3_key = import_file_task.input_s3_key
import_file_task_completion_time = import_file_task.completion_time
import_file_task_number_of_records_failed = import_file_task.number_of_records_failed
import_file_task_import_name = import_file_task.import_name
import_file_task_number_of_records_success = import_file_task.number_of_records_success
import_file_task_status_report_s3_key = import_file_task.status_report_s3_key
import_file_task_input_s3_bucket = import_file_task.input_s3_bucket
import_file_task_id = import_file_task.id
import_file_task_start_time = import_file_task.start_time
```

---


### Portfolio_summary

PortfolioSummary resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `assessment_summary` | String | <p> An assessment summary for the portfolio including the number of servers to rehost and the
      overall number of anti-patterns. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access portfolio_summary outputs
portfolio_summary_id = portfolio_summary.id
portfolio_summary_assessment_summary = portfolio_summary.assessment_summary
```

---


### Latest_assessment_id

LatestAssessmentId resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `id` | String | <p>The latest ID for the specific assessment task.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access latest_assessment_id outputs
latest_assessment_id_id = latest_assessment_id.id
latest_assessment_id_id = latest_assessment_id.id
```

---


### Application_component_config

ApplicationComponentConfig resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `secrets_manager_key` | String |  | <p> Database credentials. </p> |
| `strategy_option` | String |  | <p> The preferred strategy options for the application component. Use values from the <a>GetApplicationComponentStrategies</a> response. </p> |
| `configure_only` | bool |  | <p>Update the configuration request of an application component. If it is set to true, the
      source code and/or database credentials are updated. If it is set to false, the source code
      and/or database credentials are updated and an analysis is initiated.</p> |
| `app_type` | String |  | <p>The type of known component.</p> |
| `application_component_id` | String | ✅ | <p> The ID of the application component. The ID is unique within an AWS account. </p> |
| `inclusion_status` | String |  | <p> Indicates whether the application component has been included for server recommendation
      or not. </p> |
| `source_code_list` | Vec<String> |  | <p> The list of source code configurations to update for the application component. </p> |



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


### Assessment

Assessment resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `id` | String | <p> The ID for the specific assessment task. </p> |
| `assessment_targets` | Vec<String> | <p>List of criteria for assessment.</p> |
| `data_collection_details` | String | <p> Detailed information about the assessment. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access assessment outputs
assessment_id = assessment.id
assessment_id = assessment.id
assessment_assessment_targets = assessment.assessment_targets
assessment_data_collection_details = assessment.data_collection_details
```

---


### Server_config

ServerConfig resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `server_id` | String | ✅ | <p> The ID of the server. </p> |
| `strategy_option` | String |  | <p> The preferred strategy options for the application component. See the response from <a>GetServerStrategies</a>.</p> |



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


### Server_details

ServerDetails resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `server_detail` | String | <p> Detailed information about the server. </p> |
| `next_token` | String | <p> The token you use to retrieve the next set of results, or null if there are no more results. </p> |
| `associated_applications` | Vec<String> | <p> The associated application group the server belongs to, as defined in AWS Application Discovery Service.
    </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access server_details outputs
server_details_id = server_details.id
server_details_server_detail = server_details.server_detail
server_details_next_token = server_details.next_token
server_details_associated_applications = server_details.associated_applications
```

---


### Application_component_strategies

ApplicationComponentStrategies resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `application_component_strategies` | Vec<String> | <p> A list of application component strategy recommendations. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access application_component_strategies outputs
application_component_strategies_id = application_component_strategies.id
application_component_strategies_application_component_strategies = application_component_strategies.application_component_strategies
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple recommendation_report_details resources
recommendation_report_details_0 = provider.migrationhubstrategy.Recommendation_report_details {
}
recommendation_report_details_1 = provider.migrationhubstrategy.Recommendation_report_details {
}
recommendation_report_details_2 = provider.migrationhubstrategy.Recommendation_report_details {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    recommendation_report_details = provider.migrationhubstrategy.Recommendation_report_details {
    }
```

---

## Related Documentation

- [AWS Migrationhubstrategy Documentation](https://docs.aws.amazon.com/migrationhubstrategy/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
