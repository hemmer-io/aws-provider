# Taxsettings Service



**Resources**: 6

---

## Overview

The taxsettings service provides access to 6 resource types:

- [Tax_exemption_types](#tax_exemption_types) [R]
- [Tax_registration](#tax_registration) [CRD]
- [Supplemental_tax_registration](#supplemental_tax_registration) [CD]
- [Tax_registration_document](#tax_registration_document) [R]
- [Tax_exemption](#tax_exemption) [C]
- [Tax_inheritance](#tax_inheritance) [CR]

---

## Resources


### Tax_exemption_types

TaxExemptionTypes resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `tax_exemption_types` | Vec<String> | <p>The supported types of tax exemptions.
    </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access tax_exemption_types outputs
tax_exemption_types_id = tax_exemption_types.id
tax_exemption_types_tax_exemption_types = tax_exemption_types.tax_exemption_types
```

---


### Tax_registration

TaxRegistration resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tax_registration_entry` | String | ✅ | <p> Your TRN information that will be stored to the account mentioned in
        <code>accountId</code>. </p> |
| `account_id` | String |  | <p>Your unique account identifier. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `tax_registration` | String | <p>TRN information of the account mentioned in the request. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create tax_registration
tax_registration = provider.taxsettings.Tax_registration {
    tax_registration_entry = "value"  # <p> Your TRN information that will be stored to the account mentioned in
        <code>accountId</code>. </p>
}

# Access tax_registration outputs
tax_registration_id = tax_registration.id
tax_registration_tax_registration = tax_registration.tax_registration
```

---


### Supplemental_tax_registration

SupplementalTaxRegistration resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tax_registration_entry` | String | ✅ | <p>
      The supplemental TRN information that will be stored for the caller account ID.
    </p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create supplemental_tax_registration
supplemental_tax_registration = provider.taxsettings.Supplemental_tax_registration {
    tax_registration_entry = "value"  # <p>
      The supplemental TRN information that will be stored for the caller account ID.
    </p>
}

```

---


### Tax_registration_document

TaxRegistrationDocument resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `destination_file_path` | String | <p>The file path of the Amazon S3 bucket where you want to download your tax document to.</p> |
| `presigned_s3_url` | String | <p>The Amazon S3 presigned URL of the tax registration document.
    </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access tax_registration_document outputs
tax_registration_document_id = tax_registration_document.id
tax_registration_document_destination_file_path = tax_registration_document.destination_file_path
tax_registration_document_presigned_s3_url = tax_registration_document.presigned_s3_url
```

---


### Tax_exemption

TaxExemption resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `account_ids` | Vec<String> | ✅ | <p>
      The list of unique account identifiers.
    </p> |
| `exemption_type` | String | ✅ | <p>The exemption type. Use the supported tax exemption type description.
    </p> |
| `exemption_certificate` | String | ✅ |  |
| `authority` | String | ✅ |  |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create tax_exemption
tax_exemption = provider.taxsettings.Tax_exemption {
    account_ids = "value"  # <p>
      The list of unique account identifiers.
    </p>
    exemption_type = "value"  # <p>The exemption type. Use the supported tax exemption type description.
    </p>
    exemption_certificate = "value"  # Required field
    authority = "value"  # Required field
}

```

---


### Tax_inheritance

TaxInheritance resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `heritage_status` | String |  | <p>The tax inheritance status.
    </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `heritage_status` | String | <p>The tax inheritance status.
    </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create tax_inheritance
tax_inheritance = provider.taxsettings.Tax_inheritance {
}

# Access tax_inheritance outputs
tax_inheritance_id = tax_inheritance.id
tax_inheritance_heritage_status = tax_inheritance.heritage_status
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple tax_exemption_types resources
tax_exemption_types_0 = provider.taxsettings.Tax_exemption_types {
}
tax_exemption_types_1 = provider.taxsettings.Tax_exemption_types {
}
tax_exemption_types_2 = provider.taxsettings.Tax_exemption_types {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    tax_exemption_types = provider.taxsettings.Tax_exemption_types {
    }
```

---

## Related Documentation

- [AWS Taxsettings Documentation](https://docs.aws.amazon.com/taxsettings/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
