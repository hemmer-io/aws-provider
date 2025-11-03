# Apprunner Service



**Resources**: 8

---

## Overview

The apprunner service provides access to 8 resource types:

- [Default_auto_scaling_configuration](#default_auto_scaling_configuration) [U]
- [Vpc_ingress_connection](#vpc_ingress_connection) [CRUD]
- [Observability_configuration](#observability_configuration) [CRD]
- [Service](#service) [CRUD]
- [Connection](#connection) [CD]
- [Custom_domains](#custom_domains) [R]
- [Vpc_connector](#vpc_connector) [CRD]
- [Auto_scaling_configuration](#auto_scaling_configuration) [CRD]

---

## Resources


### Default_auto_scaling_configuration

DefaultAutoScalingConfiguration resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `auto_scaling_configuration_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the App Runner auto scaling configuration that you want to set as the default.</p>
         <p>The ARN can be a full auto scaling configuration ARN, or a partial ARN ending with either <code>.../<i>name</i>
            </code> or
          <code>.../<i>name</i>/<i>revision</i>
            </code>. If a revision isn't specified, the latest active revision is set as the
      default.</p> |



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


### Vpc_ingress_connection

VpcIngressConnection resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `vpc_ingress_connection_name` | String | ✅ | <p>A name for the VPC Ingress Connection resource. It must be unique across all the active VPC Ingress Connections in your Amazon Web Services account in the Amazon Web Services Region.
    </p> |
| `tags` | Vec<String> |  | <p>An optional list of metadata items that you can associate with the VPC Ingress Connection resource. A tag is a key-value pair.</p> |
| `service_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) for this App Runner service that is used to create the VPC Ingress Connection resource.</p> |
| `ingress_vpc_configuration` | String | ✅ | <p>Specifications for the customer’s Amazon VPC and the related Amazon Web Services PrivateLink VPC endpoint that are used to create the VPC Ingress Connection
      resource.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `vpc_ingress_connection` | String | <p>A description of the App Runner VPC Ingress Connection that you specified in this request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create vpc_ingress_connection
vpc_ingress_connection = provider.apprunner.Vpc_ingress_connection {
    vpc_ingress_connection_name = "value"  # <p>A name for the VPC Ingress Connection resource. It must be unique across all the active VPC Ingress Connections in your Amazon Web Services account in the Amazon Web Services Region.
    </p>
    service_arn = "value"  # <p>The Amazon Resource Name (ARN) for this App Runner service that is used to create the VPC Ingress Connection resource.</p>
    ingress_vpc_configuration = "value"  # <p>Specifications for the customer’s Amazon VPC and the related Amazon Web Services PrivateLink VPC endpoint that are used to create the VPC Ingress Connection
      resource.</p>
}

# Access vpc_ingress_connection outputs
vpc_ingress_connection_id = vpc_ingress_connection.id
vpc_ingress_connection_vpc_ingress_connection = vpc_ingress_connection.vpc_ingress_connection
```

---


### Observability_configuration

ObservabilityConfiguration resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `trace_configuration` | String |  | <p>The configuration of the tracing feature within this observability configuration. If you don't specify it, App Runner doesn't enable tracing.</p> |
| `tags` | Vec<String> |  | <p>A list of metadata items that you can associate with your observability configuration resource. A tag is a key-value pair.</p> |
| `observability_configuration_name` | String | ✅ | <p>A name for the observability configuration. When you use it for the first time in an Amazon Web Services Region, App Runner creates revision number
        <code>1</code> of this name. When you use the same name in subsequent calls, App Runner creates incremental revisions of the configuration.</p>
         <note>
            <p>The name <code>DefaultConfiguration</code> is reserved. You can't use it to create a new observability configuration, and you can't create a
        revision of it.</p>
            <p>When you want to use your own observability configuration for your App Runner service, <i>create a configuration with a different name</i>,
        and then provide it when you create or update your service.</p>
         </note> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `observability_configuration` | String | <p>A full description of the App Runner observability configuration that you specified in this request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create observability_configuration
observability_configuration = provider.apprunner.Observability_configuration {
    observability_configuration_name = "value"  # <p>A name for the observability configuration. When you use it for the first time in an Amazon Web Services Region, App Runner creates revision number
        <code>1</code> of this name. When you use the same name in subsequent calls, App Runner creates incremental revisions of the configuration.</p>
         <note>
            <p>The name <code>DefaultConfiguration</code> is reserved. You can't use it to create a new observability configuration, and you can't create a
        revision of it.</p>
            <p>When you want to use your own observability configuration for your App Runner service, <i>create a configuration with a different name</i>,
        and then provide it when you create or update your service.</p>
         </note>
}

# Access observability_configuration outputs
observability_configuration_id = observability_configuration.id
observability_configuration_observability_configuration = observability_configuration.observability_configuration
```

---


### Service

Service resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `health_check_configuration` | String |  | <p>The settings for the health check that App Runner performs to monitor the health of the App Runner service.</p> |
| `service_name` | String | ✅ | <p>A name for the App Runner service. It must be unique across all the running App Runner services in your Amazon Web Services account in the Amazon Web Services Region.</p> |
| `source_configuration` | String | ✅ | <p>The source to deploy to the App Runner service. It can be a code or an image repository.</p> |
| `tags` | Vec<String> |  | <p>An optional list of metadata items that you can associate with the App Runner service resource. A tag is a key-value pair.</p> |
| `observability_configuration` | String |  | <p>The observability configuration of your service.</p> |
| `instance_configuration` | String |  | <p>The runtime configuration of instances (scaling units) of your service.</p> |
| `encryption_configuration` | String |  | <p>An optional custom encryption key that App Runner uses to encrypt the copy of your source repository that it maintains and your service logs. By default,
      App Runner uses an Amazon Web Services managed key.</p> |
| `auto_scaling_configuration_arn` | String |  | <p>The Amazon Resource Name (ARN) of an App Runner automatic scaling configuration resource that you want to associate with your service. If not provided, App Runner
      associates the latest revision of a default auto scaling configuration.</p>
         <p>Specify an ARN with a name and a revision number to associate that revision. For example:
          <code>arn:aws:apprunner:us-east-1:123456789012:autoscalingconfiguration/high-availability/3</code>
         </p>
         <p>Specify just the name to associate the latest revision. For example:
        <code>arn:aws:apprunner:us-east-1:123456789012:autoscalingconfiguration/high-availability</code>
         </p> |
| `network_configuration` | String |  | <p>Configuration settings related to network traffic of the web application that the App Runner service runs.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `service` | String | <p>A full description of the App Runner service that you specified in this request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create service
service = provider.apprunner.Service {
    service_name = "value"  # <p>A name for the App Runner service. It must be unique across all the running App Runner services in your Amazon Web Services account in the Amazon Web Services Region.</p>
    source_configuration = "value"  # <p>The source to deploy to the App Runner service. It can be a code or an image repository.</p>
}

# Access service outputs
service_id = service.id
service_service = service.service
```

---


### Connection

Connection resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `provider_type` | String | ✅ | <p>The source repository provider.</p> |
| `connection_name` | String | ✅ | <p>A name for the new connection. It must be unique across all App Runner connections for the Amazon Web Services account in the Amazon Web Services Region.</p> |
| `tags` | Vec<String> |  | <p>A list of metadata items that you can associate with your connection resource. A tag is a key-value pair.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create connection
connection = provider.apprunner.Connection {
    provider_type = "value"  # <p>The source repository provider.</p>
    connection_name = "value"  # <p>A name for the new connection. It must be unique across all App Runner connections for the Amazon Web Services account in the Amazon Web Services Region.</p>
}

```

---


### Custom_domains

CustomDomains resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `service_arn` | String | <p>The Amazon Resource Name (ARN) of the App Runner service whose associated custom domain names you want to describe.</p> |
| `dns_target` | String | <p>The App Runner subdomain of the App Runner service. The associated custom domain names are mapped to this target name.</p> |
| `custom_domains` | Vec<String> | <p>A list of descriptions of custom domain names that are associated with the service. In a paginated request, the request returns up to
        <code>MaxResults</code> records per call.</p> |
| `vpc_dns_targets` | Vec<String> | <p>DNS Target records for the custom domains of this Amazon VPC.
      </p> |
| `next_token` | String | <p>The token that you can pass in a subsequent request to get the next result page. It's returned in a paginated request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access custom_domains outputs
custom_domains_id = custom_domains.id
custom_domains_service_arn = custom_domains.service_arn
custom_domains_dns_target = custom_domains.dns_target
custom_domains_custom_domains = custom_domains.custom_domains
custom_domains_vpc_dns_targets = custom_domains.vpc_dns_targets
custom_domains_next_token = custom_domains.next_token
```

---


### Vpc_connector

VpcConnector resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `security_groups` | String |  | <p>A list of IDs of security groups that App Runner should use for access to Amazon Web Services resources under the specified subnets. If not specified, App Runner uses the
      default security group of the Amazon VPC. The default security group allows all outbound traffic.</p> |
| `tags` | Vec<String> |  | <p>A list of metadata items that you can associate with your VPC connector resource. A tag is a key-value pair.</p> |
| `vpc_connector_name` | String | ✅ | <p>A name for the VPC connector.</p> |
| `subnets` | String | ✅ | <p>A list of IDs of subnets that App Runner should use when it associates your service with a custom Amazon VPC. Specify IDs of subnets of a single
        Amazon VPC. App Runner determines the Amazon VPC from the subnets you specify.</p>
         <note>
            <p>
        App Runner only supports subnets of IP address type <i>IPv4</i> and <i>dual stack</i> (IPv4 and IPv6).</p>
         </note> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `vpc_connector` | String | <p>A description of the App Runner VPC connector that you specified in this request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create vpc_connector
vpc_connector = provider.apprunner.Vpc_connector {
    vpc_connector_name = "value"  # <p>A name for the VPC connector.</p>
    subnets = "value"  # <p>A list of IDs of subnets that App Runner should use when it associates your service with a custom Amazon VPC. Specify IDs of subnets of a single
        Amazon VPC. App Runner determines the Amazon VPC from the subnets you specify.</p>
         <note>
            <p>
        App Runner only supports subnets of IP address type <i>IPv4</i> and <i>dual stack</i> (IPv4 and IPv6).</p>
         </note>
}

# Access vpc_connector outputs
vpc_connector_id = vpc_connector.id
vpc_connector_vpc_connector = vpc_connector.vpc_connector
```

---


### Auto_scaling_configuration

AutoScalingConfiguration resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `min_size` | i64 |  | <p>The minimum number of instances that App Runner provisions for your service. The service always has at least <code>MinSize</code> provisioned instances.
      Some of them actively serve traffic. The rest of them (provisioned and inactive instances) are a cost-effective compute capacity reserve and are ready to
      be quickly activated. You pay for memory usage of all the provisioned instances. You pay for CPU usage of only the active subset.</p>
         <p>App Runner temporarily doubles the number of provisioned instances during deployments, to maintain the same capacity for both old and new code.</p>
         <p>Default: <code>1</code>
         </p> |
| `max_concurrency` | i64 |  | <p>The maximum number of concurrent requests that you want an instance to process. If the number of concurrent requests exceeds this limit, App Runner scales
      up your service.</p>
         <p>Default: <code>100</code>
         </p> |
| `tags` | Vec<String> |  | <p>A list of metadata items that you can associate with your auto scaling configuration resource. A tag is a key-value pair.</p> |
| `max_size` | i64 |  | <p>The maximum number of instances that your service scales up to. At most <code>MaxSize</code> instances actively serve traffic for your service.</p>
         <p>Default: <code>25</code>
         </p> |
| `auto_scaling_configuration_name` | String | ✅ | <p>A name for the auto scaling configuration. When you use it for the first time in an Amazon Web Services Region, App Runner creates revision number
        <code>1</code> of this name. When you use the same name in subsequent calls, App Runner creates incremental revisions of the configuration.</p>
         <note>
            <p>Prior to the release of <a href="https://docs.aws.amazon.com/apprunner/latest/relnotes/release-2023-09-22-auto-scale-config.html">Auto scale
        configuration enhancements</a>, the name <code>DefaultConfiguration</code> was reserved. </p>
            <p>This restriction is no longer in place. You can now manage <code>DefaultConfiguration</code> the same way you manage your custom auto scaling
        configurations. This means you can do the following with the <code>DefaultConfiguration</code> that App Runner provides:</p>
            <ul>
               <li>
                  <p>Create new revisions of the <code>DefaultConfiguration</code>.</p>
               </li>
               <li>
                  <p>Delete the revisions of the <code>DefaultConfiguration</code>.</p>
               </li>
               <li>
                  <p>Delete the auto scaling configuration for which the App Runner <code>DefaultConfiguration</code> was created.</p>
               </li>
               <li>
                  <p>If you delete the auto scaling configuration you can create another custom auto scaling configuration with the same
              <code>DefaultConfiguration</code> name. The original <code>DefaultConfiguration</code> resource provided by App Runner remains in your account unless
            you make changes to it.</p>
               </li>
            </ul>
         </note> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `auto_scaling_configuration` | String | <p>A full description of the App Runner auto scaling configuration that you specified in this request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create auto_scaling_configuration
auto_scaling_configuration = provider.apprunner.Auto_scaling_configuration {
    auto_scaling_configuration_name = "value"  # <p>A name for the auto scaling configuration. When you use it for the first time in an Amazon Web Services Region, App Runner creates revision number
        <code>1</code> of this name. When you use the same name in subsequent calls, App Runner creates incremental revisions of the configuration.</p>
         <note>
            <p>Prior to the release of <a href="https://docs.aws.amazon.com/apprunner/latest/relnotes/release-2023-09-22-auto-scale-config.html">Auto scale
        configuration enhancements</a>, the name <code>DefaultConfiguration</code> was reserved. </p>
            <p>This restriction is no longer in place. You can now manage <code>DefaultConfiguration</code> the same way you manage your custom auto scaling
        configurations. This means you can do the following with the <code>DefaultConfiguration</code> that App Runner provides:</p>
            <ul>
               <li>
                  <p>Create new revisions of the <code>DefaultConfiguration</code>.</p>
               </li>
               <li>
                  <p>Delete the revisions of the <code>DefaultConfiguration</code>.</p>
               </li>
               <li>
                  <p>Delete the auto scaling configuration for which the App Runner <code>DefaultConfiguration</code> was created.</p>
               </li>
               <li>
                  <p>If you delete the auto scaling configuration you can create another custom auto scaling configuration with the same
              <code>DefaultConfiguration</code> name. The original <code>DefaultConfiguration</code> resource provided by App Runner remains in your account unless
            you make changes to it.</p>
               </li>
            </ul>
         </note>
}

# Access auto_scaling_configuration outputs
auto_scaling_configuration_id = auto_scaling_configuration.id
auto_scaling_configuration_auto_scaling_configuration = auto_scaling_configuration.auto_scaling_configuration
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple default_auto_scaling_configuration resources
default_auto_scaling_configuration_0 = provider.apprunner.Default_auto_scaling_configuration {
    auto_scaling_configuration_arn = "value-0"
}
default_auto_scaling_configuration_1 = provider.apprunner.Default_auto_scaling_configuration {
    auto_scaling_configuration_arn = "value-1"
}
default_auto_scaling_configuration_2 = provider.apprunner.Default_auto_scaling_configuration {
    auto_scaling_configuration_arn = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    default_auto_scaling_configuration = provider.apprunner.Default_auto_scaling_configuration {
        auto_scaling_configuration_arn = "production-value"
    }
```

---

## Related Documentation

- [AWS Apprunner Documentation](https://docs.aws.amazon.com/apprunner/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
