# Route_53 Service



**Resources**: 24

---

## Overview

The route_53 service provides access to 24 resource types:

- [Traffic_policy_comment](#traffic_policy_comment) [U]
- [Vpc_association_authorization](#vpc_association_authorization) [CD]
- [Hosted_zone_comment](#hosted_zone_comment) [U]
- [Checker_ip_ranges](#checker_ip_ranges) [R]
- [Health_check_last_failure_reason](#health_check_last_failure_reason) [R]
- [Cidr_collection](#cidr_collection) [CD]
- [Health_check](#health_check) [CRUD]
- [Traffic_policy_version](#traffic_policy_version) [C]
- [Health_check_count](#health_check_count) [R]
- [Reusable_delegation_set](#reusable_delegation_set) [CRD]
- [Hosted_zone_count](#hosted_zone_count) [R]
- [Change](#change) [R]
- [Geo_location](#geo_location) [R]
- [Query_logging_config](#query_logging_config) [CRD]
- [Health_check_status](#health_check_status) [R]
- [Hosted_zone_limit](#hosted_zone_limit) [R]
- [Traffic_policy](#traffic_policy) [CRD]
- [Reusable_delegation_set_limit](#reusable_delegation_set_limit) [R]
- [Traffic_policy_instance](#traffic_policy_instance) [CRUD]
- [Key_signing_key](#key_signing_key) [CD]
- [Hosted_zone](#hosted_zone) [CRD]
- [Account_limit](#account_limit) [R]
- [Traffic_policy_instance_count](#traffic_policy_instance_count) [R]
- [Dnssec](#dnssec) [R]

---

## Resources


### Traffic_policy_comment

TrafficPolicyComment resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | String | ✅ | <p>The value of <code>Id</code> for the traffic policy that you want to update the
			comment for.</p> |
| `version` | i64 | ✅ | <p>The value of <code>Version</code> for the traffic policy that you want to update the
			comment for.</p> |
| `comment` | String | ✅ | <p>The new comment for the specified traffic policy and version.</p> |



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


### Vpc_association_authorization

VPCAssociationAuthorization resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `hosted_zone_id` | String | ✅ | <p>The ID of the private hosted zone that you want to authorize associating a VPC
			with.</p> |
| `vpc` | String | ✅ | <p>A complex type that contains the VPC ID and region for the VPC that you want to
			authorize associating with your hosted zone.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create vpc_association_authorization
vpc_association_authorization = provider.route_53.Vpc_association_authorization {
    hosted_zone_id = "value"  # <p>The ID of the private hosted zone that you want to authorize associating a VPC
			with.</p>
    vpc = "value"  # <p>A complex type that contains the VPC ID and region for the VPC that you want to
			authorize associating with your hosted zone.</p>
}

```

---


### Hosted_zone_comment

HostedZoneComment resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `comment` | String |  | <p>The new comment for the hosted zone. If you don't specify a value for
				<code>Comment</code>, Amazon Route 53 deletes the existing value of the
				<code>Comment</code> element, if any.</p> |
| `id` | String | ✅ | <p>The ID for the hosted zone that you want to update the comment for.</p> |



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


### Checker_ip_ranges

CheckerIpRanges resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `checker_ip_ranges` | Vec<String> | <p>A complex type that contains sorted list of IP ranges in CIDR format for Amazon Route
			53 health checkers.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access checker_ip_ranges outputs
checker_ip_ranges_id = checker_ip_ranges.id
checker_ip_ranges_checker_ip_ranges = checker_ip_ranges.checker_ip_ranges
```

---


### Health_check_last_failure_reason

HealthCheckLastFailureReason resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `health_check_observations` | Vec<String> | <p>A list that contains one <code>Observation</code> element for each Amazon Route 53
			health checker that is reporting a last failure reason. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access health_check_last_failure_reason outputs
health_check_last_failure_reason_id = health_check_last_failure_reason.id
health_check_last_failure_reason_health_check_observations = health_check_last_failure_reason.health_check_observations
```

---


### Cidr_collection

CidrCollection resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `caller_reference` | String | ✅ | <p>A client-specific token that allows requests to be securely retried so that the
			intended outcome will only occur once, retries receive a similar response, and there are
			no additional edge cases to handle.</p> |
| `name` | String | ✅ | <p>A unique identifier for the account that can be used to reference the collection from
			other API calls.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create cidr_collection
cidr_collection = provider.route_53.Cidr_collection {
    caller_reference = "value"  # <p>A client-specific token that allows requests to be securely retried so that the
			intended outcome will only occur once, retries receive a similar response, and there are
			no additional edge cases to handle.</p>
    name = "value"  # <p>A unique identifier for the account that can be used to reference the collection from
			other API calls.</p>
}

```

---


### Health_check

HealthCheck resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `caller_reference` | String | ✅ | <p>A unique string that identifies the request and that allows you to retry a failed
				<code>CreateHealthCheck</code> request without the risk of creating two identical
			health checks:</p>
         <ul>
            <li>
               <p>If you send a <code>CreateHealthCheck</code> request with the same
						<code>CallerReference</code> and settings as a previous request, and if the
					health check doesn't exist, Amazon Route 53 creates the health check. If the
					health check does exist, Route 53 returns the health check configuration in the
					response. </p>
            </li>
            <li>
               <p>If you send a <code>CreateHealthCheck</code> request with the same
						<code>CallerReference</code> as a deleted health check, regardless of the
					settings, Route 53 returns a <code>HealthCheckAlreadyExists</code> error.</p>
            </li>
            <li>
               <p>If you send a <code>CreateHealthCheck</code> request with the same
						<code>CallerReference</code> as an existing health check but with different
					settings, Route 53 returns a <code>HealthCheckAlreadyExists</code> error.</p>
            </li>
            <li>
               <p>If you send a <code>CreateHealthCheck</code> request with a unique
						<code>CallerReference</code> but settings identical to an existing health
					check, Route 53 creates the health check.</p>
            </li>
         </ul>
         <p> Route 53 does not store the <code>CallerReference</code> for a deleted health check indefinitely. 
			The <code>CallerReference</code> for a deleted health check will be deleted after a number of days.</p> |
| `health_check_config` | String | ✅ | <p>A complex type that contains settings for a new health check.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `health_check` | String | <p>A complex type that contains information about one health check that is associated
			with the current Amazon Web Services account.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create health_check
health_check = provider.route_53.Health_check {
    caller_reference = "value"  # <p>A unique string that identifies the request and that allows you to retry a failed
				<code>CreateHealthCheck</code> request without the risk of creating two identical
			health checks:</p>
         <ul>
            <li>
               <p>If you send a <code>CreateHealthCheck</code> request with the same
						<code>CallerReference</code> and settings as a previous request, and if the
					health check doesn't exist, Amazon Route 53 creates the health check. If the
					health check does exist, Route 53 returns the health check configuration in the
					response. </p>
            </li>
            <li>
               <p>If you send a <code>CreateHealthCheck</code> request with the same
						<code>CallerReference</code> as a deleted health check, regardless of the
					settings, Route 53 returns a <code>HealthCheckAlreadyExists</code> error.</p>
            </li>
            <li>
               <p>If you send a <code>CreateHealthCheck</code> request with the same
						<code>CallerReference</code> as an existing health check but with different
					settings, Route 53 returns a <code>HealthCheckAlreadyExists</code> error.</p>
            </li>
            <li>
               <p>If you send a <code>CreateHealthCheck</code> request with a unique
						<code>CallerReference</code> but settings identical to an existing health
					check, Route 53 creates the health check.</p>
            </li>
         </ul>
         <p> Route 53 does not store the <code>CallerReference</code> for a deleted health check indefinitely. 
			The <code>CallerReference</code> for a deleted health check will be deleted after a number of days.</p>
    health_check_config = "value"  # <p>A complex type that contains settings for a new health check.</p>
}

# Access health_check outputs
health_check_id = health_check.id
health_check_health_check = health_check.health_check
```

---


### Traffic_policy_version

TrafficPolicyVersion resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `comment` | String |  | <p>The comment that you specified in the <code>CreateTrafficPolicyVersion</code> request,
			if any.</p> |
| `id` | String | ✅ | <p>The ID of the traffic policy for which you want to create a new version.</p> |
| `document` | String | ✅ | <p>The definition of this version of the traffic policy, in JSON format. You specified
			the JSON in the <code>CreateTrafficPolicyVersion</code> request. For more information
			about the JSON format, see <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_CreateTrafficPolicy.html">CreateTrafficPolicy</a>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create traffic_policy_version
traffic_policy_version = provider.route_53.Traffic_policy_version {
    id = "value"  # <p>The ID of the traffic policy for which you want to create a new version.</p>
    document = "value"  # <p>The definition of this version of the traffic policy, in JSON format. You specified
			the JSON in the <code>CreateTrafficPolicyVersion</code> request. For more information
			about the JSON format, see <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_CreateTrafficPolicy.html">CreateTrafficPolicy</a>.</p>
}

```

---


### Health_check_count

HealthCheckCount resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `health_check_count` | i64 | <p>The number of health checks associated with the current Amazon Web Services account.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access health_check_count outputs
health_check_count_id = health_check_count.id
health_check_count_health_check_count = health_check_count.health_check_count
```

---


### Reusable_delegation_set

ReusableDelegationSet resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `hosted_zone_id` | String |  | <p>If you want to mark the delegation set for an existing hosted zone as reusable, the ID
			for that hosted zone.</p> |
| `caller_reference` | String | ✅ | <p>A unique string that identifies the request, and that allows you to retry failed
				<code>CreateReusableDelegationSet</code> requests without the risk of executing the
			operation twice. You must use a unique <code>CallerReference</code> string every time
			you submit a <code>CreateReusableDelegationSet</code> request.
				<code>CallerReference</code> can be any unique string, for example a date/time
			stamp.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `delegation_set` | String | <p>A complex type that contains information about the reusable delegation set.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create reusable_delegation_set
reusable_delegation_set = provider.route_53.Reusable_delegation_set {
    caller_reference = "value"  # <p>A unique string that identifies the request, and that allows you to retry failed
				<code>CreateReusableDelegationSet</code> requests without the risk of executing the
			operation twice. You must use a unique <code>CallerReference</code> string every time
			you submit a <code>CreateReusableDelegationSet</code> request.
				<code>CallerReference</code> can be any unique string, for example a date/time
			stamp.</p>
}

# Access reusable_delegation_set outputs
reusable_delegation_set_id = reusable_delegation_set.id
reusable_delegation_set_delegation_set = reusable_delegation_set.delegation_set
```

---


### Hosted_zone_count

HostedZoneCount resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `hosted_zone_count` | i64 | <p>The total number of public and private hosted zones that are associated with the
			current Amazon Web Services account.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access hosted_zone_count outputs
hosted_zone_count_id = hosted_zone_count.id
hosted_zone_count_hosted_zone_count = hosted_zone_count.hosted_zone_count
```

---


### Change

Change resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `change_info` | String | <p>A complex type that contains information about the specified change batch.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access change outputs
change_id = change.id
change_change_info = change.change_info
```

---


### Geo_location

GeoLocation resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `geo_location_details` | String | <p>A complex type that contains the codes and full continent, country, and subdivision
			names for the specified geolocation code.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access geo_location outputs
geo_location_id = geo_location.id
geo_location_geo_location_details = geo_location.geo_location_details
```

---


### Query_logging_config

QueryLoggingConfig resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `hosted_zone_id` | String | ✅ | <p>The ID of the hosted zone that you want to log queries for. You can log queries only
			for public hosted zones.</p> |
| `cloud_watch_logs_log_group_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) for the log group that you want to Amazon Route 53 to
			send query logs to. This is the format of the ARN:</p>
         <p>arn:aws:logs:<i>region</i>:<i>account-id</i>:log-group:<i>log_group_name</i>
         </p>
         <p>To get the ARN for a log group, you can use the CloudWatch console, the <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_DescribeLogGroups.html">DescribeLogGroups</a> API action, the <a href="https://docs.aws.amazon.com/cli/latest/reference/logs/describe-log-groups.html">describe-log-groups</a>
			command, or the applicable command in one of the Amazon Web Services SDKs.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `query_logging_config` | String | <p>A complex type that contains information about the query logging configuration that
			you specified in a <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_GetQueryLoggingConfig.html">GetQueryLoggingConfig</a> request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create query_logging_config
query_logging_config = provider.route_53.Query_logging_config {
    hosted_zone_id = "value"  # <p>The ID of the hosted zone that you want to log queries for. You can log queries only
			for public hosted zones.</p>
    cloud_watch_logs_log_group_arn = "value"  # <p>The Amazon Resource Name (ARN) for the log group that you want to Amazon Route 53 to
			send query logs to. This is the format of the ARN:</p>
         <p>arn:aws:logs:<i>region</i>:<i>account-id</i>:log-group:<i>log_group_name</i>
         </p>
         <p>To get the ARN for a log group, you can use the CloudWatch console, the <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_DescribeLogGroups.html">DescribeLogGroups</a> API action, the <a href="https://docs.aws.amazon.com/cli/latest/reference/logs/describe-log-groups.html">describe-log-groups</a>
			command, or the applicable command in one of the Amazon Web Services SDKs.</p>
}

# Access query_logging_config outputs
query_logging_config_id = query_logging_config.id
query_logging_config_query_logging_config = query_logging_config.query_logging_config
```

---


### Health_check_status

HealthCheckStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `health_check_observations` | Vec<String> | <p>A list that contains one <code>HealthCheckObservation</code> element for each Amazon
			Route 53 health checker that is reporting a status about the health check
			endpoint.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access health_check_status outputs
health_check_status_id = health_check_status.id
health_check_status_health_check_observations = health_check_status.health_check_observations
```

---


### Hosted_zone_limit

HostedZoneLimit resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `count` | i64 | <p>The current number of entities that you have created of the specified type. For
			example, if you specified <code>MAX_RRSETS_BY_ZONE</code> for the value of
				<code>Type</code> in the request, the value of <code>Count</code> is the current
			number of records that you have created in the specified hosted zone.</p> |
| `limit` | String | <p>The current setting for the specified limit. For example, if you specified
				<code>MAX_RRSETS_BY_ZONE</code> for the value of <code>Type</code> in the request,
			the value of <code>Limit</code> is the maximum number of records that you can create in
			the specified hosted zone.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access hosted_zone_limit outputs
hosted_zone_limit_id = hosted_zone_limit.id
hosted_zone_limit_count = hosted_zone_limit.count
hosted_zone_limit_limit = hosted_zone_limit.limit
```

---


### Traffic_policy

TrafficPolicy resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `document` | String | ✅ | <p>The definition of this traffic policy in JSON format. For more information, see <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/api-policies-traffic-policy-document-format.html">Traffic Policy Document Format</a>.</p> |
| `comment` | String |  | <p>(Optional) Any comments that you want to include about the traffic policy.</p> |
| `name` | String | ✅ | <p>The name of the traffic policy.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `traffic_policy` | String | <p>A complex type that contains settings for the specified traffic policy.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create traffic_policy
traffic_policy = provider.route_53.Traffic_policy {
    document = "value"  # <p>The definition of this traffic policy in JSON format. For more information, see <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/api-policies-traffic-policy-document-format.html">Traffic Policy Document Format</a>.</p>
    name = "value"  # <p>The name of the traffic policy.</p>
}

# Access traffic_policy outputs
traffic_policy_id = traffic_policy.id
traffic_policy_traffic_policy = traffic_policy.traffic_policy
```

---


### Reusable_delegation_set_limit

ReusableDelegationSetLimit resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `count` | i64 | <p>The current number of hosted zones that you can associate with the specified reusable
			delegation set.</p> |
| `limit` | String | <p>The current setting for the limit on hosted zones that you can associate with the
			specified reusable delegation set.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access reusable_delegation_set_limit outputs
reusable_delegation_set_limit_id = reusable_delegation_set_limit.id
reusable_delegation_set_limit_count = reusable_delegation_set_limit.count
reusable_delegation_set_limit_limit = reusable_delegation_set_limit.limit
```

---


### Traffic_policy_instance

TrafficPolicyInstance resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>The domain name (such as example.com) or subdomain name (such as www.example.com) for
			which Amazon Route 53 responds to DNS queries by using the resource record sets that
			Route 53 creates for this traffic policy instance.</p> |
| `hosted_zone_id` | String | ✅ | <p>The ID of the hosted zone that you want Amazon Route 53 to create resource record sets
			in by using the configuration in a traffic policy.</p> |
| `traffic_policy_version` | i64 | ✅ | <p>The version of the traffic policy that you want to use to create resource record sets
			in the specified hosted zone.</p> |
| `traffic_policy_id` | String | ✅ | <p>The ID of the traffic policy that you want to use to create resource record sets in
			the specified hosted zone.</p> |
| `ttl` | i64 | ✅ | <p>(Optional) The TTL that you want Amazon Route 53 to assign to all of the resource
			record sets that it creates in the specified hosted zone.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `traffic_policy_instance` | String | <p>A complex type that contains settings for the traffic policy instance.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create traffic_policy_instance
traffic_policy_instance = provider.route_53.Traffic_policy_instance {
    name = "value"  # <p>The domain name (such as example.com) or subdomain name (such as www.example.com) for
			which Amazon Route 53 responds to DNS queries by using the resource record sets that
			Route 53 creates for this traffic policy instance.</p>
    hosted_zone_id = "value"  # <p>The ID of the hosted zone that you want Amazon Route 53 to create resource record sets
			in by using the configuration in a traffic policy.</p>
    traffic_policy_version = "value"  # <p>The version of the traffic policy that you want to use to create resource record sets
			in the specified hosted zone.</p>
    traffic_policy_id = "value"  # <p>The ID of the traffic policy that you want to use to create resource record sets in
			the specified hosted zone.</p>
    ttl = "value"  # <p>(Optional) The TTL that you want Amazon Route 53 to assign to all of the resource
			record sets that it creates in the specified hosted zone.</p>
}

# Access traffic_policy_instance outputs
traffic_policy_instance_id = traffic_policy_instance.id
traffic_policy_instance_traffic_policy_instance = traffic_policy_instance.traffic_policy_instance
```

---


### Key_signing_key

KeySigningKey resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `caller_reference` | String | ✅ | <p>A unique string that identifies the request.</p> |
| `name` | String | ✅ | <p>A string used to identify a key-signing key (KSK). <code>Name</code> can include
			numbers, letters, and underscores (_). <code>Name</code> must be unique for each
			key-signing key in the same hosted zone.</p> |
| `status` | String | ✅ | <p>A string specifying the initial status of the key-signing key (KSK). You can set the
			value to <code>ACTIVE</code> or <code>INACTIVE</code>.</p> |
| `key_management_service_arn` | String | ✅ | <p>The Amazon resource name (ARN) for a customer managed key in Key Management Service
				(KMS). The <code>KeyManagementServiceArn</code> must be unique for
			each key-signing key (KSK) in a single hosted zone. To see an example of
				<code>KeyManagementServiceArn</code> that grants the correct permissions for DNSSEC,
			scroll down to <b>Example</b>. </p>
         <p>You must configure the customer managed customer managed key as follows:</p>
         <dl>
            <dt>Status</dt>
            <dd>
               <p>Enabled</p>
            </dd>
            <dt>Key spec</dt>
            <dd>
               <p>ECC_NIST_P256</p>
            </dd>
            <dt>Key usage</dt>
            <dd>
               <p>Sign and verify</p>
            </dd>
            <dt>Key policy</dt>
            <dd>
               <p>The key policy must give permission for the following actions:</p>
               <ul>
                  <li>
                     <p>DescribeKey</p>
                  </li>
                  <li>
                     <p>GetPublicKey</p>
                  </li>
                  <li>
                     <p>Sign</p>
                  </li>
               </ul>
               <p>The key policy must also include the Amazon Route 53 service in the
						principal for your account. Specify the following:</p>
               <ul>
                  <li>
                     <p>
                        <code>"Service": "dnssec-route53.amazonaws.com"</code>
                     </p>
                  </li>
               </ul>
            </dd>
         </dl>
         <p>For more information about working with a customer managed key in KMS, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html">Key Management Service concepts</a>.</p> |
| `hosted_zone_id` | String | ✅ | <p>The unique string (ID) used to identify a hosted zone.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create key_signing_key
key_signing_key = provider.route_53.Key_signing_key {
    caller_reference = "value"  # <p>A unique string that identifies the request.</p>
    name = "value"  # <p>A string used to identify a key-signing key (KSK). <code>Name</code> can include
			numbers, letters, and underscores (_). <code>Name</code> must be unique for each
			key-signing key in the same hosted zone.</p>
    status = "value"  # <p>A string specifying the initial status of the key-signing key (KSK). You can set the
			value to <code>ACTIVE</code> or <code>INACTIVE</code>.</p>
    key_management_service_arn = "value"  # <p>The Amazon resource name (ARN) for a customer managed key in Key Management Service
				(KMS). The <code>KeyManagementServiceArn</code> must be unique for
			each key-signing key (KSK) in a single hosted zone. To see an example of
				<code>KeyManagementServiceArn</code> that grants the correct permissions for DNSSEC,
			scroll down to <b>Example</b>. </p>
         <p>You must configure the customer managed customer managed key as follows:</p>
         <dl>
            <dt>Status</dt>
            <dd>
               <p>Enabled</p>
            </dd>
            <dt>Key spec</dt>
            <dd>
               <p>ECC_NIST_P256</p>
            </dd>
            <dt>Key usage</dt>
            <dd>
               <p>Sign and verify</p>
            </dd>
            <dt>Key policy</dt>
            <dd>
               <p>The key policy must give permission for the following actions:</p>
               <ul>
                  <li>
                     <p>DescribeKey</p>
                  </li>
                  <li>
                     <p>GetPublicKey</p>
                  </li>
                  <li>
                     <p>Sign</p>
                  </li>
               </ul>
               <p>The key policy must also include the Amazon Route 53 service in the
						principal for your account. Specify the following:</p>
               <ul>
                  <li>
                     <p>
                        <code>"Service": "dnssec-route53.amazonaws.com"</code>
                     </p>
                  </li>
               </ul>
            </dd>
         </dl>
         <p>For more information about working with a customer managed key in KMS, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html">Key Management Service concepts</a>.</p>
    hosted_zone_id = "value"  # <p>The unique string (ID) used to identify a hosted zone.</p>
}

```

---


### Hosted_zone

HostedZone resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `delegation_set_id` | String |  | <p>If you want to associate a reusable delegation set with this hosted zone, the ID that
				Amazon Route 53 assigned to the reusable delegation set when you created it.
			For more information about reusable delegation sets, see <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_CreateReusableDelegationSet.html">CreateReusableDelegationSet</a>.</p>
         <p>If you are using a reusable delegation set to create a public hosted zone for a subdomain,
			make sure that the parent hosted zone doesn't use one or more of the same name servers.
			If you have overlapping nameservers, the operation will cause a
				<code>ConflictingDomainsExist</code> error.</p> |
| `hosted_zone_config` | String |  | <p>(Optional) A complex type that contains the following optional values:</p>
         <ul>
            <li>
               <p>For public and private hosted zones, an optional comment</p>
            </li>
            <li>
               <p>For private hosted zones, an optional <code>PrivateZone</code> element</p>
            </li>
         </ul>
         <p>If you don't specify a comment or the <code>PrivateZone</code> element, omit
				<code>HostedZoneConfig</code> and the other elements.</p> |
| `caller_reference` | String | ✅ | <p>A unique string that identifies the request and that allows failed
				<code>CreateHostedZone</code> requests to be retried without the risk of executing
			the operation twice. You must use a unique <code>CallerReference</code> string every
			time you submit a <code>CreateHostedZone</code> request. <code>CallerReference</code>
			can be any unique string, for example, a date/time stamp.</p> |
| `name` | String | ✅ | <p>The name of the domain. Specify a fully qualified domain name, for example,
				<i>www.example.com</i>. The trailing dot is optional; Amazon Route 53 assumes that the domain name is fully qualified. This means that
				Route 53 treats <i>www.example.com</i> (without a trailing
			dot) and <i>www.example.com.</i> (with a trailing dot) as
			identical.</p>
         <p>If you're creating a public hosted zone, this is the name you have registered with
			your DNS registrar. If your domain name is registered with a registrar other than
				Route 53, change the name servers for your domain to the set of
				<code>NameServers</code> that <code>CreateHostedZone</code> returns in
				<code>DelegationSet</code>.</p> |
| `vpc` | String |  | <p>(Private hosted zones only) A complex type that contains information about the Amazon
			VPC that you're associating with this hosted zone.</p>
         <p>You can specify only one Amazon VPC when you create a private hosted zone. If you are
			associating a VPC with a hosted zone with this request, the paramaters
				<code>VPCId</code> and <code>VPCRegion</code> are also required.</p>
         <p>To associate additional Amazon VPCs with the hosted zone, use <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_AssociateVPCWithHostedZone.html">AssociateVPCWithHostedZone</a> after you create a hosted zone.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `hosted_zone` | String | <p>A complex type that contains general information about the specified hosted
			zone.</p> |
| `delegation_set` | String | <p>A complex type that lists the Amazon Route 53 name servers for the specified hosted
			zone.</p> |
| `vp_cs` | Vec<String> | <p>A complex type that contains information about the VPCs that are associated with the
			specified hosted zone.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create hosted_zone
hosted_zone = provider.route_53.Hosted_zone {
    caller_reference = "value"  # <p>A unique string that identifies the request and that allows failed
				<code>CreateHostedZone</code> requests to be retried without the risk of executing
			the operation twice. You must use a unique <code>CallerReference</code> string every
			time you submit a <code>CreateHostedZone</code> request. <code>CallerReference</code>
			can be any unique string, for example, a date/time stamp.</p>
    name = "value"  # <p>The name of the domain. Specify a fully qualified domain name, for example,
				<i>www.example.com</i>. The trailing dot is optional; Amazon Route 53 assumes that the domain name is fully qualified. This means that
				Route 53 treats <i>www.example.com</i> (without a trailing
			dot) and <i>www.example.com.</i> (with a trailing dot) as
			identical.</p>
         <p>If you're creating a public hosted zone, this is the name you have registered with
			your DNS registrar. If your domain name is registered with a registrar other than
				Route 53, change the name servers for your domain to the set of
				<code>NameServers</code> that <code>CreateHostedZone</code> returns in
				<code>DelegationSet</code>.</p>
}

# Access hosted_zone outputs
hosted_zone_id = hosted_zone.id
hosted_zone_hosted_zone = hosted_zone.hosted_zone
hosted_zone_delegation_set = hosted_zone.delegation_set
hosted_zone_vp_cs = hosted_zone.vp_cs
```

---


### Account_limit

AccountLimit resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `limit` | String | <p>The current setting for the specified limit. For example, if you specified
				<code>MAX_HEALTH_CHECKS_BY_OWNER</code> for the value of <code>Type</code> in the
			request, the value of <code>Limit</code> is the maximum number of health checks that you
			can create using the current account.</p> |
| `count` | i64 | <p>The current number of entities that you have created of the specified type. For
			example, if you specified <code>MAX_HEALTH_CHECKS_BY_OWNER</code> for the value of
				<code>Type</code> in the request, the value of <code>Count</code> is the current
			number of health checks that you have created using the current account.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access account_limit outputs
account_limit_id = account_limit.id
account_limit_limit = account_limit.limit
account_limit_count = account_limit.count
```

---


### Traffic_policy_instance_count

TrafficPolicyInstanceCount resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `traffic_policy_instance_count` | i64 | <p>The number of traffic policy instances that are associated with the current Amazon Web Services account.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access traffic_policy_instance_count outputs
traffic_policy_instance_count_id = traffic_policy_instance_count.id
traffic_policy_instance_count_traffic_policy_instance_count = traffic_policy_instance_count.traffic_policy_instance_count
```

---


### Dnssec

DNSSEC resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `key_signing_keys` | Vec<String> | <p>The key-signing keys (KSKs) in your account.</p> |
| `status` | String | <p>A string representing the status of DNSSEC.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access dnssec outputs
dnssec_id = dnssec.id
dnssec_key_signing_keys = dnssec.key_signing_keys
dnssec_status = dnssec.status
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple traffic_policy_comment resources
traffic_policy_comment_0 = provider.route_53.Traffic_policy_comment {
    id = "value-0"
    version = "value-0"
    comment = "value-0"
}
traffic_policy_comment_1 = provider.route_53.Traffic_policy_comment {
    id = "value-1"
    version = "value-1"
    comment = "value-1"
}
traffic_policy_comment_2 = provider.route_53.Traffic_policy_comment {
    id = "value-2"
    version = "value-2"
    comment = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    traffic_policy_comment = provider.route_53.Traffic_policy_comment {
        id = "production-value"
        version = "production-value"
        comment = "production-value"
    }
```

---

## Related Documentation

- [AWS Route_53 Documentation](https://docs.aws.amazon.com/route_53/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
