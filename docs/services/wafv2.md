# Wafv2 Service



**Resources**: 19

---

## Overview

The wafv2 service provides access to 19 resource types:

- [Firewall_manager_rule_groups](#firewall_manager_rule_groups) [D]
- [Permission_policy](#permission_policy) [CRD]
- [Managed_rule_group](#managed_rule_group) [R]
- [Logging_configuration](#logging_configuration) [CRD]
- [Mobile_sdk_release](#mobile_sdk_release) [R]
- [Decrypted_api_key](#decrypted_api_key) [R]
- [Api_key](#api_key) [CD]
- [Sampled_requests](#sampled_requests) [R]
- [All_managed_products](#all_managed_products) [R]
- [Regex_pattern_set](#regex_pattern_set) [CRUD]
- [Rate_based_statement_managed_keys](#rate_based_statement_managed_keys) [R]
- [Ip_set](#ip_set) [CRUD]
- [Managed_products_by_vendor](#managed_products_by_vendor) [R]
- [Web_acl_for_resource](#web_acl_for_resource) [R]
- [Rule_group](#rule_group) [CRUD]
- [Managed_rule_set_version_expiry_date](#managed_rule_set_version_expiry_date) [U]
- [Managed_rule_set](#managed_rule_set) [R]
- [Managed_rule_set_versions](#managed_rule_set_versions) [C]
- [Web_acl](#web_acl) [CRUD]

---

## Resources


### Firewall_manager_rule_groups

FirewallManagerRuleGroups resource

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


### Permission_policy

PermissionPolicy resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `policy` | String | ✅ | <p>The policy to attach to the specified rule group. </p>
         <p>The policy specifications must conform to the following:</p>
         <ul>
            <li>
               <p>The policy must be composed using IAM Policy version 2012-10-17.</p>
            </li>
            <li>
               <p>The policy must include specifications for <code>Effect</code>, <code>Action</code>, and <code>Principal</code>.</p>
            </li>
            <li>
               <p>
                  <code>Effect</code> must specify <code>Allow</code>.</p>
            </li>
            <li>
               <p>
                  <code>Action</code> must specify <code>wafv2:CreateWebACL</code>, <code>wafv2:UpdateWebACL</code>, and 
             <code>wafv2:PutFirewallManagerRuleGroups</code> and may optionally specify <code>wafv2:GetRuleGroup</code>. 
                 WAF rejects any extra actions or wildcard actions in the policy.</p>
            </li>
            <li>
               <p>The policy must not include a <code>Resource</code> parameter.</p>
            </li>
         </ul>
         <p>For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies.html">IAM Policies</a>.  </p> |
| `resource_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the <a>RuleGroup</a> to which you want to
         attach the policy.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `policy` | String | <p>The IAM policy that is attached to the specified rule group.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create permission_policy
permission_policy = provider.wafv2.Permission_policy {
    policy = "value"  # <p>The policy to attach to the specified rule group. </p>
         <p>The policy specifications must conform to the following:</p>
         <ul>
            <li>
               <p>The policy must be composed using IAM Policy version 2012-10-17.</p>
            </li>
            <li>
               <p>The policy must include specifications for <code>Effect</code>, <code>Action</code>, and <code>Principal</code>.</p>
            </li>
            <li>
               <p>
                  <code>Effect</code> must specify <code>Allow</code>.</p>
            </li>
            <li>
               <p>
                  <code>Action</code> must specify <code>wafv2:CreateWebACL</code>, <code>wafv2:UpdateWebACL</code>, and 
             <code>wafv2:PutFirewallManagerRuleGroups</code> and may optionally specify <code>wafv2:GetRuleGroup</code>. 
                 WAF rejects any extra actions or wildcard actions in the policy.</p>
            </li>
            <li>
               <p>The policy must not include a <code>Resource</code> parameter.</p>
            </li>
         </ul>
         <p>For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies.html">IAM Policies</a>.  </p>
    resource_arn = "value"  # <p>The Amazon Resource Name (ARN) of the <a>RuleGroup</a> to which you want to
         attach the policy.</p>
}

# Access permission_policy outputs
permission_policy_id = permission_policy.id
permission_policy_policy = permission_policy.policy
```

---


### Managed_rule_group

ManagedRuleGroup resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `version_name` | String | <p>The managed rule group's version. </p> |
| `capacity` | i64 | <p>The web ACL capacity units (WCUs) required for this rule group.</p>
         <p>WAF uses WCUs to calculate and control the operating
         resources that are used to run your rules, rule groups, and web ACLs. WAF
         calculates capacity differently for each rule type, to reflect the relative cost of each rule. 
         Simple rules that cost little to run use fewer WCUs than more complex rules
				that use more processing power. 
				Rule group capacity is fixed at creation, which helps users plan their  
         web ACL WCU usage when they use a rule group. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/aws-waf-capacity-units.html">WAF web ACL capacity units (WCU)</a> 
    in the <i>WAF Developer Guide</i>. </p> |
| `rules` | Vec<String> | <p></p> |
| `label_namespace` | String | <p>The label namespace prefix for this rule group. All labels added by rules in this rule group have this prefix. </p>
         <ul>
            <li>
               <p>The syntax for the label namespace prefix for a managed rule group is the following: </p>
               <p>
                  <code>awswaf:managed:<vendor>:<rule group name></code>:</p>
            </li>
            <li>
               <p>When a rule with a label matches a web request, WAF adds the fully qualified label to the request. A fully qualified label is made up of the label namespace from the rule group or web ACL where the rule is defined and the label from the rule, separated by a colon: </p>
               <p>
                  <code><label namespace>:<label from rule></code>
               </p>
            </li>
         </ul> |
| `available_labels` | Vec<String> | <p>The labels that one or more rules in this rule group add to matching web requests. These labels are defined in the <code>RuleLabels</code> for a <a>Rule</a>.</p> |
| `sns_topic_arn` | String | <p>The Amazon resource name (ARN) of the Amazon Simple Notification Service SNS topic that's used to provide notification of changes
         to the managed rule group. You can subscribe to the SNS topic to receive notifications when
         the managed rule group is modified, such as for new versions and for version expiration.
         For more information, see the <a href="https://docs.aws.amazon.com/sns/latest/dg/welcome.html">Amazon Simple Notification Service Developer Guide</a>.</p> |
| `consumed_labels` | Vec<String> | <p>The labels that one or more rules in this rule group match against in label match statements. These labels are defined in a <code>LabelMatchStatement</code> specification, in the <a>Statement</a> definition of a rule.  </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access managed_rule_group outputs
managed_rule_group_id = managed_rule_group.id
managed_rule_group_version_name = managed_rule_group.version_name
managed_rule_group_capacity = managed_rule_group.capacity
managed_rule_group_rules = managed_rule_group.rules
managed_rule_group_label_namespace = managed_rule_group.label_namespace
managed_rule_group_available_labels = managed_rule_group.available_labels
managed_rule_group_sns_topic_arn = managed_rule_group.sns_topic_arn
managed_rule_group_consumed_labels = managed_rule_group.consumed_labels
```

---


### Logging_configuration

LoggingConfiguration resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `logging_configuration` | String | ✅ | <p></p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `logging_configuration` | String | <p>The <a>LoggingConfiguration</a> for the specified web ACL.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create logging_configuration
logging_configuration = provider.wafv2.Logging_configuration {
    logging_configuration = "value"  # <p></p>
}

# Access logging_configuration outputs
logging_configuration_id = logging_configuration.id
logging_configuration_logging_configuration = logging_configuration.logging_configuration
```

---


### Mobile_sdk_release

MobileSdkRelease resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `mobile_sdk_release` | String | <p>Information for a specified SDK release, including release notes and tags.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access mobile_sdk_release outputs
mobile_sdk_release_id = mobile_sdk_release.id
mobile_sdk_release_mobile_sdk_release = mobile_sdk_release.mobile_sdk_release
```

---


### Decrypted_api_key

DecryptedAPIKey resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `token_domains` | Vec<String> | <p>The token domains that are defined in this API key. </p> |
| `creation_timestamp` | String | <p>The date and time that the key was created. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access decrypted_api_key outputs
decrypted_api_key_id = decrypted_api_key.id
decrypted_api_key_token_domains = decrypted_api_key.token_domains
decrypted_api_key_creation_timestamp = decrypted_api_key.creation_timestamp
```

---


### Api_key

APIKey resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `scope` | String | ✅ | <p>Specifies whether this is for a global resource type, such as a Amazon CloudFront distribution. For an Amplify application, use <code>CLOUDFRONT</code>.</p>
         <p>To work with CloudFront, you must also specify the Region US East (N. Virginia) as follows: </p>
         <ul>
            <li>
               <p>CLI - Specify the Region when you use the CloudFront scope: <code>--scope=CLOUDFRONT --region=us-east-1</code>. </p>
            </li>
            <li>
               <p>API and SDKs - For all calls, use the Region endpoint us-east-1. </p>
            </li>
         </ul> |
| `token_domains` | Vec<String> | ✅ | <p>The client application domains that you want to use this API key for.  </p>
         <p>Example JSON: <code>"TokenDomains": ["abc.com", "store.abc.com"]</code>
         </p>
         <p>Public suffixes aren't allowed. For example, you can't use <code>gov.au</code> or <code>co.uk</code> as token domains.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create api_key
api_key = provider.wafv2.Api_key {
    scope = "value"  # <p>Specifies whether this is for a global resource type, such as a Amazon CloudFront distribution. For an Amplify application, use <code>CLOUDFRONT</code>.</p>
         <p>To work with CloudFront, you must also specify the Region US East (N. Virginia) as follows: </p>
         <ul>
            <li>
               <p>CLI - Specify the Region when you use the CloudFront scope: <code>--scope=CLOUDFRONT --region=us-east-1</code>. </p>
            </li>
            <li>
               <p>API and SDKs - For all calls, use the Region endpoint us-east-1. </p>
            </li>
         </ul>
    token_domains = "value"  # <p>The client application domains that you want to use this API key for.  </p>
         <p>Example JSON: <code>"TokenDomains": ["abc.com", "store.abc.com"]</code>
         </p>
         <p>Public suffixes aren't allowed. For example, you can't use <code>gov.au</code> or <code>co.uk</code> as token domains.</p>
}

```

---


### Sampled_requests

SampledRequests resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `population_size` | i64 | <p>The total number of requests from which <code>GetSampledRequests</code> got a sample of
            <code>MaxItems</code> requests. If <code>PopulationSize</code> is less than
            <code>MaxItems</code>, the sample includes every request that your Amazon Web Services resource
         received during the specified time range.</p> |
| `time_window` | String | <p>Usually, <code>TimeWindow</code> is the time range that you specified in the
            <code>GetSampledRequests</code> request. However, if your Amazon Web Services resource received more
         than 5,000 requests during the time range that you specified in the request,
            <code>GetSampledRequests</code> returns the time range for the first 5,000 requests.
         Times are in Coordinated Universal Time (UTC) format.</p> |
| `sampled_requests` | Vec<String> | <p>A complex type that contains detailed information about each of the requests in the
         sample.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access sampled_requests outputs
sampled_requests_id = sampled_requests.id
sampled_requests_population_size = sampled_requests.population_size
sampled_requests_time_window = sampled_requests.time_window
sampled_requests_sampled_requests = sampled_requests.sampled_requests
```

---


### All_managed_products

AllManagedProducts resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `managed_products` | Vec<String> | <p>High-level information for the Amazon Web Services Managed Rules rule groups and Amazon Web Services Marketplace managed rule groups. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access all_managed_products outputs
all_managed_products_id = all_managed_products.id
all_managed_products_managed_products = all_managed_products.managed_products
```

---


### Regex_pattern_set

RegexPatternSet resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `scope` | String | ✅ | <p>Specifies whether this is for a global resource type, such as a Amazon CloudFront distribution. For an Amplify application, use <code>CLOUDFRONT</code>.</p>
         <p>To work with CloudFront, you must also specify the Region US East (N. Virginia) as follows: </p>
         <ul>
            <li>
               <p>CLI - Specify the Region when you use the CloudFront scope: <code>--scope=CLOUDFRONT --region=us-east-1</code>. </p>
            </li>
            <li>
               <p>API and SDKs - For all calls, use the Region endpoint us-east-1. </p>
            </li>
         </ul> |
| `regular_expression_list` | Vec<String> | ✅ | <p>Array of regular expression strings. </p> |
| `tags` | Vec<String> |  | <p>An array of key:value pairs to associate with the resource.</p> |
| `name` | String | ✅ | <p>The name of the set. You cannot change the name after you create the set.</p> |
| `description` | String |  | <p>A description of the set that helps with identification. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `regex_pattern_set` | String | <p></p> |
| `lock_token` | String | <p>A token used for optimistic locking. WAF returns a token to your <code>get</code> and <code>list</code> requests, to mark the state of the entity at the time of the request. To make changes to the entity associated with the token, you provide the token to operations like <code>update</code> and <code>delete</code>. WAF uses the token to ensure that no changes have been made to the entity since you last retrieved it. If a change has been made, the update fails with a <code>WAFOptimisticLockException</code>. If this happens, perform another <code>get</code>, and use the new token returned by that operation. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create regex_pattern_set
regex_pattern_set = provider.wafv2.Regex_pattern_set {
    scope = "value"  # <p>Specifies whether this is for a global resource type, such as a Amazon CloudFront distribution. For an Amplify application, use <code>CLOUDFRONT</code>.</p>
         <p>To work with CloudFront, you must also specify the Region US East (N. Virginia) as follows: </p>
         <ul>
            <li>
               <p>CLI - Specify the Region when you use the CloudFront scope: <code>--scope=CLOUDFRONT --region=us-east-1</code>. </p>
            </li>
            <li>
               <p>API and SDKs - For all calls, use the Region endpoint us-east-1. </p>
            </li>
         </ul>
    regular_expression_list = "value"  # <p>Array of regular expression strings. </p>
    name = "value"  # <p>The name of the set. You cannot change the name after you create the set.</p>
}

# Access regex_pattern_set outputs
regex_pattern_set_id = regex_pattern_set.id
regex_pattern_set_regex_pattern_set = regex_pattern_set.regex_pattern_set
regex_pattern_set_lock_token = regex_pattern_set.lock_token
```

---


### Rate_based_statement_managed_keys

RateBasedStatementManagedKeys resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `managed_keys_ipv6` | String | <p>The keys that are of Internet Protocol version 6 (IPv6). </p> |
| `managed_keys_ipv4` | String | <p>The keys that are of Internet Protocol version 4 (IPv4). </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access rate_based_statement_managed_keys outputs
rate_based_statement_managed_keys_id = rate_based_statement_managed_keys.id
rate_based_statement_managed_keys_managed_keys_ipv6 = rate_based_statement_managed_keys.managed_keys_ipv6
rate_based_statement_managed_keys_managed_keys_ipv4 = rate_based_statement_managed_keys.managed_keys_ipv4
```

---


### Ip_set

IPSet resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>The name of the IP set. You cannot change the name of an <code>IPSet</code> after you create it.</p> |
| `description` | String |  | <p>A description of the IP set that helps with identification. </p> |
| `scope` | String | ✅ | <p>Specifies whether this is for a global resource type, such as a Amazon CloudFront distribution. For an Amplify application, use <code>CLOUDFRONT</code>.</p>
         <p>To work with CloudFront, you must also specify the Region US East (N. Virginia) as follows: </p>
         <ul>
            <li>
               <p>CLI - Specify the Region when you use the CloudFront scope: <code>--scope=CLOUDFRONT --region=us-east-1</code>. </p>
            </li>
            <li>
               <p>API and SDKs - For all calls, use the Region endpoint us-east-1. </p>
            </li>
         </ul> |
| `ip_address_version` | String | ✅ | <p>The version of the IP addresses, either <code>IPV4</code> or <code>IPV6</code>. </p> |
| `addresses` | Vec<String> | ✅ | <p>Contains an array of strings that specifies zero or more IP addresses or blocks of IP addresses that you want WAF to inspect for in incoming requests. All addresses must be specified using Classless Inter-Domain Routing (CIDR) notation. WAF supports all IPv4 and IPv6 CIDR ranges except for <code>/0</code>. </p>
         <p>Example address strings: </p>
         <ul>
            <li>
               <p>For requests that originated from the IP address 192.0.2.44, specify <code>192.0.2.44/32</code>.</p>
            </li>
            <li>
               <p>For requests that originated from IP addresses from 192.0.2.0 to 192.0.2.255, specify 
               <code>192.0.2.0/24</code>.</p>
            </li>
            <li>
               <p>For requests that originated from the IP address 1111:0000:0000:0000:0000:0000:0000:0111, specify <code>1111:0000:0000:0000:0000:0000:0000:0111/128</code>.</p>
            </li>
            <li>
               <p>For requests that originated from IP addresses 1111:0000:0000:0000:0000:0000:0000:0000 to 1111:0000:0000:0000:ffff:ffff:ffff:ffff, specify <code>1111:0000:0000:0000:0000:0000:0000:0000/64</code>.</p>
            </li>
         </ul>
         <p>For more information about CIDR notation, see the Wikipedia entry <a href="https://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing">Classless Inter-Domain Routing</a>.</p>
         <p>Example JSON <code>Addresses</code> specifications: </p>
         <ul>
            <li>
               <p>Empty array: <code>"Addresses": []</code>
               </p>
            </li>
            <li>
               <p>Array with one address: <code>"Addresses": ["192.0.2.44/32"]</code>
               </p>
            </li>
            <li>
               <p>Array with three addresses: <code>"Addresses": ["192.0.2.44/32", "192.0.2.0/24", "192.0.0.0/16"]</code>
               </p>
            </li>
            <li>
               <p>INVALID specification: <code>"Addresses": [""]</code> INVALID </p>
            </li>
         </ul> |
| `tags` | Vec<String> |  | <p>An array of key:value pairs to associate with the resource.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `ip_set` | String | <p></p> |
| `lock_token` | String | <p>A token used for optimistic locking. WAF returns a token to your <code>get</code> and <code>list</code> requests, to mark the state of the entity at the time of the request. To make changes to the entity associated with the token, you provide the token to operations like <code>update</code> and <code>delete</code>. WAF uses the token to ensure that no changes have been made to the entity since you last retrieved it. If a change has been made, the update fails with a <code>WAFOptimisticLockException</code>. If this happens, perform another <code>get</code>, and use the new token returned by that operation. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create ip_set
ip_set = provider.wafv2.Ip_set {
    name = "value"  # <p>The name of the IP set. You cannot change the name of an <code>IPSet</code> after you create it.</p>
    scope = "value"  # <p>Specifies whether this is for a global resource type, such as a Amazon CloudFront distribution. For an Amplify application, use <code>CLOUDFRONT</code>.</p>
         <p>To work with CloudFront, you must also specify the Region US East (N. Virginia) as follows: </p>
         <ul>
            <li>
               <p>CLI - Specify the Region when you use the CloudFront scope: <code>--scope=CLOUDFRONT --region=us-east-1</code>. </p>
            </li>
            <li>
               <p>API and SDKs - For all calls, use the Region endpoint us-east-1. </p>
            </li>
         </ul>
    ip_address_version = "value"  # <p>The version of the IP addresses, either <code>IPV4</code> or <code>IPV6</code>. </p>
    addresses = "value"  # <p>Contains an array of strings that specifies zero or more IP addresses or blocks of IP addresses that you want WAF to inspect for in incoming requests. All addresses must be specified using Classless Inter-Domain Routing (CIDR) notation. WAF supports all IPv4 and IPv6 CIDR ranges except for <code>/0</code>. </p>
         <p>Example address strings: </p>
         <ul>
            <li>
               <p>For requests that originated from the IP address 192.0.2.44, specify <code>192.0.2.44/32</code>.</p>
            </li>
            <li>
               <p>For requests that originated from IP addresses from 192.0.2.0 to 192.0.2.255, specify 
               <code>192.0.2.0/24</code>.</p>
            </li>
            <li>
               <p>For requests that originated from the IP address 1111:0000:0000:0000:0000:0000:0000:0111, specify <code>1111:0000:0000:0000:0000:0000:0000:0111/128</code>.</p>
            </li>
            <li>
               <p>For requests that originated from IP addresses 1111:0000:0000:0000:0000:0000:0000:0000 to 1111:0000:0000:0000:ffff:ffff:ffff:ffff, specify <code>1111:0000:0000:0000:0000:0000:0000:0000/64</code>.</p>
            </li>
         </ul>
         <p>For more information about CIDR notation, see the Wikipedia entry <a href="https://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing">Classless Inter-Domain Routing</a>.</p>
         <p>Example JSON <code>Addresses</code> specifications: </p>
         <ul>
            <li>
               <p>Empty array: <code>"Addresses": []</code>
               </p>
            </li>
            <li>
               <p>Array with one address: <code>"Addresses": ["192.0.2.44/32"]</code>
               </p>
            </li>
            <li>
               <p>Array with three addresses: <code>"Addresses": ["192.0.2.44/32", "192.0.2.0/24", "192.0.0.0/16"]</code>
               </p>
            </li>
            <li>
               <p>INVALID specification: <code>"Addresses": [""]</code> INVALID </p>
            </li>
         </ul>
}

# Access ip_set outputs
ip_set_id = ip_set.id
ip_set_ip_set = ip_set.ip_set
ip_set_lock_token = ip_set.lock_token
```

---


### Managed_products_by_vendor

ManagedProductsByVendor resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `managed_products` | Vec<String> | <p>High-level information for the managed rule groups owned by the specified vendor.  </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access managed_products_by_vendor outputs
managed_products_by_vendor_id = managed_products_by_vendor.id
managed_products_by_vendor_managed_products = managed_products_by_vendor.managed_products
```

---


### Web_acl_for_resource

WebACLForResource resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `web_acl` | String | <p>The web ACL that is associated with the resource. If there is no associated resource,
         WAF returns a null web ACL.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access web_acl_for_resource outputs
web_acl_for_resource_id = web_acl_for_resource.id
web_acl_for_resource_web_acl = web_acl_for_resource.web_acl
```

---


### Rule_group

RuleGroup resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>An array of key:value pairs to associate with the resource.</p> |
| `visibility_config` | String | ✅ | <p>Defines and enables Amazon CloudWatch metrics and web request sample collection.  </p> |
| `custom_response_bodies` | HashMap<String, String> |  | <p>A map of custom response keys and content bodies. When you create a rule with a block action, you can send a custom response to the web request. You define these for the rule group, and then use them in the rules that you define in the rule group. </p>
         <p>For information about customizing web requests and responses, 
           see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-custom-request-response.html">Customizing web requests and responses in WAF</a> 
    in the <i>WAF Developer Guide</i>. </p>
         <p>For information about the limits on count and size for custom request and response settings, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/limits.html">WAF quotas</a> 
     in the <i>WAF Developer Guide</i>. </p> |
| `capacity` | i64 | ✅ | <p>The web ACL capacity units (WCUs) required for this rule group.</p>
         <p>When you create your own rule group, you define this, and you cannot change it after creation. 
          When you add or modify the rules in a rule group, WAF enforces this limit. You can check the capacity 
          for a set of rules using <a>CheckCapacity</a>.</p>
         <p>WAF uses WCUs to calculate and control the operating
         resources that are used to run your rules, rule groups, and web ACLs. WAF
         calculates capacity differently for each rule type, to reflect the relative cost of each rule. 
         Simple rules that cost little to run use fewer WCUs than more complex rules
				that use more processing power. 
				Rule group capacity is fixed at creation, which helps users plan their  
         web ACL WCU usage when they use a rule group. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/aws-waf-capacity-units.html">WAF web ACL capacity units (WCU)</a> 
    in the <i>WAF Developer Guide</i>. </p> |
| `description` | String |  | <p>A description of the rule group that helps with identification. </p> |
| `rules` | Vec<String> |  | <p>The <a>Rule</a> statements used to identify the web requests that you 
         want to manage. Each rule includes one top-level statement that WAF uses to identify matching  
         web requests, and parameters that govern how WAF handles them. 
      </p> |
| `scope` | String | ✅ | <p>Specifies whether this is for a global resource type, such as a Amazon CloudFront distribution. For an Amplify application, use <code>CLOUDFRONT</code>.</p>
         <p>To work with CloudFront, you must also specify the Region US East (N. Virginia) as follows: </p>
         <ul>
            <li>
               <p>CLI - Specify the Region when you use the CloudFront scope: <code>--scope=CLOUDFRONT --region=us-east-1</code>. </p>
            </li>
            <li>
               <p>API and SDKs - For all calls, use the Region endpoint us-east-1. </p>
            </li>
         </ul> |
| `name` | String | ✅ | <p>The name of the rule group. You cannot change the name of a rule group after you create it.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `lock_token` | String | <p>A token used for optimistic locking. WAF returns a token to your <code>get</code> and <code>list</code> requests, to mark the state of the entity at the time of the request. To make changes to the entity associated with the token, you provide the token to operations like <code>update</code> and <code>delete</code>. WAF uses the token to ensure that no changes have been made to the entity since you last retrieved it. If a change has been made, the update fails with a <code>WAFOptimisticLockException</code>. If this happens, perform another <code>get</code>, and use the new token returned by that operation. </p> |
| `rule_group` | String | <p></p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create rule_group
rule_group = provider.wafv2.Rule_group {
    visibility_config = "value"  # <p>Defines and enables Amazon CloudWatch metrics and web request sample collection.  </p>
    capacity = "value"  # <p>The web ACL capacity units (WCUs) required for this rule group.</p>
         <p>When you create your own rule group, you define this, and you cannot change it after creation. 
          When you add or modify the rules in a rule group, WAF enforces this limit. You can check the capacity 
          for a set of rules using <a>CheckCapacity</a>.</p>
         <p>WAF uses WCUs to calculate and control the operating
         resources that are used to run your rules, rule groups, and web ACLs. WAF
         calculates capacity differently for each rule type, to reflect the relative cost of each rule. 
         Simple rules that cost little to run use fewer WCUs than more complex rules
				that use more processing power. 
				Rule group capacity is fixed at creation, which helps users plan their  
         web ACL WCU usage when they use a rule group. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/aws-waf-capacity-units.html">WAF web ACL capacity units (WCU)</a> 
    in the <i>WAF Developer Guide</i>. </p>
    scope = "value"  # <p>Specifies whether this is for a global resource type, such as a Amazon CloudFront distribution. For an Amplify application, use <code>CLOUDFRONT</code>.</p>
         <p>To work with CloudFront, you must also specify the Region US East (N. Virginia) as follows: </p>
         <ul>
            <li>
               <p>CLI - Specify the Region when you use the CloudFront scope: <code>--scope=CLOUDFRONT --region=us-east-1</code>. </p>
            </li>
            <li>
               <p>API and SDKs - For all calls, use the Region endpoint us-east-1. </p>
            </li>
         </ul>
    name = "value"  # <p>The name of the rule group. You cannot change the name of a rule group after you create it.</p>
}

# Access rule_group outputs
rule_group_id = rule_group.id
rule_group_lock_token = rule_group.lock_token
rule_group_rule_group = rule_group.rule_group
```

---


### Managed_rule_set_version_expiry_date

ManagedRuleSetVersionExpiryDate resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `version_to_expire` | String | ✅ | <p>The version that you want to remove from your list of offerings for the named managed
         rule group. </p> |
| `scope` | String | ✅ | <p>Specifies whether this is for a global resource type, such as a Amazon CloudFront distribution. For an Amplify application, use <code>CLOUDFRONT</code>.</p>
         <p>To work with CloudFront, you must also specify the Region US East (N. Virginia) as follows: </p>
         <ul>
            <li>
               <p>CLI - Specify the Region when you use the CloudFront scope: <code>--scope=CLOUDFRONT --region=us-east-1</code>. </p>
            </li>
            <li>
               <p>API and SDKs - For all calls, use the Region endpoint us-east-1. </p>
            </li>
         </ul> |
| `id` | String | ✅ | <p>A unique identifier for the managed rule set. The ID is returned in the responses to commands like <code>list</code>. You provide it to operations like <code>get</code> and <code>update</code>.</p> |
| `lock_token` | String | ✅ | <p>A token used for optimistic locking. WAF returns a token to your <code>get</code> and <code>list</code> requests, to mark the state of the entity at the time of the request. To make changes to the entity associated with the token, you provide the token to operations like <code>update</code> and <code>delete</code>. WAF uses the token to ensure that no changes have been made to the entity since you last retrieved it. If a change has been made, the update fails with a <code>WAFOptimisticLockException</code>. If this happens, perform another <code>get</code>, and use the new token returned by that operation. </p> |
| `name` | String | ✅ | <p>The name of the managed rule set. You use this, along with the rule set ID, to identify the rule set.</p>
         <p>This name is assigned to the corresponding managed rule group, which your customers can access and use. </p> |
| `expiry_timestamp` | String | ✅ | <p>The time that you want the version to expire.</p>
         <p>Times are in Coordinated Universal Time (UTC) format. UTC format includes the special designator, Z. For example, "2016-09-27T14:50Z". </p> |



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


### Managed_rule_set

ManagedRuleSet resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `lock_token` | String | <p>A token used for optimistic locking. WAF returns a token to your <code>get</code> and <code>list</code> requests, to mark the state of the entity at the time of the request. To make changes to the entity associated with the token, you provide the token to operations like <code>update</code> and <code>delete</code>. WAF uses the token to ensure that no changes have been made to the entity since you last retrieved it. If a change has been made, the update fails with a <code>WAFOptimisticLockException</code>. If this happens, perform another <code>get</code>, and use the new token returned by that operation. </p> |
| `managed_rule_set` | String | <p>The managed rule set that you requested. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access managed_rule_set outputs
managed_rule_set_id = managed_rule_set.id
managed_rule_set_lock_token = managed_rule_set.lock_token
managed_rule_set_managed_rule_set = managed_rule_set.managed_rule_set
```

---


### Managed_rule_set_versions

ManagedRuleSetVersions resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | String | ✅ | <p>A unique identifier for the managed rule set. The ID is returned in the responses to commands like <code>list</code>. You provide it to operations like <code>get</code> and <code>update</code>.</p> |
| `lock_token` | String | ✅ | <p>A token used for optimistic locking. WAF returns a token to your <code>get</code> and <code>list</code> requests, to mark the state of the entity at the time of the request. To make changes to the entity associated with the token, you provide the token to operations like <code>update</code> and <code>delete</code>. WAF uses the token to ensure that no changes have been made to the entity since you last retrieved it. If a change has been made, the update fails with a <code>WAFOptimisticLockException</code>. If this happens, perform another <code>get</code>, and use the new token returned by that operation. </p> |
| `recommended_version` | String |  | <p>The version of the named managed rule group that you'd like your customers to choose,
         from among your version offerings. </p> |
| `name` | String | ✅ | <p>The name of the managed rule set. You use this, along with the rule set ID, to identify the rule set.</p>
         <p>This name is assigned to the corresponding managed rule group, which your customers can access and use. </p> |
| `scope` | String | ✅ | <p>Specifies whether this is for a global resource type, such as a Amazon CloudFront distribution. For an Amplify application, use <code>CLOUDFRONT</code>.</p>
         <p>To work with CloudFront, you must also specify the Region US East (N. Virginia) as follows: </p>
         <ul>
            <li>
               <p>CLI - Specify the Region when you use the CloudFront scope: <code>--scope=CLOUDFRONT --region=us-east-1</code>. </p>
            </li>
            <li>
               <p>API and SDKs - For all calls, use the Region endpoint us-east-1. </p>
            </li>
         </ul> |
| `versions_to_publish` | HashMap<String, String> |  | <p>The versions of the named managed rule group that you want to offer to your customers.
      </p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create managed_rule_set_versions
managed_rule_set_versions = provider.wafv2.Managed_rule_set_versions {
    id = "value"  # <p>A unique identifier for the managed rule set. The ID is returned in the responses to commands like <code>list</code>. You provide it to operations like <code>get</code> and <code>update</code>.</p>
    lock_token = "value"  # <p>A token used for optimistic locking. WAF returns a token to your <code>get</code> and <code>list</code> requests, to mark the state of the entity at the time of the request. To make changes to the entity associated with the token, you provide the token to operations like <code>update</code> and <code>delete</code>. WAF uses the token to ensure that no changes have been made to the entity since you last retrieved it. If a change has been made, the update fails with a <code>WAFOptimisticLockException</code>. If this happens, perform another <code>get</code>, and use the new token returned by that operation. </p>
    name = "value"  # <p>The name of the managed rule set. You use this, along with the rule set ID, to identify the rule set.</p>
         <p>This name is assigned to the corresponding managed rule group, which your customers can access and use. </p>
    scope = "value"  # <p>Specifies whether this is for a global resource type, such as a Amazon CloudFront distribution. For an Amplify application, use <code>CLOUDFRONT</code>.</p>
         <p>To work with CloudFront, you must also specify the Region US East (N. Virginia) as follows: </p>
         <ul>
            <li>
               <p>CLI - Specify the Region when you use the CloudFront scope: <code>--scope=CLOUDFRONT --region=us-east-1</code>. </p>
            </li>
            <li>
               <p>API and SDKs - For all calls, use the Region endpoint us-east-1. </p>
            </li>
         </ul>
}

```

---


### Web_acl

WebACL resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `on_source_d_do_s_protection_config` | String |  | <p>Specifies the type of DDoS protection to apply to web request data for a web ACL. For most scenarios, it is recommended to use the default protection level, <code>ACTIVE_UNDER_DDOS</code>. 
   If a web ACL is associated with multiple Application Load Balancers, the changes you make to DDoS protection in that web ACL will apply to all associated Application Load Balancers.</p> |
| `captcha_config` | String |  | <p>Specifies how WAF should handle <code>CAPTCHA</code> evaluations for rules that don't have their own <code>CaptchaConfig</code> settings. If you don't specify this, WAF uses its default settings for <code>CaptchaConfig</code>. </p> |
| `rules` | Vec<String> |  | <p>The <a>Rule</a> statements used to identify the web requests that you 
         want to manage. Each rule includes one top-level statement that WAF uses to identify matching  
         web requests, and parameters that govern how WAF handles them. 
      </p> |
| `data_protection_config` | String |  | <p>Specifies data protection to apply to the web request data for the web ACL. This is a web ACL level data protection option. </p>
         <p>The data protection that you configure for the web ACL alters the data that's available for any other data collection activity, 
  including your WAF logging destinations, web ACL request sampling, and Amazon Security Lake data collection and management. Your other option for data protection is in the logging configuration, which only affects logging. </p> |
| `visibility_config` | String | ✅ | <p>Defines and enables Amazon CloudWatch metrics and web request sample collection.  </p> |
| `default_action` | String | ✅ | <p>The action to perform if none of the <code>Rules</code> contained in the <code>WebACL</code> match. </p> |
| `name` | String | ✅ | <p>The name of the web ACL. You cannot change the name of a web ACL after you create it.</p> |
| `association_config` | String |  | <p>Specifies custom configurations for the associations between the web ACL and protected resources.  </p>
         <p>Use this to customize the maximum size of the request body that your protected resources forward to WAF for inspection. You can 
    customize this setting for CloudFront, API Gateway, Amazon Cognito, App Runner, or Verified Access resources. The default setting is 16 KB (16,384 bytes). </p>
         <note>
            <p>You are charged additional fees when your protected resources forward body sizes that are larger than the default. For more information, see <a href="http://aws.amazon.com/waf/pricing/">WAF Pricing</a>.</p>
         </note>
         <p>For Application Load Balancer and AppSync, the limit is fixed at 8 KB (8,192 bytes).</p> |
| `tags` | Vec<String> |  | <p>An array of key:value pairs to associate with the resource.</p> |
| `token_domains` | Vec<String> |  | <p>Specifies the domains that WAF should accept in a web request token. This enables the use of tokens across multiple protected websites. When WAF provides a token, it uses the domain of the Amazon Web Services resource that the web ACL is protecting. If you don't specify a list of token domains, WAF accepts tokens only for the domain of the protected resource. With a token domain list, WAF accepts the resource's host domain plus all domains in the token domain list, including their prefixed subdomains.</p>
         <p>Example JSON: <code>"TokenDomains": { "mywebsite.com", "myotherwebsite.com" }</code>
         </p>
         <p>Public suffixes aren't allowed. For example, you can't use <code>gov.au</code> or <code>co.uk</code> as token domains.</p> |
| `application_config` | String |  | <p>Configures the ability for the WAF console to store and retrieve application attributes during the web ACL creation process. Application attributes help WAF give recommendations for protection packs.</p> |
| `description` | String |  | <p>A description of the web ACL that helps with identification. </p> |
| `challenge_config` | String |  | <p>Specifies how WAF should handle challenge evaluations for rules that don't have 
their own <code>ChallengeConfig</code> settings. If you don't specify this, WAF uses its default settings for <code>ChallengeConfig</code>. </p> |
| `scope` | String | ✅ | <p>Specifies whether this is for a global resource type, such as a Amazon CloudFront distribution. For an Amplify application, use <code>CLOUDFRONT</code>.</p>
         <p>To work with CloudFront, you must also specify the Region US East (N. Virginia) as follows: </p>
         <ul>
            <li>
               <p>CLI - Specify the Region when you use the CloudFront scope: <code>--scope=CLOUDFRONT --region=us-east-1</code>. </p>
            </li>
            <li>
               <p>API and SDKs - For all calls, use the Region endpoint us-east-1. </p>
            </li>
         </ul> |
| `custom_response_bodies` | HashMap<String, String> |  | <p>A map of custom response keys and content bodies. When you create a rule with a block action, you can send a custom response to the web request. You define these for the web ACL, and then use them in the rules and default actions that you define in the web ACL. </p>
         <p>For information about customizing web requests and responses, 
           see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-custom-request-response.html">Customizing web requests and responses in WAF</a> 
    in the <i>WAF Developer Guide</i>. </p>
         <p>For information about the limits on count and size for custom request and response settings, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/limits.html">WAF quotas</a> 
     in the <i>WAF Developer Guide</i>. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `web_acl` | String | <p>The web ACL specification. You can modify the settings in this web ACL and use it to
         update this web ACL or create a new one.</p> |
| `lock_token` | String | <p>A token used for optimistic locking. WAF returns a token to your <code>get</code> and <code>list</code> requests, to mark the state of the entity at the time of the request. To make changes to the entity associated with the token, you provide the token to operations like <code>update</code> and <code>delete</code>. WAF uses the token to ensure that no changes have been made to the entity since you last retrieved it. If a change has been made, the update fails with a <code>WAFOptimisticLockException</code>. If this happens, perform another <code>get</code>, and use the new token returned by that operation. </p> |
| `application_integration_url` | String | <p>The URL to use in SDK integrations with Amazon Web Services managed rule groups. For example, you can use the integration SDKs with the account takeover prevention managed rule group <code>AWSManagedRulesATPRuleSet</code> and the account creation fraud prevention managed rule group <code>AWSManagedRulesACFPRuleSet</code>. This is only populated if you are using a rule group in your web ACL that integrates with your applications in this way. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-application-integration.html">WAF client application integration</a> 
in the <i>WAF Developer Guide</i>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create web_acl
web_acl = provider.wafv2.Web_acl {
    visibility_config = "value"  # <p>Defines and enables Amazon CloudWatch metrics and web request sample collection.  </p>
    default_action = "value"  # <p>The action to perform if none of the <code>Rules</code> contained in the <code>WebACL</code> match. </p>
    name = "value"  # <p>The name of the web ACL. You cannot change the name of a web ACL after you create it.</p>
    scope = "value"  # <p>Specifies whether this is for a global resource type, such as a Amazon CloudFront distribution. For an Amplify application, use <code>CLOUDFRONT</code>.</p>
         <p>To work with CloudFront, you must also specify the Region US East (N. Virginia) as follows: </p>
         <ul>
            <li>
               <p>CLI - Specify the Region when you use the CloudFront scope: <code>--scope=CLOUDFRONT --region=us-east-1</code>. </p>
            </li>
            <li>
               <p>API and SDKs - For all calls, use the Region endpoint us-east-1. </p>
            </li>
         </ul>
}

# Access web_acl outputs
web_acl_id = web_acl.id
web_acl_web_acl = web_acl.web_acl
web_acl_lock_token = web_acl.lock_token
web_acl_application_integration_url = web_acl.application_integration_url
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple firewall_manager_rule_groups resources
firewall_manager_rule_groups_0 = provider.wafv2.Firewall_manager_rule_groups {
}
firewall_manager_rule_groups_1 = provider.wafv2.Firewall_manager_rule_groups {
}
firewall_manager_rule_groups_2 = provider.wafv2.Firewall_manager_rule_groups {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    firewall_manager_rule_groups = provider.wafv2.Firewall_manager_rule_groups {
    }
```

---

## Related Documentation

- [AWS Wafv2 Documentation](https://docs.aws.amazon.com/wafv2/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
