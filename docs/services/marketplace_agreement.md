# Marketplace_agreement Service



**Resources**: 2

---

## Overview

The marketplace_agreement service provides access to 2 resource types:

- [Agreement_terms](#agreement_terms) [R]
- [Agreement](#agreement) [R]

---

## Resources


### Agreement_terms

AgreementTerms resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>A token to specify where to start pagination</p> |
| `accepted_terms` | Vec<String> | <p>A subset of terms proposed by the proposer that have been accepted by the acceptor as
         part of the agreement creation.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access agreement_terms outputs
agreement_terms_id = agreement_terms.id
agreement_terms_next_token = agreement_terms.next_token
agreement_terms_accepted_terms = agreement_terms.accepted_terms
```

---


### Agreement

Agreement resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `start_time` | String | <p>The date and time when the agreement starts.</p> |
| `end_time` | String | <p>The date and time when the agreement ends. The field is <code>null</code> for
         pay-as-you-go agreements, which don’t have end dates.</p> |
| `acceptance_time` | String | <p>The date and time the offer was accepted or the agreement was created.</p>
         <note>
            <p>
               <code>AcceptanceTime</code> and <code>StartTime</code> can differ for future dated
            agreements (FDAs).</p>
         </note> |
| `agreement_type` | String | <p>The type of agreement. Values are <code>PurchaseAgreement</code> or
            <code>VendorInsightsAgreement</code>.</p> |
| `estimated_charges` | String | <p>The estimated cost of the agreement.</p> |
| `proposal_summary` | String | <p>A summary of the proposal received from the proposer.</p> |
| `status` | String | <p>The current status of the agreement.</p>
         <p>Statuses include:</p>
         <ul>
            <li>
               <p>
                  <code>ACTIVE</code> – The terms of the agreement are active.</p>
            </li>
            <li>
               <p>
                  <code>ARCHIVED</code> – The agreement ended without a specified reason.</p>
            </li>
            <li>
               <p>
                  <code>CANCELLED</code> – The acceptor ended the agreement before the defined end
               date.</p>
            </li>
            <li>
               <p>
                  <code>EXPIRED</code> – The agreement ended on the defined end date.</p>
            </li>
            <li>
               <p>
                  <code>RENEWED</code> – The agreement was renewed into a new agreement (for
               example, an auto-renewal).</p>
            </li>
            <li>
               <p>
                  <code>REPLACED</code> – The agreement was replaced using an agreement replacement
               offer.</p>
            </li>
            <li>
               <p>
                  <code>ROLLED_BACK</code> (Only applicable to inactive agreement revisions) – The
               agreement revision has been rolled back because of an error. An earlier revision is
               now active.</p>
            </li>
            <li>
               <p>
                  <code>SUPERCEDED</code> (Only applicable to inactive agreement revisions) – The
               agreement revision is no longer active and another agreement revision is now
               active.</p>
            </li>
            <li>
               <p>
                  <code>TERMINATED</code> – The agreement ended before the defined end date because
               of an AWS termination (for example, a payment failure).</p>
            </li>
         </ul> |
| `proposer` | String | <p>The details of the party proposing the agreement terms. This is commonly the seller for
            <code>PurchaseAgreement</code>.</p> |
| `agreement_id` | String | <p>The unique identifier of the agreement.</p> |
| `acceptor` | String | <p>The details of the party accepting the agreement terms. This is commonly the buyer for
            <code>PurchaseAgreement</code>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access agreement outputs
agreement_id = agreement.id
agreement_start_time = agreement.start_time
agreement_end_time = agreement.end_time
agreement_acceptance_time = agreement.acceptance_time
agreement_agreement_type = agreement.agreement_type
agreement_estimated_charges = agreement.estimated_charges
agreement_proposal_summary = agreement.proposal_summary
agreement_status = agreement.status
agreement_proposer = agreement.proposer
agreement_agreement_id = agreement.agreement_id
agreement_acceptor = agreement.acceptor
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple agreement_terms resources
agreement_terms_0 = provider.marketplace_agreement.Agreement_terms {
}
agreement_terms_1 = provider.marketplace_agreement.Agreement_terms {
}
agreement_terms_2 = provider.marketplace_agreement.Agreement_terms {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    agreement_terms = provider.marketplace_agreement.Agreement_terms {
    }
```

---

## Related Documentation

- [AWS Marketplace_agreement Documentation](https://docs.aws.amazon.com/marketplace_agreement/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
