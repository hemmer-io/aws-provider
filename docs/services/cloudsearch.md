# Cloudsearch Service



**Resources**: 14

---

## Overview

The cloudsearch service provides access to 14 resource types:

- [Domain](#domain) [CD]
- [Scaling_parameters](#scaling_parameters) [RU]
- [Service_access_policies](#service_access_policies) [RU]
- [Analysis_schemes](#analysis_schemes) [R]
- [Domain_endpoint_options](#domain_endpoint_options) [RU]
- [Expressions](#expressions) [R]
- [Suggester](#suggester) [D]
- [Index_fields](#index_fields) [R]
- [Domains](#domains) [R]
- [Analysis_scheme](#analysis_scheme) [D]
- [Index_field](#index_field) [D]
- [Expression](#expression) [D]
- [Suggesters](#suggesters) [R]
- [Availability_options](#availability_options) [RU]

---

## Resources


### Domain

Domain resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `domain_name` | String | ✅ | <p>A name for the domain you are creating. Allowed characters are a-z (lower-case letters), 0-9, and hyphen (-). Domain names must start with a letter or number and be at least 3 and no more than 28 characters long.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create domain
domain = provider.cloudsearch.Domain {
    domain_name = "value"  # <p>A name for the domain you are creating. Allowed characters are a-z (lower-case letters), 0-9, and hyphen (-). Domain names must start with a letter or number and be at least 3 and no more than 28 characters long.</p>
}

```

---


### Scaling_parameters

ScalingParameters resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `domain_name` | String | ✅ |  |
| `scaling_parameters` | String | ✅ |  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `scaling_parameters` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access scaling_parameters outputs
scaling_parameters_id = scaling_parameters.id
scaling_parameters_scaling_parameters = scaling_parameters.scaling_parameters
```

---


### Service_access_policies

ServiceAccessPolicies resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `access_policies` | String | ✅ | <p>The access rules you want to configure. These rules replace any existing rules. </p> |
| `domain_name` | String | ✅ |  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `access_policies` | String | <p>The access rules configured for the domain specified in the request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access service_access_policies outputs
service_access_policies_id = service_access_policies.id
service_access_policies_access_policies = service_access_policies.access_policies
```

---


### Analysis_schemes

AnalysisSchemes resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `analysis_schemes` | Vec<String> | <p>The analysis scheme descriptions.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access analysis_schemes outputs
analysis_schemes_id = analysis_schemes.id
analysis_schemes_analysis_schemes = analysis_schemes.analysis_schemes
```

---


### Domain_endpoint_options

DomainEndpointOptions resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `domain_name` | String | ✅ | <p>A string that represents the name of a domain.</p> |
| `domain_endpoint_options` | String | ✅ | <p>Whether to require that all requests to the domain arrive over HTTPS. We recommend Policy-Min-TLS-1-2-2019-07 for TLSSecurityPolicy. For compatibility with older clients, the default is Policy-Min-TLS-1-0-2019-07. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `domain_endpoint_options` | String | <p>The status and configuration of a search domain's endpoint options.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access domain_endpoint_options outputs
domain_endpoint_options_id = domain_endpoint_options.id
domain_endpoint_options_domain_endpoint_options = domain_endpoint_options.domain_endpoint_options
```

---


### Expressions

Expressions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `expressions` | Vec<String> | <p>The expressions configured for the domain.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access expressions outputs
expressions_id = expressions.id
expressions_expressions = expressions.expressions
```

---


### Suggester

Suggester resource

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


### Index_fields

IndexFields resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `index_fields` | Vec<String> | <p>The index fields configured for the domain.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access index_fields outputs
index_fields_id = index_fields.id
index_fields_index_fields = index_fields.index_fields
```

---


### Domains

Domains resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `domain_status_list` | Vec<String> |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access domains outputs
domains_id = domains.id
domains_domain_status_list = domains.domain_status_list
```

---


### Analysis_scheme

AnalysisScheme resource

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


### Index_field

IndexField resource

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


### Expression

Expression resource

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


### Suggesters

Suggesters resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `suggesters` | Vec<String> | <p>The suggesters configured for the domain specified in the request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access suggesters outputs
suggesters_id = suggesters.id
suggesters_suggesters = suggesters.suggesters
```

---


### Availability_options

AvailabilityOptions resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `domain_name` | String | ✅ |  |
| `multi_az` | bool | ✅ | <p>You expand an existing search domain to a second Availability Zone by setting the Multi-AZ option to true. Similarly, you can turn off the Multi-AZ option to downgrade the domain to a single Availability Zone by setting the Multi-AZ option to <code>false</code>. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `availability_options` | String | <p>The availability options configured for the domain. Indicates whether Multi-AZ is enabled for the domain. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access availability_options outputs
availability_options_id = availability_options.id
availability_options_availability_options = availability_options.availability_options
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple domain resources
domain_0 = provider.cloudsearch.Domain {
    domain_name = "value-0"
}
domain_1 = provider.cloudsearch.Domain {
    domain_name = "value-1"
}
domain_2 = provider.cloudsearch.Domain {
    domain_name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    domain = provider.cloudsearch.Domain {
        domain_name = "production-value"
    }
```

---

## Related Documentation

- [AWS Cloudsearch Documentation](https://docs.aws.amazon.com/cloudsearch/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
