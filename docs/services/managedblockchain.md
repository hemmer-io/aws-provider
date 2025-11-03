# Managedblockchain Service



**Resources**: 5

---

## Overview

The managedblockchain service provides access to 5 resource types:

- [Node](#node) [CRUD]
- [Network](#network) [CR]
- [Accessor](#accessor) [CRD]
- [Member](#member) [CRUD]
- [Proposal](#proposal) [CR]

---

## Resources


### Node

Node resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `network_id` | String | ✅ | <p>The unique identifier of the network for the node.</p>
         <p>Ethereum public networks have the following <code>NetworkId</code>s:</p>
         <ul>
            <li>
               <p>
                  <code>n-ethereum-mainnet</code>
               </p>
            </li>
         </ul> |
| `member_id` | String |  | <p>The unique identifier of the member that owns this node.</p>
         <p>Applies only to Hyperledger Fabric.</p> |
| `node_configuration` | String | ✅ | <p>The properties of a node configuration.</p> |
| `tags` | HashMap<String, String> |  | <p>Tags to assign to the node.</p>
         <p> Each tag consists of a key and an optional value. You can specify 
          multiple key-value pairs in a single request with an overall maximum of 50 tags 
          allowed per resource.</p>
         <p>For more information about tags, see <a href="https://docs.aws.amazon.com/managed-blockchain/latest/ethereum-dev/tagging-resources.html">Tagging Resources</a> in the <i>Amazon Managed Blockchain Ethereum Developer Guide</i>, or <a href="https://docs.aws.amazon.com/managed-blockchain/latest/hyperledger-fabric-dev/tagging-resources.html">Tagging Resources</a> in the <i>Amazon Managed Blockchain Hyperledger Fabric Developer Guide</i>.</p> |
| `client_request_token` | String | ✅ | <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the operation. An idempotent operation completes no more than one time. This identifier is required only if you make a service request directly using an HTTP client. It is generated automatically if you use an Amazon Web Services SDK or the CLI.</p> |


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
    network_id = "value"  # <p>The unique identifier of the network for the node.</p>
         <p>Ethereum public networks have the following <code>NetworkId</code>s:</p>
         <ul>
            <li>
               <p>
                  <code>n-ethereum-mainnet</code>
               </p>
            </li>
         </ul>
    node_configuration = "value"  # <p>The properties of a node configuration.</p>
    client_request_token = "value"  # <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the operation. An idempotent operation completes no more than one time. This identifier is required only if you make a service request directly using an HTTP client. It is generated automatically if you use an Amazon Web Services SDK or the CLI.</p>
}

# Access node outputs
node_id = node.id
node_node = node.node
```

---


### Network

Network resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | <p>An optional description for the network.</p> |
| `tags` | HashMap<String, String> |  | <p>Tags to assign to the network.</p>
         <p> Each tag consists of a key and an optional value. You can specify 
          multiple key-value pairs in a single request with an overall maximum of 50 tags 
          allowed per resource.</p>
         <p>For more information about tags, see <a href="https://docs.aws.amazon.com/managed-blockchain/latest/ethereum-dev/tagging-resources.html">Tagging Resources</a> in the <i>Amazon Managed Blockchain Ethereum Developer Guide</i>, or <a href="https://docs.aws.amazon.com/managed-blockchain/latest/hyperledger-fabric-dev/tagging-resources.html">Tagging Resources</a> in the <i>Amazon Managed Blockchain Hyperledger Fabric Developer Guide</i>.</p> |
| `framework` | String | ✅ | <p>The blockchain framework that the network uses.</p> |
| `member_configuration` | String | ✅ | <p>Configuration properties for the first member within the network.</p> |
| `framework_version` | String | ✅ | <p>The version of the blockchain framework that the network uses.</p> |
| `name` | String | ✅ | <p>The name of the network.</p> |
| `voting_policy` | String | ✅ | <p>
         The voting rules used by the network to determine if a proposal is approved.
      </p> |
| `client_request_token` | String | ✅ | <p>This is a unique, case-sensitive identifier that you provide to ensure the idempotency of the
         operation. An idempotent operation completes no more than once. This identifier is required only 
         if you make a service request directly using an HTTP client. It is generated automatically if you 
         use an Amazon Web Services SDK or the Amazon Web Services CLI.
      </p> |
| `framework_configuration` | String |  | <p>
         Configuration properties of the blockchain framework relevant to the network configuration.
      </p> |


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
    member_configuration = "value"  # <p>Configuration properties for the first member within the network.</p>
    framework_version = "value"  # <p>The version of the blockchain framework that the network uses.</p>
    name = "value"  # <p>The name of the network.</p>
    voting_policy = "value"  # <p>
         The voting rules used by the network to determine if a proposal is approved.
      </p>
    client_request_token = "value"  # <p>This is a unique, case-sensitive identifier that you provide to ensure the idempotency of the
         operation. An idempotent operation completes no more than once. This identifier is required only 
         if you make a service request directly using an HTTP client. It is generated automatically if you 
         use an Amazon Web Services SDK or the Amazon Web Services CLI.
      </p>
}

# Access network outputs
network_id = network.id
network_network = network.network
```

---


### Accessor

Accessor resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
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
| `client_request_token` | String | ✅ | <p>This is a unique, case-sensitive identifier that you provide to ensure the idempotency of 
         the operation. An idempotent operation completes no more than once. This 
         identifier is required only if you make a service request directly using 
         an HTTP client. It is generated automatically if you use an Amazon Web Services SDK or the 
         Amazon Web Services CLI.</p> |
| `tags` | HashMap<String, String> |  | <p>Tags to assign to the Accessor.</p>
         <p> Each tag consists of a key and an optional value. You can specify 
          multiple key-value pairs in a single request with an overall maximum of 50 tags 
          allowed per resource.</p>
         <p>For more information about tags, see <a href="https://docs.aws.amazon.com/managed-blockchain/latest/ethereum-dev/tagging-resources.html">Tagging Resources</a> in the <i>Amazon Managed Blockchain Ethereum Developer Guide</i>, or <a href="https://docs.aws.amazon.com/managed-blockchain/latest/hyperledger-fabric-dev/tagging-resources.html">Tagging Resources</a> in the <i>Amazon Managed Blockchain Hyperledger Fabric Developer Guide</i>.</p> |
| `accessor_type` | String | ✅ | <p>The type of accessor.</p>
         <note>
            <p>Currently, accessor type is restricted to <code>BILLING_TOKEN</code>.</p>
         </note> |


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
    client_request_token = "value"  # <p>This is a unique, case-sensitive identifier that you provide to ensure the idempotency of 
         the operation. An idempotent operation completes no more than once. This 
         identifier is required only if you make a service request directly using 
         an HTTP client. It is generated automatically if you use an Amazon Web Services SDK or the 
         Amazon Web Services CLI.</p>
    accessor_type = "value"  # <p>The type of accessor.</p>
         <note>
            <p>Currently, accessor type is restricted to <code>BILLING_TOKEN</code>.</p>
         </note>
}

# Access accessor outputs
accessor_id = accessor.id
accessor_accessor = accessor.accessor
```

---


### Member

Member resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `member_configuration` | String | ✅ | <p>Member configuration parameters.</p> |
| `client_request_token` | String | ✅ | <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the operation. An idempotent operation completes no more than one time. This identifier is required only if you make a service request directly using an HTTP client. It is generated automatically if you use an Amazon Web Services SDK or the CLI.</p> |
| `invitation_id` | String | ✅ | <p>The unique identifier of the invitation that is sent to the member to join the network.</p> |
| `network_id` | String | ✅ | <p>The unique identifier of the network in which the member is created.</p> |


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
    member_configuration = "value"  # <p>Member configuration parameters.</p>
    client_request_token = "value"  # <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the operation. An idempotent operation completes no more than one time. This identifier is required only if you make a service request directly using an HTTP client. It is generated automatically if you use an Amazon Web Services SDK or the CLI.</p>
    invitation_id = "value"  # <p>The unique identifier of the invitation that is sent to the member to join the network.</p>
    network_id = "value"  # <p>The unique identifier of the network in which the member is created.</p>
}

# Access member outputs
member_id = member.id
member_member = member.member
```

---


### Proposal

Proposal resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | HashMap<String, String> |  | <p>Tags to assign to the proposal.</p>
         <p> Each tag consists of a key and an optional value. You can specify 
          multiple key-value pairs in a single request with an overall maximum of 50 tags 
          allowed per resource.</p>
         <p>For more information about tags, see <a href="https://docs.aws.amazon.com/managed-blockchain/latest/ethereum-dev/tagging-resources.html">Tagging Resources</a> in the <i>Amazon Managed Blockchain Ethereum Developer Guide</i>, or <a href="https://docs.aws.amazon.com/managed-blockchain/latest/hyperledger-fabric-dev/tagging-resources.html">Tagging Resources</a> in the <i>Amazon Managed Blockchain Hyperledger Fabric Developer Guide</i>.</p> |
| `description` | String |  | <p>A description for the proposal that is visible to voting members, for example, "Proposal to add Example Corp. as member."</p> |
| `client_request_token` | String | ✅ | <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the operation. An idempotent operation completes no more than one time. This identifier is required only if you make a service request directly using an HTTP client. It is generated automatically if you use an Amazon Web Services SDK or the CLI.</p> |
| `member_id` | String | ✅ | <p>The unique identifier of the member that is creating the proposal. This 
         identifier is especially useful for identifying the member making the proposal 
         when multiple members exist in a single Amazon Web Services account.</p> |
| `network_id` | String | ✅ | <p>
         The unique identifier of the network for which the proposal is made.</p> |
| `actions` | String | ✅ | <p>The type of actions proposed, such as inviting a member or removing a member. The types of <code>Actions</code> in a proposal are mutually exclusive. For example, a proposal with <code>Invitations</code> actions cannot also contain <code>Removals</code> actions.</p> |


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
    client_request_token = "value"  # <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the operation. An idempotent operation completes no more than one time. This identifier is required only if you make a service request directly using an HTTP client. It is generated automatically if you use an Amazon Web Services SDK or the CLI.</p>
    member_id = "value"  # <p>The unique identifier of the member that is creating the proposal. This 
         identifier is especially useful for identifying the member making the proposal 
         when multiple members exist in a single Amazon Web Services account.</p>
    network_id = "value"  # <p>
         The unique identifier of the network for which the proposal is made.</p>
    actions = "value"  # <p>The type of actions proposed, such as inviting a member or removing a member. The types of <code>Actions</code> in a proposal are mutually exclusive. For example, a proposal with <code>Invitations</code> actions cannot also contain <code>Removals</code> actions.</p>
}

# Access proposal outputs
proposal_id = proposal.id
proposal_proposal = proposal.proposal
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple node resources
node_0 = provider.managedblockchain.Node {
    network_id = "value-0"
    node_configuration = "value-0"
    client_request_token = "value-0"
}
node_1 = provider.managedblockchain.Node {
    network_id = "value-1"
    node_configuration = "value-1"
    client_request_token = "value-1"
}
node_2 = provider.managedblockchain.Node {
    network_id = "value-2"
    node_configuration = "value-2"
    client_request_token = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    node = provider.managedblockchain.Node {
        network_id = "production-value"
        node_configuration = "production-value"
        client_request_token = "production-value"
    }
```

---

## Related Documentation

- [AWS Managedblockchain Documentation](https://docs.aws.amazon.com/managedblockchain/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
