# Payment_cryptography Service



**Resources**: 5

---

## Overview

The payment_cryptography service provides access to 5 resource types:

- [Certificate_signing_request](#certificate_signing_request) [R]
- [Parameters_for_export](#parameters_for_export) [R]
- [Parameters_for_import](#parameters_for_import) [R]
- [Public_key_certificate](#public_key_certificate) [R]
- [Default_key_replication_regions](#default_key_replication_regions) [R]

---

## Resources


### Certificate_signing_request

CertificateSigningRequest resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `certificate_signing_request` | String | <p>The certificate signing request generated using the key pair associated with the key identifier.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access certificate_signing_request outputs
certificate_signing_request_id = certificate_signing_request.id
certificate_signing_request_certificate_signing_request = certificate_signing_request.certificate_signing_request
```

---


### Parameters_for_export

ParametersForExport resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `export_token` | String | <p>The export token to initiate key export from Amazon Web Services Payment Cryptography. The export token expires after 30 days. You can use the same export token to export multiple keys from the same service account.</p> |
| `signing_key_certificate_chain` | String | <p>The root certificate authority (CA) that signed the signing key certificate in PEM format (base64 encoded).</p> |
| `parameters_valid_until_timestamp` | String | <p>The validity period of the export token.</p> |
| `signing_key_algorithm` | String | <p>The algorithm of the signing key certificate for use in TR-34 key block generation. <code>RSA_2048</code> is the only signing key algorithm allowed.</p> |
| `signing_key_certificate` | String | <p>The signing key certificate in PEM format (base64 encoded) of the public key for signature within the TR-34 key block. The certificate expires after 30 days.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access parameters_for_export outputs
parameters_for_export_id = parameters_for_export.id
parameters_for_export_export_token = parameters_for_export.export_token
parameters_for_export_signing_key_certificate_chain = parameters_for_export.signing_key_certificate_chain
parameters_for_export_parameters_valid_until_timestamp = parameters_for_export.parameters_valid_until_timestamp
parameters_for_export_signing_key_algorithm = parameters_for_export.signing_key_algorithm
parameters_for_export_signing_key_certificate = parameters_for_export.signing_key_certificate
```

---


### Parameters_for_import

ParametersForImport resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `import_token` | String | <p>The import token to initiate key import into Amazon Web Services Payment Cryptography. The import token expires after 30 days. You can use the same import token to import multiple keys to the same service account.</p> |
| `wrapping_key_certificate` | String | <p>The wrapping key certificate in PEM format (base64 encoded) of the wrapping key for use within the TR-34 key block. The certificate expires in 30 days.</p> |
| `wrapping_key_certificate_chain` | String | <p>The Amazon Web Services Payment Cryptography root certificate authority (CA) that signed the wrapping key certificate in PEM format (base64 encoded).</p> |
| `wrapping_key_algorithm` | String | <p>The algorithm of the wrapping key for use within TR-34 WrappedKeyBlock or RSA WrappedKeyCryptogram.</p> |
| `parameters_valid_until_timestamp` | String | <p>The validity period of the import token.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access parameters_for_import outputs
parameters_for_import_id = parameters_for_import.id
parameters_for_import_import_token = parameters_for_import.import_token
parameters_for_import_wrapping_key_certificate = parameters_for_import.wrapping_key_certificate
parameters_for_import_wrapping_key_certificate_chain = parameters_for_import.wrapping_key_certificate_chain
parameters_for_import_wrapping_key_algorithm = parameters_for_import.wrapping_key_algorithm
parameters_for_import_parameters_valid_until_timestamp = parameters_for_import.parameters_valid_until_timestamp
```

---


### Public_key_certificate

PublicKeyCertificate resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `key_certificate` | String | <p>The public key component of the asymmetric key pair in a certificate PEM format (base64 encoded). It is signed by the root certificate authority (CA). The certificate expires in 90 days.</p> |
| `key_certificate_chain` | String | <p>The root certificate authority (CA) that signed the public key certificate in PEM format (base64 encoded) of the asymmetric key pair.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access public_key_certificate outputs
public_key_certificate_id = public_key_certificate.id
public_key_certificate_key_certificate = public_key_certificate.key_certificate
public_key_certificate_key_certificate_chain = public_key_certificate.key_certificate_chain
```

---


### Default_key_replication_regions

DefaultKeyReplicationRegions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `enabled_replication_regions` | Vec<String> | <p>The list of regions where default key replication is currently enabled for the account.</p> <p>New keys created in this account will automatically be replicated to these regions unless explicitly configured otherwise during key creation.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access default_key_replication_regions outputs
default_key_replication_regions_id = default_key_replication_regions.id
default_key_replication_regions_enabled_replication_regions = default_key_replication_regions.enabled_replication_regions
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple certificate_signing_request resources
certificate_signing_request_0 = provider.payment_cryptography.Certificate_signing_request {
}
certificate_signing_request_1 = provider.payment_cryptography.Certificate_signing_request {
}
certificate_signing_request_2 = provider.payment_cryptography.Certificate_signing_request {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    certificate_signing_request = provider.payment_cryptography.Certificate_signing_request {
    }
```

---

## Related Documentation

- [AWS Payment_cryptography Documentation](https://docs.aws.amazon.com/payment_cryptography/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
