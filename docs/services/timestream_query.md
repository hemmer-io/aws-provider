# Timestream_query Service



**Resources**: 3

---

## Overview

The timestream_query service provides access to 3 resource types:

- [Scheduled_query](#scheduled_query) [CRUD]
- [Endpoints](#endpoints) [R]
- [Account_settings](#account_settings) [RU]

---

## Resources


### Scheduled_query

ScheduledQuery resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_token` | String |  | <p>Using a ClientToken makes the call to CreateScheduledQuery idempotent, in other words,
            making the same request repeatedly will produce the same result. Making multiple
            identical CreateScheduledQuery requests has the same effect as making a single request. </p>
         <ul>
            <li>
               <p> If CreateScheduledQuery is called without a <code>ClientToken</code>, the
                    Query SDK generates a <code>ClientToken</code> on your behalf.</p>
            </li>
            <li>
               <p> After 8 hours, any request with the same <code>ClientToken</code> is treated
                    as a new request. </p>
            </li>
         </ul> |
| `kms_key_id` | String |  | <p>The Amazon KMS key used to encrypt the scheduled query resource, at-rest. If the
            Amazon KMS key is not specified, the scheduled query resource will be encrypted with a
            Timestream owned Amazon KMS key. To specify a KMS key, use the key ID, key ARN, alias
            name, or alias ARN. When using an alias name, prefix the name with
                <i>alias/</i>
         </p>
         <p>If ErrorReportConfiguration uses <code>SSE_KMS</code> as encryption type, the same
            KmsKeyId is used to encrypt the error report at rest.</p> |
| `query_string` | String | ✅ | <p>The query string to run. Parameter names can be specified in the query string
                <code>@</code> character followed by an identifier. The named Parameter
                <code>@scheduled_runtime</code> is reserved and can be used in the query to get the
            time at which the query is scheduled to run.</p>
         <p>The timestamp calculated according to the ScheduleConfiguration parameter, will be the
            value of <code>@scheduled_runtime</code> paramater for each query run. For example,
            consider an instance of a scheduled query executing on 2021-12-01 00:00:00. For this
            instance, the <code>@scheduled_runtime</code> parameter is initialized to the timestamp
            2021-12-01 00:00:00 when invoking the query.</p> |
| `name` | String | ✅ | <p>Name of the scheduled query.</p> |
| `schedule_configuration` | String | ✅ | <p>The schedule configuration for the query.</p> |
| `scheduled_query_execution_role_arn` | String | ✅ | <p>The ARN for the IAM role that Timestream will assume when running the scheduled query.
        </p> |
| `tags` | Vec<String> |  | <p>A list of key-value pairs to label the scheduled query.</p> |
| `notification_configuration` | String | ✅ | <p>Notification configuration for the scheduled query. A notification is sent by
            Timestream when a query run finishes, when the state is updated or when you delete it.
        </p> |
| `error_report_configuration` | String | ✅ | <p>Configuration for error reporting. Error reports will be generated when a problem is
            encountered when writing the query results. </p> |
| `target_configuration` | String |  | <p>Configuration used for writing the result of a query.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `scheduled_query` | String | <p>The scheduled query.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create scheduled_query
scheduled_query = provider.timestream_query.Scheduled_query {
    query_string = "value"  # <p>The query string to run. Parameter names can be specified in the query string
                <code>@</code> character followed by an identifier. The named Parameter
                <code>@scheduled_runtime</code> is reserved and can be used in the query to get the
            time at which the query is scheduled to run.</p>
         <p>The timestamp calculated according to the ScheduleConfiguration parameter, will be the
            value of <code>@scheduled_runtime</code> paramater for each query run. For example,
            consider an instance of a scheduled query executing on 2021-12-01 00:00:00. For this
            instance, the <code>@scheduled_runtime</code> parameter is initialized to the timestamp
            2021-12-01 00:00:00 when invoking the query.</p>
    name = "value"  # <p>Name of the scheduled query.</p>
    schedule_configuration = "value"  # <p>The schedule configuration for the query.</p>
    scheduled_query_execution_role_arn = "value"  # <p>The ARN for the IAM role that Timestream will assume when running the scheduled query.
        </p>
    notification_configuration = "value"  # <p>Notification configuration for the scheduled query. A notification is sent by
            Timestream when a query run finishes, when the state is updated or when you delete it.
        </p>
    error_report_configuration = "value"  # <p>Configuration for error reporting. Error reports will be generated when a problem is
            encountered when writing the query results. </p>
}

# Access scheduled_query outputs
scheduled_query_id = scheduled_query.id
scheduled_query_scheduled_query = scheduled_query.scheduled_query
```

---


### Endpoints

Endpoints resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `endpoints` | Vec<String> | <p>An <code>Endpoints</code> object is returned when a <code>DescribeEndpoints</code>
            request is made.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access endpoints outputs
endpoints_id = endpoints.id
endpoints_endpoints = endpoints.endpoints
```

---


### Account_settings

AccountSettings resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `query_compute` | String |  | <p>Modifies the query compute settings configured in your account, including the query pricing model and provisioned Timestream Compute Units (TCUs) in your account.</p>
         <note>
            <p>This API is idempotent, meaning that making the same request multiple times will have the same effect as making the request once.</p>
         </note> |
| `max_query_tcu` | i64 |  | <p>The maximum number of compute units the service will use at any point in time to serve your queries. To run queries, you must set a minimum capacity of 4 TCU. You can set the maximum number of TCU in multiples of 4, for example, 4, 8, 16, 32, and so on. The maximum value supported for <code>MaxQueryTCU</code> is 1000. To request an increase to this soft limit, contact Amazon Web Services Support. For information about the default quota for maxQueryTCU, see Default quotas. This configuration is applicable only for on-demand usage of Timestream Compute Units (TCUs).</p>
         <p>The maximum value supported for <code>MaxQueryTCU</code> is 1000. To request an increase to this soft limit, contact Amazon Web Services Support. For information about the default quota for <code>maxQueryTCU</code>, see <a href="https://docs.aws.amazon.com/timestream/latest/developerguide/ts-limits.html#limits.default">Default quotas</a>.</p> |
| `query_pricing_model` | String |  | <p>The pricing model for queries in an account.</p>
         <note>
            <p>The <code>QueryPricingModel</code> parameter is used by several Timestream operations; however, the <code>UpdateAccountSettings</code> API operation doesn't recognize any values other than <code>COMPUTE_UNITS</code>.</p>
         </note> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `query_compute` | String | <p>An object that contains the usage settings for Timestream Compute Units (TCUs) in your account for the query workload. </p> |
| `max_query_tcu` | i64 | <p>The maximum number of <a href="https://docs.aws.amazon.com/timestream/latest/developerguide/tcu.html">Timestream compute units</a> (TCUs) the service will use at any point in time to serve your queries. To run queries, you must set a minimum capacity of 4 TCU. You can set the maximum number of TCU in multiples of 4, for example, 4, 8, 16, 32, and so on. This configuration is applicable only for on-demand usage of (TCUs). 
        
        
        </p> |
| `query_pricing_model` | String | <p>The pricing model for queries in your account.</p>
         <note>
            <p>The <code>QueryPricingModel</code> parameter is used by several Timestream operations; however, the <code>UpdateAccountSettings</code> API operation doesn't recognize any values other than <code>COMPUTE_UNITS</code>.</p>
         </note> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access account_settings outputs
account_settings_id = account_settings.id
account_settings_query_compute = account_settings.query_compute
account_settings_max_query_tcu = account_settings.max_query_tcu
account_settings_query_pricing_model = account_settings.query_pricing_model
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple scheduled_query resources
scheduled_query_0 = provider.timestream_query.Scheduled_query {
    query_string = "value-0"
    name = "value-0"
    schedule_configuration = "value-0"
    scheduled_query_execution_role_arn = "value-0"
    notification_configuration = "value-0"
    error_report_configuration = "value-0"
}
scheduled_query_1 = provider.timestream_query.Scheduled_query {
    query_string = "value-1"
    name = "value-1"
    schedule_configuration = "value-1"
    scheduled_query_execution_role_arn = "value-1"
    notification_configuration = "value-1"
    error_report_configuration = "value-1"
}
scheduled_query_2 = provider.timestream_query.Scheduled_query {
    query_string = "value-2"
    name = "value-2"
    schedule_configuration = "value-2"
    scheduled_query_execution_role_arn = "value-2"
    notification_configuration = "value-2"
    error_report_configuration = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    scheduled_query = provider.timestream_query.Scheduled_query {
        query_string = "production-value"
        name = "production-value"
        schedule_configuration = "production-value"
        scheduled_query_execution_role_arn = "production-value"
        notification_configuration = "production-value"
        error_report_configuration = "production-value"
    }
```

---

## Related Documentation

- [AWS Timestream_query Documentation](https://docs.aws.amazon.com/timestream_query/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
