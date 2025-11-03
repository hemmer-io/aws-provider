# Appsync Service



**Resources**: 16

---

## Overview

The appsync service provides access to 16 resource types:

- [Domain_name](#domain_name) [CRUD]
- [Resolver](#resolver) [CRUD]
- [Graphql_api_environment_variables](#graphql_api_environment_variables) [CR]
- [Channel_namespace](#channel_namespace) [CRUD]
- [Schema_creation_status](#schema_creation_status) [R]
- [Function](#function) [CRUD]
- [Type](#type) [CRUD]
- [Api_cache](#api_cache) [CRUD]
- [Api](#api) [CRUD]
- [Data_source_introspection](#data_source_introspection) [R]
- [Api_association](#api_association) [R]
- [Graphql_api](#graphql_api) [CRUD]
- [Api_key](#api_key) [CUD]
- [Data_source](#data_source) [CRUD]
- [Source_api_association](#source_api_association) [RU]
- [Introspection_schema](#introspection_schema) [R]

---

## Resources


### Domain_name

DomainName resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `certificate_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the certificate. This can be an Certificate Manager
            (ACM) certificate or an Identity and Access Management (IAM)
         server certificate.</p> |
| `tags` | HashMap<String, String> |  |  |
| `domain_name` | String | ✅ | <p>The domain name.</p> |
| `description` | String |  | <p>A description of the <code>DomainName</code>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `domain_name_config` | String | <p>The configuration for the <code>DomainName</code>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create domain_name
domain_name = provider.appsync.Domain_name {
    certificate_arn = "value"  # <p>The Amazon Resource Name (ARN) of the certificate. This can be an Certificate Manager
            (ACM) certificate or an Identity and Access Management (IAM)
         server certificate.</p>
    domain_name = "value"  # <p>The domain name.</p>
}

# Access domain_name outputs
domain_name_id = domain_name.id
domain_name_domain_name_config = domain_name.domain_name_config
```

---


### Resolver

Resolver resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `data_source_name` | String |  | <p>The name of the data source for which the resolver is being created.</p> |
| `caching_config` | String |  | <p>The caching configuration for the resolver.</p> |
| `type_name` | String | ✅ | <p>The name of the <code>Type</code>.</p> |
| `code` | String |  | <p>The <code>resolver</code> code that contains the request and response functions. When
         code is used, the <code>runtime</code> is required. The <code>runtime</code> value must be
            <code>APPSYNC_JS</code>.</p> |
| `kind` | String |  | <p>The resolver type.</p>
         <ul>
            <li>
               <p>
                  <b>UNIT</b>: A UNIT resolver type. A UNIT resolver is
               the default resolver type. You can use a UNIT resolver to run a GraphQL query against
               a single data source.</p>
            </li>
            <li>
               <p>
                  <b>PIPELINE</b>: A PIPELINE resolver type. You can
               use a PIPELINE resolver to invoke a series of <code>Function</code> objects in a
               serial manner. You can use a pipeline resolver to run a GraphQL query against
               multiple data sources.</p>
            </li>
         </ul> |
| `pipeline_config` | String |  | <p>The <code>PipelineConfig</code>.</p> |
| `max_batch_size` | i64 |  | <p>The maximum batching size for a resolver.</p> |
| `sync_config` | String |  | <p>The <code>SyncConfig</code> for a resolver attached to a versioned data source.</p> |
| `response_mapping_template` | String |  | <p>The mapping template to use for responses from the data source.</p> |
| `api_id` | String | ✅ | <p>The ID for the GraphQL API for which the resolver is being created.</p> |
| `request_mapping_template` | String |  | <p>The mapping template to use for requests.</p>
         <p>A resolver uses a request mapping template to convert a GraphQL expression into a format
         that a data source can understand. Mapping templates are written in Apache Velocity
         Template Language (VTL).</p>
         <p>VTL request mapping templates are optional when using an Lambda data
         source. For all other data sources, VTL request and response mapping templates are
         required.</p> |
| `runtime` | String |  |  |
| `metrics_config` | String |  | <p>Enables or disables enhanced resolver metrics for specified resolvers. Note that
            <code>metricsConfig</code> won't be used unless the
            <code>resolverLevelMetricsBehavior</code> value is set to
            <code>PER_RESOLVER_METRICS</code>. If the <code>resolverLevelMetricsBehavior</code> is
         set to <code>FULL_REQUEST_RESOLVER_METRICS</code> instead, <code>metricsConfig</code> will
         be ignored. However, you can still set its value.</p>
         <p>
            <code>metricsConfig</code> can be <code>ENABLED</code> or <code>DISABLED</code>.</p> |
| `field_name` | String | ✅ | <p>The name of the field to attach the resolver to.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `resolver` | String | <p>The <code>Resolver</code> object.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create resolver
resolver = provider.appsync.Resolver {
    type_name = "value"  # <p>The name of the <code>Type</code>.</p>
    api_id = "value"  # <p>The ID for the GraphQL API for which the resolver is being created.</p>
    field_name = "value"  # <p>The name of the field to attach the resolver to.</p>
}

# Access resolver outputs
resolver_id = resolver.id
resolver_resolver = resolver.resolver
```

---


### Graphql_api_environment_variables

GraphqlApiEnvironmentVariables resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `api_id` | String | ✅ | <p>The ID of the API to which the environmental variable list will be written.</p> |
| `environment_variables` | HashMap<String, String> | ✅ | <p>The list of environmental variables to add to the API.</p>
         <p>When creating an environmental variable key-value pair, it must follow the additional
         constraints below:</p>
         <ul>
            <li>
               <p>Keys must begin with a letter.</p>
            </li>
            <li>
               <p>Keys must be at least two characters long.</p>
            </li>
            <li>
               <p>Keys can only contain letters, numbers, and the underscore character
               (_).</p>
            </li>
            <li>
               <p>Values can be up to 512 characters long.</p>
            </li>
            <li>
               <p>You can configure up to 50 key-value pairs in a GraphQL API.</p>
            </li>
         </ul>
         <p>You can create a list of environmental variables by adding it to the
            <code>environmentVariables</code> payload as a list in the format
            <code>{"key1":"value1","key2":"value2", …}</code>. Note that each call of the
            <code>PutGraphqlApiEnvironmentVariables</code> action will result in the overwriting of
         the existing environmental variable list of that API. This means the existing environmental
         variables will be lost. To avoid this, you must include all existing and new environmental
         variables in the list each time you call this action.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `environment_variables` | HashMap<String, String> | <p>The payload containing each environmental variable in the <code>"key" : "value"</code>
         format.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create graphql_api_environment_variables
graphql_api_environment_variables = provider.appsync.Graphql_api_environment_variables {
    api_id = "value"  # <p>The ID of the API to which the environmental variable list will be written.</p>
    environment_variables = "value"  # <p>The list of environmental variables to add to the API.</p>
         <p>When creating an environmental variable key-value pair, it must follow the additional
         constraints below:</p>
         <ul>
            <li>
               <p>Keys must begin with a letter.</p>
            </li>
            <li>
               <p>Keys must be at least two characters long.</p>
            </li>
            <li>
               <p>Keys can only contain letters, numbers, and the underscore character
               (_).</p>
            </li>
            <li>
               <p>Values can be up to 512 characters long.</p>
            </li>
            <li>
               <p>You can configure up to 50 key-value pairs in a GraphQL API.</p>
            </li>
         </ul>
         <p>You can create a list of environmental variables by adding it to the
            <code>environmentVariables</code> payload as a list in the format
            <code>{"key1":"value1","key2":"value2", …}</code>. Note that each call of the
            <code>PutGraphqlApiEnvironmentVariables</code> action will result in the overwriting of
         the existing environmental variable list of that API. This means the existing environmental
         variables will be lost. To avoid this, you must include all existing and new environmental
         variables in the list each time you call this action.</p>
}

# Access graphql_api_environment_variables outputs
graphql_api_environment_variables_id = graphql_api_environment_variables.id
graphql_api_environment_variables_environment_variables = graphql_api_environment_variables.environment_variables
```

---


### Channel_namespace

ChannelNamespace resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>The name of the <code>ChannelNamespace</code>. This name must be unique within the
            <code>Api</code>
         </p> |
| `publish_auth_modes` | Vec<String> |  | <p>The authorization mode to use for publishing messages on the channel namespace. This
         configuration overrides the default <code>Api</code> authorization configuration.</p> |
| `subscribe_auth_modes` | Vec<String> |  | <p>The authorization mode to use for subscribing to messages on the channel namespace. This
         configuration overrides the default <code>Api</code> authorization configuration.</p> |
| `handler_configs` | String |  | <p>The configuration for the <code>OnPublish</code> and <code>OnSubscribe</code> handlers.</p> |
| `code_handlers` | String |  | <p>The event handler functions that run custom business logic to process published events
         and subscribe requests.</p> |
| `tags` | HashMap<String, String> |  |  |
| `api_id` | String | ✅ | <p>The <code>Api</code> ID.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `channel_namespace` | String | <p>The <code>ChannelNamespace</code> object.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create channel_namespace
channel_namespace = provider.appsync.Channel_namespace {
    name = "value"  # <p>The name of the <code>ChannelNamespace</code>. This name must be unique within the
            <code>Api</code>
         </p>
    api_id = "value"  # <p>The <code>Api</code> ID.</p>
}

# Access channel_namespace outputs
channel_namespace_id = channel_namespace.id
channel_namespace_channel_namespace = channel_namespace.channel_namespace
```

---


### Schema_creation_status

SchemaCreationStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `details` | String | <p>Detailed information about the status of the schema creation operation.</p> |
| `status` | String | <p>The current state of the schema (PROCESSING, FAILED, SUCCESS, or NOT_APPLICABLE). When
         the schema is in the ACTIVE state, you can add data.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access schema_creation_status outputs
schema_creation_status_id = schema_creation_status.id
schema_creation_status_details = schema_creation_status.details
schema_creation_status_status = schema_creation_status.status
```

---


### Function

Function resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | <p>The <code>Function</code> description.</p> |
| `code` | String |  | <p>The <code>function</code> code that contains the request and response functions. When
         code is used, the <code>runtime</code> is required. The <code>runtime</code> value must be
            <code>APPSYNC_JS</code>.</p> |
| `response_mapping_template` | String |  | <p>The <code>Function</code> response mapping template.</p> |
| `request_mapping_template` | String |  | <p>The <code>Function</code> request mapping template. Functions support only the
         2018-05-29 version of the request mapping template.</p> |
| `runtime` | String |  |  |
| `api_id` | String | ✅ | <p>The GraphQL API ID.</p> |
| `function_version` | String |  | <p>The <code>version</code> of the request mapping template. Currently, the supported value
         is 2018-05-29. Note that when using VTL and mapping templates, the
            <code>functionVersion</code> is required.</p> |
| `max_batch_size` | i64 |  | <p>The maximum batching size for a resolver.</p> |
| `data_source_name` | String | ✅ | <p>The <code>Function</code>
            <code>DataSource</code> name.</p> |
| `sync_config` | String |  |  |
| `name` | String | ✅ | <p>The <code>Function</code> name. The function name does not have to be unique.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `function_configuration` | String | <p>The <code>Function</code> object.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create function
function = provider.appsync.Function {
    api_id = "value"  # <p>The GraphQL API ID.</p>
    data_source_name = "value"  # <p>The <code>Function</code>
            <code>DataSource</code> name.</p>
    name = "value"  # <p>The <code>Function</code> name. The function name does not have to be unique.</p>
}

# Access function outputs
function_id = function.id
function_function_configuration = function.function_configuration
```

---


### Type

Type resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `api_id` | String | ✅ | <p>The API ID.</p> |
| `definition` | String | ✅ | <p>The type definition, in GraphQL Schema Definition Language (SDL) format.</p>
         <p>For more information, see the <a href="http://graphql.org/learn/schema/">GraphQL SDL
            documentation</a>.</p> |
| `format` | String | ✅ | <p>The type format: SDL or JSON.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `type` | String | <p>The <code>Type</code> object.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create type
type = provider.appsync.Type {
    api_id = "value"  # <p>The API ID.</p>
    definition = "value"  # <p>The type definition, in GraphQL Schema Definition Language (SDL) format.</p>
         <p>For more information, see the <a href="http://graphql.org/learn/schema/">GraphQL SDL
            documentation</a>.</p>
    format = "value"  # <p>The type format: SDL or JSON.</p>
}

# Access type outputs
type_id = type.id
type_type = type.type
```

---


### Api_cache

ApiCache resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `transit_encryption_enabled` | bool |  | <p>Transit encryption flag when connecting to cache. You cannot update this setting after
         creation.</p> |
| `api_id` | String | ✅ | <p>The GraphQL API ID.</p> |
| `type` | String | ✅ | <p>The cache instance type. Valid values are </p>
         <ul>
            <li>
               <p>
                  <code>SMALL</code>
               </p>
            </li>
            <li>
               <p>
                  <code>MEDIUM</code>
               </p>
            </li>
            <li>
               <p>
                  <code>LARGE</code>
               </p>
            </li>
            <li>
               <p>
                  <code>XLARGE</code>
               </p>
            </li>
            <li>
               <p>
                  <code>LARGE_2X</code>
               </p>
            </li>
            <li>
               <p>
                  <code>LARGE_4X</code>
               </p>
            </li>
            <li>
               <p>
                  <code>LARGE_8X</code> (not available in all regions)</p>
            </li>
            <li>
               <p>
                  <code>LARGE_12X</code>
               </p>
            </li>
         </ul>
         <p>Historically, instance types were identified by an EC2-style value. As of July 2020, this is deprecated, and the generic identifiers above should be used.</p>
         <p>The following legacy instance types are available, but their use is discouraged:</p>
         <ul>
            <li>
               <p>
                  <b>T2_SMALL</b>: A t2.small instance type.</p>
            </li>
            <li>
               <p>
                  <b>T2_MEDIUM</b>: A t2.medium instance type.</p>
            </li>
            <li>
               <p>
                  <b>R4_LARGE</b>: A r4.large instance type.</p>
            </li>
            <li>
               <p>
                  <b>R4_XLARGE</b>: A r4.xlarge instance type.</p>
            </li>
            <li>
               <p>
                  <b>R4_2XLARGE</b>: A r4.2xlarge instance type.</p>
            </li>
            <li>
               <p>
                  <b>R4_4XLARGE</b>: A r4.4xlarge instance type.</p>
            </li>
            <li>
               <p>
                  <b>R4_8XLARGE</b>: A r4.8xlarge instance type.</p>
            </li>
         </ul> |
| `ttl` | i64 | ✅ | <p>TTL in seconds for cache entries.</p>
         <p>Valid values are 1–3,600 seconds.</p> |
| `at_rest_encryption_enabled` | bool |  | <p>At-rest encryption flag for cache. You cannot update this setting after creation.</p> |
| `api_caching_behavior` | String | ✅ | <p>Caching behavior.</p>
         <ul>
            <li>
               <p>
                  <b>FULL_REQUEST_CACHING</b>: All requests from the
               same user are cached. Individual resolvers are automatically cached. All API calls
               will try to return responses from the cache.</p>
            </li>
            <li>
               <p>
                  <b>PER_RESOLVER_CACHING</b>: Individual resolvers
               that you specify are cached.</p>
            </li>
            <li>
               <p>
                  <b>OPERATION_LEVEL_CACHING</b>: Full requests are cached together and returned without executing resolvers.</p>
            </li>
         </ul> |
| `health_metrics_config` | String |  | <p>Controls how cache health metrics will be emitted to CloudWatch. Cache health metrics
         include:</p>
         <ul>
            <li>
               <p>NetworkBandwidthOutAllowanceExceeded: The network packets dropped because the
               throughput exceeded the aggregated bandwidth limit. This is useful for diagnosing
               bottlenecks in a cache configuration.</p>
            </li>
            <li>
               <p>EngineCPUUtilization: The CPU utilization (percentage) allocated to the Redis
               process. This is useful for diagnosing bottlenecks in a cache
               configuration.</p>
            </li>
         </ul>
         <p>Metrics will be recorded by API ID. You can set the value to <code>ENABLED</code> or
            <code>DISABLED</code>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `api_cache` | String | <p>The <code>ApiCache</code> object.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create api_cache
api_cache = provider.appsync.Api_cache {
    api_id = "value"  # <p>The GraphQL API ID.</p>
    type = "value"  # <p>The cache instance type. Valid values are </p>
         <ul>
            <li>
               <p>
                  <code>SMALL</code>
               </p>
            </li>
            <li>
               <p>
                  <code>MEDIUM</code>
               </p>
            </li>
            <li>
               <p>
                  <code>LARGE</code>
               </p>
            </li>
            <li>
               <p>
                  <code>XLARGE</code>
               </p>
            </li>
            <li>
               <p>
                  <code>LARGE_2X</code>
               </p>
            </li>
            <li>
               <p>
                  <code>LARGE_4X</code>
               </p>
            </li>
            <li>
               <p>
                  <code>LARGE_8X</code> (not available in all regions)</p>
            </li>
            <li>
               <p>
                  <code>LARGE_12X</code>
               </p>
            </li>
         </ul>
         <p>Historically, instance types were identified by an EC2-style value. As of July 2020, this is deprecated, and the generic identifiers above should be used.</p>
         <p>The following legacy instance types are available, but their use is discouraged:</p>
         <ul>
            <li>
               <p>
                  <b>T2_SMALL</b>: A t2.small instance type.</p>
            </li>
            <li>
               <p>
                  <b>T2_MEDIUM</b>: A t2.medium instance type.</p>
            </li>
            <li>
               <p>
                  <b>R4_LARGE</b>: A r4.large instance type.</p>
            </li>
            <li>
               <p>
                  <b>R4_XLARGE</b>: A r4.xlarge instance type.</p>
            </li>
            <li>
               <p>
                  <b>R4_2XLARGE</b>: A r4.2xlarge instance type.</p>
            </li>
            <li>
               <p>
                  <b>R4_4XLARGE</b>: A r4.4xlarge instance type.</p>
            </li>
            <li>
               <p>
                  <b>R4_8XLARGE</b>: A r4.8xlarge instance type.</p>
            </li>
         </ul>
    ttl = "value"  # <p>TTL in seconds for cache entries.</p>
         <p>Valid values are 1–3,600 seconds.</p>
    api_caching_behavior = "value"  # <p>Caching behavior.</p>
         <ul>
            <li>
               <p>
                  <b>FULL_REQUEST_CACHING</b>: All requests from the
               same user are cached. Individual resolvers are automatically cached. All API calls
               will try to return responses from the cache.</p>
            </li>
            <li>
               <p>
                  <b>PER_RESOLVER_CACHING</b>: Individual resolvers
               that you specify are cached.</p>
            </li>
            <li>
               <p>
                  <b>OPERATION_LEVEL_CACHING</b>: Full requests are cached together and returned without executing resolvers.</p>
            </li>
         </ul>
}

# Access api_cache outputs
api_cache_id = api_cache.id
api_cache_api_cache = api_cache.api_cache
```

---


### Api

Api resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `event_config` | String |  | <p>The Event API configuration. This includes the default authorization configuration for
         connecting, publishing, and subscribing to an Event API.</p> |
| `owner_contact` | String |  | <p>The owner contact information for the <code>Api</code>.</p> |
| `name` | String | ✅ | <p>The name for the <code>Api</code>.</p> |
| `tags` | HashMap<String, String> |  |  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `api` | String | <p>The <code>Api</code> object.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create api
api = provider.appsync.Api {
    name = "value"  # <p>The name for the <code>Api</code>.</p>
}

# Access api outputs
api_id = api.id
api_api = api.api
```

---


### Data_source_introspection

DataSourceIntrospection resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `introspection_result` | String | <p>The <code>DataSourceIntrospectionResult</code> object data.</p> |
| `introspection_id` | String | <p>The introspection ID. Each introspection contains a unique ID that can be used to
         reference the instrospection record.</p> |
| `introspection_status` | String | <p>The status of the introspection during retrieval. By default, when a new instrospection
         is being retrieved, the status will be set to <code>PROCESSING</code>. Once the operation
         has been completed, the status will change to <code>SUCCESS</code> or <code>FAILED</code>
         depending on how the data was parsed. A <code>FAILED</code> operation will return an error
         and its details as an <code>introspectionStatusDetail</code>.</p> |
| `introspection_status_detail` | String | <p>The error detail field. When a <code>FAILED</code>
            <code>introspectionStatus</code> is returned, the <code>introspectionStatusDetail</code>
         will also return the exact error that was generated during the operation.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access data_source_introspection outputs
data_source_introspection_id = data_source_introspection.id
data_source_introspection_introspection_result = data_source_introspection.introspection_result
data_source_introspection_introspection_id = data_source_introspection.introspection_id
data_source_introspection_introspection_status = data_source_introspection.introspection_status
data_source_introspection_introspection_status_detail = data_source_introspection.introspection_status_detail
```

---


### Api_association

ApiAssociation resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `api_association` | String | <p>The <code>ApiAssociation</code> object.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access api_association outputs
api_association_id = api_association.id
api_association_api_association = api_association.api_association
```

---


### Graphql_api

GraphqlApi resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>A user-supplied name for the <code>GraphqlApi</code>.</p> |
| `open_id_connect_config` | String |  | <p>The OIDC configuration.</p> |
| `merged_api_execution_role_arn` | String |  | <p>The Identity and Access Management service role ARN for a merged API. The AppSync
         service assumes this role on behalf of the Merged API to validate access to source APIs at
         runtime and to prompt the <code>AUTO_MERGE</code> to update the merged API endpoint with
         the source API changes automatically.</p> |
| `xray_enabled` | bool |  | <p>A flag indicating whether to use X-Ray tracing for the
            <code>GraphqlApi</code>.</p> |
| `introspection_config` | String |  | <p>Sets the value of the GraphQL API to enable (<code>ENABLED</code>) or disable
            (<code>DISABLED</code>) introspection. If no value is provided, the introspection
         configuration will be set to <code>ENABLED</code> by default. This field will produce an
         error if the operation attempts to use the introspection feature while this field is
         disabled.</p>
         <p>For more information about introspection, see <a href="https://graphql.org/learn/introspection/">GraphQL introspection</a>.</p> |
| `resolver_count_limit` | i64 |  | <p>The maximum number of resolvers that can be invoked in a single request. The default
         value is <code>0</code> (or unspecified), which will set the limit to <code>10000</code>.
         When specified, the limit value can be between <code>1</code> and <code>10000</code>. This
         field will produce a limit error if the operation falls out of bounds.</p> |
| `log_config` | String |  | <p>The Amazon CloudWatch Logs configuration.</p> |
| `additional_authentication_providers` | Vec<String> |  | <p>A list of additional authentication providers for the <code>GraphqlApi</code>
         API.</p> |
| `owner_contact` | String |  | <p>The owner contact information for an API resource.</p>
         <p>This field accepts any string input with a length of 0 - 256 characters.</p> |
| `api_type` | String |  | <p>The value that indicates whether the GraphQL API is a standard API
         (<code>GRAPHQL</code>) or merged API (<code>MERGED</code>).</p> |
| `visibility` | String |  | <p>Sets the value of the GraphQL API to public (<code>GLOBAL</code>) or private
            (<code>PRIVATE</code>). If no value is provided, the visibility will be set to
            <code>GLOBAL</code> by default. This value cannot be changed once the API has been
         created.</p> |
| `authentication_type` | String | ✅ | <p>The authentication type: API key, Identity and Access Management (IAM), OpenID
         Connect (OIDC), Amazon Cognito user pools, or Lambda.</p> |
| `query_depth_limit` | i64 |  | <p>The maximum depth a query can have in a single request. Depth refers to the amount of
         nested levels allowed in the body of query. The default value is <code>0</code> (or
         unspecified), which indicates there's no depth limit. If you set a limit, it can be between
            <code>1</code> and <code>75</code> nested levels. This field will produce a limit error
         if the operation falls out of bounds.</p>
         <p>Note that fields can still be set to nullable or non-nullable. If a non-nullable field
         produces an error, the error will be thrown upwards to the first nullable field
         available.</p> |
| `tags` | HashMap<String, String> |  | <p>A <code>TagMap</code> object.</p> |
| `enhanced_metrics_config` | String |  | <p>The <code>enhancedMetricsConfig</code> object.</p> |
| `user_pool_config` | String |  | <p>The Amazon Cognito user pool configuration.</p> |
| `lambda_authorizer_config` | String |  | <p>Configuration for Lambda function authorization.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `graphql_api` | String | <p>The <code>GraphqlApi</code> object.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create graphql_api
graphql_api = provider.appsync.Graphql_api {
    name = "value"  # <p>A user-supplied name for the <code>GraphqlApi</code>.</p>
    authentication_type = "value"  # <p>The authentication type: API key, Identity and Access Management (IAM), OpenID
         Connect (OIDC), Amazon Cognito user pools, or Lambda.</p>
}

# Access graphql_api outputs
graphql_api_id = graphql_api.id
graphql_api_graphql_api = graphql_api.graphql_api
```

---


### Api_key

ApiKey resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `api_id` | String | ✅ | <p>The ID for your GraphQL API.</p> |
| `expires` | i64 |  | <p>From the creation time, the time after which the API key expires. The date is
         represented as seconds since the epoch, rounded down to the nearest hour. The default value
         for this parameter is 7 days from creation time. For more information, see .</p> |
| `description` | String |  | <p>A description of the purpose of the API key.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create api_key
api_key = provider.appsync.Api_key {
    api_id = "value"  # <p>The ID for your GraphQL API.</p>
}

```

---


### Data_source

DataSource resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `elasticsearch_config` | String |  | <p>Amazon OpenSearch Service settings.</p>
         <p>As of September 2021, Amazon Elasticsearch service is Amazon OpenSearch Service. This
         configuration is deprecated. For new data sources, use <a>CreateDataSourceRequest$openSearchServiceConfig</a> to create an OpenSearch data source.</p> |
| `name` | String | ✅ | <p>A user-supplied name for the <code>DataSource</code>.</p> |
| `type` | String | ✅ | <p>The type of the <code>DataSource</code>.</p> |
| `lambda_config` | String |  | <p>Lambda settings.</p> |
| `service_role_arn` | String |  | <p>The Identity and Access Management (IAM) service role Amazon Resource Name (ARN)
         for the data source. The system assumes this role when accessing the data source.</p> |
| `open_search_service_config` | String |  | <p>Amazon OpenSearch Service settings.</p> |
| `relational_database_config` | String |  | <p>Relational database settings.</p> |
| `metrics_config` | String |  | <p>Enables or disables enhanced data source metrics for specified data sources. Note that
            <code>metricsConfig</code> won't be used unless the
            <code>dataSourceLevelMetricsBehavior</code> value is set to
            <code>PER_DATA_SOURCE_METRICS</code>. If the <code>dataSourceLevelMetricsBehavior</code>
         is set to <code>FULL_REQUEST_DATA_SOURCE_METRICS</code> instead, <code>metricsConfig</code>
         will be ignored. However, you can still set its value.</p>
         <p>
            <code>metricsConfig</code> can be <code>ENABLED</code> or <code>DISABLED</code>.</p> |
| `api_id` | String | ✅ | <p>The API ID for the GraphQL API for the <code>DataSource</code>.</p> |
| `http_config` | String |  | <p>HTTP endpoint settings.</p> |
| `dynamodb_config` | String |  | <p>Amazon DynamoDB settings.</p> |
| `description` | String |  | <p>A description of the <code>DataSource</code>.</p> |
| `event_bridge_config` | String |  | <p>Amazon EventBridge settings.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `data_source` | String | <p>The <code>DataSource</code> object.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create data_source
data_source = provider.appsync.Data_source {
    name = "value"  # <p>A user-supplied name for the <code>DataSource</code>.</p>
    type = "value"  # <p>The type of the <code>DataSource</code>.</p>
    api_id = "value"  # <p>The API ID for the GraphQL API for the <code>DataSource</code>.</p>
}

# Access data_source outputs
data_source_id = data_source.id
data_source_data_source = data_source.data_source
```

---


### Source_api_association

SourceApiAssociation resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `merged_api_identifier` | String | ✅ | <p>The identifier of the AppSync Merged API. This is generated by the AppSync service. In
         most cases, Merged APIs (especially in your account) only require the API ID value or ARN
         of the merged API. However, Merged APIs in other accounts (cross-account use cases)
         strictly require the full resource ARN of the merged API.</p> |
| `source_api_association_config` | String |  | <p>The <code>SourceApiAssociationConfig</code> object data.</p> |
| `description` | String |  | <p>The description field.</p> |
| `association_id` | String | ✅ | <p>The ID generated by the AppSync service for the source API association.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `source_api_association` | String | <p>The <code>SourceApiAssociation</code> object data.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access source_api_association outputs
source_api_association_id = source_api_association.id
source_api_association_source_api_association = source_api_association.source_api_association
```

---


### Introspection_schema

IntrospectionSchema resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `schema` | String | <p>The schema, in GraphQL Schema Definition Language (SDL) format.</p>
         <p>For more information, see the <a href="http://graphql.org/learn/schema/">GraphQL SDL
            documentation</a>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access introspection_schema outputs
introspection_schema_id = introspection_schema.id
introspection_schema_schema = introspection_schema.schema
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple domain_name resources
domain_name_0 = provider.appsync.Domain_name {
    certificate_arn = "value-0"
    domain_name = "value-0"
}
domain_name_1 = provider.appsync.Domain_name {
    certificate_arn = "value-1"
    domain_name = "value-1"
}
domain_name_2 = provider.appsync.Domain_name {
    certificate_arn = "value-2"
    domain_name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    domain_name = provider.appsync.Domain_name {
        certificate_arn = "production-value"
        domain_name = "production-value"
    }
```

---

## Related Documentation

- [AWS Appsync Documentation](https://docs.aws.amazon.com/appsync/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
