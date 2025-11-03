# Acm_pca Service



**Resources**: 7

---

## Overview

The acm_pca service provides access to 7 resource types:

- [Certificate_authority_csr](#certificate_authority_csr) [R]
- [Certificate](#certificate) [R]
- [Certificate_authority_audit_report](#certificate_authority_audit_report) [CR]
- [Policy](#policy) [CRD]
- [Certificate_authority_certificate](#certificate_authority_certificate) [R]
- [Certificate_authority](#certificate_authority) [CRUD]
- [Permission](#permission) [CD]

---

## Resources


### Certificate_authority_csr

CertificateAuthorityCsr resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `csr` | String | <p>The base64 PEM-encoded certificate signing request (CSR) for your private CA certificate.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access certificate_authority_csr outputs
certificate_authority_csr_id = certificate_authority_csr.id
certificate_authority_csr_csr = certificate_authority_csr.csr
```

---


### Certificate

Certificate resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `certificate` | String | <p>The base64 PEM-encoded certificate specified by the <code>CertificateArn</code> parameter.</p> |
| `certificate_chain` | String | <p>The base64 PEM-encoded certificate chain that chains up to the root CA certificate that you used to sign your private CA certificate. </p> |


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
certificate_certificate = certificate.certificate
certificate_certificate_chain = certificate.certificate_chain
```

---


### Certificate_authority_audit_report

CertificateAuthorityAuditReport resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `s3_bucket_name` | String | ✅ | <p>The name of the S3 bucket that will contain the audit report.</p> |
| `certificate_authority_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the CA to be audited. This is of the form:</p> <p> <code>arn:aws:acm-pca:<i>region</i>:<i>account</i>:certificate-authority/<i>12345678-1234-1234-1234-123456789012</i> </code>.</p> |
| `audit_report_response_format` | String | ✅ | <p>The format in which to create the report. This can be either <b>JSON</b> or <b>CSV</b>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `audit_report_status` | String | <p>Specifies whether report creation is in progress, has succeeded, or has failed.</p> |
| `s3_bucket_name` | String | <p>Name of the S3 bucket that contains the report.</p> |
| `created_at` | String | <p>The date and time at which the report was created.</p> |
| `s3_key` | String | <p>S3 <b>key</b> that uniquely identifies the report file in your S3 bucket.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create certificate_authority_audit_report
certificate_authority_audit_report = provider.acm_pca.Certificate_authority_audit_report {
    s3_bucket_name = "value"  # <p>The name of the S3 bucket that will contain the audit report.</p>
    certificate_authority_arn = "value"  # <p>The Amazon Resource Name (ARN) of the CA to be audited. This is of the form:</p> <p> <code>arn:aws:acm-pca:<i>region</i>:<i>account</i>:certificate-authority/<i>12345678-1234-1234-1234-123456789012</i> </code>.</p>
    audit_report_response_format = "value"  # <p>The format in which to create the report. This can be either <b>JSON</b> or <b>CSV</b>.</p>
}

# Access certificate_authority_audit_report outputs
certificate_authority_audit_report_id = certificate_authority_audit_report.id
certificate_authority_audit_report_audit_report_status = certificate_authority_audit_report.audit_report_status
certificate_authority_audit_report_s3_bucket_name = certificate_authority_audit_report.s3_bucket_name
certificate_authority_audit_report_created_at = certificate_authority_audit_report.created_at
certificate_authority_audit_report_s3_key = certificate_authority_audit_report.s3_key
```

---


### Policy

Policy resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `policy` | String | ✅ | <p>The path and file name of a JSON-formatted IAM policy to attach to the specified private CA resource. If this policy does not contain all required statements or if it includes any statement that is not allowed, the <code>PutPolicy</code> action returns an <code>InvalidPolicyException</code>. For information about IAM policy and statement structure, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies.html#access_policies-json">Overview of JSON Policies</a>.</p> |
| `resource_arn` | String | ✅ | <p>The Amazon Resource Number (ARN) of the private CA to associate with the policy. The ARN of the CA can be found by calling the <a href="https://docs.aws.amazon.com/privateca/latest/APIReference/API_ListCertificateAuthorities.html">ListCertificateAuthorities</a> action.</p> <p/> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `policy` | String | <p>The policy attached to the private CA as a JSON document.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create policy
policy = provider.acm_pca.Policy {
    policy = "value"  # <p>The path and file name of a JSON-formatted IAM policy to attach to the specified private CA resource. If this policy does not contain all required statements or if it includes any statement that is not allowed, the <code>PutPolicy</code> action returns an <code>InvalidPolicyException</code>. For information about IAM policy and statement structure, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies.html#access_policies-json">Overview of JSON Policies</a>.</p>
    resource_arn = "value"  # <p>The Amazon Resource Number (ARN) of the private CA to associate with the policy. The ARN of the CA can be found by calling the <a href="https://docs.aws.amazon.com/privateca/latest/APIReference/API_ListCertificateAuthorities.html">ListCertificateAuthorities</a> action.</p> <p/>
}

# Access policy outputs
policy_id = policy.id
policy_policy = policy.policy
```

---


### Certificate_authority_certificate

CertificateAuthorityCertificate resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `certificate` | String | <p>Base64-encoded certificate authority (CA) certificate.</p> |
| `certificate_chain` | String | <p>Base64-encoded certificate chain that includes any intermediate certificates and chains up to root certificate that you used to sign your private CA certificate. The chain does not include your private CA certificate. If this is a root CA, the value will be null.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access certificate_authority_certificate outputs
certificate_authority_certificate_id = certificate_authority_certificate.id
certificate_authority_certificate_certificate = certificate_authority_certificate.certificate
certificate_authority_certificate_certificate_chain = certificate_authority_certificate.certificate_chain
```

---


### Certificate_authority

CertificateAuthority resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `certificate_authority_type` | String | ✅ | <p>The type of the certificate authority.</p> |
| `idempotency_token` | String |  | <p>Custom string that can be used to distinguish between calls to the <b>CreateCertificateAuthority</b> action. Idempotency tokens for <b>CreateCertificateAuthority</b> time out after five minutes. Therefore, if you call <b>CreateCertificateAuthority</b> multiple times with the same idempotency token within five minutes, Amazon Web Services Private CA recognizes that you are requesting only certificate authority and will issue only one. If you change the idempotency token for each call, Amazon Web Services Private CA recognizes that you are requesting multiple certificate authorities.</p> |
| `key_storage_security_standard` | String |  | <p>Specifies a cryptographic key management compliance standard for handling and protecting CA keys.</p> <p>Default: FIPS_140_2_LEVEL_3_OR_HIGHER</p> <note> <p>Some Amazon Web Services Regions don't support the default value. When you create a CA in these Regions, you must use <code>CCPC_LEVEL_1_OR_HIGHER</code> for the <code>KeyStorageSecurityStandard</code> parameter. If you don't, the operation returns an <code>InvalidArgsException</code> with this message: "A certificate authority cannot be created in this region with the specified security standard."</p> <p>For information about security standard support in different Amazon Web Services Regions, see <a href="https://docs.aws.amazon.com/privateca/latest/userguide/data-protection.html#private-keys">Storage and security compliance of Amazon Web Services Private CA private keys</a>.</p> </note> |
| `tags` | Vec<String> |  | <p>Key-value pairs that will be attached to the new private CA. You can associate up to 50 tags with a private CA. For information using tags with IAM to manage permissions, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_iam-tags.html">Controlling Access Using IAM Tags</a>.</p> |
| `usage_mode` | String |  | <p>Specifies whether the CA issues general-purpose certificates that typically require a revocation mechanism, or short-lived certificates that may optionally omit revocation because they expire quickly. Short-lived certificate validity is limited to seven days.</p> <p>The default value is GENERAL_PURPOSE.</p> |
| `certificate_authority_configuration` | String | ✅ | <p>Name and bit size of the private key algorithm, the name of the signing algorithm, and X.500 certificate subject information.</p> |
| `revocation_configuration` | String |  | <p>Contains information to enable support for Online Certificate Status Protocol (OCSP), certificate revocation list (CRL), both protocols, or neither. By default, both certificate validation mechanisms are disabled.</p> <p>The following requirements apply to revocation configurations.</p> <ul> <li> <p>A configuration disabling CRLs or OCSP must contain only the <code>Enabled=False</code> parameter, and will fail if other parameters such as <code>CustomCname</code> or <code>ExpirationInDays</code> are included.</p> </li> <li> <p>In a CRL configuration, the <code>S3BucketName</code> parameter must conform to <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/bucketnamingrules.html">Amazon S3 bucket naming rules</a>.</p> </li> <li> <p>A configuration containing a custom Canonical Name (CNAME) parameter for CRLs or OCSP must conform to <a href="https://www.ietf.org/rfc/rfc2396.txt">RFC2396</a> restrictions on the use of special characters in a CNAME. </p> </li> <li> <p>In a CRL or OCSP configuration, the value of a CNAME parameter must not include a protocol prefix such as "http://" or "https://".</p> </li> </ul> <p> For more information, see the <a href="https://docs.aws.amazon.com/privateca/latest/APIReference/API_OcspConfiguration.html">OcspConfiguration</a> and <a href="https://docs.aws.amazon.com/privateca/latest/APIReference/API_CrlConfiguration.html">CrlConfiguration</a> types.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `certificate_authority` | String | <p>A <a href="https://docs.aws.amazon.com/privateca/latest/APIReference/API_CertificateAuthority.html">CertificateAuthority</a> structure that contains information about your private CA.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create certificate_authority
certificate_authority = provider.acm_pca.Certificate_authority {
    certificate_authority_type = "value"  # <p>The type of the certificate authority.</p>
    certificate_authority_configuration = "value"  # <p>Name and bit size of the private key algorithm, the name of the signing algorithm, and X.500 certificate subject information.</p>
}

# Access certificate_authority outputs
certificate_authority_id = certificate_authority.id
certificate_authority_certificate_authority = certificate_authority.certificate_authority
```

---


### Permission

Permission resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `principal` | String | ✅ | <p>The Amazon Web Services service or identity that receives the permission. At this time, the only valid principal is <code>acm.amazonaws.com</code>.</p> |
| `certificate_authority_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the CA that grants the permissions. You can find the ARN by calling the <a href="https://docs.aws.amazon.com/privateca/latest/APIReference/API_ListCertificateAuthorities.html">ListCertificateAuthorities</a> action. This must have the following form: </p> <p> <code>arn:aws:acm-pca:<i>region</i>:<i>account</i>:certificate-authority/<i>12345678-1234-1234-1234-123456789012</i> </code>. </p> |
| `source_account` | String |  | <p>The ID of the calling account.</p> |
| `actions` | Vec<String> | ✅ | <p>The actions that the specified Amazon Web Services service principal can use. These include <code>IssueCertificate</code>, <code>GetCertificate</code>, and <code>ListPermissions</code>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create permission
permission = provider.acm_pca.Permission {
    principal = "value"  # <p>The Amazon Web Services service or identity that receives the permission. At this time, the only valid principal is <code>acm.amazonaws.com</code>.</p>
    certificate_authority_arn = "value"  # <p>The Amazon Resource Name (ARN) of the CA that grants the permissions. You can find the ARN by calling the <a href="https://docs.aws.amazon.com/privateca/latest/APIReference/API_ListCertificateAuthorities.html">ListCertificateAuthorities</a> action. This must have the following form: </p> <p> <code>arn:aws:acm-pca:<i>region</i>:<i>account</i>:certificate-authority/<i>12345678-1234-1234-1234-123456789012</i> </code>. </p>
    actions = "value"  # <p>The actions that the specified Amazon Web Services service principal can use. These include <code>IssueCertificate</code>, <code>GetCertificate</code>, and <code>ListPermissions</code>.</p>
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

# Create multiple certificate_authority_csr resources
certificate_authority_csr_0 = provider.acm_pca.Certificate_authority_csr {
}
certificate_authority_csr_1 = provider.acm_pca.Certificate_authority_csr {
}
certificate_authority_csr_2 = provider.acm_pca.Certificate_authority_csr {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    certificate_authority_csr = provider.acm_pca.Certificate_authority_csr {
    }
```

---

## Related Documentation

- [AWS Acm_pca Documentation](https://docs.aws.amazon.com/acm_pca/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
