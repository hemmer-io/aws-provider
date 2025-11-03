# Shield Service



**Resources**: 9

---

## Overview

The shield service provides access to 9 resource types:

- [Attack_statistics](#attack_statistics) [R]
- [Protection_group](#protection_group) [CRUD]
- [Attack](#attack) [R]
- [Drt_access](#drt_access) [R]
- [Subscription_state](#subscription_state) [R]
- [Protection](#protection) [CRD]
- [Application_layer_automatic_response](#application_layer_automatic_response) [U]
- [Emergency_contact_settings](#emergency_contact_settings) [RU]
- [Subscription](#subscription) [CRUD]

---

## Resources


### Attack_statistics

AttackStatistics resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `time_range` | String | <p>The time range of the attack.</p> |
| `data_items` | Vec<String> | <p>The data that describes the attacks detected during the time period.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access attack_statistics outputs
attack_statistics_id = attack_statistics.id
attack_statistics_time_range = attack_statistics.time_range
attack_statistics_data_items = attack_statistics.data_items
```

---


### Protection_group

ProtectionGroup resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resource_type` | String |  | <p>The resource type to include in the protection group. All protected resources of this type are included in the protection group. Newly protected resources of this type are automatically added to the group.
           You must set this when you set <code>Pattern</code> to <code>BY_RESOURCE_TYPE</code> and you must not set it for any other <code>Pattern</code> setting. </p> |
| `pattern` | String | ✅ | <p>The criteria to use to choose the protected resources for inclusion in the group. You can include all resources that have protections, provide a list of resource Amazon Resource Names (ARNs), or include all resources of a specified resource type. </p> |
| `protection_group_id` | String | ✅ | <p>The name of the protection group. You use this to identify the protection group in lists and to manage the protection group, for example to update, delete, or describe it. </p> |
| `tags` | Vec<String> |  | <p>One or more tag key-value pairs for the protection group.</p> |
| `members` | Vec<String> |  | <p>The Amazon Resource Names (ARNs) of the resources to include in the protection group. You must set this when you set <code>Pattern</code> to <code>ARBITRARY</code> and you must not set it for any other <code>Pattern</code> setting. </p> |
| `aggregation` | String | ✅ | <p>Defines how Shield combines resource data for the group in order to detect, mitigate, and report events.</p>
         <ul>
            <li>
               <p>Sum - Use the total traffic across the group. This is a good choice for most cases. Examples include Elastic IP addresses for EC2 instances that scale manually or automatically.</p>
            </li>
            <li>
               <p>Mean - Use the average of the traffic across the group. This is a good choice for resources that share traffic uniformly. Examples include accelerators and load balancers.</p>
            </li>
            <li>
               <p>Max - Use the highest traffic from each resource. This is useful for resources that don't share traffic and for resources that share that traffic in a non-uniform way. Examples include Amazon CloudFront and origin resources for CloudFront distributions.</p>
            </li>
         </ul> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `protection_group` | String | <p>A grouping of protected resources that you and Shield Advanced can monitor as a collective. This resource grouping improves the accuracy of detection and reduces false positives. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create protection_group
protection_group = provider.shield.Protection_group {
    pattern = "value"  # <p>The criteria to use to choose the protected resources for inclusion in the group. You can include all resources that have protections, provide a list of resource Amazon Resource Names (ARNs), or include all resources of a specified resource type. </p>
    protection_group_id = "value"  # <p>The name of the protection group. You use this to identify the protection group in lists and to manage the protection group, for example to update, delete, or describe it. </p>
    aggregation = "value"  # <p>Defines how Shield combines resource data for the group in order to detect, mitigate, and report events.</p>
         <ul>
            <li>
               <p>Sum - Use the total traffic across the group. This is a good choice for most cases. Examples include Elastic IP addresses for EC2 instances that scale manually or automatically.</p>
            </li>
            <li>
               <p>Mean - Use the average of the traffic across the group. This is a good choice for resources that share traffic uniformly. Examples include accelerators and load balancers.</p>
            </li>
            <li>
               <p>Max - Use the highest traffic from each resource. This is useful for resources that don't share traffic and for resources that share that traffic in a non-uniform way. Examples include Amazon CloudFront and origin resources for CloudFront distributions.</p>
            </li>
         </ul>
}

# Access protection_group outputs
protection_group_id = protection_group.id
protection_group_protection_group = protection_group.protection_group
```

---


### Attack

Attack resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `attack` | String | <p>The attack that you requested. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access attack outputs
attack_id = attack.id
attack_attack = attack.attack
```

---


### Drt_access

DRTAccess resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `log_bucket_list` | Vec<String> | <p>The list of Amazon S3 buckets accessed by the SRT.</p> |
| `role_arn` | String | <p>The Amazon Resource Name (ARN) of the role the SRT used to access your Amazon Web Services account.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access drt_access outputs
drt_access_id = drt_access.id
drt_access_log_bucket_list = drt_access.log_bucket_list
drt_access_role_arn = drt_access.role_arn
```

---


### Subscription_state

SubscriptionState resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `subscription_state` | String | <p>The status of the subscription.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access subscription_state outputs
subscription_state_id = subscription_state.id
subscription_state_subscription_state = subscription_state.subscription_state
```

---


### Protection

Protection resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resource_arn` | String | ✅ | <p>The ARN (Amazon Resource Name) of the resource to be protected.</p>
         <p>The ARN should be in one of the following formats:</p>
         <ul>
            <li>
               <p>For an Application Load Balancer: <code>arn:aws:elasticloadbalancing:<i>region</i>:<i>account-id</i>:loadbalancer/app/<i>load-balancer-name</i>/<i>load-balancer-id</i>
                  </code>
               </p>
            </li>
            <li>
               <p>For an Elastic Load Balancer (Classic Load Balancer): <code>arn:aws:elasticloadbalancing:<i>region</i>:<i>account-id</i>:loadbalancer/<i>load-balancer-name</i>
                  </code>
               </p>
            </li>
            <li>
               <p>For an Amazon CloudFront distribution: <code>arn:aws:cloudfront::<i>account-id</i>:distribution/<i>distribution-id</i>
                  </code>
               </p>
            </li>
            <li>
               <p>For an Global Accelerator standard accelerator: <code>arn:aws:globalaccelerator::<i>account-id</i>:accelerator/<i>accelerator-id</i>
                  </code>
               </p>
            </li>
            <li>
               <p>For Amazon Route 53: <code>arn:aws:route53:::hostedzone/<i>hosted-zone-id</i>
                  </code>
               </p>
            </li>
            <li>
               <p>For an Elastic IP address: <code>arn:aws:ec2:<i>region</i>:<i>account-id</i>:eip-allocation/<i>allocation-id</i>
                  </code>
               </p>
            </li>
         </ul> |
| `tags` | Vec<String> |  | <p>One or more tag key-value pairs for the <a>Protection</a> object that is created.</p> |
| `name` | String | ✅ | <p>Friendly name for the <code>Protection</code> you are creating.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `protection` | String | <p>The <a>Protection</a> that you requested. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create protection
protection = provider.shield.Protection {
    resource_arn = "value"  # <p>The ARN (Amazon Resource Name) of the resource to be protected.</p>
         <p>The ARN should be in one of the following formats:</p>
         <ul>
            <li>
               <p>For an Application Load Balancer: <code>arn:aws:elasticloadbalancing:<i>region</i>:<i>account-id</i>:loadbalancer/app/<i>load-balancer-name</i>/<i>load-balancer-id</i>
                  </code>
               </p>
            </li>
            <li>
               <p>For an Elastic Load Balancer (Classic Load Balancer): <code>arn:aws:elasticloadbalancing:<i>region</i>:<i>account-id</i>:loadbalancer/<i>load-balancer-name</i>
                  </code>
               </p>
            </li>
            <li>
               <p>For an Amazon CloudFront distribution: <code>arn:aws:cloudfront::<i>account-id</i>:distribution/<i>distribution-id</i>
                  </code>
               </p>
            </li>
            <li>
               <p>For an Global Accelerator standard accelerator: <code>arn:aws:globalaccelerator::<i>account-id</i>:accelerator/<i>accelerator-id</i>
                  </code>
               </p>
            </li>
            <li>
               <p>For Amazon Route 53: <code>arn:aws:route53:::hostedzone/<i>hosted-zone-id</i>
                  </code>
               </p>
            </li>
            <li>
               <p>For an Elastic IP address: <code>arn:aws:ec2:<i>region</i>:<i>account-id</i>:eip-allocation/<i>allocation-id</i>
                  </code>
               </p>
            </li>
         </ul>
    name = "value"  # <p>Friendly name for the <code>Protection</code> you are creating.</p>
}

# Access protection outputs
protection_id = protection.id
protection_protection = protection.protection
```

---


### Application_layer_automatic_response

ApplicationLayerAutomaticResponse resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `action` | String | ✅ | <p>Specifies the action setting that Shield Advanced should use in the WAF rules that it creates on behalf of the
   protected resource in response to DDoS attacks. You specify this as part of the configuration for the automatic application layer DDoS mitigation feature,
   when you enable or update automatic mitigation. Shield Advanced creates the WAF rules in a Shield Advanced-managed rule group, inside the web ACL that you have associated with the resource. </p> |
| `resource_arn` | String | ✅ | <p>The ARN (Amazon Resource Name) of the resource.</p> |



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


### Emergency_contact_settings

EmergencyContactSettings resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `emergency_contact_list` | Vec<String> |  | <p>A list of email addresses and phone numbers that the Shield Response Team (SRT) can use to contact you if you have proactive engagement enabled, for escalations to the SRT and to initiate proactive customer support.</p>
         <p>If you have proactive engagement enabled, the contact list must include at least one phone number.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `emergency_contact_list` | Vec<String> | <p>A list of email addresses and phone numbers that the Shield Response Team (SRT) can use to contact you if you have proactive engagement enabled, for escalations to the SRT and to initiate proactive customer support.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access emergency_contact_settings outputs
emergency_contact_settings_id = emergency_contact_settings.id
emergency_contact_settings_emergency_contact_list = emergency_contact_settings.emergency_contact_list
```

---


### Subscription

Subscription resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `subscription` | String | <p>The Shield Advanced subscription details for an account.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create subscription
subscription = provider.shield.Subscription {
}

# Access subscription outputs
subscription_id = subscription.id
subscription_subscription = subscription.subscription
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple attack_statistics resources
attack_statistics_0 = provider.shield.Attack_statistics {
}
attack_statistics_1 = provider.shield.Attack_statistics {
}
attack_statistics_2 = provider.shield.Attack_statistics {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    attack_statistics = provider.shield.Attack_statistics {
    }
```

---

## Related Documentation

- [AWS Shield Documentation](https://docs.aws.amazon.com/shield/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
