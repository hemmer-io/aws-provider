# Ecs Service



**Resources**: 23

---

## Overview

The ecs service provides access to 23 resource types:

- [Service_deployments](#service_deployments) [R]
- [Services](#services) [R]
- [Cluster_settings](#cluster_settings) [U]
- [Task_definition](#task_definition) [R]
- [Container_instances_state](#container_instances_state) [U]
- [Capacity_provider](#capacity_provider) [CUD]
- [Service](#service) [CUD]
- [Tasks](#tasks) [R]
- [Cluster_capacity_providers](#cluster_capacity_providers) [C]
- [Container_agent](#container_agent) [U]
- [Task_set](#task_set) [CUD]
- [Task_protection](#task_protection) [RU]
- [Service_revisions](#service_revisions) [R]
- [Container_instances](#container_instances) [R]
- [Capacity_providers](#capacity_providers) [R]
- [Task_sets](#task_sets) [R]
- [Clusters](#clusters) [R]
- [Account_setting](#account_setting) [CD]
- [Cluster](#cluster) [CUD]
- [Account_setting_default](#account_setting_default) [C]
- [Attributes](#attributes) [CD]
- [Service_primary_task_set](#service_primary_task_set) [U]
- [Task_definitions](#task_definitions) [D]

---

## Resources


### Service_deployments

ServiceDeployments resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `failures` | Vec<String> | <p>Any failures associated with the call.</p>
         <p>If you decsribe a deployment with a service revision created before October 25, 2024,
			the call fails. The failure includes the service revision ARN and the reason set to
				<code>MISSING</code>.</p> |
| `service_deployments` | Vec<String> | <p>The list of service deployments described.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access service_deployments outputs
service_deployments_id = service_deployments.id
service_deployments_failures = service_deployments.failures
service_deployments_service_deployments = service_deployments.service_deployments
```

---


### Services

Services resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `services` | Vec<String> | <p>The list of services described.</p> |
| `failures` | Vec<String> | <p>Any failures associated with the call.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access services outputs
services_id = services.id
services_services = services.services
services_failures = services.failures
```

---


### Cluster_settings

ClusterSettings resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `settings` | Vec<String> | ✅ | <p>The setting to use by default for a cluster. This parameter is used to turn on
			CloudWatch Container Insights for a cluster. If this value is specified, it overrides
			the <code>containerInsights</code> value set with <a href="https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_PutAccountSetting.html">PutAccountSetting</a> or <a href="https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_PutAccountSettingDefault.html">PutAccountSettingDefault</a>.</p>
         <important>
            <p>Currently, if you delete an existing cluster that does not have Container Insights
				turned on, and then create a new cluster with the same name with Container Insights
				tuned on, Container Insights will not actually be turned on. If you want to preserve
				the same name for your existing cluster and turn on Container Insights, you must
				wait 7 days before you can re-create it.</p>
         </important> |
| `cluster` | String | ✅ | <p>The name of the cluster to modify the settings for.</p> |



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


### Task_definition

TaskDefinition resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `task_definition` | String | <p>The full task definition description.</p> |
| `tags` | Vec<String> | <p>The metadata that's applied to the task definition to help you categorize and organize
			them. Each tag consists of a key and an optional value. You define both.</p>
         <p>The following basic restrictions apply to tags:</p>
         <ul>
            <li>
               <p>Maximum number of tags per resource - 50</p>
            </li>
            <li>
               <p>For each resource, each tag key must be unique, and each tag key can have only
					one value.</p>
            </li>
            <li>
               <p>Maximum key length - 128 Unicode characters in UTF-8</p>
            </li>
            <li>
               <p>Maximum value length - 256 Unicode characters in UTF-8</p>
            </li>
            <li>
               <p>If your tagging schema is used across multiple services and resources,
					remember that other services may have restrictions on allowed characters.
					Generally allowed characters are: letters, numbers, and spaces representable in
					UTF-8, and the following characters: + - = . _ : / @.</p>
            </li>
            <li>
               <p>Tag keys and values are case-sensitive.</p>
            </li>
            <li>
               <p>Do not use <code>aws:</code>, <code>AWS:</code>, or any upper or lowercase
					combination of such as a prefix for either keys or values as it is reserved for
						Amazon Web
						Services use. You cannot edit or delete tag keys or values with
					this prefix. Tags with this prefix do not count against your tags per resource
					limit.</p>
            </li>
         </ul> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access task_definition outputs
task_definition_id = task_definition.id
task_definition_task_definition = task_definition.task_definition
task_definition_tags = task_definition.tags
```

---


### Container_instances_state

ContainerInstancesState resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `container_instances` | String | ✅ | <p>A list of up to 10 container instance IDs or full ARN entries.</p> |
| `status` | String | ✅ | <p>The container instance state to update the container instance with. The only valid
			values for this action are <code>ACTIVE</code> and <code>DRAINING</code>. A container
			instance can only be updated to <code>DRAINING</code> status once it has reached an
				<code>ACTIVE</code> state. If a container instance is in <code>REGISTERING</code>,
				<code>DEREGISTERING</code>, or <code>REGISTRATION_FAILED</code> state you can
			describe the container instance but can't update the container instance state.</p> |
| `cluster` | String |  | <p>The short name or full Amazon Resource Name (ARN) of the cluster that hosts the
			container instance to update. If you do not specify a cluster, the default cluster is
			assumed.</p> |



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


### Capacity_provider

CapacityProvider resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `auto_scaling_group_provider` | String |  | <p>The details of the Auto Scaling group for the capacity provider.</p> |
| `managed_instances_provider` | String |  | <p>The configuration for the Amazon ECS Managed Instances provider. This configuration
			specifies how Amazon ECS manages Amazon EC2 instances on your behalf, including the
			infrastructure role, instance launch template, and tag propagation settings.</p> |
| `name` | String | ✅ | <p>The name of the capacity provider. Up to 255 characters are allowed. They include
			letters (both upper and lowercase letters), numbers, underscores (_), and hyphens (-).
			The name can't be prefixed with "<code>aws</code>", "<code>ecs</code>", or
				"<code>fargate</code>".</p> |
| `tags` | Vec<String> |  | <p>The metadata that you apply to the capacity provider to categorize and organize them
			more conveniently. Each tag consists of a key and an optional value. You define both of
			them.</p>
         <p>The following basic restrictions apply to tags:</p>
         <ul>
            <li>
               <p>Maximum number of tags per resource - 50</p>
            </li>
            <li>
               <p>For each resource, each tag key must be unique, and each tag key can have only
					one value.</p>
            </li>
            <li>
               <p>Maximum key length - 128 Unicode characters in UTF-8</p>
            </li>
            <li>
               <p>Maximum value length - 256 Unicode characters in UTF-8</p>
            </li>
            <li>
               <p>If your tagging schema is used across multiple services and resources,
					remember that other services may have restrictions on allowed characters.
					Generally allowed characters are: letters, numbers, and spaces representable in
					UTF-8, and the following characters: + - = . _ : / @.</p>
            </li>
            <li>
               <p>Tag keys and values are case-sensitive.</p>
            </li>
            <li>
               <p>Do not use <code>aws:</code>, <code>AWS:</code>, or any upper or lowercase
					combination of such as a prefix for either keys or values as it is reserved for
						Amazon Web
						Services use. You cannot edit or delete tag keys or values with
					this prefix. Tags with this prefix do not count against your tags per resource
					limit.</p>
            </li>
         </ul> |
| `cluster` | String |  | <p>The name of the cluster to associate with the capacity provider. When you create a
			capacity provider with Amazon ECS Managed Instances, it becomes available only within
			the specified cluster.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create capacity_provider
capacity_provider = provider.ecs.Capacity_provider {
    name = "value"  # <p>The name of the capacity provider. Up to 255 characters are allowed. They include
			letters (both upper and lowercase letters), numbers, underscores (_), and hyphens (-).
			The name can't be prefixed with "<code>aws</code>", "<code>ecs</code>", or
				"<code>fargate</code>".</p>
}

```

---


### Service

Service resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `health_check_grace_period_seconds` | i64 |  | <p>The period of time, in seconds, that the Amazon ECS service scheduler ignores
			unhealthy Elastic Load Balancing, VPC Lattice, and container health checks after a task
			has first started. If you do not specify a health check grace period value, the default
			value of 0 is used. If you do not use any of the health checks, then
				<code>healthCheckGracePeriodSeconds</code> is unused.</p>
         <p>If your service has more running tasks than desired, unhealthy tasks in the grace
			period might be stopped to reach the desired count.</p> |
| `launch_type` | String |  | <p>The infrastructure that you run your service on. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/launch_types.html">Amazon
				ECS launch types</a> in the <i>Amazon Elastic Container Service Developer
				Guide</i>.</p>
         <note>
            <p>If you want to use Amazon ECS Managed Instances, you must use the
					<code>capacityProviderStrategy</code> request parameter and omit the
					<code>launchType</code> request parameter.</p>
         </note>
         <p>The <code>FARGATE</code> launch type runs your tasks on Fargate On-Demand
			infrastructure.</p>
         <note>
            <p>Fargate Spot infrastructure is available for use but a capacity provider strategy
				must be used. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/fargate-capacity-providers.html">Fargate capacity providers</a> in
				the <i>Amazon ECS Developer Guide</i>.</p>
         </note>
         <p>The <code>EC2</code> launch type runs your tasks on Amazon EC2 instances registered to
			your cluster.</p>
         <p>The <code>EXTERNAL</code> launch type runs your tasks on your on-premises server or
			virtual machine (VM) capacity registered to your cluster.</p>
         <p>A service can use either a launch type or a capacity provider strategy. If a
				<code>launchType</code> is specified, the <code>capacityProviderStrategy</code>
			parameter must be omitted.</p> |
| `deployment_controller` | String |  | <p>The deployment controller to use for the service. If no deployment controller is
			specified, the default value of <code>ECS</code> is used.</p> |
| `tags` | Vec<String> |  | <p>The metadata that you apply to the service to help you categorize and organize them.
			Each tag consists of a key and an optional value, both of which you define. When a
			service is deleted, the tags are deleted as well.</p>
         <p>The following basic restrictions apply to tags:</p>
         <ul>
            <li>
               <p>Maximum number of tags per resource - 50</p>
            </li>
            <li>
               <p>For each resource, each tag key must be unique, and each tag key can have only
					one value.</p>
            </li>
            <li>
               <p>Maximum key length - 128 Unicode characters in UTF-8</p>
            </li>
            <li>
               <p>Maximum value length - 256 Unicode characters in UTF-8</p>
            </li>
            <li>
               <p>If your tagging schema is used across multiple services and resources,
					remember that other services may have restrictions on allowed characters.
					Generally allowed characters are: letters, numbers, and spaces representable in
					UTF-8, and the following characters: + - = . _ : / @.</p>
            </li>
            <li>
               <p>Tag keys and values are case-sensitive.</p>
            </li>
            <li>
               <p>Do not use <code>aws:</code>, <code>AWS:</code>, or any upper or lowercase
					combination of such as a prefix for either keys or values as it is reserved for
						Amazon Web
						Services use. You cannot edit or delete tag keys or values with
					this prefix. Tags with this prefix do not count against your tags per resource
					limit.</p>
            </li>
         </ul> |
| `volume_configurations` | Vec<String> |  | <p>The configuration for a volume specified in the task definition as a volume that is
			configured at launch time. Currently, the only supported volume type is an Amazon EBS
			volume.</p> |
| `enable_execute_command` | bool |  | <p>Determines whether the execute command functionality is turned on for the service. If
				<code>true</code>, this enables execute command functionality on all containers in
			the service tasks.</p> |
| `enable_ecs_managed_tags` | bool |  | <p>Specifies whether to turn on Amazon ECS managed tags for the tasks within the service.
			For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-using-tags.html">Tagging your Amazon ECS
				resources</a> in the <i>Amazon Elastic Container Service Developer
				Guide</i>.</p>
         <p>When you use Amazon ECS managed tags, you must set the <code>propagateTags</code>
			request parameter.</p> |
| `propagate_tags` | String |  | <p>Specifies whether to propagate the tags from the task definition to the task. If no
			value is specified, the tags aren't propagated. Tags can only be propagated to the task
			during task creation. To add tags to a task after task creation, use the <a href="https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_TagResource.html">TagResource</a> API action.</p>
         <p>You must set this to a value other than <code>NONE</code> when you use Cost Explorer.
			For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/usage-reports.html">Amazon ECS usage
				reports</a> in the <i>Amazon Elastic Container Service Developer
				Guide</i>.</p>
         <p>The default is <code>NONE</code>.</p> |
| `placement_constraints` | Vec<String> |  | <p>An array of placement constraint objects to use for tasks in your service. You can
			specify a maximum of 10 constraints for each task. This limit includes constraints in
			the task definition and those specified at runtime.</p> |
| `platform_version` | String |  | <p>The platform version that your tasks in the service are running on. A platform version
			is specified only for tasks using the Fargate launch type. If one isn't specified, the
				<code>LATEST</code> platform version is used. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/platform_versions.html">Fargate platform versions</a> in the <i>Amazon Elastic
				Container Service Developer Guide</i>.</p> |
| `cluster` | String |  | <p>The short name or full Amazon Resource Name (ARN) of the cluster that you run your
			service on. If you do not specify a cluster, the default cluster is assumed.</p> |
| `scheduling_strategy` | String |  | <p>The scheduling strategy to use for the service. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs_services.html">Services</a>.</p>
         <p>There are two service scheduler strategies available:</p>
         <ul>
            <li>
               <p>
                  <code>REPLICA</code>-The replica scheduling strategy places and maintains the
					desired number of tasks across your cluster. By default, the service scheduler
					spreads tasks across Availability Zones. You can use task placement strategies
					and constraints to customize task placement decisions. This scheduler strategy
					is required if the service uses the <code>CODE_DEPLOY</code> or
						<code>EXTERNAL</code> deployment controller types.</p>
            </li>
            <li>
               <p>
                  <code>DAEMON</code>-The daemon scheduling strategy deploys exactly one task on
					each active container instance that meets all of the task placement constraints
					that you specify in your cluster. The service scheduler also evaluates the task
					placement constraints for running tasks and will stop tasks that don't meet the
					placement constraints. When you're using this strategy, you don't need to
					specify a desired number of tasks, a task placement strategy, or use Service
					Auto Scaling policies.</p>
               <note>
                  <p>Tasks using the Fargate launch type or the <code>CODE_DEPLOY</code> or
							<code>EXTERNAL</code> deployment controller types don't support the
							<code>DAEMON</code> scheduling strategy.</p>
               </note>
            </li>
         </ul> |
| `network_configuration` | String |  | <p>The network configuration for the service. This parameter is required for task
			definitions that use the <code>awsvpc</code> network mode to receive their own elastic
			network interface, and it isn't supported for other network modes. For more information,
			see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-networking.html">Task networking</a>
			in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> |
| `vpc_lattice_configurations` | Vec<String> |  | <p>The VPC Lattice configuration for the service being created.</p> |
| `role` | String |  | <p>The name or full Amazon Resource Name (ARN) of the IAM role that allows Amazon ECS to
			make calls to your load balancer on your behalf. This parameter is only permitted if you
			are using a load balancer with your service and your task definition doesn't use the
				<code>awsvpc</code> network mode. If you specify the <code>role</code> parameter,
			you must also specify a load balancer object with the <code>loadBalancers</code>
			parameter.</p>
         <important>
            <p>If your account has already created the Amazon ECS service-linked role, that role
				is used for your service unless you specify a role here. The service-linked role is
				required if your task definition uses the <code>awsvpc</code> network mode or if the
				service is configured to use service discovery, an external deployment controller,
				multiple target groups, or Elastic Inference accelerators in which case you don't
				specify a role here. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/using-service-linked-roles.html">Using
					service-linked roles for Amazon ECS</a> in the <i>Amazon Elastic
					Container Service Developer Guide</i>.</p>
         </important>
         <p>If your specified role has a path other than <code>/</code>, then you must either
			specify the full role ARN (this is recommended) or prefix the role name with the path.
			For example, if a role with the name <code>bar</code> has a path of <code>/foo/</code>
			then you would specify <code>/foo/bar</code> as the role name. For more information, see
				<a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_identifiers.html#identifiers-friendly-names">Friendly names and paths</a> in the <i>IAM User Guide</i>.</p> |
| `load_balancers` | Vec<String> |  | <p>A load balancer object representing the load balancers to use with your service. For
			more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/service-load-balancing.html">Service load balancing</a> in the <i>Amazon Elastic
				Container Service Developer Guide</i>.</p>
         <p>If the service uses the <code>ECS</code> deployment controller and using either an
			Application Load Balancer or Network Load Balancer, you must specify one or more target
			group ARNs to attach to the service. The service-linked role is required for services
			that use multiple target groups. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/using-service-linked-roles.html">Using service-linked roles for Amazon ECS</a> in the
				<i>Amazon Elastic Container Service Developer Guide</i>.</p>
         <p>If the service uses the <code>CODE_DEPLOY</code> deployment controller, the service is
			required to use either an Application Load Balancer or Network Load Balancer. When
			creating an CodeDeploy deployment group, you specify two target groups (referred to as
			a <code>targetGroupPair</code>). During a deployment, CodeDeploy determines which
			task set in your service has the status <code>PRIMARY</code>, and it associates one
			target group with it. Then, it also associates the other target group with the
			replacement task set. The load balancer can also have up to two listeners: a required
			listener for production traffic and an optional listener that you can use to perform
			validation tests with Lambda functions before routing production traffic to it.</p>
         <p>If you use the <code>CODE_DEPLOY</code> deployment controller, these values can be
			changed when updating the service.</p>
         <p>For Application Load Balancers and Network Load Balancers, this object must contain
			the load balancer target group ARN, the container name, and the container port to access
			from the load balancer. The container name must be as it appears in a container
			definition. The load balancer name parameter must be omitted. When a task from this
			service is placed on a container instance, the container instance and port combination
			is registered as a target in the target group that's specified here.</p>
         <p>For Classic Load Balancers, this object must contain the load balancer name, the
			container name , and the container port to access from the load balancer. The container
			name must be as it appears in a container definition. The target group ARN parameter
			must be omitted. When a task from this service is placed on a container instance, the
			container instance is registered with the load balancer that's specified here.</p>
         <p>Services with tasks that use the <code>awsvpc</code> network mode (for example, those
			with the Fargate launch type) only support Application Load Balancers and Network Load
			Balancers. Classic Load Balancers aren't supported. Also, when you create any target
			groups for these services, you must choose <code>ip</code> as the target type, not
				<code>instance</code>. This is because tasks that use the <code>awsvpc</code>
			network mode are associated with an elastic network interface, not an Amazon EC2
			instance.</p> |
| `capacity_provider_strategy` | Vec<String> |  | <p>The capacity provider strategy to use for the service.</p>
         <note>
            <p>If you want to use Amazon ECS Managed Instances, you must use the
					<code>capacityProviderStrategy</code> request parameter and omit the
					<code>launchType</code> request parameter.</p>
         </note>
         <p>If a <code>capacityProviderStrategy</code> is specified, the <code>launchType</code>
			parameter must be omitted. If no <code>capacityProviderStrategy</code> or
				<code>launchType</code> is specified, the
				<code>defaultCapacityProviderStrategy</code> for the cluster is used.</p>
         <p>A capacity provider strategy can contain a maximum of 20 capacity providers.</p> |
| `client_token` | String |  | <p>An identifier that you provide to ensure the idempotency of the request. It must be
			unique and is case sensitive. Up to 36 ASCII characters in the range of 33-126
			(inclusive) are allowed.</p> |
| `placement_strategy` | Vec<String> |  | <p>The placement strategy objects to use for tasks in your service. You can specify a
			maximum of 5 strategy rules for each service.</p> |
| `desired_count` | i64 |  | <p>The number of instantiations of the specified task definition to place and keep
			running in your service.</p>
         <p>This is required if <code>schedulingStrategy</code> is <code>REPLICA</code> or isn't
			specified. If <code>schedulingStrategy</code> is <code>DAEMON</code> then this isn't
			required.</p> |
| `service_name` | String | ✅ | <p>The name of your service. Up to 255 letters (uppercase and lowercase), numbers,
			underscores, and hyphens are allowed. Service names must be unique within a cluster, but
			you can have similarly named services in multiple clusters within a Region or across
			multiple Regions.</p> |
| `service_registries` | Vec<String> |  | <p>The details of the service discovery registry to associate with this service. For more
			information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/service-discovery.html">Service
				discovery</a>.</p>
         <note>
            <p>Each service may be associated with one service registry. Multiple service
				registries for each service isn't supported.</p>
         </note> |
| `service_connect_configuration` | String |  | <p>The configuration for this service to discover and connect to services, and be
			discovered by, and connected from, other services within a namespace.</p>
         <p>Tasks that run in a namespace can use short names to connect to services in the
			namespace. Tasks can connect to services across all of the clusters in the namespace.
			Tasks connect through a managed proxy container that collects logs and metrics for
			increased visibility. Only the tasks that Amazon ECS services create are supported with
			Service Connect. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/service-connect.html">Service Connect</a>
			in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> |
| `task_definition` | String |  | <p>The <code>family</code> and <code>revision</code> (<code>family:revision</code>) or
			full ARN of the task definition to run in your service. If a <code>revision</code> isn't
			specified, the latest <code>ACTIVE</code> revision is used.</p>
         <p>A task definition must be specified if the service uses either the <code>ECS</code> or
				<code>CODE_DEPLOY</code> deployment controllers.</p>
         <p>For more information about deployment types, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/deployment-types.html">Amazon ECS deployment
				types</a>.</p> |
| `availability_zone_rebalancing` | String |  | <p>Indicates whether to use Availability Zone rebalancing for the service.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/service-rebalancing.html">Balancing an Amazon
				ECS service across Availability Zones</a> in the <i>
               <i>Amazon
					Elastic Container Service Developer Guide</i>
            </i>.</p>
         <p>The default behavior of <code>AvailabilityZoneRebalancing</code> differs between
			create and update requests:</p>
         <ul>
            <li>
               <p>For create service requests, when no value is specified for
						<code>AvailabilityZoneRebalancing</code>, Amazon ECS defaults the value to
						<code>ENABLED</code>.</p>
            </li>
            <li>
               <p>For update service requests, when no value is specified for
						<code>AvailabilityZoneRebalancing</code>, Amazon ECS defaults to the
					existing service’s <code>AvailabilityZoneRebalancing</code> value. If the
					service never had an <code>AvailabilityZoneRebalancing</code> value set, Amazon
					ECS treats this as <code>DISABLED</code>.</p>
            </li>
         </ul> |
| `deployment_configuration` | String |  | <p>Optional deployment parameters that control how many tasks run during the deployment
			and the ordering of stopping and starting tasks.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create service
service = provider.ecs.Service {
    service_name = "value"  # <p>The name of your service. Up to 255 letters (uppercase and lowercase), numbers,
			underscores, and hyphens are allowed. Service names must be unique within a cluster, but
			you can have similarly named services in multiple clusters within a Region or across
			multiple Regions.</p>
}

```

---


### Tasks

Tasks resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `tasks` | Vec<String> | <p>The list of tasks.</p> |
| `failures` | Vec<String> | <p>Any failures associated with the call.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access tasks outputs
tasks_id = tasks.id
tasks_tasks = tasks.tasks
tasks_failures = tasks.failures
```

---


### Cluster_capacity_providers

ClusterCapacityProviders resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `cluster` | String | ✅ | <p>The short name or full Amazon Resource Name (ARN) of the cluster to modify the
			capacity provider settings for. If you don't specify a cluster, the default cluster is
			assumed.</p> |
| `capacity_providers` | String | ✅ | <p>The name of one or more capacity providers to associate with the cluster.</p>
         <p>If specifying a capacity provider that uses an Auto Scaling group, the capacity
			provider must already be created. New capacity providers can be created with the <a href="https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_CreateCapacityProvider.html">CreateCapacityProvider</a> API operation.</p>
         <p>To use a Fargate capacity provider, specify either the <code>FARGATE</code> or
				<code>FARGATE_SPOT</code> capacity providers. The Fargate capacity providers
			are available to all accounts and only need to be associated with a cluster to be
			used.</p> |
| `default_capacity_provider_strategy` | Vec<String> | ✅ | <p>The capacity provider strategy to use by default for the cluster.</p>
         <p>When creating a service or running a task on a cluster, if no capacity provider or
			launch type is specified then the default capacity provider strategy for the cluster is
			used.</p>
         <p>A capacity provider strategy consists of one or more capacity providers along with the
				<code>base</code> and <code>weight</code> to assign to them. A capacity provider
			must be associated with the cluster to be used in a capacity provider strategy. The
				<a href="https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_PutClusterCapacityProviders.html">PutClusterCapacityProviders</a> API is used to associate a capacity provider
			with a cluster. Only capacity providers with an <code>ACTIVE</code> or
				<code>UPDATING</code> status can be used.</p>
         <p>If specifying a capacity provider that uses an Auto Scaling group, the capacity
			provider must already be created. New capacity providers can be created with the <a href="https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_CreateCapacityProvider.html">CreateCapacityProvider</a> API operation.</p>
         <p>To use a Fargate capacity provider, specify either the <code>FARGATE</code> or
				<code>FARGATE_SPOT</code> capacity providers. The Fargate capacity providers
			are available to all accounts and only need to be associated with a cluster to be
			used.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create cluster_capacity_providers
cluster_capacity_providers = provider.ecs.Cluster_capacity_providers {
    cluster = "value"  # <p>The short name or full Amazon Resource Name (ARN) of the cluster to modify the
			capacity provider settings for. If you don't specify a cluster, the default cluster is
			assumed.</p>
    capacity_providers = "value"  # <p>The name of one or more capacity providers to associate with the cluster.</p>
         <p>If specifying a capacity provider that uses an Auto Scaling group, the capacity
			provider must already be created. New capacity providers can be created with the <a href="https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_CreateCapacityProvider.html">CreateCapacityProvider</a> API operation.</p>
         <p>To use a Fargate capacity provider, specify either the <code>FARGATE</code> or
				<code>FARGATE_SPOT</code> capacity providers. The Fargate capacity providers
			are available to all accounts and only need to be associated with a cluster to be
			used.</p>
    default_capacity_provider_strategy = "value"  # <p>The capacity provider strategy to use by default for the cluster.</p>
         <p>When creating a service or running a task on a cluster, if no capacity provider or
			launch type is specified then the default capacity provider strategy for the cluster is
			used.</p>
         <p>A capacity provider strategy consists of one or more capacity providers along with the
				<code>base</code> and <code>weight</code> to assign to them. A capacity provider
			must be associated with the cluster to be used in a capacity provider strategy. The
				<a href="https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_PutClusterCapacityProviders.html">PutClusterCapacityProviders</a> API is used to associate a capacity provider
			with a cluster. Only capacity providers with an <code>ACTIVE</code> or
				<code>UPDATING</code> status can be used.</p>
         <p>If specifying a capacity provider that uses an Auto Scaling group, the capacity
			provider must already be created. New capacity providers can be created with the <a href="https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_CreateCapacityProvider.html">CreateCapacityProvider</a> API operation.</p>
         <p>To use a Fargate capacity provider, specify either the <code>FARGATE</code> or
				<code>FARGATE_SPOT</code> capacity providers. The Fargate capacity providers
			are available to all accounts and only need to be associated with a cluster to be
			used.</p>
}

```

---


### Container_agent

ContainerAgent resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `container_instance` | String | ✅ | <p>The container instance ID or full ARN entries for the container instance where you
			would like to update the Amazon ECS container agent.</p> |
| `cluster` | String |  | <p>The short name or full Amazon Resource Name (ARN) of the cluster that your container
			instance is running on. If you do not specify a cluster, the default cluster is
			assumed.</p> |



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


### Task_set

TaskSet resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `scale` | String |  | <p>A floating-point percentage of the desired number of tasks to place and keep running
			in the task set.</p> |
| `external_id` | String |  | <p>An optional non-unique tag that identifies this task set in external systems. If the
			task set is associated with a service discovery registry, the tasks in this task set
			will have the <code>ECS_TASK_SET_EXTERNAL_ID</code>
			Cloud Map
			attribute set to the provided value.</p> |
| `load_balancers` | Vec<String> |  | <p>A load balancer object representing the load balancer to use with the task set. The
			supported load balancer types are either an Application Load Balancer or a Network Load
			Balancer.</p> |
| `capacity_provider_strategy` | Vec<String> |  | <p>The capacity provider strategy to use for the task set.</p>
         <p>A capacity provider strategy consists of one or more capacity providers along with the
				<code>base</code> and <code>weight</code> to assign to them. A capacity provider
			must be associated with the cluster to be used in a capacity provider strategy. The
				<a href="https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_PutClusterCapacityProviders.html">PutClusterCapacityProviders</a> API is used to associate a capacity provider
			with a cluster. Only capacity providers with an <code>ACTIVE</code> or
				<code>UPDATING</code> status can be used.</p>
         <p>If a <code>capacityProviderStrategy</code> is specified, the <code>launchType</code>
			parameter must be omitted. If no <code>capacityProviderStrategy</code> or
				<code>launchType</code> is specified, the
				<code>defaultCapacityProviderStrategy</code> for the cluster is used.</p>
         <p>If specifying a capacity provider that uses an Auto Scaling group, the capacity
			provider must already be created. New capacity providers can be created with the <a href="https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_CreateCapacityProviderProvider.html">CreateCapacityProviderProvider</a>API operation.</p>
         <p>To use a Fargate capacity provider, specify either the <code>FARGATE</code> or
				<code>FARGATE_SPOT</code> capacity providers. The Fargate capacity providers
			are available to all accounts and only need to be associated with a cluster to be
			used.</p>
         <p>The <a href="https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_PutClusterCapacityProviders.html">PutClusterCapacityProviders</a> API operation is used to update the list of
			available capacity providers for a cluster after the cluster is created.</p> |
| `launch_type` | String |  | <p>The launch type that new tasks in the task set uses. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/launch_types.html">Amazon
				ECS launch types</a> in the <i>Amazon Elastic Container Service Developer
				Guide</i>.</p>
         <p>If a <code>launchType</code> is specified, the <code>capacityProviderStrategy</code>
			parameter must be omitted.</p> |
| `client_token` | String |  | <p>An identifier that you provide to ensure the idempotency of the request. It must be
			unique and is case sensitive. Up to 36 ASCII characters in the range of 33-126
			(inclusive) are allowed.</p> |
| `tags` | Vec<String> |  | <p>The metadata that you apply to the task set to help you categorize and organize them.
			Each tag consists of a key and an optional value. You define both. When a service is
			deleted, the tags are deleted.</p>
         <p>The following basic restrictions apply to tags:</p>
         <ul>
            <li>
               <p>Maximum number of tags per resource - 50</p>
            </li>
            <li>
               <p>For each resource, each tag key must be unique, and each tag key can have only
					one value.</p>
            </li>
            <li>
               <p>Maximum key length - 128 Unicode characters in UTF-8</p>
            </li>
            <li>
               <p>Maximum value length - 256 Unicode characters in UTF-8</p>
            </li>
            <li>
               <p>If your tagging schema is used across multiple services and resources,
					remember that other services may have restrictions on allowed characters.
					Generally allowed characters are: letters, numbers, and spaces representable in
					UTF-8, and the following characters: + - = . _ : / @.</p>
            </li>
            <li>
               <p>Tag keys and values are case-sensitive.</p>
            </li>
            <li>
               <p>Do not use <code>aws:</code>, <code>AWS:</code>, or any upper or lowercase
					combination of such as a prefix for either keys or values as it is reserved for
						Amazon Web
						Services use. You cannot edit or delete tag keys or values with
					this prefix. Tags with this prefix do not count against your tags per resource
					limit.</p>
            </li>
         </ul> |
| `service` | String | ✅ | <p>The short name or full Amazon Resource Name (ARN) of the service to create the task
			set in.</p> |
| `cluster` | String | ✅ | <p>The short name or full Amazon Resource Name (ARN) of the cluster that hosts the
			service to create the task set in.</p> |
| `network_configuration` | String |  | <p>An object representing the network configuration for a task set.</p> |
| `service_registries` | Vec<String> |  | <p>The details of the service discovery registries to assign to this task set. For more
			information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/service-discovery.html">Service
				discovery</a>.</p> |
| `task_definition` | String | ✅ | <p>The task definition for the tasks in the task set to use. If a revision isn't
			specified, the latest <code>ACTIVE</code> revision is used.</p> |
| `platform_version` | String |  | <p>The platform version that the tasks in the task set uses. A platform version is
			specified only for tasks using the Fargate launch type. If one isn't specified, the
				<code>LATEST</code> platform version is used.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create task_set
task_set = provider.ecs.Task_set {
    service = "value"  # <p>The short name or full Amazon Resource Name (ARN) of the service to create the task
			set in.</p>
    cluster = "value"  # <p>The short name or full Amazon Resource Name (ARN) of the cluster that hosts the
			service to create the task set in.</p>
    task_definition = "value"  # <p>The task definition for the tasks in the task set to use. If a revision isn't
			specified, the latest <code>ACTIVE</code> revision is used.</p>
}

```

---


### Task_protection

TaskProtection resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `cluster` | String | ✅ | <p>The short name or full Amazon Resource Name (ARN) of the cluster that hosts the
			service that the task sets exist in.</p> |
| `protection_enabled` | bool | ✅ | <p>Specify <code>true</code> to mark a task for protection and <code>false</code> to
			unset protection, making it eligible for termination.</p> |
| `tasks` | String | ✅ | <p>A list of up to 10 task IDs or full ARN entries.</p> |
| `expires_in_minutes` | i64 |  | <p>If you set <code>protectionEnabled</code> to <code>true</code>, you can specify the
			duration for task protection in minutes. You can specify a value from 1 minute to up to
			2,880 minutes (48 hours). During this time, your task will not be terminated by scale-in
			events from Service Auto Scaling or deployments. After this time period lapses,
				<code>protectionEnabled</code> will be reset to <code>false</code>.</p>
         <p>If you don’t specify the time, then the task is automatically protected for 120
			minutes (2 hours).</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `protected_tasks` | Vec<String> | <p>A list of tasks with the following information.</p>
         <ul>
            <li>
               <p>
                  <code>taskArn</code>: The task ARN.</p>
            </li>
            <li>
               <p>
                  <code>protectionEnabled</code>: The protection status of the task. If scale-in
					protection is turned on for a task, the value is <code>true</code>. Otherwise,
					it is <code>false</code>.</p>
            </li>
            <li>
               <p>
                  <code>expirationDate</code>: The epoch time when protection for the task will
					expire.</p>
            </li>
         </ul> |
| `failures` | Vec<String> | <p>Any failures associated with the call.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access task_protection outputs
task_protection_id = task_protection.id
task_protection_protected_tasks = task_protection.protected_tasks
task_protection_failures = task_protection.failures
```

---


### Service_revisions

ServiceRevisions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `failures` | Vec<String> | <p>Any failures associated with the call.</p> |
| `service_revisions` | Vec<String> | <p>The list of service revisions described.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access service_revisions outputs
service_revisions_id = service_revisions.id
service_revisions_failures = service_revisions.failures
service_revisions_service_revisions = service_revisions.service_revisions
```

---


### Container_instances

ContainerInstances resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `failures` | Vec<String> | <p>Any failures associated with the call.</p> |
| `container_instances` | Vec<String> | <p>The list of container instances.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access container_instances outputs
container_instances_id = container_instances.id
container_instances_failures = container_instances.failures
container_instances_container_instances = container_instances.container_instances
```

---


### Capacity_providers

CapacityProviders resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `capacity_providers` | Vec<String> | <p>The list of capacity providers.</p> |
| `next_token` | String | <p>The <code>nextToken</code> value to include in a future
				<code>DescribeCapacityProviders</code> request. When the results of a
				<code>DescribeCapacityProviders</code> request exceed <code>maxResults</code>, this
			value can be used to retrieve the next page of results. This value is <code>null</code>
			when there are no more results to return.</p> |
| `failures` | Vec<String> | <p>Any failures associated with the call.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access capacity_providers outputs
capacity_providers_id = capacity_providers.id
capacity_providers_capacity_providers = capacity_providers.capacity_providers
capacity_providers_next_token = capacity_providers.next_token
capacity_providers_failures = capacity_providers.failures
```

---


### Task_sets

TaskSets resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `task_sets` | Vec<String> | <p>The list of task sets described.</p> |
| `failures` | Vec<String> | <p>Any failures associated with the call.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access task_sets outputs
task_sets_id = task_sets.id
task_sets_task_sets = task_sets.task_sets
task_sets_failures = task_sets.failures
```

---


### Clusters

Clusters resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `clusters` | Vec<String> | <p>The list of clusters.</p> |
| `failures` | Vec<String> | <p>Any failures associated with the call.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access clusters outputs
clusters_id = clusters.id
clusters_clusters = clusters.clusters
clusters_failures = clusters.failures
```

---


### Account_setting

AccountSetting resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `value` | String | ✅ | <p>The account setting value for the specified principal ARN. Accepted values are
				<code>enabled</code>, <code>disabled</code>, <code>enhanced</code>, <code>on</code>,
			and <code>off</code>.</p>
         <p>When you specify <code>fargateTaskRetirementWaitPeriod</code> for the
				<code>name</code>, the following are the valid values:</p>
         <ul>
            <li>
               <p>
                  <code>0</code> - Amazon Web Services sends the notification, and
					immediately retires the affected tasks.</p>
            </li>
            <li>
               <p>
                  <code>7</code> - Amazon Web Services sends the notification, and waits 7
					calendar days to retire the tasks.</p>
            </li>
            <li>
               <p>
                  <code>14</code> - Amazon Web Services sends the notification, and waits 14
					calendar days to retire the tasks.</p>
            </li>
         </ul> |
| `name` | String | ✅ | <p>The Amazon ECS account setting name to modify.</p>
         <p>The following are the valid values for the account setting name.</p>
         <ul>
            <li>
               <p>
                  <code>serviceLongArnFormat</code> - When modified, the Amazon Resource Name
					(ARN) and resource ID format of the resource type for a specified user, role, or
					the root user for an account is affected. The opt-in and opt-out account setting
					must be set for each Amazon ECS resource separately. The ARN and resource ID
					format of a resource is defined by the opt-in status of the user or role that
					created the resource. You must turn on this setting to use Amazon ECS features
					such as resource tagging.</p>
            </li>
            <li>
               <p>
                  <code>taskLongArnFormat</code> - When modified, the Amazon Resource Name (ARN)
					and resource ID format of the resource type for a specified user, role, or the
					root user for an account is affected. The opt-in and opt-out account setting
					must be set for each Amazon ECS resource separately. The ARN and resource ID
					format of a resource is defined by the opt-in status of the user or role that
					created the resource. You must turn on this setting to use Amazon ECS features
					such as resource tagging.</p>
            </li>
            <li>
               <p>
                  <code>containerInstanceLongArnFormat</code> - When modified, the Amazon
					Resource Name (ARN) and resource ID format of the resource type for a specified
					user, role, or the root user for an account is affected. The opt-in and opt-out
					account setting must be set for each Amazon ECS resource separately. The ARN and
					resource ID format of a resource is defined by the opt-in status of the user or
					role that created the resource. You must turn on this setting to use Amazon ECS
					features such as resource tagging.</p>
            </li>
            <li>
               <p>
                  <code>awsvpcTrunking</code> - When modified, the elastic network interface
					(ENI) limit for any new container instances that support the feature is changed.
					If <code>awsvpcTrunking</code> is turned on, any new container instances that
					support the feature are launched have the increased ENI limits available to
					them. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/container-instance-eni.html">Elastic
						Network Interface Trunking</a> in the <i>Amazon Elastic Container
						Service Developer Guide</i>.</p>
            </li>
            <li>
               <p>
                  <code>containerInsights</code> - Container Insights with enhanced
					observability provides all the Container Insights metrics, plus additional task
					and container metrics. This version supports enhanced observability for Amazon
					ECS clusters using the Amazon EC2 and Fargate launch types. After you configure
					Container Insights with enhanced observability on Amazon ECS, Container Insights
					auto-collects detailed infrastructure telemetry from the cluster level down to
					the container level in your environment and displays these critical performance
					data in curated dashboards removing the heavy lifting in observability set-up. </p>
               <p>To use Container Insights with enhanced observability, set the
						<code>containerInsights</code> account setting to
					<code>enhanced</code>.</p>
               <p>To use Container Insights, set the <code>containerInsights</code> account
					setting to <code>enabled</code>.</p>
               <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/cloudwatch-container-insights.html">Monitor Amazon ECS containers using Container Insights with enhanced
						observability</a> in the <i>Amazon Elastic Container Service
						Developer Guide</i>.</p>
            </li>
            <li>
               <p>
                  <code>dualStackIPv6</code> - When turned on, when using a VPC in dual stack
					mode, your tasks using the <code>awsvpc</code> network mode can have an IPv6
					address assigned. For more information on using IPv6 with tasks launched on
					Amazon EC2 instances, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-networking-awsvpc.html#task-networking-vpc-dual-stack">Using a VPC in dual-stack mode</a>. For more information on using IPv6
					with tasks launched on Fargate, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/fargate-task-networking.html#fargate-task-networking-vpc-dual-stack">Using a VPC in dual-stack mode</a>.</p>
            </li>
            <li>
               <p>
                  <code>fargateTaskRetirementWaitPeriod</code> - When Amazon Web Services
					determines that a security or infrastructure update is needed for an Amazon ECS
					task hosted on Fargate, the tasks need to be stopped and new tasks launched to
					replace them. Use <code>fargateTaskRetirementWaitPeriod</code> to configure the
					wait time to retire a Fargate task. For information about the Fargate tasks
					maintenance, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-maintenance.html">Amazon Web
							Services Fargate task maintenance</a> in the
						<i>Amazon ECS Developer Guide</i>.</p>
            </li>
            <li>
               <p>
                  <code>tagResourceAuthorization</code> - Amazon ECS is introducing tagging
					authorization for resource creation. Users must have permissions for actions
					that create the resource, such as <code>ecsCreateCluster</code>. If tags are
					specified when you create a resource, Amazon Web Services performs
					additional authorization to verify if users or roles have permissions to create
					tags. Therefore, you must grant explicit permissions to use the
						<code>ecs:TagResource</code> action. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/supported-iam-actions-tagging.html">Grant permission to tag resources on creation</a> in the
						<i>Amazon ECS Developer Guide</i>.</p>
            </li>
            <li>
               <p>
                  <code>defaultLogDriverMode</code> - Amazon ECS supports setting a default
					delivery mode of log messages from a container to the <code>logDriver</code>
					that you specify in the container's <code>logConfiguration</code>. The delivery
					mode affects application stability when the flow of logs from the container to
					the log driver is interrupted. The <code>defaultLogDriverMode</code> setting
					supports two values: <code>blocking</code> and <code>non-blocking</code>. If you
					don't specify a delivery mode in your container definition's
						<code>logConfiguration</code>, the mode you specify using this account
					setting will be used as the default. For more information about log delivery
					modes, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_LogConfiguration.html">LogConfiguration</a>. </p>
               <note>
                  <p>On June 25, 2025, Amazon ECS changed the default log driver mode from
							<code>blocking</code> to <code>non-blocking</code> to prioritize task
						availability over logging. To continue using the <code>blocking</code> mode
						after this change, do one of the following:</p>
                  <ul>
                     <li>
                        <p>Set the <code>mode</code> option in your container definition's
									<code>logConfiguration</code> as <code>blocking</code>.</p>
                     </li>
                     <li>
                        <p>Set the <code>defaultLogDriverMode</code> account setting to
									<code>blocking</code>.</p>
                     </li>
                  </ul>
               </note>
            </li>
            <li>
               <p>
                  <code>guardDutyActivate</code> - The <code>guardDutyActivate</code> parameter
					is read-only in Amazon ECS and indicates whether Amazon ECS Runtime Monitoring
					is enabled or disabled by your security administrator in your Amazon ECS
					account. Amazon GuardDuty controls this account setting on your behalf. For more
					information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-guard-duty-integration.html">Protecting Amazon ECS workloads with Amazon ECS Runtime
					Monitoring</a>.</p>
            </li>
         </ul> |
| `principal_arn` | String |  | <p>The ARN of the principal, which can be a user, role, or the root user. If you specify
			the root user, it modifies the account setting for all users, roles, and the root user
			of the account unless a user or role explicitly overrides these settings. If this field
			is omitted, the setting is changed only for the authenticated user.</p>
         <p>In order to use this parameter, you must be the root user, or the principal.</p>
         <note>
            <p>You must use the root user when you set the Fargate wait time
					(<code>fargateTaskRetirementWaitPeriod</code>). </p>
            <p>Federated users assume the account setting of the root user and can't have
				explicit account settings set for them.</p>
         </note> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create account_setting
account_setting = provider.ecs.Account_setting {
    value = "value"  # <p>The account setting value for the specified principal ARN. Accepted values are
				<code>enabled</code>, <code>disabled</code>, <code>enhanced</code>, <code>on</code>,
			and <code>off</code>.</p>
         <p>When you specify <code>fargateTaskRetirementWaitPeriod</code> for the
				<code>name</code>, the following are the valid values:</p>
         <ul>
            <li>
               <p>
                  <code>0</code> - Amazon Web Services sends the notification, and
					immediately retires the affected tasks.</p>
            </li>
            <li>
               <p>
                  <code>7</code> - Amazon Web Services sends the notification, and waits 7
					calendar days to retire the tasks.</p>
            </li>
            <li>
               <p>
                  <code>14</code> - Amazon Web Services sends the notification, and waits 14
					calendar days to retire the tasks.</p>
            </li>
         </ul>
    name = "value"  # <p>The Amazon ECS account setting name to modify.</p>
         <p>The following are the valid values for the account setting name.</p>
         <ul>
            <li>
               <p>
                  <code>serviceLongArnFormat</code> - When modified, the Amazon Resource Name
					(ARN) and resource ID format of the resource type for a specified user, role, or
					the root user for an account is affected. The opt-in and opt-out account setting
					must be set for each Amazon ECS resource separately. The ARN and resource ID
					format of a resource is defined by the opt-in status of the user or role that
					created the resource. You must turn on this setting to use Amazon ECS features
					such as resource tagging.</p>
            </li>
            <li>
               <p>
                  <code>taskLongArnFormat</code> - When modified, the Amazon Resource Name (ARN)
					and resource ID format of the resource type for a specified user, role, or the
					root user for an account is affected. The opt-in and opt-out account setting
					must be set for each Amazon ECS resource separately. The ARN and resource ID
					format of a resource is defined by the opt-in status of the user or role that
					created the resource. You must turn on this setting to use Amazon ECS features
					such as resource tagging.</p>
            </li>
            <li>
               <p>
                  <code>containerInstanceLongArnFormat</code> - When modified, the Amazon
					Resource Name (ARN) and resource ID format of the resource type for a specified
					user, role, or the root user for an account is affected. The opt-in and opt-out
					account setting must be set for each Amazon ECS resource separately. The ARN and
					resource ID format of a resource is defined by the opt-in status of the user or
					role that created the resource. You must turn on this setting to use Amazon ECS
					features such as resource tagging.</p>
            </li>
            <li>
               <p>
                  <code>awsvpcTrunking</code> - When modified, the elastic network interface
					(ENI) limit for any new container instances that support the feature is changed.
					If <code>awsvpcTrunking</code> is turned on, any new container instances that
					support the feature are launched have the increased ENI limits available to
					them. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/container-instance-eni.html">Elastic
						Network Interface Trunking</a> in the <i>Amazon Elastic Container
						Service Developer Guide</i>.</p>
            </li>
            <li>
               <p>
                  <code>containerInsights</code> - Container Insights with enhanced
					observability provides all the Container Insights metrics, plus additional task
					and container metrics. This version supports enhanced observability for Amazon
					ECS clusters using the Amazon EC2 and Fargate launch types. After you configure
					Container Insights with enhanced observability on Amazon ECS, Container Insights
					auto-collects detailed infrastructure telemetry from the cluster level down to
					the container level in your environment and displays these critical performance
					data in curated dashboards removing the heavy lifting in observability set-up. </p>
               <p>To use Container Insights with enhanced observability, set the
						<code>containerInsights</code> account setting to
					<code>enhanced</code>.</p>
               <p>To use Container Insights, set the <code>containerInsights</code> account
					setting to <code>enabled</code>.</p>
               <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/cloudwatch-container-insights.html">Monitor Amazon ECS containers using Container Insights with enhanced
						observability</a> in the <i>Amazon Elastic Container Service
						Developer Guide</i>.</p>
            </li>
            <li>
               <p>
                  <code>dualStackIPv6</code> - When turned on, when using a VPC in dual stack
					mode, your tasks using the <code>awsvpc</code> network mode can have an IPv6
					address assigned. For more information on using IPv6 with tasks launched on
					Amazon EC2 instances, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-networking-awsvpc.html#task-networking-vpc-dual-stack">Using a VPC in dual-stack mode</a>. For more information on using IPv6
					with tasks launched on Fargate, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/fargate-task-networking.html#fargate-task-networking-vpc-dual-stack">Using a VPC in dual-stack mode</a>.</p>
            </li>
            <li>
               <p>
                  <code>fargateTaskRetirementWaitPeriod</code> - When Amazon Web Services
					determines that a security or infrastructure update is needed for an Amazon ECS
					task hosted on Fargate, the tasks need to be stopped and new tasks launched to
					replace them. Use <code>fargateTaskRetirementWaitPeriod</code> to configure the
					wait time to retire a Fargate task. For information about the Fargate tasks
					maintenance, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-maintenance.html">Amazon Web
							Services Fargate task maintenance</a> in the
						<i>Amazon ECS Developer Guide</i>.</p>
            </li>
            <li>
               <p>
                  <code>tagResourceAuthorization</code> - Amazon ECS is introducing tagging
					authorization for resource creation. Users must have permissions for actions
					that create the resource, such as <code>ecsCreateCluster</code>. If tags are
					specified when you create a resource, Amazon Web Services performs
					additional authorization to verify if users or roles have permissions to create
					tags. Therefore, you must grant explicit permissions to use the
						<code>ecs:TagResource</code> action. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/supported-iam-actions-tagging.html">Grant permission to tag resources on creation</a> in the
						<i>Amazon ECS Developer Guide</i>.</p>
            </li>
            <li>
               <p>
                  <code>defaultLogDriverMode</code> - Amazon ECS supports setting a default
					delivery mode of log messages from a container to the <code>logDriver</code>
					that you specify in the container's <code>logConfiguration</code>. The delivery
					mode affects application stability when the flow of logs from the container to
					the log driver is interrupted. The <code>defaultLogDriverMode</code> setting
					supports two values: <code>blocking</code> and <code>non-blocking</code>. If you
					don't specify a delivery mode in your container definition's
						<code>logConfiguration</code>, the mode you specify using this account
					setting will be used as the default. For more information about log delivery
					modes, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_LogConfiguration.html">LogConfiguration</a>. </p>
               <note>
                  <p>On June 25, 2025, Amazon ECS changed the default log driver mode from
							<code>blocking</code> to <code>non-blocking</code> to prioritize task
						availability over logging. To continue using the <code>blocking</code> mode
						after this change, do one of the following:</p>
                  <ul>
                     <li>
                        <p>Set the <code>mode</code> option in your container definition's
									<code>logConfiguration</code> as <code>blocking</code>.</p>
                     </li>
                     <li>
                        <p>Set the <code>defaultLogDriverMode</code> account setting to
									<code>blocking</code>.</p>
                     </li>
                  </ul>
               </note>
            </li>
            <li>
               <p>
                  <code>guardDutyActivate</code> - The <code>guardDutyActivate</code> parameter
					is read-only in Amazon ECS and indicates whether Amazon ECS Runtime Monitoring
					is enabled or disabled by your security administrator in your Amazon ECS
					account. Amazon GuardDuty controls this account setting on your behalf. For more
					information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-guard-duty-integration.html">Protecting Amazon ECS workloads with Amazon ECS Runtime
					Monitoring</a>.</p>
            </li>
         </ul>
}

```

---


### Cluster

Cluster resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `default_capacity_provider_strategy` | Vec<String> |  | <p>The capacity provider strategy to set as the default for the cluster. After a default
			capacity provider strategy is set for a cluster, when you call the <a href="https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_CreateService.html">CreateService</a> or <a href="https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_RunTask.html">RunTask</a> APIs with no
			capacity provider strategy or launch type specified, the default capacity provider
			strategy for the cluster is used.</p>
         <p>If a default capacity provider strategy isn't defined for a cluster when it was
			created, it can be defined later with the <a href="https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_PutClusterCapacityProviders.html">PutClusterCapacityProviders</a> API operation.</p> |
| `capacity_providers` | String |  | <p>The short name of one or more capacity providers to associate with the cluster. A
			capacity provider must be associated with a cluster before it can be included as part of
			the default capacity provider strategy of the cluster or used in a capacity provider
			strategy when calling the <a href="https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_CreateService.html">CreateService</a> or
				<a href="https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_RunTask.html">RunTask</a> actions.</p>
         <p>If specifying a capacity provider that uses an Auto Scaling group, the capacity
			provider must be created but not associated with another cluster. New Auto Scaling group
			capacity providers can be created with the <a href="https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_CreateCapacityProvider.html">CreateCapacityProvider</a> API operation.</p>
         <p>To use a Fargate capacity provider, specify either the <code>FARGATE</code> or
				<code>FARGATE_SPOT</code> capacity providers. The Fargate capacity providers
			are available to all accounts and only need to be associated with a cluster to be
			used.</p>
         <p>The <a href="https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_PutCapacityProvider.html">PutCapacityProvider</a> API operation is used to update the list of available
			capacity providers for a cluster after the cluster is created.</p> |
| `service_connect_defaults` | String |  | <p>Use this parameter to set a default Service Connect namespace. After you set a default
			Service Connect namespace, any new services with Service Connect turned on that are
			created in the cluster are added as client services in the namespace. This setting only
			applies to new services that set the <code>enabled</code> parameter to <code>true</code>
			in the <code>ServiceConnectConfiguration</code>. You can set the namespace of each
			service individually in the <code>ServiceConnectConfiguration</code> to override this
			default parameter.</p>
         <p>Tasks that run in a namespace can use short names to connect to services in the
			namespace. Tasks can connect to services across all of the clusters in the namespace.
			Tasks connect through a managed proxy container that collects logs and metrics for
			increased visibility. Only the tasks that Amazon ECS services create are supported with
			Service Connect. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/service-connect.html">Service Connect</a>
			in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> |
| `tags` | Vec<String> |  | <p>The metadata that you apply to the cluster to help you categorize and organize them.
			Each tag consists of a key and an optional value. You define both.</p>
         <p>The following basic restrictions apply to tags:</p>
         <ul>
            <li>
               <p>Maximum number of tags per resource - 50</p>
            </li>
            <li>
               <p>For each resource, each tag key must be unique, and each tag key can have only
					one value.</p>
            </li>
            <li>
               <p>Maximum key length - 128 Unicode characters in UTF-8</p>
            </li>
            <li>
               <p>Maximum value length - 256 Unicode characters in UTF-8</p>
            </li>
            <li>
               <p>If your tagging schema is used across multiple services and resources,
					remember that other services may have restrictions on allowed characters.
					Generally allowed characters are: letters, numbers, and spaces representable in
					UTF-8, and the following characters: + - = . _ : / @.</p>
            </li>
            <li>
               <p>Tag keys and values are case-sensitive.</p>
            </li>
            <li>
               <p>Do not use <code>aws:</code>, <code>AWS:</code>, or any upper or lowercase
					combination of such as a prefix for either keys or values as it is reserved for
						Amazon Web
						Services use. You cannot edit or delete tag keys or values with
					this prefix. Tags with this prefix do not count against your tags per resource
					limit.</p>
            </li>
         </ul> |
| `cluster_name` | String |  | <p>The name of your cluster. If you don't specify a name for your cluster, you create a
			cluster that's named <code>default</code>. Up to 255 letters (uppercase and lowercase),
			numbers, underscores, and hyphens are allowed. </p> |
| `settings` | Vec<String> |  | <p>The setting to use when creating a cluster. This parameter is used to turn on
			CloudWatch Container Insights for a cluster. If this value is specified, it overrides
			the <code>containerInsights</code> value set with <a href="https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_PutAccountSetting.html">PutAccountSetting</a> or <a href="https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_PutAccountSettingDefault.html">PutAccountSettingDefault</a>.</p> |
| `configuration` | String |  | <p>The <code>execute</code> command configuration for the cluster.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create cluster
cluster = provider.ecs.Cluster {
}

```

---


### Account_setting_default

AccountSettingDefault resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>The resource name for which to modify the account setting.</p>
         <p>The following are the valid values for the account setting name.</p>
         <ul>
            <li>
               <p>
                  <code>serviceLongArnFormat</code> - When modified, the Amazon Resource Name
					(ARN) and resource ID format of the resource type for a specified user, role, or
					the root user for an account is affected. The opt-in and opt-out account setting
					must be set for each Amazon ECS resource separately. The ARN and resource ID
					format of a resource is defined by the opt-in status of the user or role that
					created the resource. You must turn on this setting to use Amazon ECS features
					such as resource tagging.</p>
            </li>
            <li>
               <p>
                  <code>taskLongArnFormat</code> - When modified, the Amazon Resource Name (ARN)
					and resource ID format of the resource type for a specified user, role, or the
					root user for an account is affected. The opt-in and opt-out account setting
					must be set for each Amazon ECS resource separately. The ARN and resource ID
					format of a resource is defined by the opt-in status of the user or role that
					created the resource. You must turn on this setting to use Amazon ECS features
					such as resource tagging.</p>
            </li>
            <li>
               <p>
                  <code>containerInstanceLongArnFormat</code> - When modified, the Amazon
					Resource Name (ARN) and resource ID format of the resource type for a specified
					user, role, or the root user for an account is affected. The opt-in and opt-out
					account setting must be set for each Amazon ECS resource separately. The ARN and
					resource ID format of a resource is defined by the opt-in status of the user or
					role that created the resource. You must turn on this setting to use Amazon ECS
					features such as resource tagging.</p>
            </li>
            <li>
               <p>
                  <code>awsvpcTrunking</code> - When modified, the elastic network interface
					(ENI) limit for any new container instances that support the feature is changed.
					If <code>awsvpcTrunking</code> is turned on, any new container instances that
					support the feature are launched have the increased ENI limits available to
					them. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/container-instance-eni.html">Elastic
						Network Interface Trunking</a> in the <i>Amazon Elastic Container
						Service Developer Guide</i>.</p>
            </li>
            <li>
               <p>
                  <code>containerInsights</code> - Container Insights with enhanced
					observability provides all the Container Insights metrics, plus additional task
					and container metrics. This version supports enhanced observability for Amazon
					ECS clusters using the Amazon EC2 and Fargate launch types. After you configure
					Container Insights with enhanced observability on Amazon ECS, Container Insights
					auto-collects detailed infrastructure telemetry from the cluster level down to
					the container level in your environment and displays these critical performance
					data in curated dashboards removing the heavy lifting in observability set-up. </p>
               <p>To use Container Insights with enhanced observability, set the
						<code>containerInsights</code> account setting to
					<code>enhanced</code>.</p>
               <p>To use Container Insights, set the <code>containerInsights</code> account
					setting to <code>enabled</code>.</p>
               <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/cloudwatch-container-insights.html">Monitor Amazon ECS containers using Container Insights with enhanced
						observability</a> in the <i>Amazon Elastic Container Service
						Developer Guide</i>.</p>
            </li>
            <li>
               <p>
                  <code>dualStackIPv6</code> - When turned on, when using a VPC in dual stack
					mode, your tasks using the <code>awsvpc</code> network mode can have an IPv6
					address assigned. For more information on using IPv6 with tasks launched on
					Amazon EC2 instances, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-networking-awsvpc.html#task-networking-vpc-dual-stack">Using a VPC in dual-stack mode</a>. For more information on using IPv6
					with tasks launched on Fargate, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/fargate-task-networking.html#fargate-task-networking-vpc-dual-stack">Using a VPC in dual-stack mode</a>.</p>
            </li>
            <li>
               <p>
                  <code>fargateFIPSMode</code> - If you specify <code>fargateFIPSMode</code>,
						Fargate FIPS 140 compliance is affected.</p>
            </li>
            <li>
               <p>
                  <code>fargateTaskRetirementWaitPeriod</code> - When Amazon Web Services
					determines that a security or infrastructure update is needed for an Amazon ECS
					task hosted on Fargate, the tasks need to be stopped and new tasks launched to
					replace them. Use <code>fargateTaskRetirementWaitPeriod</code> to configure the
					wait time to retire a Fargate task. For information about the Fargate tasks
					maintenance, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-maintenance.html">Amazon Web
							Services Fargate task maintenance</a> in the
						<i>Amazon ECS Developer Guide</i>.</p>
            </li>
            <li>
               <p>
                  <code>tagResourceAuthorization</code> - Amazon ECS is introducing tagging
					authorization for resource creation. Users must have permissions for actions
					that create the resource, such as <code>ecsCreateCluster</code>. If tags are
					specified when you create a resource, Amazon Web Services performs
					additional authorization to verify if users or roles have permissions to create
					tags. Therefore, you must grant explicit permissions to use the
						<code>ecs:TagResource</code> action. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/supported-iam-actions-tagging.html">Grant permission to tag resources on creation</a> in the
						<i>Amazon ECS Developer Guide</i>.</p>
            </li>
            <li>
               <p>
                  <code>defaultLogDriverMode</code> -Amazon ECS supports setting a default
					delivery mode of log messages from a container to the <code>logDriver</code>
					that you specify in the container's <code>logConfiguration</code>. The delivery
					mode affects application stability when the flow of logs from the container to
					the log driver is interrupted. The <code>defaultLogDriverMode</code> setting
					supports two values: <code>blocking</code> and <code>non-blocking</code>. If you
					don't specify a delivery mode in your container definition's
						<code>logConfiguration</code>, the mode you specify using this account
					setting will be used as the default. For more information about log delivery
					modes, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_LogConfiguration.html">LogConfiguration</a>.</p>
               <note>
                  <p>On June 25, 2025, Amazon ECS changed the default log driver mode from
							<code>blocking</code> to <code>non-blocking</code> to prioritize task
						availability over logging. To continue using the <code>blocking</code> mode
						after this change, do one of the following:</p>
                  <ul>
                     <li>
                        <p>Set the <code>mode</code> option in your container definition's
									<code>logConfiguration</code> as <code>blocking</code>.</p>
                     </li>
                     <li>
                        <p>Set the <code>defaultLogDriverMode</code> account setting to
									<code>blocking</code>.</p>
                     </li>
                  </ul>
               </note>
            </li>
            <li>
               <p>
                  <code>guardDutyActivate</code> - The <code>guardDutyActivate</code> parameter
					is read-only in Amazon ECS and indicates whether Amazon ECS Runtime Monitoring
					is enabled or disabled by your security administrator in your Amazon ECS
					account. Amazon GuardDuty controls this account setting on your behalf. For more
					information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-guard-duty-integration.html">Protecting Amazon ECS workloads with Amazon ECS Runtime
					Monitoring</a>.</p>
            </li>
         </ul> |
| `value` | String | ✅ | <p>The account setting value for the specified principal ARN. Accepted values are
				<code>enabled</code>, <code>disabled</code>, <code>on</code>, <code>enhanced</code>,
			and <code>off</code>.</p>
         <p>When you specify <code>fargateTaskRetirementWaitPeriod</code> for the
				<code>name</code>, the following are the valid values:</p>
         <ul>
            <li>
               <p>
                  <code>0</code> - Amazon Web Services sends the notification, and
					immediately retires the affected tasks.</p>
            </li>
            <li>
               <p>
                  <code>7</code> - Amazon Web Services sends the notification, and waits 7
					calendar days to retire the tasks.</p>
            </li>
            <li>
               <p>
                  <code>14</code> - Amazon Web Services sends the notification, and waits 14
					calendar days to retire the tasks.</p>
            </li>
         </ul> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create account_setting_default
account_setting_default = provider.ecs.Account_setting_default {
    name = "value"  # <p>The resource name for which to modify the account setting.</p>
         <p>The following are the valid values for the account setting name.</p>
         <ul>
            <li>
               <p>
                  <code>serviceLongArnFormat</code> - When modified, the Amazon Resource Name
					(ARN) and resource ID format of the resource type for a specified user, role, or
					the root user for an account is affected. The opt-in and opt-out account setting
					must be set for each Amazon ECS resource separately. The ARN and resource ID
					format of a resource is defined by the opt-in status of the user or role that
					created the resource. You must turn on this setting to use Amazon ECS features
					such as resource tagging.</p>
            </li>
            <li>
               <p>
                  <code>taskLongArnFormat</code> - When modified, the Amazon Resource Name (ARN)
					and resource ID format of the resource type for a specified user, role, or the
					root user for an account is affected. The opt-in and opt-out account setting
					must be set for each Amazon ECS resource separately. The ARN and resource ID
					format of a resource is defined by the opt-in status of the user or role that
					created the resource. You must turn on this setting to use Amazon ECS features
					such as resource tagging.</p>
            </li>
            <li>
               <p>
                  <code>containerInstanceLongArnFormat</code> - When modified, the Amazon
					Resource Name (ARN) and resource ID format of the resource type for a specified
					user, role, or the root user for an account is affected. The opt-in and opt-out
					account setting must be set for each Amazon ECS resource separately. The ARN and
					resource ID format of a resource is defined by the opt-in status of the user or
					role that created the resource. You must turn on this setting to use Amazon ECS
					features such as resource tagging.</p>
            </li>
            <li>
               <p>
                  <code>awsvpcTrunking</code> - When modified, the elastic network interface
					(ENI) limit for any new container instances that support the feature is changed.
					If <code>awsvpcTrunking</code> is turned on, any new container instances that
					support the feature are launched have the increased ENI limits available to
					them. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/container-instance-eni.html">Elastic
						Network Interface Trunking</a> in the <i>Amazon Elastic Container
						Service Developer Guide</i>.</p>
            </li>
            <li>
               <p>
                  <code>containerInsights</code> - Container Insights with enhanced
					observability provides all the Container Insights metrics, plus additional task
					and container metrics. This version supports enhanced observability for Amazon
					ECS clusters using the Amazon EC2 and Fargate launch types. After you configure
					Container Insights with enhanced observability on Amazon ECS, Container Insights
					auto-collects detailed infrastructure telemetry from the cluster level down to
					the container level in your environment and displays these critical performance
					data in curated dashboards removing the heavy lifting in observability set-up. </p>
               <p>To use Container Insights with enhanced observability, set the
						<code>containerInsights</code> account setting to
					<code>enhanced</code>.</p>
               <p>To use Container Insights, set the <code>containerInsights</code> account
					setting to <code>enabled</code>.</p>
               <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/cloudwatch-container-insights.html">Monitor Amazon ECS containers using Container Insights with enhanced
						observability</a> in the <i>Amazon Elastic Container Service
						Developer Guide</i>.</p>
            </li>
            <li>
               <p>
                  <code>dualStackIPv6</code> - When turned on, when using a VPC in dual stack
					mode, your tasks using the <code>awsvpc</code> network mode can have an IPv6
					address assigned. For more information on using IPv6 with tasks launched on
					Amazon EC2 instances, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-networking-awsvpc.html#task-networking-vpc-dual-stack">Using a VPC in dual-stack mode</a>. For more information on using IPv6
					with tasks launched on Fargate, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/fargate-task-networking.html#fargate-task-networking-vpc-dual-stack">Using a VPC in dual-stack mode</a>.</p>
            </li>
            <li>
               <p>
                  <code>fargateFIPSMode</code> - If you specify <code>fargateFIPSMode</code>,
						Fargate FIPS 140 compliance is affected.</p>
            </li>
            <li>
               <p>
                  <code>fargateTaskRetirementWaitPeriod</code> - When Amazon Web Services
					determines that a security or infrastructure update is needed for an Amazon ECS
					task hosted on Fargate, the tasks need to be stopped and new tasks launched to
					replace them. Use <code>fargateTaskRetirementWaitPeriod</code> to configure the
					wait time to retire a Fargate task. For information about the Fargate tasks
					maintenance, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-maintenance.html">Amazon Web
							Services Fargate task maintenance</a> in the
						<i>Amazon ECS Developer Guide</i>.</p>
            </li>
            <li>
               <p>
                  <code>tagResourceAuthorization</code> - Amazon ECS is introducing tagging
					authorization for resource creation. Users must have permissions for actions
					that create the resource, such as <code>ecsCreateCluster</code>. If tags are
					specified when you create a resource, Amazon Web Services performs
					additional authorization to verify if users or roles have permissions to create
					tags. Therefore, you must grant explicit permissions to use the
						<code>ecs:TagResource</code> action. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/supported-iam-actions-tagging.html">Grant permission to tag resources on creation</a> in the
						<i>Amazon ECS Developer Guide</i>.</p>
            </li>
            <li>
               <p>
                  <code>defaultLogDriverMode</code> -Amazon ECS supports setting a default
					delivery mode of log messages from a container to the <code>logDriver</code>
					that you specify in the container's <code>logConfiguration</code>. The delivery
					mode affects application stability when the flow of logs from the container to
					the log driver is interrupted. The <code>defaultLogDriverMode</code> setting
					supports two values: <code>blocking</code> and <code>non-blocking</code>. If you
					don't specify a delivery mode in your container definition's
						<code>logConfiguration</code>, the mode you specify using this account
					setting will be used as the default. For more information about log delivery
					modes, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_LogConfiguration.html">LogConfiguration</a>.</p>
               <note>
                  <p>On June 25, 2025, Amazon ECS changed the default log driver mode from
							<code>blocking</code> to <code>non-blocking</code> to prioritize task
						availability over logging. To continue using the <code>blocking</code> mode
						after this change, do one of the following:</p>
                  <ul>
                     <li>
                        <p>Set the <code>mode</code> option in your container definition's
									<code>logConfiguration</code> as <code>blocking</code>.</p>
                     </li>
                     <li>
                        <p>Set the <code>defaultLogDriverMode</code> account setting to
									<code>blocking</code>.</p>
                     </li>
                  </ul>
               </note>
            </li>
            <li>
               <p>
                  <code>guardDutyActivate</code> - The <code>guardDutyActivate</code> parameter
					is read-only in Amazon ECS and indicates whether Amazon ECS Runtime Monitoring
					is enabled or disabled by your security administrator in your Amazon ECS
					account. Amazon GuardDuty controls this account setting on your behalf. For more
					information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-guard-duty-integration.html">Protecting Amazon ECS workloads with Amazon ECS Runtime
					Monitoring</a>.</p>
            </li>
         </ul>
    value = "value"  # <p>The account setting value for the specified principal ARN. Accepted values are
				<code>enabled</code>, <code>disabled</code>, <code>on</code>, <code>enhanced</code>,
			and <code>off</code>.</p>
         <p>When you specify <code>fargateTaskRetirementWaitPeriod</code> for the
				<code>name</code>, the following are the valid values:</p>
         <ul>
            <li>
               <p>
                  <code>0</code> - Amazon Web Services sends the notification, and
					immediately retires the affected tasks.</p>
            </li>
            <li>
               <p>
                  <code>7</code> - Amazon Web Services sends the notification, and waits 7
					calendar days to retire the tasks.</p>
            </li>
            <li>
               <p>
                  <code>14</code> - Amazon Web Services sends the notification, and waits 14
					calendar days to retire the tasks.</p>
            </li>
         </ul>
}

```

---


### Attributes

Attributes resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `cluster` | String |  | <p>The short name or full Amazon Resource Name (ARN) of the cluster that contains the
			resource to apply attributes. If you do not specify a cluster, the default cluster is
			assumed.</p> |
| `attributes` | Vec<String> | ✅ | <p>The attributes to apply to your resource. You can specify up to 10 custom attributes
			for each resource. You can specify up to 10 attributes in a single call.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create attributes
attributes = provider.ecs.Attributes {
    attributes = "value"  # <p>The attributes to apply to your resource. You can specify up to 10 custom attributes
			for each resource. You can specify up to 10 attributes in a single call.</p>
}

```

---


### Service_primary_task_set

ServicePrimaryTaskSet resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `primary_task_set` | String | ✅ | <p>The short name or full Amazon Resource Name (ARN) of the task set to set as the
			primary task set in the deployment.</p> |
| `service` | String | ✅ | <p>The short name or full Amazon Resource Name (ARN) of the service that the task set
			exists in.</p> |
| `cluster` | String | ✅ | <p>The short name or full Amazon Resource Name (ARN) of the cluster that hosts the
			service that the task set exists in.</p> |



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


### Task_definitions

TaskDefinitions resource

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



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple service_deployments resources
service_deployments_0 = provider.ecs.Service_deployments {
}
service_deployments_1 = provider.ecs.Service_deployments {
}
service_deployments_2 = provider.ecs.Service_deployments {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    service_deployments = provider.ecs.Service_deployments {
    }
```

---

## Related Documentation

- [AWS Ecs Documentation](https://docs.aws.amazon.com/ecs/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
