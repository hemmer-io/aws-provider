# Route53resolver Service



**Resources**: 17

---

## Overview

The route53resolver service provides access to 17 resource types:

- [Resolver_query_log_config](#resolver_query_log_config) [CRD]
- [Resolver_rule_association](#resolver_rule_association) [R]
- [Firewall_domain_list](#firewall_domain_list) [CRD]
- [Resolver_rule](#resolver_rule) [CRUD]
- [Resolver_dnssec_config](#resolver_dnssec_config) [RU]
- [Resolver_query_log_config_policy](#resolver_query_log_config_policy) [CR]
- [Firewall_rule_group](#firewall_rule_group) [CRD]
- [Firewall_rule_group_association](#firewall_rule_group_association) [RU]
- [Firewall_config](#firewall_config) [RU]
- [Firewall_domains](#firewall_domains) [U]
- [Resolver_query_log_config_association](#resolver_query_log_config_association) [R]
- [Firewall_rule](#firewall_rule) [CUD]
- [Outpost_resolver](#outpost_resolver) [CRUD]
- [Resolver_config](#resolver_config) [RU]
- [Resolver_endpoint](#resolver_endpoint) [CRUD]
- [Firewall_rule_group_policy](#firewall_rule_group_policy) [CR]
- [Resolver_rule_policy](#resolver_rule_policy) [CR]

---

## Resources


### Resolver_query_log_config

ResolverQueryLogConfig resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>A list of the tag keys and values that you want to associate with the query logging configuration.</p> |
| `name` | String | ✅ | <p>The name that you want to give the query logging configuration.</p> |
| `destination_arn` | String | ✅ | <p>The ARN of the resource that you want Resolver to send query logs. You can send query logs to an S3 bucket, a CloudWatch Logs log group, 
			or a Kinesis Data Firehose delivery stream. Examples of valid values include the following:</p>
         <ul>
            <li>
               <p>
                  <b>S3 bucket</b>: </p>
               <p>
                  <code>arn:aws:s3:::amzn-s3-demo-bucket</code>
               </p>
               <p>You can optionally append a file prefix to the end of the ARN.</p>
               <p>
                  <code>arn:aws:s3:::amzn-s3-demo-bucket/development/</code>
               </p>
            </li>
            <li>
               <p>
                  <b>CloudWatch Logs log group</b>: </p>
               <p>
                  <code>arn:aws:logs:us-west-1:123456789012:log-group:/mystack-testgroup-12ABC1AB12A1:*</code>
               </p>
            </li>
            <li>
               <p>
                  <b>Kinesis Data Firehose delivery stream</b>:</p>
               <p>
                  <code>arn:aws:kinesis:us-east-2:0123456789:stream/my_stream_name</code>
               </p>
            </li>
         </ul> |
| `creator_request_id` | String | ✅ | <p>A unique string that identifies the request and that allows failed requests to be retried
			without the risk of running the operation twice. <code>CreatorRequestId</code> can be
			any unique string, for example, a date/time stamp. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `resolver_query_log_config` | String | <p>Information about the Resolver query logging configuration that you specified in a <code>GetQueryLogConfig</code> request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create resolver_query_log_config
resolver_query_log_config = provider.route53resolver.Resolver_query_log_config {
    name = "value"  # <p>The name that you want to give the query logging configuration.</p>
    destination_arn = "value"  # <p>The ARN of the resource that you want Resolver to send query logs. You can send query logs to an S3 bucket, a CloudWatch Logs log group, 
			or a Kinesis Data Firehose delivery stream. Examples of valid values include the following:</p>
         <ul>
            <li>
               <p>
                  <b>S3 bucket</b>: </p>
               <p>
                  <code>arn:aws:s3:::amzn-s3-demo-bucket</code>
               </p>
               <p>You can optionally append a file prefix to the end of the ARN.</p>
               <p>
                  <code>arn:aws:s3:::amzn-s3-demo-bucket/development/</code>
               </p>
            </li>
            <li>
               <p>
                  <b>CloudWatch Logs log group</b>: </p>
               <p>
                  <code>arn:aws:logs:us-west-1:123456789012:log-group:/mystack-testgroup-12ABC1AB12A1:*</code>
               </p>
            </li>
            <li>
               <p>
                  <b>Kinesis Data Firehose delivery stream</b>:</p>
               <p>
                  <code>arn:aws:kinesis:us-east-2:0123456789:stream/my_stream_name</code>
               </p>
            </li>
         </ul>
    creator_request_id = "value"  # <p>A unique string that identifies the request and that allows failed requests to be retried
			without the risk of running the operation twice. <code>CreatorRequestId</code> can be
			any unique string, for example, a date/time stamp. </p>
}

# Access resolver_query_log_config outputs
resolver_query_log_config_id = resolver_query_log_config.id
resolver_query_log_config_resolver_query_log_config = resolver_query_log_config.resolver_query_log_config
```

---


### Resolver_rule_association

ResolverRuleAssociation resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `resolver_rule_association` | String | <p>Information about the Resolver rule association that you specified in a <code>GetResolverRuleAssociation</code> request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access resolver_rule_association outputs
resolver_rule_association_id = resolver_rule_association.id
resolver_rule_association_resolver_rule_association = resolver_rule_association.resolver_rule_association
```

---


### Firewall_domain_list

FirewallDomainList resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>A name that lets you identify the domain list to manage and use it.</p> |
| `tags` | Vec<String> |  | <p>A list of the tag keys and values that you want to associate with the domain list. </p> |
| `creator_request_id` | String | ✅ | <p>A unique string that identifies the request and that allows you to retry failed requests
			without the risk of running the operation twice. <code>CreatorRequestId</code> can be
			any unique string, for example, a date/time stamp. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `firewall_domain_list` | String | <p>The domain list that you requested.  </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create firewall_domain_list
firewall_domain_list = provider.route53resolver.Firewall_domain_list {
    name = "value"  # <p>A name that lets you identify the domain list to manage and use it.</p>
    creator_request_id = "value"  # <p>A unique string that identifies the request and that allows you to retry failed requests
			without the risk of running the operation twice. <code>CreatorRequestId</code> can be
			any unique string, for example, a date/time stamp. </p>
}

# Access firewall_domain_list outputs
firewall_domain_list_id = firewall_domain_list.id
firewall_domain_list_firewall_domain_list = firewall_domain_list.firewall_domain_list
```

---


### Resolver_rule

ResolverRule resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `rule_type` | String | ✅ | <p>When you want to forward DNS queries for specified domain name to resolvers on your network, specify <code>FORWARD</code> or <code>DELEGATE</code>.</p>
         <p>When you have a forwarding rule to forward DNS queries for a domain to your network and you want Resolver to process queries for 
			a subdomain of that domain, specify <code>SYSTEM</code>.</p>
         <p>For example, to forward DNS queries for example.com to resolvers on your network, you create a rule and specify <code>FORWARD</code> 
			for <code>RuleType</code>. To then have Resolver process queries for apex.example.com, you create a rule and specify 
			<code>SYSTEM</code> for <code>RuleType</code>.</p>
         <p>Currently, only Resolver can create rules that have a value of <code>RECURSIVE</code> for <code>RuleType</code>.</p> |
| `name` | String |  | <p>A friendly name that lets you easily find a rule in the Resolver dashboard in the Route 53 console.</p> |
| `target_ips` | Vec<String> |  | <p>The IPs that you want Resolver to forward DNS queries to. You can specify either Ipv4 or Ipv6 addresses but not both in the same rule. Separate IP addresses with a space.</p>
         <p>
            <code>TargetIps</code> is available only when the value of <code>Rule type</code> is <code>FORWARD</code>.</p> |
| `resolver_endpoint_id` | String |  | <p>The ID of the outbound Resolver endpoint that you want to use to route DNS queries to the IP addresses that you specify 
			in <code>TargetIps</code>.</p> |
| `delegation_record` | String |  | <p>
			DNS queries with the delegation records that match this domain name are forwarded to the resolvers on your 
			network.
		</p> |
| `domain_name` | String |  | <p>DNS queries for this domain name are forwarded to the IP addresses that you specify in <code>TargetIps</code>. If a query matches 
			multiple Resolver rules (example.com and www.example.com), outbound DNS queries are routed using the Resolver rule that contains 
			the most specific domain name (www.example.com).</p> |
| `creator_request_id` | String | ✅ | <p>A unique string that identifies the request and that allows failed requests to be retried
			without the risk of running the operation twice. <code>CreatorRequestId</code> can be
			any unique string, for example, a date/time stamp. </p> |
| `tags` | Vec<String> |  | <p>A list of the tag keys and values that you want to associate with the endpoint.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `resolver_rule` | String | <p>Information about the Resolver rule that you specified in a <code>GetResolverRule</code> request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create resolver_rule
resolver_rule = provider.route53resolver.Resolver_rule {
    rule_type = "value"  # <p>When you want to forward DNS queries for specified domain name to resolvers on your network, specify <code>FORWARD</code> or <code>DELEGATE</code>.</p>
         <p>When you have a forwarding rule to forward DNS queries for a domain to your network and you want Resolver to process queries for 
			a subdomain of that domain, specify <code>SYSTEM</code>.</p>
         <p>For example, to forward DNS queries for example.com to resolvers on your network, you create a rule and specify <code>FORWARD</code> 
			for <code>RuleType</code>. To then have Resolver process queries for apex.example.com, you create a rule and specify 
			<code>SYSTEM</code> for <code>RuleType</code>.</p>
         <p>Currently, only Resolver can create rules that have a value of <code>RECURSIVE</code> for <code>RuleType</code>.</p>
    creator_request_id = "value"  # <p>A unique string that identifies the request and that allows failed requests to be retried
			without the risk of running the operation twice. <code>CreatorRequestId</code> can be
			any unique string, for example, a date/time stamp. </p>
}

# Access resolver_rule outputs
resolver_rule_id = resolver_rule.id
resolver_rule_resolver_rule = resolver_rule.resolver_rule
```

---


### Resolver_dnssec_config

ResolverDnssecConfig resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resource_id` | String | ✅ | <p>The ID of the virtual private cloud (VPC) that you're updating the DNSSEC validation status for.</p> |
| `validation` | String | ✅ | <p>The new value that you are specifying for DNSSEC validation for the VPC. The value can be <code>ENABLE</code>
			or <code>DISABLE</code>. Be aware that it can take time for a validation status change to be completed.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `resolver_dnssec_config` | String | <p>The information about a configuration for DNSSEC validation.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access resolver_dnssec_config outputs
resolver_dnssec_config_id = resolver_dnssec_config.id
resolver_dnssec_config_resolver_dnssec_config = resolver_dnssec_config.resolver_dnssec_config
```

---


### Resolver_query_log_config_policy

ResolverQueryLogConfigPolicy resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the account that you want to share rules with.</p> |
| `resolver_query_log_config_policy` | String | ✅ | <p>An Identity and Access Management policy statement that lists the query logging configurations that you want to share with another Amazon Web Services account 
			and the operations that you want the account to be able to perform. You can specify the following operations in the <code>Actions</code> section 
			of the statement:</p>
         <ul>
            <li>
               <p>
                  <code>route53resolver:AssociateResolverQueryLogConfig</code>
               </p>
            </li>
            <li>
               <p>
                  <code>route53resolver:DisassociateResolverQueryLogConfig</code>
               </p>
            </li>
            <li>
               <p>
                  <code>route53resolver:ListResolverQueryLogConfigs</code>
               </p>
            </li>
         </ul>
         <p>In the <code>Resource</code> section of the statement, you specify the ARNs for the query logging configurations that you want to share 
			with the account that you specified in <code>Arn</code>. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `resolver_query_log_config_policy` | String | <p>Information about the query logging policy for the query logging configuration that you specified in a 
			<code>GetResolverQueryLogConfigPolicy</code> request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create resolver_query_log_config_policy
resolver_query_log_config_policy = provider.route53resolver.Resolver_query_log_config_policy {
    arn = "value"  # <p>The Amazon Resource Name (ARN) of the account that you want to share rules with.</p>
    resolver_query_log_config_policy = "value"  # <p>An Identity and Access Management policy statement that lists the query logging configurations that you want to share with another Amazon Web Services account 
			and the operations that you want the account to be able to perform. You can specify the following operations in the <code>Actions</code> section 
			of the statement:</p>
         <ul>
            <li>
               <p>
                  <code>route53resolver:AssociateResolverQueryLogConfig</code>
               </p>
            </li>
            <li>
               <p>
                  <code>route53resolver:DisassociateResolverQueryLogConfig</code>
               </p>
            </li>
            <li>
               <p>
                  <code>route53resolver:ListResolverQueryLogConfigs</code>
               </p>
            </li>
         </ul>
         <p>In the <code>Resource</code> section of the statement, you specify the ARNs for the query logging configurations that you want to share 
			with the account that you specified in <code>Arn</code>. </p>
}

# Access resolver_query_log_config_policy outputs
resolver_query_log_config_policy_id = resolver_query_log_config_policy.id
resolver_query_log_config_policy_resolver_query_log_config_policy = resolver_query_log_config_policy.resolver_query_log_config_policy
```

---


### Firewall_rule_group

FirewallRuleGroup resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `creator_request_id` | String | ✅ | <p>A unique string defined by you to identify the request. This allows you to retry failed
			requests without the risk of running the operation twice. This can be any unique string,
			for example, a timestamp. </p> |
| `tags` | Vec<String> |  | <p>A list of the tag keys and values that you want to associate with the rule group. </p> |
| `name` | String | ✅ | <p>A name that lets you identify the rule group, to manage and use it.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `firewall_rule_group` | String | <p>A collection of rules used to filter DNS network traffic. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create firewall_rule_group
firewall_rule_group = provider.route53resolver.Firewall_rule_group {
    creator_request_id = "value"  # <p>A unique string defined by you to identify the request. This allows you to retry failed
			requests without the risk of running the operation twice. This can be any unique string,
			for example, a timestamp. </p>
    name = "value"  # <p>A name that lets you identify the rule group, to manage and use it.</p>
}

# Access firewall_rule_group outputs
firewall_rule_group_id = firewall_rule_group.id
firewall_rule_group_firewall_rule_group = firewall_rule_group.firewall_rule_group
```

---


### Firewall_rule_group_association

FirewallRuleGroupAssociation resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `mutation_protection` | String |  | <p>If enabled, this setting disallows modification or removal of the association, to help prevent against accidentally altering DNS firewall protections. </p> |
| `name` | String |  | <p>The name of the rule group association.</p> |
| `priority` | i64 |  | <p>The setting that determines the processing order of the rule group among the rule
			groups that you associate with the specified VPC. DNS Firewall filters VPC traffic
			starting from the rule group with the lowest numeric priority setting. </p>
         <p>You must specify a unique priority for each rule group that you associate with a single VPC. 
           To make it easier to insert rule groups later, leave space between the numbers, for example, use 100, 200, and so on. You 
   can change the priority setting for a rule group association after you create it.</p> |
| `firewall_rule_group_association_id` | String | ✅ | <p>The identifier of the <a>FirewallRuleGroupAssociation</a>. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `firewall_rule_group_association` | String | <p>The association that you requested. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access firewall_rule_group_association outputs
firewall_rule_group_association_id = firewall_rule_group_association.id
firewall_rule_group_association_firewall_rule_group_association = firewall_rule_group_association.firewall_rule_group_association
```

---


### Firewall_config

FirewallConfig resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `firewall_fail_open` | String | ✅ | <p>Determines how Route 53 Resolver handles queries during failures, for example when all traffic that is sent to DNS Firewall fails to receive a reply. </p>
         <ul>
            <li>
               <p>By default, fail open is disabled, which means the failure mode is closed. This approach favors security over availability. 
       DNS Firewall blocks queries that it is unable to evaluate properly. </p>
            </li>
            <li>
               <p>If you enable this option, the failure mode is open. This approach favors availability over security. DNS Firewall allows queries to proceed if it 
       is unable to properly evaluate them. </p>
            </li>
         </ul>
         <p>This behavior is only enforced for VPCs that have at least one DNS Firewall rule group association. </p> |
| `resource_id` | String | ✅ | <p>The ID of the VPC that the configuration is for.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `firewall_config` | String | <p>Configuration of the firewall behavior provided by DNS Firewall for a single VPC from
			AmazonVPC. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access firewall_config outputs
firewall_config_id = firewall_config.id
firewall_config_firewall_config = firewall_config.firewall_config
```

---


### Firewall_domains

FirewallDomains resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `firewall_domain_list_id` | String | ✅ | <p>The ID of the domain list whose domains you want to update. </p> |
| `domains` | Vec<String> | ✅ | <p>A list of domains to use in the update operation.</p>
         <important>
            <p>There is a limit of 1000 domains per request.</p>
         </important>
         <p>Each domain specification in your domain list must satisfy the following
	requirements: </p>
         <ul>
            <li>
               <p>It can optionally start with <code>*</code> (asterisk).</p>
            </li>
            <li>
               <p>With the exception of the optional starting asterisk, it must only contain
      	   the following characters: <code>A-Z</code>, <code>a-z</code>,
      	   <code>0-9</code>, <code>-</code> (hyphen).</p>
            </li>
            <li>
               <p>It must be from 1-255 characters in length. </p>
            </li>
         </ul> |
| `operation` | String | ✅ | <p>What you want DNS Firewall to do with the domains that you are providing: </p>
         <ul>
            <li>
               <p>
                  <code>ADD</code> - Add the domains to the ones that are already in the domain list. </p>
            </li>
            <li>
               <p>
                  <code>REMOVE</code> - Search the domain list for the domains and remove them from the list.</p>
            </li>
            <li>
               <p>
                  <code>REPLACE</code> - Update the domain list to exactly match the list that you are providing. </p>
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

```

---


### Resolver_query_log_config_association

ResolverQueryLogConfigAssociation resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `resolver_query_log_config_association` | String | <p>Information about the Resolver query logging configuration association that you specified in a <code>GetQueryLogConfigAssociation</code> request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access resolver_query_log_config_association outputs
resolver_query_log_config_association_id = resolver_query_log_config_association.id
resolver_query_log_config_association_resolver_query_log_config_association = resolver_query_log_config_association.resolver_query_log_config_association
```

---


### Firewall_rule

FirewallRule resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `block_override_domain` | String |  | <p>The custom DNS record to send back in response to the query. Used for the rule action <code>BLOCK</code> with a <code>BlockResponse</code> setting of <code>OVERRIDE</code>.</p>
         <p>This setting is required if the <code>BlockResponse</code> setting is <code>OVERRIDE</code>.</p> |
| `dns_threat_protection` | String |  | <p>
			Use to create a DNS Firewall Advanced rule.
		</p> |
| `block_response` | String |  | <p>The way that you want DNS Firewall to block the request, used with the rule action
			setting <code>BLOCK</code>. </p>
         <ul>
            <li>
               <p>
                  <code>NODATA</code> - Respond indicating that the query was successful, but no response is available for it.</p>
            </li>
            <li>
               <p>
                  <code>NXDOMAIN</code> - Respond indicating that the domain name that's in the query doesn't exist.</p>
            </li>
            <li>
               <p>
                  <code>OVERRIDE</code> - Provide a custom override in the response. This option requires custom handling details in the rule's <code>BlockOverride*</code> settings. </p>
            </li>
         </ul>
         <p>This setting is required if the rule action setting is <code>BLOCK</code>.</p> |
| `block_override_dns_type` | String |  | <p>The DNS record's type. This determines the format of the record value that you provided in <code>BlockOverrideDomain</code>. Used for the rule action <code>BLOCK</code> with a <code>BlockResponse</code> setting of <code>OVERRIDE</code>.</p>
         <p>This setting is required if the <code>BlockResponse</code> setting is <code>OVERRIDE</code>.</p> |
| `firewall_domain_list_id` | String |  | <p>The ID of the domain list that you want to use in the rule. Can't be used together with <code>DnsThreatProtecton</code>.</p> |
| `action` | String | ✅ | <p>The action that DNS Firewall should take on a DNS query when it matches one of the domains in the rule's domain list, or a threat in a DNS Firewall Advanced rule:</p>
         <ul>
            <li>
               <p>
                  <code>ALLOW</code> - Permit the request to go through. Not available for DNS Firewall Advanced rules.</p>
            </li>
            <li>
               <p>
                  <code>ALERT</code> - Permit the request and send metrics and logs to Cloud Watch.</p>
            </li>
            <li>
               <p>
                  <code>BLOCK</code> - Disallow the request. This option requires additional details in the rule's <code>BlockResponse</code>. </p>
            </li>
         </ul> |
| `firewall_rule_group_id` | String | ✅ | <p>The unique identifier of the firewall rule group where you want to create the rule. </p> |
| `creator_request_id` | String | ✅ | <p>A unique string that identifies the request and that allows you to retry failed requests
			without the risk of running the operation twice. <code>CreatorRequestId</code> can be
			any unique string, for example, a date/time stamp. </p> |
| `firewall_domain_redirection_action` | String |  | <p>
			How you want the the rule to evaluate DNS redirection in the DNS redirection chain, such as CNAME or DNAME. 
		</p>
         <p>
            <code>INSPECT_REDIRECTION_DOMAIN</code>: (Default) inspects all domains in the redirection chain. The individual domains in the redirection chain must be 
			added to the domain list.</p>
         <p>
            <code>TRUST_REDIRECTION_DOMAIN</code>: Inspects only the first domain in the redirection chain. You don't need to add the subsequent domains in the domain in the redirection list to 
			the domain list.</p> |
| `name` | String | ✅ | <p>A name that lets you identify the rule in the rule group.</p> |
| `block_override_ttl` | i64 |  | <p>The recommended amount of time, in seconds, for the DNS resolver or web browser to cache the provided override record. Used for the rule action <code>BLOCK</code> with a <code>BlockResponse</code> setting of <code>OVERRIDE</code>.</p>
         <p>This setting is required if the <code>BlockResponse</code> setting is <code>OVERRIDE</code>.</p> |
| `qtype` | String |  | <p>
			The DNS query type you want the rule to evaluate. Allowed values are;
		</p>
         <ul>
            <li>
               <p>
				A: Returns an IPv4 address.</p>
            </li>
            <li>
               <p>AAAA: Returns an Ipv6 address.</p>
            </li>
            <li>
               <p>CAA: Restricts CAs that can create SSL/TLS certifications for the domain.</p>
            </li>
            <li>
               <p>CNAME: Returns another domain name.</p>
            </li>
            <li>
               <p>DS: Record that identifies the DNSSEC signing key of a delegated zone.</p>
            </li>
            <li>
               <p>MX: Specifies mail servers.</p>
            </li>
            <li>
               <p>NAPTR: Regular-expression-based rewriting of domain names.</p>
            </li>
            <li>
               <p>NS: Authoritative name servers.</p>
            </li>
            <li>
               <p>PTR: Maps an IP address to a domain name.</p>
            </li>
            <li>
               <p>SOA: Start of authority record for the zone.</p>
            </li>
            <li>
               <p>SPF: Lists the servers authorized to send emails from a domain.</p>
            </li>
            <li>
               <p>SRV: Application specific values that identify servers.</p>
            </li>
            <li>
               <p>TXT: Verifies email senders and application-specific values.</p>
            </li>
            <li>
               <p>A query type you define by using the DNS type ID, for example 28 for AAAA. The values must be
				defined as TYPENUMBER, where the
				NUMBER can be 1-65334, for
				example, TYPE28. For more information, see 
				<a href="https://en.wikipedia.org/wiki/List_of_DNS_record_types">List of DNS record types</a>.</p>
            </li>
         </ul> |
| `confidence_threshold` | String |  | <p>
			The confidence threshold for DNS Firewall Advanced. You must provide this value when you create a DNS Firewall Advanced rule. The confidence
			level values mean:
		</p>
         <ul>
            <li>
               <p>
                  <code>LOW</code>: Provides the highest detection rate for threats, but also increases false positives.</p>
            </li>
            <li>
               <p>
                  <code>MEDIUM</code>: Provides a balance between detecting threats and false positives.</p>
            </li>
            <li>
               <p>
                  <code>HIGH</code>: Detects only the most well corroborated threats with a low rate of false positives. </p>
            </li>
         </ul> |
| `priority` | i64 | ✅ | <p>The setting that determines the processing order of the rule in the rule group. DNS Firewall 
           processes the rules in a rule group by order of priority, starting from the lowest setting.</p>
         <p>You must specify a unique priority for each rule in a rule group. 
           To make it easier to insert rules later, leave space between the numbers, for example, use 100, 200, and so on. You 
   can change the priority setting for the rules in a rule group at any time.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create firewall_rule
firewall_rule = provider.route53resolver.Firewall_rule {
    action = "value"  # <p>The action that DNS Firewall should take on a DNS query when it matches one of the domains in the rule's domain list, or a threat in a DNS Firewall Advanced rule:</p>
         <ul>
            <li>
               <p>
                  <code>ALLOW</code> - Permit the request to go through. Not available for DNS Firewall Advanced rules.</p>
            </li>
            <li>
               <p>
                  <code>ALERT</code> - Permit the request and send metrics and logs to Cloud Watch.</p>
            </li>
            <li>
               <p>
                  <code>BLOCK</code> - Disallow the request. This option requires additional details in the rule's <code>BlockResponse</code>. </p>
            </li>
         </ul>
    firewall_rule_group_id = "value"  # <p>The unique identifier of the firewall rule group where you want to create the rule. </p>
    creator_request_id = "value"  # <p>A unique string that identifies the request and that allows you to retry failed requests
			without the risk of running the operation twice. <code>CreatorRequestId</code> can be
			any unique string, for example, a date/time stamp. </p>
    name = "value"  # <p>A name that lets you identify the rule in the rule group.</p>
    priority = "value"  # <p>The setting that determines the processing order of the rule in the rule group. DNS Firewall 
           processes the rules in a rule group by order of priority, starting from the lowest setting.</p>
         <p>You must specify a unique priority for each rule in a rule group. 
           To make it easier to insert rules later, leave space between the numbers, for example, use 100, 200, and so on. You 
   can change the priority setting for the rules in a rule group at any time.</p>
}

```

---


### Outpost_resolver

OutpostResolver resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>A friendly name that lets you easily find a configuration in the 
		Resolver dashboard in the Route 53 console.</p> |
| `instance_count` | i64 |  | <p>Number of Amazon EC2 instances for the
		Resolver on Outpost. 
		The default and minimal value is 4.</p> |
| `preferred_instance_type` | String | ✅ | <p>
		The Amazon EC2 instance type. If you specify this, you must also specify a value for the <code>OutpostArn</code>.
	</p> |
| `outpost_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the Outpost. If you specify this, you must also specify a value for the <code>PreferredInstanceType</code>.</p> |
| `creator_request_id` | String | ✅ | <p>A unique string that identifies the request 
		and that allows failed requests to be retried without the risk of running the operation twice. </p>
         <p>
            <code>CreatorRequestId</code> can be any unique string, for example, a date/time stamp.</p> |
| `tags` | Vec<String> |  | <p>
			A string that helps identify the Route 53 Resolvers on Outpost.
		</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `outpost_resolver` | String | <p>Information about the <code>GetOutpostResolver</code>
		request, including the status of the request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create outpost_resolver
outpost_resolver = provider.route53resolver.Outpost_resolver {
    name = "value"  # <p>A friendly name that lets you easily find a configuration in the 
		Resolver dashboard in the Route 53 console.</p>
    preferred_instance_type = "value"  # <p>
		The Amazon EC2 instance type. If you specify this, you must also specify a value for the <code>OutpostArn</code>.
	</p>
    outpost_arn = "value"  # <p>The Amazon Resource Name (ARN) of the Outpost. If you specify this, you must also specify a value for the <code>PreferredInstanceType</code>.</p>
    creator_request_id = "value"  # <p>A unique string that identifies the request 
		and that allows failed requests to be retried without the risk of running the operation twice. </p>
         <p>
            <code>CreatorRequestId</code> can be any unique string, for example, a date/time stamp.</p>
}

# Access outpost_resolver outputs
outpost_resolver_id = outpost_resolver.id
outpost_resolver_outpost_resolver = outpost_resolver.outpost_resolver
```

---


### Resolver_config

ResolverConfig resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `autodefined_reverse_flag` | String | ✅ | <p>Indicates whether or not the Resolver will create autodefined rules for reverse DNS
			lookups. This is enabled by default. Disabling this option will also affect EC2-Classic
			instances using ClassicLink. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/vpc-classiclink.html">ClassicLink</a> in the
					<i>Amazon EC2 guide</i>.</p>
         <important>
            <p>We are retiring EC2-Classic on August 15, 2022. We recommend that you migrate from EC2-Classic to a VPC. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/vpc-migrate.html">Migrate from EC2-Classic to a VPC</a> in the
			<i>Amazon EC2 guide</i> and the blog <a href="http://aws.amazon.com/blogs/aws/ec2-classic-is-retiring-heres-how-to-prepare/">EC2-Classic Networking is Retiring – Here’s How to Prepare</a>.</p>
         </important>
         <note>
            <p>It can take some time for the status change to be completed.</p>
         </note>
         <p></p> |
| `resource_id` | String | ✅ | <p>The ID of the Amazon Virtual Private Cloud VPC or a Route 53 Profile that you're configuring Resolver for.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `resolver_config` | String | <p>Information about the behavior configuration of Route 53 Resolver behavior for the VPC you
			specified in the <code>GetResolverConfig</code> request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access resolver_config outputs
resolver_config_id = resolver_config.id
resolver_config_resolver_config = resolver_config.resolver_config
```

---


### Resolver_endpoint

ResolverEndpoint resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `ip_addresses` | Vec<String> | ✅ | <p>The subnets and IP addresses in your VPC that DNS queries originate from (for outbound endpoints) or that you forward 
			DNS queries to (for inbound endpoints). The subnet ID uniquely identifies a VPC. </p>
         <note>
            <p>Even though the minimum is 1, Route 53 requires that you create at least two.</p>
         </note> |
| `tags` | Vec<String> |  | <p>A list of the tag keys and values that you want to associate with the endpoint.</p> |
| `preferred_instance_type` | String |  | <p>The  instance type. If you specify this, you must also specify a value for the <code>OutpostArn</code>.</p> |
| `protocols` | Vec<String> |  | <p>
			The protocols you want to use for the endpoint. DoH-FIPS is applicable for default inbound endpoints only.
		</p>
         <p>For a default inbound endpoint you can apply the protocols as follows:</p>
         <ul>
            <li>
               <p> Do53  and DoH in combination.</p>
            </li>
            <li>
               <p>Do53  and DoH-FIPS in combination.</p>
            </li>
            <li>
               <p>Do53 alone.</p>
            </li>
            <li>
               <p>DoH alone.</p>
            </li>
            <li>
               <p>DoH-FIPS alone.</p>
            </li>
            <li>
               <p>None, which is treated as Do53.</p>
            </li>
         </ul>
         <p>For a delegation inbound endpoint you can use Do53 only.</p>
         <p>For an outbound endpoint you can apply the protocols as follows:</p>
         <ul>
            <li>
               <p> Do53  and DoH in combination.</p>
            </li>
            <li>
               <p>Do53 alone.</p>
            </li>
            <li>
               <p>DoH alone.</p>
            </li>
            <li>
               <p>None, which is treated as Do53.</p>
            </li>
         </ul> |
| `name` | String |  | <p>A friendly name that lets you easily find a configuration in the Resolver dashboard in the Route 53 console.</p> |
| `outpost_arn` | String |  | <p>The Amazon Resource Name (ARN) of the Outpost. If you specify this, you must also specify a
			value for the <code>PreferredInstanceType</code>. </p> |
| `resolver_endpoint_type` | String |  | <p>
			For the endpoint type you can choose either IPv4, IPv6, or dual-stack.
			A dual-stack endpoint means that it will resolve via both IPv4 and IPv6. This
			endpoint type is applied to all IP addresses.
		</p> |
| `creator_request_id` | String | ✅ | <p>A unique string that identifies the request and that allows failed requests to be retried
			without the risk of running the operation twice. <code>CreatorRequestId</code> can be
			any unique string, for example, a date/time stamp. </p> |
| `security_group_ids` | Vec<String> | ✅ | <p>The ID of one or more security groups that you want to use to control access to this VPC. The security group that you specify 
			must include one or more inbound rules (for inbound Resolver endpoints) or outbound rules (for outbound Resolver endpoints). 
			Inbound and outbound rules must allow TCP and UDP access. For inbound access, open port 53. For outbound access, open the port 
			that you're using for DNS queries on your network.</p>
         <p>Some security group rules will cause your connection to be tracked. For outbound resolver endpoint, it can potentially impact the 
			maximum queries per second from outbound endpoint to your target name server. For inbound resolver endpoint, it can bring down the overall maximum queries per second per IP address to as low as 1500. 
			To avoid connection tracking caused by security group, see  
			<a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/security-group-connection-tracking.html#untracked-connectionsl">Untracked connections</a>.</p> |
| `direction` | String | ✅ | <p>Specify the applicable value:</p>
         <ul>
            <li>
               <p>
                  <code>INBOUND</code>: Resolver forwards DNS queries to the DNS service for a VPC from your network.</p>
            </li>
            <li>
               <p>
                  <code>OUTBOUND</code>: Resolver forwards DNS queries from the DNS service for a VPC to your network.</p>
            </li>
            <li>
               <p>
                  <code>INBOUND_DELEGATION</code>: Resolver delegates queries to Route 53 private hosted zones from your network.</p>
            </li>
         </ul> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `resolver_endpoint` | String | <p>Information about the Resolver endpoint that you specified in a <code>GetResolverEndpoint</code> request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create resolver_endpoint
resolver_endpoint = provider.route53resolver.Resolver_endpoint {
    ip_addresses = "value"  # <p>The subnets and IP addresses in your VPC that DNS queries originate from (for outbound endpoints) or that you forward 
			DNS queries to (for inbound endpoints). The subnet ID uniquely identifies a VPC. </p>
         <note>
            <p>Even though the minimum is 1, Route 53 requires that you create at least two.</p>
         </note>
    creator_request_id = "value"  # <p>A unique string that identifies the request and that allows failed requests to be retried
			without the risk of running the operation twice. <code>CreatorRequestId</code> can be
			any unique string, for example, a date/time stamp. </p>
    security_group_ids = "value"  # <p>The ID of one or more security groups that you want to use to control access to this VPC. The security group that you specify 
			must include one or more inbound rules (for inbound Resolver endpoints) or outbound rules (for outbound Resolver endpoints). 
			Inbound and outbound rules must allow TCP and UDP access. For inbound access, open port 53. For outbound access, open the port 
			that you're using for DNS queries on your network.</p>
         <p>Some security group rules will cause your connection to be tracked. For outbound resolver endpoint, it can potentially impact the 
			maximum queries per second from outbound endpoint to your target name server. For inbound resolver endpoint, it can bring down the overall maximum queries per second per IP address to as low as 1500. 
			To avoid connection tracking caused by security group, see  
			<a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/security-group-connection-tracking.html#untracked-connectionsl">Untracked connections</a>.</p>
    direction = "value"  # <p>Specify the applicable value:</p>
         <ul>
            <li>
               <p>
                  <code>INBOUND</code>: Resolver forwards DNS queries to the DNS service for a VPC from your network.</p>
            </li>
            <li>
               <p>
                  <code>OUTBOUND</code>: Resolver forwards DNS queries from the DNS service for a VPC to your network.</p>
            </li>
            <li>
               <p>
                  <code>INBOUND_DELEGATION</code>: Resolver delegates queries to Route 53 private hosted zones from your network.</p>
            </li>
         </ul>
}

# Access resolver_endpoint outputs
resolver_endpoint_id = resolver_endpoint.id
resolver_endpoint_resolver_endpoint = resolver_endpoint.resolver_endpoint
```

---


### Firewall_rule_group_policy

FirewallRuleGroupPolicy resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `firewall_rule_group_policy` | String | ✅ | <p>The Identity and Access Management (Amazon Web Services IAM) policy to attach to the rule group.</p> |
| `arn` | String | ✅ | <p>The ARN (Amazon Resource Name) for the rule group that you want to share.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `firewall_rule_group_policy` | String | <p>The Identity and Access Management (Amazon Web Services IAM) policy for sharing the specified rule
			group. You can use the policy to share the rule group using Resource Access Manager
			(RAM). </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create firewall_rule_group_policy
firewall_rule_group_policy = provider.route53resolver.Firewall_rule_group_policy {
    firewall_rule_group_policy = "value"  # <p>The Identity and Access Management (Amazon Web Services IAM) policy to attach to the rule group.</p>
    arn = "value"  # <p>The ARN (Amazon Resource Name) for the rule group that you want to share.</p>
}

# Access firewall_rule_group_policy outputs
firewall_rule_group_policy_id = firewall_rule_group_policy.id
firewall_rule_group_policy_firewall_rule_group_policy = firewall_rule_group_policy.firewall_rule_group_policy
```

---


### Resolver_rule_policy

ResolverRulePolicy resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resolver_rule_policy` | String | ✅ | <p>An Identity and Access Management policy statement that lists the rules that you want to share with another Amazon Web Services account and the operations that you want the account 
			to be able to perform. You can specify the following operations in the <code>Action</code> section of the statement:</p>
         <ul>
            <li>
               <p>
                  <code>route53resolver:GetResolverRule</code>
               </p>
            </li>
            <li>
               <p>
                  <code>route53resolver:AssociateResolverRule</code>
               </p>
            </li>
            <li>
               <p>
                  <code>route53resolver:DisassociateResolverRule</code>
               </p>
            </li>
            <li>
               <p>
                  <code>route53resolver:ListResolverRules</code>
               </p>
            </li>
            <li>
               <p>
                  <code>route53resolver:ListResolverRuleAssociations</code>
               </p>
            </li>
         </ul>
         <p>In the <code>Resource</code> section of the statement, specify the ARN for the rule that you want to share with another account. Specify the same ARN 
			that you specified in <code>Arn</code>.</p> |
| `arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the rule that you want to share with another account.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `resolver_rule_policy` | String | <p>The Resolver rule policy for the rule that you specified in a <code>GetResolverRulePolicy</code> request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create resolver_rule_policy
resolver_rule_policy = provider.route53resolver.Resolver_rule_policy {
    resolver_rule_policy = "value"  # <p>An Identity and Access Management policy statement that lists the rules that you want to share with another Amazon Web Services account and the operations that you want the account 
			to be able to perform. You can specify the following operations in the <code>Action</code> section of the statement:</p>
         <ul>
            <li>
               <p>
                  <code>route53resolver:GetResolverRule</code>
               </p>
            </li>
            <li>
               <p>
                  <code>route53resolver:AssociateResolverRule</code>
               </p>
            </li>
            <li>
               <p>
                  <code>route53resolver:DisassociateResolverRule</code>
               </p>
            </li>
            <li>
               <p>
                  <code>route53resolver:ListResolverRules</code>
               </p>
            </li>
            <li>
               <p>
                  <code>route53resolver:ListResolverRuleAssociations</code>
               </p>
            </li>
         </ul>
         <p>In the <code>Resource</code> section of the statement, specify the ARN for the rule that you want to share with another account. Specify the same ARN 
			that you specified in <code>Arn</code>.</p>
    arn = "value"  # <p>The Amazon Resource Name (ARN) of the rule that you want to share with another account.</p>
}

# Access resolver_rule_policy outputs
resolver_rule_policy_id = resolver_rule_policy.id
resolver_rule_policy_resolver_rule_policy = resolver_rule_policy.resolver_rule_policy
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple resolver_query_log_config resources
resolver_query_log_config_0 = provider.route53resolver.Resolver_query_log_config {
    name = "value-0"
    destination_arn = "value-0"
    creator_request_id = "value-0"
}
resolver_query_log_config_1 = provider.route53resolver.Resolver_query_log_config {
    name = "value-1"
    destination_arn = "value-1"
    creator_request_id = "value-1"
}
resolver_query_log_config_2 = provider.route53resolver.Resolver_query_log_config {
    name = "value-2"
    destination_arn = "value-2"
    creator_request_id = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    resolver_query_log_config = provider.route53resolver.Resolver_query_log_config {
        name = "production-value"
        destination_arn = "production-value"
        creator_request_id = "production-value"
    }
```

---

## Related Documentation

- [AWS Route53resolver Documentation](https://docs.aws.amazon.com/route53resolver/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
