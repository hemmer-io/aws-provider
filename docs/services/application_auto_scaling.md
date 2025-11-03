# Application_auto_scaling Service



**Resources**: 7

---

## Overview

The application_auto_scaling service provides access to 7 resource types:

- [Scaling_policy](#scaling_policy) [CD]
- [Scalable_targets](#scalable_targets) [R]
- [Scaling_activities](#scaling_activities) [R]
- [Scheduled_action](#scheduled_action) [CD]
- [Predictive_scaling_forecast](#predictive_scaling_forecast) [R]
- [Scaling_policies](#scaling_policies) [R]
- [Scheduled_actions](#scheduled_actions) [R]

---

## Resources


### Scaling_policy

ScalingPolicy resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `policy_type` | String |  | <p>The scaling policy type. This parameter is required if you are creating a scaling
      policy.</p>
         <p>The following policy types are supported: </p>
         <p>
            <code>TargetTrackingScaling</code>—Not supported for Amazon EMR.</p>
         <p>
            <code>StepScaling</code>—Not supported for DynamoDB, Amazon Comprehend, Lambda, Amazon Keyspaces, Amazon MSK, Amazon ElastiCache, or
      Neptune.</p>
         <p>
            <code>PredictiveScaling</code>—Only supported for Amazon ECS.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/application/userguide/application-auto-scaling-target-tracking.html">Target
        tracking scaling policies</a>, <a href="https://docs.aws.amazon.com/autoscaling/application/userguide/application-auto-scaling-step-scaling-policies.html">Step scaling policies</a>, and <a href="https://docs.aws.amazon.com/autoscaling/application/userguide/aas-create-predictive-scaling-policy.html">Predictive scaling policies</a> 
        in the <i>Application Auto Scaling User Guide</i>.</p> |
| `predictive_scaling_policy_configuration` | String |  | <p>
         The configuration of the predictive scaling policy.
      </p> |
| `resource_id` | String | ✅ | <p>The identifier of the resource associated with the scaling policy.
      This string consists of the resource type and unique identifier.</p>
         <ul>
            <li>
               <p>ECS service - The resource type is <code>service</code> and the unique identifier is the cluster name  
               and service name. Example: <code>service/my-cluster/my-service</code>.</p>
            </li>
            <li>
               <p>Spot Fleet - The resource type is <code>spot-fleet-request</code> and the unique identifier is the 
               Spot Fleet request ID. Example: <code>spot-fleet-request/sfr-73fbd2ce-aa30-494c-8788-1cee4EXAMPLE</code>.</p>
            </li>
            <li>
               <p>EMR cluster - The resource type is <code>instancegroup</code> and the unique identifier is the cluster ID and instance group ID.
               Example: <code>instancegroup/j-2EEZNYKUA1NTV/ig-1791Y4E1L8YI0</code>.</p>
            </li>
            <li>
               <p>AppStream 2.0 fleet - The resource type is <code>fleet</code> and the unique identifier is the fleet name.
               Example: <code>fleet/sample-fleet</code>.</p>
            </li>
            <li>
               <p>DynamoDB table - The resource type is <code>table</code> and the unique identifier is the table name. 
               Example: <code>table/my-table</code>.</p>
            </li>
            <li>
               <p>DynamoDB global secondary index - The resource type is <code>index</code> and the unique identifier is the index name. 
               Example: <code>table/my-table/index/my-table-index</code>.</p>
            </li>
            <li>
               <p>Aurora DB cluster - The resource type is <code>cluster</code> and the unique identifier is the cluster name.
               Example: <code>cluster:my-db-cluster</code>.</p>
            </li>
            <li>
               <p>SageMaker endpoint variant - The resource type is <code>variant</code> and the unique identifier is the resource ID.
               Example: <code>endpoint/my-end-point/variant/KMeansClustering</code>.</p>
            </li>
            <li>
               <p>Custom resources are not supported with a resource type. This parameter must specify the <code>OutputValue</code> from the CloudFormation template stack used to access the resources. The unique identifier is defined by the service provider. More information
               is available in our <a href="https://github.com/aws/aws-auto-scaling-custom-resource">GitHub
                  repository</a>.</p>
            </li>
            <li>
               <p>Amazon Comprehend document classification endpoint - The resource type and unique identifier are specified using the endpoint ARN. Example: <code>arn:aws:comprehend:us-west-2:123456789012:document-classifier-endpoint/EXAMPLE</code>.</p>
            </li>
            <li>
               <p>Amazon Comprehend entity recognizer endpoint - The resource type and unique identifier are specified using the endpoint ARN. Example: <code>arn:aws:comprehend:us-west-2:123456789012:entity-recognizer-endpoint/EXAMPLE</code>.</p>
            </li>
            <li>
               <p>Lambda provisioned concurrency - The resource type is <code>function</code> and the unique identifier is the function name with a function version or alias name suffix that is not <code>$LATEST</code>. 
               Example: <code>function:my-function:prod</code> or <code>function:my-function:1</code>.</p>
            </li>
            <li>
               <p>Amazon Keyspaces table - The resource type is <code>table</code> and the unique identifier is the table name. 
               Example: <code>keyspace/mykeyspace/table/mytable</code>.</p>
            </li>
            <li>
               <p>Amazon MSK cluster - The resource type and unique identifier are specified using the cluster ARN. 
               Example: <code>arn:aws:kafka:us-east-1:123456789012:cluster/demo-cluster-1/6357e0b2-0e6a-4b86-a0b4-70df934c2e31-5</code>.</p>
            </li>
            <li>
               <p>Amazon ElastiCache replication group - The resource type is <code>replication-group</code> and the unique identifier is the replication group name.
               Example: <code>replication-group/mycluster</code>.</p>
            </li>
            <li>
               <p>Amazon ElastiCache cache cluster - The resource type is <code>cache-cluster</code> and the unique identifier is the cache cluster name.
               Example: <code>cache-cluster/mycluster</code>.</p>
            </li>
            <li>
               <p>Neptune cluster - The resource type is <code>cluster</code> and the unique identifier is the cluster name. Example: <code>cluster:mycluster</code>.</p>
            </li>
            <li>
               <p>SageMaker serverless endpoint - The resource type is <code>variant</code> and the unique identifier is the resource ID.
               Example: <code>endpoint/my-end-point/variant/KMeansClustering</code>.</p>
            </li>
            <li>
               <p>SageMaker inference component - The resource type is <code>inference-component</code> and the unique identifier is the resource ID.
               Example: <code>inference-component/my-inference-component</code>.</p>
            </li>
            <li>
               <p>Pool of WorkSpaces - The resource type is <code>workspacespool</code> and the unique identifier is the pool ID. 
               Example: <code>workspacespool/wspool-123456</code>.</p>
            </li>
         </ul> |
| `policy_name` | String | ✅ | <p>The name of the scaling policy.</p>
         <p>You cannot change the name of a scaling policy, but you can delete the original scaling
         policy and create a new scaling policy with the same settings and a different name.</p> |
| `scalable_dimension` | String | ✅ | <p>The scalable dimension. This string consists of the service namespace, resource type, and scaling property.</p>
         <ul>
            <li>
               <p>
                  <code>ecs:service:DesiredCount</code> - The task count of an ECS service.</p>
            </li>
            <li>
               <p>
                  <code>elasticmapreduce:instancegroup:InstanceCount</code> - The instance count of an EMR Instance Group.</p>
            </li>
            <li>
               <p>
                  <code>ec2:spot-fleet-request:TargetCapacity</code> - The target capacity of a Spot Fleet.</p>
            </li>
            <li>
               <p>
                  <code>appstream:fleet:DesiredCapacity</code> - The capacity of an AppStream 2.0 fleet.</p>
            </li>
            <li>
               <p>
                  <code>dynamodb:table:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB table.</p>
            </li>
            <li>
               <p>
                  <code>dynamodb:table:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB table.</p>
            </li>
            <li>
               <p>
                  <code>dynamodb:index:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB global secondary index.</p>
            </li>
            <li>
               <p>
                  <code>dynamodb:index:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB global secondary index.</p>
            </li>
            <li>
               <p>
                  <code>rds:cluster:ReadReplicaCount</code> - The count of Aurora Replicas in an Aurora DB cluster. Available for Aurora MySQL-compatible edition and Aurora PostgreSQL-compatible edition.</p>
            </li>
            <li>
               <p>
                  <code>sagemaker:variant:DesiredInstanceCount</code> - The number of EC2 instances for a SageMaker model endpoint variant.</p>
            </li>
            <li>
               <p>
                  <code>custom-resource:ResourceType:Property</code> - The scalable dimension for a custom resource provided by your own application or service.</p>
            </li>
            <li>
               <p>
                  <code>comprehend:document-classifier-endpoint:DesiredInferenceUnits</code> - The number of inference units for an Amazon Comprehend document classification endpoint.</p>
            </li>
            <li>
               <p>
                  <code>comprehend:entity-recognizer-endpoint:DesiredInferenceUnits</code> - The number of inference units for an Amazon Comprehend entity recognizer endpoint.</p>
            </li>
            <li>
               <p>
                  <code>lambda:function:ProvisionedConcurrency</code> - The provisioned concurrency for a Lambda function.</p>
            </li>
            <li>
               <p>
                  <code>cassandra:table:ReadCapacityUnits</code> - The provisioned read capacity for an Amazon Keyspaces table.</p>
            </li>
            <li>
               <p>
                  <code>cassandra:table:WriteCapacityUnits</code> - The provisioned write capacity for an Amazon Keyspaces table.</p>
            </li>
            <li>
               <p>
                  <code>kafka:broker-storage:VolumeSize</code> - The provisioned volume size (in GiB) for brokers in an Amazon MSK cluster.</p>
            </li>
            <li>
               <p>
                  <code>elasticache:cache-cluster:Nodes</code> - The number of nodes for an Amazon ElastiCache cache cluster.</p>
            </li>
            <li>
               <p>
                  <code>elasticache:replication-group:NodeGroups</code> - The number of node groups for an Amazon ElastiCache replication group.</p>
            </li>
            <li>
               <p>
                  <code>elasticache:replication-group:Replicas</code> - The number of replicas per node group for an Amazon ElastiCache replication group.</p>
            </li>
            <li>
               <p>
                  <code>neptune:cluster:ReadReplicaCount</code> - The count of read replicas in an Amazon Neptune DB cluster.</p>
            </li>
            <li>
               <p>
                  <code>sagemaker:variant:DesiredProvisionedConcurrency</code> - The provisioned concurrency for a SageMaker serverless endpoint.</p>
            </li>
            <li>
               <p>
                  <code>sagemaker:inference-component:DesiredCopyCount</code> - The number of copies across an endpoint for a SageMaker inference component.</p>
            </li>
            <li>
               <p>
                  <code>workspaces:workspacespool:DesiredUserSessions</code> - The number of user sessions for the WorkSpaces in the pool.</p>
            </li>
         </ul> |
| `service_namespace` | String | ✅ | <p>The namespace of the Amazon Web Services service that provides the resource. For a resource provided
         by your own application or service, use <code>custom-resource</code> instead.</p> |
| `step_scaling_policy_configuration` | String |  | <p>A step scaling policy.</p>
         <p>This parameter is required if you are creating a policy and the policy type is
            <code>StepScaling</code>.</p> |
| `target_tracking_scaling_policy_configuration` | String |  | <p>A target tracking scaling policy. Includes support for predefined or customized
         metrics.</p>
         <p>This parameter is required if you are creating a policy and the policy type is
            <code>TargetTrackingScaling</code>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create scaling_policy
scaling_policy = provider.application_auto_scaling.Scaling_policy {
    resource_id = "value"  # <p>The identifier of the resource associated with the scaling policy.
      This string consists of the resource type and unique identifier.</p>
         <ul>
            <li>
               <p>ECS service - The resource type is <code>service</code> and the unique identifier is the cluster name  
               and service name. Example: <code>service/my-cluster/my-service</code>.</p>
            </li>
            <li>
               <p>Spot Fleet - The resource type is <code>spot-fleet-request</code> and the unique identifier is the 
               Spot Fleet request ID. Example: <code>spot-fleet-request/sfr-73fbd2ce-aa30-494c-8788-1cee4EXAMPLE</code>.</p>
            </li>
            <li>
               <p>EMR cluster - The resource type is <code>instancegroup</code> and the unique identifier is the cluster ID and instance group ID.
               Example: <code>instancegroup/j-2EEZNYKUA1NTV/ig-1791Y4E1L8YI0</code>.</p>
            </li>
            <li>
               <p>AppStream 2.0 fleet - The resource type is <code>fleet</code> and the unique identifier is the fleet name.
               Example: <code>fleet/sample-fleet</code>.</p>
            </li>
            <li>
               <p>DynamoDB table - The resource type is <code>table</code> and the unique identifier is the table name. 
               Example: <code>table/my-table</code>.</p>
            </li>
            <li>
               <p>DynamoDB global secondary index - The resource type is <code>index</code> and the unique identifier is the index name. 
               Example: <code>table/my-table/index/my-table-index</code>.</p>
            </li>
            <li>
               <p>Aurora DB cluster - The resource type is <code>cluster</code> and the unique identifier is the cluster name.
               Example: <code>cluster:my-db-cluster</code>.</p>
            </li>
            <li>
               <p>SageMaker endpoint variant - The resource type is <code>variant</code> and the unique identifier is the resource ID.
               Example: <code>endpoint/my-end-point/variant/KMeansClustering</code>.</p>
            </li>
            <li>
               <p>Custom resources are not supported with a resource type. This parameter must specify the <code>OutputValue</code> from the CloudFormation template stack used to access the resources. The unique identifier is defined by the service provider. More information
               is available in our <a href="https://github.com/aws/aws-auto-scaling-custom-resource">GitHub
                  repository</a>.</p>
            </li>
            <li>
               <p>Amazon Comprehend document classification endpoint - The resource type and unique identifier are specified using the endpoint ARN. Example: <code>arn:aws:comprehend:us-west-2:123456789012:document-classifier-endpoint/EXAMPLE</code>.</p>
            </li>
            <li>
               <p>Amazon Comprehend entity recognizer endpoint - The resource type and unique identifier are specified using the endpoint ARN. Example: <code>arn:aws:comprehend:us-west-2:123456789012:entity-recognizer-endpoint/EXAMPLE</code>.</p>
            </li>
            <li>
               <p>Lambda provisioned concurrency - The resource type is <code>function</code> and the unique identifier is the function name with a function version or alias name suffix that is not <code>$LATEST</code>. 
               Example: <code>function:my-function:prod</code> or <code>function:my-function:1</code>.</p>
            </li>
            <li>
               <p>Amazon Keyspaces table - The resource type is <code>table</code> and the unique identifier is the table name. 
               Example: <code>keyspace/mykeyspace/table/mytable</code>.</p>
            </li>
            <li>
               <p>Amazon MSK cluster - The resource type and unique identifier are specified using the cluster ARN. 
               Example: <code>arn:aws:kafka:us-east-1:123456789012:cluster/demo-cluster-1/6357e0b2-0e6a-4b86-a0b4-70df934c2e31-5</code>.</p>
            </li>
            <li>
               <p>Amazon ElastiCache replication group - The resource type is <code>replication-group</code> and the unique identifier is the replication group name.
               Example: <code>replication-group/mycluster</code>.</p>
            </li>
            <li>
               <p>Amazon ElastiCache cache cluster - The resource type is <code>cache-cluster</code> and the unique identifier is the cache cluster name.
               Example: <code>cache-cluster/mycluster</code>.</p>
            </li>
            <li>
               <p>Neptune cluster - The resource type is <code>cluster</code> and the unique identifier is the cluster name. Example: <code>cluster:mycluster</code>.</p>
            </li>
            <li>
               <p>SageMaker serverless endpoint - The resource type is <code>variant</code> and the unique identifier is the resource ID.
               Example: <code>endpoint/my-end-point/variant/KMeansClustering</code>.</p>
            </li>
            <li>
               <p>SageMaker inference component - The resource type is <code>inference-component</code> and the unique identifier is the resource ID.
               Example: <code>inference-component/my-inference-component</code>.</p>
            </li>
            <li>
               <p>Pool of WorkSpaces - The resource type is <code>workspacespool</code> and the unique identifier is the pool ID. 
               Example: <code>workspacespool/wspool-123456</code>.</p>
            </li>
         </ul>
    policy_name = "value"  # <p>The name of the scaling policy.</p>
         <p>You cannot change the name of a scaling policy, but you can delete the original scaling
         policy and create a new scaling policy with the same settings and a different name.</p>
    scalable_dimension = "value"  # <p>The scalable dimension. This string consists of the service namespace, resource type, and scaling property.</p>
         <ul>
            <li>
               <p>
                  <code>ecs:service:DesiredCount</code> - The task count of an ECS service.</p>
            </li>
            <li>
               <p>
                  <code>elasticmapreduce:instancegroup:InstanceCount</code> - The instance count of an EMR Instance Group.</p>
            </li>
            <li>
               <p>
                  <code>ec2:spot-fleet-request:TargetCapacity</code> - The target capacity of a Spot Fleet.</p>
            </li>
            <li>
               <p>
                  <code>appstream:fleet:DesiredCapacity</code> - The capacity of an AppStream 2.0 fleet.</p>
            </li>
            <li>
               <p>
                  <code>dynamodb:table:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB table.</p>
            </li>
            <li>
               <p>
                  <code>dynamodb:table:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB table.</p>
            </li>
            <li>
               <p>
                  <code>dynamodb:index:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB global secondary index.</p>
            </li>
            <li>
               <p>
                  <code>dynamodb:index:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB global secondary index.</p>
            </li>
            <li>
               <p>
                  <code>rds:cluster:ReadReplicaCount</code> - The count of Aurora Replicas in an Aurora DB cluster. Available for Aurora MySQL-compatible edition and Aurora PostgreSQL-compatible edition.</p>
            </li>
            <li>
               <p>
                  <code>sagemaker:variant:DesiredInstanceCount</code> - The number of EC2 instances for a SageMaker model endpoint variant.</p>
            </li>
            <li>
               <p>
                  <code>custom-resource:ResourceType:Property</code> - The scalable dimension for a custom resource provided by your own application or service.</p>
            </li>
            <li>
               <p>
                  <code>comprehend:document-classifier-endpoint:DesiredInferenceUnits</code> - The number of inference units for an Amazon Comprehend document classification endpoint.</p>
            </li>
            <li>
               <p>
                  <code>comprehend:entity-recognizer-endpoint:DesiredInferenceUnits</code> - The number of inference units for an Amazon Comprehend entity recognizer endpoint.</p>
            </li>
            <li>
               <p>
                  <code>lambda:function:ProvisionedConcurrency</code> - The provisioned concurrency for a Lambda function.</p>
            </li>
            <li>
               <p>
                  <code>cassandra:table:ReadCapacityUnits</code> - The provisioned read capacity for an Amazon Keyspaces table.</p>
            </li>
            <li>
               <p>
                  <code>cassandra:table:WriteCapacityUnits</code> - The provisioned write capacity for an Amazon Keyspaces table.</p>
            </li>
            <li>
               <p>
                  <code>kafka:broker-storage:VolumeSize</code> - The provisioned volume size (in GiB) for brokers in an Amazon MSK cluster.</p>
            </li>
            <li>
               <p>
                  <code>elasticache:cache-cluster:Nodes</code> - The number of nodes for an Amazon ElastiCache cache cluster.</p>
            </li>
            <li>
               <p>
                  <code>elasticache:replication-group:NodeGroups</code> - The number of node groups for an Amazon ElastiCache replication group.</p>
            </li>
            <li>
               <p>
                  <code>elasticache:replication-group:Replicas</code> - The number of replicas per node group for an Amazon ElastiCache replication group.</p>
            </li>
            <li>
               <p>
                  <code>neptune:cluster:ReadReplicaCount</code> - The count of read replicas in an Amazon Neptune DB cluster.</p>
            </li>
            <li>
               <p>
                  <code>sagemaker:variant:DesiredProvisionedConcurrency</code> - The provisioned concurrency for a SageMaker serverless endpoint.</p>
            </li>
            <li>
               <p>
                  <code>sagemaker:inference-component:DesiredCopyCount</code> - The number of copies across an endpoint for a SageMaker inference component.</p>
            </li>
            <li>
               <p>
                  <code>workspaces:workspacespool:DesiredUserSessions</code> - The number of user sessions for the WorkSpaces in the pool.</p>
            </li>
         </ul>
    service_namespace = "value"  # <p>The namespace of the Amazon Web Services service that provides the resource. For a resource provided
         by your own application or service, use <code>custom-resource</code> instead.</p>
}

```

---


### Scalable_targets

ScalableTargets resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token required to get the next set of results. This value is <code>null</code> if
         there are no more results to return.</p> |
| `scalable_targets` | Vec<String> | <p>The scalable targets that match the request parameters.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access scalable_targets outputs
scalable_targets_id = scalable_targets.id
scalable_targets_next_token = scalable_targets.next_token
scalable_targets_scalable_targets = scalable_targets.scalable_targets
```

---


### Scaling_activities

ScalingActivities resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `scaling_activities` | Vec<String> | <p>A list of scaling activity objects.</p> |
| `next_token` | String | <p>The token required to get the next set of results. This value is <code>null</code> if
         there are no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access scaling_activities outputs
scaling_activities_id = scaling_activities.id
scaling_activities_scaling_activities = scaling_activities.scaling_activities
scaling_activities_next_token = scaling_activities.next_token
```

---


### Scheduled_action

ScheduledAction resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `start_time` | String |  | <p>The date and time for this scheduled action to start, in UTC.</p> |
| `end_time` | String |  | <p>The date and time for the recurring schedule to end, in UTC.</p> |
| `resource_id` | String | ✅ | <p>The identifier of the resource associated with the scheduled action.
      This string consists of the resource type and unique identifier.</p>
         <ul>
            <li>
               <p>ECS service - The resource type is <code>service</code> and the unique identifier is the cluster name  
               and service name. Example: <code>service/my-cluster/my-service</code>.</p>
            </li>
            <li>
               <p>Spot Fleet - The resource type is <code>spot-fleet-request</code> and the unique identifier is the 
               Spot Fleet request ID. Example: <code>spot-fleet-request/sfr-73fbd2ce-aa30-494c-8788-1cee4EXAMPLE</code>.</p>
            </li>
            <li>
               <p>EMR cluster - The resource type is <code>instancegroup</code> and the unique identifier is the cluster ID and instance group ID.
               Example: <code>instancegroup/j-2EEZNYKUA1NTV/ig-1791Y4E1L8YI0</code>.</p>
            </li>
            <li>
               <p>AppStream 2.0 fleet - The resource type is <code>fleet</code> and the unique identifier is the fleet name.
               Example: <code>fleet/sample-fleet</code>.</p>
            </li>
            <li>
               <p>DynamoDB table - The resource type is <code>table</code> and the unique identifier is the table name. 
               Example: <code>table/my-table</code>.</p>
            </li>
            <li>
               <p>DynamoDB global secondary index - The resource type is <code>index</code> and the unique identifier is the index name. 
               Example: <code>table/my-table/index/my-table-index</code>.</p>
            </li>
            <li>
               <p>Aurora DB cluster - The resource type is <code>cluster</code> and the unique identifier is the cluster name.
               Example: <code>cluster:my-db-cluster</code>.</p>
            </li>
            <li>
               <p>SageMaker endpoint variant - The resource type is <code>variant</code> and the unique identifier is the resource ID.
               Example: <code>endpoint/my-end-point/variant/KMeansClustering</code>.</p>
            </li>
            <li>
               <p>Custom resources are not supported with a resource type. This parameter must specify the <code>OutputValue</code> from the CloudFormation template stack used to access the resources. The unique identifier is defined by the service provider. More information
               is available in our <a href="https://github.com/aws/aws-auto-scaling-custom-resource">GitHub
                  repository</a>.</p>
            </li>
            <li>
               <p>Amazon Comprehend document classification endpoint - The resource type and unique identifier are specified using the endpoint ARN. Example: <code>arn:aws:comprehend:us-west-2:123456789012:document-classifier-endpoint/EXAMPLE</code>.</p>
            </li>
            <li>
               <p>Amazon Comprehend entity recognizer endpoint - The resource type and unique identifier are specified using the endpoint ARN. Example: <code>arn:aws:comprehend:us-west-2:123456789012:entity-recognizer-endpoint/EXAMPLE</code>.</p>
            </li>
            <li>
               <p>Lambda provisioned concurrency - The resource type is <code>function</code> and the unique identifier is the function name with a function version or alias name suffix that is not <code>$LATEST</code>. 
               Example: <code>function:my-function:prod</code> or <code>function:my-function:1</code>.</p>
            </li>
            <li>
               <p>Amazon Keyspaces table - The resource type is <code>table</code> and the unique identifier is the table name. 
               Example: <code>keyspace/mykeyspace/table/mytable</code>.</p>
            </li>
            <li>
               <p>Amazon MSK cluster - The resource type and unique identifier are specified using the cluster ARN. 
               Example: <code>arn:aws:kafka:us-east-1:123456789012:cluster/demo-cluster-1/6357e0b2-0e6a-4b86-a0b4-70df934c2e31-5</code>.</p>
            </li>
            <li>
               <p>Amazon ElastiCache replication group - The resource type is <code>replication-group</code> and the unique identifier is the replication group name.
               Example: <code>replication-group/mycluster</code>.</p>
            </li>
            <li>
               <p>Amazon ElastiCache cache cluster - The resource type is <code>cache-cluster</code> and the unique identifier is the cache cluster name.
               Example: <code>cache-cluster/mycluster</code>.</p>
            </li>
            <li>
               <p>Neptune cluster - The resource type is <code>cluster</code> and the unique identifier is the cluster name. Example: <code>cluster:mycluster</code>.</p>
            </li>
            <li>
               <p>SageMaker serverless endpoint - The resource type is <code>variant</code> and the unique identifier is the resource ID.
               Example: <code>endpoint/my-end-point/variant/KMeansClustering</code>.</p>
            </li>
            <li>
               <p>SageMaker inference component - The resource type is <code>inference-component</code> and the unique identifier is the resource ID.
               Example: <code>inference-component/my-inference-component</code>.</p>
            </li>
            <li>
               <p>Pool of WorkSpaces - The resource type is <code>workspacespool</code> and the unique identifier is the pool ID. 
               Example: <code>workspacespool/wspool-123456</code>.</p>
            </li>
         </ul> |
| `scalable_target_action` | String |  | <p>The new minimum and maximum capacity. You can set both values or just one. At the
         scheduled time, if the current capacity is below the minimum capacity, Application Auto Scaling scales out
         to the minimum capacity. If the current capacity is above the maximum capacity, Application Auto Scaling
         scales in to the maximum capacity.</p> |
| `timezone` | String |  | <p>Specifies the time zone used when setting a scheduled action by using an at or cron
         expression. If a time zone is not provided, UTC is used by default.</p>
         <p>Valid values are the canonical names of the IANA time zones supported by Joda-Time (such
         as <code>Etc/GMT+9</code> or <code>Pacific/Tahiti</code>). For more information, see <a href="https://www.joda.org/joda-time/timezones.html">https://www.joda.org/joda-time/timezones.html</a>.</p> |
| `scalable_dimension` | String | ✅ | <p>The scalable dimension. This string consists of the service namespace, resource type, and scaling property.</p>
         <ul>
            <li>
               <p>
                  <code>ecs:service:DesiredCount</code> - The task count of an ECS service.</p>
            </li>
            <li>
               <p>
                  <code>elasticmapreduce:instancegroup:InstanceCount</code> - The instance count of an EMR Instance Group.</p>
            </li>
            <li>
               <p>
                  <code>ec2:spot-fleet-request:TargetCapacity</code> - The target capacity of a Spot Fleet.</p>
            </li>
            <li>
               <p>
                  <code>appstream:fleet:DesiredCapacity</code> - The capacity of an AppStream 2.0 fleet.</p>
            </li>
            <li>
               <p>
                  <code>dynamodb:table:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB table.</p>
            </li>
            <li>
               <p>
                  <code>dynamodb:table:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB table.</p>
            </li>
            <li>
               <p>
                  <code>dynamodb:index:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB global secondary index.</p>
            </li>
            <li>
               <p>
                  <code>dynamodb:index:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB global secondary index.</p>
            </li>
            <li>
               <p>
                  <code>rds:cluster:ReadReplicaCount</code> - The count of Aurora Replicas in an Aurora DB cluster. Available for Aurora MySQL-compatible edition and Aurora PostgreSQL-compatible edition.</p>
            </li>
            <li>
               <p>
                  <code>sagemaker:variant:DesiredInstanceCount</code> - The number of EC2 instances for a SageMaker model endpoint variant.</p>
            </li>
            <li>
               <p>
                  <code>custom-resource:ResourceType:Property</code> - The scalable dimension for a custom resource provided by your own application or service.</p>
            </li>
            <li>
               <p>
                  <code>comprehend:document-classifier-endpoint:DesiredInferenceUnits</code> - The number of inference units for an Amazon Comprehend document classification endpoint.</p>
            </li>
            <li>
               <p>
                  <code>comprehend:entity-recognizer-endpoint:DesiredInferenceUnits</code> - The number of inference units for an Amazon Comprehend entity recognizer endpoint.</p>
            </li>
            <li>
               <p>
                  <code>lambda:function:ProvisionedConcurrency</code> - The provisioned concurrency for a Lambda function.</p>
            </li>
            <li>
               <p>
                  <code>cassandra:table:ReadCapacityUnits</code> - The provisioned read capacity for an Amazon Keyspaces table.</p>
            </li>
            <li>
               <p>
                  <code>cassandra:table:WriteCapacityUnits</code> - The provisioned write capacity for an Amazon Keyspaces table.</p>
            </li>
            <li>
               <p>
                  <code>kafka:broker-storage:VolumeSize</code> - The provisioned volume size (in GiB) for brokers in an Amazon MSK cluster.</p>
            </li>
            <li>
               <p>
                  <code>elasticache:cache-cluster:Nodes</code> - The number of nodes for an Amazon ElastiCache cache cluster.</p>
            </li>
            <li>
               <p>
                  <code>elasticache:replication-group:NodeGroups</code> - The number of node groups for an Amazon ElastiCache replication group.</p>
            </li>
            <li>
               <p>
                  <code>elasticache:replication-group:Replicas</code> - The number of replicas per node group for an Amazon ElastiCache replication group.</p>
            </li>
            <li>
               <p>
                  <code>neptune:cluster:ReadReplicaCount</code> - The count of read replicas in an Amazon Neptune DB cluster.</p>
            </li>
            <li>
               <p>
                  <code>sagemaker:variant:DesiredProvisionedConcurrency</code> - The provisioned concurrency for a SageMaker serverless endpoint.</p>
            </li>
            <li>
               <p>
                  <code>sagemaker:inference-component:DesiredCopyCount</code> - The number of copies across an endpoint for a SageMaker inference component.</p>
            </li>
            <li>
               <p>
                  <code>workspaces:workspacespool:DesiredUserSessions</code> - The number of user sessions for the WorkSpaces in the pool.</p>
            </li>
         </ul> |
| `scheduled_action_name` | String | ✅ | <p>The name of the scheduled action. This name must be unique among all other scheduled
         actions on the specified scalable target. </p> |
| `service_namespace` | String | ✅ | <p>The namespace of the Amazon Web Services service that provides the resource. For a resource provided
         by your own application or service, use <code>custom-resource</code> instead.</p> |
| `schedule` | String |  | <p>The schedule for this action. The following formats are supported:</p>
         <ul>
            <li>
               <p>At expressions - "<code>at(<i>yyyy</i>-<i>mm</i>-<i>dd</i>T<i>hh</i>:<i>mm</i>:<i>ss</i>)</code>"</p>
            </li>
            <li>
               <p>Rate expressions - "<code>rate(<i>value</i>
                     <i>unit</i>)</code>"</p>
            </li>
            <li>
               <p>Cron expressions - "<code>cron(<i>fields</i>)</code>"</p>
            </li>
         </ul>
         <p>At expressions are useful for one-time schedules. Cron expressions are useful for 
         scheduled actions that run periodically at a specified date and time, and rate expressions 
         are useful for scheduled actions that run at a regular interval.</p>
         <p>At and cron expressions use Universal Coordinated Time (UTC) by
         default.</p>
         <p>The cron format consists of six fields separated by white spaces: [Minutes] [Hours] [Day_of_Month] [Month] [Day_of_Week] [Year].</p>
         <p>For rate expressions, <i>value</i> is a positive integer and <i>unit</i> is 
         <code>minute</code> | <code>minutes</code> | <code>hour</code> | <code>hours</code> | <code>day</code> | <code>days</code>.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/application/userguide/scheduled-scaling-using-cron-expressions.html">Schedule recurring scaling actions using cron expressions</a> in the <i>Application Auto Scaling User Guide</i>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create scheduled_action
scheduled_action = provider.application_auto_scaling.Scheduled_action {
    resource_id = "value"  # <p>The identifier of the resource associated with the scheduled action.
      This string consists of the resource type and unique identifier.</p>
         <ul>
            <li>
               <p>ECS service - The resource type is <code>service</code> and the unique identifier is the cluster name  
               and service name. Example: <code>service/my-cluster/my-service</code>.</p>
            </li>
            <li>
               <p>Spot Fleet - The resource type is <code>spot-fleet-request</code> and the unique identifier is the 
               Spot Fleet request ID. Example: <code>spot-fleet-request/sfr-73fbd2ce-aa30-494c-8788-1cee4EXAMPLE</code>.</p>
            </li>
            <li>
               <p>EMR cluster - The resource type is <code>instancegroup</code> and the unique identifier is the cluster ID and instance group ID.
               Example: <code>instancegroup/j-2EEZNYKUA1NTV/ig-1791Y4E1L8YI0</code>.</p>
            </li>
            <li>
               <p>AppStream 2.0 fleet - The resource type is <code>fleet</code> and the unique identifier is the fleet name.
               Example: <code>fleet/sample-fleet</code>.</p>
            </li>
            <li>
               <p>DynamoDB table - The resource type is <code>table</code> and the unique identifier is the table name. 
               Example: <code>table/my-table</code>.</p>
            </li>
            <li>
               <p>DynamoDB global secondary index - The resource type is <code>index</code> and the unique identifier is the index name. 
               Example: <code>table/my-table/index/my-table-index</code>.</p>
            </li>
            <li>
               <p>Aurora DB cluster - The resource type is <code>cluster</code> and the unique identifier is the cluster name.
               Example: <code>cluster:my-db-cluster</code>.</p>
            </li>
            <li>
               <p>SageMaker endpoint variant - The resource type is <code>variant</code> and the unique identifier is the resource ID.
               Example: <code>endpoint/my-end-point/variant/KMeansClustering</code>.</p>
            </li>
            <li>
               <p>Custom resources are not supported with a resource type. This parameter must specify the <code>OutputValue</code> from the CloudFormation template stack used to access the resources. The unique identifier is defined by the service provider. More information
               is available in our <a href="https://github.com/aws/aws-auto-scaling-custom-resource">GitHub
                  repository</a>.</p>
            </li>
            <li>
               <p>Amazon Comprehend document classification endpoint - The resource type and unique identifier are specified using the endpoint ARN. Example: <code>arn:aws:comprehend:us-west-2:123456789012:document-classifier-endpoint/EXAMPLE</code>.</p>
            </li>
            <li>
               <p>Amazon Comprehend entity recognizer endpoint - The resource type and unique identifier are specified using the endpoint ARN. Example: <code>arn:aws:comprehend:us-west-2:123456789012:entity-recognizer-endpoint/EXAMPLE</code>.</p>
            </li>
            <li>
               <p>Lambda provisioned concurrency - The resource type is <code>function</code> and the unique identifier is the function name with a function version or alias name suffix that is not <code>$LATEST</code>. 
               Example: <code>function:my-function:prod</code> or <code>function:my-function:1</code>.</p>
            </li>
            <li>
               <p>Amazon Keyspaces table - The resource type is <code>table</code> and the unique identifier is the table name. 
               Example: <code>keyspace/mykeyspace/table/mytable</code>.</p>
            </li>
            <li>
               <p>Amazon MSK cluster - The resource type and unique identifier are specified using the cluster ARN. 
               Example: <code>arn:aws:kafka:us-east-1:123456789012:cluster/demo-cluster-1/6357e0b2-0e6a-4b86-a0b4-70df934c2e31-5</code>.</p>
            </li>
            <li>
               <p>Amazon ElastiCache replication group - The resource type is <code>replication-group</code> and the unique identifier is the replication group name.
               Example: <code>replication-group/mycluster</code>.</p>
            </li>
            <li>
               <p>Amazon ElastiCache cache cluster - The resource type is <code>cache-cluster</code> and the unique identifier is the cache cluster name.
               Example: <code>cache-cluster/mycluster</code>.</p>
            </li>
            <li>
               <p>Neptune cluster - The resource type is <code>cluster</code> and the unique identifier is the cluster name. Example: <code>cluster:mycluster</code>.</p>
            </li>
            <li>
               <p>SageMaker serverless endpoint - The resource type is <code>variant</code> and the unique identifier is the resource ID.
               Example: <code>endpoint/my-end-point/variant/KMeansClustering</code>.</p>
            </li>
            <li>
               <p>SageMaker inference component - The resource type is <code>inference-component</code> and the unique identifier is the resource ID.
               Example: <code>inference-component/my-inference-component</code>.</p>
            </li>
            <li>
               <p>Pool of WorkSpaces - The resource type is <code>workspacespool</code> and the unique identifier is the pool ID. 
               Example: <code>workspacespool/wspool-123456</code>.</p>
            </li>
         </ul>
    scalable_dimension = "value"  # <p>The scalable dimension. This string consists of the service namespace, resource type, and scaling property.</p>
         <ul>
            <li>
               <p>
                  <code>ecs:service:DesiredCount</code> - The task count of an ECS service.</p>
            </li>
            <li>
               <p>
                  <code>elasticmapreduce:instancegroup:InstanceCount</code> - The instance count of an EMR Instance Group.</p>
            </li>
            <li>
               <p>
                  <code>ec2:spot-fleet-request:TargetCapacity</code> - The target capacity of a Spot Fleet.</p>
            </li>
            <li>
               <p>
                  <code>appstream:fleet:DesiredCapacity</code> - The capacity of an AppStream 2.0 fleet.</p>
            </li>
            <li>
               <p>
                  <code>dynamodb:table:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB table.</p>
            </li>
            <li>
               <p>
                  <code>dynamodb:table:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB table.</p>
            </li>
            <li>
               <p>
                  <code>dynamodb:index:ReadCapacityUnits</code> - The provisioned read capacity for a DynamoDB global secondary index.</p>
            </li>
            <li>
               <p>
                  <code>dynamodb:index:WriteCapacityUnits</code> - The provisioned write capacity for a DynamoDB global secondary index.</p>
            </li>
            <li>
               <p>
                  <code>rds:cluster:ReadReplicaCount</code> - The count of Aurora Replicas in an Aurora DB cluster. Available for Aurora MySQL-compatible edition and Aurora PostgreSQL-compatible edition.</p>
            </li>
            <li>
               <p>
                  <code>sagemaker:variant:DesiredInstanceCount</code> - The number of EC2 instances for a SageMaker model endpoint variant.</p>
            </li>
            <li>
               <p>
                  <code>custom-resource:ResourceType:Property</code> - The scalable dimension for a custom resource provided by your own application or service.</p>
            </li>
            <li>
               <p>
                  <code>comprehend:document-classifier-endpoint:DesiredInferenceUnits</code> - The number of inference units for an Amazon Comprehend document classification endpoint.</p>
            </li>
            <li>
               <p>
                  <code>comprehend:entity-recognizer-endpoint:DesiredInferenceUnits</code> - The number of inference units for an Amazon Comprehend entity recognizer endpoint.</p>
            </li>
            <li>
               <p>
                  <code>lambda:function:ProvisionedConcurrency</code> - The provisioned concurrency for a Lambda function.</p>
            </li>
            <li>
               <p>
                  <code>cassandra:table:ReadCapacityUnits</code> - The provisioned read capacity for an Amazon Keyspaces table.</p>
            </li>
            <li>
               <p>
                  <code>cassandra:table:WriteCapacityUnits</code> - The provisioned write capacity for an Amazon Keyspaces table.</p>
            </li>
            <li>
               <p>
                  <code>kafka:broker-storage:VolumeSize</code> - The provisioned volume size (in GiB) for brokers in an Amazon MSK cluster.</p>
            </li>
            <li>
               <p>
                  <code>elasticache:cache-cluster:Nodes</code> - The number of nodes for an Amazon ElastiCache cache cluster.</p>
            </li>
            <li>
               <p>
                  <code>elasticache:replication-group:NodeGroups</code> - The number of node groups for an Amazon ElastiCache replication group.</p>
            </li>
            <li>
               <p>
                  <code>elasticache:replication-group:Replicas</code> - The number of replicas per node group for an Amazon ElastiCache replication group.</p>
            </li>
            <li>
               <p>
                  <code>neptune:cluster:ReadReplicaCount</code> - The count of read replicas in an Amazon Neptune DB cluster.</p>
            </li>
            <li>
               <p>
                  <code>sagemaker:variant:DesiredProvisionedConcurrency</code> - The provisioned concurrency for a SageMaker serverless endpoint.</p>
            </li>
            <li>
               <p>
                  <code>sagemaker:inference-component:DesiredCopyCount</code> - The number of copies across an endpoint for a SageMaker inference component.</p>
            </li>
            <li>
               <p>
                  <code>workspaces:workspacespool:DesiredUserSessions</code> - The number of user sessions for the WorkSpaces in the pool.</p>
            </li>
         </ul>
    scheduled_action_name = "value"  # <p>The name of the scheduled action. This name must be unique among all other scheduled
         actions on the specified scalable target. </p>
    service_namespace = "value"  # <p>The namespace of the Amazon Web Services service that provides the resource. For a resource provided
         by your own application or service, use <code>custom-resource</code> instead.</p>
}

```

---


### Predictive_scaling_forecast

PredictiveScalingForecast resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `load_forecast` | Vec<String> | <p>
         The load forecast.
      </p> |
| `capacity_forecast` | String | <p>
         The capacity forecast.
      </p> |
| `update_time` | String | <p>
        The time the forecast was made.
      </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access predictive_scaling_forecast outputs
predictive_scaling_forecast_id = predictive_scaling_forecast.id
predictive_scaling_forecast_load_forecast = predictive_scaling_forecast.load_forecast
predictive_scaling_forecast_capacity_forecast = predictive_scaling_forecast.capacity_forecast
predictive_scaling_forecast_update_time = predictive_scaling_forecast.update_time
```

---


### Scaling_policies

ScalingPolicies resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `scaling_policies` | Vec<String> | <p>Information about the scaling policies.</p> |
| `next_token` | String | <p>The token required to get the next set of results. This value is <code>null</code> if
         there are no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access scaling_policies outputs
scaling_policies_id = scaling_policies.id
scaling_policies_scaling_policies = scaling_policies.scaling_policies
scaling_policies_next_token = scaling_policies.next_token
```

---


### Scheduled_actions

ScheduledActions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `scheduled_actions` | Vec<String> | <p>Information about the scheduled actions.</p> |
| `next_token` | String | <p>The token required to get the next set of results. This value is <code>null</code> if
         there are no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access scheduled_actions outputs
scheduled_actions_id = scheduled_actions.id
scheduled_actions_scheduled_actions = scheduled_actions.scheduled_actions
scheduled_actions_next_token = scheduled_actions.next_token
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple scaling_policy resources
scaling_policy_0 = provider.application_auto_scaling.Scaling_policy {
    resource_id = "value-0"
    policy_name = "value-0"
    scalable_dimension = "value-0"
    service_namespace = "value-0"
}
scaling_policy_1 = provider.application_auto_scaling.Scaling_policy {
    resource_id = "value-1"
    policy_name = "value-1"
    scalable_dimension = "value-1"
    service_namespace = "value-1"
}
scaling_policy_2 = provider.application_auto_scaling.Scaling_policy {
    resource_id = "value-2"
    policy_name = "value-2"
    scalable_dimension = "value-2"
    service_namespace = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    scaling_policy = provider.application_auto_scaling.Scaling_policy {
        resource_id = "production-value"
        policy_name = "production-value"
        scalable_dimension = "production-value"
        service_namespace = "production-value"
    }
```

---

## Related Documentation

- [AWS Application_auto_scaling Documentation](https://docs.aws.amazon.com/application_auto_scaling/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
