# Cloudfront Service



**Resources**: 41

---

## Overview

The cloudfront service provides access to 41 resource types:

- [Field_level_encryption](#field_level_encryption) [R]
- [Streaming_distribution_config](#streaming_distribution_config) [R]
- [Origin_access_control](#origin_access_control) [CRUD]
- [Origin_request_policy](#origin_request_policy) [CRUD]
- [Origin_access_control_config](#origin_access_control_config) [R]
- [Cache_policy](#cache_policy) [CRUD]
- [Distribution_tenant](#distribution_tenant) [CRUD]
- [Managed_certificate_details](#managed_certificate_details) [R]
- [Key_value_store](#key_value_store) [CRUD]
- [Key_group_config](#key_group_config) [R]
- [Response_headers_policy](#response_headers_policy) [CRUD]
- [Response_headers_policy_config](#response_headers_policy_config) [R]
- [Realtime_log_config](#realtime_log_config) [CRUD]
- [Anycast_ip_list](#anycast_ip_list) [CRD]
- [Field_level_encryption_profile_config](#field_level_encryption_profile_config) [R]
- [Field_level_encryption_profile](#field_level_encryption_profile) [CRUD]
- [Streaming_distribution](#streaming_distribution) [CRUD]
- [Distribution_config](#distribution_config) [R]
- [Public_key](#public_key) [CRUD]
- [Public_key_config](#public_key_config) [R]
- [Streaming_distribution_with_tags](#streaming_distribution_with_tags) [C]
- [Cloud_front_origin_access_identity](#cloud_front_origin_access_identity) [CRUD]
- [Continuous_deployment_policy_config](#continuous_deployment_policy_config) [R]
- [Continuous_deployment_policy](#continuous_deployment_policy) [CRUD]
- [Invalidation_for_distribution_tenant](#invalidation_for_distribution_tenant) [CR]
- [Distribution](#distribution) [CRUD]
- [Key_group](#key_group) [CRUD]
- [Field_level_encryption_config](#field_level_encryption_config) [CRUD]
- [Origin_request_policy_config](#origin_request_policy_config) [R]
- [Distribution_tenant_by_domain](#distribution_tenant_by_domain) [R]
- [Connection_group_by_routing_endpoint](#connection_group_by_routing_endpoint) [R]
- [Connection_group](#connection_group) [CRUD]
- [Distribution_with_tags](#distribution_with_tags) [C]
- [Function](#function) [CRUD]
- [Cloud_front_origin_access_identity_config](#cloud_front_origin_access_identity_config) [R]
- [Domain_association](#domain_association) [U]
- [Invalidation](#invalidation) [CR]
- [Monitoring_subscription](#monitoring_subscription) [CRD]
- [Distribution_with_staging_config](#distribution_with_staging_config) [U]
- [Cache_policy_config](#cache_policy_config) [R]
- [Vpc_origin](#vpc_origin) [CRUD]

---

## Resources


### Field_level_encryption

FieldLevelEncryption resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `e_tag` | String | <p>The current version of the field level encryption configuration. For example: <code>E2QWRUHAPOMQZL</code>.</p> |
| `field_level_encryption` | String | <p>Return the field-level encryption configuration information.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access field_level_encryption outputs
field_level_encryption_id = field_level_encryption.id
field_level_encryption_e_tag = field_level_encryption.e_tag
field_level_encryption_field_level_encryption = field_level_encryption.field_level_encryption
```

---


### Streaming_distribution_config

StreamingDistributionConfig resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `streaming_distribution_config` | String | <p>The streaming distribution's configuration information.</p> |
| `e_tag` | String | <p>The current version of the configuration. For example: <code>E2QWRUHAPOMQZL</code>. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access streaming_distribution_config outputs
streaming_distribution_config_id = streaming_distribution_config.id
streaming_distribution_config_streaming_distribution_config = streaming_distribution_config.streaming_distribution_config
streaming_distribution_config_e_tag = streaming_distribution_config.e_tag
```

---


### Origin_access_control

OriginAccessControl resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `origin_access_control_config` | String | ✅ | <p>Contains the origin access control.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `e_tag` | String | <p>The version identifier for the current version of the origin access control.</p> |
| `origin_access_control` | String | <p>Contains an origin access control, including its unique identifier.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create origin_access_control
origin_access_control = provider.cloudfront.Origin_access_control {
    origin_access_control_config = "value"  # <p>Contains the origin access control.</p>
}

# Access origin_access_control outputs
origin_access_control_id = origin_access_control.id
origin_access_control_e_tag = origin_access_control.e_tag
origin_access_control_origin_access_control = origin_access_control.origin_access_control
```

---


### Origin_request_policy

OriginRequestPolicy resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `origin_request_policy_config` | String | ✅ | <p>An origin request policy configuration.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `origin_request_policy` | String | <p>The origin request policy.</p> |
| `e_tag` | String | <p>The current version of the origin request policy.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create origin_request_policy
origin_request_policy = provider.cloudfront.Origin_request_policy {
    origin_request_policy_config = "value"  # <p>An origin request policy configuration.</p>
}

# Access origin_request_policy outputs
origin_request_policy_id = origin_request_policy.id
origin_request_policy_origin_request_policy = origin_request_policy.origin_request_policy
origin_request_policy_e_tag = origin_request_policy.e_tag
```

---


### Origin_access_control_config

OriginAccessControlConfig resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `e_tag` | String | <p>The version identifier for the current version of the origin access control.</p> |
| `origin_access_control_config` | String | <p>Contains an origin access control configuration.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access origin_access_control_config outputs
origin_access_control_config_id = origin_access_control_config.id
origin_access_control_config_e_tag = origin_access_control_config.e_tag
origin_access_control_config_origin_access_control_config = origin_access_control_config.origin_access_control_config
```

---


### Cache_policy

CachePolicy resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `cache_policy_config` | String | ✅ | <p>A cache policy configuration.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `cache_policy` | String | <p>The cache policy.</p> |
| `e_tag` | String | <p>The current version of the cache policy.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create cache_policy
cache_policy = provider.cloudfront.Cache_policy {
    cache_policy_config = "value"  # <p>A cache policy configuration.</p>
}

# Access cache_policy outputs
cache_policy_id = cache_policy.id
cache_policy_cache_policy = cache_policy.cache_policy
cache_policy_e_tag = cache_policy.e_tag
```

---


### Distribution_tenant

DistributionTenant resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `connection_group_id` | String |  | <p>The ID of the connection group to associate with the distribution tenant.</p> |
| `tags` | String |  |  |
| `parameters` | Vec<String> |  | <p>A list of parameter values to add to the resource. A parameter is specified as a key-value pair. A valid parameter value must exist for any parameter that is marked as required in the multi-tenant distribution.</p> |
| `distribution_id` | String | ✅ | <p>The ID of the multi-tenant distribution to use for creating the distribution tenant.</p> |
| `domains` | Vec<String> | ✅ | <p>The domains associated with the distribution tenant. You must specify at least one domain in the request.</p> |
| `managed_certificate_request` | String |  | <p>The configuration for the CloudFront managed ACM certificate request.</p> |
| `enabled` | bool |  | <p>Indicates whether the distribution tenant should be enabled when created. If the distribution tenant is disabled, the distribution tenant won't serve traffic.</p> |
| `name` | String | ✅ | <p>The name of the distribution tenant. Enter a friendly identifier that is unique within your Amazon Web Services account. This name can't be updated after you create the distribution tenant.</p> |
| `customizations` | String |  | <p>Customizations for the distribution tenant. For each distribution tenant, you can specify the geographic restrictions, and the Amazon Resource Names (ARNs) for the ACM certificate and WAF web ACL. These are specific values that you can override or disable from the multi-tenant distribution that was used to create the distribution tenant.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `e_tag` | String | <p>The current version of the distribution tenant.</p> |
| `distribution_tenant` | String | <p>The distribution tenant that you retrieved.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create distribution_tenant
distribution_tenant = provider.cloudfront.Distribution_tenant {
    distribution_id = "value"  # <p>The ID of the multi-tenant distribution to use for creating the distribution tenant.</p>
    domains = "value"  # <p>The domains associated with the distribution tenant. You must specify at least one domain in the request.</p>
    name = "value"  # <p>The name of the distribution tenant. Enter a friendly identifier that is unique within your Amazon Web Services account. This name can't be updated after you create the distribution tenant.</p>
}

# Access distribution_tenant outputs
distribution_tenant_id = distribution_tenant.id
distribution_tenant_e_tag = distribution_tenant.e_tag
distribution_tenant_distribution_tenant = distribution_tenant.distribution_tenant
```

---


### Managed_certificate_details

ManagedCertificateDetails resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `managed_certificate_details` | String | <p>Contains details about the CloudFront managed ACM certificate.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access managed_certificate_details outputs
managed_certificate_details_id = managed_certificate_details.id
managed_certificate_details_managed_certificate_details = managed_certificate_details.managed_certificate_details
```

---


### Key_value_store

KeyValueStore resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>The name of the key value store. The minimum length is 1 character and the maximum length is 64 characters.</p> |
| `import_source` | String |  | <p>The S3 bucket that provides the source for the import. The source must be in a valid JSON format.</p> |
| `comment` | String |  | <p>The comment of the key value store.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `e_tag` | String | <p>The <code>ETag</code> of the resulting key value store.</p> |
| `key_value_store` | String | <p>The resulting key value store.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create key_value_store
key_value_store = provider.cloudfront.Key_value_store {
    name = "value"  # <p>The name of the key value store. The minimum length is 1 character and the maximum length is 64 characters.</p>
}

# Access key_value_store outputs
key_value_store_id = key_value_store.id
key_value_store_e_tag = key_value_store.e_tag
key_value_store_key_value_store = key_value_store.key_value_store
```

---


### Key_group_config

KeyGroupConfig resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `key_group_config` | String | <p>The key group configuration.</p> |
| `e_tag` | String | <p>The identifier for this version of the key group.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access key_group_config outputs
key_group_config_id = key_group_config.id
key_group_config_key_group_config = key_group_config.key_group_config
key_group_config_e_tag = key_group_config.e_tag
```

---


### Response_headers_policy

ResponseHeadersPolicy resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `response_headers_policy_config` | String | ✅ | <p>Contains metadata about the response headers policy, and a set of configurations that specify the HTTP headers.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `e_tag` | String | <p>The version identifier for the current version of the response headers policy.</p> |
| `response_headers_policy` | String | <p>Contains a response headers policy.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create response_headers_policy
response_headers_policy = provider.cloudfront.Response_headers_policy {
    response_headers_policy_config = "value"  # <p>Contains metadata about the response headers policy, and a set of configurations that specify the HTTP headers.</p>
}

# Access response_headers_policy outputs
response_headers_policy_id = response_headers_policy.id
response_headers_policy_e_tag = response_headers_policy.e_tag
response_headers_policy_response_headers_policy = response_headers_policy.response_headers_policy
```

---


### Response_headers_policy_config

ResponseHeadersPolicyConfig resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `response_headers_policy_config` | String | <p>Contains a response headers policy.</p> |
| `e_tag` | String | <p>The version identifier for the current version of the response headers policy.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access response_headers_policy_config outputs
response_headers_policy_config_id = response_headers_policy_config.id
response_headers_policy_config_response_headers_policy_config = response_headers_policy_config.response_headers_policy_config
response_headers_policy_config_e_tag = response_headers_policy_config.e_tag
```

---


### Realtime_log_config

RealtimeLogConfig resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `fields` | Vec<String> | ✅ | <p>A list of fields to include in each real-time log record.</p> <p>For more information about fields, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/real-time-logs.html#understand-real-time-log-config-fields">Real-time log configuration fields</a> in the <i>Amazon CloudFront Developer Guide</i>.</p> |
| `end_points` | Vec<String> | ✅ | <p>Contains information about the Amazon Kinesis data stream where you are sending real-time log data.</p> |
| `name` | String | ✅ | <p>A unique name to identify this real-time log configuration.</p> |
| `sampling_rate` | i64 | ✅ | <p>The sampling rate for this real-time log configuration. You can specify a whole number between 1 and 100 (inclusive) to determine the percentage of viewer requests that are represented in the real-time log data.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `realtime_log_config` | String | <p>A real-time log configuration.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create realtime_log_config
realtime_log_config = provider.cloudfront.Realtime_log_config {
    fields = "value"  # <p>A list of fields to include in each real-time log record.</p> <p>For more information about fields, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/real-time-logs.html#understand-real-time-log-config-fields">Real-time log configuration fields</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    end_points = "value"  # <p>Contains information about the Amazon Kinesis data stream where you are sending real-time log data.</p>
    name = "value"  # <p>A unique name to identify this real-time log configuration.</p>
    sampling_rate = "value"  # <p>The sampling rate for this real-time log configuration. You can specify a whole number between 1 and 100 (inclusive) to determine the percentage of viewer requests that are represented in the real-time log data.</p>
}

# Access realtime_log_config outputs
realtime_log_config_id = realtime_log_config.id
realtime_log_config_realtime_log_config = realtime_log_config.realtime_log_config
```

---


### Anycast_ip_list

AnycastIpList resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `ip_count` | i64 | ✅ | <p>The number of static IP addresses that are allocated to the Anycast static IP list. Valid values: 21 or 3.</p> |
| `name` | String | ✅ | <p>Name of the Anycast static IP list.</p> |
| `tags` | String |  |  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `anycast_ip_list` | String | <p>The Anycast static IP list details.</p> |
| `e_tag` | String | <p>The version identifier for the current version of the Anycast static IP list.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create anycast_ip_list
anycast_ip_list = provider.cloudfront.Anycast_ip_list {
    ip_count = "value"  # <p>The number of static IP addresses that are allocated to the Anycast static IP list. Valid values: 21 or 3.</p>
    name = "value"  # <p>Name of the Anycast static IP list.</p>
}

# Access anycast_ip_list outputs
anycast_ip_list_id = anycast_ip_list.id
anycast_ip_list_anycast_ip_list = anycast_ip_list.anycast_ip_list
anycast_ip_list_e_tag = anycast_ip_list.e_tag
```

---


### Field_level_encryption_profile_config

FieldLevelEncryptionProfileConfig resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `e_tag` | String | <p>The current version of the field-level encryption profile configuration result. For example: <code>E2QWRUHAPOMQZL</code>.</p> |
| `field_level_encryption_profile_config` | String | <p>Return the field-level encryption profile configuration information.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access field_level_encryption_profile_config outputs
field_level_encryption_profile_config_id = field_level_encryption_profile_config.id
field_level_encryption_profile_config_e_tag = field_level_encryption_profile_config.e_tag
field_level_encryption_profile_config_field_level_encryption_profile_config = field_level_encryption_profile_config.field_level_encryption_profile_config
```

---


### Field_level_encryption_profile

FieldLevelEncryptionProfile resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `field_level_encryption_profile_config` | String | ✅ | <p>The request to create a field-level encryption profile.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `e_tag` | String | <p>The current version of the field level encryption profile. For example: <code>E2QWRUHAPOMQZL</code>.</p> |
| `field_level_encryption_profile` | String | <p>Return the field-level encryption profile information.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create field_level_encryption_profile
field_level_encryption_profile = provider.cloudfront.Field_level_encryption_profile {
    field_level_encryption_profile_config = "value"  # <p>The request to create a field-level encryption profile.</p>
}

# Access field_level_encryption_profile outputs
field_level_encryption_profile_id = field_level_encryption_profile.id
field_level_encryption_profile_e_tag = field_level_encryption_profile.e_tag
field_level_encryption_profile_field_level_encryption_profile = field_level_encryption_profile.field_level_encryption_profile
```

---


### Streaming_distribution

StreamingDistribution resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `streaming_distribution_config` | String | ✅ | <p>The streaming distribution's configuration information.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `streaming_distribution` | String | <p>The streaming distribution's information.</p> |
| `e_tag` | String | <p>The current version of the streaming distribution's information. For example: <code>E2QWRUHAPOMQZL</code>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create streaming_distribution
streaming_distribution = provider.cloudfront.Streaming_distribution {
    streaming_distribution_config = "value"  # <p>The streaming distribution's configuration information.</p>
}

# Access streaming_distribution outputs
streaming_distribution_id = streaming_distribution.id
streaming_distribution_streaming_distribution = streaming_distribution.streaming_distribution
streaming_distribution_e_tag = streaming_distribution.e_tag
```

---


### Distribution_config

DistributionConfig resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `e_tag` | String | <p>The current version of the configuration. For example: <code>E2QWRUHAPOMQZL</code>.</p> |
| `distribution_config` | String | <p>The distribution's configuration information.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access distribution_config outputs
distribution_config_id = distribution_config.id
distribution_config_e_tag = distribution_config.e_tag
distribution_config_distribution_config = distribution_config.distribution_config
```

---


### Public_key

PublicKey resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `public_key_config` | String | ✅ | <p>A CloudFront public key configuration.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `public_key` | String | <p>The public key.</p> |
| `e_tag` | String | <p>The identifier for this version of the public key.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create public_key
public_key = provider.cloudfront.Public_key {
    public_key_config = "value"  # <p>A CloudFront public key configuration.</p>
}

# Access public_key outputs
public_key_id = public_key.id
public_key_public_key = public_key.public_key
public_key_e_tag = public_key.e_tag
```

---


### Public_key_config

PublicKeyConfig resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `public_key_config` | String | <p>A public key configuration.</p> |
| `e_tag` | String | <p>The identifier for this version of the public key configuration.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access public_key_config outputs
public_key_config_id = public_key_config.id
public_key_config_public_key_config = public_key_config.public_key_config
public_key_config_e_tag = public_key_config.e_tag
```

---


### Streaming_distribution_with_tags

StreamingDistributionWithTags resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `streaming_distribution_config_with_tags` | String | ✅ | <p>The streaming distribution's configuration information.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create streaming_distribution_with_tags
streaming_distribution_with_tags = provider.cloudfront.Streaming_distribution_with_tags {
    streaming_distribution_config_with_tags = "value"  # <p>The streaming distribution's configuration information.</p>
}

```

---


### Cloud_front_origin_access_identity

CloudFrontOriginAccessIdentity resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `cloud_front_origin_access_identity_config` | String | ✅ | <p>The current configuration information for the identity.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `e_tag` | String | <p>The current version of the origin access identity's information. For example: <code>E2QWRUHAPOMQZL</code>.</p> |
| `cloud_front_origin_access_identity` | String | <p>The origin access identity's information.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create cloud_front_origin_access_identity
cloud_front_origin_access_identity = provider.cloudfront.Cloud_front_origin_access_identity {
    cloud_front_origin_access_identity_config = "value"  # <p>The current configuration information for the identity.</p>
}

# Access cloud_front_origin_access_identity outputs
cloud_front_origin_access_identity_id = cloud_front_origin_access_identity.id
cloud_front_origin_access_identity_e_tag = cloud_front_origin_access_identity.e_tag
cloud_front_origin_access_identity_cloud_front_origin_access_identity = cloud_front_origin_access_identity.cloud_front_origin_access_identity
```

---


### Continuous_deployment_policy_config

ContinuousDeploymentPolicyConfig resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `continuous_deployment_policy_config` | String |  |
| `e_tag` | String | <p>The version identifier for the current version of the continuous deployment policy.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access continuous_deployment_policy_config outputs
continuous_deployment_policy_config_id = continuous_deployment_policy_config.id
continuous_deployment_policy_config_continuous_deployment_policy_config = continuous_deployment_policy_config.continuous_deployment_policy_config
continuous_deployment_policy_config_e_tag = continuous_deployment_policy_config.e_tag
```

---


### Continuous_deployment_policy

ContinuousDeploymentPolicy resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `continuous_deployment_policy_config` | String | ✅ | <p>Contains the configuration for a continuous deployment policy.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `continuous_deployment_policy` | String | <p>A continuous deployment policy.</p> |
| `e_tag` | String | <p>The version identifier for the current version of the continuous deployment policy.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create continuous_deployment_policy
continuous_deployment_policy = provider.cloudfront.Continuous_deployment_policy {
    continuous_deployment_policy_config = "value"  # <p>Contains the configuration for a continuous deployment policy.</p>
}

# Access continuous_deployment_policy outputs
continuous_deployment_policy_id = continuous_deployment_policy.id
continuous_deployment_policy_continuous_deployment_policy = continuous_deployment_policy.continuous_deployment_policy
continuous_deployment_policy_e_tag = continuous_deployment_policy.e_tag
```

---


### Invalidation_for_distribution_tenant

InvalidationForDistributionTenant resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | String | ✅ | <p>The ID of the distribution tenant.</p> |
| `invalidation_batch` | String | ✅ |  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `invalidation` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create invalidation_for_distribution_tenant
invalidation_for_distribution_tenant = provider.cloudfront.Invalidation_for_distribution_tenant {
    id = "value"  # <p>The ID of the distribution tenant.</p>
    invalidation_batch = "value"  # Required field
}

# Access invalidation_for_distribution_tenant outputs
invalidation_for_distribution_tenant_id = invalidation_for_distribution_tenant.id
invalidation_for_distribution_tenant_invalidation = invalidation_for_distribution_tenant.invalidation
```

---


### Distribution

Distribution resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `distribution_config` | String | ✅ | <p>The distribution's configuration information.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `e_tag` | String | <p>The current version of the distribution's information. For example: <code>E2QWRUHAPOMQZL</code>.</p> |
| `distribution` | String | <p>The distribution's information.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create distribution
distribution = provider.cloudfront.Distribution {
    distribution_config = "value"  # <p>The distribution's configuration information.</p>
}

# Access distribution outputs
distribution_id = distribution.id
distribution_e_tag = distribution.e_tag
distribution_distribution = distribution.distribution
```

---


### Key_group

KeyGroup resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `key_group_config` | String | ✅ | <p>A key group configuration.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `key_group` | String | <p>The key group.</p> |
| `e_tag` | String | <p>The identifier for this version of the key group.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create key_group
key_group = provider.cloudfront.Key_group {
    key_group_config = "value"  # <p>A key group configuration.</p>
}

# Access key_group outputs
key_group_id = key_group.id
key_group_key_group = key_group.key_group
key_group_e_tag = key_group.e_tag
```

---


### Field_level_encryption_config

FieldLevelEncryptionConfig resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `field_level_encryption_config` | String | ✅ | <p>The request to create a new field-level encryption configuration.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `e_tag` | String | <p>The current version of the field level encryption configuration. For example: <code>E2QWRUHAPOMQZL</code>.</p> |
| `field_level_encryption_config` | String | <p>Return the field-level encryption configuration information.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create field_level_encryption_config
field_level_encryption_config = provider.cloudfront.Field_level_encryption_config {
    field_level_encryption_config = "value"  # <p>The request to create a new field-level encryption configuration.</p>
}

# Access field_level_encryption_config outputs
field_level_encryption_config_id = field_level_encryption_config.id
field_level_encryption_config_e_tag = field_level_encryption_config.e_tag
field_level_encryption_config_field_level_encryption_config = field_level_encryption_config.field_level_encryption_config
```

---


### Origin_request_policy_config

OriginRequestPolicyConfig resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `e_tag` | String | <p>The current version of the origin request policy.</p> |
| `origin_request_policy_config` | String | <p>The origin request policy configuration.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access origin_request_policy_config outputs
origin_request_policy_config_id = origin_request_policy_config.id
origin_request_policy_config_e_tag = origin_request_policy_config.e_tag
origin_request_policy_config_origin_request_policy_config = origin_request_policy_config.origin_request_policy_config
```

---


### Distribution_tenant_by_domain

DistributionTenantByDomain resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `distribution_tenant` | String |  |
| `e_tag` | String | <p>The current version of the distribution tenant.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access distribution_tenant_by_domain outputs
distribution_tenant_by_domain_id = distribution_tenant_by_domain.id
distribution_tenant_by_domain_distribution_tenant = distribution_tenant_by_domain.distribution_tenant
distribution_tenant_by_domain_e_tag = distribution_tenant_by_domain.e_tag
```

---


### Connection_group_by_routing_endpoint

ConnectionGroupByRoutingEndpoint resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `e_tag` | String | <p>The current version of the connection group.</p> |
| `connection_group` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access connection_group_by_routing_endpoint outputs
connection_group_by_routing_endpoint_id = connection_group_by_routing_endpoint.id
connection_group_by_routing_endpoint_e_tag = connection_group_by_routing_endpoint.e_tag
connection_group_by_routing_endpoint_connection_group = connection_group_by_routing_endpoint.connection_group
```

---


### Connection_group

ConnectionGroup resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | String |  |  |
| `anycast_ip_list_id` | String |  | <p>The ID of the Anycast static IP list.</p> |
| `enabled` | bool |  | <p>Enable the connection group.</p> |
| `name` | String | ✅ | <p>The name of the connection group. Enter a friendly identifier that is unique within your Amazon Web Services account. This name can't be updated after you create the connection group.</p> |
| `ipv6_enabled` | bool |  | <p>Enable IPv6 for the connection group. The default is <code>true</code>. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/distribution-web-values-specify.html#DownloadDistValuesEnableIPv6">Enable IPv6</a> in the <i>Amazon CloudFront Developer Guide</i>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `e_tag` | String | <p>The current version of the connection group.</p> |
| `connection_group` | String | <p>The connection group that you retrieved.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create connection_group
connection_group = provider.cloudfront.Connection_group {
    name = "value"  # <p>The name of the connection group. Enter a friendly identifier that is unique within your Amazon Web Services account. This name can't be updated after you create the connection group.</p>
}

# Access connection_group outputs
connection_group_id = connection_group.id
connection_group_e_tag = connection_group.e_tag
connection_group_connection_group = connection_group.connection_group
```

---


### Distribution_with_tags

DistributionWithTags resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `distribution_config_with_tags` | String | ✅ | <p>The distribution's configuration information.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create distribution_with_tags
distribution_with_tags = provider.cloudfront.Distribution_with_tags {
    distribution_config_with_tags = "value"  # <p>The distribution's configuration information.</p>
}

```

---


### Function

Function resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `function_config` | String | ✅ | <p>Configuration information about the function, including an optional comment and the function's runtime.</p> |
| `name` | String | ✅ | <p>A name to identify the function.</p> |
| `function_code` | String | ✅ | <p>The function code. For more information about writing a CloudFront function, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/writing-function-code.html">Writing function code for CloudFront Functions</a> in the <i>Amazon CloudFront Developer Guide</i>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `function_code` | String | <p>The function code of a CloudFront function.</p> |
| `e_tag` | String | <p>The version identifier for the current version of the CloudFront function.</p> |
| `content_type` | String | <p>The content type (media type) of the response.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create function
function = provider.cloudfront.Function {
    function_config = "value"  # <p>Configuration information about the function, including an optional comment and the function's runtime.</p>
    name = "value"  # <p>A name to identify the function.</p>
    function_code = "value"  # <p>The function code. For more information about writing a CloudFront function, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/writing-function-code.html">Writing function code for CloudFront Functions</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
}

# Access function outputs
function_id = function.id
function_function_code = function.function_code
function_e_tag = function.e_tag
function_content_type = function.content_type
```

---


### Cloud_front_origin_access_identity_config

CloudFrontOriginAccessIdentityConfig resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `e_tag` | String | <p>The current version of the configuration. For example: <code>E2QWRUHAPOMQZL</code>.</p> |
| `cloud_front_origin_access_identity_config` | String | <p>The origin access identity's configuration information.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access cloud_front_origin_access_identity_config outputs
cloud_front_origin_access_identity_config_id = cloud_front_origin_access_identity_config.id
cloud_front_origin_access_identity_config_e_tag = cloud_front_origin_access_identity_config.e_tag
cloud_front_origin_access_identity_config_cloud_front_origin_access_identity_config = cloud_front_origin_access_identity_config.cloud_front_origin_access_identity_config
```

---


### Domain_association

DomainAssociation resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `if_match` | String |  | <p>The value of the <code>ETag</code> identifier for the standard distribution or distribution tenant that will be associated with the domain.</p> |
| `domain` | String | ✅ | <p>The domain to update.</p> |
| `target_resource` | String | ✅ | <p>The target standard distribution or distribution tenant resource for the domain. You can specify either <code>DistributionId</code> or <code>DistributionTenantId</code>, but not both.</p> |



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


### Invalidation

Invalidation resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `distribution_id` | String | ✅ | <p>The distribution's id.</p> |
| `invalidation_batch` | String | ✅ | <p>The batch information for the invalidation.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `invalidation` | String | <p>The invalidation's information. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/InvalidationDatatype.html">Invalidation Complex Type</a>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create invalidation
invalidation = provider.cloudfront.Invalidation {
    distribution_id = "value"  # <p>The distribution's id.</p>
    invalidation_batch = "value"  # <p>The batch information for the invalidation.</p>
}

# Access invalidation outputs
invalidation_id = invalidation.id
invalidation_invalidation = invalidation.invalidation
```

---


### Monitoring_subscription

MonitoringSubscription resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `distribution_id` | String | ✅ | <p>The ID of the distribution that you are enabling metrics for.</p> |
| `monitoring_subscription` | String | ✅ | <p>A monitoring subscription. This structure contains information about whether additional CloudWatch metrics are enabled for a given CloudFront distribution.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `monitoring_subscription` | String | <p>A monitoring subscription. This structure contains information about whether additional CloudWatch metrics are enabled for a given CloudFront distribution.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create monitoring_subscription
monitoring_subscription = provider.cloudfront.Monitoring_subscription {
    distribution_id = "value"  # <p>The ID of the distribution that you are enabling metrics for.</p>
    monitoring_subscription = "value"  # <p>A monitoring subscription. This structure contains information about whether additional CloudWatch metrics are enabled for a given CloudFront distribution.</p>
}

# Access monitoring_subscription outputs
monitoring_subscription_id = monitoring_subscription.id
monitoring_subscription_monitoring_subscription = monitoring_subscription.monitoring_subscription
```

---


### Distribution_with_staging_config

DistributionWithStagingConfig resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | String | ✅ | <p>The identifier of the primary distribution to which you are copying a staging distribution's configuration.</p> |
| `if_match` | String |  | <p>The current versions (<code>ETag</code> values) of both primary and staging distributions. Provide these in the following format:</p> <p> <code>&lt;primary ETag&gt;, &lt;staging ETag&gt;</code> </p> |
| `staging_distribution_id` | String |  | <p>The identifier of the staging distribution whose configuration you are copying to the primary distribution.</p> |



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


### Cache_policy_config

CachePolicyConfig resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `e_tag` | String | <p>The current version of the cache policy.</p> |
| `cache_policy_config` | String | <p>The cache policy configuration.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access cache_policy_config outputs
cache_policy_config_id = cache_policy_config.id
cache_policy_config_e_tag = cache_policy_config.e_tag
cache_policy_config_cache_policy_config = cache_policy_config.cache_policy_config
```

---


### Vpc_origin

VpcOrigin resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `vpc_origin_endpoint_config` | String | ✅ | <p>The VPC origin endpoint configuration.</p> |
| `tags` | String |  |  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `e_tag` | String | <p>The VPC origin ETag.</p> |
| `vpc_origin` | String | <p>The VPC origin.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create vpc_origin
vpc_origin = provider.cloudfront.Vpc_origin {
    vpc_origin_endpoint_config = "value"  # <p>The VPC origin endpoint configuration.</p>
}

# Access vpc_origin outputs
vpc_origin_id = vpc_origin.id
vpc_origin_e_tag = vpc_origin.e_tag
vpc_origin_vpc_origin = vpc_origin.vpc_origin
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple field_level_encryption resources
field_level_encryption_0 = provider.cloudfront.Field_level_encryption {
}
field_level_encryption_1 = provider.cloudfront.Field_level_encryption {
}
field_level_encryption_2 = provider.cloudfront.Field_level_encryption {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    field_level_encryption = provider.cloudfront.Field_level_encryption {
    }
```

---

## Related Documentation

- [AWS Cloudfront Documentation](https://docs.aws.amazon.com/cloudfront/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
