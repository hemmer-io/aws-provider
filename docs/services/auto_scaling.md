# Auto_scaling Service



**Resources**: 31

---

## Overview

The auto_scaling service provides access to 31 resource types:

- [Instance_refreshes](#instance_refreshes) [R]
- [Termination_policy_types](#termination_policy_types) [R]
- [Scheduled_actions](#scheduled_actions) [R]
- [Notification_configuration](#notification_configuration) [CD]
- [Lifecycle_hooks](#lifecycle_hooks) [R]
- [Or_update_tags](#or_update_tags) [C]
- [Load_balancer_target_groups](#load_balancer_target_groups) [R]
- [Scaling_activities](#scaling_activities) [R]
- [Launch_configurations](#launch_configurations) [R]
- [Scheduled_update_group_action](#scheduled_update_group_action) [C]
- [Adjustment_types](#adjustment_types) [R]
- [Warm_pool](#warm_pool) [CRD]
- [Metric_collection_types](#metric_collection_types) [R]
- [Launch_configuration](#launch_configuration) [CD]
- [Policy](#policy) [D]
- [Load_balancers](#load_balancers) [R]
- [Lifecycle_hook](#lifecycle_hook) [CD]
- [Tags](#tags) [RD]
- [Policies](#policies) [R]
- [Traffic_sources](#traffic_sources) [R]
- [Auto_scaling_groups](#auto_scaling_groups) [R]
- [Predictive_scaling_forecast](#predictive_scaling_forecast) [R]
- [Scaling_policy](#scaling_policy) [C]
- [Notification_configurations](#notification_configurations) [R]
- [Lifecycle_hook_types](#lifecycle_hook_types) [R]
- [Scaling_process_types](#scaling_process_types) [R]
- [Account_limits](#account_limits) [R]
- [Auto_scaling_notification_types](#auto_scaling_notification_types) [R]
- [Auto_scaling_group](#auto_scaling_group) [CUD]
- [Scheduled_action](#scheduled_action) [D]
- [Auto_scaling_instances](#auto_scaling_instances) [R]

---

## Resources


### Instance_refreshes

InstanceRefreshes resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `instance_refreshes` | Vec<String> | <p>The instance refreshes for the specified group, sorted by creation timestamp in
            descending order.</p> |
| `next_token` | String | <p>A string that indicates that the response contains more items than can be returned in
            a single response. To receive additional items, specify this string for the
                <code>NextToken</code> value when requesting the next set of items. This value is
            null when there are no more items to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access instance_refreshes outputs
instance_refreshes_id = instance_refreshes.id
instance_refreshes_instance_refreshes = instance_refreshes.instance_refreshes
instance_refreshes_next_token = instance_refreshes.next_token
```

---


### Termination_policy_types

TerminationPolicyTypes resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `termination_policy_types` | Vec<String> | <p>The termination policies supported by Amazon EC2 Auto Scaling: <code>OldestInstance</code>,
                <code>OldestLaunchConfiguration</code>, <code>NewestInstance</code>,
                <code>ClosestToNextInstanceHour</code>, <code>Default</code>,
                <code>OldestLaunchTemplate</code>, and <code>AllocationStrategy</code>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access termination_policy_types outputs
termination_policy_types_id = termination_policy_types.id
termination_policy_types_termination_policy_types = termination_policy_types.termination_policy_types
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
| `scheduled_update_group_actions` | Vec<String> | <p>The scheduled actions.</p> |
| `next_token` | String | <p>A string that indicates that the response contains more items than can be returned in
            a single response. To receive additional items, specify this string for the
                <code>NextToken</code> value when requesting the next set of items. This value is
            null when there are no more items to return.</p> |


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
scheduled_actions_scheduled_update_group_actions = scheduled_actions.scheduled_update_group_actions
scheduled_actions_next_token = scheduled_actions.next_token
```

---


### Notification_configuration

NotificationConfiguration resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `notification_types` | Vec<String> | ✅ | <p>The type of event that causes the notification to be sent. To query the notification
            types supported by Amazon EC2 Auto Scaling, call the <a href="https://docs.aws.amazon.com/autoscaling/ec2/APIReference/API_DescribeAutoScalingNotificationTypes.html">DescribeAutoScalingNotificationTypes</a> API.</p> |
| `auto_scaling_group_name` | String | ✅ | <p>The name of the Auto Scaling group.</p> |
| `topic_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the Amazon SNS topic.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create notification_configuration
notification_configuration = provider.auto_scaling.Notification_configuration {
    notification_types = "value"  # <p>The type of event that causes the notification to be sent. To query the notification
            types supported by Amazon EC2 Auto Scaling, call the <a href="https://docs.aws.amazon.com/autoscaling/ec2/APIReference/API_DescribeAutoScalingNotificationTypes.html">DescribeAutoScalingNotificationTypes</a> API.</p>
    auto_scaling_group_name = "value"  # <p>The name of the Auto Scaling group.</p>
    topic_arn = "value"  # <p>The Amazon Resource Name (ARN) of the Amazon SNS topic.</p>
}

```

---


### Lifecycle_hooks

LifecycleHooks resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `lifecycle_hooks` | Vec<String> | <p>The lifecycle hooks for the specified group.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access lifecycle_hooks outputs
lifecycle_hooks_id = lifecycle_hooks.id
lifecycle_hooks_lifecycle_hooks = lifecycle_hooks.lifecycle_hooks
```

---


### Or_update_tags

OrUpdateTags resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> | ✅ | <p>One or more tags.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create or_update_tags
or_update_tags = provider.auto_scaling.Or_update_tags {
    tags = "value"  # <p>One or more tags.</p>
}

```

---


### Load_balancer_target_groups

LoadBalancerTargetGroups resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `load_balancer_target_groups` | Vec<String> | <p>Information about the target groups.</p> |
| `next_token` | String | <p>A string that indicates that the response contains more items than can be returned in
            a single response. To receive additional items, specify this string for the
                <code>NextToken</code> value when requesting the next set of items. This value is
            null when there are no more items to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access load_balancer_target_groups outputs
load_balancer_target_groups_id = load_balancer_target_groups.id
load_balancer_target_groups_load_balancer_target_groups = load_balancer_target_groups.load_balancer_target_groups
load_balancer_target_groups_next_token = load_balancer_target_groups.next_token
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
| `activities` | Vec<String> | <p>The scaling activities. Activities are sorted by start time. Activities still in
            progress are described first.</p> |
| `next_token` | String | <p>A string that indicates that the response contains more items than can be returned in
            a single response. To receive additional items, specify this string for the
                <code>NextToken</code> value when requesting the next set of items. This value is
            null when there are no more items to return.</p> |


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
scaling_activities_activities = scaling_activities.activities
scaling_activities_next_token = scaling_activities.next_token
```

---


### Launch_configurations

LaunchConfigurations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `launch_configurations` | Vec<String> | <p>The launch configurations.</p> |
| `next_token` | String | <p>A string that indicates that the response contains more items than can be returned in
            a single response. To receive additional items, specify this string for the
                <code>NextToken</code> value when requesting the next set of items. This value is
            null when there are no more items to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access launch_configurations outputs
launch_configurations_id = launch_configurations.id
launch_configurations_launch_configurations = launch_configurations.launch_configurations
launch_configurations_next_token = launch_configurations.next_token
```

---


### Scheduled_update_group_action

ScheduledUpdateGroupAction resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `max_size` | i64 |  | <p>The maximum size of the Auto Scaling group.</p> |
| `start_time` | String |  | <p>The date and time for this action to start, in YYYY-MM-DDThh:mm:ssZ format in UTC/GMT
            only and in quotes (for example, <code>"2021-06-01T00:00:00Z"</code>).</p>
         <p>If you specify <code>Recurrence</code> and <code>StartTime</code>, Amazon EC2 Auto Scaling performs
            the action at this time, and then performs the action based on the specified
            recurrence.</p> |
| `auto_scaling_group_name` | String | ✅ | <p>The name of the Auto Scaling group.</p> |
| `scheduled_action_name` | String | ✅ | <p>The name of this scaling action.</p> |
| `end_time` | String |  | <p>The date and time for the recurring schedule to end, in UTC. For example,
                <code>"2021-06-01T00:00:00Z"</code>.</p> |
| `desired_capacity` | i64 |  | <p>The desired capacity is the initial capacity of the Auto Scaling group after the scheduled
            action runs and the capacity it attempts to maintain. It can scale beyond this capacity
            if you add more scaling conditions. </p>
         <note>
            <p>You must specify at least one of the following properties: <code>MaxSize</code>,
                    <code>MinSize</code>, or <code>DesiredCapacity</code>. </p>
         </note> |
| `time_zone` | String |  | <p>Specifies the time zone for a cron expression. If a time zone is not provided, UTC is
            used by default. </p>
         <p>Valid values are the canonical names of the IANA time zones, derived from the IANA
            Time Zone Database (such as <code>Etc/GMT+9</code> or <code>Pacific/Tahiti</code>). For
            more information, see <a href="https://en.wikipedia.org/wiki/List_of_tz_database_time_zones">https://en.wikipedia.org/wiki/List_of_tz_database_time_zones</a>.</p> |
| `time` | String |  | <p>This property is no longer used.</p> |
| `recurrence` | String |  | <p>The recurring schedule for this action. This format consists of five fields separated
            by white spaces: [Minute] [Hour] [Day_of_Month] [Month_of_Year] [Day_of_Week]. The value
            must be in quotes (for example, <code>"30 0 1 1,6,12 *"</code>). For more information
            about this format, see <a href="http://crontab.org">Crontab</a>.</p>
         <p>When <code>StartTime</code> and <code>EndTime</code> are specified with
                <code>Recurrence</code>, they form the boundaries of when the recurring action
            starts and stops.</p>
         <p>Cron expressions use Universal Coordinated Time (UTC) by default.</p> |
| `min_size` | i64 |  | <p>The minimum size of the Auto Scaling group.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create scheduled_update_group_action
scheduled_update_group_action = provider.auto_scaling.Scheduled_update_group_action {
    auto_scaling_group_name = "value"  # <p>The name of the Auto Scaling group.</p>
    scheduled_action_name = "value"  # <p>The name of this scaling action.</p>
}

```

---


### Adjustment_types

AdjustmentTypes resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `adjustment_types` | Vec<String> | <p>The policy adjustment types.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access adjustment_types outputs
adjustment_types_id = adjustment_types.id
adjustment_types_adjustment_types = adjustment_types.adjustment_types
```

---


### Warm_pool

WarmPool resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `max_group_prepared_capacity` | i64 |  | <p>Specifies the maximum number of instances that are allowed to be in the warm pool or
            in any state except <code>Terminated</code> for the Auto Scaling group. This is an optional
            property. Specify it only if you do not want the warm pool size to be determined by the
            difference between the group's maximum capacity and its desired capacity. </p>
         <important>
            <p>If a value for <code>MaxGroupPreparedCapacity</code> is not specified, Amazon EC2 Auto Scaling
                launches and maintains the difference between the group's maximum capacity and its
                desired capacity. If you specify a value for <code>MaxGroupPreparedCapacity</code>,
                Amazon EC2 Auto Scaling uses the difference between the <code>MaxGroupPreparedCapacity</code> and
                the desired capacity instead. </p>
            <p>The size of the warm pool is dynamic. Only when
                    <code>MaxGroupPreparedCapacity</code> and <code>MinSize</code> are set to the
                same value does the warm pool have an absolute size.</p>
         </important>
         <p>If the desired capacity of the Auto Scaling group is higher than the
                <code>MaxGroupPreparedCapacity</code>, the capacity of the warm pool is 0, unless
            you specify a value for <code>MinSize</code>. To remove a value that you previously set,
            include the property but specify -1 for the value. </p> |
| `pool_state` | String |  | <p>Sets the instance state to transition to after the lifecycle actions are complete.
            Default is <code>Stopped</code>.</p> |
| `instance_reuse_policy` | String |  | <p>Indicates whether instances in the Auto Scaling group can be returned to the warm pool on
            scale in. The default is to terminate instances in the Auto Scaling group when the group scales
            in.</p> |
| `auto_scaling_group_name` | String | ✅ | <p>The name of the Auto Scaling group.</p> |
| `min_size` | i64 |  | <p>Specifies the minimum number of instances to maintain in the warm pool. This helps you
            to ensure that there is always a certain number of warmed instances available to handle
            traffic spikes. Defaults to 0 if not specified.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `warm_pool_configuration` | String | <p>The warm pool configuration details. </p> |
| `instances` | Vec<String> | <p>The instances that are currently in the warm pool.</p> |
| `next_token` | String | <p>This string indicates that the response contains more items than can be returned in a
            single response. To receive additional items, specify this string for the
                <code>NextToken</code> value when requesting the next set of items. This value is
            null when there are no more items to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create warm_pool
warm_pool = provider.auto_scaling.Warm_pool {
    auto_scaling_group_name = "value"  # <p>The name of the Auto Scaling group.</p>
}

# Access warm_pool outputs
warm_pool_id = warm_pool.id
warm_pool_warm_pool_configuration = warm_pool.warm_pool_configuration
warm_pool_instances = warm_pool.instances
warm_pool_next_token = warm_pool.next_token
```

---


### Metric_collection_types

MetricCollectionTypes resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `metrics` | Vec<String> | <p>The metrics.</p> |
| `granularities` | Vec<String> | <p>The granularities for the metrics.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access metric_collection_types outputs
metric_collection_types_id = metric_collection_types.id
metric_collection_types_metrics = metric_collection_types.metrics
metric_collection_types_granularities = metric_collection_types.granularities
```

---


### Launch_configuration

LaunchConfiguration resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `key_name` | String |  | <p>The name of the key pair. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-key-pairs.html">Amazon EC2 key pairs and Amazon EC2
                instances</a> in the <i>Amazon EC2 User Guide</i>.</p> |
| `classic_link_vpc_security_groups` | Vec<String> |  | <p>Available for backward compatibility.</p> |
| `instance_id` | String |  | <p>The ID of the instance to use to create the launch configuration. The new launch
            configuration derives attributes from the instance, except for the block device
            mapping.</p>
         <p>To create a launch configuration with a block device mapping or override any other
            instance attributes, specify them as part of the same request.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/create-launch-config.html">Create a launch
                configuration</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p> |
| `security_groups` | Vec<String> |  | <p>A list that contains the security group IDs to assign to the instances in the Auto Scaling
            group. For more information, see <a href="https://docs.aws.amazon.com/vpc/latest/userguide/vpc-security-groups.html">Control traffic to your Amazon Web Services
                resources using security groups</a> in the <i>Amazon Virtual Private
                Cloud User Guide</i>.</p> |
| `classic_link_vpc_id` | String |  | <p>Available for backward compatibility.</p> |
| `kernel_id` | String |  | <p>The ID of the kernel associated with the AMI.</p>
         <note>
            <p>We recommend that you use PV-GRUB instead of kernels and RAM disks. For more
                information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/UserProvidedKernels.html">User provided
                    kernels</a> in the <i>Amazon EC2 User Guide</i>.</p>
         </note> |
| `image_id` | String |  | <p>The ID of the Amazon Machine Image (AMI) that was assigned during registration. For
            more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/finding-an-ami.html">Find a Linux AMI</a> in the
                <i>Amazon EC2 User Guide</i>.</p>
         <p>If you specify <code>InstanceId</code>, an <code>ImageId</code> is not
            required.</p> |
| `instance_type` | String |  | <p>Specifies the instance type of the EC2 instance. For information about available
            instance types, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instance-types.html#AvailableInstanceTypes">Available
                instance types</a> in the <i>Amazon EC2 User Guide</i>.</p>
         <p>If you specify <code>InstanceId</code>, an <code>InstanceType</code> is not
            required.</p> |
| `ramdisk_id` | String |  | <p>The ID of the RAM disk to select.</p>
         <note>
            <p>We recommend that you use PV-GRUB instead of kernels and RAM disks. For more
                information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/UserProvidedKernels.html">User provided
                    kernels</a> in the <i>Amazon EC2 User Guide</i>.</p>
         </note> |
| `iam_instance_profile` | String |  | <p>The name or the Amazon Resource Name (ARN) of the instance profile associated with the
            IAM role for the instance. The instance profile contains the IAM role. For more
            information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/us-iam-role.html">IAM role for applications that run
                on Amazon EC2 instances</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p> |
| `instance_monitoring` | String |  | <p>Controls whether instances in this group are launched with detailed
            (<code>true</code>) or basic (<code>false</code>) monitoring.</p>
         <p>The default value is <code>true</code> (enabled).</p>
         <important>
            <p>When detailed monitoring is enabled, Amazon CloudWatch generates metrics every minute and
                your account is charged a fee. When you disable detailed monitoring, CloudWatch generates
                metrics every 5 minutes. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/latest/userguide/enable-as-instance-metrics.html">Configure
                    monitoring for Auto Scaling instances</a> in the
                <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
         </important> |
| `associate_public_ip_address` | bool |  | <p>Specifies whether to assign a public IPv4 address to the group's instances. If the
            instance is launched into a default subnet, the default is to assign a public IPv4
            address, unless you disabled the option to assign a public IPv4 address on the subnet.
            If the instance is launched into a nondefault subnet, the default is not to assign a
            public IPv4 address, unless you enabled the option to assign a public IPv4 address on
            the subnet.</p>
         <p>If you specify <code>true</code>, each instance in the Auto Scaling group receives a unique
            public IPv4 address. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/asg-in-vpc.html">Provide network connectivity for
                your Auto Scaling instances using Amazon VPC</a> in the
            <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
         <p>If you specify this property, you must specify at least one subnet for
                <code>VPCZoneIdentifier</code> when you create your group.</p> |
| `spot_price` | String |  | <p>The maximum hourly price to be paid for any Spot Instance launched to fulfill the
            request. Spot Instances are launched when the price you specify exceeds the current Spot
            price. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/launch-template-spot-instances.html">Request Spot
                Instances for fault-tolerant and flexible applications</a> in the
                <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
         <p>Valid Range: Minimum value of 0.001</p>
         <note>
            <p>When you change your maximum price by creating a new launch configuration, running
                instances will continue to run as long as the maximum price for those running
                instances is higher than the current Spot price.</p>
         </note> |
| `placement_tenancy` | String |  | <p>The tenancy of the instance, either <code>default</code> or <code>dedicated</code>. An
            instance with <code>dedicated</code> tenancy runs on isolated, single-tenant hardware
            and can only be launched into a VPC. To launch dedicated instances into a shared tenancy
            VPC (a VPC with the instance placement tenancy attribute set to <code>default</code>),
            you must set the value of this property to <code>dedicated</code>.</p>
         <p>If you specify <code>PlacementTenancy</code>, you must specify at least one subnet for
                <code>VPCZoneIdentifier</code> when you create your group.</p>
         <p>Valid values: <code>default</code> | <code>dedicated</code>
         </p> |
| `block_device_mappings` | Vec<String> |  | <p>The block device mapping entries that define the block devices to attach to the
            instances at launch. By default, the block devices specified in the block device mapping
            for the AMI are used. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/block-device-mapping-concepts.html">Block device
                mappings</a> in the <i>Amazon EC2 User Guide</i>.</p> |
| `metadata_options` | String |  | <p>The metadata options for the instances. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/create-launch-config.html#launch-configurations-imds">Configure the instance metadata options</a> in the
                <i>Amazon EC2 Auto Scaling User Guide</i>.</p> |
| `user_data` | String |  | <p>The user data to make available to the launched EC2 instances. For more information,
            see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-instance-metadata.html">Instance metadata and user data</a> (Linux) and <a href="https://docs.aws.amazon.com/AWSEC2/latest/WindowsGuide/ec2-instance-metadata.html">Instance metadata and
                user data</a> (Windows). If you are using a command line tool, base64-encoding
            is performed for you, and you can load the text from a file. Otherwise, you must provide
            base64-encoded text. User data is limited to 16 KB.</p> |
| `ebs_optimized` | bool |  | <p>Specifies whether the launch configuration is optimized for EBS I/O
            (<code>true</code>) or not (<code>false</code>). The optimization provides dedicated
            throughput to Amazon EBS and an optimized configuration stack to provide optimal I/O
            performance. This optimization is not available with all instance types. Additional fees
            are incurred when you enable EBS optimization for an instance type that is not
            EBS-optimized by default. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ebs-optimized.html">Amazon EBS-optimized instances</a>
            in the <i>Amazon EC2 User Guide</i>.</p>
         <p>The default value is <code>false</code>.</p> |
| `launch_configuration_name` | String | ✅ | <p>The name of the launch configuration. This name must be unique per Region per
            account.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create launch_configuration
launch_configuration = provider.auto_scaling.Launch_configuration {
    launch_configuration_name = "value"  # <p>The name of the launch configuration. This name must be unique per Region per
            account.</p>
}

```

---


### Policy

Policy resource

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


### Load_balancers

LoadBalancers resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>A string that indicates that the response contains more items than can be returned in
            a single response. To receive additional items, specify this string for the
                <code>NextToken</code> value when requesting the next set of items. This value is
            null when there are no more items to return.</p> |
| `load_balancers` | Vec<String> | <p>The load balancers.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access load_balancers outputs
load_balancers_id = load_balancers.id
load_balancers_next_token = load_balancers.next_token
load_balancers_load_balancers = load_balancers.load_balancers
```

---


### Lifecycle_hook

LifecycleHook resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `auto_scaling_group_name` | String | ✅ | <p>The name of the Auto Scaling group.</p> |
| `lifecycle_hook_name` | String | ✅ | <p>The name of the lifecycle hook.</p> |
| `role_arn` | String |  | <p>The ARN of the IAM role that allows the Auto Scaling group to publish to the specified
            notification target.</p>
         <p>Valid only if the notification target is an Amazon SNS topic or an Amazon SQS queue. Required
            for new lifecycle hooks, but optional when updating existing hooks.</p> |
| `notification_metadata` | String |  | <p>Additional information that you want to include any time Amazon EC2 Auto Scaling sends a message to
            the notification target.</p> |
| `heartbeat_timeout` | i64 |  | <p>The maximum time, in seconds, that can elapse before the lifecycle hook times out. The
            range is from <code>30</code> to <code>7200</code> seconds. The default value is
                <code>3600</code> seconds (1 hour).</p> |
| `lifecycle_transition` | String |  | <p>The lifecycle transition. For Auto Scaling groups, there are two major lifecycle
            transitions.</p>
         <ul>
            <li>
               <p>To create a lifecycle hook for scale-out events, specify
                        <code>autoscaling:EC2_INSTANCE_LAUNCHING</code>.</p>
            </li>
            <li>
               <p>To create a lifecycle hook for scale-in events, specify
                        <code>autoscaling:EC2_INSTANCE_TERMINATING</code>.</p>
            </li>
         </ul>
         <p>Required for new lifecycle hooks, but optional when updating existing hooks.</p> |
| `default_result` | String |  | <p>The action the Auto Scaling group takes when the lifecycle hook timeout elapses or if an
            unexpected failure occurs. The default value is <code>ABANDON</code>.</p>
         <p>Valid values: <code>CONTINUE</code> | <code>ABANDON</code>
         </p> |
| `notification_target_arn` | String |  | <p>The Amazon Resource Name (ARN) of the notification target that Amazon EC2 Auto Scaling uses to notify
            you when an instance is in a wait state for the lifecycle hook. You can specify either
            an Amazon SNS topic or an Amazon SQS queue.</p>
         <p>If you specify an empty string, this overrides the current ARN.</p>
         <p>This operation uses the JSON format when sending notifications to an Amazon SQS queue, and
            an email key-value pair format when sending notifications to an Amazon SNS topic.</p>
         <p>When you specify a notification target, Amazon EC2 Auto Scaling sends it a test message. Test
            messages contain the following additional key-value pair: <code>"Event":
                "autoscaling:TEST_NOTIFICATION"</code>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create lifecycle_hook
lifecycle_hook = provider.auto_scaling.Lifecycle_hook {
    auto_scaling_group_name = "value"  # <p>The name of the Auto Scaling group.</p>
    lifecycle_hook_name = "value"  # <p>The name of the lifecycle hook.</p>
}

```

---


### Tags

Tags resource

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>A string that indicates that the response contains more items than can be returned in
            a single response. To receive additional items, specify this string for the
                <code>NextToken</code> value when requesting the next set of items. This value is
            null when there are no more items to return.</p> |
| `tags` | Vec<String> | <p>One or more tags.</p> |


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
tags_next_token = tags.next_token
tags_tags = tags.tags
```

---


### Policies

Policies resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `scaling_policies` | Vec<String> | <p>The scaling policies.</p> |
| `next_token` | String | <p>A string that indicates that the response contains more items than can be returned in
            a single response. To receive additional items, specify this string for the
                <code>NextToken</code> value when requesting the next set of items. This value is
            null when there are no more items to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access policies outputs
policies_id = policies.id
policies_scaling_policies = policies.scaling_policies
policies_next_token = policies.next_token
```

---


### Traffic_sources

TrafficSources resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>This string indicates that the response contains more items than can be returned in a
            single response. To receive additional items, specify this string for the
                <code>NextToken</code> value when requesting the next set of items. This value is
            null when there are no more items to return.</p> |
| `traffic_sources` | Vec<String> | <p>Information about the traffic sources.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access traffic_sources outputs
traffic_sources_id = traffic_sources.id
traffic_sources_next_token = traffic_sources.next_token
traffic_sources_traffic_sources = traffic_sources.traffic_sources
```

---


### Auto_scaling_groups

AutoScalingGroups resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `auto_scaling_groups` | Vec<String> | <p>The groups.</p> |
| `next_token` | String | <p>A string that indicates that the response contains more items than can be returned in
            a single response. To receive additional items, specify this string for the
                <code>NextToken</code> value when requesting the next set of items. This value is
            null when there are no more items to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access auto_scaling_groups outputs
auto_scaling_groups_id = auto_scaling_groups.id
auto_scaling_groups_auto_scaling_groups = auto_scaling_groups.auto_scaling_groups
auto_scaling_groups_next_token = auto_scaling_groups.next_token
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
| `load_forecast` | Vec<String> | <p>The load forecast.</p> |
| `update_time` | String | <p>The time the forecast was made.</p> |
| `capacity_forecast` | String | <p>The capacity forecast.</p> |


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
predictive_scaling_forecast_update_time = predictive_scaling_forecast.update_time
predictive_scaling_forecast_capacity_forecast = predictive_scaling_forecast.capacity_forecast
```

---


### Scaling_policy

ScalingPolicy resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `scaling_adjustment` | i64 |  | <p>The amount by which to scale, based on the specified adjustment type. A positive value
            adds to the current capacity while a negative number removes from the current capacity.
            For exact capacity, you must specify a non-negative value.</p>
         <p>Required if the policy type is <code>SimpleScaling</code>. (Not used with any other
            policy type.) </p> |
| `min_adjustment_step` | i64 |  | <p>Available for backward compatibility. Use <code>MinAdjustmentMagnitude</code>
            instead.</p> |
| `enabled` | bool |  | <p>Indicates whether the scaling policy is enabled or disabled. The default is enabled.
            For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-enable-disable-scaling-policy.html">Disable a
                scaling policy for an Auto Scaling group</a> in the
            <i>Amazon EC2 Auto Scaling User Guide</i>.</p> |
| `auto_scaling_group_name` | String | ✅ | <p>The name of the Auto Scaling group.</p> |
| `cooldown` | i64 |  | <p>A cooldown period, in seconds, that applies to a specific simple scaling policy. When
            a cooldown period is specified here, it overrides the default cooldown.</p>
         <p>Valid only if the policy type is <code>SimpleScaling</code>. For more information, see
                <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/ec2-auto-scaling-scaling-cooldowns.html">Scaling
                cooldowns for Amazon EC2 Auto Scaling</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
         <p>Default: None</p> |
| `adjustment_type` | String |  | <p>Specifies how the scaling adjustment is interpreted (for example, an absolute number
            or a percentage). The valid values are <code>ChangeInCapacity</code>,
                <code>ExactCapacity</code>, and <code>PercentChangeInCapacity</code>.</p>
         <p>Required if the policy type is <code>StepScaling</code> or <code>SimpleScaling</code>.
            For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-scaling-simple-step.html#as-scaling-adjustment">Scaling adjustment types</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p> |
| `metric_aggregation_type` | String |  | <p>The aggregation type for the CloudWatch metrics. The valid values are <code>Minimum</code>,
                <code>Maximum</code>, and <code>Average</code>. If the aggregation type is null, the
            value is treated as <code>Average</code>.</p>
         <p>Valid only if the policy type is <code>StepScaling</code>.</p> |
| `step_adjustments` | Vec<String> |  | <p>A set of adjustments that enable you to scale based on the size of the alarm
            breach.</p>
         <p>Required if the policy type is <code>StepScaling</code>. (Not used with any other
            policy type.) </p> |
| `estimated_instance_warmup` | i64 |  | <p>
            <i>Not needed if the default instance warmup is defined for the
                group.</i>
         </p>
         <p>The estimated time, in seconds, until a newly launched instance can contribute to the
            CloudWatch metrics. This warm-up period applies to instances launched due to a specific target
            tracking or step scaling policy. When a warm-up period is specified here, it overrides
            the default instance warmup.</p>
         <p>Valid only if the policy type is <code>TargetTrackingScaling</code> or
                <code>StepScaling</code>.</p>
         <note>
            <p>The default is to use the value for the default instance warmup defined for the
                group. If default instance warmup is null, then <code>EstimatedInstanceWarmup</code>
                falls back to the value of default cooldown.</p>
         </note> |
| `policy_name` | String | ✅ | <p>The name of the policy.</p> |
| `target_tracking_configuration` | String |  | <p>A target tracking scaling policy. Provides support for predefined or custom
            metrics.</p>
         <p>The following predefined metrics are available:</p>
         <ul>
            <li>
               <p>
                  <code>ASGAverageCPUUtilization</code>
               </p>
            </li>
            <li>
               <p>
                  <code>ASGAverageNetworkIn</code>
               </p>
            </li>
            <li>
               <p>
                  <code>ASGAverageNetworkOut</code>
               </p>
            </li>
            <li>
               <p>
                  <code>ALBRequestCountPerTarget</code>
               </p>
            </li>
         </ul>
         <p>If you specify <code>ALBRequestCountPerTarget</code> for the metric, you must specify
            the <code>ResourceLabel</code> property with the
                <code>PredefinedMetricSpecification</code>.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/APIReference/API_TargetTrackingConfiguration.html">TargetTrackingConfiguration</a> in the <i>Amazon EC2 Auto Scaling API
                Reference</i>.</p>
         <p>Required if the policy type is <code>TargetTrackingScaling</code>.</p> |
| `min_adjustment_magnitude` | i64 |  | <p>The minimum value to scale by when the adjustment type is
                <code>PercentChangeInCapacity</code>. For example, suppose that you create a step
            scaling policy to scale out an Auto Scaling group by 25 percent and you specify a
                <code>MinAdjustmentMagnitude</code> of 2. If the group has 4 instances and the
            scaling policy is performed, 25 percent of 4 is 1. However, because you specified a
                <code>MinAdjustmentMagnitude</code> of 2, Amazon EC2 Auto Scaling scales out the group by 2
            instances.</p>
         <p>Valid only if the policy type is <code>StepScaling</code> or
                <code>SimpleScaling</code>. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-scaling-simple-step.html#as-scaling-adjustment">Scaling adjustment types</a> in the <i>Amazon EC2 Auto Scaling User
            Guide</i>.</p>
         <note>
            <p>Some Auto Scaling groups use instance weights. In this case, set the
                    <code>MinAdjustmentMagnitude</code> to a value that is at least as large as your
                largest instance weight.</p>
         </note> |
| `policy_type` | String |  | <p>One of the following policy types: </p>
         <ul>
            <li>
               <p>
                  <code>TargetTrackingScaling</code>
               </p>
            </li>
            <li>
               <p>
                  <code>StepScaling</code>
               </p>
            </li>
            <li>
               <p>
                  <code>SimpleScaling</code> (default)</p>
            </li>
            <li>
               <p>
                  <code>PredictiveScaling</code>
               </p>
            </li>
         </ul> |
| `predictive_scaling_configuration` | String |  | <p>A predictive scaling policy. Provides support for predefined and custom
            metrics.</p>
         <p>Predefined metrics include CPU utilization, network in/out, and the Application Load
            Balancer request count.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/APIReference/API_PredictiveScalingConfiguration.html">PredictiveScalingConfiguration</a> in the <i>Amazon EC2 Auto Scaling API
                Reference</i>.</p>
         <p>Required if the policy type is <code>PredictiveScaling</code>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create scaling_policy
scaling_policy = provider.auto_scaling.Scaling_policy {
    auto_scaling_group_name = "value"  # <p>The name of the Auto Scaling group.</p>
    policy_name = "value"  # <p>The name of the policy.</p>
}

```

---


### Notification_configurations

NotificationConfigurations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>A string that indicates that the response contains more items than can be returned in
            a single response. To receive additional items, specify this string for the
                <code>NextToken</code> value when requesting the next set of items. This value is
            null when there are no more items to return.</p> |
| `notification_configurations` | Vec<String> | <p>The notification configurations.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access notification_configurations outputs
notification_configurations_id = notification_configurations.id
notification_configurations_next_token = notification_configurations.next_token
notification_configurations_notification_configurations = notification_configurations.notification_configurations
```

---


### Lifecycle_hook_types

LifecycleHookTypes resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `lifecycle_hook_types` | Vec<String> | <p>The lifecycle hook types.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access lifecycle_hook_types outputs
lifecycle_hook_types_id = lifecycle_hook_types.id
lifecycle_hook_types_lifecycle_hook_types = lifecycle_hook_types.lifecycle_hook_types
```

---


### Scaling_process_types

ScalingProcessTypes resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `processes` | Vec<String> | <p>The names of the process types.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access scaling_process_types outputs
scaling_process_types_id = scaling_process_types.id
scaling_process_types_processes = scaling_process_types.processes
```

---


### Account_limits

AccountLimits resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `number_of_launch_configurations` | i64 | <p>The current number of launch configurations for your account.</p> |
| `number_of_auto_scaling_groups` | i64 | <p>The current number of groups for your account.</p> |
| `max_number_of_launch_configurations` | i64 | <p>The maximum number of launch configurations allowed for your account. The default is
            200 launch configurations per Region.</p> |
| `max_number_of_auto_scaling_groups` | i64 | <p>The maximum number of groups allowed for your account. The default is 200 groups per
            Region.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access account_limits outputs
account_limits_id = account_limits.id
account_limits_number_of_launch_configurations = account_limits.number_of_launch_configurations
account_limits_number_of_auto_scaling_groups = account_limits.number_of_auto_scaling_groups
account_limits_max_number_of_launch_configurations = account_limits.max_number_of_launch_configurations
account_limits_max_number_of_auto_scaling_groups = account_limits.max_number_of_auto_scaling_groups
```

---


### Auto_scaling_notification_types

AutoScalingNotificationTypes resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `auto_scaling_notification_types` | Vec<String> | <p>The notification types.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access auto_scaling_notification_types outputs
auto_scaling_notification_types_id = auto_scaling_notification_types.id
auto_scaling_notification_types_auto_scaling_notification_types = auto_scaling_notification_types.auto_scaling_notification_types
```

---


### Auto_scaling_group

AutoScalingGroup resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `service_linked_role_arn` | String |  | <p>The Amazon Resource Name (ARN) of the service-linked role that the Auto Scaling group uses to
            call other Amazon Web Services service on your behalf. By default, Amazon EC2 Auto Scaling uses a service-linked role
            named <code>AWSServiceRoleForAutoScaling</code>, which it creates if it does not exist.
            For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/autoscaling-service-linked-role.html">Service-linked
                roles</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p> |
| `skip_zonal_shift_validation` | bool |  | <p>
            If you enable zonal shift with cross-zone disabled load balancers, capacity could become imbalanced across Availability Zones. To skip the validation, specify <code>true</code>. For more information, see
            <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/ec2-auto-scaling-zonal-shift.html">Auto Scaling group zonal shift</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.
        </p> |
| `auto_scaling_group_name` | String | ✅ | <p>The name of the Auto Scaling group. This name must be unique per Region per account.</p>
         <p>The name can contain any ASCII character 33 to 126 including most punctuation
            characters, digits, and upper and lowercased letters.</p>
         <note>
            <p>You cannot use a colon (:) in the name.</p>
         </note> |
| `context` | String |  | <p>Reserved.</p> |
| `termination_policies` | Vec<String> |  | <p>A policy or a list of policies that are used to select the instance to terminate.
            These policies are executed in the order that you list them. For more information, see
                <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/ec2-auto-scaling-termination-policies.html">Configure
                termination policies for Amazon EC2 Auto Scaling</a> in the
            <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
         <p>Valid values: <code>Default</code> | <code>AllocationStrategy</code> |
                <code>ClosestToNextInstanceHour</code> | <code>NewestInstance</code> |
                <code>OldestInstance</code> | <code>OldestLaunchConfiguration</code> |
                <code>OldestLaunchTemplate</code> |
                <code>arn:aws:lambda:region:account-id:function:my-function:my-alias</code>
         </p> |
| `availability_zone_impairment_policy` | String |  | <p>
            The policy for Availability Zone impairment.
        </p> |
| `max_size` | i64 | ✅ | <p>The maximum size of the group.</p>
         <note>
            <p>With a mixed instances policy that uses instance weighting, Amazon EC2 Auto Scaling may need to
                go above <code>MaxSize</code> to meet your capacity requirements. In this event,
                Amazon EC2 Auto Scaling will never go above <code>MaxSize</code> by more than your largest instance
                weight (weights that define how many units each instance contributes to the desired
                capacity of the group).</p>
         </note> |
| `mixed_instances_policy` | String |  | <p>The mixed instances policy. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/ec2-auto-scaling-mixed-instances-groups.html">Auto Scaling
                groups with multiple instance types and purchase options</a> in the
                <i>Amazon EC2 Auto Scaling User Guide</i>.</p> |
| `target_group_ar_ns` | Vec<String> |  | <p>The Amazon Resource Names (ARN) of the Elastic Load Balancing target groups to associate with the Auto Scaling
            group. Instances are registered as targets with the target groups. The target groups
            receive incoming traffic and route requests to one or more registered targets. For more
            information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/autoscaling-load-balancer.html">Use Elastic Load Balancing to
                distribute traffic across the instances in your Auto Scaling group</a> in the
                <i>Amazon EC2 Auto Scaling User Guide</i>.</p> |
| `launch_configuration_name` | String |  | <p>The name of the launch configuration to use to launch instances. </p>
         <p>Conditional: You must specify either a launch template (<code>LaunchTemplate</code> or
                <code>MixedInstancesPolicy</code>) or a launch configuration
                (<code>LaunchConfigurationName</code> or <code>InstanceId</code>).</p> |
| `health_check_grace_period` | i64 |  | <p>The amount of time, in seconds, that Amazon EC2 Auto Scaling waits before checking the health status
            of an EC2 instance that has come into service and marking it unhealthy due to a failed
            health check. This is useful if your instances do not immediately pass their health
            checks after they enter the <code>InService</code> state. For more information, see
                <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/health-check-grace-period.html">Set the health check
                grace period for an Auto Scaling group</a> in the
            <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
         <p>Default: <code>0</code> seconds</p> |
| `lifecycle_hook_specification_list` | Vec<String> |  | <p>One or more lifecycle hooks to add to the Auto Scaling group before instances are
            launched.</p> |
| `capacity_reservation_specification` | String |  | <p>
        The capacity reservation specification for the Auto Scaling group. 
    </p> |
| `min_size` | i64 | ✅ | <p>The minimum size of the group.</p> |
| `default_cooldown` | i64 |  | <p>
            <i>Only needed if you use simple scaling policies.</i>
         </p>
         <p>The amount of time, in seconds, between one scaling activity ending and another one
            starting due to simple scaling policies. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/ec2-auto-scaling-scaling-cooldowns.html">Scaling
                cooldowns for Amazon EC2 Auto Scaling</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
         <p>Default: <code>300</code> seconds</p> |
| `health_check_type` | String |  | <p>A comma-separated value string of one or more health check types.</p>
         <p>The valid values are <code>EC2</code>, <code>EBS</code>, <code>ELB</code>, and
                <code>VPC_LATTICE</code>. <code>EC2</code> is the default health check and cannot be
            disabled. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/ec2-auto-scaling-health-checks.html">Health checks
                for instances in an Auto Scaling group</a> in the
            <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
         <p>Only specify <code>EC2</code> if you must clear a value that was previously
            set.</p> |
| `launch_template` | String |  | <p>Information used to specify the launch template and version to use to launch
            instances. </p>
         <p>Conditional: You must specify either a launch template (<code>LaunchTemplate</code> or
                <code>MixedInstancesPolicy</code>) or a launch configuration
                (<code>LaunchConfigurationName</code> or <code>InstanceId</code>).</p>
         <note>
            <p>The launch template that is specified must be configured for use with an Auto Scaling
                group. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/create-launch-template.html">Create a launch
                    template for an Auto Scaling group</a> in the
                <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
         </note> |
| `vpc_zone_identifier` | String |  | <p>A comma-separated list of subnet IDs for a virtual private cloud (VPC) where instances
            in the Auto Scaling group can be created. If you specify <code>VPCZoneIdentifier</code> with
                <code>AvailabilityZones</code>, the subnets that you specify must reside in those
            Availability Zones.</p> |
| `instance_id` | String |  | <p>The ID of the instance used to base the launch configuration on. If specified, Amazon
            EC2 Auto Scaling uses the configuration values from the specified instance to create a
            new launch configuration. To get the instance ID, use the Amazon EC2 <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeInstances.html">DescribeInstances</a> API operation. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/create-asg-from-instance.html">Create an Auto Scaling group using parameters from an existing instance</a> in the
                <i>Amazon EC2 Auto Scaling User Guide</i>.</p> |
| `load_balancer_names` | Vec<String> |  | <p>A list of Classic Load Balancers associated with this Auto Scaling group. For Application Load Balancers, Network Load Balancers, and Gateway Load Balancers,
            specify the <code>TargetGroupARNs</code> property instead.</p> |
| `placement_group` | String |  | <p>The name of the placement group into which to launch your instances. For more
            information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/placement-groups.html">Placement groups</a> in the
                <i>Amazon EC2 User Guide</i>.</p>
         <note>
            <p>A <i>cluster</i> placement group is a logical grouping of instances
                within a single Availability Zone. You cannot specify multiple Availability Zones
                and a cluster placement group. </p>
         </note> |
| `default_instance_warmup` | i64 |  | <p>The amount of time, in seconds, until a new instance is considered to have finished
            initializing and resource consumption to become stable after it enters the
                <code>InService</code> state. </p>
         <p>During an instance refresh, Amazon EC2 Auto Scaling waits for the warm-up period after it replaces an
            instance before it moves on to replacing the next instance. Amazon EC2 Auto Scaling also waits for the
            warm-up period before aggregating the metrics for new instances with existing instances
            in the Amazon CloudWatch metrics that are used for scaling, resulting in more reliable usage
            data. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/ec2-auto-scaling-default-instance-warmup.html">Set
                the default instance warmup for an Auto Scaling group</a> in the
                <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
         <important>
            <p>To manage various warm-up settings at the group level, we recommend that you set
                the default instance warmup, <i>even if it is set to 0 seconds</i>. To
                remove a value that you previously set, include the property but specify
                    <code>-1</code> for the value. However, we strongly recommend keeping the
                default instance warmup enabled by specifying a value of <code>0</code> or other
                nominal value.</p>
         </important>
         <p>Default: None </p> |
| `availability_zone_distribution` | String |  | <p>The instance capacity distribution across Availability Zones.</p> |
| `max_instance_lifetime` | i64 |  | <p>The maximum amount of time, in seconds, that an instance can be in service. The
            default is null. If specified, the value must be either 0 or a number equal to or
            greater than 86,400 seconds (1 day). For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/asg-max-instance-lifetime.html">Replace Auto Scaling instances based on maximum instance lifetime</a> in the
                <i>Amazon EC2 Auto Scaling User Guide</i>.</p> |
| `instance_maintenance_policy` | String |  | <p>An instance maintenance policy. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/ec2-auto-scaling-instance-maintenance-policy.html">Set instance maintenance policy</a> in the
            <i>Amazon EC2 Auto Scaling User Guide</i>.</p> |
| `traffic_sources` | Vec<String> |  | <p>The list of traffic sources to attach to this Auto Scaling group. You can use any of the
            following as traffic sources for an Auto Scaling group: Classic Load Balancer, Application Load Balancer, Gateway Load Balancer, Network Load Balancer, and
            VPC Lattice.</p> |
| `desired_capacity_type` | String |  | <p>The unit of measurement for the value specified for desired capacity. Amazon EC2 Auto Scaling
            supports <code>DesiredCapacityType</code> for attribute-based instance type selection
            only. For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/create-mixed-instances-group-attribute-based-instance-type-selection.html">Create a mixed instances group using attribute-based instance type
                selection</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
         <p>By default, Amazon EC2 Auto Scaling specifies <code>units</code>, which translates into number of
            instances.</p>
         <p>Valid values: <code>units</code> | <code>vcpu</code> | <code>memory-mib</code>
         </p> |
| `capacity_rebalance` | bool |  | <p>Indicates whether Capacity Rebalancing is enabled. Otherwise, Capacity Rebalancing is
            disabled. When you turn on Capacity Rebalancing, Amazon EC2 Auto Scaling attempts to launch a Spot
            Instance whenever Amazon EC2 notifies that a Spot Instance is at an elevated risk of
            interruption. After launching a new instance, it then terminates an old instance. For
            more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/ec2-auto-scaling-capacity-rebalancing.html">Use Capacity
                Rebalancing to handle Amazon EC2 Spot Interruptions</a> in the in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p> |
| `tags` | Vec<String> |  | <p>One or more tags. You can tag your Auto Scaling group and propagate the tags to the Amazon EC2
            instances it launches. Tags are not propagated to Amazon EBS volumes. To add tags to Amazon EBS
            volumes, specify the tags in a launch template but use caution. If the launch template
            specifies an instance tag with a key that is also specified for the Auto Scaling group, Amazon EC2 Auto Scaling
            overrides the value of that instance tag with the value specified by the Auto Scaling group. For
            more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/ec2-auto-scaling-tagging.html">Tag Auto Scaling groups and
                instances</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p> |
| `new_instances_protected_from_scale_in` | bool |  | <p>Indicates whether newly launched instances are protected from termination by Amazon EC2 Auto Scaling
            when scaling in. For more information about preventing instances from terminating on
            scale in, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/ec2-auto-scaling-instance-protection.html">Use
                instance scale-in protection</a> in the
            <i>Amazon EC2 Auto Scaling User Guide</i>.</p> |
| `availability_zones` | Vec<String> |  | <p>A list of Availability Zones where instances in the Auto Scaling group can be created. Used
            for launching into the default VPC subnet in each Availability Zone when not using the
                <code>VPCZoneIdentifier</code> property, or for attaching a network interface when
            an existing network interface ID is specified in a launch template.</p> |
| `desired_capacity` | i64 |  | <p>The desired capacity is the initial capacity of the Auto Scaling group at the time of its
            creation and the capacity it attempts to maintain. It can scale beyond this capacity if
            you configure auto scaling. This number must be greater than or equal to the minimum
            size of the group and less than or equal to the maximum size of the group. If you do not
            specify a desired capacity, the default is the minimum size of the group.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create auto_scaling_group
auto_scaling_group = provider.auto_scaling.Auto_scaling_group {
    auto_scaling_group_name = "value"  # <p>The name of the Auto Scaling group. This name must be unique per Region per account.</p>
         <p>The name can contain any ASCII character 33 to 126 including most punctuation
            characters, digits, and upper and lowercased letters.</p>
         <note>
            <p>You cannot use a colon (:) in the name.</p>
         </note>
    max_size = "value"  # <p>The maximum size of the group.</p>
         <note>
            <p>With a mixed instances policy that uses instance weighting, Amazon EC2 Auto Scaling may need to
                go above <code>MaxSize</code> to meet your capacity requirements. In this event,
                Amazon EC2 Auto Scaling will never go above <code>MaxSize</code> by more than your largest instance
                weight (weights that define how many units each instance contributes to the desired
                capacity of the group).</p>
         </note>
    min_size = "value"  # <p>The minimum size of the group.</p>
}

```

---


### Scheduled_action

ScheduledAction resource

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


### Auto_scaling_instances

AutoScalingInstances resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>A string that indicates that the response contains more items than can be returned in
            a single response. To receive additional items, specify this string for the
                <code>NextToken</code> value when requesting the next set of items. This value is
            null when there are no more items to return.</p> |
| `auto_scaling_instances` | Vec<String> | <p>The instances.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access auto_scaling_instances outputs
auto_scaling_instances_id = auto_scaling_instances.id
auto_scaling_instances_next_token = auto_scaling_instances.next_token
auto_scaling_instances_auto_scaling_instances = auto_scaling_instances.auto_scaling_instances
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple instance_refreshes resources
instance_refreshes_0 = provider.auto_scaling.Instance_refreshes {
}
instance_refreshes_1 = provider.auto_scaling.Instance_refreshes {
}
instance_refreshes_2 = provider.auto_scaling.Instance_refreshes {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    instance_refreshes = provider.auto_scaling.Instance_refreshes {
    }
```

---

## Related Documentation

- [AWS Auto_scaling Documentation](https://docs.aws.amazon.com/auto_scaling/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
