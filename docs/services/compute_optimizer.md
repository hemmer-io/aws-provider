# Compute_optimizer Service



**Resources**: 17

---

## Overview

The compute_optimizer service provides access to 17 resource types:

- [Recommendation_preferences](#recommendation_preferences) [CRD]
- [Ecs_service_recommendation_projected_metrics](#ecs_service_recommendation_projected_metrics) [R]
- [Enrollment_status](#enrollment_status) [RU]
- [License_recommendations](#license_recommendations) [R]
- [Lambda_function_recommendations](#lambda_function_recommendations) [R]
- [Effective_recommendation_preferences](#effective_recommendation_preferences) [R]
- [Ec2_recommendation_projected_metrics](#ec2_recommendation_projected_metrics) [R]
- [Idle_recommendations](#idle_recommendations) [R]
- [Rds_database_recommendation_projected_metrics](#rds_database_recommendation_projected_metrics) [R]
- [Rds_database_recommendations](#rds_database_recommendations) [R]
- [Enrollment_statuses_for_organization](#enrollment_statuses_for_organization) [R]
- [Auto_scaling_group_recommendations](#auto_scaling_group_recommendations) [R]
- [Ec2_instance_recommendations](#ec2_instance_recommendations) [R]
- [Ebs_volume_recommendations](#ebs_volume_recommendations) [R]
- [Recommendation_export_jobs](#recommendation_export_jobs) [R]
- [Ecs_service_recommendations](#ecs_service_recommendations) [R]
- [Recommendation_summaries](#recommendation_summaries) [R]

---

## Resources


### Recommendation_preferences

RecommendationPreferences resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `external_metrics_preference` | String |  | <p>The provider of the external metrics recommendation preference to create or
            update.</p>
         <p>Specify a valid provider in the <code>source</code> field to activate the preference.
            To delete this preference, see the <a>DeleteRecommendationPreferences</a>
            action.</p>
         <p>This preference can only be set for the <code>Ec2Instance</code> resource type.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/compute-optimizer/latest/ug/external-metrics-ingestion.html">External metrics
                ingestion</a> in the <i>Compute Optimizer User
            Guide</i>.</p> |
| `utilization_preferences` | Vec<String> |  | <p>
            The preference to control the resource’s CPU utilization threshold, CPU utilization headroom, and memory utilization headroom. When this 
            preference isn't specified, we use the following default values.
        </p>
         <p>CPU utilization:</p>
         <ul>
            <li>
               <p>
                  <code>P99_5</code> for threshold</p>
            </li>
            <li>
               <p>
                  <code>PERCENT_20</code> for headroom</p>
            </li>
         </ul>
         <p>Memory utilization:</p>
         <ul>
            <li>
               <p>
                  <code>PERCENT_20</code> for headroom</p>
            </li>
         </ul>
         <note>
            <ul>
               <li>
                  <p>You can only set CPU and memory utilization preferences for the Amazon EC2 instance resource type.</p>
               </li>
               <li>
                  <p>The threshold setting isn’t available for memory utilization.</p>
               </li>
            </ul>
         </note> |
| `inferred_workload_types` | String |  | <p>The status of the inferred workload types recommendation preference to create or
            update.</p>
         <note>
            <p>The inferred workload type feature is active by default. To deactivate it, create
                a recommendation preference.</p>
         </note>
         <p>Specify the <code>Inactive</code> status to deactivate the feature, or specify
                <code>Active</code> to activate it.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/compute-optimizer/latest/ug/inferred-workload-types.html">Inferred workload
                types</a> in the <i>Compute Optimizer User Guide</i>.</p> |
| `enhanced_infrastructure_metrics` | String |  | <p>The status of the enhanced infrastructure metrics recommendation preference to create
            or update.</p>
         <p>Specify the <code>Active</code> status to activate the preference, or specify
                <code>Inactive</code> to deactivate the preference.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/compute-optimizer/latest/ug/enhanced-infrastructure-metrics.html">Enhanced
                infrastructure metrics</a> in the <i>Compute Optimizer User
                Guide</i>.</p> |
| `preferred_resources` | Vec<String> |  | <p>
            The preference to control which resource type values are considered when generating rightsizing recommendations. 
            You can specify this preference as a combination of include and exclude lists. You must specify either an 
            <code>includeList</code> or <code>excludeList</code>. If the preference is an empty set of resource type values, 
            an error occurs.
        </p>
         <note>
            <p>You can only set this preference for the Amazon EC2 instance and Auto Scaling group resource types.</p>
         </note> |
| `savings_estimation_mode` | String |  | <p>
            The status of the savings estimation mode preference to create or update.
        </p>
         <p>Specify the <code>AfterDiscounts</code> status to activate the preference, or specify <code>BeforeDiscounts</code> to deactivate the preference.</p>
         <p>Only the account manager or delegated administrator of your organization can activate this preference.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/compute-optimizer/latest/ug/savings-estimation-mode.html">
            Savings estimation mode</a> in the <i>Compute Optimizer User Guide</i>.</p> |
| `scope` | String |  | <p>An object that describes the scope of the recommendation preference to create.</p>
         <p>You can create recommendation preferences at the organization level (for management
            accounts of an organization only), account level, and resource level. For more
            information, see <a href="https://docs.aws.amazon.com/compute-optimizer/latest/ug/enhanced-infrastructure-metrics.html">Activating
                enhanced infrastructure metrics</a> in the <i>Compute Optimizer User
                Guide</i>.</p>
         <note>
            <p>You cannot create recommendation preferences for Auto Scaling groups at the
                organization and account levels. You can create recommendation preferences for
                    Auto Scaling groups only at the resource level by specifying a scope name
                of <code>ResourceArn</code> and a scope value of the Auto Scaling group Amazon
                Resource Name (ARN). This will configure the preference for all instances that are
                part of the specified Auto Scaling group. You also cannot create recommendation
                preferences at the resource level for instances that are part of an Auto Scaling group. You can create recommendation preferences at the resource level only for
                standalone instances.</p>
         </note> |
| `look_back_period` | String |  | <p>
            The preference to control the number of days the utilization metrics of the Amazon Web Services resource are analyzed. 
            When this preference isn't specified, we use the default value <code>DAYS_14</code>.
        </p>
         <p>You can only set this preference for the Amazon EC2 instance and Auto Scaling group resource types.
            </p>
         <note>
            <ul>
               <li>
                  <p>Amazon EC2 instance lookback preferences can be set at the organization, account, and resource levels.</p>
               </li>
               <li>
                  <p>Auto Scaling group lookback preferences can only be set at the resource level.</p>
               </li>
            </ul>
         </note> |
| `resource_type` | String | ✅ | <p>The target resource type of the recommendation preference to create.</p>
         <p>The <code>Ec2Instance</code> option encompasses standalone instances and instances
            that are part of Auto Scaling groups. The <code>AutoScalingGroup</code> option
            encompasses only instances that are part of an Auto Scaling group.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to use to advance to the next page of recommendation preferences.</p>
         <p>This value is null when there are no more pages of recommendation preferences to
            return.</p> |
| `recommendation_preferences_details` | Vec<String> | <p>An array of objects that describe recommendation preferences.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create recommendation_preferences
recommendation_preferences = provider.compute_optimizer.Recommendation_preferences {
    resource_type = "value"  # <p>The target resource type of the recommendation preference to create.</p>
         <p>The <code>Ec2Instance</code> option encompasses standalone instances and instances
            that are part of Auto Scaling groups. The <code>AutoScalingGroup</code> option
            encompasses only instances that are part of an Auto Scaling group.</p>
}

# Access recommendation_preferences outputs
recommendation_preferences_id = recommendation_preferences.id
recommendation_preferences_next_token = recommendation_preferences.next_token
recommendation_preferences_recommendation_preferences_details = recommendation_preferences.recommendation_preferences_details
```

---


### Ecs_service_recommendation_projected_metrics

ECSServiceRecommendationProjectedMetrics resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `recommended_option_projected_metrics` | Vec<String> | <p>
            An array of objects that describes the projected metrics.
        </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access ecs_service_recommendation_projected_metrics outputs
ecs_service_recommendation_projected_metrics_id = ecs_service_recommendation_projected_metrics.id
ecs_service_recommendation_projected_metrics_recommended_option_projected_metrics = ecs_service_recommendation_projected_metrics.recommended_option_projected_metrics
```

---


### Enrollment_status

EnrollmentStatus resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `include_member_accounts` | bool |  | <p>Indicates whether to enroll member accounts of the organization if the account is the
            management account of an organization.</p> |
| `status` | String | ✅ | <p>The new enrollment status of the account.</p>
         <p>The following status options are available:</p>
         <ul>
            <li>
               <p>
                  <code>Active</code> - Opts in your account to the Compute Optimizer service.
                        Compute Optimizer begins analyzing the configuration and utilization metrics
                    of your Amazon Web Services resources after you opt in. For more information, see
                        <a href="https://docs.aws.amazon.com/compute-optimizer/latest/ug/metrics.html">Metrics analyzed by Compute Optimizer</a> in the <i>Compute Optimizer User Guide</i>.</p>
            </li>
            <li>
               <p>
                  <code>Inactive</code> - Opts out your account from the Compute Optimizer
                    service. Your account's recommendations and related metrics data will be deleted
                    from Compute Optimizer after you opt out.</p>
            </li>
         </ul>
         <note>
            <p>The <code>Pending</code> and <code>Failed</code> options cannot be used to update
                the enrollment status of an account. They are returned in the response of a request
                to update the enrollment status of an account.</p>
         </note> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status_reason` | String | <p>The reason for the enrollment status of the account.</p>
         <p>For example, an account might show a status of <code>Pending</code> because member
            accounts of an organization require more time to be enrolled in the service.</p> |
| `status` | String | <p>The enrollment status of the account.</p> |
| `member_accounts_enrolled` | bool | <p>Confirms the enrollment status of member accounts of the organization, if the account
            is a management account of an organization.</p> |
| `last_updated_timestamp` | String | <p>The Unix epoch timestamp, in seconds, of when the account enrollment status was last
            updated.</p> |
| `number_of_member_accounts_opted_in` | i64 | <p>The count of organization member accounts that are opted in to the service, if your
            account is an organization management account.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access enrollment_status outputs
enrollment_status_id = enrollment_status.id
enrollment_status_status_reason = enrollment_status.status_reason
enrollment_status_status = enrollment_status.status
enrollment_status_member_accounts_enrolled = enrollment_status.member_accounts_enrolled
enrollment_status_last_updated_timestamp = enrollment_status.last_updated_timestamp
enrollment_status_number_of_member_accounts_opted_in = enrollment_status.number_of_member_accounts_opted_in
```

---


### License_recommendations

LicenseRecommendations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `license_recommendations` | Vec<String> | <p>
            An array of objects that describe license recommendations.
        </p> |
| `errors` | Vec<String> | <p>
            An array of objects that describe errors of the request.
        </p> |
| `next_token` | String | <p>
            The token to use to advance to the next page of license recommendations.
        </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access license_recommendations outputs
license_recommendations_id = license_recommendations.id
license_recommendations_license_recommendations = license_recommendations.license_recommendations
license_recommendations_errors = license_recommendations.errors
license_recommendations_next_token = license_recommendations.next_token
```

---


### Lambda_function_recommendations

LambdaFunctionRecommendations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to use to advance to the next page of function recommendations.</p>
         <p>This value is null when there are no more pages of function recommendations to
            return.</p> |
| `lambda_function_recommendations` | Vec<String> | <p>An array of objects that describe function recommendations.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access lambda_function_recommendations outputs
lambda_function_recommendations_id = lambda_function_recommendations.id
lambda_function_recommendations_next_token = lambda_function_recommendations.next_token
lambda_function_recommendations_lambda_function_recommendations = lambda_function_recommendations.lambda_function_recommendations
```

---


### Effective_recommendation_preferences

EffectiveRecommendationPreferences resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `look_back_period` | String | <p>
            The number of days the utilization metrics of the Amazon Web Services resource are analyzed. 
        </p>
         <p>To validate that the preference is applied to your last generated set of recommendations, review 
            the <code>effectiveRecommendationPreferences</code> value in the response of the 
            GetAutoScalingGroupRecommendations or GetEC2InstanceRecommendations actions.</p> |
| `enhanced_infrastructure_metrics` | String | <p>The status of the enhanced infrastructure metrics recommendation preference. Considers
            all applicable preferences that you might have set at the resource, account, and
            organization level.</p>
         <p>A status of <code>Active</code> confirms that the preference is applied in the latest
            recommendation refresh, and a status of <code>Inactive</code> confirms that it's not yet
            applied to recommendations.</p>
         <p>To validate whether the preference is applied to your last generated set of
            recommendations, review the <code>effectiveRecommendationPreferences</code> value in the
            response of the <a>GetAutoScalingGroupRecommendations</a> and <a>GetEC2InstanceRecommendations</a> actions.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/compute-optimizer/latest/ug/enhanced-infrastructure-metrics.html">Enhanced
                infrastructure metrics</a> in the <i>Compute Optimizer User
                Guide</i>.</p> |
| `preferred_resources` | Vec<String> | <p>
            The resource type values that are considered as candidates when generating rightsizing recommendations. 
            This object resolves any wildcard expressions and returns the effective list of candidate resource type 
            values. It also considers all applicable preferences that you set at the resource, account, and 
            organization level.
        </p>
         <p>To validate that the preference is applied to your last generated set of recommendations, review the 
            <code>effectiveRecommendationPreferences</code> value in the response of the GetAutoScalingGroupRecommendations 
            or GetEC2InstanceRecommendations actions.</p> |
| `utilization_preferences` | Vec<String> | <p>
            The resource’s CPU and memory utilization preferences, such as threshold and headroom, 
            that were used to generate rightsizing recommendations. It considers all applicable preferences 
            that you set at the resource, account, and organization level.
        </p>
         <p>To validate that the preference is applied to your last generated set of recommendations, review the 
            <code>effectiveRecommendationPreferences</code> value in the response of the 
            GetAutoScalingGroupRecommendations or GetEC2InstanceRecommendations actions.</p> |
| `external_metrics_preference` | String | <p>The provider of the external metrics recommendation preference. Considers all
            applicable preferences that you might have set at the account and organization
            level.</p>
         <p>If the preference is applied in the latest recommendation refresh, an object with a
            valid <code>source</code> value appears in the response. If the preference isn't applied
            to the recommendations already, then this object doesn't appear in the response.</p>
         <p>To validate whether the preference is applied to your last generated set of
            recommendations, review the <code>effectiveRecommendationPreferences</code> value in the
            response of the <a>GetEC2InstanceRecommendations</a> actions.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/compute-optimizer/latest/ug/external-metrics-ingestion.html">Enhanced
                infrastructure metrics</a> in the <i>Compute Optimizer User
                Guide</i>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access effective_recommendation_preferences outputs
effective_recommendation_preferences_id = effective_recommendation_preferences.id
effective_recommendation_preferences_look_back_period = effective_recommendation_preferences.look_back_period
effective_recommendation_preferences_enhanced_infrastructure_metrics = effective_recommendation_preferences.enhanced_infrastructure_metrics
effective_recommendation_preferences_preferred_resources = effective_recommendation_preferences.preferred_resources
effective_recommendation_preferences_utilization_preferences = effective_recommendation_preferences.utilization_preferences
effective_recommendation_preferences_external_metrics_preference = effective_recommendation_preferences.external_metrics_preference
```

---


### Ec2_recommendation_projected_metrics

EC2RecommendationProjectedMetrics resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `recommended_option_projected_metrics` | Vec<String> | <p>An array of objects that describes projected metrics.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access ec2_recommendation_projected_metrics outputs
ec2_recommendation_projected_metrics_id = ec2_recommendation_projected_metrics.id
ec2_recommendation_projected_metrics_recommended_option_projected_metrics = ec2_recommendation_projected_metrics.recommended_option_projected_metrics
```

---


### Idle_recommendations

IdleRecommendations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `errors` | Vec<String> | <p>An array of objects that describe errors of the request.</p> |
| `idle_recommendations` | Vec<String> | <p>An array of objects that describe the idle resource recommendations.</p> |
| `next_token` | String | <p>The token to advance to the next page of idle resource recommendations.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access idle_recommendations outputs
idle_recommendations_id = idle_recommendations.id
idle_recommendations_errors = idle_recommendations.errors
idle_recommendations_idle_recommendations = idle_recommendations.idle_recommendations
idle_recommendations_next_token = idle_recommendations.next_token
```

---


### Rds_database_recommendation_projected_metrics

RDSDatabaseRecommendationProjectedMetrics resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `recommended_option_projected_metrics` | Vec<String> | <p>
            An array of objects that describes the projected metrics.
        </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access rds_database_recommendation_projected_metrics outputs
rds_database_recommendation_projected_metrics_id = rds_database_recommendation_projected_metrics.id
rds_database_recommendation_projected_metrics_recommended_option_projected_metrics = rds_database_recommendation_projected_metrics.recommended_option_projected_metrics
```

---


### Rds_database_recommendations

RDSDatabaseRecommendations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `rds_db_recommendations` | Vec<String> | <p>
            An array of objects that describe the Amazon Aurora and RDS database recommendations.
        </p> |
| `errors` | Vec<String> | <p>
            An array of objects that describe errors of the request.
        </p> |
| `next_token` | String | <p>
            The token to advance to the next page of Amazon Aurora and RDS database recommendations.
        </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access rds_database_recommendations outputs
rds_database_recommendations_id = rds_database_recommendations.id
rds_database_recommendations_rds_db_recommendations = rds_database_recommendations.rds_db_recommendations
rds_database_recommendations_errors = rds_database_recommendations.errors
rds_database_recommendations_next_token = rds_database_recommendations.next_token
```

---


### Enrollment_statuses_for_organization

EnrollmentStatusesForOrganization resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to use to advance to the next page of account enrollment statuses.</p>
         <p>This value is null when there are no more pages of account enrollment statuses to
            return.</p> |
| `account_enrollment_statuses` | Vec<String> | <p>An array of objects that describe the enrollment statuses of organization member
            accounts.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access enrollment_statuses_for_organization outputs
enrollment_statuses_for_organization_id = enrollment_statuses_for_organization.id
enrollment_statuses_for_organization_next_token = enrollment_statuses_for_organization.next_token
enrollment_statuses_for_organization_account_enrollment_statuses = enrollment_statuses_for_organization.account_enrollment_statuses
```

---


### Auto_scaling_group_recommendations

AutoScalingGroupRecommendations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `auto_scaling_group_recommendations` | Vec<String> | <p>An array of objects that describe Auto Scaling group recommendations.</p> |
| `next_token` | String | <p>The token to use to advance to the next page of Auto Scaling group
            recommendations.</p>
         <p>This value is null when there are no more pages of Auto Scaling group
            recommendations to return.</p> |
| `errors` | Vec<String> | <p>An array of objects that describe errors of the request.</p>
         <p>For example, an error is returned if you request recommendations for an unsupported
                Auto Scaling group.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access auto_scaling_group_recommendations outputs
auto_scaling_group_recommendations_id = auto_scaling_group_recommendations.id
auto_scaling_group_recommendations_auto_scaling_group_recommendations = auto_scaling_group_recommendations.auto_scaling_group_recommendations
auto_scaling_group_recommendations_next_token = auto_scaling_group_recommendations.next_token
auto_scaling_group_recommendations_errors = auto_scaling_group_recommendations.errors
```

---


### Ec2_instance_recommendations

EC2InstanceRecommendations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `errors` | Vec<String> | <p>An array of objects that describe errors of the request.</p>
         <p>For example, an error is returned if you request recommendations for an instance of an
            unsupported instance family.</p> |
| `next_token` | String | <p>The token to use to advance to the next page of instance recommendations.</p>
         <p>This value is null when there are no more pages of instance recommendations to
            return.</p> |
| `instance_recommendations` | Vec<String> | <p>An array of objects that describe instance recommendations.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access ec2_instance_recommendations outputs
ec2_instance_recommendations_id = ec2_instance_recommendations.id
ec2_instance_recommendations_errors = ec2_instance_recommendations.errors
ec2_instance_recommendations_next_token = ec2_instance_recommendations.next_token
ec2_instance_recommendations_instance_recommendations = ec2_instance_recommendations.instance_recommendations
```

---


### Ebs_volume_recommendations

EBSVolumeRecommendations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to use to advance to the next page of volume recommendations.</p>
         <p>This value is null when there are no more pages of volume recommendations to
            return.</p> |
| `errors` | Vec<String> | <p>An array of objects that describe errors of the request.</p>
         <p>For example, an error is returned if you request recommendations for an unsupported
            volume.</p> |
| `volume_recommendations` | Vec<String> | <p>An array of objects that describe volume recommendations.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access ebs_volume_recommendations outputs
ebs_volume_recommendations_id = ebs_volume_recommendations.id
ebs_volume_recommendations_next_token = ebs_volume_recommendations.next_token
ebs_volume_recommendations_errors = ebs_volume_recommendations.errors
ebs_volume_recommendations_volume_recommendations = ebs_volume_recommendations.volume_recommendations
```

---


### Recommendation_export_jobs

RecommendationExportJobs resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to use to advance to the next page of export jobs.</p>
         <p>This value is null when there are no more pages of export jobs to return.</p> |
| `recommendation_export_jobs` | Vec<String> | <p>An array of objects that describe recommendation export jobs.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access recommendation_export_jobs outputs
recommendation_export_jobs_id = recommendation_export_jobs.id
recommendation_export_jobs_next_token = recommendation_export_jobs.next_token
recommendation_export_jobs_recommendation_export_jobs = recommendation_export_jobs.recommendation_export_jobs
```

---


### Ecs_service_recommendations

ECSServiceRecommendations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `ecs_service_recommendations` | Vec<String> | <p>
            An array of objects that describe the Amazon ECS service recommendations.
        </p> |
| `errors` | Vec<String> | <p>
            An array of objects that describe errors of the request.
        </p> |
| `next_token` | String | <p>
            The token to advance to the next page of Amazon ECS service recommendations.
        </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access ecs_service_recommendations outputs
ecs_service_recommendations_id = ecs_service_recommendations.id
ecs_service_recommendations_ecs_service_recommendations = ecs_service_recommendations.ecs_service_recommendations
ecs_service_recommendations_errors = ecs_service_recommendations.errors
ecs_service_recommendations_next_token = ecs_service_recommendations.next_token
```

---


### Recommendation_summaries

RecommendationSummaries resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to use to advance to the next page of recommendation summaries.</p>
         <p>This value is null when there are no more pages of recommendation summaries to
            return.</p> |
| `recommendation_summaries` | Vec<String> | <p>An array of objects that summarize a recommendation.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access recommendation_summaries outputs
recommendation_summaries_id = recommendation_summaries.id
recommendation_summaries_next_token = recommendation_summaries.next_token
recommendation_summaries_recommendation_summaries = recommendation_summaries.recommendation_summaries
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple recommendation_preferences resources
recommendation_preferences_0 = provider.compute_optimizer.Recommendation_preferences {
    resource_type = "value-0"
}
recommendation_preferences_1 = provider.compute_optimizer.Recommendation_preferences {
    resource_type = "value-1"
}
recommendation_preferences_2 = provider.compute_optimizer.Recommendation_preferences {
    resource_type = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    recommendation_preferences = provider.compute_optimizer.Recommendation_preferences {
        resource_type = "production-value"
    }
```

---

## Related Documentation

- [AWS Compute_optimizer Documentation](https://docs.aws.amazon.com/compute_optimizer/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
