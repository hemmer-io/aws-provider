# Managedblockchain_query Service



**Resources**: 3

---

## Overview

The managedblockchain_query service provides access to 3 resource types:

- [Transaction](#transaction) [R]
- [Asset_contract](#asset_contract) [R]
- [Token_balance](#token_balance) [R]

---

## Resources


### Transaction

Transaction resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `transaction` | String | <p>Contains the details of the transaction.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access transaction outputs
transaction_id = transaction.id
transaction_transaction = transaction.transaction
```

---


### Asset_contract

AssetContract resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `deployer_address` | String | <p>The address of the deployer of contract.</p> |
| `token_standard` | String | <p>The token standard of the contract requested.</p> |
| `metadata` | String |  |
| `contract_identifier` | String | <p>Contains the blockchain address and network information about the contract.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access asset_contract outputs
asset_contract_id = asset_contract.id
asset_contract_deployer_address = asset_contract.deployer_address
asset_contract_token_standard = asset_contract.token_standard
asset_contract_metadata = asset_contract.metadata
asset_contract_contract_identifier = asset_contract.contract_identifier
```

---


### Token_balance

TokenBalance resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `at_blockchain_instant` | String |  |
| `token_identifier` | String |  |
| `owner_identifier` | String |  |
| `last_updated_time` | String |  |
| `balance` | String | <p>The container for the token balance.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access token_balance outputs
token_balance_id = token_balance.id
token_balance_at_blockchain_instant = token_balance.at_blockchain_instant
token_balance_token_identifier = token_balance.token_identifier
token_balance_owner_identifier = token_balance.owner_identifier
token_balance_last_updated_time = token_balance.last_updated_time
token_balance_balance = token_balance.balance
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple transaction resources
transaction_0 = provider.managedblockchain_query.Transaction {
}
transaction_1 = provider.managedblockchain_query.Transaction {
}
transaction_2 = provider.managedblockchain_query.Transaction {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    transaction = provider.managedblockchain_query.Transaction {
    }
```

---

## Related Documentation

- [AWS Managedblockchain_query Documentation](https://docs.aws.amazon.com/managedblockchain_query/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
