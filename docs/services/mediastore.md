# Mediastore Service



**Resources**: 5

---

## Overview

The mediastore service provides access to 5 resource types:

- [Metric_policy](#metric_policy) [CRD]
- [Container](#container) [CRD]
- [Cors_policy](#cors_policy) [CRD]
- [Lifecycle_policy](#lifecycle_policy) [CRD]
- [Container_policy](#container_policy) [CRD]

---

## Resources


### Metric_policy

MetricPolicy resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `metric_policy` | String | ✅ | <p>The metric policy that you want to associate with the container. In the policy, you must indicate whether you want MediaStore to send container-level metrics. You can also include up to five rules to define groups of objects that you want MediaStore to send object-level metrics for.  If you include rules in the policy, construct each rule with both of the following:</p>
         <ul>
            <li>
               <p>An object group that defines which objects to include in the group. The definition can be a path or a file name, but it can't have more than 900 characters. Valid characters are: a-z, A-Z, 0-9, _ (underscore), = (equal), : (colon), . (period), - (hyphen), ~ (tilde), / (forward slash), and * (asterisk). Wildcards (*) are acceptable.</p>
            </li>
            <li>
               <p>An object group name that allows you to refer to the object group. The name can't have more than 30 characters. Valid characters are: a-z, A-Z, 0-9, and _ (underscore).</p>
            </li>
         </ul> |
| `container_name` | String | ✅ | <p>The name of the container that you want to add the metric policy to.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `metric_policy` | String | <p>The metric policy that is associated with the specific container.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create metric_policy
metric_policy = provider.mediastore.Metric_policy {
    metric_policy = "value"  # <p>The metric policy that you want to associate with the container. In the policy, you must indicate whether you want MediaStore to send container-level metrics. You can also include up to five rules to define groups of objects that you want MediaStore to send object-level metrics for.  If you include rules in the policy, construct each rule with both of the following:</p>
         <ul>
            <li>
               <p>An object group that defines which objects to include in the group. The definition can be a path or a file name, but it can't have more than 900 characters. Valid characters are: a-z, A-Z, 0-9, _ (underscore), = (equal), : (colon), . (period), - (hyphen), ~ (tilde), / (forward slash), and * (asterisk). Wildcards (*) are acceptable.</p>
            </li>
            <li>
               <p>An object group name that allows you to refer to the object group. The name can't have more than 30 characters. Valid characters are: a-z, A-Z, 0-9, and _ (underscore).</p>
            </li>
         </ul>
    container_name = "value"  # <p>The name of the container that you want to add the metric policy to.</p>
}

# Access metric_policy outputs
metric_policy_id = metric_policy.id
metric_policy_metric_policy = metric_policy.metric_policy
```

---


### Container

Container resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>An array of key:value pairs that you define. These values can be anything that you want. Typically, the tag key represents a category (such as
           "environment") and the tag value represents a specific value within that category (such as "test," "development," or "production"). You can add up to 50 
           tags to each container. For more information about tagging, including naming and usage conventions, see <a href="https://docs.aws.amazon.com/mediastore/latest/ug/tagging.html">Tagging Resources in MediaStore</a>.</p> |
| `container_name` | String | ✅ | <p>The name for the container. The name must be from 1 to 255 characters. Container
         names must be unique to your AWS account within a specific region. As an example, you could
         create a container named <code>movies</code> in every region, as long as you don’t have an
         existing container with that name.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `container` | String | <p>The name of the queried container.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create container
container = provider.mediastore.Container {
    container_name = "value"  # <p>The name for the container. The name must be from 1 to 255 characters. Container
         names must be unique to your AWS account within a specific region. As an example, you could
         create a container named <code>movies</code> in every region, as long as you don’t have an
         existing container with that name.</p>
}

# Access container outputs
container_id = container.id
container_container = container.container
```

---


### Cors_policy

CorsPolicy resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `cors_policy` | Vec<String> | ✅ | <p>The CORS policy to apply to the container.  </p> |
| `container_name` | String | ✅ | <p>The name of the container that you want to assign the CORS policy to.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `cors_policy` | Vec<String> | <p>The CORS policy assigned to the container.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create cors_policy
cors_policy = provider.mediastore.Cors_policy {
    cors_policy = "value"  # <p>The CORS policy to apply to the container.  </p>
    container_name = "value"  # <p>The name of the container that you want to assign the CORS policy to.</p>
}

# Access cors_policy outputs
cors_policy_id = cors_policy.id
cors_policy_cors_policy = cors_policy.cors_policy
```

---


### Lifecycle_policy

LifecyclePolicy resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `container_name` | String | ✅ | <p>The name of the container that you want to assign the object lifecycle policy to.</p> |
| `lifecycle_policy` | String | ✅ | <p>The object lifecycle policy to apply to the container.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `lifecycle_policy` | String | <p>The object lifecycle policy that is assigned to the container.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create lifecycle_policy
lifecycle_policy = provider.mediastore.Lifecycle_policy {
    container_name = "value"  # <p>The name of the container that you want to assign the object lifecycle policy to.</p>
    lifecycle_policy = "value"  # <p>The object lifecycle policy to apply to the container.</p>
}

# Access lifecycle_policy outputs
lifecycle_policy_id = lifecycle_policy.id
lifecycle_policy_lifecycle_policy = lifecycle_policy.lifecycle_policy
```

---


### Container_policy

ContainerPolicy resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `container_name` | String | ✅ | <p>The name of the container.</p> |
| `policy` | String | ✅ | <p>The contents of the policy, which includes the following: </p>
         <ul>
            <li>
               <p>One <code>Version</code> tag</p>
            </li>
            <li>
               <p>One <code>Statement</code> tag that contains the standard tags for the
               policy.</p>
            </li>
         </ul> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `policy` | String | <p>The contents of the access policy.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create container_policy
container_policy = provider.mediastore.Container_policy {
    container_name = "value"  # <p>The name of the container.</p>
    policy = "value"  # <p>The contents of the policy, which includes the following: </p>
         <ul>
            <li>
               <p>One <code>Version</code> tag</p>
            </li>
            <li>
               <p>One <code>Statement</code> tag that contains the standard tags for the
               policy.</p>
            </li>
         </ul>
}

# Access container_policy outputs
container_policy_id = container_policy.id
container_policy_policy = container_policy.policy
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple metric_policy resources
metric_policy_0 = provider.mediastore.Metric_policy {
    metric_policy = "value-0"
    container_name = "value-0"
}
metric_policy_1 = provider.mediastore.Metric_policy {
    metric_policy = "value-1"
    container_name = "value-1"
}
metric_policy_2 = provider.mediastore.Metric_policy {
    metric_policy = "value-2"
    container_name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    metric_policy = provider.mediastore.Metric_policy {
        metric_policy = "production-value"
        container_name = "production-value"
    }
```

---

## Related Documentation

- [AWS Mediastore Documentation](https://docs.aws.amazon.com/mediastore/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
