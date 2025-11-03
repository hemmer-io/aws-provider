# Managedblockchain Service



**Resources**: 5

---

## Overview

The managedblockchain service provides access to 5 resource types:

- [Network](#network) [CR]
- [Node](#node) [CRUD]
- [Accessor](#accessor) [CRD]
- [Proposal](#proposal) [CR]
- [Member](#member) [CRUD]

---

## Resources


### Network

Network resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `framework` | String | ✅ | <p>The blockchain framework that the network uses.</p> |
| `framework_version` | String | ✅ | <p>The version of the blockchain framework that the network uses.</p> |
| `description` | String |  | <p>An optional description for the network.</p> |
| `framework_configuration` | String |  | <p>
         Configuration properties of the blockchain framework relevant to the network configuration.
      </p> |
| `voting_policy` | String | ✅ | <p>
         The voting rules used by the network to determine if a proposal is approved.
      </p> |
| `name` | String | ✅ | <p>The name of the network.</p> |
| `tags` | HashMap<String, String> |  | <p>Tags to assign to the network.</p>
         <p> Each tag consists of a key and an optional value. You can specify 
          multiple key-value pairs in a single request with an overall maximum of 50 tags 
          allowed per resource.</p>
         <p>For more information about tags, see <a href="https://docs.aws.amazon.com/managed-blockchain/latest/ethereum-dev/tagging-resources.html">Tagging Resources</a> in the <i>Amazon Managed Blockchain Ethereum Developer Guide</i>, or <a href="https://docs.aws.amazon.com/managed-blockchain/latest/hyperledger-fabric-dev/tagging-resources.html">Tagging Resources</a> in the <i>Amazon Managed Blockchain Hyperledger Fabric Developer Guide</i>.</p> |
| `client_request_token` | String | ✅ | <p>This is a unique, case-sensitive identifier that you provide to ensure the idempotency of the
         operation. An idempotent operation completes no more than once. This identifier is required only 
         if you make a service request directly using an HTTP client. It is generated automatically if you 
         use an Amazon Web Services SDK or the Amazon Web Services CLI.
      </p> |
| `member_configuration` | String | ✅ | <p>Configuration properties for the first member within the network.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `network` | String | <p>An object containing network configuration parameters.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create network
network = provider.managedblockchain.Network {
    framework = "value"  # <p>The blockchain framework that the network uses.</p>
    framework_version = "value"  # <p>The version of the blockchain framework that the network uses.</p>
    voting_policy = "value"  # <p>
         The voting rules used by the network to determine if a proposal is approved.
      </p>
    name = "value"  # <p>The name of the network.</p>
    client_request_token = "value"  # <p>This is a unique, case-sensitive identifier that you provide to ensure the idempotency of the
         operation. An idempotent operation completes no more than once. This identifier is required only 
         if you make a service request directly using an HTTP client. It is generated automatically if you 
         use an Amazon Web Services SDK or the Amazon Web Services CLI.
      </p>
    member_configuration = "value"  # <p>Configuration properties for the first member within the network.</p>
}

# Access network outputs
network_id = network.id
network_network = network.network
```

---


### Node

Node resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `member_id` | String |  | <p>The unique identifier of the member that owns this node.</p>
         <p>Applies only to Hyperledger Fabric.</p> |
| `node_configuration` | String | ✅ | <p>The properties of a node configuration.</p> |
| `client_request_token` | String | ✅ | <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the operation. An idempotent operation completes no more than one time. This identifier is required only if you make a service request directly using an HTTP client. It is generated automatically if you use an Amazon Web Services SDK or the CLI.</p> |
| `tags` | HashMap<String, String> |  | <p>Tags to assign to the node.</p>
         <p> Each tag consists of a key and an optional value. You can specify 
          multiple key-value pairs in a single request with an overall maximum of 50 tags 
          allowed per resource.</p>
         <p>For more information about tags, see <a href="https://docs.aws.amazon.com/managed-blockchain/latest/ethereum-dev/tagging-resources.html">Tagging Resources</a> in the <i>Amazon Managed Blockchain Ethereum Developer Guide</i>, or <a href="https://docs.aws.amazon.com/managed-blockchain/latest/hyperledger-fabric-dev/tagging-resources.html">Tagging Resources</a> in the <i>Amazon Managed Blockchain Hyperledger Fabric Developer Guide</i>.</p> |
| `network_id` | String | ✅ | <p>The unique identifier of the network for the node.</p>
         <p>Ethereum public networks have the following <code>NetworkId</code>s:</p>
         <ul>
            <li>
               <p>
                  <code>n-ethereum-mainnet</code>
               </p>
            </li>
         </ul> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `node` | String | <p>Properties of the node configuration.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create node
node = provider.managedblockchain.Node {
    node_configuration = "value"  # <p>The properties of a node configuration.</p>
    client_request_token = "value"  # <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the operation. An idempotent operation completes no more than one time. This identifier is required only if you make a service request directly using an HTTP client. It is generated automatically if you use an Amazon Web Services SDK or the CLI.</p>
    network_id = "value"  # <p>The unique identifier of the network for the node.</p>
         <p>Ethereum public networks have the following <code>NetworkId</code>s:</p>
         <ul>
            <li>
               <p>
                  <code>n-ethereum-mainnet</code>
               </p>
            </li>
         </ul>
}

# Access node outputs
node_id = node.id
node_node = node.node
```

---


### Accessor

Accessor resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | HashMap<String, String> |  | <p>Tags to assign to the Accessor.</p>
         <p> Each tag consists of a key and an optional value. You can specify 
          multiple key-value pairs in a single request with an overall maximum of 50 tags 
          allowed per resource.</p>
         <p>For more information about tags, see <a href="https://docs.aws.amazon.com/managed-blockchain/latest/ethereum-dev/tagging-resources.html">Tagging Resources</a> in the <i>Amazon Managed Blockchain Ethereum Developer Guide</i>, or <a href="https://docs.aws.amazon.com/managed-blockchain/latest/hyperledger-fabric-dev/tagging-resources.html">Tagging Resources</a> in the <i>Amazon Managed Blockchain Hyperledger Fabric Developer Guide</i>.</p> |
| `network_type` | String |  | <p>The blockchain network that the <code>Accessor</code> token is created for.</p>
         <note>
            <ul>
               <li>
                  <p>Use the actual <code>networkType</code> value for the blockchain network that you are creating 
            the <code>Accessor</code> token for.</p>
               </li>
               <li>
                  <p>With the shut down of the <i>Ethereum Goerli</i> and <i>Polygon Mumbai 
            Testnet</i> networks the following <code>networkType</code> values are no longer available 
            for selection and use.</p>
                  <ul>
                     <li>
                        <p>
                           <code>ETHEREUM_MAINNET_AND_GOERLI</code>
                        </p>
                     </li>
                     <li>
                        <p>
                           <code>ETHEREUM_GOERLI</code>
                        </p>
                     </li>
                     <li>
                        <p>
                           <code>POLYGON_MUMBAI</code>
                        </p>
                     </li>
                  </ul>
                  <p>However, your existing <code>Accessor</code> tokens with these <code>networkType</code> 
               values will remain unchanged.</p>
               </li>
            </ul>
         </note> |
| `accessor_type` | String | ✅ | <p>The type of accessor.</p>
         <note>
            <p>Currently, accessor type is restricted to <code>BILLING_TOKEN</code>.</p>
         </note> |
| `client_request_token` | String | ✅ | <p>This is a unique, case-sensitive identifier that you provide to ensure the idempotency of 
         the operation. An idempotent operation completes no more than once. This 
         identifier is required only if you make a service request directly using 
         an HTTP client. It is generated automatically if you use an Amazon Web Services SDK or the 
         Amazon Web Services CLI.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `accessor` | String | <p>The properties of the accessor.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create accessor
accessor = provider.managedblockchain.Accessor {
    accessor_type = "value"  # <p>The type of accessor.</p>
         <note>
            <p>Currently, accessor type is restricted to <code>BILLING_TOKEN</code>.</p>
         </note>
    client_request_token = "value"  # <p>This is a unique, case-sensitive identifier that you provide to ensure the idempotency of 
         the operation. An idempotent operation completes no more than once. This 
         identifier is required only if you make a service request directly using 
         an HTTP client. It is generated automatically if you use an Amazon Web Services SDK or the 
         Amazon Web Services CLI.</p>
}

# Access accessor outputs
accessor_id = accessor.id
accessor_accessor = accessor.accessor
```

---


### Proposal

Proposal resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `member_id` | String | ✅ | <p>The unique identifier of the member that is creating the proposal. This 
         identifier is especially useful for identifying the member making the proposal 
         when multiple members exist in a single Amazon Web Services account.</p> |
| `network_id` | String | ✅ | <p>
         The unique identifier of the network for which the proposal is made.</p> |
| `actions` | String | ✅ | <p>The type of actions proposed, such as inviting a member or removing a member. The types of <code>Actions</code> in a proposal are mutually exclusive. For example, a proposal with <code>Invitations</code> actions cannot also contain <code>Removals</code> actions.</p> |
| `client_request_token` | String | ✅ | <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the operation. An idempotent operation completes no more than one time. This identifier is required only if you make a service request directly using an HTTP client. It is generated automatically if you use an Amazon Web Services SDK or the CLI.</p> |
| `description` | String |  | <p>A description for the proposal that is visible to voting members, for example, "Proposal to add Example Corp. as member."</p> |
| `tags` | HashMap<String, String> |  | <p>Tags to assign to the proposal.</p>
         <p> Each tag consists of a key and an optional value. You can specify 
          multiple key-value pairs in a single request with an overall maximum of 50 tags 
          allowed per resource.</p>
         <p>For more information about tags, see <a href="https://docs.aws.amazon.com/managed-blockchain/latest/ethereum-dev/tagging-resources.html">Tagging Resources</a> in the <i>Amazon Managed Blockchain Ethereum Developer Guide</i>, or <a href="https://docs.aws.amazon.com/managed-blockchain/latest/hyperledger-fabric-dev/tagging-resources.html">Tagging Resources</a> in the <i>Amazon Managed Blockchain Hyperledger Fabric Developer Guide</i>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `proposal` | String | <p>Information about a proposal.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create proposal
proposal = provider.managedblockchain.Proposal {
    member_id = "value"  # <p>The unique identifier of the member that is creating the proposal. This 
         identifier is especially useful for identifying the member making the proposal 
         when multiple members exist in a single Amazon Web Services account.</p>
    network_id = "value"  # <p>
         The unique identifier of the network for which the proposal is made.</p>
    actions = "value"  # <p>The type of actions proposed, such as inviting a member or removing a member. The types of <code>Actions</code> in a proposal are mutually exclusive. For example, a proposal with <code>Invitations</code> actions cannot also contain <code>Removals</code> actions.</p>
    client_request_token = "value"  # <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the operation. An idempotent operation completes no more than one time. This identifier is required only if you make a service request directly using an HTTP client. It is generated automatically if you use an Amazon Web Services SDK or the CLI.</p>
}

# Access proposal outputs
proposal_id = proposal.id
proposal_proposal = proposal.proposal
```

---


### Member

Member resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `network_id` | String | ✅ | <p>The unique identifier of the network in which the member is created.</p> |
| `member_configuration` | String | ✅ | <p>Member configuration parameters.</p> |
| `client_request_token` | String | ✅ | <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the operation. An idempotent operation completes no more than one time. This identifier is required only if you make a service request directly using an HTTP client. It is generated automatically if you use an Amazon Web Services SDK or the CLI.</p> |
| `invitation_id` | String | ✅ | <p>The unique identifier of the invitation that is sent to the member to join the network.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `member` | String | <p>The properties of a member.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create member
member = provider.managedblockchain.Member {
    network_id = "value"  # <p>The unique identifier of the network in which the member is created.</p>
    member_configuration = "value"  # <p>Member configuration parameters.</p>
    client_request_token = "value"  # <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the operation. An idempotent operation completes no more than one time. This identifier is required only if you make a service request directly using an HTTP client. It is generated automatically if you use an Amazon Web Services SDK or the CLI.</p>
    invitation_id = "value"  # <p>The unique identifier of the invitation that is sent to the member to join the network.</p>
}

# Access member outputs
member_id = member.id
member_member = member.member
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple network resources
network_0 = provider.managedblockchain.Network {
    framework = "value-0"
    framework_version = "value-0"
    voting_policy = "value-0"
    name = "value-0"
    client_request_token = "value-0"
    member_configuration = "value-0"
}
network_1 = provider.managedblockchain.Network {
    framework = "value-1"
    framework_version = "value-1"
    voting_policy = "value-1"
    name = "value-1"
    client_request_token = "value-1"
    member_configuration = "value-1"
}
network_2 = provider.managedblockchain.Network {
    framework = "value-2"
    framework_version = "value-2"
    voting_policy = "value-2"
    name = "value-2"
    client_request_token = "value-2"
    member_configuration = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    network = provider.managedblockchain.Network {
        framework = "production-value"
        framework_version = "production-value"
        voting_policy = "production-value"
        name = "production-value"
        client_request_token = "production-value"
        member_configuration = "production-value"
    }
```

---

## Related Documentation

- [AWS Managedblockchain Documentation](https://docs.aws.amazon.com/managedblockchain/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
