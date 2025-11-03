# Elasticsearch_service Service



**Resources**: 21

---

## Overview

The elasticsearch_service service provides access to 21 resource types:

- [Reserved_elasticsearch_instance_offerings](#reserved_elasticsearch_instance_offerings) [R]
- [Domain_auto_tunes](#domain_auto_tunes) [R]
- [Upgrade_status](#upgrade_status) [R]
- [Outbound_cross_cluster_search_connection](#outbound_cross_cluster_search_connection) [CD]
- [Elasticsearch_domain](#elasticsearch_domain) [CRD]
- [Domain_change_progress](#domain_change_progress) [R]
- [Package_version_history](#package_version_history) [R]
- [Reserved_elasticsearch_instances](#reserved_elasticsearch_instances) [R]
- [Elasticsearch_domains](#elasticsearch_domains) [R]
- [Elasticsearch_domain_config](#elasticsearch_domain_config) [RU]
- [Inbound_cross_cluster_search_connection](#inbound_cross_cluster_search_connection) [D]
- [Compatible_elasticsearch_versions](#compatible_elasticsearch_versions) [R]
- [Package](#package) [CUD]
- [Vpc_endpoint](#vpc_endpoint) [CUD]
- [Vpc_endpoints](#vpc_endpoints) [R]
- [Elasticsearch_instance_type_limits](#elasticsearch_instance_type_limits) [R]
- [Elasticsearch_service_role](#elasticsearch_service_role) [D]
- [Outbound_cross_cluster_search_connections](#outbound_cross_cluster_search_connections) [R]
- [Packages](#packages) [R]
- [Inbound_cross_cluster_search_connections](#inbound_cross_cluster_search_connections) [R]
- [Upgrade_history](#upgrade_history) [R]

---

## Resources


### Reserved_elasticsearch_instance_offerings

ReservedElasticsearchInstanceOfferings resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>Provides an identifier to allow retrieval of paginated results.</p> |
| `reserved_elasticsearch_instance_offerings` | Vec<String> | <p>List of reserved Elasticsearch instance offerings</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access reserved_elasticsearch_instance_offerings outputs
reserved_elasticsearch_instance_offerings_id = reserved_elasticsearch_instance_offerings.id
reserved_elasticsearch_instance_offerings_next_token = reserved_elasticsearch_instance_offerings.next_token
reserved_elasticsearch_instance_offerings_reserved_elasticsearch_instance_offerings = reserved_elasticsearch_instance_offerings.reserved_elasticsearch_instance_offerings
```

---


### Domain_auto_tunes

DomainAutoTunes resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>Specifies an identifier to allow retrieval of paginated results.</p> |
| `auto_tunes` | Vec<String> | <p>Specifies the list of setting adjustments that Auto-Tune has made to the domain. See the <a href="https://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/auto-tune.html" target="_blank">Developer Guide</a> for more information.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access domain_auto_tunes outputs
domain_auto_tunes_id = domain_auto_tunes.id
domain_auto_tunes_next_token = domain_auto_tunes.next_token
domain_auto_tunes_auto_tunes = domain_auto_tunes.auto_tunes
```

---


### Upgrade_status

UpgradeStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `upgrade_name` | String | <p>A string that describes the update briefly</p> |
| `upgrade_step` | String | <p>
      Represents one of 3 steps that an Upgrade or Upgrade Eligibility Check does through:
      <ul>
        <li>PreUpgradeCheck</li>
        <li>Snapshot</li>
        <li>Upgrade</li>
      </ul>
    </p> |
| `step_status` | String | <p>
      One of 4 statuses that a step can go through returned as part of the
      <code>
        <a>GetUpgradeStatusResponse</a>
      </code>
      object. The status can take one of the following values:
      <ul>
        <li>In Progress</li>
        <li>Succeeded</li>
        <li>Succeeded with Issues</li>
        <li>Failed</li>
      </ul>
    </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access upgrade_status outputs
upgrade_status_id = upgrade_status.id
upgrade_status_upgrade_name = upgrade_status.upgrade_name
upgrade_status_upgrade_step = upgrade_status.upgrade_step
upgrade_status_step_status = upgrade_status.step_status
```

---


### Outbound_cross_cluster_search_connection

OutboundCrossClusterSearchConnection resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `source_domain_info` | String | ✅ | <p>Specifies the <code><a>DomainInformation</a></code> for the source Elasticsearch domain.</p> |
| `destination_domain_info` | String | ✅ | <p>Specifies the <code><a>DomainInformation</a></code> for the destination Elasticsearch domain.</p> |
| `connection_alias` | String | ✅ | <p>Specifies the connection alias that will be used by the customer for this connection.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create outbound_cross_cluster_search_connection
outbound_cross_cluster_search_connection = provider.elasticsearch_service.Outbound_cross_cluster_search_connection {
    source_domain_info = "value"  # <p>Specifies the <code><a>DomainInformation</a></code> for the source Elasticsearch domain.</p>
    destination_domain_info = "value"  # <p>Specifies the <code><a>DomainInformation</a></code> for the destination Elasticsearch domain.</p>
    connection_alias = "value"  # <p>Specifies the connection alias that will be used by the customer for this connection.</p>
}

```

---


### Elasticsearch_domain

ElasticsearchDomain resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `encryption_at_rest_options` | String |  | <p>Specifies the Encryption At Rest Options.</p> |
| `snapshot_options` | String |  | <p>Option to set time, in UTC format, of the daily automated snapshot. Default value is 0 hours. </p> |
| `log_publishing_options` | HashMap<String, String> |  | <p>Map of <code>LogType</code> and <code>LogPublishingOption</code>, each containing options to publish a given type of Elasticsearch log.</p> |
| `auto_tune_options` | String |  | <p>Specifies Auto-Tune options.</p> |
| `ebs_options` | String |  | <p>Options to enable, disable and specify the type and size of EBS storage volumes. </p> |
| `vpc_options` | String |  | <p>Options to specify the subnets and security groups for VPC endpoint. For more information, see <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-vpc.html#es-creating-vpc" target="_blank">Creating a VPC</a> in <i>VPC Endpoints for Amazon Elasticsearch Service Domains</i></p> |
| `elasticsearch_version` | String |  | <p>String of format X.Y to specify version for the Elasticsearch domain eg. "1.5" or "2.3". For more information,
          see <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-createupdatedomains.html#es-createdomains" target="_blank">Creating Elasticsearch Domains</a> in the <i>Amazon Elasticsearch Service Developer Guide</i>.</p> |
| `advanced_security_options` | String |  | <p>Specifies advanced security options.</p> |
| `access_policies` | String |  | <p> IAM access policy as a JSON-formatted string.</p> |
| `domain_endpoint_options` | String |  | <p>Options to specify configuration that will be applied to the domain endpoint.</p> |
| `cognito_options` | String |  | <p>Options to specify the Cognito user and identity pools for Kibana authentication. For more information, see <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-cognito-auth.html" target="_blank">Amazon Cognito Authentication for Kibana</a>.</p> |
| `advanced_options` | HashMap<String, String> |  | <p> Option to allow references to indices in an HTTP request body.  Must be <code>false</code> when configuring access to individual sub-resources.  By default, the value is <code>true</code>.
          See <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-createupdatedomains.html#es-createdomain-configure-advanced-options" target="_blank">Configuration Advanced Options</a> for more information.</p> |
| `domain_name` | String | ✅ | <p>The name of the Elasticsearch domain that you are creating. Domain names are unique across the domains owned by an account within an AWS region. Domain names must start with a lowercase letter and can contain the following characters: a-z (lowercase), 0-9, and - (hyphen).</p> |
| `node_to_node_encryption_options` | String |  | <p>Specifies the NodeToNodeEncryptionOptions.</p> |
| `elasticsearch_cluster_config` | String |  | <p>Configuration options for an Elasticsearch domain. Specifies the instance type and number of instances in the domain cluster. </p> |
| `tag_list` | Vec<String> |  | <p>A list of <code>Tag</code> added during domain creation.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `domain_status` | String | <p>The current status of the Elasticsearch domain.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create elasticsearch_domain
elasticsearch_domain = provider.elasticsearch_service.Elasticsearch_domain {
    domain_name = "value"  # <p>The name of the Elasticsearch domain that you are creating. Domain names are unique across the domains owned by an account within an AWS region. Domain names must start with a lowercase letter and can contain the following characters: a-z (lowercase), 0-9, and - (hyphen).</p>
}

# Access elasticsearch_domain outputs
elasticsearch_domain_id = elasticsearch_domain.id
elasticsearch_domain_domain_status = elasticsearch_domain.domain_status
```

---


### Domain_change_progress

DomainChangeProgress resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `change_progress_status` | String | <p>Progress information for the configuration change that is requested in the <code>DescribeDomainChangeProgress</code> request.
      </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access domain_change_progress outputs
domain_change_progress_id = domain_change_progress.id
domain_change_progress_change_progress_status = domain_change_progress.change_progress_status
```

---


### Package_version_history

PackageVersionHistory resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `package_id` | String |  |
| `package_version_history_list` | Vec<String> | <p>List of <code>PackageVersionHistory</code> objects.</p> |
| `next_token` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access package_version_history outputs
package_version_history_id = package_version_history.id
package_version_history_package_id = package_version_history.package_id
package_version_history_package_version_history_list = package_version_history.package_version_history_list
package_version_history_next_token = package_version_history.next_token
```

---


### Reserved_elasticsearch_instances

ReservedElasticsearchInstances resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>Provides an identifier to allow retrieval of paginated results.</p> |
| `reserved_elasticsearch_instances` | Vec<String> | <p>List of reserved Elasticsearch instances.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access reserved_elasticsearch_instances outputs
reserved_elasticsearch_instances_id = reserved_elasticsearch_instances.id
reserved_elasticsearch_instances_next_token = reserved_elasticsearch_instances.next_token
reserved_elasticsearch_instances_reserved_elasticsearch_instances = reserved_elasticsearch_instances.reserved_elasticsearch_instances
```

---


### Elasticsearch_domains

ElasticsearchDomains resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `domain_status_list` | Vec<String> | <p>The status of the domains requested in the <code>DescribeElasticsearchDomains</code> request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access elasticsearch_domains outputs
elasticsearch_domains_id = elasticsearch_domains.id
elasticsearch_domains_domain_status_list = elasticsearch_domains.domain_status_list
```

---


### Elasticsearch_domain_config

ElasticsearchDomainConfig resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `cognito_options` | String |  | <p>Options to specify the Cognito user and identity pools for Kibana authentication. For more information, see <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-cognito-auth.html" target="_blank">Amazon Cognito Authentication for Kibana</a>.</p> |
| `snapshot_options` | String |  | <p>Option to set the time, in UTC format, for the daily automated snapshot. Default value is <code>0</code> hours. </p> |
| `access_policies` | String |  | <p>IAM access policy as a JSON-formatted string.</p> |
| `ebs_options` | String |  | <p>Specify the type and size of the EBS volume that you want to use. </p> |
| `advanced_security_options` | String |  | <p>Specifies advanced security options.</p> |
| `auto_tune_options` | String |  | <p>Specifies Auto-Tune options.</p> |
| `node_to_node_encryption_options` | String |  | <p>Specifies the NodeToNodeEncryptionOptions.</p> |
| `encryption_at_rest_options` | String |  | <p>Specifies the Encryption At Rest Options.</p> |
| `elasticsearch_cluster_config` | String |  | <p>The type and number of instances to instantiate for the domain cluster.</p> |
| `domain_endpoint_options` | String |  | <p>Options to specify configuration that will be applied to the domain endpoint.</p> |
| `advanced_options` | HashMap<String, String> |  | <p>Modifies the advanced option to allow references to indices in an HTTP request body.  Must be <code>false</code> when configuring access to individual sub-resources.  By default, the value is <code>true</code>.
       See <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-createupdatedomains.html#es-createdomain-configure-advanced-options" target="_blank">Configuration Advanced Options</a> for more information.</p> |
| `domain_name` | String | ✅ | <p>The name of the Elasticsearch domain that you are updating. </p> |
| `log_publishing_options` | HashMap<String, String> |  | <p>Map of <code>LogType</code> and <code>LogPublishingOption</code>, each containing options to publish a given type of Elasticsearch log.</p> |
| `dry_run` | bool |  | <p>
           This flag, when set to True, specifies whether the <code>UpdateElasticsearchDomain</code> request should return the results of validation checks without actually applying the change.
           This flag, when set to True, specifies the deployment mechanism through which the update shall be applied on the domain.
           This will not actually perform the Update.
       </p> |
| `vpc_options` | String |  | <p>Options to specify the subnets and security groups for VPC endpoint. For more information, see <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-vpc.html#es-creating-vpc" target="_blank">Creating a VPC</a> in <i>VPC Endpoints for Amazon Elasticsearch Service Domains</i></p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `domain_config` | String | <p>The configuration information of the domain requested in the <code>DescribeElasticsearchDomainConfig</code> request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access elasticsearch_domain_config outputs
elasticsearch_domain_config_id = elasticsearch_domain_config.id
elasticsearch_domain_config_domain_config = elasticsearch_domain_config.domain_config
```

---


### Inbound_cross_cluster_search_connection

InboundCrossClusterSearchConnection resource

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


### Compatible_elasticsearch_versions

CompatibleElasticsearchVersions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `compatible_elasticsearch_versions` | Vec<String> | <p>
      A map of compatible Elasticsearch versions returned as part of the
      <code>
        <a>GetCompatibleElasticsearchVersions</a>
      </code>
      operation.
    </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access compatible_elasticsearch_versions outputs
compatible_elasticsearch_versions_id = compatible_elasticsearch_versions.id
compatible_elasticsearch_versions_compatible_elasticsearch_versions = compatible_elasticsearch_versions.compatible_elasticsearch_versions
```

---


### Package

Package resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `package_name` | String | ✅ | <p>Unique identifier for the package.</p> |
| `package_type` | String | ✅ | <p>Type of package. Currently supports only TXT-DICTIONARY.</p> |
| `package_source` | String | ✅ | <p>The customer S3 location <code>PackageSource</code> for importing the package.</p> |
| `package_description` | String |  | <p>Description of the package.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create package
package = provider.elasticsearch_service.Package {
    package_name = "value"  # <p>Unique identifier for the package.</p>
    package_type = "value"  # <p>Type of package. Currently supports only TXT-DICTIONARY.</p>
    package_source = "value"  # <p>The customer S3 location <code>PackageSource</code> for importing the package.</p>
}

```

---


### Vpc_endpoint

VpcEndpoint resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `domain_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the domain to grant access to.</p> |
| `vpc_options` | String | ✅ | <p>Options to specify the subnets and security groups for the endpoint.</p> |
| `client_token` | String |  | <p>Unique, case-sensitive identifier to ensure idempotency of the request.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create vpc_endpoint
vpc_endpoint = provider.elasticsearch_service.Vpc_endpoint {
    domain_arn = "value"  # <p>The Amazon Resource Name (ARN) of the domain to grant access to.</p>
    vpc_options = "value"  # <p>Options to specify the subnets and security groups for the endpoint.</p>
}

```

---


### Vpc_endpoints

VpcEndpoints resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `vpc_endpoints` | Vec<String> | <p>Information about each requested VPC endpoint.</p> |
| `vpc_endpoint_errors` | Vec<String> | <p>Any errors associated with the request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access vpc_endpoints outputs
vpc_endpoints_id = vpc_endpoints.id
vpc_endpoints_vpc_endpoints = vpc_endpoints.vpc_endpoints
vpc_endpoints_vpc_endpoint_errors = vpc_endpoints.vpc_endpoint_errors
```

---


### Elasticsearch_instance_type_limits

ElasticsearchInstanceTypeLimits resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `limits_by_role` | HashMap<String, String> |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access elasticsearch_instance_type_limits outputs
elasticsearch_instance_type_limits_id = elasticsearch_instance_type_limits.id
elasticsearch_instance_type_limits_limits_by_role = elasticsearch_instance_type_limits.limits_by_role
```

---


### Elasticsearch_service_role

ElasticsearchServiceRole resource

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


### Outbound_cross_cluster_search_connections

OutboundCrossClusterSearchConnections resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>If more results are available and NextToken is present, make the next request to the same API with the received NextToken to paginate the remaining results.
    </p> |
| `cross_cluster_search_connections` | Vec<String> | <p>Consists of list of <code><a>OutboundCrossClusterSearchConnection</a></code> matching the specified filter criteria.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access outbound_cross_cluster_search_connections outputs
outbound_cross_cluster_search_connections_id = outbound_cross_cluster_search_connections.id
outbound_cross_cluster_search_connections_next_token = outbound_cross_cluster_search_connections.next_token
outbound_cross_cluster_search_connections_cross_cluster_search_connections = outbound_cross_cluster_search_connections.cross_cluster_search_connections
```

---


### Packages

Packages resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String |  |
| `package_details_list` | Vec<String> | <p>List of <code>PackageDetails</code> objects.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access packages outputs
packages_id = packages.id
packages_next_token = packages.next_token
packages_package_details_list = packages.package_details_list
```

---


### Inbound_cross_cluster_search_connections

InboundCrossClusterSearchConnections resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `cross_cluster_search_connections` | Vec<String> | <p>Consists of list of <code><a>InboundCrossClusterSearchConnection</a></code> matching the specified filter criteria.</p> |
| `next_token` | String | <p>If more results are available and NextToken is present, make the next request to the same API with the received NextToken to paginate the remaining results.
    </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access inbound_cross_cluster_search_connections outputs
inbound_cross_cluster_search_connections_id = inbound_cross_cluster_search_connections.id
inbound_cross_cluster_search_connections_cross_cluster_search_connections = inbound_cross_cluster_search_connections.cross_cluster_search_connections
inbound_cross_cluster_search_connections_next_token = inbound_cross_cluster_search_connections.next_token
```

---


### Upgrade_history

UpgradeHistory resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `upgrade_histories` | Vec<String> | <p>
      A list of
      <code>
        <a>UpgradeHistory</a>
      </code>
      objects corresponding to each Upgrade or Upgrade Eligibility Check performed on a domain returned as part of
      <code>
        <a>GetUpgradeHistoryResponse</a>
      </code>
      object.
    </p> |
| `next_token` | String | <p>Pagination token that needs to be supplied to the next call to get the next page of results</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access upgrade_history outputs
upgrade_history_id = upgrade_history.id
upgrade_history_upgrade_histories = upgrade_history.upgrade_histories
upgrade_history_next_token = upgrade_history.next_token
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple reserved_elasticsearch_instance_offerings resources
reserved_elasticsearch_instance_offerings_0 = provider.elasticsearch_service.Reserved_elasticsearch_instance_offerings {
}
reserved_elasticsearch_instance_offerings_1 = provider.elasticsearch_service.Reserved_elasticsearch_instance_offerings {
}
reserved_elasticsearch_instance_offerings_2 = provider.elasticsearch_service.Reserved_elasticsearch_instance_offerings {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    reserved_elasticsearch_instance_offerings = provider.elasticsearch_service.Reserved_elasticsearch_instance_offerings {
    }
```

---

## Related Documentation

- [AWS Elasticsearch_service Documentation](https://docs.aws.amazon.com/elasticsearch_service/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
