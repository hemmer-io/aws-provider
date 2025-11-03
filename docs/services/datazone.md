# Datazone Service



**Resources**: 26

---

## Overview

The datazone service provides access to 26 resource types:

- [Group_profile](#group_profile) [CRU]
- [Time_series_data_point](#time_series_data_point) [R]
- [Job_run](#job_run) [R]
- [Subscription_grant](#subscription_grant) [CRD]
- [Subscription_request_details](#subscription_request_details) [R]
- [Time_series_data_points](#time_series_data_points) [D]
- [Environment](#environment) [CRUD]
- [Environment_profile](#environment_profile) [CRUD]
- [Environment_action](#environment_action) [CRUD]
- [Subscription_target](#subscription_target) [CRUD]
- [Account_pool](#account_pool) [CRUD]
- [Project_membership](#project_membership) [CD]
- [Environment_blueprint](#environment_blueprint) [CRUD]
- [Project_profile](#project_profile) [CRUD]
- [Connection](#connection) [CRUD]
- [User_profile](#user_profile) [CRU]
- [Lineage_node](#lineage_node) [R]
- [Subscription](#subscription) [R]
- [Subscription_request](#subscription_request) [CUD]
- [Subscription_grant_status](#subscription_grant_status) [U]
- [Iam_portal_login_url](#iam_portal_login_url) [R]
- [Lineage_event](#lineage_event) [R]
- [Environment_credentials](#environment_credentials) [R]
- [Project](#project) [CRUD]
- [Listing_change_set](#listing_change_set) [C]
- [Asset_filter](#asset_filter) [CRUD]

---

## Resources


### Group_profile

GroupProfile resource

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `domain_identifier` | String | ✅ | <p>The identifier of the Amazon DataZone domain in which the group profile is created.</p> |
| `group_identifier` | String | ✅ | <p>The identifier of the group for which the group profile is created.</p> |
| `client_token` | String |  | <p> A unique, case-sensitive identifier that is provided to ensure the idempotency of the request.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | String | <p>The identifier of the group profile.</p> |
| `domain_id` | String | <p>The identifier of the Amazon DataZone domain in which the group profile exists.</p> |
| `group_name` | String | <p>The name of the group for which the specified group profile exists.</p> |
| `id` | String | <p>The identifier of the group profile.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create group_profile
group_profile = provider.datazone.Group_profile {
    domain_identifier = "value"  # <p>The identifier of the Amazon DataZone domain in which the group profile is created.</p>
    group_identifier = "value"  # <p>The identifier of the group for which the group profile is created.</p>
}

# Access group_profile outputs
group_profile_id = group_profile.id
group_profile_status = group_profile.status
group_profile_domain_id = group_profile.domain_id
group_profile_group_name = group_profile.group_name
group_profile_id = group_profile.id
```

---


### Time_series_data_point

TimeSeriesDataPoint resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `entity_type` | String | <p>The type of the asset for which you want to get the data point.</p> |
| `form` | String | <p>The time series form that houses the data point that you want to get.</p> |
| `entity_id` | String | <p>The ID of the asset for which you want to get the data point.</p> |
| `form_name` | String | <p>The name of the time series form that houses the data point that you want to get.</p> |
| `domain_id` | String | <p>The ID of the Amazon DataZone domain that houses the asset data point that you want to get.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access time_series_data_point outputs
time_series_data_point_id = time_series_data_point.id
time_series_data_point_entity_type = time_series_data_point.entity_type
time_series_data_point_form = time_series_data_point.form
time_series_data_point_entity_id = time_series_data_point.entity_id
time_series_data_point_form_name = time_series_data_point.form_name
time_series_data_point_domain_id = time_series_data_point.domain_id
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
| `job_type` | String | <p>The type of the job run.</p> |
| `details` | String | <p>The details of the job run.</p> |
| `run_mode` | String | <p>The mode of the job run.</p> |
| `created_at` | String | <p>The timestamp of when the job run was created.</p> |
| `job_id` | String | <p>The ID of the job run.</p> |
| `error` | String | <p>The error generated if the action is not completed successfully.</p> |
| `created_by` | String | <p>The user who created the job run.</p> |
| `end_time` | String | <p>The timestamp of when the job run ended.</p> |
| `domain_id` | String | <p>The ID of the domain.</p> |
| `status` | String | <p>The status of the job run.</p> |
| `start_time` | String | <p>The timestamp of when the job run started.</p> |
| `id` | String | <p>The ID of the job run.</p> |


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
job_run_job_type = job_run.job_type
job_run_details = job_run.details
job_run_run_mode = job_run.run_mode
job_run_created_at = job_run.created_at
job_run_job_id = job_run.job_id
job_run_error = job_run.error
job_run_created_by = job_run.created_by
job_run_end_time = job_run.end_time
job_run_domain_id = job_run.domain_id
job_run_status = job_run.status
job_run_start_time = job_run.start_time
job_run_id = job_run.id
```

---


### Subscription_grant

SubscriptionGrant resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `environment_identifier` | String | ✅ | <p>The ID of the environment in which the subscription grant is created.</p> |
| `subscription_target_identifier` | String |  | <p>The ID of the subscription target for which the subscription grant is created.</p> |
| `granted_entity` | String | ✅ | <p>The entity to which the subscription is to be granted.</p> |
| `domain_identifier` | String | ✅ | <p>The ID of the Amazon DataZone domain in which the subscription grant is created.</p> |
| `client_token` | String |  | <p>A unique, case-sensitive identifier that is provided to ensure the idempotency of the request.</p> |
| `asset_target_names` | Vec<String> |  | <p>The names of the assets for which the subscription grant is created.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `created_by` | String | <p>The Amazon DataZone user who created the subscription grant.</p> |
| `updated_by` | String | <p>The Amazon DataZone user who updated the subscription grant.</p> |
| `granted_entity` | String | <p>The entity to which the subscription is granted.</p> |
| `status` | String | <p>The status of the subscription grant.</p> |
| `subscription_target_id` | String | <p>The subscription target ID associated with the subscription grant.</p> |
| `domain_id` | String | <p>The ID of the Amazon DataZone domain in which the subscription grant exists.</p> |
| `updated_at` | String | <p>The timestamp of when the subscription grant was upated.</p> |
| `assets` | Vec<String> | <p>The assets for which the subscription grant is created.</p> |
| `subscription_id` | String | <p>The identifier of the subscription.</p> |
| `id` | String | <p>The ID of the subscription grant.</p> |
| `created_at` | String | <p>The timestamp of when the subscription grant is created.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create subscription_grant
subscription_grant = provider.datazone.Subscription_grant {
    environment_identifier = "value"  # <p>The ID of the environment in which the subscription grant is created.</p>
    granted_entity = "value"  # <p>The entity to which the subscription is to be granted.</p>
    domain_identifier = "value"  # <p>The ID of the Amazon DataZone domain in which the subscription grant is created.</p>
}

# Access subscription_grant outputs
subscription_grant_id = subscription_grant.id
subscription_grant_created_by = subscription_grant.created_by
subscription_grant_updated_by = subscription_grant.updated_by
subscription_grant_granted_entity = subscription_grant.granted_entity
subscription_grant_status = subscription_grant.status
subscription_grant_subscription_target_id = subscription_grant.subscription_target_id
subscription_grant_domain_id = subscription_grant.domain_id
subscription_grant_updated_at = subscription_grant.updated_at
subscription_grant_assets = subscription_grant.assets
subscription_grant_subscription_id = subscription_grant.subscription_id
subscription_grant_id = subscription_grant.id
subscription_grant_created_at = subscription_grant.created_at
```

---


### Subscription_request_details

SubscriptionRequestDetails resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | String | <p>The status of the subscription request.</p> |
| `subscribed_principals` | Vec<String> | <p>The subscribed principals in the subscription request.</p> |
| `id` | String | <p>The identifier of the subscription request.</p> |
| `created_by` | String | <p>The Amazon DataZone user who created the subscription request.</p> |
| `updated_by` | String | <p>The Amazon DataZone user who updated the subscription request.</p> |
| `created_at` | String | <p>The timestamp of when the specified subscription request was created.</p> |
| `updated_at` | String | <p>The timestamp of when the subscription request was updated.</p> |
| `reviewer_id` | String | <p>The identifier of the Amazon DataZone user who reviewed the subscription request.</p> |
| `decision_comment` | String | <p>The decision comment of the subscription request.</p> |
| `domain_id` | String | <p>The Amazon DataZone domain of the subscription request.</p> |
| `subscribed_listings` | Vec<String> | <p>The subscribed listings in the subscription request.</p> |
| `metadata_forms` | Vec<String> | <p>The metadata forms included in the subscription request.</p> |
| `request_reason` | String | <p>The reason for the subscription request.</p> |
| `existing_subscription_id` | String | <p>The ID of the existing subscription.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access subscription_request_details outputs
subscription_request_details_id = subscription_request_details.id
subscription_request_details_status = subscription_request_details.status
subscription_request_details_subscribed_principals = subscription_request_details.subscribed_principals
subscription_request_details_id = subscription_request_details.id
subscription_request_details_created_by = subscription_request_details.created_by
subscription_request_details_updated_by = subscription_request_details.updated_by
subscription_request_details_created_at = subscription_request_details.created_at
subscription_request_details_updated_at = subscription_request_details.updated_at
subscription_request_details_reviewer_id = subscription_request_details.reviewer_id
subscription_request_details_decision_comment = subscription_request_details.decision_comment
subscription_request_details_domain_id = subscription_request_details.domain_id
subscription_request_details_subscribed_listings = subscription_request_details.subscribed_listings
subscription_request_details_metadata_forms = subscription_request_details.metadata_forms
subscription_request_details_request_reason = subscription_request_details.request_reason
subscription_request_details_existing_subscription_id = subscription_request_details.existing_subscription_id
```

---


### Time_series_data_points

TimeSeriesDataPoints resource

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


### Environment

Environment resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `project_identifier` | String | ✅ | <p>The identifier of the Amazon DataZone project in which this environment is created.</p> |
| `user_parameters` | Vec<String> |  | <p>The user parameters of this Amazon DataZone environment.</p> |
| `glossary_terms` | Vec<String> |  | <p>The glossary terms that can be used in this Amazon DataZone environment.</p> |
| `environment_account_region` | String |  | <p>The region of the account in which the environment is being created.</p> |
| `deployment_order` | i64 |  | <p>The deployment order of the environment.</p> |
| `environment_configuration_id` | String |  | <p>The configuration ID of the environment.</p> |
| `domain_identifier` | String | ✅ | <p>The identifier of the Amazon DataZone domain in which the environment is created.</p> |
| `name` | String | ✅ | <p>The name of the Amazon DataZone environment.</p> |
| `description` | String |  | <p>The description of the Amazon DataZone environment.</p> |
| `environment_blueprint_identifier` | String |  | <p>The ID of the blueprint with which the environment is being created.</p> |
| `environment_profile_identifier` | String |  | <p>The identifier of the environment profile that is used to create this Amazon DataZone environment.</p> |
| `environment_account_identifier` | String |  | <p>The ID of the account in which the environment is being created.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `updated_at` | String | <p>The timestamp of when this environment was updated.</p> |
| `environment_profile_id` | String | <p>The ID of the environment profile with which the environment is created.</p> |
| `aws_account_region` | String | <p>The Amazon Web Services region where the environment exists.</p> |
| `last_deployment` | String | <p>The details of the last deployment of the environment.</p> |
| `id` | String | <p>The ID of the environment.</p> |
| `deployment_properties` | String | <p>The deployment properties of the environment.</p> |
| `glossary_terms` | Vec<String> | <p>The business glossary terms that can be used in this environment.</p> |
| `environment_configuration_id` | String | <p>The configuration ID that is used to create the environment.</p> |
| `user_parameters` | Vec<String> | <p>The user parameters of this Amazon DataZone environment.</p> |
| `aws_account_id` | String | <p>The ID of the Amazon Web Services account where the environment exists.</p> |
| `project_id` | String | <p>The ID of the Amazon DataZone project in which this environment is created.</p> |
| `created_at` | String | <p>The timestamp of when the environment was created.</p> |
| `provisioning_properties` | String | <p>The provisioning properties of this Amazon DataZone environment.</p> |
| `name` | String | <p>The name of the environment.</p> |
| `description` | String | <p>The description of the environment.</p> |
| `environment_blueprint_id` | String | <p>The blueprint with which the environment is created.</p> |
| `status` | String | <p>The status of this Amazon DataZone environment.</p> |
| `environment_actions` | Vec<String> | <p>The actions of the environment.</p> |
| `created_by` | String | <p>The Amazon DataZone user who created the environment.</p> |
| `provider` | String | <p>The provider of this Amazon DataZone environment.</p> |
| `provisioned_resources` | Vec<String> | <p>The provisioned resources of this Amazon DataZone environment.</p> |
| `domain_id` | String | <p>The ID of the Amazon DataZone domain where the environment exists.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create environment
environment = provider.datazone.Environment {
    project_identifier = "value"  # <p>The identifier of the Amazon DataZone project in which this environment is created.</p>
    domain_identifier = "value"  # <p>The identifier of the Amazon DataZone domain in which the environment is created.</p>
    name = "value"  # <p>The name of the Amazon DataZone environment.</p>
}

# Access environment outputs
environment_id = environment.id
environment_updated_at = environment.updated_at
environment_environment_profile_id = environment.environment_profile_id
environment_aws_account_region = environment.aws_account_region
environment_last_deployment = environment.last_deployment
environment_id = environment.id
environment_deployment_properties = environment.deployment_properties
environment_glossary_terms = environment.glossary_terms
environment_environment_configuration_id = environment.environment_configuration_id
environment_user_parameters = environment.user_parameters
environment_aws_account_id = environment.aws_account_id
environment_project_id = environment.project_id
environment_created_at = environment.created_at
environment_provisioning_properties = environment.provisioning_properties
environment_name = environment.name
environment_description = environment.description
environment_environment_blueprint_id = environment.environment_blueprint_id
environment_status = environment.status
environment_environment_actions = environment.environment_actions
environment_created_by = environment.created_by
environment_provider = environment.provider
environment_provisioned_resources = environment.provisioned_resources
environment_domain_id = environment.domain_id
```

---


### Environment_profile

EnvironmentProfile resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>The name of this Amazon DataZone environment profile.</p> |
| `aws_account_id` | String |  | <p>The Amazon Web Services account in which the Amazon DataZone environment is created.</p> |
| `domain_identifier` | String | ✅ | <p>The ID of the Amazon DataZone domain in which this environment profile is created.</p> |
| `user_parameters` | Vec<String> |  | <p>The user parameters of this Amazon DataZone environment profile.</p> |
| `project_identifier` | String | ✅ | <p>The identifier of the project in which to create the environment profile.</p> |
| `environment_blueprint_identifier` | String | ✅ | <p>The ID of the blueprint with which this environment profile is created.</p> |
| `description` | String |  | <p>The description of this Amazon DataZone environment profile.</p> |
| `aws_account_region` | String |  | <p>The Amazon Web Services region in which this environment profile is created.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | <p>The name of the environment profile.</p> |
| `project_id` | String | <p>The ID of the Amazon DataZone project in which this environment profile is created.</p> |
| `id` | String | <p>The ID of the environment profile.</p> |
| `aws_account_id` | String | <p>The ID of the Amazon Web Services account where this environment profile exists.</p> |
| `user_parameters` | Vec<String> | <p>The user parameters of the environment profile.</p> |
| `updated_at` | String | <p>The timestamp of when this environment profile was upated.</p> |
| `description` | String | <p>The description of the environment profile.</p> |
| `environment_blueprint_id` | String | <p>The ID of the blueprint with which this environment profile is created.</p> |
| `created_by` | String | <p>The Amazon DataZone user who created this environment profile.</p> |
| `aws_account_region` | String | <p>The Amazon Web Services region where this environment profile exists.</p> |
| `domain_id` | String | <p>The ID of the Amazon DataZone domain in which this environment profile exists.</p> |
| `created_at` | String | <p>The timestamp of when this environment profile was created.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create environment_profile
environment_profile = provider.datazone.Environment_profile {
    name = "value"  # <p>The name of this Amazon DataZone environment profile.</p>
    domain_identifier = "value"  # <p>The ID of the Amazon DataZone domain in which this environment profile is created.</p>
    project_identifier = "value"  # <p>The identifier of the project in which to create the environment profile.</p>
    environment_blueprint_identifier = "value"  # <p>The ID of the blueprint with which this environment profile is created.</p>
}

# Access environment_profile outputs
environment_profile_id = environment_profile.id
environment_profile_name = environment_profile.name
environment_profile_project_id = environment_profile.project_id
environment_profile_id = environment_profile.id
environment_profile_aws_account_id = environment_profile.aws_account_id
environment_profile_user_parameters = environment_profile.user_parameters
environment_profile_updated_at = environment_profile.updated_at
environment_profile_description = environment_profile.description
environment_profile_environment_blueprint_id = environment_profile.environment_blueprint_id
environment_profile_created_by = environment_profile.created_by
environment_profile_aws_account_region = environment_profile.aws_account_region
environment_profile_domain_id = environment_profile.domain_id
environment_profile_created_at = environment_profile.created_at
```

---


### Environment_action

EnvironmentAction resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>The name of the environment action.</p> |
| `parameters` | String | ✅ | <p>The parameters of the environment action.</p> |
| `environment_identifier` | String | ✅ | <p>The ID of the environment in which the environment action is created.</p> |
| `description` | String |  | <p>The description of the environment action that is being created in the environment.</p> |
| `domain_identifier` | String | ✅ | <p>The ID of the Amazon DataZone domain in which the environment action is created.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `parameters` | String | <p>The parameters of the environment action.</p> |
| `domain_id` | String | <p>The ID of the Amazon DataZone domain in which the environment action lives.</p> |
| `name` | String | <p>The name of the environment action.</p> |
| `environment_id` | String | <p>The environment ID of the environment action.</p> |
| `description` | String | <p>The description of the environment action.</p> |
| `id` | String | <p>The ID of the environment action.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create environment_action
environment_action = provider.datazone.Environment_action {
    name = "value"  # <p>The name of the environment action.</p>
    parameters = "value"  # <p>The parameters of the environment action.</p>
    environment_identifier = "value"  # <p>The ID of the environment in which the environment action is created.</p>
    domain_identifier = "value"  # <p>The ID of the Amazon DataZone domain in which the environment action is created.</p>
}

# Access environment_action outputs
environment_action_id = environment_action.id
environment_action_parameters = environment_action.parameters
environment_action_domain_id = environment_action.domain_id
environment_action_name = environment_action.name
environment_action_environment_id = environment_action.environment_id
environment_action_description = environment_action.description
environment_action_id = environment_action.id
```

---


### Subscription_target

SubscriptionTarget resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `manage_access_role` | String | ✅ | <p>The manage access role that is used to create the subscription target.</p> |
| `type` | String | ✅ | <p>The type of the subscription target.</p> |
| `applicable_asset_types` | Vec<String> | ✅ | <p>The asset types that can be included in the subscription target.</p> |
| `environment_identifier` | String | ✅ | <p>The ID of the environment in which subscription target is created.</p> |
| `name` | String | ✅ | <p>The name of the subscription target.</p> |
| `domain_identifier` | String | ✅ | <p>The ID of the Amazon DataZone domain in which subscription target is created.</p> |
| `authorized_principals` | Vec<String> | ✅ | <p>The authorized principals of the subscription target.</p> |
| `provider` | String |  | <p>The provider of the subscription target.</p> |
| `subscription_target_config` | Vec<String> | ✅ | <p>The configuration of the subscription target.</p> |
| `client_token` | String |  | <p>A unique, case-sensitive identifier that is provided to ensure the idempotency of the request.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `project_id` | String | <p>The ID of the project associated with the subscription target.</p> |
| `domain_id` | String | <p>The ID of the Amazon DataZone domain in which the subscription target exists.</p> |
| `name` | String | <p>The name of the subscription target.</p> |
| `updated_at` | String | <p>The timestamp of when the subscription target was updated.</p> |
| `applicable_asset_types` | Vec<String> | <p>The asset types associated with the subscription target.</p> |
| `subscription_target_config` | Vec<String> | <p>The configuration of teh subscription target.</p> |
| `authorized_principals` | Vec<String> | <p>The authorized principals of the subscription target.</p> |
| `created_by` | String | <p>The Amazon DataZone user who created the subscription target.</p> |
| `manage_access_role` | String | <p>The manage access role with which the subscription target was created.</p> |
| `id` | String | <p>The ID of the subscription target.</p> |
| `environment_id` | String | <p>The ID of the environment associated with the subscription target.</p> |
| `updated_by` | String | <p>The Amazon DataZone user who updated the subscription target.</p> |
| `type` | String | <p>The type of the subscription target.</p> |
| `created_at` | String | <p>The timestamp of when the subscription target was created.</p> |
| `provider` | String | <p>The provider of the subscription target.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create subscription_target
subscription_target = provider.datazone.Subscription_target {
    manage_access_role = "value"  # <p>The manage access role that is used to create the subscription target.</p>
    type = "value"  # <p>The type of the subscription target.</p>
    applicable_asset_types = "value"  # <p>The asset types that can be included in the subscription target.</p>
    environment_identifier = "value"  # <p>The ID of the environment in which subscription target is created.</p>
    name = "value"  # <p>The name of the subscription target.</p>
    domain_identifier = "value"  # <p>The ID of the Amazon DataZone domain in which subscription target is created.</p>
    authorized_principals = "value"  # <p>The authorized principals of the subscription target.</p>
    subscription_target_config = "value"  # <p>The configuration of the subscription target.</p>
}

# Access subscription_target outputs
subscription_target_id = subscription_target.id
subscription_target_project_id = subscription_target.project_id
subscription_target_domain_id = subscription_target.domain_id
subscription_target_name = subscription_target.name
subscription_target_updated_at = subscription_target.updated_at
subscription_target_applicable_asset_types = subscription_target.applicable_asset_types
subscription_target_subscription_target_config = subscription_target.subscription_target_config
subscription_target_authorized_principals = subscription_target.authorized_principals
subscription_target_created_by = subscription_target.created_by
subscription_target_manage_access_role = subscription_target.manage_access_role
subscription_target_id = subscription_target.id
subscription_target_environment_id = subscription_target.environment_id
subscription_target_updated_by = subscription_target.updated_by
subscription_target_type = subscription_target.type
subscription_target_created_at = subscription_target.created_at
subscription_target_provider = subscription_target.provider
```

---


### Account_pool

AccountPool resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>The name of the account pool.</p> |
| `resolution_strategy` | String | ✅ | <p>The mechanism used to resolve the account selection from the account pool.</p> |
| `domain_identifier` | String | ✅ | <p>The ID of the domain where the account pool is created.</p> |
| `description` | String |  | <p>The description of the account pool.</p> |
| `account_source` | String | ✅ | <p>The source of accounts for the account pool. In the current release, it's either a static list of accounts provided by the customer or a custom Amazon Web Services Lambda handler. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `account_source` | String | <p>The source of accounts for the account pool. In the current release, it's either a static list of accounts provided by the customer or a custom Amazon Web Services Lambda handler. </p> |
| `updated_by` | String | <p>The user who last updated the account pool.</p> |
| `name` | String | <p>The name of the account pool.</p> |
| `domain_id` | String | <p>The ID of the domain in which the account pool lives whose details are to be displayed.</p> |
| `resolution_strategy` | String | <p>The mechanism used to resolve the account selection from the account pool.</p> |
| `id` | String | <p>The ID of the account pool.</p> |
| `created_at` | String | <p>The timestamp at which the account pool was created.</p> |
| `description` | String | <p>The description of the account pool.</p> |
| `created_by` | String | <p>The user who created the account pool.</p> |
| `last_updated_at` | String | <p>The timestamp at which the account pool was last updated.</p> |
| `domain_unit_id` | String | <p>The domain unit ID of the account pool.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create account_pool
account_pool = provider.datazone.Account_pool {
    name = "value"  # <p>The name of the account pool.</p>
    resolution_strategy = "value"  # <p>The mechanism used to resolve the account selection from the account pool.</p>
    domain_identifier = "value"  # <p>The ID of the domain where the account pool is created.</p>
    account_source = "value"  # <p>The source of accounts for the account pool. In the current release, it's either a static list of accounts provided by the customer or a custom Amazon Web Services Lambda handler. </p>
}

# Access account_pool outputs
account_pool_id = account_pool.id
account_pool_account_source = account_pool.account_source
account_pool_updated_by = account_pool.updated_by
account_pool_name = account_pool.name
account_pool_domain_id = account_pool.domain_id
account_pool_resolution_strategy = account_pool.resolution_strategy
account_pool_id = account_pool.id
account_pool_created_at = account_pool.created_at
account_pool_description = account_pool.description
account_pool_created_by = account_pool.created_by
account_pool_last_updated_at = account_pool.last_updated_at
account_pool_domain_unit_id = account_pool.domain_unit_id
```

---


### Project_membership

ProjectMembership resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `domain_identifier` | String | ✅ | <p>The ID of the Amazon DataZone domain in which project membership is created.</p> |
| `designation` | String | ✅ | <p>The designation of the project membership.</p> |
| `member` | String | ✅ | <p>The project member whose project membership was created.</p> |
| `project_identifier` | String | ✅ | <p>The ID of the project for which this project membership was created.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create project_membership
project_membership = provider.datazone.Project_membership {
    domain_identifier = "value"  # <p>The ID of the Amazon DataZone domain in which project membership is created.</p>
    designation = "value"  # <p>The designation of the project membership.</p>
    member = "value"  # <p>The project member whose project membership was created.</p>
    project_identifier = "value"  # <p>The ID of the project for which this project membership was created.</p>
}

```

---


### Environment_blueprint

EnvironmentBlueprint resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>The name of this Amazon DataZone blueprint.</p> |
| `domain_identifier` | String | ✅ | <p>The identifier of the domain in which this blueprint is created.</p> |
| `provisioning_properties` | String | ✅ | <p>The provisioning properties of this Amazon DataZone blueprint.</p> |
| `description` | String |  | <p>The description of the Amazon DataZone blueprint.</p> |
| `user_parameters` | Vec<String> |  | <p>The user parameters of this Amazon DataZone blueprint.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `updated_at` | String | <p>The timestamp of when this blueprint was updated.</p> |
| `description` | String | <p>The description of this Amazon DataZone blueprint.</p> |
| `provisioning_properties` | String | <p>The provisioning properties of this Amazon DataZone blueprint.</p> |
| `deployment_properties` | String | <p>The deployment properties of this Amazon DataZone blueprint.</p> |
| `user_parameters` | Vec<String> | <p>The user parameters of this blueprint.</p> |
| `provider` | String | <p>The provider of this Amazon DataZone blueprint.</p> |
| `id` | String | <p>The ID of this Amazon DataZone blueprint.</p> |
| `name` | String | <p>The name of this Amazon DataZone blueprint.</p> |
| `glossary_terms` | Vec<String> | <p>The glossary terms attached to this Amazon DataZone blueprint.</p> |
| `created_at` | String | <p>A timestamp of when this blueprint was created.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create environment_blueprint
environment_blueprint = provider.datazone.Environment_blueprint {
    name = "value"  # <p>The name of this Amazon DataZone blueprint.</p>
    domain_identifier = "value"  # <p>The identifier of the domain in which this blueprint is created.</p>
    provisioning_properties = "value"  # <p>The provisioning properties of this Amazon DataZone blueprint.</p>
}

# Access environment_blueprint outputs
environment_blueprint_id = environment_blueprint.id
environment_blueprint_updated_at = environment_blueprint.updated_at
environment_blueprint_description = environment_blueprint.description
environment_blueprint_provisioning_properties = environment_blueprint.provisioning_properties
environment_blueprint_deployment_properties = environment_blueprint.deployment_properties
environment_blueprint_user_parameters = environment_blueprint.user_parameters
environment_blueprint_provider = environment_blueprint.provider
environment_blueprint_id = environment_blueprint.id
environment_blueprint_name = environment_blueprint.name
environment_blueprint_glossary_terms = environment_blueprint.glossary_terms
environment_blueprint_created_at = environment_blueprint.created_at
```

---


### Project_profile

ProjectProfile resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>Project profile name.</p> |
| `domain_identifier` | String | ✅ | <p>A domain ID of the project profile.</p> |
| `description` | String |  | <p>A description of a project profile.</p> |
| `environment_configurations` | Vec<String> |  | <p>Environment configurations of the project profile.</p> |
| `domain_unit_identifier` | String |  | <p>A domain unit ID of the project profile.</p> |
| `status` | String |  | <p>Project profile status.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | <p>The name of the project profile.</p> |
| `description` | String | <p>The description of the project profile.</p> |
| `last_updated_at` | String | <p>The timestamp of when project profile was last updated.</p> |
| `domain_id` | String | <p>The ID of the domain of the project profile.</p> |
| `environment_configurations` | Vec<String> | <p>The environment configurations of the project profile.</p> |
| `created_by` | String | <p>The user who created the project profile.</p> |
| `status` | String | <p>The status of the project profile.</p> |
| `created_at` | String | <p>The timestamp of when the project profile was created.</p> |
| `domain_unit_id` | String | <p>The ID of the domain unit of the project profile.</p> |
| `id` | String | <p>The ID of the project profile.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create project_profile
project_profile = provider.datazone.Project_profile {
    name = "value"  # <p>Project profile name.</p>
    domain_identifier = "value"  # <p>A domain ID of the project profile.</p>
}

# Access project_profile outputs
project_profile_id = project_profile.id
project_profile_name = project_profile.name
project_profile_description = project_profile.description
project_profile_last_updated_at = project_profile.last_updated_at
project_profile_domain_id = project_profile.domain_id
project_profile_environment_configurations = project_profile.environment_configurations
project_profile_created_by = project_profile.created_by
project_profile_status = project_profile.status
project_profile_created_at = project_profile.created_at
project_profile_domain_unit_id = project_profile.domain_unit_id
project_profile_id = project_profile.id
```

---


### Connection

Connection resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `domain_identifier` | String | ✅ | <p>The ID of the domain where the connection is created.</p> |
| `props` | String |  | <p>The connection props.</p> |
| `aws_location` | String |  | <p>The location where the connection is created.</p> |
| `description` | String |  | <p>A connection description.</p> |
| `name` | String | ✅ | <p>The connection name.</p> |
| `enable_trusted_identity_propagation` | bool |  | <p>Specifies whether the trusted identity propagation is enabled.</p> |
| `scope` | String |  | <p>The scope of the connection.</p> |
| `environment_identifier` | String |  | <p>The ID of the environment where the connection is created.</p> |
| `client_token` | String |  | <p>A unique, case-sensitive identifier that is provided to ensure the idempotency of the request.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `physical_endpoints` | Vec<String> | <p>The physical endpoints of the connection.</p> |
| `scope` | String | <p>The scope of the connection.</p> |
| `description` | String | <p>Connection description.</p> |
| `domain_unit_id` | String | <p>The domain unit ID of the connection.</p> |
| `environment_id` | String | <p>The ID of the environment.</p> |
| `project_id` | String | <p>The ID of the project.</p> |
| `name` | String | <p>The name of the connection.</p> |
| `domain_id` | String | <p>The domain ID of the connection.</p> |
| `environment_user_role` | String | <p>The environment user role.</p> |
| `props` | String | <p>Connection props.</p> |
| `connection_credentials` | String | <p>Connection credentials.</p> |
| `connection_id` | String | <p>The ID of the connection.</p> |
| `type` | String | <p>The type of the connection.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create connection
connection = provider.datazone.Connection {
    domain_identifier = "value"  # <p>The ID of the domain where the connection is created.</p>
    name = "value"  # <p>The connection name.</p>
}

# Access connection outputs
connection_id = connection.id
connection_physical_endpoints = connection.physical_endpoints
connection_scope = connection.scope
connection_description = connection.description
connection_domain_unit_id = connection.domain_unit_id
connection_environment_id = connection.environment_id
connection_project_id = connection.project_id
connection_name = connection.name
connection_domain_id = connection.domain_id
connection_environment_user_role = connection.environment_user_role
connection_props = connection.props
connection_connection_credentials = connection.connection_credentials
connection_connection_id = connection.connection_id
connection_type = connection.type
```

---


### User_profile

UserProfile resource

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `user_type` | String |  | <p>The user type of the user for which the user profile is created.</p> |
| `client_token` | String |  | <p>A unique, case-sensitive identifier that is provided to ensure the idempotency of the request.</p> |
| `user_identifier` | String | ✅ | <p>The identifier of the user for which the user profile is created.</p> |
| `domain_identifier` | String | ✅ | <p>The identifier of the Amazon DataZone domain in which a user profile is created.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | String | <p>The status of the user profile.</p> |
| `domain_id` | String | <p>the identifier of the Amazon DataZone domain of which you want to get the user profile.</p> |
| `type` | String | <p>The type of the user profile.</p> |
| `id` | String | <p>The identifier of the user profile.</p> |
| `details` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create user_profile
user_profile = provider.datazone.User_profile {
    user_identifier = "value"  # <p>The identifier of the user for which the user profile is created.</p>
    domain_identifier = "value"  # <p>The identifier of the Amazon DataZone domain in which a user profile is created.</p>
}

# Access user_profile outputs
user_profile_id = user_profile.id
user_profile_status = user_profile.status
user_profile_domain_id = user_profile.domain_id
user_profile_type = user_profile.type
user_profile_id = user_profile.id
user_profile_details = user_profile.details
```

---


### Lineage_node

LineageNode resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `domain_id` | String | <p>The ID of the domain where you're getting the data lineage node.</p> |
| `description` | String | <p>The description of the data lineage node.</p> |
| `updated_by` | String | <p>The user who updated the data lineage node.</p> |
| `type_name` | String | <p>The name of the type of the specified data lineage node.</p> |
| `type_revision` | String | <p>The revision type of the specified data lineage node.</p> |
| `source_identifier` | String | <p>The source identifier of the data lineage node.</p> |
| `id` | String | <p>The ID of the data lineage node.</p> |
| `created_at` | String | <p>The timestamp at which the data lineage node was created.</p> |
| `created_by` | String | <p>The user who created the data lineage node.</p> |
| `updated_at` | String | <p>The timestamp at which the data lineage node was updated.</p> |
| `forms_output` | Vec<String> | <p>The metadata of the specified data lineage node.</p> |
| `event_timestamp` | String | <p>The timestamp of the event described in the data lineage node.</p> |
| `name` | String | <p>The name of the data lineage node.</p> |
| `upstream_nodes` | Vec<String> | <p>The upstream nodes of the specified data lineage node.</p> |
| `downstream_nodes` | Vec<String> | <p>The downsteam nodes of the specified data lineage node.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access lineage_node outputs
lineage_node_id = lineage_node.id
lineage_node_domain_id = lineage_node.domain_id
lineage_node_description = lineage_node.description
lineage_node_updated_by = lineage_node.updated_by
lineage_node_type_name = lineage_node.type_name
lineage_node_type_revision = lineage_node.type_revision
lineage_node_source_identifier = lineage_node.source_identifier
lineage_node_id = lineage_node.id
lineage_node_created_at = lineage_node.created_at
lineage_node_created_by = lineage_node.created_by
lineage_node_updated_at = lineage_node.updated_at
lineage_node_forms_output = lineage_node.forms_output
lineage_node_event_timestamp = lineage_node.event_timestamp
lineage_node_name = lineage_node.name
lineage_node_upstream_nodes = lineage_node.upstream_nodes
lineage_node_downstream_nodes = lineage_node.downstream_nodes
```

---


### Subscription

Subscription resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `retain_permissions` | bool | <p>The retain permissions of the subscription.</p> |
| `updated_by` | String | <p>The Amazon DataZone user who updated the subscription.</p> |
| `status` | String | <p>The status of the subscription.</p> |
| `created_at` | String | <p>The timestamp of when the subscription was created.</p> |
| `updated_at` | String | <p>The timestamp of when the subscription was updated.</p> |
| `id` | String | <p>The ID of the subscription.</p> |
| `created_by` | String | <p>The Amazon DataZone user who created the subscription.</p> |
| `domain_id` | String | <p>The ID of the Amazon DataZone domain in which the subscription exists.</p> |
| `subscribed_principal` | String | <p>The principal that owns the subscription.</p> |
| `subscribed_listing` | String | <p>The details of the published asset for which the subscription grant is created.</p> |
| `subscription_request_id` | String | <p>The ID of the subscription request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access subscription outputs
subscription_id = subscription.id
subscription_retain_permissions = subscription.retain_permissions
subscription_updated_by = subscription.updated_by
subscription_status = subscription.status
subscription_created_at = subscription.created_at
subscription_updated_at = subscription.updated_at
subscription_id = subscription.id
subscription_created_by = subscription.created_by
subscription_domain_id = subscription.domain_id
subscription_subscribed_principal = subscription.subscribed_principal
subscription_subscribed_listing = subscription.subscribed_listing
subscription_subscription_request_id = subscription.subscription_request_id
```

---


### Subscription_request

SubscriptionRequest resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_token` | String |  | <p>A unique, case-sensitive identifier that is provided to ensure the idempotency of the request.</p> |
| `domain_identifier` | String | ✅ | <p>The ID of the Amazon DataZone domain in which the subscription request is created.</p> |
| `subscribed_principals` | Vec<String> | ✅ | <p>The Amazon DataZone principals for whom the subscription request is created.</p> |
| `metadata_forms` | Vec<String> |  | <p>The metadata form included in the subscription request.</p> |
| `subscribed_listings` | Vec<String> | ✅ | <p>The published asset for which the subscription grant is to be created.</p> |
| `request_reason` | String | ✅ | <p>The reason for the subscription request.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create subscription_request
subscription_request = provider.datazone.Subscription_request {
    domain_identifier = "value"  # <p>The ID of the Amazon DataZone domain in which the subscription request is created.</p>
    subscribed_principals = "value"  # <p>The Amazon DataZone principals for whom the subscription request is created.</p>
    subscribed_listings = "value"  # <p>The published asset for which the subscription grant is to be created.</p>
    request_reason = "value"  # <p>The reason for the subscription request.</p>
}

```

---


### Subscription_grant_status

SubscriptionGrantStatus resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `domain_identifier` | String | ✅ | <p>The identifier of the Amazon DataZone domain in which a subscription grant status is to be updated.</p> |
| `target_name` | String |  | <p>The target name to be updated as part of the <code>UpdateSubscriptionGrantStatus</code> action.</p> |
| `status` | String | ✅ | <p>The status to be updated as part of the <code>UpdateSubscriptionGrantStatus</code> action.</p> |
| `asset_identifier` | String | ✅ | <p>The identifier of the asset the subscription grant status of which is to be updated.</p> |
| `failure_cause` | String |  | <p>Specifies the error message that is returned if the operation cannot be successfully completed.</p> |
| `identifier` | String | ✅ | <p>The identifier of the subscription grant the status of which is to be updated.</p> |



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


### Iam_portal_login_url

IamPortalLoginUrl resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `auth_code_url` | String | <p>The data portal URL of the specified Amazon DataZone domain.</p> |
| `user_profile_id` | String | <p>The ID of the user profile.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access iam_portal_login_url outputs
iam_portal_login_url_id = iam_portal_login_url.id
iam_portal_login_url_auth_code_url = iam_portal_login_url.auth_code_url
iam_portal_login_url_user_profile_id = iam_portal_login_url.user_profile_id
```

---


### Lineage_event

LineageEvent resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `processing_status` | String | <p>The progressing status of the lineage event.</p> |
| `created_at` | String | <p>The timestamp of when the lineage event was created.</p> |
| `event_time` | String | <p>The time of the lineage event.</p> |
| `created_by` | String | <p>The user who created the lineage event.</p> |
| `id` | String | <p>The ID of the lineage event.</p> |
| `domain_id` | String | <p>The ID of the domain.</p> |
| `event` | String | <p>The lineage event details.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access lineage_event outputs
lineage_event_id = lineage_event.id
lineage_event_processing_status = lineage_event.processing_status
lineage_event_created_at = lineage_event.created_at
lineage_event_event_time = lineage_event.event_time
lineage_event_created_by = lineage_event.created_by
lineage_event_id = lineage_event.id
lineage_event_domain_id = lineage_event.domain_id
lineage_event_event = lineage_event.event
```

---


### Environment_credentials

EnvironmentCredentials resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `access_key_id` | String | <p>The access key ID of the environment.</p> |
| `session_token` | String | <p>The session token of the environment credentials.</p> |
| `secret_access_key` | String | <p>The secret access key of the environment credentials.</p> |
| `expiration` | String | <p>The expiration timestamp of the environment credentials.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access environment_credentials outputs
environment_credentials_id = environment_credentials.id
environment_credentials_access_key_id = environment_credentials.access_key_id
environment_credentials_session_token = environment_credentials.session_token
environment_credentials_secret_access_key = environment_credentials.secret_access_key
environment_credentials_expiration = environment_credentials.expiration
```

---


### Project

Project resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | <p>The description of the Amazon DataZone project.</p> |
| `glossary_terms` | Vec<String> |  | <p>The glossary terms that can be used in this Amazon DataZone project.</p> |
| `user_parameters` | Vec<String> |  | <p>The user parameters of the project.</p> |
| `domain_identifier` | String | ✅ | <p>The ID of the Amazon DataZone domain in which this project is created.</p> |
| `domain_unit_id` | String |  | <p>The ID of the domain unit. This parameter is not required and if it is not specified, then the project is created at the root domain unit level.</p> |
| `project_profile_id` | String |  | <p>The ID of the project profile.</p> |
| `name` | String | ✅ | <p>The name of the Amazon DataZone project.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `domain_id` | String | <p>The ID of the Amazon DataZone domain in which the project exists.</p> |
| `glossary_terms` | Vec<String> | <p>The business glossary terms that can be used in the project.</p> |
| `domain_unit_id` | String | <p>The ID of the domain unit.</p> |
| `id` | String | <p>&gt;The ID of the project.</p> |
| `project_profile_id` | String | <p>The ID of the project profile of a project.</p> |
| `user_parameters` | Vec<String> | <p>The user parameters of a project.</p> |
| `name` | String | <p>The name of the project.</p> |
| `environment_deployment_details` | String | <p>The environment deployment status of a project.</p> |
| `project_status` | String | <p>The status of the project.</p> |
| `failure_reasons` | Vec<String> | <p>Specifies the error message that is returned if the operation cannot be successfully completed.</p> |
| `created_at` | String | <p>The timestamp of when the project was created.</p> |
| `created_by` | String | <p>The Amazon DataZone user who created the project.</p> |
| `description` | String | <p>The description of the project.</p> |
| `last_updated_at` | String | <p>The timestamp of when the project was last updated.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create project
project = provider.datazone.Project {
    domain_identifier = "value"  # <p>The ID of the Amazon DataZone domain in which this project is created.</p>
    name = "value"  # <p>The name of the Amazon DataZone project.</p>
}

# Access project outputs
project_id = project.id
project_domain_id = project.domain_id
project_glossary_terms = project.glossary_terms
project_domain_unit_id = project.domain_unit_id
project_id = project.id
project_project_profile_id = project.project_profile_id
project_user_parameters = project.user_parameters
project_name = project.name
project_environment_deployment_details = project.environment_deployment_details
project_project_status = project.project_status
project_failure_reasons = project.failure_reasons
project_created_at = project.created_at
project_created_by = project.created_by
project_description = project.description
project_last_updated_at = project.last_updated_at
```

---


### Listing_change_set

ListingChangeSet resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `domain_identifier` | String | ✅ | <p>The ID of the Amazon DataZone domain.</p> |
| `entity_revision` | String |  | <p>The revision of an asset.</p> |
| `action` | String | ✅ | <p>Specifies whether to publish or unpublish a listing.</p> |
| `entity_type` | String | ✅ | <p>The type of an entity.</p> |
| `entity_identifier` | String | ✅ | <p>The ID of the asset.</p> |
| `client_token` | String |  | <p>A unique, case-sensitive identifier that is provided to ensure the idempotency of the request.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create listing_change_set
listing_change_set = provider.datazone.Listing_change_set {
    domain_identifier = "value"  # <p>The ID of the Amazon DataZone domain.</p>
    action = "value"  # <p>Specifies whether to publish or unpublish a listing.</p>
    entity_type = "value"  # <p>The type of an entity.</p>
    entity_identifier = "value"  # <p>The ID of the asset.</p>
}

```

---


### Asset_filter

AssetFilter resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | <p>The description of the asset filter.</p> |
| `asset_identifier` | String | ✅ | <p>The ID of the data asset.</p> |
| `configuration` | String | ✅ | <p>The configuration of the asset filter.</p> |
| `client_token` | String |  | <p>A unique, case-sensitive identifier that is provided to ensure the idempotency of the request.</p> |
| `domain_identifier` | String | ✅ | <p>The ID of the domain in which you want to create an asset filter.</p> |
| `name` | String | ✅ | <p>The name of the asset filter.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `error_message` | String | <p>The error message that is displayed if the action does not complete successfully.</p> |
| `domain_id` | String | <p>The ID of the domain where you want to get an asset filter.</p> |
| `effective_column_names` | Vec<String> | <p>The column names of the asset filter.</p> |
| `id` | String | <p>The ID of the asset filter.</p> |
| `status` | String | <p>The status of the asset filter.</p> |
| `created_at` | String | <p>The timestamp at which the asset filter was created.</p> |
| `effective_row_filter` | String | <p>The row filter of the asset filter.</p> |
| `name` | String | <p>The name of the asset filter.</p> |
| `asset_id` | String | <p>The ID of the data asset.</p> |
| `configuration` | String | <p>The configuration of the asset filter.</p> |
| `description` | String | <p>The description of the asset filter.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create asset_filter
asset_filter = provider.datazone.Asset_filter {
    asset_identifier = "value"  # <p>The ID of the data asset.</p>
    configuration = "value"  # <p>The configuration of the asset filter.</p>
    domain_identifier = "value"  # <p>The ID of the domain in which you want to create an asset filter.</p>
    name = "value"  # <p>The name of the asset filter.</p>
}

# Access asset_filter outputs
asset_filter_id = asset_filter.id
asset_filter_error_message = asset_filter.error_message
asset_filter_domain_id = asset_filter.domain_id
asset_filter_effective_column_names = asset_filter.effective_column_names
asset_filter_id = asset_filter.id
asset_filter_status = asset_filter.status
asset_filter_created_at = asset_filter.created_at
asset_filter_effective_row_filter = asset_filter.effective_row_filter
asset_filter_name = asset_filter.name
asset_filter_asset_id = asset_filter.asset_id
asset_filter_configuration = asset_filter.configuration
asset_filter_description = asset_filter.description
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple group_profile resources
group_profile_0 = provider.datazone.Group_profile {
    domain_identifier = "value-0"
    group_identifier = "value-0"
}
group_profile_1 = provider.datazone.Group_profile {
    domain_identifier = "value-1"
    group_identifier = "value-1"
}
group_profile_2 = provider.datazone.Group_profile {
    domain_identifier = "value-2"
    group_identifier = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    group_profile = provider.datazone.Group_profile {
        domain_identifier = "production-value"
        group_identifier = "production-value"
    }
```

---

## Related Documentation

- [AWS Datazone Documentation](https://docs.aws.amazon.com/datazone/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
