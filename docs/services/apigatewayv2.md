# Apigatewayv2 Service



**Resources**: 31

---

## Overview

The apigatewayv2 service provides access to 31 resource types:

- [Vpc_links](#vpc_links) [R]
- [Stage](#stage) [CRUD]
- [Routing_rule](#routing_rule) [CRD]
- [Apis](#apis) [R]
- [Model_template](#model_template) [R]
- [Deployments](#deployments) [R]
- [Tags](#tags) [R]
- [Routes](#routes) [R]
- [Access_log_settings](#access_log_settings) [D]
- [Api](#api) [CRUD]
- [Integration_response](#integration_response) [CRUD]
- [Route_request_parameter](#route_request_parameter) [D]
- [Api_mapping](#api_mapping) [CRUD]
- [Integration_responses](#integration_responses) [R]
- [Model](#model) [CRUD]
- [Route](#route) [CRUD]
- [Cors_configuration](#cors_configuration) [D]
- [Stages](#stages) [R]
- [Models](#models) [R]
- [Route_response](#route_response) [CRUD]
- [Domain_names](#domain_names) [R]
- [Authorizers](#authorizers) [R]
- [Authorizer](#authorizer) [CRUD]
- [Deployment](#deployment) [CRUD]
- [Vpc_link](#vpc_link) [CRUD]
- [Route_settings](#route_settings) [D]
- [Route_responses](#route_responses) [R]
- [Domain_name](#domain_name) [CRUD]
- [Integration](#integration) [CRUD]
- [Api_mappings](#api_mappings) [R]
- [Integrations](#integrations) [R]

---

## Resources


### Vpc_links

VpcLinks resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The next page of elements from this collection. Not valid for the last element of the collection.</p> |
| `items` | Vec<String> | <p>A collection of VPC links.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access vpc_links outputs
vpc_links_id = vpc_links.id
vpc_links_next_token = vpc_links.next_token
vpc_links_items = vpc_links.items
```

---


### Stage

Stage resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `stage_name` | String | ✅ | <p>The name of the stage.</p> |
| `default_route_settings` | String |  | <p>The default route settings for the stage.</p> |
| `route_settings` | HashMap<String, String> |  | <p>Route settings for the stage, by routeKey.</p> |
| `access_log_settings` | String |  | <p>Settings for logging access in this stage.</p> |
| `stage_variables` | HashMap<String, String> |  | <p>A map that defines the stage variables for a Stage. Variable names can have alphanumeric and underscore characters, and the values must match [A-Za-z0-9-._~:/?#&amp;=,]+.</p> |
| `api_id` | String | ✅ | <p>The API identifier.</p> |
| `description` | String |  | <p>The description for the API stage.</p> |
| `auto_deploy` | bool |  | <p>Specifies whether updates to an API automatically trigger a new deployment. The default value is false.</p> |
| `deployment_id` | String |  | <p>The deployment identifier of the API stage.</p> |
| `client_certificate_id` | String |  | <p>The identifier of a client certificate for a Stage. Supported only for WebSocket APIs.</p> |
| `tags` | HashMap<String, String> |  | <p>The collection of tags. Each tag element is associated with a given resource.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `auto_deploy` | bool | <p>Specifies whether updates to an API automatically trigger a new deployment. The default value is false.</p> |
| `created_date` | String | <p>The timestamp when the stage was created.</p> |
| `access_log_settings` | String | <p>Settings for logging access in this stage.</p> |
| `client_certificate_id` | String | <p>The identifier of a client certificate for a Stage. Supported only for WebSocket APIs.</p> |
| `last_updated_date` | String | <p>The timestamp when the stage was last updated.</p> |
| `route_settings` | HashMap<String, String> | <p>Route settings for the stage, by routeKey.</p> |
| `description` | String | <p>The description of the stage.</p> |
| `last_deployment_status_message` | String | <p>Describes the status of the last deployment of a stage. Supported only for stages with autoDeploy enabled.</p> |
| `stage_variables` | HashMap<String, String> | <p>A map that defines the stage variables for a stage resource. Variable names can have alphanumeric and underscore characters, and the values must match [A-Za-z0-9-._~:/?#&amp;=,]+.</p> |
| `deployment_id` | String | <p>The identifier of the Deployment that the Stage is associated with. Can't be updated if autoDeploy is enabled.</p> |
| `stage_name` | String | <p>The name of the stage.</p> |
| `default_route_settings` | String | <p>Default route settings for the stage.</p> |
| `tags` | HashMap<String, String> | <p>The collection of tags. Each tag element is associated with a given resource.</p> |
| `api_gateway_managed` | bool | <p>Specifies whether a stage is managed by API Gateway. If you created an API using quick create, the $default stage is managed by API Gateway. You can't modify the $default stage.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create stage
stage = provider.apigatewayv2.Stage {
    stage_name = "value"  # <p>The name of the stage.</p>
    api_id = "value"  # <p>The API identifier.</p>
}

# Access stage outputs
stage_id = stage.id
stage_auto_deploy = stage.auto_deploy
stage_created_date = stage.created_date
stage_access_log_settings = stage.access_log_settings
stage_client_certificate_id = stage.client_certificate_id
stage_last_updated_date = stage.last_updated_date
stage_route_settings = stage.route_settings
stage_description = stage.description
stage_last_deployment_status_message = stage.last_deployment_status_message
stage_stage_variables = stage.stage_variables
stage_deployment_id = stage.deployment_id
stage_stage_name = stage.stage_name
stage_default_route_settings = stage.default_route_settings
stage_tags = stage.tags
stage_api_gateway_managed = stage.api_gateway_managed
```

---


### Routing_rule

RoutingRule resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `conditions` | Vec<String> | ✅ | <p>The routing rule condition.</p> |
| `domain_name` | String | ✅ | <p>The domain name.</p> |
| `actions` | Vec<String> | ✅ | <p>The routing rule action.</p> |
| `domain_name_id` | String |  | <p>The domain name ID.</p> |
| `priority` | i64 | ✅ | <p>The routing rule priority.</p> |
| `routing_rule_id` | String | ✅ | <p>The routing rule ID.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `actions` | Vec<String> | <p>The resulting action based on matching a routing rules condition. Only InvokeApi is supported.</p> |
| `conditions` | Vec<String> | <p>The conditions of the routing rule.</p> |
| `priority` | i64 | <p>The order in which API Gateway evaluates a rule. Priority is evaluated from the lowest value to the highest value.</p> |
| `routing_rule_arn` | String | <p>The routing rule ARN.</p> |
| `routing_rule_id` | String | <p>The routing rule ID.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create routing_rule
routing_rule = provider.apigatewayv2.Routing_rule {
    conditions = "value"  # <p>The routing rule condition.</p>
    domain_name = "value"  # <p>The domain name.</p>
    actions = "value"  # <p>The routing rule action.</p>
    priority = "value"  # <p>The routing rule priority.</p>
    routing_rule_id = "value"  # <p>The routing rule ID.</p>
}

# Access routing_rule outputs
routing_rule_id = routing_rule.id
routing_rule_actions = routing_rule.actions
routing_rule_conditions = routing_rule.conditions
routing_rule_priority = routing_rule.priority
routing_rule_routing_rule_arn = routing_rule.routing_rule_arn
routing_rule_routing_rule_id = routing_rule.routing_rule_id
```

---


### Apis

Apis resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `items` | Vec<String> | <p>The elements from this collection.</p> |
| `next_token` | String | <p>The next page of elements from this collection. Not valid for the last element of the collection.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access apis outputs
apis_id = apis.id
apis_items = apis.items
apis_next_token = apis.next_token
```

---


### Model_template

ModelTemplate resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `value` | String | <p>The template value.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access model_template outputs
model_template_id = model_template.id
model_template_value = model_template.value
```

---


### Deployments

Deployments resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The next page of elements from this collection. Not valid for the last element of the collection.</p> |
| `items` | Vec<String> | <p>The elements from this collection.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access deployments outputs
deployments_id = deployments.id
deployments_next_token = deployments.next_token
deployments_items = deployments.items
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
| `tags` | HashMap<String, String> |  |


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
tags_tags = tags.tags
```

---


### Routes

Routes resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The next page of elements from this collection. Not valid for the last element of the collection.</p> |
| `items` | Vec<String> | <p>The elements from this collection.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access routes outputs
routes_id = routes.id
routes_next_token = routes.next_token
routes_items = routes.items
```

---


### Access_log_settings

AccessLogSettings resource

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


### Api

Api resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `target` | String |  | <p>This property is part of quick create. Quick create produces an API with an integration, a default catch-all route, and a default stage which is configured to automatically deploy changes. For HTTP integrations, specify a fully qualified URL. For Lambda integrations, specify a function ARN. The type of the integration will be HTTP_PROXY or AWS_PROXY, respectively. Supported only for HTTP APIs.</p> |
| `disable_execute_api_endpoint` | bool |  | <p>Specifies whether clients can invoke your API by using the default execute-api endpoint. By default, clients can invoke your API with the default https://{api_id}.execute-api.{region}.amazonaws.com endpoint. To require that clients use a custom domain name to invoke your API, disable the default endpoint.</p> |
| `credentials_arn` | String |  | <p>This property is part of quick create. It specifies the credentials required for the integration, if any. For a Lambda integration, three options are available. To specify an IAM Role for API Gateway to assume, use the role's Amazon Resource Name (ARN). To require that the caller's identity be passed through from the request, specify arn:aws:iam::*:user/*. To use resource-based permissions on supported AWS services, specify null. Currently, this property is not used for HTTP integrations. Supported only for HTTP APIs.</p> |
| `ip_address_type` | String |  | <p>The IP address types that can invoke the API.</p> |
| `cors_configuration` | String |  | <p>A CORS configuration. Supported only for HTTP APIs. See <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-cors.html">Configuring CORS</a> for more information.</p> |
| `description` | String |  | <p>The description of the API.</p> |
| `name` | String | ✅ | <p>The name of the API.</p> |
| `protocol_type` | String | ✅ | <p>The API protocol.</p> |
| `route_key` | String |  | <p>This property is part of quick create. If you don't specify a routeKey, a default route of $default is created. The $default route acts as a catch-all for any request made to your API, for a particular stage. The $default route key can't be modified. You can add routes after creating the API, and you can update the route keys of additional routes. Supported only for HTTP APIs.</p> |
| `api_key_selection_expression` | String |  | <p>An API key selection expression. Supported only for WebSocket APIs. See <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api-selection-expressions.html#apigateway-websocket-api-apikey-selection-expressions">API Key Selection Expressions</a>.</p> |
| `disable_schema_validation` | bool |  | <p>Avoid validating models when creating a deployment. Supported only for WebSocket APIs.</p> |
| `route_selection_expression` | String |  | <p>The route selection expression for the API. For HTTP APIs, the routeSelectionExpression must be ${request.method} ${request.path}. If not provided, this will be the default for HTTP APIs. This property is required for WebSocket APIs.</p> |
| `tags` | HashMap<String, String> |  | <p>The collection of tags. Each tag element is associated with a given resource.</p> |
| `version` | String |  | <p>A version identifier for the API.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `ip_address_type` | String | <p>The IP address types that can invoke the API.</p> |
| `version` | String | <p>A version identifier for the API.</p> |
| `name` | String | <p>The name of the API.</p> |
| `import_info` | Vec<String> | <p>The validation information during API import. This may include particular properties of your OpenAPI definition which are ignored during import. Supported only for HTTP APIs.</p> |
| `disable_schema_validation` | bool | <p>Avoid validating models when creating a deployment. Supported only for WebSocket APIs.</p> |
| `api_gateway_managed` | bool | <p>Specifies whether an API is managed by API Gateway. You can't update or delete a managed API by using API Gateway. A managed API can be deleted only through the tooling or service that created it.</p> |
| `cors_configuration` | String | <p>A CORS configuration. Supported only for HTTP APIs.</p> |
| `route_selection_expression` | String | <p>The route selection expression for the API. For HTTP APIs, the routeSelectionExpression must be ${request.method} ${request.path}. If not provided, this will be the default for HTTP APIs. This property is required for WebSocket APIs.</p> |
| `api_endpoint` | String | <p>The URI of the API, of the form {api-id}.execute-api.{region}.amazonaws.com. The stage name is typically appended to this URI to form a complete path to a deployed API stage.</p> |
| `disable_execute_api_endpoint` | bool | <p>Specifies whether clients can invoke your API by using the default execute-api endpoint. By default, clients can invoke your API with the default https://{api_id}.execute-api.{region}.amazonaws.com endpoint. To require that clients use a custom domain name to invoke your API, disable the default endpoint.</p> |
| `api_key_selection_expression` | String | <p>An API key selection expression. Supported only for WebSocket APIs. See <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api-selection-expressions.html#apigateway-websocket-api-apikey-selection-expressions">API Key Selection Expressions</a>.</p> |
| `created_date` | String | <p>The timestamp when the API was created.</p> |
| `protocol_type` | String | <p>The API protocol.</p> |
| `tags` | HashMap<String, String> | <p>A collection of tags associated with the API.</p> |
| `warnings` | Vec<String> | <p>The warning messages reported when failonwarnings is turned on during API import.</p> |
| `api_id` | String | <p>The API ID.</p> |
| `description` | String | <p>The description of the API.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create api
api = provider.apigatewayv2.Api {
    name = "value"  # <p>The name of the API.</p>
    protocol_type = "value"  # <p>The API protocol.</p>
}

# Access api outputs
api_id = api.id
api_ip_address_type = api.ip_address_type
api_version = api.version
api_name = api.name
api_import_info = api.import_info
api_disable_schema_validation = api.disable_schema_validation
api_api_gateway_managed = api.api_gateway_managed
api_cors_configuration = api.cors_configuration
api_route_selection_expression = api.route_selection_expression
api_api_endpoint = api.api_endpoint
api_disable_execute_api_endpoint = api.disable_execute_api_endpoint
api_api_key_selection_expression = api.api_key_selection_expression
api_created_date = api.created_date
api_protocol_type = api.protocol_type
api_tags = api.tags
api_warnings = api.warnings
api_api_id = api.api_id
api_description = api.description
```

---


### Integration_response

IntegrationResponse resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `response_parameters` | HashMap<String, String> |  | <p>A key-value map specifying response parameters that are passed to the method response from the backend. The key is a method response header parameter name and the mapped value is an integration response header value, a static value enclosed within a pair of single quotes, or a JSON expression from the integration response body. The mapping key must match the pattern of method.response.header.{name}, where {name} is a valid and unique header name. The mapped non-static value must match the pattern of integration.response.header.{name} or integration.response.body.{JSON-expression}, where {name} is a valid and unique response header name and {JSON-expression} is a valid JSON expression without the $ prefix.</p> |
| `integration_id` | String | ✅ | <p>The integration ID.</p> |
| `integration_response_key` | String | ✅ | <p>The integration response key.</p> |
| `api_id` | String | ✅ | <p>The API identifier.</p> |
| `response_templates` | HashMap<String, String> |  | <p>The collection of response templates for the integration response as a string-to-string map of key-value pairs. Response templates are represented as a key/value map, with a content-type as the key and a template as the value.</p> |
| `template_selection_expression` | String |  | <p>The template selection expression for the integration response. Supported only for WebSocket APIs.</p> |
| `content_handling_strategy` | String |  | <p>Specifies how to handle response payload content type conversions. Supported values are CONVERT_TO_BINARY and CONVERT_TO_TEXT, with the following behaviors:</p> <p>CONVERT_TO_BINARY: Converts a response payload from a Base64-encoded string to the corresponding binary blob.</p> <p>CONVERT_TO_TEXT: Converts a response payload from a binary blob to a Base64-encoded string.</p> <p>If this property is not defined, the response payload will be passed through from the integration response to the route response or method response without modification.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `integration_response_key` | String | <p>The integration response key.</p> |
| `response_parameters` | HashMap<String, String> | <p>A key-value map specifying response parameters that are passed to the method response from the backend. The key is a method response header parameter name and the mapped value is an integration response header value, a static value enclosed within a pair of single quotes, or a JSON expression from the integration response body. The mapping key must match the pattern of method.response.header.{name}, where name is a valid and unique header name. The mapped non-static value must match the pattern of integration.response.header.{name} or integration.response.body.{JSON-expression}, where name is a valid and unique response header name and JSON-expression is a valid JSON expression without the $ prefix.</p> |
| `template_selection_expression` | String | <p>The template selection expressions for the integration response.</p> |
| `content_handling_strategy` | String | <p>Supported only for WebSocket APIs. Specifies how to handle response payload content type conversions. Supported values are CONVERT_TO_BINARY and CONVERT_TO_TEXT, with the following behaviors:</p> <p>CONVERT_TO_BINARY: Converts a response payload from a Base64-encoded string to the corresponding binary blob.</p> <p>CONVERT_TO_TEXT: Converts a response payload from a binary blob to a Base64-encoded string.</p> <p>If this property is not defined, the response payload will be passed through from the integration response to the route response or method response without modification.</p> |
| `integration_response_id` | String | <p>The integration response ID.</p> |
| `response_templates` | HashMap<String, String> | <p>The collection of response templates for the integration response as a string-to-string map of key-value pairs. Response templates are represented as a key/value map, with a content-type as the key and a template as the value.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create integration_response
integration_response = provider.apigatewayv2.Integration_response {
    integration_id = "value"  # <p>The integration ID.</p>
    integration_response_key = "value"  # <p>The integration response key.</p>
    api_id = "value"  # <p>The API identifier.</p>
}

# Access integration_response outputs
integration_response_id = integration_response.id
integration_response_integration_response_key = integration_response.integration_response_key
integration_response_response_parameters = integration_response.response_parameters
integration_response_template_selection_expression = integration_response.template_selection_expression
integration_response_content_handling_strategy = integration_response.content_handling_strategy
integration_response_integration_response_id = integration_response.integration_response_id
integration_response_response_templates = integration_response.response_templates
```

---


### Route_request_parameter

RouteRequestParameter resource

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


### Api_mapping

ApiMapping resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `api_mapping_key` | String |  | The API mapping key. |
| `stage` | String | ✅ | <p>The API stage.</p> |
| `domain_name` | String | ✅ | <p>The domain name.</p> |
| `api_id` | String | ✅ | <p>The API identifier.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `api_id` | String | <p>The API identifier.</p> |
| `stage` | String | <p>The API stage.</p> |
| `api_mapping_key` | String | <p>The API mapping key.</p> |
| `api_mapping_id` | String | <p>The API mapping identifier.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create api_mapping
api_mapping = provider.apigatewayv2.Api_mapping {
    stage = "value"  # <p>The API stage.</p>
    domain_name = "value"  # <p>The domain name.</p>
    api_id = "value"  # <p>The API identifier.</p>
}

# Access api_mapping outputs
api_mapping_id = api_mapping.id
api_mapping_api_id = api_mapping.api_id
api_mapping_stage = api_mapping.stage
api_mapping_api_mapping_key = api_mapping.api_mapping_key
api_mapping_api_mapping_id = api_mapping.api_mapping_id
```

---


### Integration_responses

IntegrationResponses resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `items` | Vec<String> | <p>The elements from this collection.</p> |
| `next_token` | String | <p>The next page of elements from this collection. Not valid for the last element of the collection.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access integration_responses outputs
integration_responses_id = integration_responses.id
integration_responses_items = integration_responses.items
integration_responses_next_token = integration_responses.next_token
```

---


### Model

Model resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | <p>The description of the model.</p> |
| `name` | String | ✅ | <p>The name of the model. Must be alphanumeric.</p> |
| `api_id` | String | ✅ | <p>The API identifier.</p> |
| `content_type` | String |  | <p>The content-type for the model, for example, "application/json".</p> |
| `schema` | String | ✅ | <p>The schema for the model. For application/json models, this should be JSON schema draft 4 model.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | <p>The description of the model.</p> |
| `model_id` | String | <p>The model identifier.</p> |
| `name` | String | <p>The name of the model. Must be alphanumeric.</p> |
| `schema` | String | <p>The schema for the model. For application/json models, this should be JSON schema draft 4 model.</p> |
| `content_type` | String | <p>The content-type for the model, for example, "application/json".</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create model
model = provider.apigatewayv2.Model {
    name = "value"  # <p>The name of the model. Must be alphanumeric.</p>
    api_id = "value"  # <p>The API identifier.</p>
    schema = "value"  # <p>The schema for the model. For application/json models, this should be JSON schema draft 4 model.</p>
}

# Access model outputs
model_id = model.id
model_description = model.description
model_model_id = model.model_id
model_name = model.name
model_schema = model.schema
model_content_type = model.content_type
```

---


### Route

Route resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `authorization_scopes` | Vec<String> |  | <p>The authorization scopes supported by this route.</p> |
| `authorization_type` | String |  | <p>The authorization type for the route. For WebSocket APIs, valid values are NONE for open access, AWS_IAM for using AWS IAM permissions, and CUSTOM for using a Lambda authorizer For HTTP APIs, valid values are NONE for open access, JWT for using JSON Web Tokens, AWS_IAM for using AWS IAM permissions, and CUSTOM for using a Lambda authorizer.</p> |
| `operation_name` | String |  | <p>The operation name for the route.</p> |
| `route_key` | String | ✅ | <p>The route key for the route.</p> |
| `api_id` | String | ✅ | <p>The API identifier.</p> |
| `api_key_required` | bool |  | <p>Specifies whether an API key is required for the route. Supported only for WebSocket APIs.</p> |
| `route_response_selection_expression` | String |  | <p>The route response selection expression for the route. Supported only for WebSocket APIs.</p> |
| `request_parameters` | HashMap<String, String> |  | <p>The request parameters for the route. Supported only for WebSocket APIs.</p> |
| `target` | String |  | <p>The target for the route.</p> |
| `request_models` | HashMap<String, String> |  | <p>The request models for the route. Supported only for WebSocket APIs.</p> |
| `model_selection_expression` | String |  | <p>The model selection expression for the route. Supported only for WebSocket APIs.</p> |
| `authorizer_id` | String |  | <p>The identifier of the Authorizer resource to be associated with this route. The authorizer identifier is generated by API Gateway when you created the authorizer.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `target` | String | <p>The target for the route.</p> |
| `authorization_type` | String | <p>The authorization type for the route. For WebSocket APIs, valid values are NONE for open access, AWS_IAM for using AWS IAM permissions, and CUSTOM for using a Lambda authorizer For HTTP APIs, valid values are NONE for open access, JWT for using JSON Web Tokens, AWS_IAM for using AWS IAM permissions, and CUSTOM for using a Lambda authorizer.</p> |
| `authorization_scopes` | Vec<String> | <p>A list of authorization scopes configured on a route. The scopes are used with a JWT authorizer to authorize the method invocation. The authorization works by matching the route scopes against the scopes parsed from the access token in the incoming request. The method invocation is authorized if any route scope matches a claimed scope in the access token. Otherwise, the invocation is not authorized. When the route scope is configured, the client must provide an access token instead of an identity token for authorization purposes.</p> |
| `api_gateway_managed` | bool | <p>Specifies whether a route is managed by API Gateway. If you created an API using quick create, the $default route is managed by API Gateway. You can't modify the $default route key.</p> |
| `authorizer_id` | String | <p>The identifier of the Authorizer resource to be associated with this route. The authorizer identifier is generated by API Gateway when you created the authorizer.</p> |
| `model_selection_expression` | String | <p>The model selection expression for the route. Supported only for WebSocket APIs.</p> |
| `operation_name` | String | <p>The operation name for the route.</p> |
| `request_models` | HashMap<String, String> | <p>The request models for the route. Supported only for WebSocket APIs.</p> |
| `request_parameters` | HashMap<String, String> | <p>The request parameters for the route. Supported only for WebSocket APIs.</p> |
| `api_key_required` | bool | <p>Specifies whether an API key is required for this route. Supported only for WebSocket APIs.</p> |
| `route_id` | String | <p>The route ID.</p> |
| `route_key` | String | <p>The route key for the route.</p> |
| `route_response_selection_expression` | String | <p>The route response selection expression for the route. Supported only for WebSocket APIs.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create route
route = provider.apigatewayv2.Route {
    route_key = "value"  # <p>The route key for the route.</p>
    api_id = "value"  # <p>The API identifier.</p>
}

# Access route outputs
route_id = route.id
route_target = route.target
route_authorization_type = route.authorization_type
route_authorization_scopes = route.authorization_scopes
route_api_gateway_managed = route.api_gateway_managed
route_authorizer_id = route.authorizer_id
route_model_selection_expression = route.model_selection_expression
route_operation_name = route.operation_name
route_request_models = route.request_models
route_request_parameters = route.request_parameters
route_api_key_required = route.api_key_required
route_route_id = route.route_id
route_route_key = route.route_key
route_route_response_selection_expression = route.route_response_selection_expression
```

---


### Cors_configuration

CorsConfiguration resource

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


### Stages

Stages resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The next page of elements from this collection. Not valid for the last element of the collection.</p> |
| `items` | Vec<String> | <p>The elements from this collection.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access stages outputs
stages_id = stages.id
stages_next_token = stages.next_token
stages_items = stages.items
```

---


### Models

Models resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The next page of elements from this collection. Not valid for the last element of the collection.</p> |
| `items` | Vec<String> | <p>The elements from this collection.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access models outputs
models_id = models.id
models_next_token = models.next_token
models_items = models.items
```

---


### Route_response

RouteResponse resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `model_selection_expression` | String |  | <p>The model selection expression for the route response. Supported only for WebSocket APIs.</p> |
| `api_id` | String | ✅ | <p>The API identifier.</p> |
| `response_models` | HashMap<String, String> |  | <p>The response models for the route response.</p> |
| `response_parameters` | HashMap<String, String> |  | <p>The route response parameters.</p> |
| `route_id` | String | ✅ | <p>The route ID.</p> |
| `route_response_key` | String | ✅ | <p>The route response key.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `model_selection_expression` | String | <p>Represents the model selection expression of a route response. Supported only for WebSocket APIs.</p> |
| `route_response_key` | String | <p>Represents the route response key of a route response.</p> |
| `response_models` | HashMap<String, String> | <p>Represents the response models of a route response.</p> |
| `response_parameters` | HashMap<String, String> | <p>Represents the response parameters of a route response.</p> |
| `route_response_id` | String | <p>Represents the identifier of a route response.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create route_response
route_response = provider.apigatewayv2.Route_response {
    api_id = "value"  # <p>The API identifier.</p>
    route_id = "value"  # <p>The route ID.</p>
    route_response_key = "value"  # <p>The route response key.</p>
}

# Access route_response outputs
route_response_id = route_response.id
route_response_model_selection_expression = route_response.model_selection_expression
route_response_route_response_key = route_response.route_response_key
route_response_response_models = route_response.response_models
route_response_response_parameters = route_response.response_parameters
route_response_route_response_id = route_response.route_response_id
```

---


### Domain_names

DomainNames resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The next page of elements from this collection. Not valid for the last element of the collection.</p> |
| `items` | Vec<String> | <p>The elements from this collection.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access domain_names outputs
domain_names_id = domain_names.id
domain_names_next_token = domain_names.next_token
domain_names_items = domain_names.items
```

---


### Authorizers

Authorizers resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `items` | Vec<String> | <p>The elements from this collection.</p> |
| `next_token` | String | <p>The next page of elements from this collection. Not valid for the last element of the collection.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access authorizers outputs
authorizers_id = authorizers.id
authorizers_items = authorizers.items
authorizers_next_token = authorizers.next_token
```

---


### Authorizer

Authorizer resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `authorizer_uri` | String |  | <p>The authorizer's Uniform Resource Identifier (URI). For REQUEST authorizers, this must be a well-formed Lambda function URI, for example, arn:aws:apigateway:us-west-2:lambda:path/2015-03-31/functions/arn:aws:lambda:us-west-2:<replaceable>{account_id}</replaceable>:function:<replaceable>{lambda_function_name}</replaceable>/invocations. In general, the URI has this form: arn:aws:apigateway:<replaceable>{region}</replaceable>:lambda:path/<replaceable>{service_api}</replaceable>
               , where <replaceable></replaceable>{region} is the same as the region hosting the Lambda function, path indicates that the remaining substring in the URI should be treated as the path to the resource, including the initial /. For Lambda functions, this is usually of the form /2015-03-31/functions/[FunctionARN]/invocations. Supported only for REQUEST authorizers.</p> |
| `authorizer_credentials_arn` | String |  | <p>Specifies the required credentials as an IAM role for API Gateway to invoke the authorizer. To specify an IAM role for API Gateway to assume, use the role's Amazon Resource Name (ARN). To use resource-based permissions on the Lambda function, don't specify this parameter. Supported only for REQUEST authorizers.</p> |
| `name` | String | ✅ | <p>The name of the authorizer.</p> |
| `identity_source` | Vec<String> | ✅ | <p>The identity source for which authorization is requested.</p> <p>For a REQUEST authorizer, this is optional. The value is a set of one or more mapping expressions of the specified request parameters. The identity source can be headers, query string parameters, stage variables, and context parameters. For example, if an Auth header and a Name query string parameter are defined as identity sources, this value is route.request.header.Auth, route.request.querystring.Name for WebSocket APIs. For HTTP APIs, use selection expressions prefixed with $, for example, $request.header.Auth, $request.querystring.Name. These parameters are used to perform runtime validation for Lambda-based authorizers by verifying all of the identity-related request parameters are present in the request, not null, and non-empty. Only when this is true does the authorizer invoke the authorizer Lambda function. Otherwise, it returns a 401 Unauthorized response without calling the Lambda function. For HTTP APIs, identity sources are also used as the cache key when caching is enabled. To learn more, see <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-lambda-authorizer.html">Working with AWS Lambda authorizers for HTTP APIs</a>.</p> <p>For JWT, a single entry that specifies where to extract the JSON Web Token (JWT) from inbound requests. Currently only header-based and query parameter-based selections are supported, for example $request.header.Authorization.</p> |
| `authorizer_type` | String | ✅ | <p>The authorizer type. Specify REQUEST for a Lambda function using incoming request parameters. Specify JWT to use JSON Web Tokens (supported only for HTTP APIs).</p> |
| `enable_simple_responses` | bool |  | <p>Specifies whether a Lambda authorizer returns a response in a simple format. By default, a Lambda authorizer must return an IAM policy. If enabled, the Lambda authorizer can return a boolean value instead of an IAM policy. Supported only for HTTP APIs. To learn more, see <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-lambda-authorizer.html">Working with AWS Lambda authorizers for HTTP APIs</a></p> |
| `api_id` | String | ✅ | <p>The API identifier.</p> |
| `identity_validation_expression` | String |  | <p>This parameter is not used.</p> |
| `jwt_configuration` | String |  | <p>Represents the configuration of a JWT authorizer. Required for the JWT authorizer type. Supported only for HTTP APIs.</p> |
| `authorizer_result_ttl_in_seconds` | i64 |  | <p>The time to live (TTL) for cached authorizer results, in seconds. If it equals 0, authorization caching is disabled. If it is greater than 0, API Gateway caches authorizer responses. The maximum value is 3600, or 1 hour. Supported only for HTTP API Lambda authorizers.</p> |
| `authorizer_payload_format_version` | String |  | <p>Specifies the format of the payload sent to an HTTP API Lambda authorizer. Required for HTTP API Lambda authorizers. Supported values are 1.0 and 2.0. To learn more, see <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-lambda-authorizer.html">Working with AWS Lambda authorizers for HTTP APIs</a>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `authorizer_id` | String | <p>The authorizer identifier.</p> |
| `jwt_configuration` | String | <p>Represents the configuration of a JWT authorizer. Required for the JWT authorizer type. Supported only for HTTP APIs.</p> |
| `identity_source` | Vec<String> | <p>The identity source for which authorization is requested.</p> <p>For a REQUEST authorizer, this is optional. The value is a set of one or more mapping expressions of the specified request parameters. The identity source can be headers, query string parameters, stage variables, and context parameters. For example, if an Auth header and a Name query string parameter are defined as identity sources, this value is route.request.header.Auth, route.request.querystring.Name for WebSocket APIs. For HTTP APIs, use selection expressions prefixed with $, for example, $request.header.Auth, $request.querystring.Name. These parameters are used to perform runtime validation for Lambda-based authorizers by verifying all of the identity-related request parameters are present in the request, not null, and non-empty. Only when this is true does the authorizer invoke the authorizer Lambda function. Otherwise, it returns a 401 Unauthorized response without calling the Lambda function. For HTTP APIs, identity sources are also used as the cache key when caching is enabled. To learn more, see <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-lambda-authorizer.html">Working with AWS Lambda authorizers for HTTP APIs</a>.</p> <p>For JWT, a single entry that specifies where to extract the JSON Web Token (JWT) from inbound requests. Currently only header-based and query parameter-based selections are supported, for example $request.header.Authorization.</p> |
| `identity_validation_expression` | String | <p>The validation expression does not apply to the REQUEST authorizer.</p> |
| `name` | String | <p>The name of the authorizer.</p> |
| `authorizer_uri` | String | <p>The authorizer's Uniform Resource Identifier (URI). For REQUEST authorizers, this must be a well-formed Lambda function URI, for example, arn:aws:apigateway:us-west-2:lambda:path/2015-03-31/functions/arn:aws:lambda:us-west-2:<replaceable>{account_id}</replaceable>:function:<replaceable>{lambda_function_name}</replaceable>/invocations. In general, the URI has this form: arn:aws:apigateway:<replaceable>{region}</replaceable>:lambda:path/<replaceable>{service_api}</replaceable>
               , where <replaceable></replaceable>{region} is the same as the region hosting the Lambda function, path indicates that the remaining substring in the URI should be treated as the path to the resource, including the initial /. For Lambda functions, this is usually of the form /2015-03-31/functions/[FunctionARN]/invocations. Supported only for REQUEST authorizers.</p> |
| `authorizer_result_ttl_in_seconds` | i64 | <p>The time to live (TTL) for cached authorizer results, in seconds. If it equals 0, authorization caching is disabled. If it is greater than 0, API Gateway caches authorizer responses. The maximum value is 3600, or 1 hour. Supported only for HTTP API Lambda authorizers.</p> |
| `authorizer_payload_format_version` | String | <p>Specifies the format of the payload sent to an HTTP API Lambda authorizer. Required for HTTP API Lambda authorizers. Supported values are 1.0 and 2.0. To learn more, see <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-lambda-authorizer.html">Working with AWS Lambda authorizers for HTTP APIs</a>.</p> |
| `authorizer_type` | String | <p>The authorizer type. Specify REQUEST for a Lambda function using incoming request parameters. Specify JWT to use JSON Web Tokens (supported only for HTTP APIs).</p> |
| `enable_simple_responses` | bool | <p>Specifies whether a Lambda authorizer returns a response in a simple format. If enabled, the Lambda authorizer can return a boolean value instead of an IAM policy. Supported only for HTTP APIs. To learn more, see <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-lambda-authorizer.html">Working with AWS Lambda authorizers for HTTP APIs</a></p> |
| `authorizer_credentials_arn` | String | <p>Specifies the required credentials as an IAM role for API Gateway to invoke the authorizer. To specify an IAM role for API Gateway to assume, use the role's Amazon Resource Name (ARN). To use resource-based permissions on the Lambda function, don't specify this parameter. Supported only for REQUEST authorizers.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create authorizer
authorizer = provider.apigatewayv2.Authorizer {
    name = "value"  # <p>The name of the authorizer.</p>
    identity_source = "value"  # <p>The identity source for which authorization is requested.</p> <p>For a REQUEST authorizer, this is optional. The value is a set of one or more mapping expressions of the specified request parameters. The identity source can be headers, query string parameters, stage variables, and context parameters. For example, if an Auth header and a Name query string parameter are defined as identity sources, this value is route.request.header.Auth, route.request.querystring.Name for WebSocket APIs. For HTTP APIs, use selection expressions prefixed with $, for example, $request.header.Auth, $request.querystring.Name. These parameters are used to perform runtime validation for Lambda-based authorizers by verifying all of the identity-related request parameters are present in the request, not null, and non-empty. Only when this is true does the authorizer invoke the authorizer Lambda function. Otherwise, it returns a 401 Unauthorized response without calling the Lambda function. For HTTP APIs, identity sources are also used as the cache key when caching is enabled. To learn more, see <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-lambda-authorizer.html">Working with AWS Lambda authorizers for HTTP APIs</a>.</p> <p>For JWT, a single entry that specifies where to extract the JSON Web Token (JWT) from inbound requests. Currently only header-based and query parameter-based selections are supported, for example $request.header.Authorization.</p>
    authorizer_type = "value"  # <p>The authorizer type. Specify REQUEST for a Lambda function using incoming request parameters. Specify JWT to use JSON Web Tokens (supported only for HTTP APIs).</p>
    api_id = "value"  # <p>The API identifier.</p>
}

# Access authorizer outputs
authorizer_id = authorizer.id
authorizer_authorizer_id = authorizer.authorizer_id
authorizer_jwt_configuration = authorizer.jwt_configuration
authorizer_identity_source = authorizer.identity_source
authorizer_identity_validation_expression = authorizer.identity_validation_expression
authorizer_name = authorizer.name
authorizer_authorizer_uri = authorizer.authorizer_uri
authorizer_authorizer_result_ttl_in_seconds = authorizer.authorizer_result_ttl_in_seconds
authorizer_authorizer_payload_format_version = authorizer.authorizer_payload_format_version
authorizer_authorizer_type = authorizer.authorizer_type
authorizer_enable_simple_responses = authorizer.enable_simple_responses
authorizer_authorizer_credentials_arn = authorizer.authorizer_credentials_arn
```

---


### Deployment

Deployment resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `api_id` | String | ✅ | <p>The API identifier.</p> |
| `description` | String |  | <p>The description for the deployment resource.</p> |
| `stage_name` | String |  | <p>The name of the Stage resource for the Deployment resource to create.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | <p>The description for the deployment.</p> |
| `auto_deployed` | bool | <p>Specifies whether a deployment was automatically released.</p> |
| `deployment_status` | String | <p>The status of the deployment: PENDING, FAILED, or SUCCEEDED.</p> |
| `deployment_status_message` | String | <p>May contain additional feedback on the status of an API deployment.</p> |
| `deployment_id` | String | <p>The identifier for the deployment.</p> |
| `created_date` | String | <p>The date and time when the Deployment resource was created.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create deployment
deployment = provider.apigatewayv2.Deployment {
    api_id = "value"  # <p>The API identifier.</p>
}

# Access deployment outputs
deployment_id = deployment.id
deployment_description = deployment.description
deployment_auto_deployed = deployment.auto_deployed
deployment_deployment_status = deployment.deployment_status
deployment_deployment_status_message = deployment.deployment_status_message
deployment_deployment_id = deployment.deployment_id
deployment_created_date = deployment.created_date
```

---


### Vpc_link

VpcLink resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>The name of the VPC link.</p> |
| `tags` | HashMap<String, String> |  | <p>A list of tags.</p> |
| `security_group_ids` | Vec<String> |  | <p>A list of security group IDs for the VPC link.</p> |
| `subnet_ids` | Vec<String> | ✅ | <p>A list of subnet IDs to include in the VPC link.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `vpc_link_status_message` | String | <p>A message summarizing the cause of the status of the VPC link.</p> |
| `security_group_ids` | Vec<String> | <p>A list of security group IDs for the VPC link.</p> |
| `name` | String | <p>The name of the VPC link.</p> |
| `tags` | HashMap<String, String> | <p>Tags for the VPC link.</p> |
| `vpc_link_id` | String | <p>The ID of the VPC link.</p> |
| `created_date` | String | <p>The timestamp when the VPC link was created.</p> |
| `subnet_ids` | Vec<String> | <p>A list of subnet IDs to include in the VPC link.</p> |
| `vpc_link_version` | String | <p>The version of the VPC link.</p> |
| `vpc_link_status` | String | <p>The status of the VPC link.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create vpc_link
vpc_link = provider.apigatewayv2.Vpc_link {
    name = "value"  # <p>The name of the VPC link.</p>
    subnet_ids = "value"  # <p>A list of subnet IDs to include in the VPC link.</p>
}

# Access vpc_link outputs
vpc_link_id = vpc_link.id
vpc_link_vpc_link_status_message = vpc_link.vpc_link_status_message
vpc_link_security_group_ids = vpc_link.security_group_ids
vpc_link_name = vpc_link.name
vpc_link_tags = vpc_link.tags
vpc_link_vpc_link_id = vpc_link.vpc_link_id
vpc_link_created_date = vpc_link.created_date
vpc_link_subnet_ids = vpc_link.subnet_ids
vpc_link_vpc_link_version = vpc_link.vpc_link_version
vpc_link_vpc_link_status = vpc_link.vpc_link_status
```

---


### Route_settings

RouteSettings resource

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


### Route_responses

RouteResponses resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The next page of elements from this collection. Not valid for the last element of the collection.</p> |
| `items` | Vec<String> | <p>The elements from this collection.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access route_responses outputs
route_responses_id = route_responses.id
route_responses_next_token = route_responses.next_token
route_responses_items = route_responses.items
```

---


### Domain_name

DomainName resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `mutual_tls_authentication` | String |  | <p>The mutual TLS authentication configuration for a custom domain name.</p> |
| `routing_mode` | String |  | <p>The routing mode.</p> |
| `tags` | HashMap<String, String> |  | <p>The collection of tags associated with a domain name.</p> |
| `domain_name` | String | ✅ | <p>The domain name.</p> |
| `domain_name_configurations` | Vec<String> |  | <p>The domain name configurations.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `mutual_tls_authentication` | String | <p>The mutual TLS authentication configuration for a custom domain name.</p> |
| `api_mapping_selection_expression` | String | <p>The API mapping selection expression.</p> |
| `routing_mode` | String | <p>The routing mode.</p> |
| `domain_name_arn` | String |  |
| `domain_name_configurations` | Vec<String> | <p>The domain name configurations.</p> |
| `tags` | HashMap<String, String> | <p>The collection of tags associated with a domain name.</p> |
| `domain_name` | String | <p>The name of the DomainName resource.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create domain_name
domain_name = provider.apigatewayv2.Domain_name {
    domain_name = "value"  # <p>The domain name.</p>
}

# Access domain_name outputs
domain_name_id = domain_name.id
domain_name_mutual_tls_authentication = domain_name.mutual_tls_authentication
domain_name_api_mapping_selection_expression = domain_name.api_mapping_selection_expression
domain_name_routing_mode = domain_name.routing_mode
domain_name_domain_name_arn = domain_name.domain_name_arn
domain_name_domain_name_configurations = domain_name.domain_name_configurations
domain_name_tags = domain_name.tags
domain_name_domain_name = domain_name.domain_name
```

---


### Integration

Integration resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | <p>The description of the integration.</p> |
| `integration_subtype` | String |  | <p>Supported only for HTTP API AWS_PROXY integrations. Specifies the AWS service action to invoke. To learn more, see <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-develop-integrations-aws-services-reference.html">Integration subtype reference</a>.</p> |
| `template_selection_expression` | String |  | <p>The template selection expression for the integration.</p> |
| `timeout_in_millis` | i64 |  | <p>Custom timeout between 50 and 29,000 milliseconds for WebSocket APIs and between 50 and 30,000 milliseconds for HTTP APIs. The default timeout is 29 seconds for WebSocket APIs and 30 seconds for HTTP APIs.</p> |
| `tls_config` | String |  | <p>The TLS configuration for a private integration. If you specify a TLS configuration, private integration traffic uses the HTTPS protocol. Supported only for HTTP APIs.</p> |
| `integration_type` | String | ✅ | <p>The integration type of an integration. One of the following:</p> <p>AWS: for integrating the route or method request with an AWS service action, including the Lambda function-invoking action. With the Lambda function-invoking action, this is referred to as the Lambda custom integration. With any other AWS service action, this is known as AWS integration. Supported only for WebSocket APIs.</p> <p>AWS_PROXY: for integrating the route or method request with a Lambda function or other AWS service action. This integration is also referred to as a Lambda proxy integration.</p> <p>HTTP: for integrating the route or method request with an HTTP endpoint. This integration is also referred to as the HTTP custom integration. Supported only for WebSocket APIs.</p> <p>HTTP_PROXY: for integrating the route or method request with an HTTP endpoint, with the client request passed through as-is. This is also referred to as HTTP proxy integration. For HTTP API private integrations, use an HTTP_PROXY integration.</p> <p>MOCK: for integrating the route or method request with API Gateway as a "loopback" endpoint without invoking any backend. Supported only for WebSocket APIs.</p> |
| `content_handling_strategy` | String |  | <p>Supported only for WebSocket APIs. Specifies how to handle response payload content type conversions. Supported values are CONVERT_TO_BINARY and CONVERT_TO_TEXT, with the following behaviors:</p> <p>CONVERT_TO_BINARY: Converts a response payload from a Base64-encoded string to the corresponding binary blob.</p> <p>CONVERT_TO_TEXT: Converts a response payload from a binary blob to a Base64-encoded string.</p> <p>If this property is not defined, the response payload will be passed through from the integration response to the route response or method response without modification.</p> |
| `request_parameters` | HashMap<String, String> |  | <p>For WebSocket APIs, a key-value map specifying request parameters that are passed from the method request to the backend. The key is an integration request parameter name and the associated value is a method request parameter value or static value that must be enclosed within single quotes and pre-encoded as required by the backend. The method request parameter value must match the pattern of method.request.<replaceable>{location}</replaceable>.<replaceable>{name}</replaceable>
               , where 
                  <replaceable>{location}</replaceable>
                is querystring, path, or header; and 
                  <replaceable>{name}</replaceable>
                must be a valid and unique method request parameter name.</p> <p>For HTTP API integrations with a specified integrationSubtype, request parameters are a key-value map specifying parameters that are passed to AWS_PROXY integrations. You can provide static values, or map request data, stage variables, or context variables that are evaluated at runtime. To learn more, see <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-develop-integrations-aws-services.html">Working with AWS service integrations for HTTP APIs</a>.</p> <p>For HTTP API integrations without a specified integrationSubtype request parameters are a key-value map specifying how to transform HTTP requests before sending them to the backend. The key should follow the pattern &lt;action&gt;:&lt;header|querystring|path&gt;.&lt;location&gt; where action can be append, overwrite or remove. For values, you can provide static values, or map request data, stage variables, or context variables that are evaluated at runtime. To learn more, see <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-parameter-mapping.html">Transforming API requests and responses</a>.</p> |
| `passthrough_behavior` | String |  | <p>Specifies the pass-through behavior for incoming requests based on the Content-Type header in the request, and the available mapping templates specified as the requestTemplates property on the Integration resource. There are three valid values: WHEN_NO_MATCH, WHEN_NO_TEMPLATES, and NEVER. Supported only for WebSocket APIs.</p> <p>WHEN_NO_MATCH passes the request body for unmapped content types through to the integration backend without transformation.</p> <p>NEVER rejects unmapped content types with an HTTP 415 Unsupported Media Type response.</p> <p>WHEN_NO_TEMPLATES allows pass-through when the integration has no content types mapped to templates. However, if there is at least one content type defined, unmapped content types will be rejected with the same HTTP 415 Unsupported Media Type response.</p> |
| `connection_type` | String |  | <p>The type of the network connection to the integration endpoint. Specify INTERNET for connections through the public routable internet or VPC_LINK for private connections between API Gateway and resources in a VPC. The default value is INTERNET.</p> |
| `payload_format_version` | String |  | <p>Specifies the format of the payload sent to an integration. Required for HTTP APIs. Supported values for Lambda proxy integrations are 1.0 and 2.0. For all other integrations, 1.0 is the only supported value. To learn more, see <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-develop-integrations-lambda.html">Working with AWS Lambda proxy integrations for HTTP APIs</a>.</p> |
| `response_parameters` | HashMap<String, HashMap<String, String>> |  | <p>Supported only for HTTP APIs. You use response parameters to transform the HTTP response from a backend integration before returning the response to clients. Specify a key-value map from a selection key to response parameters. The selection key must be a valid HTTP status code within the range of 200-599. Response parameters are a key-value map. The key must match pattern &lt;action&gt;:&lt;header&gt;.&lt;location&gt; or overwrite.statuscode. The action can be append, overwrite or remove. The value can be a static value, or map to response data, stage variables, or context variables that are evaluated at runtime. To learn more, see <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-parameter-mapping.html">Transforming API requests and responses</a>.</p> |
| `integration_uri` | String |  | <p>For a Lambda integration, specify the URI of a Lambda function.</p> <p>For an HTTP integration, specify a fully-qualified URL.</p> <p>For an HTTP API private integration, specify the ARN of an Application Load Balancer listener, Network Load Balancer listener, or AWS Cloud Map service. If you specify the ARN of an AWS Cloud Map service, API Gateway uses DiscoverInstances to identify resources. You can use query parameters to target specific resources. To learn more, see <a href="https://docs.aws.amazon.com/cloud-map/latest/api/API_DiscoverInstances.html">DiscoverInstances</a>. For private integrations, all resources must be owned by the same AWS account.</p> |
| `connection_id` | String |  | <p>The ID of the VPC link for a private integration. Supported only for HTTP APIs.</p> |
| `api_id` | String | ✅ | <p>The API identifier.</p> |
| `request_templates` | HashMap<String, String> |  | <p>Represents a map of Velocity templates that are applied on the request payload based on the value of the Content-Type header sent by the client. The content type value is the key in this map, and the template (as a String) is the value. Supported only for WebSocket APIs.</p> |
| `integration_method` | String |  | <p>Specifies the integration's HTTP method type.</p> |
| `credentials_arn` | String |  | <p>Specifies the credentials required for the integration, if any. For AWS integrations, three options are available. To specify an IAM Role for API Gateway to assume, use the role's Amazon Resource Name (ARN). To require that the caller's identity be passed through from the request, specify the string arn:aws:iam::*:user/*. To use resource-based permissions on supported AWS services, specify null.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `timeout_in_millis` | i64 | <p>Custom timeout between 50 and 29,000 milliseconds for WebSocket APIs and between 50 and 30,000 milliseconds for HTTP APIs. The default timeout is 29 seconds for WebSocket APIs and 30 seconds for HTTP APIs.</p> |
| `connection_id` | String | <p>The ID of the VPC link for a private integration. Supported only for HTTP APIs.</p> |
| `integration_id` | String | <p>Represents the identifier of an integration.</p> |
| `integration_type` | String | <p>The integration type of an integration. One of the following:</p> <p>AWS: for integrating the route or method request with an AWS service action, including the Lambda function-invoking action. With the Lambda function-invoking action, this is referred to as the Lambda custom integration. With any other AWS service action, this is known as AWS integration. Supported only for WebSocket APIs.</p> <p>AWS_PROXY: for integrating the route or method request with a Lambda function or other AWS service action. This integration is also referred to as a Lambda proxy integration.</p> <p>HTTP: for integrating the route or method request with an HTTP endpoint. This integration is also referred to as the HTTP custom integration. Supported only for WebSocket APIs.</p> <p>HTTP_PROXY: for integrating the route or method request with an HTTP endpoint, with the client request passed through as-is. This is also referred to as HTTP proxy integration.</p> <p>MOCK: for integrating the route or method request with API Gateway as a "loopback" endpoint without invoking any backend. Supported only for WebSocket APIs.</p> |
| `response_parameters` | HashMap<String, HashMap<String, String>> | <p>Supported only for HTTP APIs. You use response parameters to transform the HTTP response from a backend integration before returning the response to clients. Specify a key-value map from a selection key to response parameters. The selection key must be a valid HTTP status code within the range of 200-599. Response parameters are a key-value map. The key must match pattern &lt;action&gt;:&lt;header&gt;.&lt;location&gt; or overwrite.statuscode. The action can be append, overwrite or remove. The value can be a static value, or map to response data, stage variables, or context variables that are evaluated at runtime. To learn more, see <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-parameter-mapping.html">Transforming API requests and responses</a>.</p> |
| `description` | String | <p>Represents the description of an integration.</p> |
| `integration_uri` | String | <p>For a Lambda integration, specify the URI of a Lambda function.</p> <p>For an HTTP integration, specify a fully-qualified URL.</p> <p>For an HTTP API private integration, specify the ARN of an Application Load Balancer listener, Network Load Balancer listener, or AWS Cloud Map service. If you specify the ARN of an AWS Cloud Map service, API Gateway uses DiscoverInstances to identify resources. You can use query parameters to target specific resources. To learn more, see <a href="https://docs.aws.amazon.com/cloud-map/latest/api/API_DiscoverInstances.html">DiscoverInstances</a>. For private integrations, all resources must be owned by the same AWS account.</p> |
| `passthrough_behavior` | String | <p>Specifies the pass-through behavior for incoming requests based on the Content-Type header in the request, and the available mapping templates specified as the requestTemplates property on the Integration resource. There are three valid values: WHEN_NO_MATCH, WHEN_NO_TEMPLATES, and NEVER. Supported only for WebSocket APIs.</p> <p>WHEN_NO_MATCH passes the request body for unmapped content types through to the integration backend without transformation.</p> <p>NEVER rejects unmapped content types with an HTTP 415 Unsupported Media Type response.</p> <p>WHEN_NO_TEMPLATES allows pass-through when the integration has no content types mapped to templates. However, if there is at least one content type defined, unmapped content types will be rejected with the same HTTP 415 Unsupported Media Type response.</p> |
| `payload_format_version` | String | <p>Specifies the format of the payload sent to an integration. Required for HTTP APIs. Supported values for Lambda proxy integrations are 1.0 and 2.0. For all other integrations, 1.0 is the only supported value. To learn more, see <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-develop-integrations-lambda.html">Working with AWS Lambda proxy integrations for HTTP APIs</a>.</p> |
| `request_parameters` | HashMap<String, String> | <p>For WebSocket APIs, a key-value map specifying request parameters that are passed from the method request to the backend. The key is an integration request parameter name and the associated value is a method request parameter value or static value that must be enclosed within single quotes and pre-encoded as required by the backend. The method request parameter value must match the pattern of method.request.<replaceable>{location}</replaceable>.<replaceable>{name}</replaceable>
          , where 
            <replaceable>{location}</replaceable>
           is querystring, path, or header; and 
            <replaceable>{name}</replaceable>
           must be a valid and unique method request parameter name.</p> <p>For HTTP API integrations with a specified integrationSubtype, request parameters are a key-value map specifying parameters that are passed to AWS_PROXY integrations. You can provide static values, or map request data, stage variables, or context variables that are evaluated at runtime. To learn more, see <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-develop-integrations-aws-services.html">Working with AWS service integrations for HTTP APIs</a>.</p> <p>For HTTP API integrations, without a specified integrationSubtype request parameters are a key-value map specifying how to transform HTTP requests before sending them to backend integrations. The key should follow the pattern &lt;action&gt;:&lt;header|querystring|path&gt;.&lt;location&gt;. The action can be append, overwrite or remove. For values, you can provide static values, or map request data, stage variables, or context variables that are evaluated at runtime. To learn more, see <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-parameter-mapping.html">Transforming API requests and responses</a>.</p> |
| `connection_type` | String | <p>The type of the network connection to the integration endpoint. Specify INTERNET for connections through the public routable internet or VPC_LINK for private connections between API Gateway and resources in a VPC. The default value is INTERNET.</p> |
| `template_selection_expression` | String | <p>The template selection expression for the integration. Supported only for WebSocket APIs.</p> |
| `content_handling_strategy` | String | <p>Supported only for WebSocket APIs. Specifies how to handle response payload content type conversions. Supported values are CONVERT_TO_BINARY and CONVERT_TO_TEXT, with the following behaviors:</p> <p>CONVERT_TO_BINARY: Converts a response payload from a Base64-encoded string to the corresponding binary blob.</p> <p>CONVERT_TO_TEXT: Converts a response payload from a binary blob to a Base64-encoded string.</p> <p>If this property is not defined, the response payload will be passed through from the integration response to the route response or method response without modification.</p> |
| `integration_method` | String | <p>Specifies the integration's HTTP method type.</p> |
| `integration_subtype` | String | <p>Supported only for HTTP API AWS_PROXY integrations. Specifies the AWS service action to invoke. To learn more, see <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-develop-integrations-aws-services-reference.html">Integration subtype reference</a>.</p> |
| `request_templates` | HashMap<String, String> | <p>Represents a map of Velocity templates that are applied on the request payload based on the value of the Content-Type header sent by the client. The content type value is the key in this map, and the template (as a String) is the value. Supported only for WebSocket APIs.</p> |
| `api_gateway_managed` | bool | <p>Specifies whether an integration is managed by API Gateway. If you created an API using using quick create, the resulting integration is managed by API Gateway. You can update a managed integration, but you can't delete it.</p> |
| `tls_config` | String | <p>The TLS configuration for a private integration. If you specify a TLS configuration, private integration traffic uses the HTTPS protocol. Supported only for HTTP APIs.</p> |
| `credentials_arn` | String | <p>Specifies the credentials required for the integration, if any. For AWS integrations, three options are available. To specify an IAM Role for API Gateway to assume, use the role's Amazon Resource Name (ARN). To require that the caller's identity be passed through from the request, specify the string arn:aws:iam::*:user/*. To use resource-based permissions on supported AWS services, specify null.</p> |
| `integration_response_selection_expression` | String | <p>The integration response selection expression for the integration. Supported only for WebSocket APIs. See <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api-selection-expressions.html#apigateway-websocket-api-integration-response-selection-expressions">Integration Response Selection Expressions</a>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create integration
integration = provider.apigatewayv2.Integration {
    integration_type = "value"  # <p>The integration type of an integration. One of the following:</p> <p>AWS: for integrating the route or method request with an AWS service action, including the Lambda function-invoking action. With the Lambda function-invoking action, this is referred to as the Lambda custom integration. With any other AWS service action, this is known as AWS integration. Supported only for WebSocket APIs.</p> <p>AWS_PROXY: for integrating the route or method request with a Lambda function or other AWS service action. This integration is also referred to as a Lambda proxy integration.</p> <p>HTTP: for integrating the route or method request with an HTTP endpoint. This integration is also referred to as the HTTP custom integration. Supported only for WebSocket APIs.</p> <p>HTTP_PROXY: for integrating the route or method request with an HTTP endpoint, with the client request passed through as-is. This is also referred to as HTTP proxy integration. For HTTP API private integrations, use an HTTP_PROXY integration.</p> <p>MOCK: for integrating the route or method request with API Gateway as a "loopback" endpoint without invoking any backend. Supported only for WebSocket APIs.</p>
    api_id = "value"  # <p>The API identifier.</p>
}

# Access integration outputs
integration_id = integration.id
integration_timeout_in_millis = integration.timeout_in_millis
integration_connection_id = integration.connection_id
integration_integration_id = integration.integration_id
integration_integration_type = integration.integration_type
integration_response_parameters = integration.response_parameters
integration_description = integration.description
integration_integration_uri = integration.integration_uri
integration_passthrough_behavior = integration.passthrough_behavior
integration_payload_format_version = integration.payload_format_version
integration_request_parameters = integration.request_parameters
integration_connection_type = integration.connection_type
integration_template_selection_expression = integration.template_selection_expression
integration_content_handling_strategy = integration.content_handling_strategy
integration_integration_method = integration.integration_method
integration_integration_subtype = integration.integration_subtype
integration_request_templates = integration.request_templates
integration_api_gateway_managed = integration.api_gateway_managed
integration_tls_config = integration.tls_config
integration_credentials_arn = integration.credentials_arn
integration_integration_response_selection_expression = integration.integration_response_selection_expression
```

---


### Api_mappings

ApiMappings resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `items` | Vec<String> | <p>The elements from this collection.</p> |
| `next_token` | String | <p>The next page of elements from this collection. Not valid for the last element of the collection.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access api_mappings outputs
api_mappings_id = api_mappings.id
api_mappings_items = api_mappings.items
api_mappings_next_token = api_mappings.next_token
```

---


### Integrations

Integrations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `items` | Vec<String> | <p>The elements from this collection.</p> |
| `next_token` | String | <p>The next page of elements from this collection. Not valid for the last element of the collection.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access integrations outputs
integrations_id = integrations.id
integrations_items = integrations.items
integrations_next_token = integrations.next_token
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple vpc_links resources
vpc_links_0 = provider.apigatewayv2.Vpc_links {
}
vpc_links_1 = provider.apigatewayv2.Vpc_links {
}
vpc_links_2 = provider.apigatewayv2.Vpc_links {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    vpc_links = provider.apigatewayv2.Vpc_links {
    }
```

---

## Related Documentation

- [AWS Apigatewayv2 Documentation](https://docs.aws.amazon.com/apigatewayv2/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
