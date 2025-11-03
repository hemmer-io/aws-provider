# Migration_hub_refactor_spaces Service



**Resources**: 5

---

## Overview

The migration_hub_refactor_spaces service provides access to 5 resource types:

- [Resource_policy](#resource_policy) [CRD]
- [Route](#route) [CRUD]
- [Environment](#environment) [CRD]
- [Application](#application) [CRD]
- [Service](#service) [CRD]

---

## Resources


### Resource_policy

ResourcePolicy resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resource_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the resource to which the policy is being attached.
    </p> |
| `policy` | String | ✅ | <p>A JSON-formatted string for an Amazon Web Services resource-based policy. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `policy` | String | <p>A JSON-formatted string for an Amazon Web Services resource-based policy. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create resource_policy
resource_policy = provider.migration_hub_refactor_spaces.Resource_policy {
    resource_arn = "value"  # <p>The Amazon Resource Name (ARN) of the resource to which the policy is being attached.
    </p>
    policy = "value"  # <p>A JSON-formatted string for an Amazon Web Services resource-based policy. </p>
}

# Access resource_policy outputs
resource_policy_id = resource_policy.id
resource_policy_policy = resource_policy.policy
```

---


### Route

Route resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | HashMap<String, String> |  | <p>The tags to assign to the route. A tag is a label that you assign to an Amazon Web Services resource. Each tag consists of a key-value pair.. </p> |
| `default_route` | String |  | <p> Configuration for the default route type. </p> |
| `environment_identifier` | String | ✅ | <p>The ID of the environment in which the route is created.</p> |
| `application_identifier` | String | ✅ | <p>The ID of the application within which the route is being created.</p> |
| `route_type` | String | ✅ | <p>The route type of the route. <code>DEFAULT</code> indicates that all traffic that does not
      match another route is forwarded to the default route. Applications must have a default route
      before any other routes can be created. <code>URI_PATH</code> indicates a route that is based
      on a URI path.</p> |
| `client_token` | String |  | <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the
      request.</p> |
| `service_identifier` | String | ✅ | <p>The ID of the service in which the route is created. Traffic that matches this route is
      forwarded to this service.</p> |
| `uri_path_route` | String |  | <p>The configuration for the URI path route type. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `include_child_paths` | bool | <p>Indicates whether to match all subpaths of the given source path. If this value is
        <code>false</code>, requests must match the source path exactly before they are forwarded to
      this route's service. </p> |
| `tags` | HashMap<String, String> | <p>The tags assigned to the route. A tag is a label that you assign to an Amazon Web Services resource. Each tag consists of a key-value pair. </p> |
| `route_type` | String | <p>The type of route.</p> |
| `methods` | Vec<String> | <p>A list of HTTP methods to match. An empty list matches all values. If a method is present,
      only HTTP requests using that method are forwarded to this route’s service. </p> |
| `state` | String | <p>The current state of the route. </p> |
| `application_id` | String | <p>The ID of the application that the route belongs to. </p> |
| `owner_account_id` | String | <p>The Amazon Web Services account ID of the route owner.</p> |
| `error` | String | <p>Any error associated with the route resource. </p> |
| `created_time` | String | <p>The timestamp of when the route is created. </p> |
| `last_updated_time` | String | <p>A timestamp that indicates when the route was last updated. </p> |
| `service_id` | String | <p>The unique identifier of the service.</p> |
| `source_path` | String | <p>This is the path that Refactor Spaces uses to match traffic. Paths must start with <code>/</code> and are relative to
      the base of the application. To use path parameters in the source path, add a variable in curly braces. 
      For example, the resource path {user} represents a path parameter called 'user'.</p> |
| `append_source_path` | bool | <p>If set to <code>true</code>, this option appends the source path to the service URL endpoint.</p> |
| `path_resource_to_id` | HashMap<String, String> | <p>A mapping of Amazon API Gateway path resources to resource IDs. </p> |
| `route_id` | String | <p>The unique identifier of the route.</p>
         <p>
            <b>DEFAULT</b>: All traffic that does not match another route is
      forwarded to the default route. Applications must have a default route before any other routes
      can be created.</p>
         <p>
            <b>URI_PATH</b>: A route that is based on a URI path.</p> |
| `environment_id` | String | <p>Unique identifier of the environment.</p> |
| `created_by_account_id` | String | <p>The Amazon Web Services account ID of the route creator.</p> |
| `arn` | String | <p>The Amazon Resource Name (ARN) of the route.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create route
route = provider.migration_hub_refactor_spaces.Route {
    environment_identifier = "value"  # <p>The ID of the environment in which the route is created.</p>
    application_identifier = "value"  # <p>The ID of the application within which the route is being created.</p>
    route_type = "value"  # <p>The route type of the route. <code>DEFAULT</code> indicates that all traffic that does not
      match another route is forwarded to the default route. Applications must have a default route
      before any other routes can be created. <code>URI_PATH</code> indicates a route that is based
      on a URI path.</p>
    service_identifier = "value"  # <p>The ID of the service in which the route is created. Traffic that matches this route is
      forwarded to this service.</p>
}

# Access route outputs
route_id = route.id
route_include_child_paths = route.include_child_paths
route_tags = route.tags
route_route_type = route.route_type
route_methods = route.methods
route_state = route.state
route_application_id = route.application_id
route_owner_account_id = route.owner_account_id
route_error = route.error
route_created_time = route.created_time
route_last_updated_time = route.last_updated_time
route_service_id = route.service_id
route_source_path = route.source_path
route_append_source_path = route.append_source_path
route_path_resource_to_id = route.path_resource_to_id
route_route_id = route.route_id
route_environment_id = route.environment_id
route_created_by_account_id = route.created_by_account_id
route_arn = route.arn
```

---


### Environment

Environment resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_token` | String |  | <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the
      request.</p> |
| `network_fabric_type` | String | ✅ | <p>The network fabric type of the environment.</p> |
| `description` | String |  | <p>The description of the environment.</p> |
| `name` | String | ✅ | <p>The name of the environment.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags to assign to the environment. A tag is a label that you assign to an Amazon Web Services resource. Each tag consists of a key-value pair.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `environment_id` | String | <p>The unique identifier of the environment. </p> |
| `network_fabric_type` | String | <p>The network fabric type of the environment. </p> |
| `last_updated_time` | String | <p>A timestamp that indicates when the environment was last updated. </p> |
| `created_time` | String | <p>A timestamp that indicates when the environment is created. </p> |
| `owner_account_id` | String | <p>The Amazon Web Services account ID of the environment owner.</p> |
| `state` | String | <p>The current state of the environment. </p> |
| `error` | String | <p>Any error associated with the environment resource. </p> |
| `name` | String | <p>The name of the environment.</p> |
| `arn` | String | <p>The Amazon Resource Name (ARN) of the environment.</p> |
| `description` | String | <p>The description of the environment. </p> |
| `transit_gateway_id` | String | <p>The ID of the Transit Gateway set up by the environment, if applicable.</p> |
| `tags` | HashMap<String, String> | <p>The tags to assign to the environment. A tag is a label that you assign to an Amazon Web Services resource. Each tag consists of a key-value pair. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create environment
environment = provider.migration_hub_refactor_spaces.Environment {
    network_fabric_type = "value"  # <p>The network fabric type of the environment.</p>
    name = "value"  # <p>The name of the environment.</p>
}

# Access environment outputs
environment_id = environment.id
environment_environment_id = environment.environment_id
environment_network_fabric_type = environment.network_fabric_type
environment_last_updated_time = environment.last_updated_time
environment_created_time = environment.created_time
environment_owner_account_id = environment.owner_account_id
environment_state = environment.state
environment_error = environment.error
environment_name = environment.name
environment_arn = environment.arn
environment_description = environment.description
environment_transit_gateway_id = environment.transit_gateway_id
environment_tags = environment.tags
```

---


### Application

Application resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `proxy_type` | String | ✅ | <p>The proxy type of the proxy created within the application. </p> |
| `vpc_id` | String | ✅ | <p>The ID of the virtual private cloud (VPC).</p> |
| `environment_identifier` | String | ✅ | <p>The unique identifier of the environment.</p> |
| `client_token` | String |  | <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the
      request.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags to assign to the application. A tag is a label that you assign to an Amazon Web Services resource. Each tag consists of a key-value pair.</p> |
| `api_gateway_proxy` | String |  | <p>A wrapper object holding the API Gateway endpoint type and stage name for the
      proxy. </p> |
| `name` | String | ✅ | <p>The name to use for the application. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `error` | String | <p>Any error associated with the application resource. </p> |
| `last_updated_time` | String | <p>A timestamp that indicates when the application was last updated. </p> |
| `api_gateway_proxy` | String | <p>The endpoint URL of the API Gateway proxy. </p> |
| `vpc_id` | String | <p>The ID of the virtual private cloud (VPC). </p> |
| `arn` | String | <p>The Amazon Resource Name (ARN) of the application.</p> |
| `created_time` | String | <p>A timestamp that indicates when the application is created. </p> |
| `tags` | HashMap<String, String> | <p>The tags assigned to the application. A tag is a label that you assign to an Amazon Web Services resource. Each tag consists of a key-value pair. </p> |
| `created_by_account_id` | String | <p>The Amazon Web Services account ID of the application creator. </p> |
| `application_id` | String | <p>The unique identifier of the application.</p> |
| `state` | String | <p>The current state of the application. </p> |
| `environment_id` | String | <p>The unique identifier of the environment.</p> |
| `name` | String | <p>The name of the application.</p> |
| `owner_account_id` | String | <p>The Amazon Web Services account ID of the application owner (which is always the same as
      the environment owner account ID).</p> |
| `proxy_type` | String | <p>The proxy type of the proxy created within the application. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create application
application = provider.migration_hub_refactor_spaces.Application {
    proxy_type = "value"  # <p>The proxy type of the proxy created within the application. </p>
    vpc_id = "value"  # <p>The ID of the virtual private cloud (VPC).</p>
    environment_identifier = "value"  # <p>The unique identifier of the environment.</p>
    name = "value"  # <p>The name to use for the application. </p>
}

# Access application outputs
application_id = application.id
application_error = application.error
application_last_updated_time = application.last_updated_time
application_api_gateway_proxy = application.api_gateway_proxy
application_vpc_id = application.vpc_id
application_arn = application.arn
application_created_time = application.created_time
application_tags = application.tags
application_created_by_account_id = application.created_by_account_id
application_application_id = application.application_id
application_state = application.state
application_environment_id = application.environment_id
application_name = application.name
application_owner_account_id = application.owner_account_id
application_proxy_type = application.proxy_type
```

---


### Service

Service resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>The name of the service.</p> |
| `endpoint_type` | String | ✅ | <p>The type of endpoint to use for the service. The type can be a URL in a VPC or an Lambda function.</p> |
| `application_identifier` | String | ✅ | <p>The ID of the application which the service is created.</p> |
| `url_endpoint` | String |  | <p>The configuration for the URL endpoint type. When creating a route to a service, Refactor Spaces
      automatically resolves the address in the <code>UrlEndpointInput</code> object URL when the
      Domain Name System (DNS) time-to-live (TTL) expires, or every 60 seconds for TTLs less than 60
      seconds.</p> |
| `description` | String |  | <p>The description of the service.</p> |
| `lambda_endpoint` | String |  | <p>The configuration for the Lambda endpoint type.</p> |
| `client_token` | String |  | <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the
      request.</p> |
| `vpc_id` | String |  | <p>The ID of the VPC.</p> |
| `environment_identifier` | String | ✅ | <p>The ID of the environment in which the service is created.</p> |
| `tags` | HashMap<String, String> |  | <p>The tags to assign to the service. A tag is a label that you assign to an Amazon Web Services resource. Each tag consists of a key-value pair.. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `created_time` | String | <p>The timestamp of when the service is created.</p> |
| `url_endpoint` | String | <p>The configuration for the URL endpoint type.</p>
         <p>The <b>Url</b> isthe URL of the endpoint type.</p>
         <p>The <b>HealthUrl</b> is the health check URL of the endpoint
      type. </p> |
| `name` | String | <p>The name of the service.</p> |
| `owner_account_id` | String | <p>The Amazon Web Services account ID of the service owner.</p> |
| `application_id` | String | <p>The ID of the application.</p> |
| `state` | String | <p>The current state of the service. </p> |
| `last_updated_time` | String | <p>A timestamp that indicates when the service was last updated. </p> |
| `endpoint_type` | String | <p>The endpoint type of the service.</p> |
| `environment_id` | String | <p>The unique identifier of the environment.</p> |
| `created_by_account_id` | String | <p>The Amazon Web Services account ID of the service creator.</p> |
| `tags` | HashMap<String, String> | <p>The tags assigned to the service. A tag is a label that you assign to an Amazon Web Services resource. Each tag consists of a key-value pair. </p> |
| `service_id` | String | <p>The unique identifier of the service.</p> |
| `arn` | String | <p>The Amazon Resource Name (ARN) of the service.</p> |
| `description` | String | <p>The description of the service. </p> |
| `lambda_endpoint` | String | <p>The configuration for the Lambda endpoint type.</p>
         <p>The <b>Arn</b> is the Amazon Resource Name (ARN) of the Lambda function associated with this service. </p> |
| `error` | String | <p>Any error associated with the service resource. </p> |
| `vpc_id` | String | <p>The ID of the virtual private cloud (VPC). </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create service
service = provider.migration_hub_refactor_spaces.Service {
    name = "value"  # <p>The name of the service.</p>
    endpoint_type = "value"  # <p>The type of endpoint to use for the service. The type can be a URL in a VPC or an Lambda function.</p>
    application_identifier = "value"  # <p>The ID of the application which the service is created.</p>
    environment_identifier = "value"  # <p>The ID of the environment in which the service is created.</p>
}

# Access service outputs
service_id = service.id
service_created_time = service.created_time
service_url_endpoint = service.url_endpoint
service_name = service.name
service_owner_account_id = service.owner_account_id
service_application_id = service.application_id
service_state = service.state
service_last_updated_time = service.last_updated_time
service_endpoint_type = service.endpoint_type
service_environment_id = service.environment_id
service_created_by_account_id = service.created_by_account_id
service_tags = service.tags
service_service_id = service.service_id
service_arn = service.arn
service_description = service.description
service_lambda_endpoint = service.lambda_endpoint
service_error = service.error
service_vpc_id = service.vpc_id
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple resource_policy resources
resource_policy_0 = provider.migration_hub_refactor_spaces.Resource_policy {
    resource_arn = "value-0"
    policy = "value-0"
}
resource_policy_1 = provider.migration_hub_refactor_spaces.Resource_policy {
    resource_arn = "value-1"
    policy = "value-1"
}
resource_policy_2 = provider.migration_hub_refactor_spaces.Resource_policy {
    resource_arn = "value-2"
    policy = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    resource_policy = provider.migration_hub_refactor_spaces.Resource_policy {
        resource_arn = "production-value"
        policy = "production-value"
    }
```

---

## Related Documentation

- [AWS Migration_hub_refactor_spaces Documentation](https://docs.aws.amazon.com/migration_hub_refactor_spaces/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
