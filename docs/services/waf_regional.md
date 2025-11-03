# Waf_regional Service



**Resources**: 20

---

## Overview

The waf_regional service provides access to 20 resource types:

- [Geo_match_set](#geo_match_set) [CRUD]
- [Web_acl_migration_stack](#web_acl_migration_stack) [C]
- [Regex_pattern_set](#regex_pattern_set) [CRUD]
- [Web_acl_for_resource](#web_acl_for_resource) [R]
- [Byte_match_set](#byte_match_set) [CRUD]
- [Rate_based_rule](#rate_based_rule) [CRUD]
- [Rule](#rule) [CRUD]
- [Logging_configuration](#logging_configuration) [CRD]
- [Sql_injection_match_set](#sql_injection_match_set) [CRUD]
- [Change_token](#change_token) [R]
- [Permission_policy](#permission_policy) [CRD]
- [Change_token_status](#change_token_status) [R]
- [Rate_based_rule_managed_keys](#rate_based_rule_managed_keys) [R]
- [Rule_group](#rule_group) [CRUD]
- [Ip_set](#ip_set) [CRUD]
- [Size_constraint_set](#size_constraint_set) [CRUD]
- [Web_acl](#web_acl) [CRUD]
- [Regex_match_set](#regex_match_set) [CRUD]
- [Xss_match_set](#xss_match_set) [CRUD]
- [Sampled_requests](#sampled_requests) [R]

---

## Resources


### Geo_match_set

GeoMatchSet resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>A friendly name or description of the <a>GeoMatchSet</a>. You can't change <code>Name</code> after you create the <code>GeoMatchSet</code>.</p> |
| `change_token` | String | ✅ | <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `geo_match_set` | String | <p>Information about the <a>GeoMatchSet</a> that you specified in the <code>GetGeoMatchSet</code> request. This includes the <code>Type</code>, which for a <code>GeoMatchContraint</code> is always <code>Country</code>, as well as the <code>Value</code>, which is the identifier for a specific country.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create geo_match_set
geo_match_set = provider.waf_regional.Geo_match_set {
    name = "value"  # <p>A friendly name or description of the <a>GeoMatchSet</a>. You can't change <code>Name</code> after you create the <code>GeoMatchSet</code>.</p>
    change_token = "value"  # <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
}

# Access geo_match_set outputs
geo_match_set_id = geo_match_set.id
geo_match_set_geo_match_set = geo_match_set.geo_match_set
```

---


### Web_acl_migration_stack

WebACLMigrationStack resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `web_acl_id` | String | ✅ | <p>The UUID of the WAF Classic web ACL that you want to migrate to WAF v2.</p> |
| `s3_bucket_name` | String | ✅ | <p>The name of the Amazon S3 bucket to store the CloudFormation template in. The S3 bucket must be 
           configured as follows for the migration:  </p>
         <ul>
            <li>
               <p>The bucket name must start with <code>aws-waf-migration-</code>. For example, <code>aws-waf-migration-my-web-acl</code>.</p>
            </li>
            <li>
               <p>The bucket must be in the Region where you are deploying the template. For example, for a web ACL in us-west-2, you must use an Amazon S3 bucket in us-west-2 and you must deploy the template stack to us-west-2. </p>
            </li>
            <li>
               <p>The bucket policies must permit the migration process to write data. For listings of the 
       bucket policies, see the Examples section. </p>
           </li>
         </ul> |
| `ignore_unsupported_type` | bool | ✅ | <p>Indicates whether to exclude entities that can't be migrated or to stop the migration.
           Set this to true to ignore unsupported entities in the web ACL during the migration. Otherwise, if AWS WAF encounters unsupported 
           entities, it stops the process and throws an exception. </p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create web_acl_migration_stack
web_acl_migration_stack = provider.waf_regional.Web_acl_migration_stack {
    web_acl_id = "value"  # <p>The UUID of the WAF Classic web ACL that you want to migrate to WAF v2.</p>
    s3_bucket_name = "value"  # <p>The name of the Amazon S3 bucket to store the CloudFormation template in. The S3 bucket must be 
           configured as follows for the migration:  </p>
         <ul>
            <li>
               <p>The bucket name must start with <code>aws-waf-migration-</code>. For example, <code>aws-waf-migration-my-web-acl</code>.</p>
            </li>
            <li>
               <p>The bucket must be in the Region where you are deploying the template. For example, for a web ACL in us-west-2, you must use an Amazon S3 bucket in us-west-2 and you must deploy the template stack to us-west-2. </p>
            </li>
            <li>
               <p>The bucket policies must permit the migration process to write data. For listings of the 
       bucket policies, see the Examples section. </p>
           </li>
         </ul>
    ignore_unsupported_type = "value"  # <p>Indicates whether to exclude entities that can't be migrated or to stop the migration.
           Set this to true to ignore unsupported entities in the web ACL during the migration. Otherwise, if AWS WAF encounters unsupported 
           entities, it stops the process and throws an exception. </p>
}

```

---


### Regex_pattern_set

RegexPatternSet resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `change_token` | String | ✅ | <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p> |
| `name` | String | ✅ | <p>A friendly name or description of the <a>RegexPatternSet</a>. You can't change <code>Name</code> after you create a 
			<code>RegexPatternSet</code>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `regex_pattern_set` | String | <p>Information about the <a>RegexPatternSet</a> that you specified in the <code>GetRegexPatternSet</code> request, including the identifier of the pattern set and the regular expression patterns you want AWS WAF to search for. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create regex_pattern_set
regex_pattern_set = provider.waf_regional.Regex_pattern_set {
    change_token = "value"  # <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    name = "value"  # <p>A friendly name or description of the <a>RegexPatternSet</a>. You can't change <code>Name</code> after you create a 
			<code>RegexPatternSet</code>.</p>
}

# Access regex_pattern_set outputs
regex_pattern_set_id = regex_pattern_set.id
regex_pattern_set_regex_pattern_set = regex_pattern_set.regex_pattern_set
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
| `web_acl_summary` | String | <p>Information about the web ACL that you specified in the <code>GetWebACLForResource</code> request. If there is no associated resource, a null WebACLSummary is returned.</p> |


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
web_acl_for_resource_web_acl_summary = web_acl_for_resource.web_acl_summary
```

---


### Byte_match_set

ByteMatchSet resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `change_token` | String | ✅ | <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p> |
| `name` | String | ✅ | <p>A friendly name or description of the <a>ByteMatchSet</a>. You can't change <code>Name</code> after you create a 
			<code>ByteMatchSet</code>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `byte_match_set` | String | <p>Information about the <a>ByteMatchSet</a> that you specified in the <code>GetByteMatchSet</code> request. For more information, see the 
			following topics:</p>
		       <ul>
            <li>
               <p>
                  <a>ByteMatchSet</a>: Contains <code>ByteMatchSetId</code>, <code>ByteMatchTuples</code>, and <code>Name</code>
               </p>
            </li>
            <li>
               <p>
                  <code>ByteMatchTuples</code>: Contains an array of <a>ByteMatchTuple</a> objects. Each <code>ByteMatchTuple</code> 
				object contains <a>FieldToMatch</a>, <code>PositionalConstraint</code>, <code>TargetString</code>, 
				and <code>TextTransformation</code>
               </p>
            </li>
            <li>
               <p>
                  <a>FieldToMatch</a>: Contains <code>Data</code> and <code>Type</code>
               </p>
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

# Create byte_match_set
byte_match_set = provider.waf_regional.Byte_match_set {
    change_token = "value"  # <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    name = "value"  # <p>A friendly name or description of the <a>ByteMatchSet</a>. You can't change <code>Name</code> after you create a 
			<code>ByteMatchSet</code>.</p>
}

# Access byte_match_set outputs
byte_match_set_id = byte_match_set.id
byte_match_set_byte_match_set = byte_match_set.byte_match_set
```

---


### Rate_based_rule

RateBasedRule resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `rate_limit` | i64 | ✅ | <p>The maximum number of requests, which have an identical value in the field that is
         specified by <code>RateKey</code>, allowed in a five-minute period. If the number of
         requests exceeds the <code>RateLimit</code> and the other predicates specified in the rule
         are also met, AWS WAF triggers the action that is specified for this rule.</p> |
| `change_token` | String | ✅ | <p>The <code>ChangeToken</code> that you used to submit the
            <code>CreateRateBasedRule</code> request. You can also use this value to query the
         status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p> |
| `rate_key` | String | ✅ | <p>The field that AWS WAF uses to determine if requests are likely arriving from a single
         source and thus subject to rate monitoring. The only valid value for <code>RateKey</code>
         is <code>IP</code>. <code>IP</code> indicates that requests that arrive from the same IP
         address are subject to the <code>RateLimit</code> that is specified in
         the <code>RateBasedRule</code>.</p> |
| `name` | String | ✅ | <p>A friendly name or description of the <a>RateBasedRule</a>. You can't
         change the name of a <code>RateBasedRule</code> after you create it.</p> |
| `metric_name` | String | ✅ | <p>A friendly name or description for the metrics for this <code>RateBasedRule</code>.
          The name can contain only alphanumeric characters (A-Z, a-z, 0-9), with maximum length 128 and minimum length one. It can't contain
          whitespace or metric names reserved for AWS WAF, including "All" and "Default_Action." You can't change the name of the metric after you create the
            <code>RateBasedRule</code>.</p> |
| `tags` | Vec<String> |  | <p></p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `rule` | String | <p>Information about the <a>RateBasedRule</a> that you specified in the
            <code>GetRateBasedRule</code> request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create rate_based_rule
rate_based_rule = provider.waf_regional.Rate_based_rule {
    rate_limit = "value"  # <p>The maximum number of requests, which have an identical value in the field that is
         specified by <code>RateKey</code>, allowed in a five-minute period. If the number of
         requests exceeds the <code>RateLimit</code> and the other predicates specified in the rule
         are also met, AWS WAF triggers the action that is specified for this rule.</p>
    change_token = "value"  # <p>The <code>ChangeToken</code> that you used to submit the
            <code>CreateRateBasedRule</code> request. You can also use this value to query the
         status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    rate_key = "value"  # <p>The field that AWS WAF uses to determine if requests are likely arriving from a single
         source and thus subject to rate monitoring. The only valid value for <code>RateKey</code>
         is <code>IP</code>. <code>IP</code> indicates that requests that arrive from the same IP
         address are subject to the <code>RateLimit</code> that is specified in
         the <code>RateBasedRule</code>.</p>
    name = "value"  # <p>A friendly name or description of the <a>RateBasedRule</a>. You can't
         change the name of a <code>RateBasedRule</code> after you create it.</p>
    metric_name = "value"  # <p>A friendly name or description for the metrics for this <code>RateBasedRule</code>.
          The name can contain only alphanumeric characters (A-Z, a-z, 0-9), with maximum length 128 and minimum length one. It can't contain
          whitespace or metric names reserved for AWS WAF, including "All" and "Default_Action." You can't change the name of the metric after you create the
            <code>RateBasedRule</code>.</p>
}

# Access rate_based_rule outputs
rate_based_rule_id = rate_based_rule.id
rate_based_rule_rule = rate_based_rule.rule
```

---


### Rule

Rule resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `metric_name` | String | ✅ | <p>A friendly name or description for the metrics for this <code>Rule</code>. The name can contain only alphanumeric characters (A-Z, a-z, 0-9), with maximum length 128 and minimum length one. It can't contain
	        whitespace or metric names reserved for AWS WAF, including "All" and "Default_Action." You can't change the name of the metric after you create the
            <code>Rule</code>.</p> |
| `name` | String | ✅ | <p>A friendly name or description of the <a>Rule</a>. You can't change the name of a <code>Rule</code> after you create it.</p> |
| `change_token` | String | ✅ | <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p> |
| `tags` | Vec<String> |  | <p></p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `rule` | String | <p>Information about the <a>Rule</a> that you specified in the <code>GetRule</code> request. 
			For more information, see the following topics:</p>
		       <ul>
            <li>
               <p>
                  <a>Rule</a>: Contains <code>MetricName</code>, <code>Name</code>, an array of <code>Predicate</code> objects, 
				and <code>RuleId</code>
               </p>
            </li>
            <li>
               <p>
                  <a>Predicate</a>: Each <code>Predicate</code> object contains <code>DataId</code>, <code>Negated</code>, and 
				<code>Type</code>
               </p>
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

# Create rule
rule = provider.waf_regional.Rule {
    metric_name = "value"  # <p>A friendly name or description for the metrics for this <code>Rule</code>. The name can contain only alphanumeric characters (A-Z, a-z, 0-9), with maximum length 128 and minimum length one. It can't contain
	        whitespace or metric names reserved for AWS WAF, including "All" and "Default_Action." You can't change the name of the metric after you create the
            <code>Rule</code>.</p>
    name = "value"  # <p>A friendly name or description of the <a>Rule</a>. You can't change the name of a <code>Rule</code> after you create it.</p>
    change_token = "value"  # <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
}

# Access rule outputs
rule_id = rule.id
rule_rule = rule.rule
```

---


### Logging_configuration

LoggingConfiguration resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `logging_configuration` | String | ✅ | <p>The Amazon Kinesis Data Firehose that contains the inspected traffic
         information, the redacted fields details, and the Amazon Resource Name (ARN) of the web ACL
         to monitor.</p>
         <note>
            <p>When specifying <code>Type</code> in <code>RedactedFields</code>, you must use one of
            the following values: <code>URI</code>, <code>QUERY_STRING</code>, <code>HEADER</code>,
            or <code>METHOD</code>.</p>
         </note> |


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
logging_configuration = provider.waf_regional.Logging_configuration {
    logging_configuration = "value"  # <p>The Amazon Kinesis Data Firehose that contains the inspected traffic
         information, the redacted fields details, and the Amazon Resource Name (ARN) of the web ACL
         to monitor.</p>
         <note>
            <p>When specifying <code>Type</code> in <code>RedactedFields</code>, you must use one of
            the following values: <code>URI</code>, <code>QUERY_STRING</code>, <code>HEADER</code>,
            or <code>METHOD</code>.</p>
         </note>
}

# Access logging_configuration outputs
logging_configuration_id = logging_configuration.id
logging_configuration_logging_configuration = logging_configuration.logging_configuration
```

---


### Sql_injection_match_set

SqlInjectionMatchSet resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `change_token` | String | ✅ | <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p> |
| `name` | String | ✅ | <p>A friendly name or description for the <a>SqlInjectionMatchSet</a> that you're creating. You can't change <code>Name</code> 
			after you create the <code>SqlInjectionMatchSet</code>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `sql_injection_match_set` | String | <p>Information about the <a>SqlInjectionMatchSet</a> that you specified in the <code>GetSqlInjectionMatchSet</code> request. 
			For more information, see the following topics:</p>
		       <ul>
            <li>
               <p>
                  <a>SqlInjectionMatchSet</a>: Contains <code>Name</code>, <code>SqlInjectionMatchSetId</code>, and an array of 
				<code>SqlInjectionMatchTuple</code> objects</p>
            </li>
            <li>
               <p>
                  <a>SqlInjectionMatchTuple</a>: Each <code>SqlInjectionMatchTuple</code> object contains <code>FieldToMatch</code> and 
				<code>TextTransformation</code>
               </p>
            </li>
            <li>
               <p>
                  <a>FieldToMatch</a>: Contains <code>Data</code> and <code>Type</code>
               </p>
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

# Create sql_injection_match_set
sql_injection_match_set = provider.waf_regional.Sql_injection_match_set {
    change_token = "value"  # <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    name = "value"  # <p>A friendly name or description for the <a>SqlInjectionMatchSet</a> that you're creating. You can't change <code>Name</code> 
			after you create the <code>SqlInjectionMatchSet</code>.</p>
}

# Access sql_injection_match_set outputs
sql_injection_match_set_id = sql_injection_match_set.id
sql_injection_match_set_sql_injection_match_set = sql_injection_match_set.sql_injection_match_set
```

---


### Change_token

ChangeToken resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `change_token` | String | <p>The <code>ChangeToken</code> that you used in the request. Use this value in a <code>GetChangeTokenStatus</code> request 
			to get the current status of the request. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access change_token outputs
change_token_id = change_token.id
change_token_change_token = change_token.change_token
```

---


### Permission_policy

PermissionPolicy resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `policy` | String | ✅ | <p>The policy to attach to the specified RuleGroup.</p> |
| `resource_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the RuleGroup to which you want to attach the policy.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `policy` | String | <p>The IAM policy attached to the specified RuleGroup.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create permission_policy
permission_policy = provider.waf_regional.Permission_policy {
    policy = "value"  # <p>The policy to attach to the specified RuleGroup.</p>
    resource_arn = "value"  # <p>The Amazon Resource Name (ARN) of the RuleGroup to which you want to attach the policy.</p>
}

# Access permission_policy outputs
permission_policy_id = permission_policy.id
permission_policy_policy = permission_policy.policy
```

---


### Change_token_status

ChangeTokenStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `change_token_status` | String | <p>The status of the change token.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access change_token_status outputs
change_token_status_id = change_token_status.id
change_token_status_change_token_status = change_token_status.change_token_status
```

---


### Rate_based_rule_managed_keys

RateBasedRuleManagedKeys resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_marker` | String | <p>A null value and not currently used.</p> |
| `managed_keys` | Vec<String> | <p>An array of IP addresses that currently are blocked by the specified <a>RateBasedRule</a>. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access rate_based_rule_managed_keys outputs
rate_based_rule_managed_keys_id = rate_based_rule_managed_keys.id
rate_based_rule_managed_keys_next_marker = rate_based_rule_managed_keys.next_marker
rate_based_rule_managed_keys_managed_keys = rate_based_rule_managed_keys.managed_keys
```

---


### Rule_group

RuleGroup resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>A friendly name or description of the <a>RuleGroup</a>. You can't change <code>Name</code> after you create a 
         <code>RuleGroup</code>.</p> |
| `metric_name` | String | ✅ | <p>A friendly name or description for the metrics for this <code>RuleGroup</code>. The name can contain only alphanumeric characters (A-Z, a-z, 0-9), with maximum length 128 and minimum length one. It can't contain
         whitespace or metric names reserved for AWS WAF, including "All" and "Default_Action." You can't change the name of the metric after you create the <code>RuleGroup</code>.</p> |
| `change_token` | String | ✅ | <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p> |
| `tags` | Vec<String> |  | <p></p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `rule_group` | String | <p>Information about the <a>RuleGroup</a> that you specified in the <code>GetRuleGroup</code> request. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create rule_group
rule_group = provider.waf_regional.Rule_group {
    name = "value"  # <p>A friendly name or description of the <a>RuleGroup</a>. You can't change <code>Name</code> after you create a 
         <code>RuleGroup</code>.</p>
    metric_name = "value"  # <p>A friendly name or description for the metrics for this <code>RuleGroup</code>. The name can contain only alphanumeric characters (A-Z, a-z, 0-9), with maximum length 128 and minimum length one. It can't contain
         whitespace or metric names reserved for AWS WAF, including "All" and "Default_Action." You can't change the name of the metric after you create the <code>RuleGroup</code>.</p>
    change_token = "value"  # <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
}

# Access rule_group outputs
rule_group_id = rule_group.id
rule_group_rule_group = rule_group.rule_group
```

---


### Ip_set

IPSet resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>A friendly name or description of the <a>IPSet</a>. You can't change <code>Name</code> after you create the <code>IPSet</code>.</p> |
| `change_token` | String | ✅ | <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `ip_set` | String | <p>Information about the <a>IPSet</a> that you specified in the <code>GetIPSet</code> request. For more information, see the 
			following topics:</p>
		       <ul>
            <li>
               <p>
                  <a>IPSet</a>: Contains <code>IPSetDescriptors</code>, <code>IPSetId</code>, and <code>Name</code>
               </p>
            </li>
            <li>
               <p>
                  <code>IPSetDescriptors</code>: Contains an array of <a>IPSetDescriptor</a> objects. Each <code>IPSetDescriptor</code> 
				object contains <code>Type</code> and <code>Value</code>
               </p>
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

# Create ip_set
ip_set = provider.waf_regional.Ip_set {
    name = "value"  # <p>A friendly name or description of the <a>IPSet</a>. You can't change <code>Name</code> after you create the <code>IPSet</code>.</p>
    change_token = "value"  # <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
}

# Access ip_set outputs
ip_set_id = ip_set.id
ip_set_ip_set = ip_set.ip_set
```

---


### Size_constraint_set

SizeConstraintSet resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>A friendly name or description of the <a>SizeConstraintSet</a>. You can't change <code>Name</code> after you create a 
			<code>SizeConstraintSet</code>.</p> |
| `change_token` | String | ✅ | <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `size_constraint_set` | String | <p>Information about the <a>SizeConstraintSet</a> that you specified in the <code>GetSizeConstraintSet</code> request. For more information, see the 
			following topics:</p>
		       <ul>
            <li>
               <p>
                  <a>SizeConstraintSet</a>: Contains <code>SizeConstraintSetId</code>, <code>SizeConstraints</code>, and <code>Name</code>
               </p>
            </li>
            <li>
               <p>
                  <code>SizeConstraints</code>: Contains an array of <a>SizeConstraint</a> objects. Each <code>SizeConstraint</code> 
				object contains <a>FieldToMatch</a>, <code>TextTransformation</code>, <code>ComparisonOperator</code>, 
				and <code>Size</code>
               </p>
            </li>
            <li>
               <p>
                  <a>FieldToMatch</a>: Contains <code>Data</code> and <code>Type</code>
               </p>
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

# Create size_constraint_set
size_constraint_set = provider.waf_regional.Size_constraint_set {
    name = "value"  # <p>A friendly name or description of the <a>SizeConstraintSet</a>. You can't change <code>Name</code> after you create a 
			<code>SizeConstraintSet</code>.</p>
    change_token = "value"  # <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
}

# Access size_constraint_set outputs
size_constraint_set_id = size_constraint_set.id
size_constraint_set_size_constraint_set = size_constraint_set.size_constraint_set
```

---


### Web_acl

WebACL resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `metric_name` | String | ✅ | <p>A friendly name or description for the metrics for this <code>WebACL</code>.The name can contain only alphanumeric characters (A-Z, a-z, 0-9), with maximum length 128 and minimum length one. It can't contain
	        whitespace or metric names reserved for AWS WAF, including "All" and "Default_Action." You can't change <code>MetricName</code> after you create the
            <code>WebACL</code>.</p> |
| `default_action` | String | ✅ | <p>The action that you want  AWS WAF to take when a request doesn't match the criteria specified in any of the <code>Rule</code> 
			objects that are associated with the <code>WebACL</code>.</p> |
| `name` | String | ✅ | <p>A friendly name or description of the <a>WebACL</a>. You can't change <code>Name</code> after you create the <code>WebACL</code>.</p> |
| `change_token` | String | ✅ | <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p> |
| `tags` | Vec<String> |  | <p></p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `web_acl` | String | <p>Information about the <a>WebACL</a> that you specified in the <code>GetWebACL</code> request. 
			For more information, see the following topics:</p>
		       <ul>
            <li>
               <p>
                  <a>WebACL</a>: Contains <code>DefaultAction</code>, <code>MetricName</code>, <code>Name</code>, an array of 
				<code>Rule</code> objects, and <code>WebACLId</code>
               </p>
            </li>
            <li>
               <p>
                  <code>DefaultAction</code> (Data type is <a>WafAction</a>): Contains <code>Type</code>
               </p>
            </li>
            <li>
               <p>
                  <code>Rules</code>: Contains an array of <code>ActivatedRule</code> objects, which contain <code>Action</code>, 
				<code>Priority</code>, and <code>RuleId</code>
               </p>
            </li>
            <li>
               <p>
                  <code>Action</code>: Contains <code>Type</code>
               </p>
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

# Create web_acl
web_acl = provider.waf_regional.Web_acl {
    metric_name = "value"  # <p>A friendly name or description for the metrics for this <code>WebACL</code>.The name can contain only alphanumeric characters (A-Z, a-z, 0-9), with maximum length 128 and minimum length one. It can't contain
	        whitespace or metric names reserved for AWS WAF, including "All" and "Default_Action." You can't change <code>MetricName</code> after you create the
            <code>WebACL</code>.</p>
    default_action = "value"  # <p>The action that you want  AWS WAF to take when a request doesn't match the criteria specified in any of the <code>Rule</code> 
			objects that are associated with the <code>WebACL</code>.</p>
    name = "value"  # <p>A friendly name or description of the <a>WebACL</a>. You can't change <code>Name</code> after you create the <code>WebACL</code>.</p>
    change_token = "value"  # <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
}

# Access web_acl outputs
web_acl_id = web_acl.id
web_acl_web_acl = web_acl.web_acl
```

---


### Regex_match_set

RegexMatchSet resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>A friendly name or description of the <a>RegexMatchSet</a>. You can't change <code>Name</code> after you create a 
			<code>RegexMatchSet</code>.</p> |
| `change_token` | String | ✅ | <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `regex_match_set` | String | <p>Information about the <a>RegexMatchSet</a> that you specified in the <code>GetRegexMatchSet</code> request. For more information, see <a>RegexMatchTuple</a>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create regex_match_set
regex_match_set = provider.waf_regional.Regex_match_set {
    name = "value"  # <p>A friendly name or description of the <a>RegexMatchSet</a>. You can't change <code>Name</code> after you create a 
			<code>RegexMatchSet</code>.</p>
    change_token = "value"  # <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
}

# Access regex_match_set outputs
regex_match_set_id = regex_match_set.id
regex_match_set_regex_match_set = regex_match_set.regex_match_set
```

---


### Xss_match_set

XssMatchSet resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>A friendly name or description for the <a>XssMatchSet</a> that you're creating. You can't change <code>Name</code> 
			after you create the <code>XssMatchSet</code>.</p> |
| `change_token` | String | ✅ | <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `xss_match_set` | String | <p>Information about the <a>XssMatchSet</a> that you specified in the <code>GetXssMatchSet</code> request. 
			For more information, see the following topics:</p>
		       <ul>
            <li>
               <p>
                  <a>XssMatchSet</a>: Contains <code>Name</code>, <code>XssMatchSetId</code>, and an array of 
				<code>XssMatchTuple</code> objects</p>
            </li>
            <li>
               <p>
                  <a>XssMatchTuple</a>: Each <code>XssMatchTuple</code> object contains <code>FieldToMatch</code> and 
				<code>TextTransformation</code>
               </p>
            </li>
            <li>
               <p>
                  <a>FieldToMatch</a>: Contains <code>Data</code> and <code>Type</code>
               </p>
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

# Create xss_match_set
xss_match_set = provider.waf_regional.Xss_match_set {
    name = "value"  # <p>A friendly name or description for the <a>XssMatchSet</a> that you're creating. You can't change <code>Name</code> 
			after you create the <code>XssMatchSet</code>.</p>
    change_token = "value"  # <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
}

# Access xss_match_set outputs
xss_match_set_id = xss_match_set.id
xss_match_set_xss_match_set = xss_match_set.xss_match_set
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
| `time_window` | String | <p>Usually, <code>TimeWindow</code> is the time range that you specified in the <code>GetSampledRequests</code> request. However, 
			if your AWS resource received more than 5,000 requests during the time range that you specified in the request, 
			<code>GetSampledRequests</code> returns the time range for the first 5,000 requests. Times are in Coordinated Universal Time (UTC) format.</p> |
| `population_size` | i64 | <p>The total number of requests from which <code>GetSampledRequests</code> got a sample of <code>MaxItems</code> requests. 
			If <code>PopulationSize</code> is less than <code>MaxItems</code>, the sample includes every request that your AWS resource 
			received during the specified time range.</p> |
| `sampled_requests` | Vec<String> | <p>A complex type that contains detailed information about each of the requests in the sample.</p> |


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
sampled_requests_time_window = sampled_requests.time_window
sampled_requests_population_size = sampled_requests.population_size
sampled_requests_sampled_requests = sampled_requests.sampled_requests
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple geo_match_set resources
geo_match_set_0 = provider.waf_regional.Geo_match_set {
    name = "value-0"
    change_token = "value-0"
}
geo_match_set_1 = provider.waf_regional.Geo_match_set {
    name = "value-1"
    change_token = "value-1"
}
geo_match_set_2 = provider.waf_regional.Geo_match_set {
    name = "value-2"
    change_token = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    geo_match_set = provider.waf_regional.Geo_match_set {
        name = "production-value"
        change_token = "production-value"
    }
```

---

## Related Documentation

- [AWS Waf_regional Documentation](https://docs.aws.amazon.com/waf_regional/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
