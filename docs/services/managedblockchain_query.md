# Managedblockchain_query Service



**Resources**: 3

---

## Overview

The managedblockchain_query service provides access to 3 resource types:

- [Asset_contract](#asset_contract) [R]
- [Transaction](#transaction) [R]
- [Token_balance](#token_balance) [R]

---

## Resources


### Asset_contract

AssetContract resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `metadata` | String |  |
| `contract_identifier` | String | <p>Contains the blockchain address and network information about the contract.</p> |
| `token_standard` | String | <p>The token standard of the contract requested.</p> |
| `deployer_address` | String | <p>The address of the deployer of contract.</p> |


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
asset_contract_metadata = asset_contract.metadata
asset_contract_contract_identifier = asset_contract.contract_identifier
asset_contract_token_standard = asset_contract.token_standard
asset_contract_deployer_address = asset_contract.deployer_address
```

---


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


### Token_balance

TokenBalance resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `token_identifier` | String |  |
| `at_blockchain_instant` | String |  |
| `last_updated_time` | String |  |
| `balance` | String | <p>The container for the token balance.</p> |
| `owner_identifier` | String |  |


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
token_balance_token_identifier = token_balance.token_identifier
token_balance_at_blockchain_instant = token_balance.at_blockchain_instant
token_balance_last_updated_time = token_balance.last_updated_time
token_balance_balance = token_balance.balance
token_balance_owner_identifier = token_balance.owner_identifier
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple asset_contract resources
asset_contract_0 = provider.managedblockchain_query.Asset_contract {
}
asset_contract_1 = provider.managedblockchain_query.Asset_contract {
}
asset_contract_2 = provider.managedblockchain_query.Asset_contract {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    asset_contract = provider.managedblockchain_query.Asset_contract {
    }
```

---

## Related Documentation

- [AWS Managedblockchain_query Documentation](https://docs.aws.amazon.com/managedblockchain_query/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
