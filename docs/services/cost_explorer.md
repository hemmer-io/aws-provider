# Cost_explorer Service



**Resources**: 27

---

## Overview

The cost_explorer service provides access to 27 resource types:

- [Savings_plans_coverage](#savings_plans_coverage) [R]
- [Cost_category_definition](#cost_category_definition) [CRUD]
- [Savings_plans_utilization_details](#savings_plans_utilization_details) [R]
- [Anomalies](#anomalies) [R]
- [Reservation_purchase_recommendation](#reservation_purchase_recommendation) [R]
- [Reservation_utilization](#reservation_utilization) [R]
- [Commitment_purchase_analysis](#commitment_purchase_analysis) [R]
- [Cost_categories](#cost_categories) [R]
- [Anomaly_monitor](#anomaly_monitor) [CUD]
- [Cost_and_usage](#cost_and_usage) [R]
- [Rightsizing_recommendation](#rightsizing_recommendation) [R]
- [Dimension_values](#dimension_values) [R]
- [Approximate_usage_records](#approximate_usage_records) [R]
- [Reservation_coverage](#reservation_coverage) [R]
- [Cost_allocation_tags_status](#cost_allocation_tags_status) [U]
- [Anomaly_monitors](#anomaly_monitors) [R]
- [Usage_forecast](#usage_forecast) [R]
- [Cost_comparison_drivers](#cost_comparison_drivers) [R]
- [Cost_and_usage_comparisons](#cost_and_usage_comparisons) [R]
- [Savings_plans_utilization](#savings_plans_utilization) [R]
- [Savings_plan_purchase_recommendation_details](#savings_plan_purchase_recommendation_details) [R]
- [Tags](#tags) [R]
- [Anomaly_subscription](#anomaly_subscription) [CUD]
- [Anomaly_subscriptions](#anomaly_subscriptions) [R]
- [Cost_forecast](#cost_forecast) [R]
- [Cost_and_usage_with_resources](#cost_and_usage_with_resources) [R]
- [Savings_plans_purchase_recommendation](#savings_plans_purchase_recommendation) [R]

---

## Resources


### Savings_plans_coverage

SavingsPlansCoverage resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `savings_plans_coverages` | Vec<String> | <p>The amount of spend that your Savings Plans covered.</p> |
| `next_token` | String | <p>The token to retrieve the next set of results. Amazon Web Services provides the token when
      the response from a previous call has more results than the maximum page size.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access savings_plans_coverage outputs
savings_plans_coverage_id = savings_plans_coverage.id
savings_plans_coverage_savings_plans_coverages = savings_plans_coverage.savings_plans_coverages
savings_plans_coverage_next_token = savings_plans_coverage.next_token
```

---


### Cost_category_definition

CostCategoryDefinition resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ |  |
| `rules` | Vec<String> | ✅ | <p>The Cost Category rules used to categorize costs. For more information, see <a href="https://docs.aws.amazon.com/aws-cost-management/latest/APIReference/API_CostCategoryRule.html">CostCategoryRule</a>.</p> |
| `effective_start` | String |  | <p>The Cost Category's effective start date. It can only be a billing start date (first day of the month). If the date isn't provided, it's the first day of the current month. Dates can't be before the previous twelve months, or in the future.</p> |
| `rule_version` | String | ✅ |  |
| `default_value` | String |  |  |
| `split_charge_rules` | Vec<String> |  | <p> The split charge rules used to allocate your charges between your Cost Category values.
    </p> |
| `resource_tags` | Vec<String> |  | <p>An optional list of tags to associate with the specified <a href="https://docs.aws.amazon.com/aws-cost-management/latest/APIReference/API_CostCategory.html">
               <code>CostCategory</code>
            </a>. You can use resource tags to control access to your
        <code>cost category</code> using IAM policies.</p>
         <p>Each tag consists of a key and a value, and each key must be unique for the resource. The
      following restrictions apply to resource tags:</p>
         <ul>
            <li>
               <p>Although the maximum number of array members is 200, you can assign a maximum of 50
          user-tags to one resource. The remaining are reserved for Amazon Web Services use</p>
            </li>
            <li>
               <p>The maximum length of a key is 128 characters</p>
            </li>
            <li>
               <p>The maximum length of a value is 256 characters</p>
            </li>
            <li>
               <p>Keys and values can only contain alphanumeric characters, spaces, and any of the
          following: <code>_.:/=+@-</code>
               </p>
            </li>
            <li>
               <p>Keys and values are case sensitive</p>
            </li>
            <li>
               <p>Keys and values are trimmed for any leading or trailing whitespaces</p>
            </li>
            <li>
               <p>Don’t use <code>aws:</code> as a prefix for your keys. This prefix is reserved for
            Amazon Web Services use</p>
            </li>
         </ul> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `cost_category` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create cost_category_definition
cost_category_definition = provider.cost_explorer.Cost_category_definition {
    name = "value"  # Required field
    rules = "value"  # <p>The Cost Category rules used to categorize costs. For more information, see <a href="https://docs.aws.amazon.com/aws-cost-management/latest/APIReference/API_CostCategoryRule.html">CostCategoryRule</a>.</p>
    rule_version = "value"  # Required field
}

# Access cost_category_definition outputs
cost_category_definition_id = cost_category_definition.id
cost_category_definition_cost_category = cost_category_definition.cost_category
```

---


### Savings_plans_utilization_details

SavingsPlansUtilizationDetails resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `savings_plans_utilization_details` | Vec<String> | <p>Retrieves a single daily or monthly Savings Plans utilization rate and details for your
      account.</p> |
| `total` | String | <p>The total Savings Plans utilization, regardless of time period.</p> |
| `time_period` | String |  |
| `next_token` | String | <p>The token to retrieve the next set of results. Amazon Web Services provides the token when
      the response from a previous call has more results than the maximum page size.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access savings_plans_utilization_details outputs
savings_plans_utilization_details_id = savings_plans_utilization_details.id
savings_plans_utilization_details_savings_plans_utilization_details = savings_plans_utilization_details.savings_plans_utilization_details
savings_plans_utilization_details_total = savings_plans_utilization_details.total
savings_plans_utilization_details_time_period = savings_plans_utilization_details.time_period
savings_plans_utilization_details_next_token = savings_plans_utilization_details.next_token
```

---


### Anomalies

Anomalies resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `anomalies` | Vec<String> | <p>A list of cost anomalies. </p> |
| `next_page_token` | String | <p>The token to retrieve the next set of results. Amazon Web Services provides the token when
      the response from a previous call has more results than the maximum page size. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access anomalies outputs
anomalies_id = anomalies.id
anomalies_anomalies = anomalies.anomalies
anomalies_next_page_token = anomalies.next_page_token
```

---


### Reservation_purchase_recommendation

ReservationPurchaseRecommendation resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `metadata` | String | <p>Information about this specific recommendation call, such as the time stamp for when
      Cost Explorer generated this recommendation.</p> |
| `next_page_token` | String | <p>The pagination token for the next set of retrievable results.</p> |
| `recommendations` | Vec<String> | <p>Recommendations for reservations to purchase.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access reservation_purchase_recommendation outputs
reservation_purchase_recommendation_id = reservation_purchase_recommendation.id
reservation_purchase_recommendation_metadata = reservation_purchase_recommendation.metadata
reservation_purchase_recommendation_next_page_token = reservation_purchase_recommendation.next_page_token
reservation_purchase_recommendation_recommendations = reservation_purchase_recommendation.recommendations
```

---


### Reservation_utilization

ReservationUtilization resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | <p>The token for the next set of retrievable results. Amazon Web Services provides the token
      when the response from a previous call has more results than the maximum page size.</p> |
| `utilizations_by_time` | Vec<String> | <p>The amount of time that you used your Reserved Instances (RIs).</p> |
| `total` | String | <p>The total amount of time that you used your Reserved Instances (RIs).</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access reservation_utilization outputs
reservation_utilization_id = reservation_utilization.id
reservation_utilization_next_page_token = reservation_utilization.next_page_token
reservation_utilization_utilizations_by_time = reservation_utilization.utilizations_by_time
reservation_utilization_total = reservation_utilization.total
```

---


### Commitment_purchase_analysis

CommitmentPurchaseAnalysis resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `estimated_completion_time` | String | <p>The estimated time for when the analysis will complete.</p> |
| `commitment_purchase_analysis_configuration` | String | <p>The configuration for the commitment purchase analysis.</p> |
| `analysis_completion_time` | String | <p>The completion time of the analysis.</p> |
| `analysis_id` | String | <p>The analysis ID that's associated with the commitment purchase analysis.</p> |
| `analysis_details` | String | <p>Details about the analysis.</p> |
| `analysis_status` | String | <p>The status of the analysis.</p> |
| `error_code` | String | <p>The error code used for the analysis.</p> |
| `analysis_started_time` | String | <p>The start time of the analysis.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access commitment_purchase_analysis outputs
commitment_purchase_analysis_id = commitment_purchase_analysis.id
commitment_purchase_analysis_estimated_completion_time = commitment_purchase_analysis.estimated_completion_time
commitment_purchase_analysis_commitment_purchase_analysis_configuration = commitment_purchase_analysis.commitment_purchase_analysis_configuration
commitment_purchase_analysis_analysis_completion_time = commitment_purchase_analysis.analysis_completion_time
commitment_purchase_analysis_analysis_id = commitment_purchase_analysis.analysis_id
commitment_purchase_analysis_analysis_details = commitment_purchase_analysis.analysis_details
commitment_purchase_analysis_analysis_status = commitment_purchase_analysis.analysis_status
commitment_purchase_analysis_error_code = commitment_purchase_analysis.error_code
commitment_purchase_analysis_analysis_started_time = commitment_purchase_analysis.analysis_started_time
```

---


### Cost_categories

CostCategories resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `cost_category_names` | Vec<String> | <p>The names of the Cost Categories.</p> |
| `cost_category_values` | Vec<String> | <p>The Cost Category values.</p>
         <p>If the <code>CostCategoryName</code> key isn't specified in the request, the
        <code>CostCategoryValues</code> fields aren't returned. </p> |
| `return_size` | i64 | <p>The number of objects that are returned.</p> |
| `total_size` | i64 | <p>The total number of objects.</p> |
| `next_page_token` | String | <p>If the number of objects that are still available for retrieval exceeds the quota, Amazon Web Services returns a NextPageToken value in the response. To retrieve the next batch of
      objects, provide the marker from the prior call in your next request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access cost_categories outputs
cost_categories_id = cost_categories.id
cost_categories_cost_category_names = cost_categories.cost_category_names
cost_categories_cost_category_values = cost_categories.cost_category_values
cost_categories_return_size = cost_categories.return_size
cost_categories_total_size = cost_categories.total_size
cost_categories_next_page_token = cost_categories.next_page_token
```

---


### Anomaly_monitor

AnomalyMonitor resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `anomaly_monitor` | String | ✅ | <p>The cost anomaly detection monitor object that you want to create.</p> |
| `resource_tags` | Vec<String> |  | <p>An optional list of tags to associate with the specified <a href="https://docs.aws.amazon.com/aws-cost-management/latest/APIReference/API_AnomalyMonitor.html">
               <code>AnomalyMonitor</code>
            </a>. You can use resource tags to control access to your
        <code>monitor</code> using IAM policies.</p>
         <p>Each tag consists of a key and a value, and each key must be unique for the resource. The
      following restrictions apply to resource tags:</p>
         <ul>
            <li>
               <p>Although the maximum number of array members is 200, you can assign a maximum of 50
          user-tags to one resource. The remaining are reserved for Amazon Web Services use</p>
            </li>
            <li>
               <p>The maximum length of a key is 128 characters</p>
            </li>
            <li>
               <p>The maximum length of a value is 256 characters</p>
            </li>
            <li>
               <p>Keys and values can only contain alphanumeric characters, spaces, and any of the
          following: <code>_.:/=+@-</code>
               </p>
            </li>
            <li>
               <p>Keys and values are case sensitive</p>
            </li>
            <li>
               <p>Keys and values are trimmed for any leading or trailing whitespaces</p>
            </li>
            <li>
               <p>Don’t use <code>aws:</code> as a prefix for your keys. This prefix is reserved for
            Amazon Web Services use</p>
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

# Create anomaly_monitor
anomaly_monitor = provider.cost_explorer.Anomaly_monitor {
    anomaly_monitor = "value"  # <p>The cost anomaly detection monitor object that you want to create.</p>
}

```

---


### Cost_and_usage

CostAndUsage resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `dimension_value_attributes` | Vec<String> | <p>The attributes that apply to a specific dimension value. For example, if the value is a
      linked account, the attribute is that account name.</p> |
| `group_definitions` | Vec<String> | <p>The groups that are specified by the <code>Filter</code> or <code>GroupBy</code>
      parameters in the request.</p> |
| `results_by_time` | Vec<String> | <p>The time period that's covered by the results in the response.</p> |
| `next_page_token` | String | <p>The token for the next set of retrievable results. Amazon Web Services provides the token
      when the response from a previous call has more results than the maximum page size.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access cost_and_usage outputs
cost_and_usage_id = cost_and_usage.id
cost_and_usage_dimension_value_attributes = cost_and_usage.dimension_value_attributes
cost_and_usage_group_definitions = cost_and_usage.group_definitions
cost_and_usage_results_by_time = cost_and_usage.results_by_time
cost_and_usage_next_page_token = cost_and_usage.next_page_token
```

---


### Rightsizing_recommendation

RightsizingRecommendation resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | <p>The token to retrieve the next set of results.</p> |
| `metadata` | String | <p>Information regarding this specific recommendation set.</p> |
| `summary` | String | <p>Summary of this recommendation set.</p> |
| `rightsizing_recommendations` | Vec<String> | <p>Recommendations to rightsize resources.</p> |
| `configuration` | String | <p>You can use Configuration to customize recommendations across two attributes. You can
      choose to view recommendations for instances within the same instance families or across
      different instance families. You can also choose to view your estimated savings that are
      associated with recommendations with consideration of existing Savings Plans or RI benefits,
      or neither. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access rightsizing_recommendation outputs
rightsizing_recommendation_id = rightsizing_recommendation.id
rightsizing_recommendation_next_page_token = rightsizing_recommendation.next_page_token
rightsizing_recommendation_metadata = rightsizing_recommendation.metadata
rightsizing_recommendation_summary = rightsizing_recommendation.summary
rightsizing_recommendation_rightsizing_recommendations = rightsizing_recommendation.rightsizing_recommendations
rightsizing_recommendation_configuration = rightsizing_recommendation.configuration
```

---


### Dimension_values

DimensionValues resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `return_size` | i64 | <p>The number of results that Amazon Web Services returned at one time.</p> |
| `total_size` | i64 | <p>The total number of search results.</p> |
| `dimension_values` | Vec<String> | <p>The filters that you used to filter your request. Some dimensions are available only
      for a specific context.</p>
         <p>If you set the context to <code>COST_AND_USAGE</code>, you can use the following
      dimensions for searching:</p>
         <ul>
            <li>
               <p>AZ - The Availability Zone. An example is <code>us-east-1a</code>.</p>
            </li>
            <li>
               <p>DATABASE_ENGINE - The Amazon Relational Database Service database. Examples are
          Aurora or MySQL.</p>
            </li>
            <li>
               <p>INSTANCE_TYPE - The type of Amazon EC2 instance. An example is
            <code>m4.xlarge</code>.</p>
            </li>
            <li>
               <p>LEGAL_ENTITY_NAME - The name of the organization that sells you Amazon Web Services
          services, such as Amazon Web Services.</p>
            </li>
            <li>
               <p>LINKED_ACCOUNT - The description in the attribute map that includes the full name
          of the member account. The value field contains the Amazon Web Services ID of the member
          account.</p>
            </li>
            <li>
               <p>OPERATING_SYSTEM - The operating system. Examples are Windows or Linux.</p>
            </li>
            <li>
               <p>OPERATION - The action performed. Examples include <code>RunInstance</code> and
            <code>CreateBucket</code>.</p>
            </li>
            <li>
               <p>PLATFORM - The Amazon EC2 operating system. Examples are Windows or
          Linux.</p>
            </li>
            <li>
               <p>PURCHASE_TYPE - The reservation type of the purchase to which this usage is
          related. Examples include On-Demand Instances and Standard Reserved Instances.</p>
            </li>
            <li>
               <p>SERVICE - The Amazon Web Services service such as Amazon DynamoDB.</p>
            </li>
            <li>
               <p>USAGE_TYPE - The type of usage. An example is DataTransfer-In-Bytes. The response
          for the <code>GetDimensionValues</code> operation includes a unit attribute. Examples
          include GB and Hrs.</p>
            </li>
            <li>
               <p>USAGE_TYPE_GROUP - The grouping of common usage types. An example is Amazon EC2:
          CloudWatch – Alarms. The response for this operation includes a unit attribute.</p>
            </li>
            <li>
               <p>RECORD_TYPE - The different types of charges such as RI fees, usage costs, tax
          refunds, and credits.</p>
            </li>
            <li>
               <p>RESOURCE_ID - The unique identifier of the resource. ResourceId is an opt-in
          feature only available for last 14 days for EC2-Compute Service. You can opt-in by
          enabling <code>Hourly</code> and <code>Resource Level Data</code> in Cost Management Console preferences.</p>
            </li>
         </ul>
         <p>If you set the context to <code>RESERVATIONS</code>, you can use the following
      dimensions for searching:</p>
         <ul>
            <li>
               <p>AZ - The Availability Zone. An example is <code>us-east-1a</code>.</p>
            </li>
            <li>
               <p>CACHE_ENGINE - The Amazon ElastiCache operating system. Examples are Windows or
          Linux.</p>
            </li>
            <li>
               <p>DEPLOYMENT_OPTION - The scope of Amazon Relational Database Service deployments.
          Valid values are <code>SingleAZ</code> and <code>MultiAZ</code>.</p>
            </li>
            <li>
               <p>INSTANCE_TYPE - The type of Amazon EC2 instance. An example is
            <code>m4.xlarge</code>.</p>
            </li>
            <li>
               <p>LINKED_ACCOUNT - The description in the attribute map that includes the full name
          of the member account. The value field contains the Amazon Web Services ID of the member
          account.</p>
            </li>
            <li>
               <p>PLATFORM - The Amazon EC2 operating system. Examples are Windows or
          Linux.</p>
            </li>
            <li>
               <p>REGION - The Amazon Web Services Region.</p>
            </li>
            <li>
               <p>SCOPE (Utilization only) - The scope of a Reserved Instance (RI). Values are
          regional or a single Availability Zone.</p>
            </li>
            <li>
               <p>TAG (Coverage only) - The tags that are associated with a Reserved Instance
          (RI).</p>
            </li>
            <li>
               <p>TENANCY - The tenancy of a resource. Examples are shared or dedicated.</p>
            </li>
         </ul>
         <p>If you set the context to <code>SAVINGS_PLANS</code>, you can use the following
      dimensions for searching:</p>
         <ul>
            <li>
               <p>SAVINGS_PLANS_TYPE - Type of Savings Plans (EC2 Instance or Compute)</p>
            </li>
            <li>
               <p>PAYMENT_OPTION - Payment option for the given Savings Plans (for example, All
          Upfront)</p>
            </li>
            <li>
               <p>REGION - The Amazon Web Services Region.</p>
            </li>
            <li>
               <p>INSTANCE_TYPE_FAMILY - The family of instances (For example,
          <code>m5</code>)</p>
            </li>
            <li>
               <p>LINKED_ACCOUNT - The description in the attribute map that includes the full name
          of the member account. The value field contains the Amazon Web Services ID of the member
          account.</p>
            </li>
            <li>
               <p>SAVINGS_PLAN_ARN - The unique identifier for your Savings Plan</p>
            </li>
         </ul> |
| `next_page_token` | String | <p>The token for the next set of retrievable results. Amazon Web Services provides the token
      when the response from a previous call has more results than the maximum page size.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access dimension_values outputs
dimension_values_id = dimension_values.id
dimension_values_return_size = dimension_values.return_size
dimension_values_total_size = dimension_values.total_size
dimension_values_dimension_values = dimension_values.dimension_values
dimension_values_next_page_token = dimension_values.next_page_token
```

---


### Approximate_usage_records

ApproximateUsageRecords resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `services` | HashMap<String, i64> | <p>The service metadata for the service or services in the response.</p> |
| `total_records` | i64 | <p>The total number of usage records for all services in the services list.</p> |
| `lookback_period` | String | <p>The lookback period that's used for the estimation.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access approximate_usage_records outputs
approximate_usage_records_id = approximate_usage_records.id
approximate_usage_records_services = approximate_usage_records.services
approximate_usage_records_total_records = approximate_usage_records.total_records
approximate_usage_records_lookback_period = approximate_usage_records.lookback_period
```

---


### Reservation_coverage

ReservationCoverage resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `total` | String | <p>The total amount of instance usage that a reservation covered.</p> |
| `coverages_by_time` | Vec<String> | <p>The amount of time that your reservations covered.</p> |
| `next_page_token` | String | <p>The token for the next set of retrievable results. Amazon Web Services provides the token
      when the response from a previous call has more results than the maximum page size.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access reservation_coverage outputs
reservation_coverage_id = reservation_coverage.id
reservation_coverage_total = reservation_coverage.total
reservation_coverage_coverages_by_time = reservation_coverage.coverages_by_time
reservation_coverage_next_page_token = reservation_coverage.next_page_token
```

---


### Cost_allocation_tags_status

CostAllocationTagsStatus resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `cost_allocation_tags_status` | Vec<String> | ✅ | <p>The list of <code>CostAllocationTagStatusEntry</code> objects that are used to update cost
      allocation tags status for this request. </p> |



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


### Anomaly_monitors

AnomalyMonitors resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | <p>The token to retrieve the next set of results. Amazon Web Services provides the token when
      the response from a previous call has more results than the maximum page size. </p> |
| `anomaly_monitors` | Vec<String> | <p>A list of cost anomaly monitors that includes the detailed metadata for each monitor.
    </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access anomaly_monitors outputs
anomaly_monitors_id = anomaly_monitors.id
anomaly_monitors_next_page_token = anomaly_monitors.next_page_token
anomaly_monitors_anomaly_monitors = anomaly_monitors.anomaly_monitors
```

---


### Usage_forecast

UsageForecast resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `total` | String | <p>How much you're forecasted to use over the forecast period.</p> |
| `forecast_results_by_time` | Vec<String> | <p>The forecasts for your query, in order. For <code>DAILY</code> forecasts, this is a
      list of days. For <code>MONTHLY</code> forecasts, this is a list of months.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access usage_forecast outputs
usage_forecast_id = usage_forecast.id
usage_forecast_total = usage_forecast.total
usage_forecast_forecast_results_by_time = usage_forecast.forecast_results_by_time
```

---


### Cost_comparison_drivers

CostComparisonDrivers resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `cost_comparison_drivers` | Vec<String> | <p>An array of comparison results showing factors that drive significant cost differences
      between <code>BaselineTimePeriod</code> and <code>ComparisonTimePeriod</code>.</p> |
| `next_page_token` | String | <p>The token to retrieve the next set of paginated results.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access cost_comparison_drivers outputs
cost_comparison_drivers_id = cost_comparison_drivers.id
cost_comparison_drivers_cost_comparison_drivers = cost_comparison_drivers.cost_comparison_drivers
cost_comparison_drivers_next_page_token = cost_comparison_drivers.next_page_token
```

---


### Cost_and_usage_comparisons

CostAndUsageComparisons resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `total_cost_and_usage` | HashMap<String, String> | <p>A summary of the total cost and usage, comparing amounts between
        <code>BaselineTimePeriod</code> and <code>ComparisonTimePeriod</code> and their differences.
      This total represents the aggregate total across all paginated results, if the response spans
      multiple pages.</p> |
| `cost_and_usage_comparisons` | Vec<String> | <p>An array of comparison results showing cost and usage metrics between
        <code>BaselineTimePeriod</code> and <code>ComparisonTimePeriod</code>.</p> |
| `next_page_token` | String | <p>The token to retrieve the next set of paginated results.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access cost_and_usage_comparisons outputs
cost_and_usage_comparisons_id = cost_and_usage_comparisons.id
cost_and_usage_comparisons_total_cost_and_usage = cost_and_usage_comparisons.total_cost_and_usage
cost_and_usage_comparisons_cost_and_usage_comparisons = cost_and_usage_comparisons.cost_and_usage_comparisons
cost_and_usage_comparisons_next_page_token = cost_and_usage_comparisons.next_page_token
```

---


### Savings_plans_utilization

SavingsPlansUtilization resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `total` | String | <p>The total amount of cost/commitment that you used your Savings Plans, regardless of date
      ranges.</p> |
| `savings_plans_utilizations_by_time` | Vec<String> | <p>The amount of cost/commitment that you used your Savings Plans. You can use it to specify
      date ranges.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access savings_plans_utilization outputs
savings_plans_utilization_id = savings_plans_utilization.id
savings_plans_utilization_total = savings_plans_utilization.total
savings_plans_utilization_savings_plans_utilizations_by_time = savings_plans_utilization.savings_plans_utilizations_by_time
```

---


### Savings_plan_purchase_recommendation_details

SavingsPlanPurchaseRecommendationDetails resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `recommendation_detail_data` | String | <p>Contains detailed information about a specific Savings Plan recommendation.</p> |
| `recommendation_detail_id` | String | <p>The ID that is associated with the Savings Plan recommendation.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access savings_plan_purchase_recommendation_details outputs
savings_plan_purchase_recommendation_details_id = savings_plan_purchase_recommendation_details.id
savings_plan_purchase_recommendation_details_recommendation_detail_data = savings_plan_purchase_recommendation_details.recommendation_detail_data
savings_plan_purchase_recommendation_details_recommendation_detail_id = savings_plan_purchase_recommendation_details.recommendation_detail_id
```

---


### Tags

Tags resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `return_size` | i64 | <p>The number of query results that Amazon Web Services returns at a time.</p> |
| `tags` | Vec<String> | <p>The tags that match your request.</p> |
| `next_page_token` | String | <p>The token for the next set of retrievable results. Amazon Web Services provides the token
      when the response from a previous call has more results than the maximum page size.</p> |
| `total_size` | i64 | <p>The total number of query results.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access tags outputs
tags_id = tags.id
tags_return_size = tags.return_size
tags_tags = tags.tags
tags_next_page_token = tags.next_page_token
tags_total_size = tags.total_size
```

---


### Anomaly_subscription

AnomalySubscription resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resource_tags` | Vec<String> |  | <p>An optional list of tags to associate with the specified <a href="https://docs.aws.amazon.com/aws-cost-management/latest/APIReference/API_AnomalySubscription.html">
               <code>AnomalySubscription</code>
            </a>. You can use resource tags to control access to
      your <code>subscription</code> using IAM policies.</p>
         <p>Each tag consists of a key and a value, and each key must be unique for the resource. The
      following restrictions apply to resource tags:</p>
         <ul>
            <li>
               <p>Although the maximum number of array members is 200, you can assign a maximum of 50
          user-tags to one resource. The remaining are reserved for Amazon Web Services use</p>
            </li>
            <li>
               <p>The maximum length of a key is 128 characters</p>
            </li>
            <li>
               <p>The maximum length of a value is 256 characters</p>
            </li>
            <li>
               <p>Keys and values can only contain alphanumeric characters, spaces, and any of the
          following: <code>_.:/=+@-</code>
               </p>
            </li>
            <li>
               <p>Keys and values are case sensitive</p>
            </li>
            <li>
               <p>Keys and values are trimmed for any leading or trailing whitespaces</p>
            </li>
            <li>
               <p>Don’t use <code>aws:</code> as a prefix for your keys. This prefix is reserved for
            Amazon Web Services use</p>
            </li>
         </ul> |
| `anomaly_subscription` | String | ✅ | <p>The cost anomaly subscription object that you want to create. </p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create anomaly_subscription
anomaly_subscription = provider.cost_explorer.Anomaly_subscription {
    anomaly_subscription = "value"  # <p>The cost anomaly subscription object that you want to create. </p>
}

```

---


### Anomaly_subscriptions

AnomalySubscriptions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | <p>The token to retrieve the next set of results. Amazon Web Services provides the token when
      the response from a previous call has more results than the maximum page size. </p> |
| `anomaly_subscriptions` | Vec<String> | <p>A list of cost anomaly subscriptions that includes the detailed metadata for each one.
    </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access anomaly_subscriptions outputs
anomaly_subscriptions_id = anomaly_subscriptions.id
anomaly_subscriptions_next_page_token = anomaly_subscriptions.next_page_token
anomaly_subscriptions_anomaly_subscriptions = anomaly_subscriptions.anomaly_subscriptions
```

---


### Cost_forecast

CostForecast resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `forecast_results_by_time` | Vec<String> | <p>The forecasts for your query, in order. For <code>DAILY</code> forecasts, this is a list
      of days. For <code>MONTHLY</code> forecasts, this is a list of months.</p> |
| `total` | String | <p>How much you are forecasted to spend over the forecast period, in <code>USD</code>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access cost_forecast outputs
cost_forecast_id = cost_forecast.id
cost_forecast_forecast_results_by_time = cost_forecast.forecast_results_by_time
cost_forecast_total = cost_forecast.total
```

---


### Cost_and_usage_with_resources

CostAndUsageWithResources resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | <p>The token for the next set of retrievable results. Amazon Web Services provides the token
      when the response from a previous call has more results than the maximum page size.</p> |
| `group_definitions` | Vec<String> | <p>The groups that are specified by the <code>Filter</code> or <code>GroupBy</code>
      parameters in the request.</p> |
| `results_by_time` | Vec<String> | <p>The time period that's covered by the results in the response.</p> |
| `dimension_value_attributes` | Vec<String> | <p>The attributes that apply to a specific dimension value. For example, if the value is a
      linked account, the attribute is that account name.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access cost_and_usage_with_resources outputs
cost_and_usage_with_resources_id = cost_and_usage_with_resources.id
cost_and_usage_with_resources_next_page_token = cost_and_usage_with_resources.next_page_token
cost_and_usage_with_resources_group_definitions = cost_and_usage_with_resources.group_definitions
cost_and_usage_with_resources_results_by_time = cost_and_usage_with_resources.results_by_time
cost_and_usage_with_resources_dimension_value_attributes = cost_and_usage_with_resources.dimension_value_attributes
```

---


### Savings_plans_purchase_recommendation

SavingsPlansPurchaseRecommendation resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | <p>The token for the next set of retrievable results. Amazon Web Services provides the token
      when the response from a previous call has more results than the maximum page size.</p> |
| `savings_plans_purchase_recommendation` | String | <p>Contains your request parameters, Savings Plan Recommendations Summary, and
      Details.</p> |
| `metadata` | String | <p>Information that regards this specific recommendation set.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access savings_plans_purchase_recommendation outputs
savings_plans_purchase_recommendation_id = savings_plans_purchase_recommendation.id
savings_plans_purchase_recommendation_next_page_token = savings_plans_purchase_recommendation.next_page_token
savings_plans_purchase_recommendation_savings_plans_purchase_recommendation = savings_plans_purchase_recommendation.savings_plans_purchase_recommendation
savings_plans_purchase_recommendation_metadata = savings_plans_purchase_recommendation.metadata
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple savings_plans_coverage resources
savings_plans_coverage_0 = provider.cost_explorer.Savings_plans_coverage {
}
savings_plans_coverage_1 = provider.cost_explorer.Savings_plans_coverage {
}
savings_plans_coverage_2 = provider.cost_explorer.Savings_plans_coverage {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    savings_plans_coverage = provider.cost_explorer.Savings_plans_coverage {
    }
```

---

## Related Documentation

- [AWS Cost_explorer Documentation](https://docs.aws.amazon.com/cost_explorer/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
