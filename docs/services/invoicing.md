# Invoicing Service



**Resources**: 1

---

## Overview

The invoicing service provides access to 1 resource type:

- [Invoice_unit](#invoice_unit) [CRUD]

---

## Resources


### Invoice_unit

InvoiceUnit resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `rule` | String | ✅ | <p>The <code>InvoiceUnitRule</code> object used to create invoice units. </p> |
| `name` | String | ✅ | <p> The unique name of the invoice unit that is shown on the generated invoice. This can't be changed once it is set. To change this name, you must delete the invoice unit recreate. </p> |
| `resource_tags` | Vec<String> |  | <p> The tag structure that contains a tag key and value. </p> |
| `invoice_receiver` | String | ✅ | <p> The Amazon Web Services account ID chosen to be the receiver of an invoice unit. All invoices generated for that invoice unit will be sent to this account ID. </p> |
| `description` | String |  | <p> The invoice unit's description. This can be changed at a later time. </p> |
| `tax_inheritance_disabled` | bool |  | <p>Whether the invoice unit based tax inheritance is/ should be enabled or disabled. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `tax_inheritance_disabled` | bool | <p> Whether the invoice unit based tax inheritance is/ should be enabled or disabled. </p> |
| `name` | String | <p> The unique name of the invoice unit that is shown on the generated invoice. </p> |
| `description` | String | <p> The assigned description for an invoice unit. </p> |
| `rule` | String |  |
| `last_modified` | String | <p> The most recent date the invoice unit response was updated. </p> |
| `invoice_receiver` | String | <p> The Amazon Web Services account ID chosen to be the receiver of an invoice unit. All invoices generated for that invoice unit will be sent to this account ID. </p> |
| `invoice_unit_arn` | String | <p> The ARN to identify an invoice unit. This information can't be modified or deleted. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create invoice_unit
invoice_unit = provider.invoicing.Invoice_unit {
    rule = "value"  # <p>The <code>InvoiceUnitRule</code> object used to create invoice units. </p>
    name = "value"  # <p> The unique name of the invoice unit that is shown on the generated invoice. This can't be changed once it is set. To change this name, you must delete the invoice unit recreate. </p>
    invoice_receiver = "value"  # <p> The Amazon Web Services account ID chosen to be the receiver of an invoice unit. All invoices generated for that invoice unit will be sent to this account ID. </p>
}

# Access invoice_unit outputs
invoice_unit_id = invoice_unit.id
invoice_unit_tax_inheritance_disabled = invoice_unit.tax_inheritance_disabled
invoice_unit_name = invoice_unit.name
invoice_unit_description = invoice_unit.description
invoice_unit_rule = invoice_unit.rule
invoice_unit_last_modified = invoice_unit.last_modified
invoice_unit_invoice_receiver = invoice_unit.invoice_receiver
invoice_unit_invoice_unit_arn = invoice_unit.invoice_unit_arn
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple invoice_unit resources
invoice_unit_0 = provider.invoicing.Invoice_unit {
    rule = "value-0"
    name = "value-0"
    invoice_receiver = "value-0"
}
invoice_unit_1 = provider.invoicing.Invoice_unit {
    rule = "value-1"
    name = "value-1"
    invoice_receiver = "value-1"
}
invoice_unit_2 = provider.invoicing.Invoice_unit {
    rule = "value-2"
    name = "value-2"
    invoice_receiver = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    invoice_unit = provider.invoicing.Invoice_unit {
        rule = "production-value"
        name = "production-value"
        invoice_receiver = "production-value"
    }
```

---

## Related Documentation

- [AWS Invoicing Documentation](https://docs.aws.amazon.com/invoicing/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
