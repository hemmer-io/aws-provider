# Dlm Service



**Resources**: 2

---

## Overview

The dlm service provides access to 2 resource types:

- [Lifecycle_policy](#lifecycle_policy) [CRUD]
- [Lifecycle_policies](#lifecycle_policies) [R]

---

## Resources


### Lifecycle_policy

LifecyclePolicy resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | HashMap<String, String> |  | <p>The tags to apply to the lifecycle policy during creation.</p> |
| `exclusions` | String |  | <p>
            <b>[Default policies only]</b> Specifies exclusion parameters for volumes or instances for which you 
			do not want to create snapshots or AMIs. The policy will not create snapshots or AMIs 
			for target resources that match any of the specified exclusion parameters.</p> |
| `description` | String | ✅ | <p>A description of the lifecycle policy. The characters ^[0-9A-Za-z _-]+$ are
			supported.</p> |
| `state` | String | ✅ | <p>The activation state of the lifecycle policy after creation.</p> |
| `execution_role_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the IAM role used to run the operations specified by
			the lifecycle policy.</p> |
| `default_policy` | String |  | <p>
            <b>[Default policies only]</b> Specify the type of default policy to create.</p>
         <ul>
            <li>
               <p>To create a default policy for EBS snapshots, that creates snapshots of all volumes in the
					Region that do not have recent backups, specify <code>VOLUME</code>.</p>
            </li>
            <li>
               <p>To create a default policy for EBS-backed AMIs, that creates EBS-backed 
					AMIs from all instances in the Region that do not have recent backups, specify 
					<code>INSTANCE</code>.</p>
            </li>
         </ul> |
| `policy_details` | String |  | <p>The configuration details of the lifecycle policy.</p>
         <important>
            <p>If you create a default policy, you can specify the request parameters either in  
				the request body, or in the PolicyDetails request structure, but not both.</p>
         </important> |
| `create_interval` | i64 |  | <p>
            <b>[Default policies only]</b> Specifies how often the policy should run and create snapshots or AMIs. 
			The creation frequency can range from 1 to 7 days. If you do not specify a value, the 
			default is 1.</p>
         <p>Default: 1</p> |
| `cross_region_copy_targets` | Vec<String> |  | <p>
            <b>[Default policies only]</b> Specifies destination Regions for snapshot or AMI copies. You can specify 
			up to 3 destination Regions. If you do not want to create cross-Region copies, omit this 
			parameter.</p> |
| `retain_interval` | i64 |  | <p>
            <b>[Default policies only]</b> Specifies how long the policy should retain snapshots or AMIs before 
			deleting them. The retention period can range from 2 to 14 days, but it must be greater 
			than the creation frequency to ensure that the policy retains at least 1 snapshot or 
			AMI at any given time. If you do not specify a value, the default is 7.</p>
         <p>Default: 7</p> |
| `copy_tags` | bool |  | <p>
            <b>[Default policies only]</b> Indicates whether the policy should copy tags from the source resource 
			to the snapshot or AMI. If you do not specify a value, the default is <code>false</code>.</p>
         <p>Default: false</p> |
| `extend_deletion` | bool |  | <p>
            <b>[Default policies only]</b> Defines the snapshot or AMI retention behavior for the policy if the 
			source volume or instance is deleted, or if the policy enters the error, disabled, or 
			deleted state.</p>
         <p>By default (<b>ExtendDeletion=false</b>):</p>
         <ul>
            <li>
               <p>If a source resource is deleted, Amazon Data Lifecycle Manager will continue to delete previously 
				created snapshots or AMIs, up to but not including the last one, based on the 
				specified retention period. If you want Amazon Data Lifecycle Manager to delete all snapshots or AMIs, 
				including the last one, specify <code>true</code>.</p>
            </li>
            <li>
               <p>If a policy enters the error, disabled, or deleted state, Amazon Data Lifecycle Manager stops deleting 
					snapshots and AMIs. If you want Amazon Data Lifecycle Manager to continue deleting snapshots or AMIs, 
					including the last one, if the policy enters one of these states, specify 
					<code>true</code>.</p>
            </li>
         </ul>
         <p>If you enable extended deletion (<b>ExtendDeletion=true</b>), 
			you override both default behaviors simultaneously.</p>
         <p>If you do not specify a value, the default is <code>false</code>.</p>
         <p>Default: false</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `policy` | String | <p>Detailed information about the lifecycle policy.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create lifecycle_policy
lifecycle_policy = provider.dlm.Lifecycle_policy {
    description = "value"  # <p>A description of the lifecycle policy. The characters ^[0-9A-Za-z _-]+$ are
			supported.</p>
    state = "value"  # <p>The activation state of the lifecycle policy after creation.</p>
    execution_role_arn = "value"  # <p>The Amazon Resource Name (ARN) of the IAM role used to run the operations specified by
			the lifecycle policy.</p>
}

# Access lifecycle_policy outputs
lifecycle_policy_id = lifecycle_policy.id
lifecycle_policy_policy = lifecycle_policy.policy
```

---


### Lifecycle_policies

LifecyclePolicies resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `policies` | Vec<String> | <p>Summary information about the lifecycle policies.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access lifecycle_policies outputs
lifecycle_policies_id = lifecycle_policies.id
lifecycle_policies_policies = lifecycle_policies.policies
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple lifecycle_policy resources
lifecycle_policy_0 = provider.dlm.Lifecycle_policy {
    description = "value-0"
    state = "value-0"
    execution_role_arn = "value-0"
}
lifecycle_policy_1 = provider.dlm.Lifecycle_policy {
    description = "value-1"
    state = "value-1"
    execution_role_arn = "value-1"
}
lifecycle_policy_2 = provider.dlm.Lifecycle_policy {
    description = "value-2"
    state = "value-2"
    execution_role_arn = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    lifecycle_policy = provider.dlm.Lifecycle_policy {
        description = "production-value"
        state = "production-value"
        execution_role_arn = "production-value"
    }
```

---

## Related Documentation

- [AWS Dlm Documentation](https://docs.aws.amazon.com/dlm/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
