# Lightsail Service



**Resources**: 89

---

## Overview

The lightsail service provides access to 89 resource types:

- [Distribution_bundle](#distribution_bundle) [U]
- [Bucket](#bucket) [CUD]
- [Disk_snapshots](#disk_snapshots) [R]
- [Container_service_registry_login](#container_service_registry_login) [C]
- [Relational_database_from_snapshot](#relational_database_from_snapshot) [C]
- [Bucket_access_keys](#bucket_access_keys) [R]
- [Contact_methods](#contact_methods) [R]
- [Container_service_metric_data](#container_service_metric_data) [R]
- [Container_services](#container_services) [R]
- [Container_images](#container_images) [R]
- [Load_balancer](#load_balancer) [CRD]
- [Key_pair](#key_pair) [CRD]
- [Auto_snapshots](#auto_snapshots) [R]
- [Load_balancer_metric_data](#load_balancer_metric_data) [R]
- [Instance_public_ports](#instance_public_ports) [C]
- [Container_service](#container_service) [CUD]
- [Container_image](#container_image) [D]
- [Bundles](#bundles) [R]
- [Relational_database_log_streams](#relational_database_log_streams) [R]
- [Container_log](#container_log) [R]
- [Cost_estimate](#cost_estimate) [R]
- [Gui_session_access_details](#gui_session_access_details) [C]
- [Cloud_formation_stack_records](#cloud_formation_stack_records) [R]
- [Instance_access_details](#instance_access_details) [R]
- [Contact_method](#contact_method) [CD]
- [Distribution_metric_data](#distribution_metric_data) [R]
- [Instance_port_states](#instance_port_states) [R]
- [Key_pairs](#key_pairs) [R]
- [Instance](#instance) [RD]
- [Disk_from_snapshot](#disk_from_snapshot) [C]
- [Bucket_metric_data](#bucket_metric_data) [R]
- [Domains](#domains) [R]
- [Disk](#disk) [CRD]
- [Cloud_formation_stack](#cloud_formation_stack) [C]
- [Relational_database_snapshots](#relational_database_snapshots) [R]
- [Bucket_bundle](#bucket_bundle) [U]
- [Load_balancer_tls_certificate](#load_balancer_tls_certificate) [CD]
- [Container_service_deployment](#container_service_deployment) [C]
- [Alarm](#alarm) [CD]
- [Export_snapshot_records](#export_snapshot_records) [R]
- [Certificates](#certificates) [R]
- [Buckets](#buckets) [R]
- [Instance_snapshots](#instance_snapshots) [R]
- [Operations_for_resource](#operations_for_resource) [R]
- [Regions](#regions) [R]
- [Distribution_bundles](#distribution_bundles) [R]
- [Load_balancer_attribute](#load_balancer_attribute) [U]
- [Container_api_metadata](#container_api_metadata) [R]
- [Certificate](#certificate) [CD]
- [Active_names](#active_names) [R]
- [Bucket_bundles](#bucket_bundles) [R]
- [Domain_entry](#domain_entry) [CUD]
- [Distribution_latest_cache_reset](#distribution_latest_cache_reset) [R]
- [Load_balancers](#load_balancers) [R]
- [Load_balancer_tls_certificates](#load_balancer_tls_certificates) [R]
- [Relational_database_blueprints](#relational_database_blueprints) [R]
- [Relational_database_bundles](#relational_database_bundles) [R]
- [Instance_metadata_options](#instance_metadata_options) [U]
- [Instances_from_snapshot](#instances_from_snapshot) [C]
- [Static_ip](#static_ip) [R]
- [Relational_database_master_user_password](#relational_database_master_user_password) [R]
- [Blueprints](#blueprints) [R]
- [Container_service_deployments](#container_service_deployments) [R]
- [Instance_snapshot](#instance_snapshot) [CRD]
- [Static_ips](#static_ips) [R]
- [Relational_database_events](#relational_database_events) [R]
- [Container_service_powers](#container_service_powers) [R]
- [Disk_snapshot](#disk_snapshot) [CRD]
- [Bucket_access_key](#bucket_access_key) [CD]
- [Instances](#instances) [CR]
- [Load_balancer_tls_policies](#load_balancer_tls_policies) [R]
- [Instance_metric_data](#instance_metric_data) [R]
- [Relational_database_metric_data](#relational_database_metric_data) [R]
- [Domain](#domain) [CRD]
- [Relational_database](#relational_database) [CRUD]
- [Relational_databases](#relational_databases) [R]
- [Disks](#disks) [R]
- [Distributions](#distributions) [R]
- [Relational_database_snapshot](#relational_database_snapshot) [CRD]
- [Relational_database_parameters](#relational_database_parameters) [RU]
- [Instance_state](#instance_state) [R]
- [Operation](#operation) [R]
- [Relational_database_log_events](#relational_database_log_events) [R]
- [Distribution](#distribution) [CUD]
- [Known_host_keys](#known_host_keys) [D]
- [Setup_history](#setup_history) [R]
- [Auto_snapshot](#auto_snapshot) [D]
- [Alarms](#alarms) [R]
- [Operations](#operations) [R]

---

## Resources


### Distribution_bundle

DistributionBundle resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `distribution_name` | String |  | <p>The name of the distribution for which to update the bundle.</p>
         <p>Use the <code>GetDistributions</code> action to get a list of distribution names that you
      can specify.</p> |
| `bundle_id` | String |  | <p>The bundle ID of the new bundle to apply to your distribution.</p>
         <p>Use the <code>GetDistributionBundles</code> action to get a list of distribution bundle
      IDs that you can specify.</p> |



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


### Bucket

Bucket resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `enable_object_versioning` | bool |  | <p>A Boolean value that indicates whether to enable versioning of objects in the
      bucket.</p>
         <p>For more information about versioning, see <a href="https://docs.aws.amazon.com/lightsail/latest/userguide/amazon-lightsail-managing-bucket-object-versioning">Enabling and suspending object versioning in a bucket in Amazon Lightsail</a> in the
        <i>Amazon Lightsail Developer Guide</i>.</p> |
| `bundle_id` | String | ✅ | <p>The ID of the bundle to use for the bucket.</p>
         <p>A bucket bundle specifies the monthly cost, storage space, and data transfer quota for a
      bucket.</p>
         <p>Use the <a href="https://docs.aws.amazon.com/lightsail/2016-11-28/api-reference/API_GetBucketBundles.html">GetBucketBundles</a> action to get a list of
      bundle IDs that you can specify.</p>
         <p>Use the <a href="https://docs.aws.amazon.com/lightsail/2016-11-28/api-reference/API_UpdateBucketBundle.html">UpdateBucketBundle</a> action to change the
      bundle after the bucket is created.</p> |
| `tags` | Vec<String> |  | <p>The tag keys and optional values to add to the bucket during creation.</p>
         <p>Use the <a href="https://docs.aws.amazon.com/lightsail/2016-11-28/api-reference/API_TagResource.html">TagResource</a> action to tag the bucket after it's
      created.</p> |
| `bucket_name` | String | ✅ | <p>The name for the bucket.</p>
         <p>For more information about bucket names, see <a href="https://docs.aws.amazon.com/lightsail/latest/userguide/bucket-naming-rules-in-amazon-lightsail">Bucket naming rules in Amazon Lightsail</a> in the <i>Amazon Lightsail Developer
        Guide</i>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create bucket
bucket = provider.lightsail.Bucket {
    bundle_id = "value"  # <p>The ID of the bundle to use for the bucket.</p>
         <p>A bucket bundle specifies the monthly cost, storage space, and data transfer quota for a
      bucket.</p>
         <p>Use the <a href="https://docs.aws.amazon.com/lightsail/2016-11-28/api-reference/API_GetBucketBundles.html">GetBucketBundles</a> action to get a list of
      bundle IDs that you can specify.</p>
         <p>Use the <a href="https://docs.aws.amazon.com/lightsail/2016-11-28/api-reference/API_UpdateBucketBundle.html">UpdateBucketBundle</a> action to change the
      bundle after the bucket is created.</p>
    bucket_name = "value"  # <p>The name for the bucket.</p>
         <p>For more information about bucket names, see <a href="https://docs.aws.amazon.com/lightsail/latest/userguide/bucket-naming-rules-in-amazon-lightsail">Bucket naming rules in Amazon Lightsail</a> in the <i>Amazon Lightsail Developer
        Guide</i>.</p>
}

```

---


### Disk_snapshots

DiskSnapshots resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | <p>The token to advance to the next page of results from your request.</p>
         <p>A next page token is not returned if there are no more results to display.</p>
         <p>To get the next page of results, perform another <code>GetDiskSnapshots</code> request and
      specify the next page token using the <code>pageToken</code> parameter.</p> |
| `disk_snapshots` | Vec<String> | <p>An array of objects containing information about all block storage disk snapshots.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access disk_snapshots outputs
disk_snapshots_id = disk_snapshots.id
disk_snapshots_next_page_token = disk_snapshots.next_page_token
disk_snapshots_disk_snapshots = disk_snapshots.disk_snapshots
```

---


### Container_service_registry_login

ContainerServiceRegistryLogin resource

**Operations**: ✅ Create

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

# Create container_service_registry_login
container_service_registry_login = provider.lightsail.Container_service_registry_login {
}

```

---


### Relational_database_from_snapshot

RelationalDatabaseFromSnapshot resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>The tag keys and optional values to add to the resource during create.</p>
         <p>Use the <code>TagResource</code> action to tag a resource after it's created.</p> |
| `relational_database_name` | String | ✅ | <p>The name to use for your new Lightsail database resource.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must contain from 2 to 255 alphanumeric characters, or hyphens.</p>
            </li>
            <li>
               <p>The first and last character must be a letter or number.</p>
            </li>
         </ul> |
| `source_relational_database_name` | String |  | <p>The name of the source database.</p> |
| `use_latest_restorable_time` | bool |  | <p>Specifies whether your database is restored from the latest backup time. A value of
        <code>true</code> restores from the latest backup time. </p>
         <p>Default: <code>false</code>
         </p>
         <p>Constraints: Cannot be specified if the <code>restore time</code> parameter is
      provided.</p> |
| `availability_zone` | String |  | <p>The Availability Zone in which to create your new database. Use the
        <code>us-east-2a</code> case-sensitive format.</p>
         <p>You can get a list of Availability Zones by using the <code>get regions</code> operation.
      Be sure to add the <code>include relational database Availability Zones</code> parameter to
      your request.</p> |
| `relational_database_bundle_id` | String |  | <p>The bundle ID for your new database. A bundle describes the performance specifications for
      your database.</p>
         <p>You can get a list of database bundle IDs by using the <code>get relational database
        bundles</code> operation.</p>
         <p>When creating a new database from a snapshot, you cannot choose a bundle that is smaller
      than the bundle of the source database.</p> |
| `relational_database_snapshot_name` | String |  | <p>The name of the database snapshot from which to create your new database.</p> |
| `restore_time` | String |  | <p>The date and time to restore your database from.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must be before the latest restorable time for the database.</p>
            </li>
            <li>
               <p>Cannot be specified if the <code>use latest restorable time</code> parameter is
            <code>true</code>.</p>
            </li>
            <li>
               <p>Specified in Coordinated Universal Time (UTC).</p>
            </li>
            <li>
               <p>Specified in the Unix time format.</p>
               <p>For example, if you wish to use a restore time of October 1, 2018, at 8 PM UTC, then
          you input <code>1538424000</code> as the restore time.</p>
            </li>
         </ul> |
| `publicly_accessible` | bool |  | <p>Specifies the accessibility options for your new database. A value of <code>true</code>
      specifies a database that is available to resources outside of your Lightsail account. A
      value of <code>false</code> specifies a database that is available only to your Lightsail
      resources in the same region as your database.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create relational_database_from_snapshot
relational_database_from_snapshot = provider.lightsail.Relational_database_from_snapshot {
    relational_database_name = "value"  # <p>The name to use for your new Lightsail database resource.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must contain from 2 to 255 alphanumeric characters, or hyphens.</p>
            </li>
            <li>
               <p>The first and last character must be a letter or number.</p>
            </li>
         </ul>
}

```

---


### Bucket_access_keys

BucketAccessKeys resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `access_keys` | Vec<String> | <p>An object that describes the access keys for the specified bucket.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access bucket_access_keys outputs
bucket_access_keys_id = bucket_access_keys.id
bucket_access_keys_access_keys = bucket_access_keys.access_keys
```

---


### Contact_methods

ContactMethods resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `contact_methods` | Vec<String> | <p>An array of objects that describe the contact methods.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access contact_methods outputs
contact_methods_id = contact_methods.id
contact_methods_contact_methods = contact_methods.contact_methods
```

---


### Container_service_metric_data

ContainerServiceMetricData resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `metric_name` | String | <p>The name of the metric returned. </p> |
| `metric_data` | Vec<String> | <p>An array of objects that describe the metric data returned.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access container_service_metric_data outputs
container_service_metric_data_id = container_service_metric_data.id
container_service_metric_data_metric_name = container_service_metric_data.metric_name
container_service_metric_data_metric_data = container_service_metric_data.metric_data
```

---


### Container_services

ContainerServices resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `container_services` | Vec<String> | <p>An array of objects that describe one or more container services.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access container_services outputs
container_services_id = container_services.id
container_services_container_services = container_services.container_services
```

---


### Container_images

ContainerImages resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `container_images` | Vec<String> | <p>An array of objects that describe container images that are registered to the container
      service.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access container_images outputs
container_images_id = container_images.id
container_images_container_images = container_images.container_images
```

---


### Load_balancer

LoadBalancer resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tls_policy_name` | String |  | <p>The name of the TLS policy to apply to the load balancer.</p>
         <p>Use the <a href="https://docs.aws.amazon.com/lightsail/2016-11-28/api-reference/API_GetLoadBalancerTlsPolicies.html">GetLoadBalancerTlsPolicies</a> action to get a list of TLS policy names that you can
      specify.</p>
         <p>For more information about load balancer TLS policies, see <a href="https://docs.aws.amazon.com/lightsail/latest/userguide/amazon-lightsail-configure-load-balancer-tls-security-policy">Configuring TLS security policies on your Amazon Lightsail load
        balancers</a> in the <i>Amazon Lightsail Developer Guide</i>.</p> |
| `instance_port` | i64 | ✅ | <p>The instance port where you're creating your load balancer.</p> |
| `health_check_path` | String |  | <p>The path you provided to perform the load balancer health check. If you didn't specify a
      health check path, Lightsail uses the root path of your website (<code>"/"</code>).</p>
         <p>You may want to specify a custom health check path other than the root of your application
      if your home page loads slowly or has a lot of media or scripting on it.</p> |
| `certificate_name` | String |  | <p>The name of the SSL/TLS certificate.</p>
         <p>If you specify <code>certificateName</code>, then <code>certificateDomainName</code> is
      required (and vice-versa).</p> |
| `certificate_domain_name` | String |  | <p>The domain name with which your certificate is associated
      (<code>example.com</code>).</p>
         <p>If you specify <code>certificateDomainName</code>, then <code>certificateName</code> is
      required (and vice-versa).</p> |
| `certificate_alternative_names` | Vec<String> |  | <p>The optional alternative domains and subdomains to use with your SSL/TLS certificate
        (<code>www.example.com</code>, <code>example.com</code>, <code>m.example.com</code>,
        <code>blog.example.com</code>).</p> |
| `ip_address_type` | String |  | <p>The IP address type for the load balancer.</p>
         <p>The possible values are <code>ipv4</code> for IPv4 only, <code>ipv6</code> for IPv6 only,
      and <code>dualstack</code> for IPv4 and IPv6.</p>
         <p>The default value is <code>dualstack</code>.</p> |
| `load_balancer_name` | String | ✅ | <p>The name of your load balancer.</p> |
| `tags` | Vec<String> |  | <p>The tag keys and optional values to add to the resource during create.</p>
         <p>Use the <code>TagResource</code> action to tag a resource after it's created.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `load_balancer` | String | <p>An object containing information about your load balancer.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create load_balancer
load_balancer = provider.lightsail.Load_balancer {
    instance_port = "value"  # <p>The instance port where you're creating your load balancer.</p>
    load_balancer_name = "value"  # <p>The name of your load balancer.</p>
}

# Access load_balancer outputs
load_balancer_id = load_balancer.id
load_balancer_load_balancer = load_balancer.load_balancer
```

---


### Key_pair

KeyPair resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>The tag keys and optional values to add to the resource during create.</p>
         <p>Use the <code>TagResource</code> action to tag a resource after it's created.</p> |
| `key_pair_name` | String | ✅ | <p>The name for your new key pair.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `key_pair` | String | <p>An array of key-value pairs containing information about the key pair.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create key_pair
key_pair = provider.lightsail.Key_pair {
    key_pair_name = "value"  # <p>The name for your new key pair.</p>
}

# Access key_pair outputs
key_pair_id = key_pair.id
key_pair_key_pair = key_pair.key_pair
```

---


### Auto_snapshots

AutoSnapshots resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `auto_snapshots` | Vec<String> | <p>An array of objects that describe the automatic snapshots that are available for the
      specified source instance or disk.</p> |
| `resource_type` | String | <p>The resource type of the automatic snapshot. The possible values are
      <code>Instance</code>, and <code>Disk</code>.</p> |
| `resource_name` | String | <p>The name of the source instance or disk for the automatic snapshots.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access auto_snapshots outputs
auto_snapshots_id = auto_snapshots.id
auto_snapshots_auto_snapshots = auto_snapshots.auto_snapshots
auto_snapshots_resource_type = auto_snapshots.resource_type
auto_snapshots_resource_name = auto_snapshots.resource_name
```

---


### Load_balancer_metric_data

LoadBalancerMetricData resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `metric_data` | Vec<String> | <p>An array of objects that describe the metric data returned.</p> |
| `metric_name` | String | <p>The name of the metric returned.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access load_balancer_metric_data outputs
load_balancer_metric_data_id = load_balancer_metric_data.id
load_balancer_metric_data_metric_data = load_balancer_metric_data.metric_data
load_balancer_metric_data_metric_name = load_balancer_metric_data.metric_name
```

---


### Instance_public_ports

InstancePublicPorts resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `instance_name` | String | ✅ | <p>The name of the instance for which to open ports.</p> |
| `port_infos` | Vec<String> | ✅ | <p>An array of objects to describe the ports to open for the specified instance.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create instance_public_ports
instance_public_ports = provider.lightsail.Instance_public_ports {
    instance_name = "value"  # <p>The name of the instance for which to open ports.</p>
    port_infos = "value"  # <p>An array of objects to describe the ports to open for the specified instance.</p>
}

```

---


### Container_service

ContainerService resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `power` | String | ✅ | <p>The power specification for the container service.</p>
         <p>The power specifies the amount of memory, vCPUs, and base monthly cost of each node of the
      container service. The <code>power</code> and <code>scale</code> of a container service makes
      up its configured capacity. To determine the monthly price of your container service, multiply
      the base price of the <code>power</code> with the <code>scale</code> (the number of nodes) of
      the service.</p>
         <p>Use the <code>GetContainerServicePowers</code> action to get a list of power options that
      you can specify using this parameter, and their base monthly cost.</p> |
| `deployment` | String |  | <p>An object that describes a deployment for the container service.</p>
         <p>A deployment specifies the containers that will be launched on the container service and
      their settings, such as the ports to open, the environment variables to apply, and the launch
      command to run. It also specifies the container that will serve as the public endpoint of the
      deployment and its settings, such as the HTTP or HTTPS port to use, and the health check
      configuration.</p> |
| `tags` | Vec<String> |  | <p>The tag keys and optional values to add to the container service during create.</p>
         <p>Use the <code>TagResource</code> action to tag a resource after it's created.</p>
         <p>For more information about tags in Lightsail, see the <a href="https://docs.aws.amazon.com/lightsail/latest/userguide/amazon-lightsail-tags">Amazon Lightsail Developer Guide</a>.</p> |
| `scale` | i64 | ✅ | <p>The scale specification for the container service.</p>
         <p>The scale specifies the allocated compute nodes of the container service. The
        <code>power</code> and <code>scale</code> of a container service makes up its configured
      capacity. To determine the monthly price of your container service, multiply the base price of
      the <code>power</code> with the <code>scale</code> (the number of nodes) of the
      service.</p> |
| `public_domain_names` | HashMap<String, Vec<String>> |  | <p>The public domain names to use with the container service, such as
        <code>example.com</code> and <code>www.example.com</code>.</p>
         <p>You can specify up to four public domain names for a container service. The domain names
      that you specify are used when you create a deployment with a container configured as the
      public endpoint of your container service.</p>
         <p>If you don't specify public domain names, then you can use the default domain of the
      container service.</p>
         <important>
            <p>You must create and validate an SSL/TLS certificate before you can use public domain
        names with your container service. Use the <code>CreateCertificate</code> action to create a
        certificate for the public domain names you want to use with your container service.</p>
         </important>
         <p>You can specify public domain names using a string to array map as shown in the example
      later on this page.</p> |
| `private_registry_access` | String |  | <p>An object to describe the configuration for the container service to access private
      container image repositories, such as Amazon Elastic Container Registry (Amazon ECR) private
      repositories.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/lightsail/latest/userguide/amazon-lightsail-container-service-ecr-private-repo-access">Configuring access to an Amazon ECR private repository for an Amazon Lightsail container service</a> in the <i>Amazon Lightsail Developer Guide</i>.</p> |
| `service_name` | String | ✅ | <p>The name for the container service.</p>
         <p>The name that you specify for your container service will make up part of its default
      domain. The default domain of a container service is typically
        <code>https://<ServiceName>.<RandomGUID>.<AWSRegion>.cs.amazonlightsail.com</code>.
      If the name of your container service is <code>container-service-1</code>, and it's located in
      the US East (Ohio) Amazon Web Services Region (<code>us-east-2</code>), then the domain for
      your container service will be like the following example:
        <code>https://container-service-1.ur4EXAMPLE2uq.us-east-2.cs.amazonlightsail.com</code>
         </p>
         <p>The following are the requirements for container service names:</p>
         <ul>
            <li>
               <p>Must be unique within each Amazon Web Services Region in your Lightsail
          account.</p>
            </li>
            <li>
               <p>Must contain 1 to 63 characters.</p>
            </li>
            <li>
               <p>Must contain only alphanumeric characters and hyphens.</p>
            </li>
            <li>
               <p>A hyphen (-) can separate words but cannot be at the start or end of the name.</p>
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

# Create container_service
container_service = provider.lightsail.Container_service {
    power = "value"  # <p>The power specification for the container service.</p>
         <p>The power specifies the amount of memory, vCPUs, and base monthly cost of each node of the
      container service. The <code>power</code> and <code>scale</code> of a container service makes
      up its configured capacity. To determine the monthly price of your container service, multiply
      the base price of the <code>power</code> with the <code>scale</code> (the number of nodes) of
      the service.</p>
         <p>Use the <code>GetContainerServicePowers</code> action to get a list of power options that
      you can specify using this parameter, and their base monthly cost.</p>
    scale = "value"  # <p>The scale specification for the container service.</p>
         <p>The scale specifies the allocated compute nodes of the container service. The
        <code>power</code> and <code>scale</code> of a container service makes up its configured
      capacity. To determine the monthly price of your container service, multiply the base price of
      the <code>power</code> with the <code>scale</code> (the number of nodes) of the
      service.</p>
    service_name = "value"  # <p>The name for the container service.</p>
         <p>The name that you specify for your container service will make up part of its default
      domain. The default domain of a container service is typically
        <code>https://<ServiceName>.<RandomGUID>.<AWSRegion>.cs.amazonlightsail.com</code>.
      If the name of your container service is <code>container-service-1</code>, and it's located in
      the US East (Ohio) Amazon Web Services Region (<code>us-east-2</code>), then the domain for
      your container service will be like the following example:
        <code>https://container-service-1.ur4EXAMPLE2uq.us-east-2.cs.amazonlightsail.com</code>
         </p>
         <p>The following are the requirements for container service names:</p>
         <ul>
            <li>
               <p>Must be unique within each Amazon Web Services Region in your Lightsail
          account.</p>
            </li>
            <li>
               <p>Must contain 1 to 63 characters.</p>
            </li>
            <li>
               <p>Must contain only alphanumeric characters and hyphens.</p>
            </li>
            <li>
               <p>A hyphen (-) can separate words but cannot be at the start or end of the name.</p>
            </li>
         </ul>
}

```

---


### Container_image

ContainerImage resource

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


### Bundles

Bundles resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | <p>The token to advance to the next page of results from your request.</p>
         <p>A next page token is not returned if there are no more results to display.</p>
         <p>To get the next page of results, perform another <code>GetBundles</code> request and
      specify the next page token using the <code>pageToken</code> parameter.</p> |
| `bundles` | Vec<String> | <p>An array of key-value pairs that contains information about the available bundles.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access bundles outputs
bundles_id = bundles.id
bundles_next_page_token = bundles.next_page_token
bundles_bundles = bundles.bundles
```

---


### Relational_database_log_streams

RelationalDatabaseLogStreams resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `log_streams` | String | <p>An object describing the result of your get relational database log streams
      request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access relational_database_log_streams outputs
relational_database_log_streams_id = relational_database_log_streams.id
relational_database_log_streams_log_streams = relational_database_log_streams.log_streams
```

---


### Container_log

ContainerLog resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `log_events` | Vec<String> | <p>An array of objects that describe the log events of a container.</p> |
| `next_page_token` | String | <p>The token to advance to the next page of results from your request.</p>
         <p>A next page token is not returned if there are no more results to display.</p>
         <p>To get the next page of results, perform another <code>GetContainerLog</code> request and
      specify the next page token using the <code>pageToken</code> parameter.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access container_log outputs
container_log_id = container_log.id
container_log_log_events = container_log.log_events
container_log_next_page_token = container_log.next_page_token
```

---


### Cost_estimate

CostEstimate resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `resources_budget_estimate` | Vec<String> | <p>Returns the estimate's forecasted cost or usage.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access cost_estimate outputs
cost_estimate_id = cost_estimate.id
cost_estimate_resources_budget_estimate = cost_estimate.resources_budget_estimate
```

---


### Gui_session_access_details

GUISessionAccessDetails resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resource_name` | String | ✅ | <p>The resource name.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create gui_session_access_details
gui_session_access_details = provider.lightsail.Gui_session_access_details {
    resource_name = "value"  # <p>The resource name.</p>
}

```

---


### Cloud_formation_stack_records

CloudFormationStackRecords resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | <p>The token to advance to the next page of results from your request.</p>
         <p>A next page token is not returned if there are no more results to display.</p>
         <p>To get the next page of results, perform another
        <code>GetCloudFormationStackRecords</code> request and specify the next page token using the
        <code>pageToken</code> parameter.</p> |
| `cloud_formation_stack_records` | Vec<String> | <p>A list of objects describing the CloudFormation stack records.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access cloud_formation_stack_records outputs
cloud_formation_stack_records_id = cloud_formation_stack_records.id
cloud_formation_stack_records_next_page_token = cloud_formation_stack_records.next_page_token
cloud_formation_stack_records_cloud_formation_stack_records = cloud_formation_stack_records.cloud_formation_stack_records
```

---


### Instance_access_details

InstanceAccessDetails resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `access_details` | String | <p>An array of key-value pairs containing information about a get instance access
      request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access instance_access_details outputs
instance_access_details_id = instance_access_details.id
instance_access_details_access_details = instance_access_details.access_details
```

---


### Contact_method

ContactMethod resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `protocol` | String | ✅ | <p>The protocol of the contact method, such as <code>Email</code> or <code>SMS</code> (text
      messaging).</p>
         <p>The <code>SMS</code> protocol is supported only in the following Amazon Web Services
      Regions.</p>
         <ul>
            <li>
               <p>US East (N. Virginia) (<code>us-east-1</code>)</p>
            </li>
            <li>
               <p>US West (Oregon) (<code>us-west-2</code>)</p>
            </li>
            <li>
               <p>Europe (Ireland) (<code>eu-west-1</code>)</p>
            </li>
            <li>
               <p>Asia Pacific (Tokyo) (<code>ap-northeast-1</code>)</p>
            </li>
            <li>
               <p>Asia Pacific (Singapore) (<code>ap-southeast-1</code>)</p>
            </li>
            <li>
               <p>Asia Pacific (Sydney) (<code>ap-southeast-2</code>)</p>
            </li>
         </ul>
         <p>For a list of countries/regions where SMS text messages can be sent, and the latest
        Amazon Web Services Regions where SMS text messaging is supported, see <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-supported-regions-countries.html">Supported Regions and Countries</a> in the <i>Amazon SNS Developer
        Guide</i>.</p>
         <p>For more information about notifications in Amazon Lightsail, see <a href="https://docs.aws.amazon.com/lightsail/latest/userguide/amazon-lightsail-notifications">Notifications in Amazon Lightsail</a>.</p> |
| `contact_endpoint` | String | ✅ | <p>The destination of the contact method, such as an email address or a mobile phone
      number.</p>
         <p>Use the E.164 format when specifying a mobile phone number. E.164 is a standard for the
      phone number structure used for international telecommunication. Phone numbers that follow
      this format can have a maximum of 15 digits, and they are prefixed with the plus character (+)
      and the country code. For example, a U.S. phone number in E.164 format would be specified as
      +1XXX5550100. For more information, see <a href="https://en.wikipedia.org/wiki/E.164">E.164</a> on <i>Wikipedia</i>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create contact_method
contact_method = provider.lightsail.Contact_method {
    protocol = "value"  # <p>The protocol of the contact method, such as <code>Email</code> or <code>SMS</code> (text
      messaging).</p>
         <p>The <code>SMS</code> protocol is supported only in the following Amazon Web Services
      Regions.</p>
         <ul>
            <li>
               <p>US East (N. Virginia) (<code>us-east-1</code>)</p>
            </li>
            <li>
               <p>US West (Oregon) (<code>us-west-2</code>)</p>
            </li>
            <li>
               <p>Europe (Ireland) (<code>eu-west-1</code>)</p>
            </li>
            <li>
               <p>Asia Pacific (Tokyo) (<code>ap-northeast-1</code>)</p>
            </li>
            <li>
               <p>Asia Pacific (Singapore) (<code>ap-southeast-1</code>)</p>
            </li>
            <li>
               <p>Asia Pacific (Sydney) (<code>ap-southeast-2</code>)</p>
            </li>
         </ul>
         <p>For a list of countries/regions where SMS text messages can be sent, and the latest
        Amazon Web Services Regions where SMS text messaging is supported, see <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-supported-regions-countries.html">Supported Regions and Countries</a> in the <i>Amazon SNS Developer
        Guide</i>.</p>
         <p>For more information about notifications in Amazon Lightsail, see <a href="https://docs.aws.amazon.com/lightsail/latest/userguide/amazon-lightsail-notifications">Notifications in Amazon Lightsail</a>.</p>
    contact_endpoint = "value"  # <p>The destination of the contact method, such as an email address or a mobile phone
      number.</p>
         <p>Use the E.164 format when specifying a mobile phone number. E.164 is a standard for the
      phone number structure used for international telecommunication. Phone numbers that follow
      this format can have a maximum of 15 digits, and they are prefixed with the plus character (+)
      and the country code. For example, a U.S. phone number in E.164 format would be specified as
      +1XXX5550100. For more information, see <a href="https://en.wikipedia.org/wiki/E.164">E.164</a> on <i>Wikipedia</i>.</p>
}

```

---


### Distribution_metric_data

DistributionMetricData resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `metric_data` | Vec<String> | <p>An array of objects that describe the metric data returned.</p> |
| `metric_name` | String | <p>The name of the metric returned.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access distribution_metric_data outputs
distribution_metric_data_id = distribution_metric_data.id
distribution_metric_data_metric_data = distribution_metric_data.metric_data
distribution_metric_data_metric_name = distribution_metric_data.metric_name
```

---


### Instance_port_states

InstancePortStates resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `port_states` | Vec<String> | <p>An array of objects that describe the firewall port states for the specified
      instance.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access instance_port_states outputs
instance_port_states_id = instance_port_states.id
instance_port_states_port_states = instance_port_states.port_states
```

---


### Key_pairs

KeyPairs resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | <p>The token to advance to the next page of results from your request.</p>
         <p>A next page token is not returned if there are no more results to display.</p>
         <p>To get the next page of results, perform another <code>GetKeyPairs</code> request and
      specify the next page token using the <code>pageToken</code> parameter.</p> |
| `key_pairs` | Vec<String> | <p>An array of key-value pairs containing information about the key pairs.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access key_pairs outputs
key_pairs_id = key_pairs.id
key_pairs_next_page_token = key_pairs.next_page_token
key_pairs_key_pairs = key_pairs.key_pairs
```

---


### Instance

Instance resource

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `instance` | String | <p>An array of key-value pairs containing information about the specified instance.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access instance outputs
instance_id = instance.id
instance_instance = instance.instance
```

---


### Disk_from_snapshot

DiskFromSnapshot resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `availability_zone` | String | ✅ | <p>The Availability Zone where you want to create the disk (<code>us-east-2a</code>). Choose
      the same Availability Zone as the Lightsail instance where you want to create the
      disk.</p>
         <p>Use the GetRegions operation to list the Availability Zones where Lightsail is currently
      available.</p> |
| `source_disk_name` | String |  | <p>The name of the source disk from which the source automatic snapshot was created.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>This parameter cannot be defined together with the <code>disk snapshot name</code>
          parameter. The <code>source disk name</code> and <code>disk snapshot name</code>
          parameters are mutually exclusive.</p>
            </li>
            <li>
               <p>Define this parameter only when creating a new disk from an automatic snapshot. For
          more information, see the <a href="https://docs.aws.amazon.com/lightsail/latest/userguide/amazon-lightsail-configuring-automatic-snapshots">Amazon Lightsail Developer Guide</a>.</p>
            </li>
         </ul> |
| `disk_snapshot_name` | String |  | <p>The name of the disk snapshot (<code>my-snapshot</code>) from which to create the new
      storage disk.</p>
         <p>Constraint:</p>
         <ul>
            <li>
               <p>This parameter cannot be defined together with the <code>source disk name</code>
          parameter. The <code>disk snapshot name</code> and <code>source disk name</code>
          parameters are mutually exclusive.</p>
            </li>
         </ul> |
| `size_in_gb` | i64 | ✅ | <p>The size of the disk in GB (<code>32</code>).</p> |
| `add_ons` | Vec<String> |  | <p>An array of objects that represent the add-ons to enable for the new disk.</p> |
| `restore_date` | String |  | <p>The date of the automatic snapshot to use for the new disk. Use the <code>get auto
        snapshots</code> operation to identify the dates of the available automatic
      snapshots.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must be specified in <code>YYYY-MM-DD</code> format.</p>
            </li>
            <li>
               <p>This parameter cannot be defined together with the <code>use latest restorable auto
            snapshot</code> parameter. The <code>restore date</code> and <code>use latest restorable
            auto snapshot</code> parameters are mutually exclusive.</p>
            </li>
            <li>
               <p>Define this parameter only when creating a new disk from an automatic snapshot. For
          more information, see the <a href="https://docs.aws.amazon.com/lightsail/latest/userguide/amazon-lightsail-configuring-automatic-snapshots">Amazon Lightsail Developer Guide</a>.</p>
            </li>
         </ul> |
| `disk_name` | String | ✅ | <p>The unique Lightsail disk name (<code>my-disk</code>).</p> |
| `tags` | Vec<String> |  | <p>The tag keys and optional values to add to the resource during create.</p>
         <p>Use the <code>TagResource</code> action to tag a resource after it's created.</p> |
| `use_latest_restorable_auto_snapshot` | bool |  | <p>A Boolean value to indicate whether to use the latest available automatic snapshot.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>This parameter cannot be defined together with the <code>restore date</code>
          parameter. The <code>use latest restorable auto snapshot</code> and <code>restore
            date</code> parameters are mutually exclusive.</p>
            </li>
            <li>
               <p>Define this parameter only when creating a new disk from an automatic snapshot. For
          more information, see the <a href="https://docs.aws.amazon.com/lightsail/latest/userguide/amazon-lightsail-configuring-automatic-snapshots">Amazon Lightsail Developer Guide</a>.</p>
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

# Create disk_from_snapshot
disk_from_snapshot = provider.lightsail.Disk_from_snapshot {
    availability_zone = "value"  # <p>The Availability Zone where you want to create the disk (<code>us-east-2a</code>). Choose
      the same Availability Zone as the Lightsail instance where you want to create the
      disk.</p>
         <p>Use the GetRegions operation to list the Availability Zones where Lightsail is currently
      available.</p>
    size_in_gb = "value"  # <p>The size of the disk in GB (<code>32</code>).</p>
    disk_name = "value"  # <p>The unique Lightsail disk name (<code>my-disk</code>).</p>
}

```

---


### Bucket_metric_data

BucketMetricData resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `metric_data` | Vec<String> | <p>An array of objects that describe the metric data returned.</p> |
| `metric_name` | String | <p>The name of the metric returned.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access bucket_metric_data outputs
bucket_metric_data_id = bucket_metric_data.id
bucket_metric_data_metric_data = bucket_metric_data.metric_data
bucket_metric_data_metric_name = bucket_metric_data.metric_name
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
| `domains` | Vec<String> | <p>An array of key-value pairs containing information about each of the domain entries in the
      user's account.</p> |
| `next_page_token` | String | <p>The token to advance to the next page of results from your request.</p>
         <p>A next page token is not returned if there are no more results to display.</p>
         <p>To get the next page of results, perform another <code>GetDomains</code> request and
      specify the next page token using the <code>pageToken</code> parameter.</p> |


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
domains_domains = domains.domains
domains_next_page_token = domains.next_page_token
```

---


### Disk

Disk resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `availability_zone` | String | ✅ | <p>The Availability Zone where you want to create the disk (<code>us-east-2a</code>). Use the
      same Availability Zone as the Lightsail instance to which you want to attach the
      disk.</p>
         <p>Use the <code>get regions</code> operation to list the Availability Zones where
      Lightsail is currently available.</p> |
| `tags` | Vec<String> |  | <p>The tag keys and optional values to add to the resource during create.</p>
         <p>Use the <code>TagResource</code> action to tag a resource after it's created.</p> |
| `add_ons` | Vec<String> |  | <p>An array of objects that represent the add-ons to enable for the new disk.</p> |
| `size_in_gb` | i64 | ✅ | <p>The size of the disk in GB (<code>32</code>).</p> |
| `disk_name` | String | ✅ | <p>The unique Lightsail disk name (<code>my-disk</code>).</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `disk` | String | <p>An object containing information about the disk.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create disk
disk = provider.lightsail.Disk {
    availability_zone = "value"  # <p>The Availability Zone where you want to create the disk (<code>us-east-2a</code>). Use the
      same Availability Zone as the Lightsail instance to which you want to attach the
      disk.</p>
         <p>Use the <code>get regions</code> operation to list the Availability Zones where
      Lightsail is currently available.</p>
    size_in_gb = "value"  # <p>The size of the disk in GB (<code>32</code>).</p>
    disk_name = "value"  # <p>The unique Lightsail disk name (<code>my-disk</code>).</p>
}

# Access disk outputs
disk_id = disk.id
disk_disk = disk.disk
```

---


### Cloud_formation_stack

CloudFormationStack resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `instances` | Vec<String> | ✅ | <p>An array of parameters that will be used to create the new Amazon EC2 instance. You can only
      pass one instance entry at a time in this array. You will get an invalid parameter error if
      you pass more than one instance entry in this array.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create cloud_formation_stack
cloud_formation_stack = provider.lightsail.Cloud_formation_stack {
    instances = "value"  # <p>An array of parameters that will be used to create the new Amazon EC2 instance. You can only
      pass one instance entry at a time in this array. You will get an invalid parameter error if
      you pass more than one instance entry in this array.</p>
}

```

---


### Relational_database_snapshots

RelationalDatabaseSnapshots resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `relational_database_snapshots` | Vec<String> | <p>An object describing the result of your get relational database snapshots request.</p> |
| `next_page_token` | String | <p>The token to advance to the next page of results from your request.</p>
         <p>A next page token is not returned if there are no more results to display.</p>
         <p>To get the next page of results, perform another
        <code>GetRelationalDatabaseSnapshots</code> request and specify the next page token using
      the <code>pageToken</code> parameter.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access relational_database_snapshots outputs
relational_database_snapshots_id = relational_database_snapshots.id
relational_database_snapshots_relational_database_snapshots = relational_database_snapshots.relational_database_snapshots
relational_database_snapshots_next_page_token = relational_database_snapshots.next_page_token
```

---


### Bucket_bundle

BucketBundle resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `bundle_id` | String | ✅ | <p>The ID of the new bundle to apply to the bucket.</p>
         <p>Use the <a href="https://docs.aws.amazon.com/lightsail/2016-11-28/api-reference/API_GetBucketBundles.html">GetBucketBundles</a> action to get a list of
      bundle IDs that you can specify.</p> |
| `bucket_name` | String | ✅ | <p>The name of the bucket for which to update the bundle.</p> |



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


### Load_balancer_tls_certificate

LoadBalancerTlsCertificate resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `certificate_alternative_names` | Vec<String> |  | <p>An array of strings listing alternative domains and subdomains for your SSL/TLS
      certificate. Lightsail will de-dupe the names for you. You can have a maximum of 9
      alternative names (in addition to the 1 primary domain). We do not support wildcards
        (<code>*.example.com</code>).</p> |
| `tags` | Vec<String> |  | <p>The tag keys and optional values to add to the resource during create.</p>
         <p>Use the <code>TagResource</code> action to tag a resource after it's created.</p> |
| `load_balancer_name` | String | ✅ | <p>The load balancer name where you want to create the SSL/TLS certificate.</p> |
| `certificate_name` | String | ✅ | <p>The SSL/TLS certificate name.</p>
         <p>You can have up to 10 certificates in your account at one time. Each Lightsail load
      balancer can have up to 2 certificates associated with it at one time. There is also an
      overall limit to the number of certificates that can be issue in a 365-day period. For more
      information, see <a href="http://docs.aws.amazon.com/acm/latest/userguide/acm-limits.html">Limits</a>.</p> |
| `certificate_domain_name` | String | ✅ | <p>The domain name (<code>example.com</code>) for your SSL/TLS certificate.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create load_balancer_tls_certificate
load_balancer_tls_certificate = provider.lightsail.Load_balancer_tls_certificate {
    load_balancer_name = "value"  # <p>The load balancer name where you want to create the SSL/TLS certificate.</p>
    certificate_name = "value"  # <p>The SSL/TLS certificate name.</p>
         <p>You can have up to 10 certificates in your account at one time. Each Lightsail load
      balancer can have up to 2 certificates associated with it at one time. There is also an
      overall limit to the number of certificates that can be issue in a 365-day period. For more
      information, see <a href="http://docs.aws.amazon.com/acm/latest/userguide/acm-limits.html">Limits</a>.</p>
    certificate_domain_name = "value"  # <p>The domain name (<code>example.com</code>) for your SSL/TLS certificate.</p>
}

```

---


### Container_service_deployment

ContainerServiceDeployment resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `service_name` | String | ✅ | <p>The name of the container service for which to create the deployment.</p> |
| `containers` | HashMap<String, String> |  | <p>An object that describes the settings of the containers that will be launched on the
      container service.</p> |
| `public_endpoint` | String |  | <p>An object that describes the settings of the public endpoint for the container
      service.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create container_service_deployment
container_service_deployment = provider.lightsail.Container_service_deployment {
    service_name = "value"  # <p>The name of the container service for which to create the deployment.</p>
}

```

---


### Alarm

Alarm resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `alarm_name` | String | ✅ | <p>The name for the alarm. Specify the name of an existing alarm to update, and overwrite the
      previous configuration of the alarm.</p> |
| `metric_name` | String | ✅ | <p>The name of the metric to associate with the alarm.</p>
         <p>You can configure up to two alarms per metric.</p>
         <p>The following metrics are available for each resource type:</p>
         <ul>
            <li>
               <p>
                  <b>Instances</b>: <code>BurstCapacityPercentage</code>,
            <code>BurstCapacityTime</code>, <code>CPUUtilization</code>, <code>NetworkIn</code>,
            <code>NetworkOut</code>, <code>StatusCheckFailed</code>,
            <code>StatusCheckFailed_Instance</code>, and
          <code>StatusCheckFailed_System</code>.</p>
            </li>
            <li>
               <p>
                  <b>Load balancers</b>:
            <code>ClientTLSNegotiationErrorCount</code>, <code>HealthyHostCount</code>,
            <code>UnhealthyHostCount</code>, <code>HTTPCode_LB_4XX_Count</code>,
            <code>HTTPCode_LB_5XX_Count</code>, <code>HTTPCode_Instance_2XX_Count</code>,
            <code>HTTPCode_Instance_3XX_Count</code>, <code>HTTPCode_Instance_4XX_Count</code>,
            <code>HTTPCode_Instance_5XX_Count</code>, <code>InstanceResponseTime</code>,
            <code>RejectedConnectionCount</code>, and <code>RequestCount</code>.</p>
            </li>
            <li>
               <p>
                  <b>Relational databases</b>: <code>CPUUtilization</code>,
            <code>DatabaseConnections</code>, <code>DiskQueueDepth</code>,
            <code>FreeStorageSpace</code>, <code>NetworkReceiveThroughput</code>, and
            <code>NetworkTransmitThroughput</code>.</p>
            </li>
         </ul>
         <p>For more information about these metrics, see <a href="https://docs.aws.amazon.com/lightsail/latest/userguide/amazon-lightsail-resource-health-metrics#available-metrics">Metrics available in Lightsail</a>.</p> |
| `comparison_operator` | String | ✅ | <p>The arithmetic operation to use when comparing the specified statistic to the threshold.
      The specified statistic value is used as the first operand.</p> |
| `notification_triggers` | Vec<String> |  | <p>The alarm states that trigger a notification.</p>
         <p>An alarm has the following possible states:</p>
         <ul>
            <li>
               <p>
                  <code>ALARM</code> - The metric is outside of the defined threshold.</p>
            </li>
            <li>
               <p>
                  <code>INSUFFICIENT_DATA</code> - The alarm has just started, the metric is not
          available, or not enough data is available for the metric to determine the alarm
          state.</p>
            </li>
            <li>
               <p>
                  <code>OK</code> - The metric is within the defined threshold.</p>
            </li>
         </ul>
         <p>When you specify a notification trigger, the <code>ALARM</code> state must be specified.
      The <code>INSUFFICIENT_DATA</code> and <code>OK</code> states can be specified in addition to
      the <code>ALARM</code> state.</p>
         <ul>
            <li>
               <p>If you specify <code>OK</code> as an alarm trigger, a notification is sent when the
          alarm switches from an <code>ALARM</code> or <code>INSUFFICIENT_DATA</code> alarm state to
          an <code>OK</code> state. This can be thought of as an <i>all clear</i>
          alarm notification.</p>
            </li>
            <li>
               <p>If you specify <code>INSUFFICIENT_DATA</code> as the alarm trigger, a notification is
          sent when the alarm switches from an <code>OK</code> or <code>ALARM</code> alarm state to
          an <code>INSUFFICIENT_DATA</code> state.</p>
            </li>
         </ul>
         <p>The notification trigger defaults to <code>ALARM</code> if you don't specify this
      parameter.</p> |
| `monitored_resource_name` | String | ✅ | <p>The name of the Lightsail resource that will be monitored.</p>
         <p>Instances, load balancers, and relational databases are the only Lightsail resources
      that can currently be monitored by alarms.</p> |
| `evaluation_periods` | i64 | ✅ | <p>The number of most recent periods over which data is compared to the specified threshold.
      If you are setting an "M out of N" alarm, this value (<code>evaluationPeriods</code>) is the
      N.</p>
         <p>If you are setting an alarm that requires that a number of consecutive data points be
      breaching to trigger the alarm, this value specifies the rolling period of time in which data
      points are evaluated.</p>
         <p>Each evaluation period is five minutes long. For example, specify an evaluation period of
      24 to evaluate a metric over a rolling period of two hours.</p>
         <p>You can specify a minimum valuation period of 1 (5 minutes), and a maximum evaluation
      period of 288 (24 hours).</p> |
| `contact_protocols` | Vec<String> |  | <p>The contact protocols to use for the alarm, such as <code>Email</code>, <code>SMS</code>
      (text messaging), or both.</p>
         <p>A notification is sent via the specified contact protocol if notifications are enabled for
      the alarm, and when the alarm is triggered.</p>
         <p>A notification is not sent if a contact protocol is not specified, if the specified
      contact protocol is not configured in the Amazon Web Services Region, or if notifications are
      not enabled for the alarm using the <code>notificationEnabled</code> paramater.</p>
         <p>Use the <code>CreateContactMethod</code> action to configure a contact protocol in an
        Amazon Web Services Region.</p> |
| `notification_enabled` | bool |  | <p>Indicates whether the alarm is enabled.</p>
         <p>Notifications are enabled by default if you don't specify this parameter.</p> |
| `threshold` | f64 | ✅ | <p>The value against which the specified statistic is compared.</p> |
| `datapoints_to_alarm` | i64 |  | <p>The number of data points that must be not within the specified threshold to trigger the
      alarm. If you are setting an "M out of N" alarm, this value (<code>datapointsToAlarm</code>)
      is the M.</p> |
| `treat_missing_data` | String |  | <p>Sets how this alarm will handle missing data points.</p>
         <p>An alarm can treat missing data in the following ways:</p>
         <ul>
            <li>
               <p>
                  <code>breaching</code> - Assume the missing data is not within the threshold. Missing
          data counts towards the number of times the metric is not within the threshold.</p>
            </li>
            <li>
               <p>
                  <code>notBreaching</code> - Assume the missing data is within the threshold. Missing
          data does not count towards the number of times the metric is not within the
          threshold.</p>
            </li>
            <li>
               <p>
                  <code>ignore</code> - Ignore the missing data. Maintains the current alarm
          state.</p>
            </li>
            <li>
               <p>
                  <code>missing</code> - Missing data is treated as missing.</p>
            </li>
         </ul>
         <p>If <code>treatMissingData</code> is not specified, the default behavior of
        <code>missing</code> is used.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create alarm
alarm = provider.lightsail.Alarm {
    alarm_name = "value"  # <p>The name for the alarm. Specify the name of an existing alarm to update, and overwrite the
      previous configuration of the alarm.</p>
    metric_name = "value"  # <p>The name of the metric to associate with the alarm.</p>
         <p>You can configure up to two alarms per metric.</p>
         <p>The following metrics are available for each resource type:</p>
         <ul>
            <li>
               <p>
                  <b>Instances</b>: <code>BurstCapacityPercentage</code>,
            <code>BurstCapacityTime</code>, <code>CPUUtilization</code>, <code>NetworkIn</code>,
            <code>NetworkOut</code>, <code>StatusCheckFailed</code>,
            <code>StatusCheckFailed_Instance</code>, and
          <code>StatusCheckFailed_System</code>.</p>
            </li>
            <li>
               <p>
                  <b>Load balancers</b>:
            <code>ClientTLSNegotiationErrorCount</code>, <code>HealthyHostCount</code>,
            <code>UnhealthyHostCount</code>, <code>HTTPCode_LB_4XX_Count</code>,
            <code>HTTPCode_LB_5XX_Count</code>, <code>HTTPCode_Instance_2XX_Count</code>,
            <code>HTTPCode_Instance_3XX_Count</code>, <code>HTTPCode_Instance_4XX_Count</code>,
            <code>HTTPCode_Instance_5XX_Count</code>, <code>InstanceResponseTime</code>,
            <code>RejectedConnectionCount</code>, and <code>RequestCount</code>.</p>
            </li>
            <li>
               <p>
                  <b>Relational databases</b>: <code>CPUUtilization</code>,
            <code>DatabaseConnections</code>, <code>DiskQueueDepth</code>,
            <code>FreeStorageSpace</code>, <code>NetworkReceiveThroughput</code>, and
            <code>NetworkTransmitThroughput</code>.</p>
            </li>
         </ul>
         <p>For more information about these metrics, see <a href="https://docs.aws.amazon.com/lightsail/latest/userguide/amazon-lightsail-resource-health-metrics#available-metrics">Metrics available in Lightsail</a>.</p>
    comparison_operator = "value"  # <p>The arithmetic operation to use when comparing the specified statistic to the threshold.
      The specified statistic value is used as the first operand.</p>
    monitored_resource_name = "value"  # <p>The name of the Lightsail resource that will be monitored.</p>
         <p>Instances, load balancers, and relational databases are the only Lightsail resources
      that can currently be monitored by alarms.</p>
    evaluation_periods = "value"  # <p>The number of most recent periods over which data is compared to the specified threshold.
      If you are setting an "M out of N" alarm, this value (<code>evaluationPeriods</code>) is the
      N.</p>
         <p>If you are setting an alarm that requires that a number of consecutive data points be
      breaching to trigger the alarm, this value specifies the rolling period of time in which data
      points are evaluated.</p>
         <p>Each evaluation period is five minutes long. For example, specify an evaluation period of
      24 to evaluate a metric over a rolling period of two hours.</p>
         <p>You can specify a minimum valuation period of 1 (5 minutes), and a maximum evaluation
      period of 288 (24 hours).</p>
    threshold = "value"  # <p>The value against which the specified statistic is compared.</p>
}

```

---


### Export_snapshot_records

ExportSnapshotRecords resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `export_snapshot_records` | Vec<String> | <p>A list of objects describing the export snapshot records.</p> |
| `next_page_token` | String | <p>The token to advance to the next page of results from your request.</p>
         <p>A next page token is not returned if there are no more results to display.</p>
         <p>To get the next page of results, perform another <code>GetExportSnapshotRecords</code>
      request and specify the next page token using the <code>pageToken</code> parameter.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access export_snapshot_records outputs
export_snapshot_records_id = export_snapshot_records.id
export_snapshot_records_export_snapshot_records = export_snapshot_records.export_snapshot_records
export_snapshot_records_next_page_token = export_snapshot_records.next_page_token
```

---


### Certificates

Certificates resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | <p>If <code>NextPageToken</code> is returned there are more results available. The value of
        <code>NextPageToken</code> is a unique pagination token for each page. Make the call again
      using the returned token to retrieve the next page. Keep all other arguments unchanged.</p> |
| `certificates` | Vec<String> | <p>An object that describes certificates.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access certificates outputs
certificates_id = certificates.id
certificates_next_page_token = certificates.next_page_token
certificates_certificates = certificates.certificates
```

---


### Buckets

Buckets resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | <p>The token to advance to the next page of results from your request.</p>
         <p>A next page token is not returned if there are no more results to display.</p>
         <p>To get the next page of results, perform another <code>GetBuckets</code> request and
      specify the next page token using the <code>pageToken</code> parameter.</p> |
| `buckets` | Vec<String> | <p>An array of objects that describe buckets.</p> |
| `account_level_bpa_sync` | String | <p>An object that describes the synchronization status of the Amazon S3 account-level
      block public access feature for your Lightsail buckets.</p>
         <p>For more information about this feature and how it affects Lightsail buckets, see <a href="https://docs.aws.amazon.com/lightsail/latest/userguide/amazon-lightsail-block-public-access-for-buckets">Block public access for buckets in Amazon Lightsail</a>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access buckets outputs
buckets_id = buckets.id
buckets_next_page_token = buckets.next_page_token
buckets_buckets = buckets.buckets
buckets_account_level_bpa_sync = buckets.account_level_bpa_sync
```

---


### Instance_snapshots

InstanceSnapshots resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | <p>The token to advance to the next page of results from your request.</p>
         <p>A next page token is not returned if there are no more results to display.</p>
         <p>To get the next page of results, perform another <code>GetInstanceSnapshots</code> request
      and specify the next page token using the <code>pageToken</code> parameter.</p> |
| `instance_snapshots` | Vec<String> | <p>An array of key-value pairs containing information about the results of your get instance
      snapshots request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access instance_snapshots outputs
instance_snapshots_id = instance_snapshots.id
instance_snapshots_next_page_token = instance_snapshots.next_page_token
instance_snapshots_instance_snapshots = instance_snapshots.instance_snapshots
```

---


### Operations_for_resource

OperationsForResource resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_count` | String | <p>(Discontinued) Returns the number of pages of results that remain.</p>
         <note>
            <p>In releases prior to June 12, 2017, this parameter returned <code>null</code> by the
        API. It is now discontinued, and the API returns the <code>next page token</code> parameter
        instead.</p>
         </note> |
| `next_page_token` | String | <p>The token to advance to the next page of results from your request.</p>
         <p>A next page token is not returned if there are no more results to display.</p>
         <p>To get the next page of results, perform another <code>GetOperationsForResource</code>
      request and specify the next page token using the <code>pageToken</code> parameter.</p> |
| `operations` | Vec<String> | <p>An array of objects that describe the result of the action, such as the status of the
      request, the timestamp of the request, and the resources affected by the request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access operations_for_resource outputs
operations_for_resource_id = operations_for_resource.id
operations_for_resource_next_page_count = operations_for_resource.next_page_count
operations_for_resource_next_page_token = operations_for_resource.next_page_token
operations_for_resource_operations = operations_for_resource.operations
```

---


### Regions

Regions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `regions` | Vec<String> | <p>An array of key-value pairs containing information about your get regions request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access regions outputs
regions_id = regions.id
regions_regions = regions.regions
```

---


### Distribution_bundles

DistributionBundles resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `bundles` | Vec<String> | <p>An object that describes a distribution bundle.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access distribution_bundles outputs
distribution_bundles_id = distribution_bundles.id
distribution_bundles_bundles = distribution_bundles.bundles
```

---


### Load_balancer_attribute

LoadBalancerAttribute resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `attribute_name` | String | ✅ | <p>The name of the attribute you want to update.</p> |
| `attribute_value` | String | ✅ | <p>The value that you want to specify for the attribute name.</p>
         <p>The following values are supported depending on what you specify for the
        <code>attributeName</code> request parameter:</p>
         <ul>
            <li>
               <p>If you specify <code>HealthCheckPath</code> for the <code>attributeName</code> request
          parameter, then the <code>attributeValue</code> request parameter must be the path to ping
          on the target (for example, <code>/weather/us/wa/seattle</code>).</p>
            </li>
            <li>
               <p>If you specify <code>SessionStickinessEnabled</code> for the
            <code>attributeName</code> request parameter, then the <code>attributeValue</code>
          request parameter must be <code>true</code> to activate session stickiness or
            <code>false</code> to deactivate session stickiness.</p>
            </li>
            <li>
               <p>If you specify <code>SessionStickiness_LB_CookieDurationSeconds</code> for the
            <code>attributeName</code> request parameter, then the <code>attributeValue</code>
          request parameter must be an interger that represents the cookie duration in
          seconds.</p>
            </li>
            <li>
               <p>If you specify <code>HttpsRedirectionEnabled</code> for the <code>attributeName</code>
          request parameter, then the <code>attributeValue</code> request parameter must be
            <code>true</code> to activate HTTP to HTTPS redirection or <code>false</code> to
          deactivate HTTP to HTTPS redirection.</p>
            </li>
            <li>
               <p>If you specify <code>TlsPolicyName</code> for the <code>attributeName</code> request
          parameter, then the <code>attributeValue</code> request parameter must be the name of the
          TLS policy.</p>
               <p>Use the <a href="https://docs.aws.amazon.com/lightsail/2016-11-28/api-reference/API_GetLoadBalancerTlsPolicies.html">GetLoadBalancerTlsPolicies</a> action to get a list of TLS policy names that you
          can specify.</p>
            </li>
         </ul> |
| `load_balancer_name` | String | ✅ | <p>The name of the load balancer that you want to modify
      (<code>my-load-balancer</code>.</p> |



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


### Container_api_metadata

ContainerAPIMetadata resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `metadata` | Vec<HashMap<String, String>> | <p>Metadata about Lightsail containers, such as the current version of the Lightsail
      Control (lightsailctl) plugin.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access container_api_metadata outputs
container_api_metadata_id = container_api_metadata.id
container_api_metadata_metadata = container_api_metadata.metadata
```

---


### Certificate

Certificate resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `certificate_name` | String | ✅ | <p>The name for the certificate.</p> |
| `subject_alternative_names` | Vec<String> |  | <p>An array of strings that specify the alternate domains (<code>example2.com</code>) and
      subdomains (<code>blog.example.com</code>) for the certificate.</p>
         <p>You can specify a maximum of nine alternate domains (in addition to the primary domain
      name).</p>
         <p>Wildcard domain entries (<code>*.example.com</code>) are not supported.</p> |
| `domain_name` | String | ✅ | <p>The domain name (<code>example.com</code>) for the certificate.</p> |
| `tags` | Vec<String> |  | <p>The tag keys and optional values to add to the certificate during create.</p>
         <p>Use the <code>TagResource</code> action to tag a resource after it's created.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create certificate
certificate = provider.lightsail.Certificate {
    certificate_name = "value"  # <p>The name for the certificate.</p>
    domain_name = "value"  # <p>The domain name (<code>example.com</code>) for the certificate.</p>
}

```

---


### Active_names

ActiveNames resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `active_names` | String | <p>The list of active names returned by the get active names request.</p> |
| `next_page_token` | String | <p>The token to advance to the next page of results from your request.</p>
         <p>A next page token is not returned if there are no more results to display.</p>
         <p>To get the next page of results, perform another <code>GetActiveNames</code> request and
      specify the next page token using the <code>pageToken</code> parameter.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access active_names outputs
active_names_id = active_names.id
active_names_active_names = active_names.active_names
active_names_next_page_token = active_names.next_page_token
```

---


### Bucket_bundles

BucketBundles resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `bundles` | Vec<String> | <p>An object that describes bucket bundles.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access bucket_bundles outputs
bucket_bundles_id = bucket_bundles.id
bucket_bundles_bundles = bucket_bundles.bundles
```

---


### Domain_entry

DomainEntry resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `domain_entry` | String | ✅ | <p>An array of key-value pairs containing information about the domain entry request.</p> |
| `domain_name` | String | ✅ | <p>The domain name (<code>example.com</code>) for which you want to create the domain
      entry.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create domain_entry
domain_entry = provider.lightsail.Domain_entry {
    domain_entry = "value"  # <p>An array of key-value pairs containing information about the domain entry request.</p>
    domain_name = "value"  # <p>The domain name (<code>example.com</code>) for which you want to create the domain
      entry.</p>
}

```

---


### Distribution_latest_cache_reset

DistributionLatestCacheReset resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | <p>The timestamp of the last cache reset (<code>1479734909.17</code>) in Unix time
      format.</p> |
| `status` | String | <p>The status of the last cache reset.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access distribution_latest_cache_reset outputs
distribution_latest_cache_reset_id = distribution_latest_cache_reset.id
distribution_latest_cache_reset_create_time = distribution_latest_cache_reset.create_time
distribution_latest_cache_reset_status = distribution_latest_cache_reset.status
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
| `next_page_token` | String | <p>The token to advance to the next page of results from your request.</p>
         <p>A next page token is not returned if there are no more results to display.</p>
         <p>To get the next page of results, perform another <code>GetLoadBalancers</code> request and
      specify the next page token using the <code>pageToken</code> parameter.</p> |
| `load_balancers` | Vec<String> | <p>An array of LoadBalancer objects describing your load balancers.</p> |


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
load_balancers_next_page_token = load_balancers.next_page_token
load_balancers_load_balancers = load_balancers.load_balancers
```

---


### Load_balancer_tls_certificates

LoadBalancerTlsCertificates resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `tls_certificates` | Vec<String> | <p>An array of LoadBalancerTlsCertificate objects describing your SSL/TLS
      certificates.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access load_balancer_tls_certificates outputs
load_balancer_tls_certificates_id = load_balancer_tls_certificates.id
load_balancer_tls_certificates_tls_certificates = load_balancer_tls_certificates.tls_certificates
```

---


### Relational_database_blueprints

RelationalDatabaseBlueprints resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `blueprints` | Vec<String> | <p>An object describing the result of your get relational database blueprints request.</p> |
| `next_page_token` | String | <p>The token to advance to the next page of results from your request.</p>
         <p>A next page token is not returned if there are no more results to display.</p>
         <p>To get the next page of results, perform another
        <code>GetRelationalDatabaseBlueprints</code> request and specify the next page token using
      the <code>pageToken</code> parameter.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access relational_database_blueprints outputs
relational_database_blueprints_id = relational_database_blueprints.id
relational_database_blueprints_blueprints = relational_database_blueprints.blueprints
relational_database_blueprints_next_page_token = relational_database_blueprints.next_page_token
```

---


### Relational_database_bundles

RelationalDatabaseBundles resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | <p>The token to advance to the next page of results from your request.</p>
         <p>A next page token is not returned if there are no more results to display.</p>
         <p>To get the next page of results, perform another <code>GetRelationalDatabaseBundles</code>
      request and specify the next page token using the <code>pageToken</code> parameter.</p> |
| `bundles` | Vec<String> | <p>An object describing the result of your get relational database bundles request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access relational_database_bundles outputs
relational_database_bundles_id = relational_database_bundles.id
relational_database_bundles_next_page_token = relational_database_bundles.next_page_token
relational_database_bundles_bundles = relational_database_bundles.bundles
```

---


### Instance_metadata_options

InstanceMetadataOptions resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `http_protocol_ipv6` | String |  | <p>Enables or disables the IPv6 endpoint for the instance metadata service. This setting
      applies only when the HTTP metadata endpoint is enabled.</p>
         <note>
            <p>This parameter is available only for instances in the Europe (Stockholm) Amazon Web Services Region (<code>eu-north-1</code>).</p>
         </note> |
| `http_tokens` | String |  | <p>The state of token usage for your instance metadata requests. If the parameter is not
      specified in the request, the default state is <code>optional</code>.</p>
         <p>If the state is <code>optional</code>, you can choose whether to retrieve instance
      metadata with a signed token header on your request. If you retrieve the IAM role credentials
      without a token, the version 1.0 role credentials are returned. If you retrieve the IAM role
      credentials by using a valid signed token, the version 2.0 role credentials are
      returned.</p>
         <p>If the state is <code>required</code>, you must send a signed token header with all
      instance metadata retrieval requests. In this state, retrieving the IAM role credential always
      returns the version 2.0 credentials. The version 1.0 credentials are not available.</p> |
| `http_endpoint` | String |  | <p>Enables or disables the HTTP metadata endpoint on your instances. If this parameter is not
      specified, the existing state is maintained.</p>
         <p>If you specify a value of <code>disabled</code>, you cannot access your instance
      metadata.</p> |
| `instance_name` | String | ✅ | <p>The name of the instance for which to update metadata parameters.</p> |
| `http_put_response_hop_limit` | i64 |  | <p>The desired HTTP PUT response hop limit for instance metadata requests. A larger number
      means that the instance metadata requests can travel farther. If no parameter is specified,
      the existing state is maintained.</p> |



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


### Instances_from_snapshot

InstancesFromSnapshot resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `key_pair_name` | String |  | <p>The name for your key pair.</p> |
| `instance_names` | String | ✅ | <p>The names for your new instances.</p> |
| `attached_disk_mapping` | HashMap<String, Vec<String>> |  | <p>An object containing information about one or more disk mappings.</p> |
| `user_data` | String |  | <p>You can create a launch script that configures a server with additional user data. For
      example, <code>apt-get -y update</code>.</p>
         <note>
            <p>Depending on the machine image you choose, the command to get software on your instance
        varies. Amazon Linux and CentOS use <code>yum</code>, Debian and Ubuntu use
          <code>apt-get</code>, and FreeBSD uses <code>pkg</code>. For a complete list, see the
          <a href="https://docs.aws.amazon.com/lightsail/latest/userguide/compare-options-choose-lightsail-instance-image">Amazon Lightsail Developer Guide</a>.</p>
         </note> |
| `tags` | Vec<String> |  | <p>The tag keys and optional values to add to the resource during create.</p>
         <p>Use the <code>TagResource</code> action to tag a resource after it's created.</p> |
| `add_ons` | Vec<String> |  | <p>An array of objects representing the add-ons to enable for the new instance.</p> |
| `source_instance_name` | String |  | <p>The name of the source instance from which the source automatic snapshot was
      created.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>This parameter cannot be defined together with the <code>instance snapshot name</code>
          parameter. The <code>source instance name</code> and <code>instance snapshot name</code>
          parameters are mutually exclusive.</p>
            </li>
            <li>
               <p>Define this parameter only when creating a new instance from an automatic snapshot.
          For more information, see the <a href="https://docs.aws.amazon.com/lightsail/latest/userguide/amazon-lightsail-configuring-automatic-snapshots">Amazon Lightsail Developer Guide</a>.</p>
            </li>
         </ul> |
| `instance_snapshot_name` | String |  | <p>The name of the instance snapshot on which you are basing your new instances. Use the get
      instance snapshots operation to return information about your existing snapshots.</p>
         <p>Constraint:</p>
         <ul>
            <li>
               <p>This parameter cannot be defined together with the <code>source instance name</code>
          parameter. The <code>instance snapshot name</code> and <code>source instance name</code>
          parameters are mutually exclusive.</p>
            </li>
         </ul> |
| `bundle_id` | String | ✅ | <p>The bundle of specification information for your virtual private server (or
        <i>instance</i>), including the pricing plan (<code>micro_x_x</code>).</p> |
| `use_latest_restorable_auto_snapshot` | bool |  | <p>A Boolean value to indicate whether to use the latest available automatic snapshot.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>This parameter cannot be defined together with the <code>restore date</code>
          parameter. The <code>use latest restorable auto snapshot</code> and <code>restore
            date</code> parameters are mutually exclusive.</p>
            </li>
            <li>
               <p>Define this parameter only when creating a new instance from an automatic snapshot.
          For more information, see the <a href="https://docs.aws.amazon.com/lightsail/latest/userguide/amazon-lightsail-configuring-automatic-snapshots">Amazon Lightsail Developer Guide</a>.</p>
            </li>
         </ul> |
| `availability_zone` | String | ✅ | <p>The Availability Zone where you want to create your instances. Use the following
      formatting: <code>us-east-2a</code> (case sensitive). You can get a list of Availability Zones
      by using the <a href="http://docs.aws.amazon.com/lightsail/2016-11-28/api-reference/API_GetRegions.html">get
        regions</a> operation. Be sure to add the <code>include Availability Zones</code>
      parameter to your request.</p> |
| `ip_address_type` | String |  | <p>The IP address type for the instance.</p>
         <p>The possible values are <code>ipv4</code> for IPv4 only, <code>ipv6</code> for IPv6 only,
      and <code>dualstack</code> for IPv4 and IPv6.</p>
         <p>The default value is <code>dualstack</code>.</p> |
| `restore_date` | String |  | <p>The date of the automatic snapshot to use for the new instance. Use the <code>get auto
        snapshots</code> operation to identify the dates of the available automatic
      snapshots.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must be specified in <code>YYYY-MM-DD</code> format.</p>
            </li>
            <li>
               <p>This parameter cannot be defined together with the <code>use latest restorable auto
            snapshot</code> parameter. The <code>restore date</code> and <code>use latest restorable
            auto snapshot</code> parameters are mutually exclusive.</p>
            </li>
            <li>
               <p>Define this parameter only when creating a new instance from an automatic snapshot.
          For more information, see the <a href="https://docs.aws.amazon.com/lightsail/latest/userguide/amazon-lightsail-configuring-automatic-snapshots">Amazon Lightsail Developer Guide</a>.</p>
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

# Create instances_from_snapshot
instances_from_snapshot = provider.lightsail.Instances_from_snapshot {
    instance_names = "value"  # <p>The names for your new instances.</p>
    bundle_id = "value"  # <p>The bundle of specification information for your virtual private server (or
        <i>instance</i>), including the pricing plan (<code>micro_x_x</code>).</p>
    availability_zone = "value"  # <p>The Availability Zone where you want to create your instances. Use the following
      formatting: <code>us-east-2a</code> (case sensitive). You can get a list of Availability Zones
      by using the <a href="http://docs.aws.amazon.com/lightsail/2016-11-28/api-reference/API_GetRegions.html">get
        regions</a> operation. Be sure to add the <code>include Availability Zones</code>
      parameter to your request.</p>
}

```

---


### Static_ip

StaticIp resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `static_ip` | String | <p>An array of key-value pairs containing information about the requested static IP.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access static_ip outputs
static_ip_id = static_ip.id
static_ip_static_ip = static_ip.static_ip
```

---


### Relational_database_master_user_password

RelationalDatabaseMasterUserPassword resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `created_at` | String | <p>The timestamp when the specified version of the master user password was created.</p> |
| `master_user_password` | String | <p>The master user password for the <code>password version</code> specified.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access relational_database_master_user_password outputs
relational_database_master_user_password_id = relational_database_master_user_password.id
relational_database_master_user_password_created_at = relational_database_master_user_password.created_at
relational_database_master_user_password_master_user_password = relational_database_master_user_password.master_user_password
```

---


### Blueprints

Blueprints resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | <p>The token to advance to the next page of results from your request.</p>
         <p>A next page token is not returned if there are no more results to display.</p>
         <p>To get the next page of results, perform another <code>GetBlueprints</code> request and
      specify the next page token using the <code>pageToken</code> parameter.</p> |
| `blueprints` | Vec<String> | <p>An array of key-value pairs that contains information about the available
      blueprints.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access blueprints outputs
blueprints_id = blueprints.id
blueprints_next_page_token = blueprints.next_page_token
blueprints_blueprints = blueprints.blueprints
```

---


### Container_service_deployments

ContainerServiceDeployments resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `deployments` | Vec<String> | <p>An array of objects that describe deployments for a container service.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access container_service_deployments outputs
container_service_deployments_id = container_service_deployments.id
container_service_deployments_deployments = container_service_deployments.deployments
```

---


### Instance_snapshot

InstanceSnapshot resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `instance_snapshot_name` | String | ✅ | <p>The name for your new snapshot.</p> |
| `tags` | Vec<String> |  | <p>The tag keys and optional values to add to the resource during create.</p>
         <p>Use the <code>TagResource</code> action to tag a resource after it's created.</p> |
| `instance_name` | String | ✅ | <p>The Lightsail instance on which to base your snapshot.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `instance_snapshot` | String | <p>An array of key-value pairs containing information about the results of your get instance
      snapshot request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create instance_snapshot
instance_snapshot = provider.lightsail.Instance_snapshot {
    instance_snapshot_name = "value"  # <p>The name for your new snapshot.</p>
    instance_name = "value"  # <p>The Lightsail instance on which to base your snapshot.</p>
}

# Access instance_snapshot outputs
instance_snapshot_id = instance_snapshot.id
instance_snapshot_instance_snapshot = instance_snapshot.instance_snapshot
```

---


### Static_ips

StaticIps resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `static_ips` | Vec<String> | <p>An array of key-value pairs containing information about your get static IPs
      request.</p> |
| `next_page_token` | String | <p>The token to advance to the next page of results from your request.</p>
         <p>A next page token is not returned if there are no more results to display.</p>
         <p>To get the next page of results, perform another <code>GetStaticIps</code> request and
      specify the next page token using the <code>pageToken</code> parameter.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access static_ips outputs
static_ips_id = static_ips.id
static_ips_static_ips = static_ips.static_ips
static_ips_next_page_token = static_ips.next_page_token
```

---


### Relational_database_events

RelationalDatabaseEvents resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `relational_database_events` | Vec<String> | <p>An object describing the result of your get relational database events request.</p> |
| `next_page_token` | String | <p>The token to advance to the next page of results from your request.</p>
         <p>A next page token is not returned if there are no more results to display.</p>
         <p>To get the next page of results, perform another <code>GetRelationalDatabaseEvents</code>
      request and specify the next page token using the <code>pageToken</code> parameter.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access relational_database_events outputs
relational_database_events_id = relational_database_events.id
relational_database_events_relational_database_events = relational_database_events.relational_database_events
relational_database_events_next_page_token = relational_database_events.next_page_token
```

---


### Container_service_powers

ContainerServicePowers resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `powers` | Vec<String> | <p>An array of objects that describe the powers that can be specified for a container
      service.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access container_service_powers outputs
container_service_powers_id = container_service_powers.id
container_service_powers_powers = container_service_powers.powers
```

---


### Disk_snapshot

DiskSnapshot resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `disk_snapshot_name` | String | ✅ | <p>The name of the destination disk snapshot (<code>my-disk-snapshot</code>) based on the
      source disk.</p> |
| `instance_name` | String |  | <p>The unique name of the source instance (<code>Amazon_Linux-512MB-Virginia-1</code>). When
      this is defined, a snapshot of the instance's system volume is created.</p>
         <note>
            <p>This parameter cannot be defined together with the <code>disk name</code> parameter. The
          <code>instance name</code> and <code>disk name</code> parameters are mutually
        exclusive.</p>
         </note> |
| `disk_name` | String |  | <p>The unique name of the source disk (<code>Disk-Virginia-1</code>).</p>
         <note>
            <p>This parameter cannot be defined together with the <code>instance name</code> parameter.
        The <code>disk name</code> and <code>instance name</code> parameters are mutually
        exclusive.</p>
         </note> |
| `tags` | Vec<String> |  | <p>The tag keys and optional values to add to the resource during create.</p>
         <p>Use the <code>TagResource</code> action to tag a resource after it's created.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `disk_snapshot` | String | <p>An object containing information about the disk snapshot.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create disk_snapshot
disk_snapshot = provider.lightsail.Disk_snapshot {
    disk_snapshot_name = "value"  # <p>The name of the destination disk snapshot (<code>my-disk-snapshot</code>) based on the
      source disk.</p>
}

# Access disk_snapshot outputs
disk_snapshot_id = disk_snapshot.id
disk_snapshot_disk_snapshot = disk_snapshot.disk_snapshot
```

---


### Bucket_access_key

BucketAccessKey resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `bucket_name` | String | ✅ | <p>The name of the bucket that the new access key will belong to, and grant access to.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create bucket_access_key
bucket_access_key = provider.lightsail.Bucket_access_key {
    bucket_name = "value"  # <p>The name of the bucket that the new access key will belong to, and grant access to.</p>
}

```

---


### Instances

Instances resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `add_ons` | Vec<String> |  | <p>An array of objects representing the add-ons to enable for the new instance.</p> |
| `bundle_id` | String | ✅ | <p>The bundle of specification information for your virtual private server (or
        <i>instance</i>), including the pricing plan (<code>medium_x_x</code>).</p> |
| `availability_zone` | String | ✅ | <p>The Availability Zone in which to create your instance. Use the following format:
        <code>us-east-2a</code> (case sensitive). You can get a list of Availability Zones by using
      the <a href="http://docs.aws.amazon.com/lightsail/2016-11-28/api-reference/API_GetRegions.html">get
        regions</a> operation. Be sure to add the <code>include Availability Zones</code>
      parameter to your request.</p> |
| `tags` | Vec<String> |  | <p>The tag keys and optional values to add to the resource during create.</p>
         <p>Use the <code>TagResource</code> action to tag a resource after it's created.</p> |
| `custom_image_name` | String |  | <p>(Discontinued) The name for your custom image.</p>
         <note>
            <p>In releases prior to June 12, 2017, this parameter was ignored by the API. It is now
        discontinued.</p>
         </note> |
| `ip_address_type` | String |  | <p>The IP address type for the instance.</p>
         <p>The possible values are <code>ipv4</code> for IPv4 only, <code>ipv6</code> for IPv6 only,
      and <code>dualstack</code> for IPv4 and IPv6.</p>
         <p>The default value is <code>dualstack</code>.</p> |
| `instance_names` | String | ✅ | <p>The names to use for your new Lightsail instances. Separate multiple values using
      quotation marks and commas, for example:
      <code>["MyFirstInstance","MySecondInstance"]</code>
         </p> |
| `blueprint_id` | String | ✅ | <p>The ID for a virtual private server image (<code>app_wordpress_x_x</code> or
        <code>app_lamp_x_x</code>). Use the <code>get blueprints</code> operation to return a list
      of available images (or <i>blueprints</i>).</p>
         <note>
            <p>Use active blueprints when creating new instances. Inactive blueprints are listed to
        support customers with existing instances and are not necessarily available to create new
        instances. Blueprints are marked inactive when they become outdated due to operating system
        updates or new application releases.</p>
         </note> |
| `key_pair_name` | String |  | <p>The name of your key pair.</p> |
| `user_data` | String |  | <p>A launch script you can create that configures a server with additional user data. For
      example, you might want to run <code>apt-get -y update</code>.</p>
         <note>
            <p>Depending on the machine image you choose, the command to get software on your instance
        varies. Amazon Linux and CentOS use <code>yum</code>, Debian and Ubuntu use
          <code>apt-get</code>, and FreeBSD uses <code>pkg</code>. For a complete list, see the
          <a href="https://docs.aws.amazon.com/lightsail/latest/userguide/compare-options-choose-lightsail-instance-image">Amazon Lightsail Developer Guide</a>.</p>
         </note> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `instances` | Vec<String> | <p>An array of key-value pairs containing information about your instances.</p> |
| `next_page_token` | String | <p>The token to advance to the next page of results from your request.</p>
         <p>A next page token is not returned if there are no more results to display.</p>
         <p>To get the next page of results, perform another <code>GetInstances</code> request and
      specify the next page token using the <code>pageToken</code> parameter.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create instances
instances = provider.lightsail.Instances {
    bundle_id = "value"  # <p>The bundle of specification information for your virtual private server (or
        <i>instance</i>), including the pricing plan (<code>medium_x_x</code>).</p>
    availability_zone = "value"  # <p>The Availability Zone in which to create your instance. Use the following format:
        <code>us-east-2a</code> (case sensitive). You can get a list of Availability Zones by using
      the <a href="http://docs.aws.amazon.com/lightsail/2016-11-28/api-reference/API_GetRegions.html">get
        regions</a> operation. Be sure to add the <code>include Availability Zones</code>
      parameter to your request.</p>
    instance_names = "value"  # <p>The names to use for your new Lightsail instances. Separate multiple values using
      quotation marks and commas, for example:
      <code>["MyFirstInstance","MySecondInstance"]</code>
         </p>
    blueprint_id = "value"  # <p>The ID for a virtual private server image (<code>app_wordpress_x_x</code> or
        <code>app_lamp_x_x</code>). Use the <code>get blueprints</code> operation to return a list
      of available images (or <i>blueprints</i>).</p>
         <note>
            <p>Use active blueprints when creating new instances. Inactive blueprints are listed to
        support customers with existing instances and are not necessarily available to create new
        instances. Blueprints are marked inactive when they become outdated due to operating system
        updates or new application releases.</p>
         </note>
}

# Access instances outputs
instances_id = instances.id
instances_instances = instances.instances
instances_next_page_token = instances.next_page_token
```

---


### Load_balancer_tls_policies

LoadBalancerTlsPolicies resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `tls_policies` | Vec<String> | <p>An array of objects that describe the TLS security policies that are available.</p> |
| `next_page_token` | String | <p>The token to advance to the next page of results from your request.</p>
         <p>A next page token is not returned if there are no more results to display.</p>
         <p>To get the next page of results, perform another <code>GetLoadBalancerTlsPolicies</code>
      request and specify the next page token using the <code>pageToken</code> parameter.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access load_balancer_tls_policies outputs
load_balancer_tls_policies_id = load_balancer_tls_policies.id
load_balancer_tls_policies_tls_policies = load_balancer_tls_policies.tls_policies
load_balancer_tls_policies_next_page_token = load_balancer_tls_policies.next_page_token
```

---


### Instance_metric_data

InstanceMetricData resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `metric_data` | Vec<String> | <p>An array of objects that describe the metric data returned.</p> |
| `metric_name` | String | <p>The name of the metric returned.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access instance_metric_data outputs
instance_metric_data_id = instance_metric_data.id
instance_metric_data_metric_data = instance_metric_data.metric_data
instance_metric_data_metric_name = instance_metric_data.metric_name
```

---


### Relational_database_metric_data

RelationalDatabaseMetricData resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `metric_data` | Vec<String> | <p>An array of objects that describe the metric data returned.</p> |
| `metric_name` | String | <p>The name of the metric returned.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access relational_database_metric_data outputs
relational_database_metric_data_id = relational_database_metric_data.id
relational_database_metric_data_metric_data = relational_database_metric_data.metric_data
relational_database_metric_data_metric_name = relational_database_metric_data.metric_name
```

---


### Domain

Domain resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `domain_name` | String | ✅ | <p>The domain name to manage (<code>example.com</code>).</p> |
| `tags` | Vec<String> |  | <p>The tag keys and optional values to add to the resource during create.</p>
         <p>Use the <code>TagResource</code> action to tag a resource after it's created.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `domain` | String | <p>An array of key-value pairs containing information about your get domain request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create domain
domain = provider.lightsail.Domain {
    domain_name = "value"  # <p>The domain name to manage (<code>example.com</code>).</p>
}

# Access domain outputs
domain_id = domain.id
domain_domain = domain.domain
```

---


### Relational_database

RelationalDatabase resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `master_username` | String | ✅ | <p>The name for the master user.</p>
         <p>
            <b>MySQL</b>
         </p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Required for MySQL.</p>
            </li>
            <li>
               <p>Must be 1 to 16 letters or numbers. Can contain underscores.</p>
            </li>
            <li>
               <p>First character must be a letter.</p>
            </li>
            <li>
               <p>Can't be a reserved word for the chosen database engine.</p>
               <p>For more information about reserved words in MySQL 5.6 or 5.7, see the Keywords and
          Reserved Words articles for <a href="https://dev.mysql.com/doc/refman/5.6/en/keywords.html">MySQL 5.6</a>, <a href="https://dev.mysql.com/doc/refman/5.7/en/keywords.html">MySQL 5.7</a>, or <a href="https://dev.mysql.com/doc/refman/8.0/en/keywords.html">MySQL 8.0</a>.</p>
            </li>
         </ul>
         <p>
            <b>PostgreSQL</b>
         </p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Required for PostgreSQL.</p>
            </li>
            <li>
               <p>Must be 1 to 63 letters or numbers. Can contain underscores.</p>
            </li>
            <li>
               <p>First character must be a letter.</p>
            </li>
            <li>
               <p>Can't be a reserved word for the chosen database engine.</p>
               <p>For more information about reserved words in MySQL 5.6 or 5.7, see the Keywords and
          Reserved Words articles for <a href="https://www.postgresql.org/docs/9.6/sql-keywords-appendix.html">PostgreSQL
            9.6</a>, <a href="https://www.postgresql.org/docs/10/sql-keywords-appendix.html">PostgreSQL 10</a>, <a href="https://www.postgresql.org/docs/11/sql-keywords-appendix.html">PostgreSQL
            11</a>, and <a href="https://www.postgresql.org/docs/12/sql-keywords-appendix.html">PostgreSQL
            12</a>.</p>
            </li>
         </ul> |
| `preferred_maintenance_window` | String |  | <p>The weekly time range during which system maintenance can occur on your new
      database.</p>
         <p>The default is a 30-minute window selected at random from an 8-hour block of time for each
      AWS Region, occurring on a random day of the week.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must be in the <code>ddd:hh24:mi-ddd:hh24:mi</code> format.</p>
            </li>
            <li>
               <p>Valid days: Mon, Tue, Wed, Thu, Fri, Sat, Sun.</p>
            </li>
            <li>
               <p>Must be at least 30 minutes.</p>
            </li>
            <li>
               <p>Specified in Coordinated Universal Time (UTC).</p>
            </li>
            <li>
               <p>Example: <code>Tue:17:00-Tue:17:30</code>
               </p>
            </li>
         </ul> |
| `relational_database_blueprint_id` | String | ✅ | <p>The blueprint ID for your new database. A blueprint describes the major engine version of
      a database.</p>
         <p>You can get a list of database blueprints IDs by using the <code>get relational database
        blueprints</code> operation.</p> |
| `relational_database_name` | String | ✅ | <p>The name to use for your new Lightsail database resource.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must contain from 2 to 255 alphanumeric characters, or hyphens.</p>
            </li>
            <li>
               <p>The first and last character must be a letter or number.</p>
            </li>
         </ul> |
| `publicly_accessible` | bool |  | <p>Specifies the accessibility options for your new database. A value of <code>true</code>
      specifies a database that is available to resources outside of your Lightsail account. A
      value of <code>false</code> specifies a database that is available only to your Lightsail
      resources in the same region as your database.</p> |
| `relational_database_bundle_id` | String | ✅ | <p>The bundle ID for your new database. A bundle describes the performance specifications for
      your database.</p>
         <p>You can get a list of database bundle IDs by using the <code>get relational database
        bundles</code> operation.</p> |
| `tags` | Vec<String> |  | <p>The tag keys and optional values to add to the resource during create.</p>
         <p>Use the <code>TagResource</code> action to tag a resource after it's created.</p> |
| `availability_zone` | String |  | <p>The Availability Zone in which to create your new database. Use the
        <code>us-east-2a</code> case-sensitive format.</p>
         <p>You can get a list of Availability Zones by using the <code>get regions</code> operation.
      Be sure to add the <code>include relational database Availability Zones</code> parameter to
      your request.</p> |
| `master_database_name` | String | ✅ | <p>The meaning of this parameter differs according to the database engine you use.</p>
         <p>
            <b>MySQL</b>
         </p>
         <p>The name of the database to create when the Lightsail database resource is created. If
      this parameter isn't specified, no database is created in the database resource.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must contain 1 to 64 letters or numbers.</p>
            </li>
            <li>
               <p>Must begin with a letter. Subsequent characters can be letters, underscores, or digits
          (0- 9).</p>
            </li>
            <li>
               <p>Can't be a word reserved by the specified database engine.</p>
               <p>For more information about reserved words in MySQL, see the Keywords and Reserved
          Words articles for <a href="https://dev.mysql.com/doc/refman/5.6/en/keywords.html">MySQL 5.6</a>, <a href="https://dev.mysql.com/doc/refman/5.7/en/keywords.html">MySQL 5.7</a>, and <a href="https://dev.mysql.com/doc/refman/8.0/en/keywords.html">MySQL 8.0</a>.</p>
            </li>
         </ul>
         <p>
            <b>PostgreSQL</b>
         </p>
         <p>The name of the database to create when the Lightsail database resource is created. If
      this parameter isn't specified, a database named <code>postgres</code> is created in the
      database resource.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must contain 1 to 63 letters or numbers.</p>
            </li>
            <li>
               <p>Must begin with a letter. Subsequent characters can be letters, underscores, or digits
          (0- 9).</p>
            </li>
            <li>
               <p>Can't be a word reserved by the specified database engine.</p>
               <p>For more information about reserved words in PostgreSQL, see the SQL Key Words
          articles for <a href="https://www.postgresql.org/docs/9.6/sql-keywords-appendix.html">PostgreSQL 9.6</a>, <a href="https://www.postgresql.org/docs/10/sql-keywords-appendix.html">PostgreSQL
            10</a>, <a href="https://www.postgresql.org/docs/11/sql-keywords-appendix.html">PostgreSQL 11</a>, and <a href="https://www.postgresql.org/docs/12/sql-keywords-appendix.html">PostgreSQL
            12</a>.</p>
            </li>
         </ul> |
| `master_user_password` | String |  | <p>The password for the master user. The password can include any printable ASCII character
      except "/", """, or "@". It cannot contain spaces.</p>
         <p>
            <b>MySQL</b>
         </p>
         <p>Constraints: Must contain from 8 to 41 characters.</p>
         <p>
            <b>PostgreSQL</b>
         </p>
         <p>Constraints: Must contain from 8 to 128 characters.</p> |
| `preferred_backup_window` | String |  | <p>The daily time range during which automated backups are created for your new database if
      automated backups are enabled.</p>
         <p>The default is a 30-minute window selected at random from an 8-hour block of time for each
      AWS Region. For more information about the preferred backup window time blocks for each
      region, see the <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_WorkingWithAutomatedBackups.html#USER_WorkingWithAutomatedBackups.BackupWindow">Working With Backups</a> guide in the Amazon Relational Database Service documentation.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must be in the <code>hh24:mi-hh24:mi</code> format.</p>
               <p>Example: <code>16:00-16:30</code>
               </p>
            </li>
            <li>
               <p>Specified in Coordinated Universal Time (UTC).</p>
            </li>
            <li>
               <p>Must not conflict with the preferred maintenance window.</p>
            </li>
            <li>
               <p>Must be at least 30 minutes.</p>
            </li>
         </ul> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `relational_database` | String | <p>An object describing the specified database.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create relational_database
relational_database = provider.lightsail.Relational_database {
    master_username = "value"  # <p>The name for the master user.</p>
         <p>
            <b>MySQL</b>
         </p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Required for MySQL.</p>
            </li>
            <li>
               <p>Must be 1 to 16 letters or numbers. Can contain underscores.</p>
            </li>
            <li>
               <p>First character must be a letter.</p>
            </li>
            <li>
               <p>Can't be a reserved word for the chosen database engine.</p>
               <p>For more information about reserved words in MySQL 5.6 or 5.7, see the Keywords and
          Reserved Words articles for <a href="https://dev.mysql.com/doc/refman/5.6/en/keywords.html">MySQL 5.6</a>, <a href="https://dev.mysql.com/doc/refman/5.7/en/keywords.html">MySQL 5.7</a>, or <a href="https://dev.mysql.com/doc/refman/8.0/en/keywords.html">MySQL 8.0</a>.</p>
            </li>
         </ul>
         <p>
            <b>PostgreSQL</b>
         </p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Required for PostgreSQL.</p>
            </li>
            <li>
               <p>Must be 1 to 63 letters or numbers. Can contain underscores.</p>
            </li>
            <li>
               <p>First character must be a letter.</p>
            </li>
            <li>
               <p>Can't be a reserved word for the chosen database engine.</p>
               <p>For more information about reserved words in MySQL 5.6 or 5.7, see the Keywords and
          Reserved Words articles for <a href="https://www.postgresql.org/docs/9.6/sql-keywords-appendix.html">PostgreSQL
            9.6</a>, <a href="https://www.postgresql.org/docs/10/sql-keywords-appendix.html">PostgreSQL 10</a>, <a href="https://www.postgresql.org/docs/11/sql-keywords-appendix.html">PostgreSQL
            11</a>, and <a href="https://www.postgresql.org/docs/12/sql-keywords-appendix.html">PostgreSQL
            12</a>.</p>
            </li>
         </ul>
    relational_database_blueprint_id = "value"  # <p>The blueprint ID for your new database. A blueprint describes the major engine version of
      a database.</p>
         <p>You can get a list of database blueprints IDs by using the <code>get relational database
        blueprints</code> operation.</p>
    relational_database_name = "value"  # <p>The name to use for your new Lightsail database resource.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must contain from 2 to 255 alphanumeric characters, or hyphens.</p>
            </li>
            <li>
               <p>The first and last character must be a letter or number.</p>
            </li>
         </ul>
    relational_database_bundle_id = "value"  # <p>The bundle ID for your new database. A bundle describes the performance specifications for
      your database.</p>
         <p>You can get a list of database bundle IDs by using the <code>get relational database
        bundles</code> operation.</p>
    master_database_name = "value"  # <p>The meaning of this parameter differs according to the database engine you use.</p>
         <p>
            <b>MySQL</b>
         </p>
         <p>The name of the database to create when the Lightsail database resource is created. If
      this parameter isn't specified, no database is created in the database resource.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must contain 1 to 64 letters or numbers.</p>
            </li>
            <li>
               <p>Must begin with a letter. Subsequent characters can be letters, underscores, or digits
          (0- 9).</p>
            </li>
            <li>
               <p>Can't be a word reserved by the specified database engine.</p>
               <p>For more information about reserved words in MySQL, see the Keywords and Reserved
          Words articles for <a href="https://dev.mysql.com/doc/refman/5.6/en/keywords.html">MySQL 5.6</a>, <a href="https://dev.mysql.com/doc/refman/5.7/en/keywords.html">MySQL 5.7</a>, and <a href="https://dev.mysql.com/doc/refman/8.0/en/keywords.html">MySQL 8.0</a>.</p>
            </li>
         </ul>
         <p>
            <b>PostgreSQL</b>
         </p>
         <p>The name of the database to create when the Lightsail database resource is created. If
      this parameter isn't specified, a database named <code>postgres</code> is created in the
      database resource.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must contain 1 to 63 letters or numbers.</p>
            </li>
            <li>
               <p>Must begin with a letter. Subsequent characters can be letters, underscores, or digits
          (0- 9).</p>
            </li>
            <li>
               <p>Can't be a word reserved by the specified database engine.</p>
               <p>For more information about reserved words in PostgreSQL, see the SQL Key Words
          articles for <a href="https://www.postgresql.org/docs/9.6/sql-keywords-appendix.html">PostgreSQL 9.6</a>, <a href="https://www.postgresql.org/docs/10/sql-keywords-appendix.html">PostgreSQL
            10</a>, <a href="https://www.postgresql.org/docs/11/sql-keywords-appendix.html">PostgreSQL 11</a>, and <a href="https://www.postgresql.org/docs/12/sql-keywords-appendix.html">PostgreSQL
            12</a>.</p>
            </li>
         </ul>
}

# Access relational_database outputs
relational_database_id = relational_database.id
relational_database_relational_database = relational_database.relational_database
```

---


### Relational_databases

RelationalDatabases resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `relational_databases` | Vec<String> | <p>An object describing the result of your get relational databases request.</p> |
| `next_page_token` | String | <p>The token to advance to the next page of results from your request.</p>
         <p>A next page token is not returned if there are no more results to display.</p>
         <p>To get the next page of results, perform another <code>GetRelationalDatabases</code>
      request and specify the next page token using the <code>pageToken</code> parameter.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access relational_databases outputs
relational_databases_id = relational_databases.id
relational_databases_relational_databases = relational_databases.relational_databases
relational_databases_next_page_token = relational_databases.next_page_token
```

---


### Disks

Disks resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `disks` | Vec<String> | <p>An array of objects containing information about all block storage disks.</p> |
| `next_page_token` | String | <p>The token to advance to the next page of results from your request.</p>
         <p>A next page token is not returned if there are no more results to display.</p>
         <p>To get the next page of results, perform another <code>GetDisks</code> request and specify
      the next page token using the <code>pageToken</code> parameter.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access disks outputs
disks_id = disks.id
disks_disks = disks.disks
disks_next_page_token = disks.next_page_token
```

---


### Distributions

Distributions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | <p>The token to advance to the next page of results from your request.</p>
         <p>A next page token is not returned if there are no more results to display.</p>
         <p>To get the next page of results, perform another <code>GetDistributions</code> request and
      specify the next page token using the <code>pageToken</code> parameter.</p> |
| `distributions` | Vec<String> | <p>An array of objects that describe your distributions.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access distributions outputs
distributions_id = distributions.id
distributions_next_page_token = distributions.next_page_token
distributions_distributions = distributions.distributions
```

---


### Relational_database_snapshot

RelationalDatabaseSnapshot resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>The tag keys and optional values to add to the resource during create.</p>
         <p>Use the <code>TagResource</code> action to tag a resource after it's created.</p> |
| `relational_database_name` | String | ✅ | <p>The name of the database on which to base your new snapshot.</p> |
| `relational_database_snapshot_name` | String | ✅ | <p>The name for your new database snapshot.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must contain from 2 to 255 alphanumeric characters, or hyphens.</p>
            </li>
            <li>
               <p>The first and last character must be a letter or number.</p>
            </li>
         </ul> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `relational_database_snapshot` | String | <p>An object describing the specified database snapshot.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create relational_database_snapshot
relational_database_snapshot = provider.lightsail.Relational_database_snapshot {
    relational_database_name = "value"  # <p>The name of the database on which to base your new snapshot.</p>
    relational_database_snapshot_name = "value"  # <p>The name for your new database snapshot.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Must contain from 2 to 255 alphanumeric characters, or hyphens.</p>
            </li>
            <li>
               <p>The first and last character must be a letter or number.</p>
            </li>
         </ul>
}

# Access relational_database_snapshot outputs
relational_database_snapshot_id = relational_database_snapshot.id
relational_database_snapshot_relational_database_snapshot = relational_database_snapshot.relational_database_snapshot
```

---


### Relational_database_parameters

RelationalDatabaseParameters resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `parameters` | Vec<String> | ✅ | <p>The database parameters to update.</p> |
| `relational_database_name` | String | ✅ | <p>The name of your database for which to update parameters.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | <p>The token to advance to the next page of results from your request.</p>
         <p>A next page token is not returned if there are no more results to display.</p>
         <p>To get the next page of results, perform another
        <code>GetRelationalDatabaseParameters</code> request and specify the next page token using
      the <code>pageToken</code> parameter.</p> |
| `parameters` | Vec<String> | <p>An object describing the result of your get relational database parameters request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access relational_database_parameters outputs
relational_database_parameters_id = relational_database_parameters.id
relational_database_parameters_next_page_token = relational_database_parameters.next_page_token
relational_database_parameters_parameters = relational_database_parameters.parameters
```

---


### Instance_state

InstanceState resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | <p>The state of the instance.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access instance_state outputs
instance_state_id = instance_state.id
instance_state_state = instance_state.state
```

---


### Operation

Operation resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `operation` | String | <p>An array of objects that describe the result of the action, such as the status of the
      request, the timestamp of the request, and the resources affected by the request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access operation outputs
operation_id = operation.id
operation_operation = operation.operation
```

---


### Relational_database_log_events

RelationalDatabaseLogEvents resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `resource_log_events` | Vec<String> | <p>An object describing the result of your get relational database log events request.</p> |
| `next_backward_token` | String | <p>A token used for advancing to the previous page of results from your get relational
      database log events request.</p> |
| `next_forward_token` | String | <p>A token used for advancing to the next page of results from your get relational database
      log events request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access relational_database_log_events outputs
relational_database_log_events_id = relational_database_log_events.id
relational_database_log_events_resource_log_events = relational_database_log_events.resource_log_events
relational_database_log_events_next_backward_token = relational_database_log_events.next_backward_token
relational_database_log_events_next_forward_token = relational_database_log_events.next_forward_token
```

---


### Distribution

Distribution resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `certificate_name` | String |  | <p>The name of the SSL/TLS certificate that you want to attach to the distribution.</p>
         <p>Use the <a href="https://docs.aws.amazon.com/lightsail/2016-11-28/api-reference/API_GetCertificates.html">GetCertificates</a>
      action to get a list of certificate names that you can specify.</p> |
| `tags` | Vec<String> |  | <p>The tag keys and optional values to add to the distribution during create.</p>
         <p>Use the <code>TagResource</code> action to tag a resource after it's created.</p> |
| `default_cache_behavior` | String | ✅ | <p>An object that describes the default cache behavior for the distribution.</p> |
| `cache_behaviors` | Vec<String> |  | <p>An array of objects that describe the per-path cache behavior for the distribution.</p> |
| `bundle_id` | String | ✅ | <p>The bundle ID to use for the distribution.</p>
         <p>A distribution bundle describes the specifications of your distribution, such as the
      monthly cost and monthly network transfer quota.</p>
         <p>Use the <code>GetDistributionBundles</code> action to get a list of distribution bundle
      IDs that you can specify.</p> |
| `origin` | String | ✅ | <p>An object that describes the origin resource for the distribution, such as a Lightsail
      instance, bucket, or load balancer.</p>
         <p>The distribution pulls, caches, and serves content from the origin.</p> |
| `ip_address_type` | String |  | <p>The IP address type for the distribution.</p>
         <p>The possible values are <code>ipv4</code> for IPv4 only, and <code>dualstack</code> for
      IPv4 and IPv6.</p>
         <p>The default value is <code>dualstack</code>.</p> |
| `distribution_name` | String | ✅ | <p>The name for the distribution.</p> |
| `cache_behavior_settings` | String |  | <p>An object that describes the cache behavior settings for the distribution.</p> |
| `viewer_minimum_tls_protocol_version` | String |  | <p>The minimum TLS protocol version for the SSL/TLS certificate.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create distribution
distribution = provider.lightsail.Distribution {
    default_cache_behavior = "value"  # <p>An object that describes the default cache behavior for the distribution.</p>
    bundle_id = "value"  # <p>The bundle ID to use for the distribution.</p>
         <p>A distribution bundle describes the specifications of your distribution, such as the
      monthly cost and monthly network transfer quota.</p>
         <p>Use the <code>GetDistributionBundles</code> action to get a list of distribution bundle
      IDs that you can specify.</p>
    origin = "value"  # <p>An object that describes the origin resource for the distribution, such as a Lightsail
      instance, bucket, or load balancer.</p>
         <p>The distribution pulls, caches, and serves content from the origin.</p>
    distribution_name = "value"  # <p>The name for the distribution.</p>
}

```

---


### Known_host_keys

KnownHostKeys resource

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


### Setup_history

SetupHistory resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `setup_history` | Vec<String> | <p>The historical information that's returned.</p> |
| `next_page_token` | String | <p>The token to advance to the next page of results from your request.</p>
         <p>A next page token is not returned if there are no more results to display.</p>
         <p>To get the next page of results, perform another <code>GetSetupHistory</code> request and
      specify the next page token using the pageToken parameter.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access setup_history outputs
setup_history_id = setup_history.id
setup_history_setup_history = setup_history.setup_history
setup_history_next_page_token = setup_history.next_page_token
```

---


### Auto_snapshot

AutoSnapshot resource

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


### Alarms

Alarms resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `alarms` | Vec<String> | <p>An array of objects that describe the alarms.</p> |
| `next_page_token` | String | <p>The token to advance to the next page of results from your request.</p>
         <p>A next page token is not returned if there are no more results to display.</p>
         <p>To get the next page of results, perform another <code>GetAlarms</code> request and
      specify the next page token using the <code>pageToken</code> parameter.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access alarms outputs
alarms_id = alarms.id
alarms_alarms = alarms.alarms
alarms_next_page_token = alarms.next_page_token
```

---


### Operations

Operations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `operations` | Vec<String> | <p>An array of objects that describe the result of the action, such as the status of the
      request, the timestamp of the request, and the resources affected by the request.</p> |
| `next_page_token` | String | <p>The token to advance to the next page of results from your request.</p>
         <p>A next page token is not returned if there are no more results to display.</p>
         <p>To get the next page of results, perform another <code>GetOperations</code> request and
      specify the next page token using the <code>pageToken</code> parameter.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access operations outputs
operations_id = operations.id
operations_operations = operations.operations
operations_next_page_token = operations.next_page_token
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple distribution_bundle resources
distribution_bundle_0 = provider.lightsail.Distribution_bundle {
}
distribution_bundle_1 = provider.lightsail.Distribution_bundle {
}
distribution_bundle_2 = provider.lightsail.Distribution_bundle {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    distribution_bundle = provider.lightsail.Distribution_bundle {
    }
```

---

## Related Documentation

- [AWS Lightsail Documentation](https://docs.aws.amazon.com/lightsail/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
