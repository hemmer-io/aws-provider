# Acm Service



**Resources**: 3

---

## Overview

The acm service provides access to 3 resource types:

- [Account_configuration](#account_configuration) [CR]
- [Certificate](#certificate) [RD]
- [Certificate_options](#certificate_options) [U]

---

## Resources


### Account_configuration

AccountConfiguration resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `expiry_events` | String |  | <p>Specifies expiration events associated with an account.</p> |
| `idempotency_token` | String | ✅ | <p>Customer-chosen string used to distinguish between calls to <code>PutAccountConfiguration</code>. Idempotency tokens time out after one hour. If you call <code>PutAccountConfiguration</code> multiple times with the same unexpired idempotency token, ACM treats it as the same request and returns the original result. If you change the idempotency token for each call, ACM treats each call as a new request.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `expiry_events` | String | <p>Expiration events configuration options associated with the Amazon Web Services account.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create account_configuration
account_configuration = provider.acm.Account_configuration {
    idempotency_token = "value"  # <p>Customer-chosen string used to distinguish between calls to <code>PutAccountConfiguration</code>. Idempotency tokens time out after one hour. If you call <code>PutAccountConfiguration</code> multiple times with the same unexpired idempotency token, ACM treats it as the same request and returns the original result. If you change the idempotency token for each call, ACM treats each call as a new request.</p>
}

# Access account_configuration outputs
account_configuration_id = account_configuration.id
account_configuration_expiry_events = account_configuration.expiry_events
```

---


### Certificate

Certificate resource

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `certificate_chain` | String | <p>Certificates forming the requested certificate's chain of trust. The chain consists of the certificate of the issuing CA and the intermediate certificates of any other subordinate CAs. </p> |
| `certificate` | String | <p>The ACM-issued certificate corresponding to the ARN specified as input.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access certificate outputs
certificate_id = certificate.id
certificate_certificate_chain = certificate.certificate_chain
certificate_certificate = certificate.certificate
```

---


### Certificate_options

CertificateOptions resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `options` | String | ✅ | <p>Use to update the options for your certificate. Currently, you can specify whether to add your certificate to a transparency log or export your certificate. Certificate transparency makes it possible to detect SSL/TLS certificates that have been mistakenly or maliciously issued. Certificates that have not been logged typically produce an error message in a browser. </p> |
| `certificate_arn` | String | ✅ | <p>ARN of the requested certificate to update. This must be of the form:</p> <p> <code>arn:aws:acm:us-east-1:<i>account</i>:certificate/<i>12345678-1234-1234-1234-123456789012</i> </code> </p> |



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



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple account_configuration resources
account_configuration_0 = provider.acm.Account_configuration {
    idempotency_token = "value-0"
}
account_configuration_1 = provider.acm.Account_configuration {
    idempotency_token = "value-1"
}
account_configuration_2 = provider.acm.Account_configuration {
    idempotency_token = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    account_configuration = provider.acm.Account_configuration {
        idempotency_token = "production-value"
    }
```

---

## Related Documentation

- [AWS Acm Documentation](https://docs.aws.amazon.com/acm/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
