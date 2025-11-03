# Route53_recovery_readiness Service



**Resources**: 10

---

## Overview

The route53_recovery_readiness service provides access to 10 resource types:

- [Cell](#cell) [CRUD]
- [Architecture_recommendations](#architecture_recommendations) [R]
- [Cell_readiness_summary](#cell_readiness_summary) [R]
- [Readiness_check_status](#readiness_check_status) [R]
- [Recovery_group_readiness_summary](#recovery_group_readiness_summary) [R]
- [Cross_account_authorization](#cross_account_authorization) [CD]
- [Recovery_group](#recovery_group) [CRUD]
- [Readiness_check_resource_status](#readiness_check_resource_status) [R]
- [Readiness_check](#readiness_check) [CRUD]
- [Resource_set](#resource_set) [CRUD]

---

## Resources


### Cell

Cell resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `cell_name` | String | ✅ | <p>The name of the cell to create.</p> |
| `cells` | Vec<String> |  | <p>A list of cell Amazon Resource Names (ARNs) contained within this cell, for use in nested cells. For example, Availability Zones within specific Amazon Web Services Regions.</p> |
| `tags` | HashMap<String, String> |  |  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `parent_readiness_scopes` | Vec<String> | <p>The readiness scope for the cell, which can be a cell Amazon Resource Name (ARN) or a recovery group ARN. This is a list but currently can have only one element.</p> |
| `cell_name` | String | <p>The name of the cell.</p> |
| `tags` | HashMap<String, String> | <p>Tags on the resources.</p> |
| `cells` | Vec<String> | <p>A list of cell ARNs.</p> |
| `cell_arn` | String | <p>The Amazon Resource Name (ARN) for the cell.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create cell
cell = provider.route53_recovery_readiness.Cell {
    cell_name = "value"  # <p>The name of the cell to create.</p>
}

# Access cell outputs
cell_id = cell.id
cell_parent_readiness_scopes = cell.parent_readiness_scopes
cell_cell_name = cell.cell_name
cell_tags = cell.tags
cell_cells = cell.cells
cell_cell_arn = cell.cell_arn
```

---


### Architecture_recommendations

ArchitectureRecommendations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `recommendations` | Vec<String> | <p>A list of the recommendations for the customer's application.</p> |
| `last_audit_timestamp` | String | <p>The time that a recovery group was last assessed for recommendations, in UTC ISO-8601 format.</p> |
| `next_token` | String | <p>The token that identifies which batch of results you want to see.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access architecture_recommendations outputs
architecture_recommendations_id = architecture_recommendations.id
architecture_recommendations_recommendations = architecture_recommendations.recommendations
architecture_recommendations_last_audit_timestamp = architecture_recommendations.last_audit_timestamp
architecture_recommendations_next_token = architecture_recommendations.next_token
```

---


### Cell_readiness_summary

CellReadinessSummary resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `readiness` | String | <p>The readiness at a cell level.</p> |
| `readiness_checks` | Vec<String> | <p>Summaries for the readiness checks that make up the cell.</p> |
| `next_token` | String | <p>The token that identifies which batch of results you want to see.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access cell_readiness_summary outputs
cell_readiness_summary_id = cell_readiness_summary.id
cell_readiness_summary_readiness = cell_readiness_summary.readiness
cell_readiness_summary_readiness_checks = cell_readiness_summary.readiness_checks
cell_readiness_summary_next_token = cell_readiness_summary.next_token
```

---


### Readiness_check_status

ReadinessCheckStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `resources` | Vec<String> | <p>Summary of the readiness of resources.</p> |
| `messages` | Vec<String> | <p>Top level messages for readiness check status</p> |
| `next_token` | String | <p>The token that identifies which batch of results you want to see.</p> |
| `readiness` | String | <p>The readiness at rule level.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access readiness_check_status outputs
readiness_check_status_id = readiness_check_status.id
readiness_check_status_resources = readiness_check_status.resources
readiness_check_status_messages = readiness_check_status.messages
readiness_check_status_next_token = readiness_check_status.next_token
readiness_check_status_readiness = readiness_check_status.readiness
```

---


### Recovery_group_readiness_summary

RecoveryGroupReadinessSummary resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `readiness` | String | <p>The readiness status at a recovery group level.</p> |
| `readiness_checks` | Vec<String> | <p>Summaries of the readiness checks for the recovery group.</p> |
| `next_token` | String | <p>The token that identifies which batch of results you want to see.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access recovery_group_readiness_summary outputs
recovery_group_readiness_summary_id = recovery_group_readiness_summary.id
recovery_group_readiness_summary_readiness = recovery_group_readiness_summary.readiness
recovery_group_readiness_summary_readiness_checks = recovery_group_readiness_summary.readiness_checks
recovery_group_readiness_summary_next_token = recovery_group_readiness_summary.next_token
```

---


### Cross_account_authorization

CrossAccountAuthorization resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `cross_account_authorization` | String | ✅ | <p>The cross-account authorization.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create cross_account_authorization
cross_account_authorization = provider.route53_recovery_readiness.Cross_account_authorization {
    cross_account_authorization = "value"  # <p>The cross-account authorization.</p>
}

```

---


### Recovery_group

RecoveryGroup resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `recovery_group_name` | String | ✅ | <p>The name of the recovery group to create.</p> |
| `cells` | Vec<String> |  | <p>A list of the cell Amazon Resource Names (ARNs) in the recovery group.</p> |
| `tags` | HashMap<String, String> |  |  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `tags` | HashMap<String, String> | <p>The tags associated with the recovery group.</p> |
| `cells` | Vec<String> | <p>A list of a cell's Amazon Resource Names (ARNs).</p> |
| `recovery_group_arn` | String | <p>The Amazon Resource Name (ARN) for the recovery group.</p> |
| `recovery_group_name` | String | <p>The name of the recovery group.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create recovery_group
recovery_group = provider.route53_recovery_readiness.Recovery_group {
    recovery_group_name = "value"  # <p>The name of the recovery group to create.</p>
}

# Access recovery_group outputs
recovery_group_id = recovery_group.id
recovery_group_tags = recovery_group.tags
recovery_group_cells = recovery_group.cells
recovery_group_recovery_group_arn = recovery_group.recovery_group_arn
recovery_group_recovery_group_name = recovery_group.recovery_group_name
```

---


### Readiness_check_resource_status

ReadinessCheckResourceStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token that identifies which batch of results you want to see.</p> |
| `readiness` | String | <p>The readiness at a rule level.</p> |
| `rules` | Vec<String> | <p>Details of the rule's results.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access readiness_check_resource_status outputs
readiness_check_resource_status_id = readiness_check_resource_status.id
readiness_check_resource_status_next_token = readiness_check_resource_status.next_token
readiness_check_resource_status_readiness = readiness_check_resource_status.readiness
readiness_check_resource_status_rules = readiness_check_resource_status.rules
```

---


### Readiness_check

ReadinessCheck resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | HashMap<String, String> |  |  |
| `resource_set_name` | String | ✅ | <p>The name of the resource set to check.</p> |
| `readiness_check_name` | String | ✅ | <p>The name of the readiness check to create.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `resource_set` | String | <p>Name of the resource set to be checked.</p> |
| `tags` | HashMap<String, String> |  |
| `readiness_check_arn` | String | <p>The Amazon Resource Name (ARN) associated with a readiness check.</p> |
| `readiness_check_name` | String | <p>Name of a readiness check.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create readiness_check
readiness_check = provider.route53_recovery_readiness.Readiness_check {
    resource_set_name = "value"  # <p>The name of the resource set to check.</p>
    readiness_check_name = "value"  # <p>The name of the readiness check to create.</p>
}

# Access readiness_check outputs
readiness_check_id = readiness_check.id
readiness_check_resource_set = readiness_check.resource_set
readiness_check_tags = readiness_check.tags
readiness_check_readiness_check_arn = readiness_check.readiness_check_arn
readiness_check_readiness_check_name = readiness_check.readiness_check_name
```

---


### Resource_set

ResourceSet resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resources` | Vec<String> | ✅ | <p>A list of resource objects in the resource set.</p> |
| `tags` | HashMap<String, String> |  | <p>A tag to associate with the parameters for a resource set.</p> |
| `resource_set_type` | String | ✅ | <p>The resource type of the resources in the resource set. Enter one of the following values for resource type:</p> <p>AWS::ApiGateway::Stage, AWS::ApiGatewayV2::Stage, AWS::AutoScaling::AutoScalingGroup, AWS::CloudWatch::Alarm, AWS::EC2::CustomerGateway, AWS::DynamoDB::Table, AWS::EC2::Volume, AWS::ElasticLoadBalancing::LoadBalancer, AWS::ElasticLoadBalancingV2::LoadBalancer, AWS::Lambda::Function, AWS::MSK::Cluster, AWS::RDS::DBCluster, AWS::Route53::HealthCheck, AWS::SQS::Queue, AWS::SNS::Topic, AWS::SNS::Subscription, AWS::EC2::VPC, AWS::EC2::VPNConnection, AWS::EC2::VPNGateway, AWS::Route53RecoveryReadiness::DNSTargetResource</p> |
| `resource_set_name` | String | ✅ | <p>The name of the resource set to create.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `tags` | HashMap<String, String> |  |
| `resource_set_arn` | String | <p>The Amazon Resource Name (ARN) for the resource set.</p> |
| `resource_set_type` | String | <p>The resource type of the resources in the resource set. Enter one of the following values for resource type:</p> <p>AWS::ApiGateway::Stage, AWS::ApiGatewayV2::Stage, AWS::AutoScaling::AutoScalingGroup, AWS::CloudWatch::Alarm, AWS::EC2::CustomerGateway, AWS::DynamoDB::Table, AWS::EC2::Volume, AWS::ElasticLoadBalancing::LoadBalancer, AWS::ElasticLoadBalancingV2::LoadBalancer, AWS::Lambda::Function, AWS::MSK::Cluster, AWS::RDS::DBCluster, AWS::Route53::HealthCheck, AWS::SQS::Queue, AWS::SNS::Topic, AWS::SNS::Subscription, AWS::EC2::VPC, AWS::EC2::VPNConnection, AWS::EC2::VPNGateway, AWS::Route53RecoveryReadiness::DNSTargetResource</p> |
| `resource_set_name` | String | <p>The name of the resource set.</p> |
| `resources` | Vec<String> | <p>A list of resource objects.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create resource_set
resource_set = provider.route53_recovery_readiness.Resource_set {
    resources = "value"  # <p>A list of resource objects in the resource set.</p>
    resource_set_type = "value"  # <p>The resource type of the resources in the resource set. Enter one of the following values for resource type:</p> <p>AWS::ApiGateway::Stage, AWS::ApiGatewayV2::Stage, AWS::AutoScaling::AutoScalingGroup, AWS::CloudWatch::Alarm, AWS::EC2::CustomerGateway, AWS::DynamoDB::Table, AWS::EC2::Volume, AWS::ElasticLoadBalancing::LoadBalancer, AWS::ElasticLoadBalancingV2::LoadBalancer, AWS::Lambda::Function, AWS::MSK::Cluster, AWS::RDS::DBCluster, AWS::Route53::HealthCheck, AWS::SQS::Queue, AWS::SNS::Topic, AWS::SNS::Subscription, AWS::EC2::VPC, AWS::EC2::VPNConnection, AWS::EC2::VPNGateway, AWS::Route53RecoveryReadiness::DNSTargetResource</p>
    resource_set_name = "value"  # <p>The name of the resource set to create.</p>
}

# Access resource_set outputs
resource_set_id = resource_set.id
resource_set_tags = resource_set.tags
resource_set_resource_set_arn = resource_set.resource_set_arn
resource_set_resource_set_type = resource_set.resource_set_type
resource_set_resource_set_name = resource_set.resource_set_name
resource_set_resources = resource_set.resources
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple cell resources
cell_0 = provider.route53_recovery_readiness.Cell {
    cell_name = "value-0"
}
cell_1 = provider.route53_recovery_readiness.Cell {
    cell_name = "value-1"
}
cell_2 = provider.route53_recovery_readiness.Cell {
    cell_name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    cell = provider.route53_recovery_readiness.Cell {
        cell_name = "production-value"
    }
```

---

## Related Documentation

- [AWS Route53_recovery_readiness Documentation](https://docs.aws.amazon.com/route53_recovery_readiness/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
