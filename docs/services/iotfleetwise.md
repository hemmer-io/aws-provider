# Iotfleetwise Service



**Resources**: 4

---

## Overview

The iotfleetwise service provides access to 4 resource types:

- [Logging_options](#logging_options) [CR]
- [Register_account_status](#register_account_status) [R]
- [Vehicle_status](#vehicle_status) [R]
- [Encryption_configuration](#encryption_configuration) [CR]

---

## Resources


### Logging_options

LoggingOptions resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `cloud_watch_log_delivery` | String | ✅ | <p>Creates or updates the log delivery option to Amazon CloudWatch Logs.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `cloud_watch_log_delivery` | String | <p>Returns information about log delivery to Amazon CloudWatch Logs.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create logging_options
logging_options = provider.iotfleetwise.Logging_options {
    cloud_watch_log_delivery = "value"  # <p>Creates or updates the log delivery option to Amazon CloudWatch Logs.</p>
}

# Access logging_options outputs
logging_options_id = logging_options.id
logging_options_cloud_watch_log_delivery = logging_options.cloud_watch_log_delivery
```

---


### Register_account_status

RegisterAccountStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `timestream_registration_response` | String | <p> Information about the registered Amazon Timestream resources or errors, if any.</p> |
| `creation_time` | String | <p> The time the account was registered, in seconds since epoch (January 1, 1970 at
            midnight UTC time). </p> |
| `iam_registration_response` | String | <p> Information about the registered IAM resources or errors, if any. </p> |
| `customer_account_id` | String | <p> The unique ID of the Amazon Web Services account, provided at account creation. </p> |
| `account_status` | String | <p> The status of registering your account and resources. The status can be one
            of:</p>
         <ul>
            <li>
               <p>
                  <code>REGISTRATION_SUCCESS</code> - The Amazon Web Services resource is successfully
                    registered.</p>
            </li>
            <li>
               <p>
                  <code>REGISTRATION_PENDING</code> - Amazon Web Services IoT FleetWise is processing the registration
                    request. This process takes approximately five minutes to complete.</p>
            </li>
            <li>
               <p>
                  <code>REGISTRATION_FAILURE</code> - Amazon Web Services IoT FleetWise can't register the AWS resource.
                    Try again later.</p>
            </li>
         </ul> |
| `last_modification_time` | String | <p> The time this registration was last updated, in seconds since epoch (January 1, 1970
            at midnight UTC time). </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access register_account_status outputs
register_account_status_id = register_account_status.id
register_account_status_timestream_registration_response = register_account_status.timestream_registration_response
register_account_status_creation_time = register_account_status.creation_time
register_account_status_iam_registration_response = register_account_status.iam_registration_response
register_account_status_customer_account_id = register_account_status.customer_account_id
register_account_status_account_status = register_account_status.account_status
register_account_status_last_modification_time = register_account_status.last_modification_time
```

---


### Vehicle_status

VehicleStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p> The token to retrieve the next set of results, or <code>null</code> if there are no more results. </p> |
| `campaigns` | Vec<String> | <p> Lists information about the state of the vehicle with deployed campaigns. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access vehicle_status outputs
vehicle_status_id = vehicle_status.id
vehicle_status_next_token = vehicle_status.next_token
vehicle_status_campaigns = vehicle_status.campaigns
```

---


### Encryption_configuration

EncryptionConfiguration resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `kms_key_id` | String |  | <p>The ID of the KMS key that is used for encryption.</p> |
| `encryption_type` | String | ✅ | <p>The type of encryption. Choose <code>KMS_BASED_ENCRYPTION</code> to use a KMS key or
                <code>FLEETWISE_DEFAULT_ENCRYPTION</code> to use an Amazon Web Services managed key.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `last_modification_time` | String | <p>The time when encryption was last updated in seconds since epoch (January 1, 1970 at
            midnight UTC time).</p> |
| `kms_key_id` | String | <p>The ID of the KMS key that is used for encryption.</p> |
| `encryption_type` | String | <p>The type of encryption. Set to <code>KMS_BASED_ENCRYPTION</code> to use a KMS key
            that you own and manage. Set to <code>FLEETWISE_DEFAULT_ENCRYPTION</code> to use an
            Amazon Web Services managed key that is owned by the Amazon Web Services IoT FleetWise service account.</p> |
| `error_message` | String | <p>The error message that describes why encryption settings couldn't be configured, if
            applicable.</p> |
| `encryption_status` | String | <p>The encryption status.</p> |
| `creation_time` | String | <p>The time when encryption was configured in seconds since epoch (January 1, 1970 at
            midnight UTC time).</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create encryption_configuration
encryption_configuration = provider.iotfleetwise.Encryption_configuration {
    encryption_type = "value"  # <p>The type of encryption. Choose <code>KMS_BASED_ENCRYPTION</code> to use a KMS key or
                <code>FLEETWISE_DEFAULT_ENCRYPTION</code> to use an Amazon Web Services managed key.</p>
}

# Access encryption_configuration outputs
encryption_configuration_id = encryption_configuration.id
encryption_configuration_last_modification_time = encryption_configuration.last_modification_time
encryption_configuration_kms_key_id = encryption_configuration.kms_key_id
encryption_configuration_encryption_type = encryption_configuration.encryption_type
encryption_configuration_error_message = encryption_configuration.error_message
encryption_configuration_encryption_status = encryption_configuration.encryption_status
encryption_configuration_creation_time = encryption_configuration.creation_time
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple logging_options resources
logging_options_0 = provider.iotfleetwise.Logging_options {
    cloud_watch_log_delivery = "value-0"
}
logging_options_1 = provider.iotfleetwise.Logging_options {
    cloud_watch_log_delivery = "value-1"
}
logging_options_2 = provider.iotfleetwise.Logging_options {
    cloud_watch_log_delivery = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    logging_options = provider.iotfleetwise.Logging_options {
        cloud_watch_log_delivery = "production-value"
    }
```

---

## Related Documentation

- [AWS Iotfleetwise Documentation](https://docs.aws.amazon.com/iotfleetwise/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
