# Api_gateway Service



**Resources**: 48

---

## Overview

The api_gateway service provides access to 48 resource types:

- [Integration_response](#integration_response) [CRUD]
- [Tags](#tags) [R]
- [Deployments](#deployments) [R]
- [Model](#model) [CRUD]
- [Method_response](#method_response) [CRUD]
- [Gateway_responses](#gateway_responses) [R]
- [Domain_name](#domain_name) [CRUD]
- [Export](#export) [R]
- [Request_validator](#request_validator) [CRUD]
- [Rest_apis](#rest_apis) [R]
- [Sdk_type](#sdk_type) [R]
- [Usage_plan_key](#usage_plan_key) [CRD]
- [Sdk_types](#sdk_types) [R]
- [Domain_name_access_association](#domain_name_access_association) [CD]
- [Stages](#stages) [R]
- [Usage_plans](#usage_plans) [R]
- [Request_validators](#request_validators) [R]
- [Authorizers](#authorizers) [R]
- [Documentation_parts](#documentation_parts) [R]
- [Domain_names](#domain_names) [R]
- [Deployment](#deployment) [CRUD]
- [Authorizer](#authorizer) [CRUD]
- [Api_keys](#api_keys) [R]
- [Gateway_response](#gateway_response) [CRUD]
- [Client_certificate](#client_certificate) [RUD]
- [Documentation_part](#documentation_part) [CRUD]
- [Usage_plan](#usage_plan) [CRUD]
- [Integration](#integration) [CRUD]
- [Method](#method) [CRUD]
- [Base_path_mappings](#base_path_mappings) [R]
- [Rest_api](#rest_api) [CRUD]
- [Vpc_link](#vpc_link) [CRUD]
- [Model_template](#model_template) [R]
- [Models](#models) [R]
- [Vpc_links](#vpc_links) [R]
- [Resources](#resources) [R]
- [Api_key](#api_key) [CRUD]
- [Stage](#stage) [CRUD]
- [Account](#account) [RU]
- [Base_path_mapping](#base_path_mapping) [CRUD]
- [Domain_name_access_associations](#domain_name_access_associations) [R]
- [Documentation_version](#documentation_version) [CRUD]
- [Usage_plan_keys](#usage_plan_keys) [R]
- [Resource](#resource) [CRUD]
- [Usage](#usage) [RU]
- [Documentation_versions](#documentation_versions) [R]
- [Client_certificates](#client_certificates) [R]
- [Sdk](#sdk) [R]

---

## Resources


### Integration_response

IntegrationResponse resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `content_handling` | String |  | <p>Specifies how to handle response payload content type conversions. Supported values are <code>CONVERT_TO_BINARY</code> and <code>CONVERT_TO_TEXT</code>, with the following behaviors:</p>
         <p>If this property is not defined, the response payload will be passed through from the integration response to the method response without modification.</p> |
| `rest_api_id` | String | ✅ | <p>The string identifier of the associated RestApi.</p> |
| `selection_pattern` | String |  | <p>Specifies the selection pattern of a put integration response.</p> |
| `resource_id` | String | ✅ | <p>Specifies a put integration response request's resource identifier.</p> |
| `http_method` | String | ✅ | <p>Specifies a put integration response request's HTTP method.</p> |
| `status_code` | String | ✅ | <p>Specifies the status code that is used to map the integration response to an existing MethodResponse.</p> |
| `response_parameters` | String |  | <p>A key-value map specifying response parameters that are passed to the method response from the back end.
            The key is a method response header parameter name and the mapped value is an integration response header value, a static value enclosed within a pair of single quotes, or a JSON expression from the integration response body. The mapping key must match the pattern of <code>method.response.header.{name}</code>, where <code>name</code> is a valid and unique header name. The mapped non-static value must match the pattern of <code>integration.response.header.{name}</code> or <code>integration.response.body.{JSON-expression}</code>, where <code>name</code> must be a valid and unique response header name and <code>JSON-expression</code> a valid JSON expression without the <code>$</code> prefix.</p> |
| `response_templates` | String |  | <p>Specifies a put integration response's templates.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `response_templates` | String | <p>Specifies the templates used to transform the integration response body. Response templates are represented as a key/value map, with a content-type as the key and a template as the value.</p> |
| `content_handling` | String | <p>Specifies how to handle response payload content type conversions. Supported values are <code>CONVERT_TO_BINARY</code> and <code>CONVERT_TO_TEXT</code>, with the following behaviors:</p>
         <p>If this property is not defined, the response payload will be passed through from the integration response to the method response without modification.</p> |
| `selection_pattern` | String | <p>Specifies the regular expression (regex) pattern used to choose an integration response based on the response from the back end. For example, if the success response returns nothing and the error response returns some string, you could use the <code>.+</code> regex to match error response. However, make sure that the error response does not contain any newline (<code>\n</code>) character in such cases. If the back end is an Lambda function, the Lambda function error header is matched. For all other HTTP and Amazon Web Services back ends, the HTTP status code is matched.</p> |
| `status_code` | String | <p>Specifies the status code that is used to map the integration response to an existing MethodResponse.</p> |
| `response_parameters` | String | <p>A key-value map specifying response parameters that are passed to the method response from the back end.
            The key is a method response header parameter name and the mapped value is an integration response header value, a static value enclosed within a pair of single quotes, or a JSON expression from the integration response body. The mapping key must match the pattern of <code>method.response.header.{name}</code>, where <code>name</code> is a valid and unique header name. The mapped non-static value must match the pattern of <code>integration.response.header.{name}</code> or <code>integration.response.body.{JSON-expression}</code>, where <code>name</code> is a valid and unique response header name and <code>JSON-expression</code> is a valid JSON expression without the <code>$</code> prefix.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create integration_response
integration_response = provider.api_gateway.Integration_response {
    rest_api_id = "value"  # <p>The string identifier of the associated RestApi.</p>
    resource_id = "value"  # <p>Specifies a put integration response request's resource identifier.</p>
    http_method = "value"  # <p>Specifies a put integration response request's HTTP method.</p>
    status_code = "value"  # <p>Specifies the status code that is used to map the integration response to an existing MethodResponse.</p>
}

# Access integration_response outputs
integration_response_id = integration_response.id
integration_response_response_templates = integration_response.response_templates
integration_response_content_handling = integration_response.content_handling
integration_response_selection_pattern = integration_response.selection_pattern
integration_response_status_code = integration_response.status_code
integration_response_response_parameters = integration_response.response_parameters
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
| `tags` | String | <p>The collection of tags. Each tag element is associated with a given resource.</p> |


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


### Deployments

Deployments resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `position` | String | <p>The current pagination position in the paged result set.</p> |
| `items` | Vec<String> | <p>The current page of elements from this collection.</p> |


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
deployments_position = deployments.position
deployments_items = deployments.items
```

---


### Model

Model resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `schema` | String |  | <p>The schema for the model. For <code>application/json</code> models, this should be JSON schema draft 4 model. The maximum size of the model is 400 KB.</p> |
| `description` | String |  | <p>The description of the model.</p> |
| `content_type` | String | ✅ | <p>The content-type for the model.</p> |
| `rest_api_id` | String | ✅ | <p>The RestApi identifier under which the Model will be created.</p> |
| `name` | String | ✅ | <p>The name of the model. Must be alphanumeric.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `content_type` | String | <p>The content-type for the model.</p> |
| `schema` | String | <p>The schema for the model. For <code>application/json</code> models, this should be JSON schema draft 4 model. Do not include "\*/" characters in the description of any properties because such "\*/" characters may be interpreted as the closing marker for comments in some languages, such as Java or JavaScript, causing the installation of your API's SDK generated by API Gateway to fail.</p> |
| `name` | String | <p>The name of the model. Must be an alphanumeric string.</p> |
| `description` | String | <p>The description of the model.</p> |
| `id` | String | <p>The identifier for the model resource.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create model
model = provider.api_gateway.Model {
    content_type = "value"  # <p>The content-type for the model.</p>
    rest_api_id = "value"  # <p>The RestApi identifier under which the Model will be created.</p>
    name = "value"  # <p>The name of the model. Must be alphanumeric.</p>
}

# Access model outputs
model_id = model.id
model_content_type = model.content_type
model_schema = model.schema
model_name = model.name
model_description = model.description
model_id = model.id
```

---


### Method_response

MethodResponse resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `rest_api_id` | String | ✅ | <p>The string identifier of the associated RestApi.</p> |
| `resource_id` | String | ✅ | <p>The Resource identifier for the Method resource.</p> |
| `http_method` | String | ✅ | <p>The HTTP verb of the Method resource.</p> |
| `response_parameters` | HashMap<String, bool> |  | <p>A key-value map specifying required or optional response parameters that API Gateway can send back to the caller. A key defines a method response header name and the associated value is a Boolean flag indicating whether the method response parameter is required or not. The method response header names must match the pattern of <code>method.response.header.{name}</code>, where <code>name</code> is a valid and unique header name. The response parameter names defined here are available in the integration response to be mapped from an integration response header expressed in <code>integration.response.header.{name}</code>, a static value enclosed within a pair of single quotes (e.g., <code>'application/json'</code>), or a JSON expression from the back-end response payload in the form of <code>integration.response.body.{JSON-expression}</code>, where <code>JSON-expression</code> is a valid JSON expression without the <code>$</code> prefix.)</p> |
| `response_models` | String |  | <p>Specifies the Model resources used for the response's content type. Response models are represented as a key/value map, with a content type as the key and a Model name as the value.</p> |
| `status_code` | String | ✅ | <p>The method response's status code.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status_code` | String | <p>The method response's status code.</p> |
| `response_parameters` | HashMap<String, bool> | <p>A key-value map specifying required or optional response parameters that API Gateway can send back to the caller. A key defines a method response header and the value specifies whether the associated method response header is required or not. The expression of the key must match the pattern <code>method.response.header.{name}</code>, where <code>name</code> is a valid and unique header name. API Gateway passes certain integration response data to the method response headers specified here according to the mapping you prescribe in the API's IntegrationResponse. The integration response data that can be mapped include an integration response header expressed in <code>integration.response.header.{name}</code>, a static value enclosed within a pair of single quotes (e.g., <code>'application/json'</code>), or a JSON expression from the back-end response payload in the form of <code>integration.response.body.{JSON-expression}</code>, where <code>JSON-expression</code> is a valid JSON expression without the <code>$</code> prefix.)</p> |
| `response_models` | String | <p>Specifies the Model resources used for the response's content-type. Response models are represented as a key/value map, with a content-type as the key and a Model name as the value.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create method_response
method_response = provider.api_gateway.Method_response {
    rest_api_id = "value"  # <p>The string identifier of the associated RestApi.</p>
    resource_id = "value"  # <p>The Resource identifier for the Method resource.</p>
    http_method = "value"  # <p>The HTTP verb of the Method resource.</p>
    status_code = "value"  # <p>The method response's status code.</p>
}

# Access method_response outputs
method_response_id = method_response.id
method_response_status_code = method_response.status_code
method_response_response_parameters = method_response.response_parameters
method_response_response_models = method_response.response_models
```

---


### Gateway_responses

GatewayResponses resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `items` | Vec<String> | <p>Returns the entire collection, because of no pagination support.</p> |
| `position` | String | <p>The current pagination position in the paged result set. The GatewayResponse collection does not support pagination and the position does not apply here.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access gateway_responses outputs
gateway_responses_id = gateway_responses.id
gateway_responses_items = gateway_responses.items
gateway_responses_position = gateway_responses.position
```

---


### Domain_name

DomainName resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `endpoint_configuration` | String |  | <p>The endpoint configuration of this DomainName showing the endpoint types and IP address types of the domain name. </p> |
| `certificate_private_key` | String |  | <p>[Deprecated] Your edge-optimized endpoint's domain name certificate's private key.</p> |
| `certificate_body` | String |  | <p>[Deprecated] The body of the server certificate that will be used by edge-optimized endpoint or private endpoint for this domain name provided by your certificate authority.</p> |
| `certificate_arn` | String |  | <p>The reference to an Amazon Web Services-managed certificate that will be used by edge-optimized endpoint or private endpoint for this domain name. Certificate Manager is the only supported source.</p> |
| `tags` | String |  | <p>The key-value map of strings. The valid character set is [a-zA-Z+-=._:/]. The tag key can be up to 128 characters and must not start with <code>aws:</code>. The tag value can be up to 256 characters.</p> |
| `ownership_verification_certificate_arn` | String |  | <p>The ARN of the public certificate issued by ACM to validate ownership of your custom
      domain. Only required when configuring mutual TLS and using an ACM imported or private CA
      certificate ARN as the regionalCertificateArn.</p> |
| `certificate_chain` | String |  | <p>[Deprecated] The intermediate certificates and optionally the root certificate, one after the other without any blank lines, used by an edge-optimized endpoint for this domain name. If you include the root certificate, your certificate chain must start with intermediate certificates and end with the root certificate. Use the intermediate certificates that were provided by your certificate authority. Do not include any intermediaries that are not in the chain of trust path.</p> |
| `security_policy` | String |  | <p>The Transport Layer Security (TLS) version + cipher suite for this DomainName. The valid values are <code>TLS_1_0</code> and <code>TLS_1_2</code>.</p> |
| `policy` | String |  | <p>A stringified JSON policy document that applies to the <code>execute-api</code> service for this DomainName regardless of the caller and Method
      configuration. Supported only for private custom
      domain names.</p> |
| `regional_certificate_arn` | String |  | <p>The reference to an Amazon Web Services-managed certificate that will be used by regional endpoint for this domain name. Certificate Manager is the only supported source.</p> |
| `domain_name` | String | ✅ | <p>The name of the DomainName resource.</p> |
| `certificate_name` | String |  | <p>The user-friendly name of the certificate that will be used by edge-optimized endpoint or private endpoint for this domain name.</p> |
| `routing_mode` | String |  | <p>
The routing mode for this domain name. The routing mode determines how API Gateway sends traffic from your custom domain name to your private APIs.
</p> |
| `regional_certificate_name` | String |  | <p>The user-friendly name of the certificate that will be used by regional endpoint for this domain name.</p> |
| `mutual_tls_authentication` | String |  |  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `regional_hosted_zone_id` | String | <p>The region-specific Amazon Route 53 Hosted Zone ID of the regional endpoint. For more information, see Set up a Regional Custom Domain Name and AWS Regions and Endpoints for API Gateway. </p> |
| `regional_certificate_arn` | String | <p>The reference to an Amazon Web Services-managed certificate that will be used for validating the regional domain name. Certificate Manager is the only supported source.</p> |
| `domain_name` | String | <p>The custom domain name as an API host name, for example, <code>my-api.example.com</code>.</p> |
| `distribution_domain_name` | String | <p>The domain name of the Amazon CloudFront distribution associated with this custom domain name for an edge-optimized endpoint. You set up this association when adding a DNS record pointing the custom domain name to this distribution name. For more information about CloudFront distributions, see the Amazon CloudFront documentation.</p> |
| `endpoint_configuration` | String | <p>The endpoint configuration of this DomainName showing the endpoint types and IP address types of the domain name. </p> |
| `mutual_tls_authentication` | String | <p>The mutual TLS authentication configuration for a custom domain name. If specified, API Gateway
      performs two-way authentication between the client and the server. Clients must present a
      trusted certificate to access your API.</p> |
| `routing_mode` | String | <p>The routing mode for this domain name. The routing mode determines how API Gateway sends traffic from your custom domain name to your private APIs.</p> |
| `policy` | String | <p>A stringified JSON policy document that applies to the <code>execute-api</code> service for this DomainName regardless of the caller and Method
      configuration. Supported only for private custom
      domain names.</p> |
| `domain_name_status_message` | String | <p>An optional text message containing detailed information about status of the DomainName migration.</p> |
| `certificate_upload_date` | String | <p>The timestamp when the certificate that was used by edge-optimized endpoint or private endpoint for this domain name was uploaded.</p> |
| `regional_domain_name` | String | <p>The domain name associated with the regional endpoint for this custom domain name. You set up this association by adding a DNS record that points the custom domain name to this regional domain name. The regional domain name is returned by API Gateway when you create a regional endpoint.</p> |
| `tags` | String | <p>The collection of tags. Each tag element is associated with a given resource.</p> |
| `management_policy` | String | <p>A stringified JSON policy document that applies to the API Gateway Management service for this DomainName. This policy document controls access for access association sources to create domain name access associations with this DomainName. Supported only for private custom domain names.</p> |
| `security_policy` | String | <p>The Transport Layer Security (TLS) version + cipher suite for this DomainName. The valid values are <code>TLS_1_0</code> and <code>TLS_1_2</code>.</p> |
| `certificate_name` | String | <p>The name of the certificate that will be used by edge-optimized endpoint or private endpoint for this domain name.</p> |
| `distribution_hosted_zone_id` | String | <p>The region-agnostic Amazon Route 53 Hosted Zone ID of the edge-optimized endpoint. The valid value is <code>Z2FDTNDATAQYW2</code> for all the regions. For more information, see Set up a Regional Custom Domain Name and AWS Regions and Endpoints for API Gateway. </p> |
| `domain_name_status` | String | <p>The status of the DomainName migration. The valid values are <code>AVAILABLE</code> and <code>UPDATING</code>. If the status is <code>UPDATING</code>, the domain cannot be modified further until the existing operation is complete. If it is <code>AVAILABLE</code>, the domain can be updated.</p> |
| `regional_certificate_name` | String | <p>The name of the certificate that will be used for validating the regional domain name.</p> |
| `ownership_verification_certificate_arn` | String | <p>The ARN of the public certificate issued by ACM to validate ownership of your custom
      domain. Only required when configuring mutual TLS and using an ACM imported or private CA
      certificate ARN as the regionalCertificateArn.</p> |
| `domain_name_arn` | String | <p>The ARN of the domain name.
</p> |
| `domain_name_id` | String | <p>The identifier for the domain name resource. Supported only for private custom domain names.</p> |
| `certificate_arn` | String | <p>The reference to an Amazon Web Services-managed certificate that will be used by edge-optimized endpoint or private endpoint for this domain name. Certificate Manager is the only supported source.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create domain_name
domain_name = provider.api_gateway.Domain_name {
    domain_name = "value"  # <p>The name of the DomainName resource.</p>
}

# Access domain_name outputs
domain_name_id = domain_name.id
domain_name_regional_hosted_zone_id = domain_name.regional_hosted_zone_id
domain_name_regional_certificate_arn = domain_name.regional_certificate_arn
domain_name_domain_name = domain_name.domain_name
domain_name_distribution_domain_name = domain_name.distribution_domain_name
domain_name_endpoint_configuration = domain_name.endpoint_configuration
domain_name_mutual_tls_authentication = domain_name.mutual_tls_authentication
domain_name_routing_mode = domain_name.routing_mode
domain_name_policy = domain_name.policy
domain_name_domain_name_status_message = domain_name.domain_name_status_message
domain_name_certificate_upload_date = domain_name.certificate_upload_date
domain_name_regional_domain_name = domain_name.regional_domain_name
domain_name_tags = domain_name.tags
domain_name_management_policy = domain_name.management_policy
domain_name_security_policy = domain_name.security_policy
domain_name_certificate_name = domain_name.certificate_name
domain_name_distribution_hosted_zone_id = domain_name.distribution_hosted_zone_id
domain_name_domain_name_status = domain_name.domain_name_status
domain_name_regional_certificate_name = domain_name.regional_certificate_name
domain_name_ownership_verification_certificate_arn = domain_name.ownership_verification_certificate_arn
domain_name_domain_name_arn = domain_name.domain_name_arn
domain_name_domain_name_id = domain_name.domain_name_id
domain_name_certificate_arn = domain_name.certificate_arn
```

---


### Export

Export resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `body` | String | <p>The binary blob response to GetExport, which contains the export.</p> |
| `content_disposition` | String | <p>The content-disposition header value in the HTTP response.</p> |
| `content_type` | String | <p>The content-type header value in the HTTP response. This will correspond to a valid 'accept' type in the request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access export outputs
export_id = export.id
export_body = export.body
export_content_disposition = export.content_disposition
export_content_type = export.content_type
```

---


### Request_validator

RequestValidator resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | <p>The name of the to-be-created RequestValidator.</p> |
| `validate_request_parameters` | bool |  | <p>A Boolean flag to indicate whether to validate request parameters, <code>true</code>, or not <code>false</code>.</p> |
| `validate_request_body` | bool |  | <p>A Boolean flag to indicate whether to validate request body according to the configured model schema for the method (<code>true</code>) or not (<code>false</code>).</p> |
| `rest_api_id` | String | ✅ | <p>The string identifier of the associated RestApi.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `validate_request_body` | bool | <p>A Boolean flag to indicate whether to validate a request body according to the configured Model schema.</p> |
| `name` | String | <p>The name of this RequestValidator</p> |
| `validate_request_parameters` | bool | <p>A Boolean flag to indicate whether to validate request parameters (<code>true</code>) or not (<code>false</code>).</p> |
| `id` | String | <p>The identifier of this RequestValidator.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create request_validator
request_validator = provider.api_gateway.Request_validator {
    rest_api_id = "value"  # <p>The string identifier of the associated RestApi.</p>
}

# Access request_validator outputs
request_validator_id = request_validator.id
request_validator_validate_request_body = request_validator.validate_request_body
request_validator_name = request_validator.name
request_validator_validate_request_parameters = request_validator.validate_request_parameters
request_validator_id = request_validator.id
```

---


### Rest_apis

RestApis resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `items` | Vec<String> | <p>The current page of elements from this collection.</p> |
| `position` | String | <p>The current pagination position in the paged result set.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access rest_apis outputs
rest_apis_id = rest_apis.id
rest_apis_items = rest_apis.items
rest_apis_position = rest_apis.position
```

---


### Sdk_type

SdkType resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | <p>The description of an SdkType.</p> |
| `configuration_properties` | Vec<String> | <p>A list of configuration properties of an SdkType.</p> |
| `id` | String | <p>The identifier of an SdkType instance.</p> |
| `friendly_name` | String | <p>The user-friendly name of an SdkType instance.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access sdk_type outputs
sdk_type_id = sdk_type.id
sdk_type_description = sdk_type.description
sdk_type_configuration_properties = sdk_type.configuration_properties
sdk_type_id = sdk_type.id
sdk_type_friendly_name = sdk_type.friendly_name
```

---


### Usage_plan_key

UsagePlanKey resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `key_type` | String | ✅ | <p>The type of a UsagePlanKey resource for a plan customer.</p> |
| `usage_plan_id` | String | ✅ | <p>The Id of the UsagePlan resource representing the usage plan containing the to-be-created UsagePlanKey resource representing a plan customer.</p> |
| `key_id` | String | ✅ | <p>The identifier of a UsagePlanKey resource for a plan customer.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | <p>The name of a usage plan key.</p> |
| `id` | String | <p>The Id of a usage plan key.</p> |
| `value` | String | <p>The value of a usage plan key.</p> |
| `type` | String | <p>The type of a usage plan key. Currently, the valid key type is <code>API_KEY</code>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create usage_plan_key
usage_plan_key = provider.api_gateway.Usage_plan_key {
    key_type = "value"  # <p>The type of a UsagePlanKey resource for a plan customer.</p>
    usage_plan_id = "value"  # <p>The Id of the UsagePlan resource representing the usage plan containing the to-be-created UsagePlanKey resource representing a plan customer.</p>
    key_id = "value"  # <p>The identifier of a UsagePlanKey resource for a plan customer.</p>
}

# Access usage_plan_key outputs
usage_plan_key_id = usage_plan_key.id
usage_plan_key_name = usage_plan_key.name
usage_plan_key_id = usage_plan_key.id
usage_plan_key_value = usage_plan_key.value
usage_plan_key_type = usage_plan_key.type
```

---


### Sdk_types

SdkTypes resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `items` | Vec<String> | <p>The current page of elements from this collection.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access sdk_types outputs
sdk_types_id = sdk_types.id
sdk_types_items = sdk_types.items
```

---


### Domain_name_access_association

DomainNameAccessAssociation resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `domain_name_arn` | String | ✅ | <p>
  The ARN of the domain name.
</p> |
| `access_association_source_type` | String | ✅ | <p>
The type of the domain name access association source.
</p> |
| `tags` | String |  | <p>The key-value map of strings. The valid character set is [a-zA-Z+-=._:/]. The tag key can be up to 128 characters and must not start with <code>aws:</code>. The tag value can be up to 256 characters.</p> |
| `access_association_source` | String | ✅ | <p>
The identifier of the domain name access association source. For a VPCE, the value is the VPC endpoint ID.
</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create domain_name_access_association
domain_name_access_association = provider.api_gateway.Domain_name_access_association {
    domain_name_arn = "value"  # <p>
  The ARN of the domain name.
</p>
    access_association_source_type = "value"  # <p>
The type of the domain name access association source.
</p>
    access_association_source = "value"  # <p>
The identifier of the domain name access association source. For a VPCE, the value is the VPC endpoint ID.
</p>
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
| `item` | Vec<String> | <p>The current page of elements from this collection.</p> |


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
stages_item = stages.item
```

---


### Usage_plans

UsagePlans resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `position` | String | <p>The current pagination position in the paged result set.</p> |
| `items` | Vec<String> | <p>The current page of elements from this collection.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access usage_plans outputs
usage_plans_id = usage_plans.id
usage_plans_position = usage_plans.position
usage_plans_items = usage_plans.items
```

---


### Request_validators

RequestValidators resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `items` | Vec<String> | <p>The current page of elements from this collection.</p> |
| `position` | String | <p>The current pagination position in the paged result set.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access request_validators outputs
request_validators_id = request_validators.id
request_validators_items = request_validators.items
request_validators_position = request_validators.position
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
| `items` | Vec<String> | <p>The current page of elements from this collection.</p> |
| `position` | String | <p>The current pagination position in the paged result set.</p> |


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
authorizers_position = authorizers.position
```

---


### Documentation_parts

DocumentationParts resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `items` | Vec<String> | <p>The current page of elements from this collection.</p> |
| `position` | String | <p>The current pagination position in the paged result set.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access documentation_parts outputs
documentation_parts_id = documentation_parts.id
documentation_parts_items = documentation_parts.items
documentation_parts_position = documentation_parts.position
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
| `items` | Vec<String> | <p>The current page of elements from this collection.</p> |
| `position` | String | <p>The current pagination position in the paged result set.</p> |


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
domain_names_items = domain_names.items
domain_names_position = domain_names.position
```

---


### Deployment

Deployment resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `stage_description` | String |  | <p>The description of the Stage resource for the Deployment resource to create.</p> |
| `tracing_enabled` | bool |  | <p>Specifies whether active tracing with X-ray is enabled for the Stage.</p> |
| `variables` | String |  | <p>A map that defines the stage variables for the Stage resource that is associated
          with the new deployment. Variable names can have alphanumeric and underscore characters, and the values
          must match <code>[A-Za-z0-9-._~:/?#&=,]+</code>.</p> |
| `rest_api_id` | String | ✅ | <p>The string identifier of the associated RestApi.</p> |
| `cache_cluster_size` | String |  | <p>The stage's cache capacity in GB. For more information about choosing a cache size, see <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-caching.html">Enabling API caching to enhance responsiveness</a>.</p> |
| `cache_cluster_enabled` | bool |  | <p>Enables a cache cluster for the Stage resource specified in the input.</p> |
| `stage_name` | String |  | <p>The name of the Stage resource for the Deployment resource to create.</p> |
| `description` | String |  | <p>The description for the Deployment resource to create.</p> |
| `canary_settings` | String |  | <p>The input configuration for the canary deployment when the deployment is a canary release deployment. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `created_date` | String | <p>The date and time that the deployment resource was created.</p> |
| `api_summary` | HashMap<String, HashMap<String, String>> | <p>A summary of the RestApi at the date and time that the deployment resource was created.</p> |
| `description` | String | <p>The description for the deployment resource.</p> |
| `id` | String | <p>The identifier for the deployment resource.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create deployment
deployment = provider.api_gateway.Deployment {
    rest_api_id = "value"  # <p>The string identifier of the associated RestApi.</p>
}

# Access deployment outputs
deployment_id = deployment.id
deployment_created_date = deployment.created_date
deployment_api_summary = deployment.api_summary
deployment_description = deployment.description
deployment_id = deployment.id
```

---


### Authorizer

Authorizer resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `identity_source` | String |  | <p>The identity source for which authorization is requested. For a <code>TOKEN</code> or
        <code>COGNITO_USER_POOLS</code> authorizer, this is required and specifies the request
      header mapping expression for the custom header holding the authorization token submitted by
      the client. For example, if the token header name is <code>Auth</code>, the header mapping
      expression is <code>method.request.header.Auth</code>. For the <code>REQUEST</code>
      authorizer, this is required when authorization caching is enabled. The value is a
      comma-separated string of one or more mapping expressions of the specified request parameters.
      For example, if an <code>Auth</code> header, a <code>Name</code> query string parameter are
      defined as identity sources, this value is <code>method.request.header.Auth,
        method.request.querystring.Name</code>. These parameters will be used to derive the
      authorization caching key and to perform runtime validation of the <code>REQUEST</code>
      authorizer by verifying all of the identity-related request parameters are present, not null
      and non-empty. Only when this is true does the authorizer invoke the authorizer Lambda
      function, otherwise, it returns a 401 Unauthorized response without calling the Lambda
      function. The valid value is a string of comma-separated mapping expressions of the specified
      request parameters. When the authorization caching is not enabled, this property is
      optional.</p> |
| `authorizer_result_ttl_in_seconds` | i64 |  | <p>The TTL in seconds of cached authorizer results. If it equals 0, authorization caching is disabled. If it is greater than 0, API Gateway will cache authorizer responses. If this field is not set, the default value is 300. The maximum value is 3600, or 1 hour.</p> |
| `type` | String | ✅ | <p>The authorizer type. Valid values are <code>TOKEN</code> for a Lambda function using a single authorization token submitted in a custom header, <code>REQUEST</code> for a Lambda function using incoming request parameters, and <code>COGNITO_USER_POOLS</code> for using an Amazon Cognito user pool.</p> |
| `auth_type` | String |  | <p>Optional customer-defined field, used in OpenAPI imports and exports without functional impact.</p> |
| `authorizer_uri` | String |  | <p>Specifies the authorizer's Uniform Resource Identifier (URI). For <code>TOKEN</code> or <code>REQUEST</code> authorizers, this must be a well-formed Lambda function URI, for example, <code>arn:aws:apigateway:us-west-2:lambda:path/2015-03-31/functions/arn:aws:lambda:us-west-2:{account_id}:function:{lambda_function_name}/invocations</code>. In general, the URI has this form  <code>arn:aws:apigateway:{region}:lambda:path/{service_api}</code>, where <code>{region}</code> is the same as the region hosting the Lambda function, <code>path</code> indicates that the remaining substring in the URI should be treated as the path to the resource, including the initial <code>/</code>. For Lambda functions, this is usually of the form <code>/2015-03-31/functions/[FunctionARN]/invocations</code>.</p> |
| `authorizer_credentials` | String |  | <p>Specifies the required credentials as an IAM role for API Gateway to invoke the authorizer. To specify an IAM role for API Gateway to assume, use the role's Amazon Resource Name (ARN). To use resource-based permissions on the Lambda function, specify null.</p> |
| `name` | String | ✅ | <p>The name of the authorizer.</p> |
| `identity_validation_expression` | String |  | <p>A validation expression for the incoming identity token. For <code>TOKEN</code> authorizers, this value is a regular expression. For <code>COGNITO_USER_POOLS</code> authorizers, API Gateway will match the <code>aud</code> field of the incoming token from the client against the specified regular expression. It will invoke the authorizer's Lambda function when there is a match. Otherwise, it will return a 401 Unauthorized response without calling the Lambda function. The validation expression does not apply to the <code>REQUEST</code> authorizer.</p> |
| `rest_api_id` | String | ✅ | <p>The string identifier of the associated RestApi.</p> |
| `provider_ar_ns` | Vec<String> |  | <p>A list of the Amazon Cognito user pool ARNs for the <code>COGNITO_USER_POOLS</code> authorizer. Each element is of this format: <code>arn:aws:cognito-idp:{region}:{account_id}:userpool/{user_pool_id}</code>. For a <code>TOKEN</code> or <code>REQUEST</code> authorizer, this is not defined. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `type` | String | <p>The authorizer type. Valid values are <code>TOKEN</code> for a Lambda function using a single authorization token submitted in a custom header, <code>REQUEST</code> for a Lambda function using incoming request parameters, and <code>COGNITO_USER_POOLS</code> for using an Amazon Cognito user pool.</p> |
| `auth_type` | String | <p>Optional customer-defined field, used in OpenAPI imports and exports without functional impact.</p> |
| `provider_ar_ns` | Vec<String> | <p>A list of the Amazon Cognito user pool ARNs for the <code>COGNITO_USER_POOLS</code> authorizer. Each element is of this format: <code>arn:aws:cognito-idp:{region}:{account_id}:userpool/{user_pool_id}</code>. For a <code>TOKEN</code> or <code>REQUEST</code> authorizer, this is not defined. </p> |
| `authorizer_credentials` | String | <p>Specifies the required credentials as an IAM role for API Gateway to invoke the authorizer. To specify an IAM role for API Gateway to assume, use the role's Amazon Resource Name (ARN). To use resource-based permissions on the Lambda function, specify null.</p> |
| `identity_source` | String | <p>The identity source for which authorization is requested. For a <code>TOKEN</code> or
        <code>COGNITO_USER_POOLS</code> authorizer, this is required and specifies the request
      header mapping expression for the custom header holding the authorization token submitted by
      the client. For example, if the token header name is <code>Auth</code>, the header mapping expression is
      <code>method.request.header.Auth</code>. For the <code>REQUEST</code> authorizer, this is required when authorization
      caching is enabled. The value is a comma-separated string of one or more mapping expressions
      of the specified request parameters. For example, if an <code>Auth</code> header, a <code>Name</code> query string
      parameter are defined as identity sources, this value is <code>method.request.header.Auth</code>,
      <code>method.request.querystring.Name</code>. These parameters will be used to derive the authorization
      caching key and to perform runtime validation of the <code>REQUEST</code> authorizer by verifying all of
      the identity-related request parameters are present, not null and non-empty. Only when this is
      true does the authorizer invoke the authorizer Lambda function, otherwise, it returns a 401
      Unauthorized response without calling the Lambda function. The valid value is a string of
      comma-separated mapping expressions of the specified request parameters. When the
      authorization caching is not enabled, this property is optional. </p> |
| `id` | String | <p>The identifier for the authorizer resource.</p> |
| `authorizer_uri` | String | <p>Specifies the authorizer's Uniform Resource Identifier (URI). For <code>TOKEN</code> or <code>REQUEST</code> authorizers, this must be a well-formed Lambda function URI, for example, <code>arn:aws:apigateway:us-west-2:lambda:path/2015-03-31/functions/arn:aws:lambda:us-west-2:{account_id}:function:{lambda_function_name}/invocations</code>. In general, the URI has this form  <code>arn:aws:apigateway:{region}:lambda:path/{service_api}</code>, where <code>{region}</code> is the same as the region hosting the Lambda function, <code>path</code> indicates that the remaining substring in the URI should be treated as the path to the resource, including the initial <code>/</code>. For Lambda functions, this is usually of the form <code>/2015-03-31/functions/[FunctionARN]/invocations</code>.</p> |
| `identity_validation_expression` | String | <p>A validation expression for the incoming identity token. For <code>TOKEN</code> authorizers, this value is a regular expression. For <code>COGNITO_USER_POOLS</code> authorizers, API Gateway will match the <code>aud</code> field of the incoming token from the client against the specified regular expression. It will invoke the authorizer's Lambda function when there is a match. Otherwise, it will return a 401 Unauthorized response without calling the Lambda function. The validation expression does not apply to the <code>REQUEST</code> authorizer.</p> |
| `authorizer_result_ttl_in_seconds` | i64 | <p>The TTL in seconds of cached authorizer results. If it equals 0, authorization caching is disabled. If it is greater than 0, API Gateway will cache authorizer responses. If this field is not set, the default value is 300. The maximum value is 3600, or 1 hour.</p> |
| `name` | String | <p>The name of the authorizer.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create authorizer
authorizer = provider.api_gateway.Authorizer {
    type = "value"  # <p>The authorizer type. Valid values are <code>TOKEN</code> for a Lambda function using a single authorization token submitted in a custom header, <code>REQUEST</code> for a Lambda function using incoming request parameters, and <code>COGNITO_USER_POOLS</code> for using an Amazon Cognito user pool.</p>
    name = "value"  # <p>The name of the authorizer.</p>
    rest_api_id = "value"  # <p>The string identifier of the associated RestApi.</p>
}

# Access authorizer outputs
authorizer_id = authorizer.id
authorizer_type = authorizer.type
authorizer_auth_type = authorizer.auth_type
authorizer_provider_ar_ns = authorizer.provider_ar_ns
authorizer_authorizer_credentials = authorizer.authorizer_credentials
authorizer_identity_source = authorizer.identity_source
authorizer_id = authorizer.id
authorizer_authorizer_uri = authorizer.authorizer_uri
authorizer_identity_validation_expression = authorizer.identity_validation_expression
authorizer_authorizer_result_ttl_in_seconds = authorizer.authorizer_result_ttl_in_seconds
authorizer_name = authorizer.name
```

---


### Api_keys

ApiKeys resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `items` | Vec<String> | <p>The current page of elements from this collection.</p> |
| `position` | String | <p>The current pagination position in the paged result set.</p> |
| `warnings` | String | <p>A list of warning messages logged during the import of API keys when the <code>failOnWarnings</code> option is set to true.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access api_keys outputs
api_keys_id = api_keys.id
api_keys_items = api_keys.items
api_keys_position = api_keys.position
api_keys_warnings = api_keys.warnings
```

---


### Gateway_response

GatewayResponse resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `response_parameters` | String |  | <p>Response parameters (paths, query strings and headers) of the GatewayResponse as a string-to-string map of key-value  pairs.</p> |
| `rest_api_id` | String | ✅ | <p>The string identifier of the associated RestApi.</p> |
| `response_type` | String | ✅ | <p>The response type of the associated GatewayResponse</p> |
| `response_templates` | String |  | <p>Response templates of the GatewayResponse as a string-to-string map of key-value pairs.</p> |
| `status_code` | String |  | <p>The HTTP status code of the GatewayResponse.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `response_parameters` | String | <p>Response parameters (paths, query strings and headers) of the GatewayResponse as a
      string-to-string map of key-value pairs.</p> |
| `response_type` | String | <p>The response type of the associated GatewayResponse.</p> |
| `status_code` | String | <p>The HTTP status code for this GatewayResponse.</p> |
| `response_templates` | String | <p>Response templates of the GatewayResponse as a string-to-string map of key-value pairs.</p> |
| `default_response` | bool | <p>A Boolean flag to indicate whether this GatewayResponse is the default gateway response (<code>true</code>) or not (<code>false</code>). A default gateway response is one generated by API Gateway without any customization by an API developer. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create gateway_response
gateway_response = provider.api_gateway.Gateway_response {
    rest_api_id = "value"  # <p>The string identifier of the associated RestApi.</p>
    response_type = "value"  # <p>The response type of the associated GatewayResponse</p>
}

# Access gateway_response outputs
gateway_response_id = gateway_response.id
gateway_response_response_parameters = gateway_response.response_parameters
gateway_response_response_type = gateway_response.response_type
gateway_response_status_code = gateway_response.status_code
gateway_response_response_templates = gateway_response.response_templates
gateway_response_default_response = gateway_response.default_response
```

---


### Client_certificate

ClientCertificate resource

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_certificate_id` | String | ✅ | <p>The identifier of the ClientCertificate resource to be updated.</p> |
| `patch_operations` | Vec<String> |  | <p>For more information about supported patch operations, see <a href="https://docs.aws.amazon.com/apigateway/latest/api/patch-operations.html">Patch Operations</a>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | <p>The description of the client certificate.</p> |
| `expiration_date` | String | <p>The timestamp when the client certificate will expire.</p> |
| `client_certificate_id` | String | <p>The identifier of the client certificate.</p> |
| `pem_encoded_certificate` | String | <p>The PEM-encoded public key of the client certificate, which can be used to configure certificate authentication in the integration endpoint .</p> |
| `created_date` | String | <p>The timestamp when the client certificate was created.</p> |
| `tags` | String | <p>The collection of tags. Each tag element is associated with a given resource.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access client_certificate outputs
client_certificate_id = client_certificate.id
client_certificate_description = client_certificate.description
client_certificate_expiration_date = client_certificate.expiration_date
client_certificate_client_certificate_id = client_certificate.client_certificate_id
client_certificate_pem_encoded_certificate = client_certificate.pem_encoded_certificate
client_certificate_created_date = client_certificate.created_date
client_certificate_tags = client_certificate.tags
```

---


### Documentation_part

DocumentationPart resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `properties` | String | ✅ | <p>The new documentation content map of the targeted API entity. Enclosed key-value pairs are API-specific, but only OpenAPI-compliant key-value pairs can be exported and, hence, published.</p> |
| `rest_api_id` | String | ✅ | <p>The string identifier of the associated RestApi.</p> |
| `location` | String | ✅ | <p>The location of the targeted API entity of the to-be-created documentation part.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `id` | String | <p>The DocumentationPart identifier, generated by API Gateway when the <code>DocumentationPart</code> is created.</p> |
| `properties` | String | <p>A content map of API-specific key-value pairs describing the targeted API entity. The map must be encoded as a JSON string, e.g., <code>"{ \"description\": \"The API does ...\" }"</code>.  Only OpenAPI-compliant documentation-related fields from the properties map are exported and, hence, published as part of the API entity definitions, while the original documentation parts are exported in a OpenAPI extension of <code>x-amazon-apigateway-documentation</code>.</p> |
| `location` | String | <p>The location of the API entity to which the documentation applies. Valid fields depend on the targeted API entity type. All the valid location fields are not required. If not explicitly specified, a valid location field is treated as a wildcard and associated documentation content may be inherited by matching entities, unless overridden.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create documentation_part
documentation_part = provider.api_gateway.Documentation_part {
    properties = "value"  # <p>The new documentation content map of the targeted API entity. Enclosed key-value pairs are API-specific, but only OpenAPI-compliant key-value pairs can be exported and, hence, published.</p>
    rest_api_id = "value"  # <p>The string identifier of the associated RestApi.</p>
    location = "value"  # <p>The location of the targeted API entity of the to-be-created documentation part.</p>
}

# Access documentation_part outputs
documentation_part_id = documentation_part.id
documentation_part_id = documentation_part.id
documentation_part_properties = documentation_part.properties
documentation_part_location = documentation_part.location
```

---


### Usage_plan

UsagePlan resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | <p>The description of the usage plan.</p> |
| `api_stages` | Vec<String> |  | <p>The associated API stages of the usage plan.</p> |
| `name` | String | ✅ | <p>The name of the usage plan.</p> |
| `tags` | String |  | <p>The key-value map of strings. The valid character set is [a-zA-Z+-=._:/]. The tag key can be up to 128 characters and must not start with <code>aws:</code>. The tag value can be up to 256 characters.</p> |
| `throttle` | String |  | <p>The throttling limits of the usage plan.</p> |
| `quota` | String |  | <p>The quota of the usage plan.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `api_stages` | Vec<String> | <p>The associated API stages of a usage plan.</p> |
| `tags` | String | <p>The collection of tags. Each tag element is associated with a given resource.</p> |
| `name` | String | <p>The name of a usage plan.</p> |
| `quota` | String | <p>The target maximum number of permitted requests per a given unit time interval.</p> |
| `id` | String | <p>The identifier of a UsagePlan resource.</p> |
| `throttle` | String | <p>A map containing method level throttling information for API stage in a usage plan.</p> |
| `description` | String | <p>The description of a usage plan.</p> |
| `product_code` | String | <p>The Amazon Web Services Marketplace product identifier to associate with the usage plan as a SaaS product on the Amazon Web Services Marketplace.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create usage_plan
usage_plan = provider.api_gateway.Usage_plan {
    name = "value"  # <p>The name of the usage plan.</p>
}

# Access usage_plan outputs
usage_plan_id = usage_plan.id
usage_plan_api_stages = usage_plan.api_stages
usage_plan_tags = usage_plan.tags
usage_plan_name = usage_plan.name
usage_plan_quota = usage_plan.quota
usage_plan_id = usage_plan.id
usage_plan_throttle = usage_plan.throttle
usage_plan_description = usage_plan.description
usage_plan_product_code = usage_plan.product_code
```

---


### Integration

Integration resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `cache_namespace` | String |  | <p>Specifies a group of related cached parameters. By default, API Gateway uses the resource ID as the <code>cacheNamespace</code>. You can specify the same <code>cacheNamespace</code> across resources to return the same cached data for requests to different resources.</p> |
| `tls_config` | String |  |  |
| `content_handling` | String |  | <p>Specifies how to handle request payload content type conversions. Supported values are <code>CONVERT_TO_BINARY</code> and <code>CONVERT_TO_TEXT</code>, with the following behaviors:</p>
         <p>If this property is not defined, the request payload will be passed through from the method request to integration request without modification, provided that the <code>passthroughBehavior</code> is configured to support payload pass-through.</p> |
| `timeout_in_millis` | i64 |  | <p>Custom timeout between 50 and 29,000 milliseconds. The default value is 29,000 milliseconds or 29 seconds.  You can increase the default value to longer than 29 seconds for Regional or private APIs only.</p> |
| `integration_http_method` | String |  | <p>The HTTP method for the integration.</p> |
| `http_method` | String | ✅ | <p>Specifies the HTTP method for the integration.</p> |
| `uri` | String |  | <p>Specifies Uniform Resource Identifier (URI) of the integration endpoint. For HTTP or
      <code>HTTP_PROXY</code> integrations, the URI must be a fully formed, encoded HTTP(S) URL according to the
      RFC-3986 specification, for either standard integration, where <code>connectionType</code> is not <code>VPC_LINK</code>,
      or private integration, where <code>connectionType</code> is <code>VPC_LINK</code>. For a private HTTP integration, the
      URI is not used for routing. For <code>AWS</code> or <code>AWS_PROXY</code> integrations, the URI is of the form
      <code>arn:aws:apigateway:{region}:{subdomain.service|service}:path|action/{service_api</code>}. Here,
      {Region} is the API Gateway region (e.g., us-east-1); {service} is the name of the integrated
      Amazon Web Services service (e.g., s3); and {subdomain} is a designated subdomain supported by certain Amazon Web Services
      service for fast host-name lookup. action can be used for an Amazon Web Services service action-based API,
      using an Action={name}&{p1}={v1}&p2={v2}... query string. The ensuing {service_api} refers to
      a supported action {name} plus any required input parameters. Alternatively, path can be used
      for an Amazon Web Services service path-based API. The ensuing service_api refers to the path to an Amazon Web Services
      service resource, including the region of the integrated Amazon Web Services service, if applicable. For
      example, for integration with the S3 API of <code>GetObject</code>, the <code>uri</code> can be either
      <code>arn:aws:apigateway:us-west-2:s3:action/GetObject&Bucket={bucket}&Key={key}</code> or
      <code>arn:aws:apigateway:us-west-2:s3:path/{bucket}/{key}</code>.</p> |
| `connection_type` | String |  | <p>The type of the network connection to the integration endpoint. The valid value is <code>INTERNET</code> for connections through the public routable internet or <code>VPC_LINK</code> for private connections between API Gateway and a network load balancer in a VPC. The default value is <code>INTERNET</code>.</p> |
| `cache_key_parameters` | String |  | <p>A list of request parameters whose values API Gateway caches. To be valid values for <code>cacheKeyParameters</code>, these parameters must also be specified for Method <code>requestParameters</code>.</p> |
| `passthrough_behavior` | String |  | <p>Specifies the pass-through behavior for incoming requests based on the Content-Type header in the request, and the available mapping templates specified as the <code>requestTemplates</code> property on the Integration resource. There are three valid values:  <code>WHEN_NO_MATCH</code>, <code>WHEN_NO_TEMPLATES</code>, and <code>NEVER</code>.
        </p> |
| `rest_api_id` | String | ✅ | <p>The string identifier of the associated RestApi.</p> |
| `type` | String | ✅ | <p>Specifies a put integration input's type.</p> |
| `credentials` | String |  | <p>Specifies whether credentials are required for a put integration.</p> |
| `request_parameters` | String |  | <p>A key-value map specifying request parameters that are passed from the method request to the back end. The key is an integration request parameter name and the associated value is a method request parameter value or static value that must be enclosed within single quotes and pre-encoded as required by the back end. The method request parameter value must match the pattern of  <code>method.request.{location}.{name}</code>, where <code>location</code> is <code>querystring</code>, <code>path</code>, or <code>header</code> and <code>name</code> must be a valid and unique method request parameter name.</p> |
| `resource_id` | String | ✅ | <p>Specifies a put integration request's resource ID.</p> |
| `request_templates` | String |  | <p>Represents a map of Velocity templates that are applied on the request payload based on the value of the Content-Type header sent by the client. The content type value is the key in this map, and the template (as a String) is the value.</p> |
| `connection_id` | String |  | <p>The ID of the VpcLink used for the integration. Specify this value only if you specify <code>VPC_LINK</code> as the connection type.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `request_templates` | String | <p>Represents a map of Velocity templates that are applied on the request payload based on the value of the Content-Type header sent by the client. The content type value is the key in this map, and the template (as a String) is the value.</p> |
| `request_parameters` | String | <p>A key-value map specifying request parameters that are passed from the method request to the back end. The key is an integration request parameter name and the associated value is a method request parameter value or static value that must be enclosed within single quotes and pre-encoded as required by the back end. The method request parameter value must match the pattern of  <code>method.request.{location}.{name}</code>, where <code>location</code> is <code>querystring</code>, <code>path</code>, or <code>header</code> and <code>name</code> must be a valid and unique method request parameter name.</p> |
| `cache_namespace` | String | <p>Specifies a group of related cached parameters. By default, API Gateway uses the resource ID as the <code>cacheNamespace</code>. You can specify the same <code>cacheNamespace</code> across resources to return the same cached data for requests to different resources.</p> |
| `passthrough_behavior` | String | <p>Specifies how the method request body of an unmapped content type will be passed through
      the integration request to the back end without transformation. A content type is unmapped if
      no mapping template is defined in the integration or the content type does not match any of
      the mapped content types, as specified in <code>requestTemplates</code>. The valid value is one of the
      following: <code>WHEN_NO_MATCH</code>: passes the method request body through the integration request to
      the back end without transformation when the method request content type does not match any
      content type associated with the mapping templates defined in the integration request.
      <code>WHEN_NO_TEMPLATES</code>: passes the method request body through the integration request to the back
      end without transformation when no mapping template is defined in the integration request. If
      a template is defined when this option is selected, the method request of an unmapped
      content-type will be rejected with an HTTP 415 Unsupported Media Type response. <code>NEVER</code>: rejects
      the method request with an HTTP 415 Unsupported Media Type response when either the method
      request content type does not match any content type associated with the mapping templates
      defined in the integration request or no mapping template is defined in the integration
      request.</p> |
| `content_handling` | String | <p>Specifies how to handle request payload content type conversions. Supported values are <code>CONVERT_TO_BINARY</code> and <code>CONVERT_TO_TEXT</code>, with the following behaviors:</p>
         <p>If this property is not defined, the request payload will be passed through from the method request to integration request without modification, provided that the <code>passthroughBehavior</code> is configured to support payload pass-through.</p> |
| `type` | String | <p>Specifies an API method integration type. The valid value is one of the following:</p>
         <p>For the HTTP and HTTP proxy integrations, each integration can specify a protocol (<code>http/https</code>), port and path. Standard 80 and 443 ports are supported as well as custom ports above 1024. An HTTP or HTTP proxy integration with a <code>connectionType</code> of <code>VPC_LINK</code> is referred to as a private integration and uses a VpcLink to connect API Gateway to a network load balancer of a VPC.</p> |
| `timeout_in_millis` | i64 | <p>Custom timeout between 50 and 29,000 milliseconds. The default value is 29,000 milliseconds or 29 seconds. You can increase the default value to longer than 29 seconds for Regional or private APIs only.</p> |
| `connection_id` | String | <p>The ID of the VpcLink used for the integration when <code>connectionType=VPC_LINK</code> and undefined, otherwise.</p> |
| `uri` | String | <p>Specifies Uniform Resource Identifier (URI) of the integration endpoint.</p>
         <p>For <code>HTTP</code> or <code>HTTP_PROXY</code> integrations, the URI must be a fully formed, encoded HTTP(S) URL
	    according to the RFC-3986 specification for standard integrations. If <code>connectionType</code> is <code>VPC_LINK</code> specify the Network Load Balancer DNS name.
      For <code>AWS</code> or <code>AWS_PROXY</code> integrations, the URI is of
      the form <code>arn:aws:apigateway:{region}:{subdomain.service|service}:path|action/{service_api}</code>.
      Here, {Region} is the API Gateway region (e.g., us-east-1); {service} is the name of the
      integrated Amazon Web Services service (e.g., s3); and {subdomain} is a designated subdomain supported by
      certain Amazon Web Services  service for fast host-name lookup. action can be used for an Amazon Web Services  service
      action-based API, using an Action={name}&{p1}={v1}&p2={v2}... query string. The ensuing
      {service_api} refers to a supported action {name} plus any required input parameters.
      Alternatively, path can be used for an Amazon Web Services service path-based API. The ensuing service_api
      refers to the path to an Amazon Web Services  service resource, including the region of the integrated Amazon Web Services 
      service, if applicable. For example, for integration with the S3 API of GetObject, the uri can
      be either <code>arn:aws:apigateway:us-west-2:s3:action/GetObject&Bucket={bucket}&Key={key}</code> or
      <code>arn:aws:apigateway:us-west-2:s3:path/{bucket}/{key}</code>
         </p> |
| `tls_config` | String | <p>Specifies the TLS configuration for an integration.</p> |
| `http_method` | String | <p>Specifies the integration's HTTP method type. For the Type property, if you specify <code>MOCK</code>, this property is optional. For Lambda integrations, you must set the integration method to <code>POST</code>. For all other types, you must specify this property.</p> |
| `connection_type` | String | <p>The type of the network connection to the integration endpoint. The valid value is <code>INTERNET</code> for connections through the public routable internet or <code>VPC_LINK</code> for private connections between API Gateway and a network load balancer in a VPC. The default value is <code>INTERNET</code>.</p> |
| `credentials` | String | <p>Specifies the credentials required for the integration, if any. For AWS integrations, three options are available. To specify an IAM Role for API Gateway to assume, use the role's Amazon Resource Name (ARN). To require that the caller's identity be passed through from the request, specify the string <code>arn:aws:iam::\*:user/\*</code>. To use resource-based permissions on supported Amazon Web Services services, specify null.</p> |
| `cache_key_parameters` | String | <p>A list of request parameters whose values API Gateway caches. To be valid values for <code>cacheKeyParameters</code>, these parameters must also be specified for Method <code>requestParameters</code>.</p> |
| `integration_responses` | HashMap<String, String> | <p>Specifies the integration's responses.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create integration
integration = provider.api_gateway.Integration {
    http_method = "value"  # <p>Specifies the HTTP method for the integration.</p>
    rest_api_id = "value"  # <p>The string identifier of the associated RestApi.</p>
    type = "value"  # <p>Specifies a put integration input's type.</p>
    resource_id = "value"  # <p>Specifies a put integration request's resource ID.</p>
}

# Access integration outputs
integration_id = integration.id
integration_request_templates = integration.request_templates
integration_request_parameters = integration.request_parameters
integration_cache_namespace = integration.cache_namespace
integration_passthrough_behavior = integration.passthrough_behavior
integration_content_handling = integration.content_handling
integration_type = integration.type
integration_timeout_in_millis = integration.timeout_in_millis
integration_connection_id = integration.connection_id
integration_uri = integration.uri
integration_tls_config = integration.tls_config
integration_http_method = integration.http_method
integration_connection_type = integration.connection_type
integration_credentials = integration.credentials
integration_cache_key_parameters = integration.cache_key_parameters
integration_integration_responses = integration.integration_responses
```

---


### Method

Method resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `request_parameters` | HashMap<String, bool> |  | <p>A key-value map defining required or optional method request parameters that can be accepted by API Gateway. A key defines a method request parameter name matching the pattern of  <code>method.request.{location}.{name}</code>, where <code>location</code> is <code>querystring</code>, <code>path</code>, or <code>header</code> and <code>name</code> is a valid and unique parameter name. The value associated with the key is a Boolean flag indicating whether the parameter is required (<code>true</code>) or optional (<code>false</code>).  The method request parameter names defined here are available in Integration to be mapped to integration request parameters or body-mapping templates.</p> |
| `authorizer_id` | String |  | <p>Specifies the identifier of an Authorizer to use on this Method, if the type is CUSTOM or COGNITO_USER_POOLS. The authorizer identifier is generated by API Gateway when you created the authorizer.</p> |
| `operation_name` | String |  | <p>A human-friendly operation identifier for the method. For example, you can assign the <code>operationName</code> of <code>ListPets</code> for the <code>GET /pets</code> method in the <code>PetStore</code> example.</p> |
| `request_validator_id` | String |  | <p>The identifier of a RequestValidator for validating the method request.</p> |
| `resource_id` | String | ✅ | <p>The Resource identifier for the new Method resource.</p> |
| `authorization_type` | String | ✅ | <p>The method's authorization type. Valid values are <code>NONE</code> for open access, <code>AWS_IAM</code> for using AWS IAM permissions, <code>CUSTOM</code> for using a custom authorizer, or <code>COGNITO_USER_POOLS</code> for using a Cognito user pool.</p> |
| `authorization_scopes` | String |  | <p>A list of authorization scopes configured on the method. The scopes are used with a <code>COGNITO_USER_POOLS</code> authorizer to authorize the method invocation. The authorization works by matching the method scopes against the scopes parsed from the access token in the incoming request. The method invocation is authorized if any method scopes matches a claimed scope in the access token. Otherwise, the invocation is not authorized. When the method scope is configured, the client must provide an access token instead of an identity token for authorization purposes.</p> |
| `request_models` | String |  | <p>Specifies the Model resources used for the request's content type. Request models are represented as a key/value map, with a content type as the key and a Model name as the value.</p> |
| `http_method` | String | ✅ | <p>Specifies the method request's HTTP method type.</p> |
| `rest_api_id` | String | ✅ | <p>The string identifier of the associated RestApi.</p> |
| `api_key_required` | bool |  | <p>Specifies whether the method required a valid ApiKey.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `authorizer_id` | String | <p>The identifier of an Authorizer to use on this method. The <code>authorizationType</code> must be <code>CUSTOM</code>.</p> |
| `method_integration` | String | <p>Gets the method's integration responsible for passing the client-submitted request to the back end and performing necessary transformations to make the request compliant with the back end.</p> |
| `request_validator_id` | String | <p>The identifier of a RequestValidator for request validation.</p> |
| `api_key_required` | bool | <p>A boolean flag specifying whether a valid ApiKey is required to invoke this method.</p> |
| `operation_name` | String | <p>A human-friendly operation identifier for the method. For example, you can assign the <code>operationName</code> of <code>ListPets</code> for the <code>GET /pets</code> method in the <code>PetStore</code> example.</p> |
| `request_models` | String | <p>A key-value map specifying data schemas, represented by Model resources, (as the mapped value) of the request payloads of given content types (as the mapping key).</p> |
| `http_method` | String | <p>The method's HTTP verb.</p> |
| `method_responses` | HashMap<String, String> | <p>Gets a method response associated with a given HTTP status code. </p> |
| `authorization_type` | String | <p>The method's authorization type. Valid values are <code>NONE</code> for open access, <code>AWS_IAM</code> for using AWS IAM permissions, <code>CUSTOM</code> for using a custom authorizer, or <code>COGNITO_USER_POOLS</code> for using a Cognito user pool.</p> |
| `authorization_scopes` | String | <p>A list of authorization scopes configured on the method. The scopes are used with a <code>COGNITO_USER_POOLS</code> authorizer to authorize the method invocation. The authorization works by matching the method scopes against the scopes parsed from the access token in the incoming request. The method invocation is authorized if any method scopes matches a claimed scope in the access token. Otherwise, the invocation is not authorized. When the method scope is configured, the client must provide an access token instead of an identity token for authorization purposes.</p> |
| `request_parameters` | HashMap<String, bool> | <p>A key-value map defining required or optional method request parameters that can be accepted by API Gateway. A key is a method request parameter name matching the pattern of  <code>method.request.{location}.{name}</code>, where <code>location</code> is <code>querystring</code>, <code>path</code>, or <code>header</code> and <code>name</code> is a valid and unique parameter name. The value associated with the key is a Boolean flag indicating whether the parameter is required (<code>true</code>) or optional (<code>false</code>).  The method request parameter names defined here are available in Integration to be mapped to integration request parameters or templates.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create method
method = provider.api_gateway.Method {
    resource_id = "value"  # <p>The Resource identifier for the new Method resource.</p>
    authorization_type = "value"  # <p>The method's authorization type. Valid values are <code>NONE</code> for open access, <code>AWS_IAM</code> for using AWS IAM permissions, <code>CUSTOM</code> for using a custom authorizer, or <code>COGNITO_USER_POOLS</code> for using a Cognito user pool.</p>
    http_method = "value"  # <p>Specifies the method request's HTTP method type.</p>
    rest_api_id = "value"  # <p>The string identifier of the associated RestApi.</p>
}

# Access method outputs
method_id = method.id
method_authorizer_id = method.authorizer_id
method_method_integration = method.method_integration
method_request_validator_id = method.request_validator_id
method_api_key_required = method.api_key_required
method_operation_name = method.operation_name
method_request_models = method.request_models
method_http_method = method.http_method
method_method_responses = method.method_responses
method_authorization_type = method.authorization_type
method_authorization_scopes = method.authorization_scopes
method_request_parameters = method.request_parameters
```

---


### Base_path_mappings

BasePathMappings resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `items` | Vec<String> | <p>The current page of elements from this collection.</p> |
| `position` | String | <p>The current pagination position in the paged result set.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access base_path_mappings outputs
base_path_mappings_id = base_path_mappings.id
base_path_mappings_items = base_path_mappings.items
base_path_mappings_position = base_path_mappings.position
```

---


### Rest_api

RestApi resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `parameters` | String |  | <p>Custom header parameters as part of the request. For example, to exclude DocumentationParts from an imported API, set <code>ignore=documentation</code> as a <code>parameters</code> value, as in the AWS CLI command of <code>aws apigateway import-rest-api --parameters ignore=documentation --body 'file:///path/to/imported-api-body.json'</code>.</p> |
| `fail_on_warnings` | bool |  | <p>A query parameter to indicate whether to rollback the API update (<code>true</code>) or not (<code>false</code>)
            when a warning is encountered. The default value is <code>false</code>.</p> |
| `rest_api_id` | String | ✅ | <p>The string identifier of the associated RestApi.</p> |
| `mode` | String |  | <p>The <code>mode</code> query parameter to specify the update mode. Valid values are "merge" and "overwrite". By default,
        the update mode is "merge".</p> |
| `body` | String | ✅ | <p>The PUT request body containing external API definitions. Currently, only OpenAPI definition JSON/YAML files are supported. The maximum size of the API definition file is 6MB.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | <p>The API's description.</p> |
| `binary_media_types` | String | <p>The list of binary media types supported by the RestApi. By default, the RestApi supports only UTF-8-encoded text payloads.</p> |
| `warnings` | String | <p>The warning messages reported when <code>failonwarnings</code> is turned on during API import.</p> |
| `id` | String | <p>The API's identifier. This identifier is unique across all of your APIs in API Gateway.</p> |
| `version` | String | <p>A version identifier for the API.</p> |
| `api_key_source` | String | <p>The source of the API key for metering requests according to a usage plan. Valid values
      are: ><code>HEADER</code> to read the API key from the <code>X-API-Key</code> header of a
      request. <code>AUTHORIZER</code> to read the API key from the <code>UsageIdentifierKey</code>
      from a custom authorizer.</p> |
| `minimum_compression_size` | i64 | <p>A nullable integer that is used to enable compression (with non-negative between 0 and 10485760 (10M) bytes, inclusive) or disable compression (with a null value) on an API. When compression is enabled, compression or decompression is not applied on the payload if the payload size is smaller than this value. Setting it to zero allows compression for any payload size.</p> |
| `endpoint_configuration` | String | <p>The endpoint configuration of this RestApi showing the endpoint types and IP address types of the API. </p> |
| `policy` | String | <p>A stringified JSON policy document that applies to this RestApi regardless of the caller and Method configuration.</p> |
| `root_resource_id` | String | <p>The API's root resource ID.</p> |
| `disable_execute_api_endpoint` | bool | <p>Specifies whether clients can invoke your API by using the default <code>execute-api</code> endpoint.
      By default, clients can invoke your API with the default
      <code>https://{api_id}.execute-api.{region}.amazonaws.com</code> endpoint. To require that clients use a
      custom domain name to invoke your API, disable the default endpoint.</p> |
| `name` | String | <p>The API's name.</p> |
| `created_date` | String | <p>The timestamp when the API was created.</p> |
| `tags` | String | <p>The collection of tags. Each tag element is associated with a given resource.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create rest_api
rest_api = provider.api_gateway.Rest_api {
    rest_api_id = "value"  # <p>The string identifier of the associated RestApi.</p>
    body = "value"  # <p>The PUT request body containing external API definitions. Currently, only OpenAPI definition JSON/YAML files are supported. The maximum size of the API definition file is 6MB.</p>
}

# Access rest_api outputs
rest_api_id = rest_api.id
rest_api_description = rest_api.description
rest_api_binary_media_types = rest_api.binary_media_types
rest_api_warnings = rest_api.warnings
rest_api_id = rest_api.id
rest_api_version = rest_api.version
rest_api_api_key_source = rest_api.api_key_source
rest_api_minimum_compression_size = rest_api.minimum_compression_size
rest_api_endpoint_configuration = rest_api.endpoint_configuration
rest_api_policy = rest_api.policy
rest_api_root_resource_id = rest_api.root_resource_id
rest_api_disable_execute_api_endpoint = rest_api.disable_execute_api_endpoint
rest_api_name = rest_api.name
rest_api_created_date = rest_api.created_date
rest_api_tags = rest_api.tags
```

---


### Vpc_link

VpcLink resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | String |  | <p>The key-value map of strings. The valid character set is [a-zA-Z+-=._:/]. The tag key can be up to 128 characters and must not start with <code>aws:</code>. The tag value can be up to 256 characters.</p> |
| `target_arns` | String | ✅ | <p>The ARN of the network load balancer of the VPC targeted by the VPC link. The network load balancer must be owned by the same Amazon Web Services account of the API owner.</p> |
| `name` | String | ✅ | <p>The name used to label and identify the VPC link.</p> |
| `description` | String |  | <p>The description of the VPC link.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `target_arns` | String | <p>The ARN of the network load balancer of the VPC targeted by the VPC link. The network load balancer must be owned by the same Amazon Web Services account of the API owner.</p> |
| `tags` | String | <p>The collection of tags. Each tag element is associated with a given resource.</p> |
| `id` | String | <p>The identifier of the  VpcLink. It is used in an Integration to reference this VpcLink.</p> |
| `name` | String | <p>The name used to label and identify the VPC link.</p> |
| `description` | String | <p>The description of the VPC link.</p> |
| `status` | String | <p>The status of the VPC link. The valid values are <code>AVAILABLE</code>, <code>PENDING</code>, <code>DELETING</code>, or <code>FAILED</code>. Deploying an API will wait if the status is <code>PENDING</code> and will fail if the status is <code>DELETING</code>.  </p> |
| `status_message` | String | <p>A description about the VPC link status.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create vpc_link
vpc_link = provider.api_gateway.Vpc_link {
    target_arns = "value"  # <p>The ARN of the network load balancer of the VPC targeted by the VPC link. The network load balancer must be owned by the same Amazon Web Services account of the API owner.</p>
    name = "value"  # <p>The name used to label and identify the VPC link.</p>
}

# Access vpc_link outputs
vpc_link_id = vpc_link.id
vpc_link_target_arns = vpc_link.target_arns
vpc_link_tags = vpc_link.tags
vpc_link_id = vpc_link.id
vpc_link_name = vpc_link.name
vpc_link_description = vpc_link.description
vpc_link_status = vpc_link.status
vpc_link_status_message = vpc_link.status_message
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
| `value` | String | <p>The Apache Velocity Template Language (VTL) template content used for the template resource.</p> |


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


### Models

Models resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `items` | Vec<String> | <p>The current page of elements from this collection.</p> |
| `position` | String | <p>The current pagination position in the paged result set.</p> |


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
models_items = models.items
models_position = models.position
```

---


### Vpc_links

VpcLinks resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `items` | Vec<String> | <p>The current page of elements from this collection.</p> |
| `position` | String | <p>The current pagination position in the paged result set.</p> |


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
vpc_links_items = vpc_links.items
vpc_links_position = vpc_links.position
```

---


### Resources

Resources resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `items` | Vec<String> | <p>The current page of elements from this collection.</p> |
| `position` | String | <p>The current pagination position in the paged result set.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access resources outputs
resources_id = resources.id
resources_items = resources.items
resources_position = resources.position
```

---


### Api_key

ApiKey resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `generate_distinct_id` | bool |  | <p>Specifies whether (<code>true</code>) or not (<code>false</code>) the key identifier is distinct from the created API key value. This parameter is deprecated and should not be used.</p> |
| `value` | String |  | <p>Specifies a value of the API key.</p> |
| `stage_keys` | Vec<String> |  | <p>DEPRECATED FOR USAGE PLANS - Specifies stages associated with the API key.</p> |
| `name` | String |  | <p>The name of the ApiKey.</p> |
| `enabled` | bool |  | <p>Specifies whether the ApiKey can be used by callers.</p> |
| `description` | String |  | <p>The description of the ApiKey.</p> |
| `customer_id` | String |  | <p>An Amazon Web Services Marketplace customer identifier, when integrating with the Amazon Web Services SaaS Marketplace.</p> |
| `tags` | String |  | <p>The key-value map of strings. The valid character set is [a-zA-Z+-=._:/]. The tag key can be up to 128 characters and must not start with <code>aws:</code>. The tag value can be up to 256 characters.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | <p>The name of the API Key.</p> |
| `created_date` | String | <p>The timestamp when the API Key was created.</p> |
| `last_updated_date` | String | <p>The timestamp when the API Key was last updated.</p> |
| `tags` | String | <p>The collection of tags. Each tag element is associated with a given resource.</p> |
| `value` | String | <p>The value of the API Key.</p> |
| `enabled` | bool | <p>Specifies whether the API Key can be used by callers.</p> |
| `stage_keys` | String | <p>A list of Stage resources that are associated with the ApiKey resource.</p> |
| `customer_id` | String | <p>An Amazon Web Services Marketplace customer identifier, when integrating with the Amazon Web Services SaaS Marketplace.</p> |
| `id` | String | <p>The identifier of the API Key.</p> |
| `description` | String | <p>The description of the API Key.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create api_key
api_key = provider.api_gateway.Api_key {
}

# Access api_key outputs
api_key_id = api_key.id
api_key_name = api_key.name
api_key_created_date = api_key.created_date
api_key_last_updated_date = api_key.last_updated_date
api_key_tags = api_key.tags
api_key_value = api_key.value
api_key_enabled = api_key.enabled
api_key_stage_keys = api_key.stage_keys
api_key_customer_id = api_key.customer_id
api_key_id = api_key.id
api_key_description = api_key.description
```

---


### Stage

Stage resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `deployment_id` | String | ✅ | <p>The identifier of the Deployment resource for the Stage resource.</p> |
| `cache_cluster_size` | String |  | <p>The stage's cache capacity in GB. For more information about choosing a cache size, see <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-caching.html">Enabling API caching to enhance responsiveness</a>.</p> |
| `variables` | String |  | <p>A map that defines the stage variables for the new Stage resource. Variable names
          can have alphanumeric and underscore characters, and the values must match
          <code>[A-Za-z0-9-._~:/?#&=,]+</code>.</p> |
| `stage_name` | String | ✅ | <p>The name for the Stage resource. Stage names can only contain alphanumeric characters, hyphens, and underscores. Maximum length is 128 characters.</p> |
| `description` | String |  | <p>The description of the Stage resource.</p> |
| `documentation_version` | String |  | <p>The version of the associated API documentation.</p> |
| `canary_settings` | String |  | <p>The canary deployment settings of this stage.</p> |
| `tracing_enabled` | bool |  | <p>Specifies whether active tracing with X-ray is enabled for the Stage.</p> |
| `tags` | String |  | <p>The key-value map of strings. The valid character set is [a-zA-Z+-=._:/]. The tag key can be up to 128 characters and must not start with <code>aws:</code>. The tag value can be up to 256 characters.</p> |
| `cache_cluster_enabled` | bool |  | <p>Whether cache clustering is enabled for the stage.</p> |
| `rest_api_id` | String | ✅ | <p>The string identifier of the associated RestApi.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `client_certificate_id` | String | <p>The identifier of a client certificate for an API stage.</p> |
| `deployment_id` | String | <p>The identifier of the Deployment that the stage points to.</p> |
| `cache_cluster_status` | String | <p>The status of the cache cluster for the stage, if enabled.</p> |
| `method_settings` | HashMap<String, String> | <p>A map that defines the method settings for a Stage resource. Keys (designated as <code>/{method_setting_key</code> below) are method paths defined as <code>{resource_path}/{http_method}</code> for an individual method override, or <code>/\*/\*</code> for overriding all methods in the stage.  </p> |
| `canary_settings` | String | <p>Settings for the canary deployment in this stage.</p> |
| `tracing_enabled` | bool | <p>Specifies whether active tracing with X-ray is enabled for the Stage.</p> |
| `description` | String | <p>The stage's description.</p> |
| `last_updated_date` | String | <p>The timestamp when the stage last updated.</p> |
| `cache_cluster_size` | String | <p>The stage's cache capacity in GB. For more information about choosing a cache size, see <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-caching.html">Enabling API caching to enhance responsiveness</a>.</p> |
| `cache_cluster_enabled` | bool | <p>Specifies whether a cache cluster is enabled for the stage. To activate a method-level cache, set <code>CachingEnabled</code> to <code>true</code> for a method. </p> |
| `access_log_settings` | String | <p>Settings for logging access in this stage.</p> |
| `stage_name` | String | <p>The name of the stage is the first path segment in the Uniform Resource Identifier (URI) of a call to API Gateway. Stage names can only contain alphanumeric characters, hyphens, and underscores. Maximum length is 128 characters.</p> |
| `variables` | String | <p>A map that defines the stage variables for a Stage resource. Variable names can
          have alphanumeric and underscore characters, and the values must match <code>[A-Za-z0-9-._~:/?#&=,]+</code>.</p> |
| `web_acl_arn` | String | <p>The ARN of the WebAcl associated with the Stage.</p> |
| `tags` | String | <p>The collection of tags. Each tag element is associated with a given resource.</p> |
| `created_date` | String | <p>The timestamp when the stage was created.</p> |
| `documentation_version` | String | <p>The version of the associated API documentation.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create stage
stage = provider.api_gateway.Stage {
    deployment_id = "value"  # <p>The identifier of the Deployment resource for the Stage resource.</p>
    stage_name = "value"  # <p>The name for the Stage resource. Stage names can only contain alphanumeric characters, hyphens, and underscores. Maximum length is 128 characters.</p>
    rest_api_id = "value"  # <p>The string identifier of the associated RestApi.</p>
}

# Access stage outputs
stage_id = stage.id
stage_client_certificate_id = stage.client_certificate_id
stage_deployment_id = stage.deployment_id
stage_cache_cluster_status = stage.cache_cluster_status
stage_method_settings = stage.method_settings
stage_canary_settings = stage.canary_settings
stage_tracing_enabled = stage.tracing_enabled
stage_description = stage.description
stage_last_updated_date = stage.last_updated_date
stage_cache_cluster_size = stage.cache_cluster_size
stage_cache_cluster_enabled = stage.cache_cluster_enabled
stage_access_log_settings = stage.access_log_settings
stage_stage_name = stage.stage_name
stage_variables = stage.variables
stage_web_acl_arn = stage.web_acl_arn
stage_tags = stage.tags
stage_created_date = stage.created_date
stage_documentation_version = stage.documentation_version
```

---


### Account

Account resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `patch_operations` | Vec<String> |  | <p>For more information about supported patch operations, see <a href="https://docs.aws.amazon.com/apigateway/latest/api/patch-operations.html">Patch Operations</a>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `throttle_settings` | String | <p>Specifies the API request limits configured for the current Account.</p> |
| `api_key_version` | String | <p>The version of the API keys used for the account.</p> |
| `features` | String | <p>A list of features supported for the account. When usage plans are enabled, the features list will include an entry of <code>"UsagePlans"</code>.</p> |
| `cloudwatch_role_arn` | String | <p>The ARN of an Amazon CloudWatch role for the current Account. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access account outputs
account_id = account.id
account_throttle_settings = account.throttle_settings
account_api_key_version = account.api_key_version
account_features = account.features
account_cloudwatch_role_arn = account.cloudwatch_role_arn
```

---


### Base_path_mapping

BasePathMapping resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `base_path` | String |  | <p>The base path name that callers of the API must provide as part of the URL after the domain name. This value must be unique for all of the mappings across a single API. Specify '(none)' if you do not want callers to specify a base path name after the domain name.</p> |
| `domain_name` | String | ✅ | <p>The domain name of the BasePathMapping resource to create.</p> |
| `domain_name_id` | String |  | <p>The identifier for the domain name resource. Required for private custom domain names.</p> |
| `rest_api_id` | String | ✅ | <p>The string identifier of the associated RestApi.</p> |
| `stage` | String |  | <p>The name of the API's stage that you want to use for this mapping. Specify '(none)' if you want callers to explicitly specify the stage name after any base path name.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `rest_api_id` | String | <p>The string identifier of the associated RestApi.</p> |
| `base_path` | String | <p>The base path name that callers of the API must provide as part of the URL after the domain name.</p> |
| `stage` | String | <p>The name of the associated stage.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create base_path_mapping
base_path_mapping = provider.api_gateway.Base_path_mapping {
    domain_name = "value"  # <p>The domain name of the BasePathMapping resource to create.</p>
    rest_api_id = "value"  # <p>The string identifier of the associated RestApi.</p>
}

# Access base_path_mapping outputs
base_path_mapping_id = base_path_mapping.id
base_path_mapping_rest_api_id = base_path_mapping.rest_api_id
base_path_mapping_base_path = base_path_mapping.base_path
base_path_mapping_stage = base_path_mapping.stage
```

---


### Domain_name_access_associations

DomainNameAccessAssociations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `items` | Vec<String> | <p>
The current page of elements from this collection.
</p> |
| `position` | String | <p>The current pagination position in the paged result set.
</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access domain_name_access_associations outputs
domain_name_access_associations_id = domain_name_access_associations.id
domain_name_access_associations_items = domain_name_access_associations.items
domain_name_access_associations_position = domain_name_access_associations.position
```

---


### Documentation_version

DocumentationVersion resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `documentation_version` | String | ✅ | <p>The version identifier of the new snapshot.</p> |
| `stage_name` | String |  | <p>The stage name to be associated with the new documentation snapshot.</p> |
| `rest_api_id` | String | ✅ | <p>The string identifier of the associated RestApi.</p> |
| `description` | String |  | <p>A description about the new documentation snapshot.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `version` | String | <p>The version identifier of the API documentation snapshot.</p> |
| `created_date` | String | <p>The date when the API documentation snapshot is created.</p> |
| `description` | String | <p>The description of the API documentation snapshot.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create documentation_version
documentation_version = provider.api_gateway.Documentation_version {
    documentation_version = "value"  # <p>The version identifier of the new snapshot.</p>
    rest_api_id = "value"  # <p>The string identifier of the associated RestApi.</p>
}

# Access documentation_version outputs
documentation_version_id = documentation_version.id
documentation_version_version = documentation_version.version
documentation_version_created_date = documentation_version.created_date
documentation_version_description = documentation_version.description
```

---


### Usage_plan_keys

UsagePlanKeys resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `position` | String | <p>The current pagination position in the paged result set.</p> |
| `items` | Vec<String> | <p>The current page of elements from this collection.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access usage_plan_keys outputs
usage_plan_keys_id = usage_plan_keys.id
usage_plan_keys_position = usage_plan_keys.position
usage_plan_keys_items = usage_plan_keys.items
```

---


### Resource

Resource resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `parent_id` | String | ✅ | <p>The parent resource's identifier.</p> |
| `path_part` | String | ✅ | <p>The last path segment for this resource.</p> |
| `rest_api_id` | String | ✅ | <p>The string identifier of the associated RestApi.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `resource_methods` | HashMap<String, String> | <p>Gets an API resource's method of a given HTTP verb.</p> |
| `id` | String | <p>The resource's identifier.</p> |
| `parent_id` | String | <p>The parent resource's identifier.</p> |
| `path` | String | <p>The full path for this resource.</p> |
| `path_part` | String | <p>The last path segment for this resource.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create resource
resource = provider.api_gateway.Resource {
    parent_id = "value"  # <p>The parent resource's identifier.</p>
    path_part = "value"  # <p>The last path segment for this resource.</p>
    rest_api_id = "value"  # <p>The string identifier of the associated RestApi.</p>
}

# Access resource outputs
resource_id = resource.id
resource_resource_methods = resource.resource_methods
resource_id = resource.id
resource_parent_id = resource.parent_id
resource_path = resource.path
resource_path_part = resource.path_part
```

---


### Usage

Usage resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `key_id` | String | ✅ | <p>The identifier of the API key associated with the usage plan in which a temporary extension is granted to the remaining quota.</p> |
| `patch_operations` | Vec<String> |  | <p>For more information about supported patch operations, see <a href="https://docs.aws.amazon.com/apigateway/latest/api/patch-operations.html">Patch Operations</a>.</p> |
| `usage_plan_id` | String | ✅ | <p>The Id of the usage plan associated with the usage data.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `items` | HashMap<String, Vec<Vec<i64>>> | <p>The usage data, as daily logs of used and remaining quotas, over the specified time interval indexed over the API keys in a usage plan. For example, <code>{..., "values" : { "{api_key}" : [ [0, 100], [10, 90], [100, 10]]}</code>, where <code>{api_key}</code> stands for an API key value and the daily log entry is of the format <code>[used quota, remaining quota]</code>.</p> |
| `position` | String | <p>The current pagination position in the paged result set.</p> |
| `usage_plan_id` | String | <p>The plan Id associated with this usage data.</p> |
| `start_date` | String | <p>The starting date of the usage data.</p> |
| `end_date` | String | <p>The ending date of the usage data.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access usage outputs
usage_id = usage.id
usage_items = usage.items
usage_position = usage.position
usage_usage_plan_id = usage.usage_plan_id
usage_start_date = usage.start_date
usage_end_date = usage.end_date
```

---


### Documentation_versions

DocumentationVersions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `position` | String | <p>The current pagination position in the paged result set.</p> |
| `items` | Vec<String> | <p>The current page of elements from this collection.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access documentation_versions outputs
documentation_versions_id = documentation_versions.id
documentation_versions_position = documentation_versions.position
documentation_versions_items = documentation_versions.items
```

---


### Client_certificates

ClientCertificates resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `items` | Vec<String> | <p>The current page of elements from this collection.</p> |
| `position` | String | <p>The current pagination position in the paged result set.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access client_certificates outputs
client_certificates_id = client_certificates.id
client_certificates_items = client_certificates.items
client_certificates_position = client_certificates.position
```

---


### Sdk

Sdk resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `content_type` | String | <p>The content-type header value in the HTTP response.</p> |
| `content_disposition` | String | <p>The content-disposition header value in the HTTP response.</p> |
| `body` | String | <p>The binary blob response to GetSdk, which contains the generated SDK.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access sdk outputs
sdk_id = sdk.id
sdk_content_type = sdk.content_type
sdk_content_disposition = sdk.content_disposition
sdk_body = sdk.body
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple integration_response resources
integration_response_0 = provider.api_gateway.Integration_response {
    rest_api_id = "value-0"
    resource_id = "value-0"
    http_method = "value-0"
    status_code = "value-0"
}
integration_response_1 = provider.api_gateway.Integration_response {
    rest_api_id = "value-1"
    resource_id = "value-1"
    http_method = "value-1"
    status_code = "value-1"
}
integration_response_2 = provider.api_gateway.Integration_response {
    rest_api_id = "value-2"
    resource_id = "value-2"
    http_method = "value-2"
    status_code = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    integration_response = provider.api_gateway.Integration_response {
        rest_api_id = "production-value"
        resource_id = "production-value"
        http_method = "production-value"
        status_code = "production-value"
    }
```

---

## Related Documentation

- [AWS Api_gateway Documentation](https://docs.aws.amazon.com/api_gateway/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
