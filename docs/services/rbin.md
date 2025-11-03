# Rbin Service



**Resources**: 1

---

## Overview

The rbin service provides access to 1 resource type:

- [Rule](#rule) [CRUD]

---

## Resources


### Rule

Rule resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `retention_period` | String | ✅ | <p>Information about the retention period for which the retention rule is to retain resources.</p> |
| `tags` | Vec<String> |  | <p>Information about the tags to assign to the retention rule.</p> |
| `resource_type` | String | ✅ | <p>The resource type to be retained by the retention rule. Currently, only Amazon EBS snapshots 
      and EBS-backed AMIs are supported. To retain snapshots, specify <code>EBS_SNAPSHOT</code>. To 
      retain EBS-backed AMIs, specify <code>EC2_IMAGE</code>.</p> |
| `exclude_resource_tags` | Vec<String> |  | <p>[Region-level retention rules only] Specifies the exclusion tags to use to identify resources that are to be excluded, 
or ignored, by a Region-level retention rule. Resources that have any of these tags are not retained by the retention rule 
upon deletion.</p>
         <p>You can't specify exclusion tags for tag-level retention rules.</p> |
| `description` | String |  | <p>The retention rule description.</p> |
| `resource_tags` | Vec<String> |  | <p>[Tag-level retention rules only] Specifies the resource tags to use to identify resources that are to be retained by a 
  tag-level retention rule. For tag-level retention rules, only deleted resources, of the specified resource type, that 
  have one or more of the specified tag key and value pairs are retained. If a resource is deleted, but it does not have 
  any of the specified tag key and value pairs, it is immediately deleted without being retained by the retention rule.</p>
         <p>You can add the same tag key and value pair to a maximum or five retention rules.</p>
         <p>To create a Region-level retention rule, omit this parameter. A Region-level retention rule 
      does not have any resource tags specified. It retains all deleted resources of the specified 
      resource type in the Region in which the rule is created, even if the resources are not tagged.</p> |
| `lock_configuration` | String |  | <p>Information about the retention rule lock configuration.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `lock_state` | String | <p>[Region-level retention rules only] The lock state for the retention rule.</p>
         <ul>
            <li>
               <p>
                  <code>locked</code> - The retention rule is locked and can't be modified or deleted.</p>
            </li>
            <li>
               <p>
                  <code>pending_unlock</code> - The retention rule has been unlocked but it is still within 
          the unlock delay period. The retention rule can be modified or deleted only after the unlock 
          delay period has expired.</p>
            </li>
            <li>
               <p>
                  <code>unlocked</code> - The retention rule is unlocked and it can be modified or deleted by 
          any user with the required permissions.</p>
            </li>
            <li>
               <p>
                  <code>null</code> - The retention rule has never been locked. Once a retention rule has 
        been locked, it can transition between the <code>locked</code> and <code>unlocked</code> states 
        only; it can never transition back to <code>null</code>.</p>
            </li>
         </ul> |
| `exclude_resource_tags` | Vec<String> | <p>[Region-level retention rules only] Information about the exclusion tags used to identify resources that are to be 
excluded, or ignored, by the retention rule.</p> |
| `status` | String | <p>The state of the retention rule. Only retention rules that are in the <code>available</code> 
      state retain resources.</p> |
| `identifier` | String | <p>The unique ID of the retention rule.</p> |
| `rule_arn` | String | <p>The Amazon Resource Name (ARN) of the retention rule.</p> |
| `lock_configuration` | String | <p>Information about the retention rule lock configuration.</p> |
| `lock_end_time` | String | <p>The date and time at which the unlock delay is set to expire. Only returned 
      for retention rules that have been unlocked and that are still within the unlock 
      delay period.</p> |
| `resource_tags` | Vec<String> | <p>[Tag-level retention rules only] Information about the resource tags used to identify resources that are retained by the retention 
      rule.</p> |
| `retention_period` | String | <p>Information about the retention period for which the retention rule is to retain resources.</p> |
| `description` | String | <p>The retention rule description.</p> |
| `resource_type` | String | <p>The resource type retained by the retention rule.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create rule
rule = provider.rbin.Rule {
    retention_period = "value"  # <p>Information about the retention period for which the retention rule is to retain resources.</p>
    resource_type = "value"  # <p>The resource type to be retained by the retention rule. Currently, only Amazon EBS snapshots 
      and EBS-backed AMIs are supported. To retain snapshots, specify <code>EBS_SNAPSHOT</code>. To 
      retain EBS-backed AMIs, specify <code>EC2_IMAGE</code>.</p>
}

# Access rule outputs
rule_id = rule.id
rule_lock_state = rule.lock_state
rule_exclude_resource_tags = rule.exclude_resource_tags
rule_status = rule.status
rule_identifier = rule.identifier
rule_rule_arn = rule.rule_arn
rule_lock_configuration = rule.lock_configuration
rule_lock_end_time = rule.lock_end_time
rule_resource_tags = rule.resource_tags
rule_retention_period = rule.retention_period
rule_description = rule.description
rule_resource_type = rule.resource_type
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple rule resources
rule_0 = provider.rbin.Rule {
    retention_period = "value-0"
    resource_type = "value-0"
}
rule_1 = provider.rbin.Rule {
    retention_period = "value-1"
    resource_type = "value-1"
}
rule_2 = provider.rbin.Rule {
    retention_period = "value-2"
    resource_type = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    rule = provider.rbin.Rule {
        retention_period = "production-value"
        resource_type = "production-value"
    }
```

---

## Related Documentation

- [AWS Rbin Documentation](https://docs.aws.amazon.com/rbin/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
