# Snowball Service



**Resources**: 11

---

## Overview

The snowball service provides access to 11 resource types:

- [Snowball_usage](#snowball_usage) [R]
- [Software_updates](#software_updates) [R]
- [Addresses](#addresses) [R]
- [Job_shipment_state](#job_shipment_state) [U]
- [Job](#job) [CRU]
- [Cluster](#cluster) [CRU]
- [Long_term_pricing](#long_term_pricing) [CU]
- [Job_unlock_code](#job_unlock_code) [R]
- [Address](#address) [CR]
- [Job_manifest](#job_manifest) [R]
- [Return_shipping_label](#return_shipping_label) [CR]

---

## Resources


### Snowball_usage

SnowballUsage resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `snowball_limit` | i64 | <p>The service limit for number of Snow devices this account can have at once. The default
      service limit is 1 (one).</p> |
| `snowballs_in_use` | i64 | <p>The number of Snow devices that this account is currently using.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access snowball_usage outputs
snowball_usage_id = snowball_usage.id
snowball_usage_snowball_limit = snowball_usage.snowball_limit
snowball_usage_snowballs_in_use = snowball_usage.snowballs_in_use
```

---


### Software_updates

SoftwareUpdates resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `updates_uri` | String | <p>The Amazon S3 presigned URL for the update file associated with the specified
        <code>JobId</code> value. The software update will be available for 2 days after this
      request is made. To access an update after the 2 days have passed, you'll have to make another
      call to <code>GetSoftwareUpdates</code>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access software_updates outputs
software_updates_id = software_updates.id
software_updates_updates_uri = software_updates.updates_uri
```

---


### Addresses

Addresses resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `addresses` | Vec<String> | <p>The Snow device shipping addresses that were created for this account.</p> |
| `next_token` | String | <p>HTTP requests are stateless. If you use the automatically generated
        <code>NextToken</code> value in your next <code>DescribeAddresses</code> call, your list of
      returned addresses will start from this point in the array.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access addresses outputs
addresses_id = addresses.id
addresses_addresses = addresses.addresses
addresses_next_token = addresses.next_token
```

---


### Job_shipment_state

JobShipmentState resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `shipment_state` | String | ✅ | <p>The state of a device when it is being shipped. </p>
         <p>Set to <code>RECEIVED</code> when the device arrives at your location.</p>
         <p>Set to <code>RETURNED</code> when you have returned the device to Amazon Web Services.</p> |
| `job_id` | String | ✅ | <p>The job ID of the job whose shipment date you want to update, for example
        <code>JID123e4567-e89b-12d3-a456-426655440000</code>.</p> |



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


### Job

Job resource

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tax_documents` | String |  | <p>The tax documents required in your Amazon Web Services Region.</p> |
| `forwarding_address_id` | String |  | <p>The forwarding address ID for a job. This field is not supported in most
      Regions.</p> |
| `remote_management` | String |  | <p>Allows you to securely operate and manage Snowcone devices remotely from outside of your
      internal network. When set to <code>INSTALLED_AUTOSTART</code>, remote management will
      automatically be available when the device arrives at your location. Otherwise, you need to
      use the Snowball Edge client to manage the device. When set to <code>NOT_INSTALLED</code>, remote management will not be available on the device. </p> |
| `long_term_pricing_id` | i64 |  | <p>The ID of the long-term pricing type for the device.</p> |
| `kms_key_arn` | String |  | <p>The <code>KmsKeyARN</code> that you want to associate with this job.
        <code>KmsKeyARN</code>s are created using the <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_CreateKey.html">CreateKey</a>
      Key Management Service (KMS) API action.</p> |
| `pickup_details` | String |  | <p>Information identifying the person picking up the device.</p> |
| `snowball_capacity_preference` | String |  | <p>If your job is being created in one of the US regions, you have the option of
      specifying what size Snow device you'd like for this job. In all other regions, Snowballs come
      with 80 TB in storage capacity.</p>
         <p>For more information, see
      "https://docs.aws.amazon.com/snowball/latest/snowcone-guide/snow-device-types.html" (Snow
      Family Devices and Capacity) in the <i>Snowcone User Guide</i> or
      "https://docs.aws.amazon.com/snowball/latest/developer-guide/snow-device-types.html" (Snow
      Family Devices and Capacity) in the <i>Snowcone User Guide</i>.</p> |
| `resources` | String |  | <p>Defines the Amazon S3 buckets associated with this job.</p>
         <p>With <code>IMPORT</code> jobs, you specify the bucket or buckets that your transferred
      data will be imported into.</p>
         <p>With <code>EXPORT</code> jobs, you specify the bucket or buckets that your transferred
      data will be exported from. Optionally, you can also specify a <code>KeyRange</code> value. If
      you choose to export a range, you define the length of the range by providing either an
      inclusive <code>BeginMarker</code> value, an inclusive <code>EndMarker</code> value, or both.
      Ranges are UTF-8 binary sorted.</p> |
| `description` | String |  | <p>Defines an optional description of this specific job, for example <code>Important
        Photos 2016-08-11</code>.</p> |
| `notification` | String |  | <p>Defines the Amazon Simple Notification Service (Amazon SNS) notification settings for
      this job.</p> |
| `snowball_type` | String |  | <p>The type of Snow Family devices to use for this job.
      </p>
         <note>
            <p>For cluster jobs, Amazon Web Services Snow Family currently supports only the
          <code>EDGE</code> device type.</p>
         </note>
         <p>The type of Amazon Web Services Snow device to use for this job. Currently, the only
      supported device type for cluster jobs is <code>EDGE</code>.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/snowball/latest/developer-guide/device-differences.html">Snowball Edge Device
        Options</a> in the Snowball Edge Developer Guide.</p>
         <p>For more information, see
      "https://docs.aws.amazon.com/snowball/latest/snowcone-guide/snow-device-types.html" (Snow
      Family Devices and Capacity) in the <i>Snowcone User Guide</i> or
      "https://docs.aws.amazon.com/snowball/latest/developer-guide/snow-device-types.html" (Snow
      Family Devices and Capacity) in the <i>Snowcone User Guide</i>.</p> |
| `on_device_service_configuration` | String |  | <p>Specifies the service or services on the Snow Family device that your transferred data
      will be exported from or imported into. Amazon Web Services Snow Family supports Amazon S3 and NFS (Network File
      System) and the Amazon Web Services Storage Gateway service Tape Gateway type.</p> |
| `shipping_option` | String |  | <p>The shipping speed for this job. This speed doesn't dictate how soon you'll get the
      Snow device, rather it represents how quickly the Snow device moves to its destination while
      in transit. Regional shipping speeds are as follows:</p>
         <ul>
            <li>
               <p>In Australia, you have access to express shipping. Typically, Snow devices shipped
          express are delivered in about a day.</p>
            </li>
            <li>
               <p>In the European Union (EU), you have access to express shipping. Typically, Snow
          devices shipped express are delivered in about a day. In addition, most countries in the
          EU have access to standard shipping, which typically takes less than a week, one
          way.</p>
            </li>
            <li>
               <p>In India, Snow devices are delivered in one to seven days.</p>
            </li>
            <li>
               <p>In the US, you have access to one-day shipping and two-day shipping.</p>
            </li>
         </ul> |
| `job_type` | String |  | <p>Defines the type of job that you're creating.
      </p> |
| `impact_level` | String |  | <p>The highest impact level of data that will be stored or processed on the device, provided at job creation.</p> |
| `address_id` | String |  | <p>The ID for the address that you want the Snow device shipped to.</p> |
| `cluster_id` | String |  | <p>The ID of a cluster. If you're creating a job for a node in a cluster, you need to
      provide only this <code>clusterId</code> value. The other job attributes are inherited from
      the cluster.</p> |
| `role_arn` | String |  | <p>The <code>RoleARN</code> that you want to associate with this job.
      <code>RoleArn</code>s are created using the <a href="https://docs.aws.amazon.com/IAM/latest/APIReference/API_CreateRole.html">CreateRole</a>
      Identity and Access Management (IAM) API action.</p> |
| `device_configuration` | String |  | <p>Defines the device configuration for an Snowball Edge job.</p>
         <p>For more information, see
      "https://docs.aws.amazon.com/snowball/latest/snowcone-guide/snow-device-types.html" (Snow
      Family Devices and Capacity) in the <i>Snowcone User Guide</i> or
      "https://docs.aws.amazon.com/snowball/latest/developer-guide/snow-device-types.html" (Snow
      Family Devices and Capacity) in the <i>Snowcone User Guide</i>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `sub_job_metadata` | Vec<String> | <p>Information about a specific job part (in the case of an export job), including
      shipping information, job status, and other important metadata.</p> |
| `job_metadata` | String | <p>Information about a specific job, including shipping information, job status, and other
      important metadata.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create job
job = provider.snowball.Job {
}

# Access job outputs
job_id = job.id
job_sub_job_metadata = job.sub_job_metadata
job_job_metadata = job.job_metadata
```

---


### Cluster

Cluster resource

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `job_type` | String | ✅ | <p>The type of job for this cluster. Currently, the only job type supported for clusters
      is <code>LOCAL_USE</code>.</p>
         <p>For more information, see
      "https://docs.aws.amazon.com/snowball/latest/snowcone-guide/snow-device-types.html" (Snow
      Family Devices and Capacity) in the <i>Snowcone User Guide</i> or
      "https://docs.aws.amazon.com/snowball/latest/developer-guide/snow-device-types.html" (Snow
      Family Devices and Capacity) in the <i>Snowcone User Guide</i>.</p> |
| `address_id` | String | ✅ | <p>The ID for the address that you want the cluster shipped to.</p> |
| `tax_documents` | String |  | <p>The tax documents required in your Amazon Web Services Region.</p> |
| `initial_cluster_size` | i64 |  | <p>If provided, each job will be automatically created and associated with the new cluster. If not provided, will be treated as 0.</p> |
| `snowball_capacity_preference` | String |  | <p>If your job is being created in one of the US regions, you have the option of
      specifying what size Snow device you'd like for this job. In all other regions, Snowballs come
      with 80 TB in storage capacity.</p>
         <p>For more information, see
      "https://docs.aws.amazon.com/snowball/latest/snowcone-guide/snow-device-types.html" (Snow
      Family Devices and Capacity) in the <i>Snowcone User Guide</i> or
      "https://docs.aws.amazon.com/snowball/latest/developer-guide/snow-device-types.html" (Snow
      Family Devices and Capacity) in the <i>Snowcone User Guide</i>.</p> |
| `remote_management` | String |  | <p>Allows you to securely operate and manage Snow devices in a cluster remotely from outside
      of your internal network. When set to <code>INSTALLED_AUTOSTART</code>, remote management will
      automatically be available when the device arrives at your location. Otherwise, you need to
      use the Snowball Client to manage the device.</p> |
| `kms_key_arn` | String |  | <p>The <code>KmsKeyARN</code> value that you want to associate with this cluster.
        <code>KmsKeyARN</code> values are created by using the <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_CreateKey.html">CreateKey</a> API action in Key Management Service (KMS). </p> |
| `force_create_jobs` | bool |  | <p>Force to create cluster when user attempts to overprovision or underprovision a cluster. A cluster is overprovisioned or underprovisioned if the initial size of the cluster is more (overprovisioned) or less (underprovisioned) than what 
      needed to meet capacity requirement specified with <code>OnDeviceServiceConfiguration</code>.</p> |
| `shipping_option` | String | ✅ | <p>The shipping speed for each node in this cluster. This speed doesn't dictate how soon
      you'll get each Snowball Edge device, rather it represents how quickly each device moves to
      its destination while in transit. Regional shipping speeds are as follows: </p>
         <ul>
            <li>
               <p>In Australia, you have access to express shipping. Typically, Snow devices shipped
          express are delivered in about a day.</p>
            </li>
            <li>
               <p>In the European Union (EU), you have access to express shipping. Typically, Snow
          devices shipped express are delivered in about a day. In addition, most countries in the
          EU have access to standard shipping, which typically takes less than a week, one
          way.</p>
            </li>
            <li>
               <p>In India, Snow devices are delivered in one to seven days.</p>
            </li>
            <li>
               <p>In the United States of America (US), you have access to one-day shipping and
          two-day shipping.</p>
            </li>
         </ul>
         <ul>
            <li>
               <p>In Australia, you have access to express shipping. Typically, devices shipped
          express are delivered in about a day.</p>
            </li>
            <li>
               <p>In the European Union (EU), you have access to express shipping. Typically, Snow
          devices shipped express are delivered in about a day. In addition, most countries in the
          EU have access to standard shipping, which typically takes less than a week, one
          way.</p>
            </li>
            <li>
               <p>In India, Snow devices are delivered in one to seven days.</p>
            </li>
            <li>
               <p>In the US, you have access to one-day shipping and two-day shipping.</p>
            </li>
         </ul> |
| `snowball_type` | String | ✅ | <p>The type of Snow Family devices to use for this cluster. </p>
         <note>
            <p>For cluster jobs, Amazon Web Services Snow Family currently supports only the
          <code>EDGE</code> device type.</p>
         </note>
         <p>For more information, see
      "https://docs.aws.amazon.com/snowball/latest/snowcone-guide/snow-device-types.html" (Snow
      Family Devices and Capacity) in the <i>Snowcone User Guide</i> or
      "https://docs.aws.amazon.com/snowball/latest/developer-guide/snow-device-types.html" (Snow
      Family Devices and Capacity) in the <i>Snowcone User Guide</i>.</p> |
| `forwarding_address_id` | String |  | <p>The forwarding address ID for a cluster. This field is not supported in most
      regions.</p> |
| `role_arn` | String |  | <p>The <code>RoleARN</code> that you want to associate with this cluster.
        <code>RoleArn</code> values are created by using the <a href="https://docs.aws.amazon.com/IAM/latest/APIReference/API_CreateRole.html">CreateRole</a> API action in Identity and Access Management (IAM).</p> |
| `long_term_pricing_ids` | i64 |  | <p>Lists long-term pricing id that will be used to associate with jobs automatically created for the new cluster.</p> |
| `resources` | String |  | <p>The resources associated with the cluster job. These resources include Amazon S3
      buckets and optional Lambda functions written in the Python language.
    </p> |
| `on_device_service_configuration` | String |  | <p>Specifies the service or services on the Snow Family device that your transferred data
      will be exported from or imported into. Amazon Web Services Snow Family device clusters support Amazon S3 and NFS
      (Network File System).</p> |
| `description` | String |  | <p>An optional description of this specific cluster, for example <code>Environmental Data
        Cluster-01</code>.</p> |
| `notification` | String |  | <p>The Amazon Simple Notification Service (Amazon SNS) notification settings for this
      cluster.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `cluster_metadata` | String | <p>Information about a specific cluster, including shipping information, cluster status,
      and other important metadata.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create cluster
cluster = provider.snowball.Cluster {
    job_type = "value"  # <p>The type of job for this cluster. Currently, the only job type supported for clusters
      is <code>LOCAL_USE</code>.</p>
         <p>For more information, see
      "https://docs.aws.amazon.com/snowball/latest/snowcone-guide/snow-device-types.html" (Snow
      Family Devices and Capacity) in the <i>Snowcone User Guide</i> or
      "https://docs.aws.amazon.com/snowball/latest/developer-guide/snow-device-types.html" (Snow
      Family Devices and Capacity) in the <i>Snowcone User Guide</i>.</p>
    address_id = "value"  # <p>The ID for the address that you want the cluster shipped to.</p>
    shipping_option = "value"  # <p>The shipping speed for each node in this cluster. This speed doesn't dictate how soon
      you'll get each Snowball Edge device, rather it represents how quickly each device moves to
      its destination while in transit. Regional shipping speeds are as follows: </p>
         <ul>
            <li>
               <p>In Australia, you have access to express shipping. Typically, Snow devices shipped
          express are delivered in about a day.</p>
            </li>
            <li>
               <p>In the European Union (EU), you have access to express shipping. Typically, Snow
          devices shipped express are delivered in about a day. In addition, most countries in the
          EU have access to standard shipping, which typically takes less than a week, one
          way.</p>
            </li>
            <li>
               <p>In India, Snow devices are delivered in one to seven days.</p>
            </li>
            <li>
               <p>In the United States of America (US), you have access to one-day shipping and
          two-day shipping.</p>
            </li>
         </ul>
         <ul>
            <li>
               <p>In Australia, you have access to express shipping. Typically, devices shipped
          express are delivered in about a day.</p>
            </li>
            <li>
               <p>In the European Union (EU), you have access to express shipping. Typically, Snow
          devices shipped express are delivered in about a day. In addition, most countries in the
          EU have access to standard shipping, which typically takes less than a week, one
          way.</p>
            </li>
            <li>
               <p>In India, Snow devices are delivered in one to seven days.</p>
            </li>
            <li>
               <p>In the US, you have access to one-day shipping and two-day shipping.</p>
            </li>
         </ul>
    snowball_type = "value"  # <p>The type of Snow Family devices to use for this cluster. </p>
         <note>
            <p>For cluster jobs, Amazon Web Services Snow Family currently supports only the
          <code>EDGE</code> device type.</p>
         </note>
         <p>For more information, see
      "https://docs.aws.amazon.com/snowball/latest/snowcone-guide/snow-device-types.html" (Snow
      Family Devices and Capacity) in the <i>Snowcone User Guide</i> or
      "https://docs.aws.amazon.com/snowball/latest/developer-guide/snow-device-types.html" (Snow
      Family Devices and Capacity) in the <i>Snowcone User Guide</i>.</p>
}

# Access cluster outputs
cluster_id = cluster.id
cluster_cluster_metadata = cluster.cluster_metadata
```

---


### Long_term_pricing

LongTermPricing resource

**Operations**: ✅ Create ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `snowball_type` | String | ✅ | <p>The type of Snow Family devices to use for the long-term pricing job.</p> |
| `long_term_pricing_type` | i64 | ✅ | <p>The type of long-term pricing option you want for the device, either 1-year or 3-year
      long-term pricing.</p> |
| `is_long_term_pricing_auto_renew` | bool |  | <p>Specifies whether the current long-term pricing type for the device should be
      renewed.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create long_term_pricing
long_term_pricing = provider.snowball.Long_term_pricing {
    snowball_type = "value"  # <p>The type of Snow Family devices to use for the long-term pricing job.</p>
    long_term_pricing_type = "value"  # <p>The type of long-term pricing option you want for the device, either 1-year or 3-year
      long-term pricing.</p>
}

```

---


### Job_unlock_code

JobUnlockCode resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `unlock_code` | String | <p>The <code>UnlockCode</code> value for the specified job. The <code>UnlockCode</code>
      value can be accessed for up to 360 days after the job has been created.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access job_unlock_code outputs
job_unlock_code_id = job_unlock_code.id
job_unlock_code_unlock_code = job_unlock_code.unlock_code
```

---


### Address

Address resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `address` | String | ✅ | <p>The address that you want the Snow device shipped to.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `address` | String | <p>The address that you want the Snow device(s) associated with a specific job to be
      shipped to.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create address
address = provider.snowball.Address {
    address = "value"  # <p>The address that you want the Snow device shipped to.</p>
}

# Access address outputs
address_id = address.id
address_address = address.address
```

---


### Job_manifest

JobManifest resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `manifest_uri` | String | <p>The Amazon S3 presigned URL for the manifest file associated with the specified
        <code>JobId</code> value.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access job_manifest outputs
job_manifest_id = job_manifest.id
job_manifest_manifest_uri = job_manifest.manifest_uri
```

---


### Return_shipping_label

ReturnShippingLabel resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `shipping_option` | String |  | <p>The shipping speed for a particular job. This speed doesn't dictate how soon the device
      is returned to Amazon Web Services. This speed represents how quickly it moves to its
      destination while in transit. Regional shipping speeds are as follows:</p> |
| `job_id` | String | ✅ | <p>The ID for a job that you want to create the return shipping label for; for example,
        <code>JID123e4567-e89b-12d3-a456-426655440000</code>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `expiration_date` | String | <p>The expiration date of the current return shipping label.</p> |
| `status` | String | <p>The status information of the task on a Snow device that is being returned to Amazon Web Services.</p> |
| `return_shipping_label_uri` | String | <p>The pre-signed Amazon S3 URI used to download the return shipping label.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create return_shipping_label
return_shipping_label = provider.snowball.Return_shipping_label {
    job_id = "value"  # <p>The ID for a job that you want to create the return shipping label for; for example,
        <code>JID123e4567-e89b-12d3-a456-426655440000</code>.</p>
}

# Access return_shipping_label outputs
return_shipping_label_id = return_shipping_label.id
return_shipping_label_expiration_date = return_shipping_label.expiration_date
return_shipping_label_status = return_shipping_label.status
return_shipping_label_return_shipping_label_uri = return_shipping_label.return_shipping_label_uri
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple snowball_usage resources
snowball_usage_0 = provider.snowball.Snowball_usage {
}
snowball_usage_1 = provider.snowball.Snowball_usage {
}
snowball_usage_2 = provider.snowball.Snowball_usage {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    snowball_usage = provider.snowball.Snowball_usage {
    }
```

---

## Related Documentation

- [AWS Snowball Documentation](https://docs.aws.amazon.com/snowball/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
