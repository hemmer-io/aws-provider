# Kms Service



**Resources**: 12

---

## Overview

The kms service provides access to 12 resource types:

- [Custom_key_stores](#custom_key_stores) [R]
- [Key](#key) [CR]
- [Parameters_for_import](#parameters_for_import) [R]
- [Primary_region](#primary_region) [U]
- [Custom_key_store](#custom_key_store) [CUD]
- [Imported_key_material](#imported_key_material) [D]
- [Grant](#grant) [C]
- [Alias](#alias) [CUD]
- [Key_rotation_status](#key_rotation_status) [R]
- [Key_policy](#key_policy) [CR]
- [Public_key](#public_key) [R]
- [Key_description](#key_description) [U]

---

## Resources


### Custom_key_stores

CustomKeyStores resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `truncated` | bool | <p>A flag that indicates whether there are more items in the list. When this
    value is true, the list in this response is truncated. To get more items, pass the value of
    the <code>NextMarker</code> element in this response to the <code>Marker</code> parameter in a
    subsequent request.</p> |
| `custom_key_stores` | Vec<String> | <p>Contains metadata about each custom key store.</p> |
| `next_marker` | String | <p>When <code>Truncated</code> is true, this element is present and contains the
    value to use for the <code>Marker</code> parameter in a subsequent request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access custom_key_stores outputs
custom_key_stores_id = custom_key_stores.id
custom_key_stores_truncated = custom_key_stores.truncated
custom_key_stores_custom_key_stores = custom_key_stores.custom_key_stores
custom_key_stores_next_marker = custom_key_stores.next_marker
```

---


### Key

Key resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `key_spec` | String |  | <p>Specifies the type of KMS key to create. The default value,
      <code>SYMMETRIC_DEFAULT</code>, creates a KMS key with a 256-bit AES-GCM key that is used for
      encryption and decryption, except in China Regions, where it creates a 128-bit symmetric key
      that uses SM4 encryption. For a detailed description of all supported key specs, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symm-asymm-choose-key-spec.html">Key spec
        reference</a> in the <i>
               <i>Key Management Service Developer Guide</i>
            </i>.</p>
         <p>The <code>KeySpec</code> determines whether the KMS key contains a symmetric key or an
      asymmetric key pair. It also determines the algorithms that the KMS key supports. You can't
      change the <code>KeySpec</code> after the KMS key is created. To further restrict the
      algorithms that can be used with the KMS key, use a condition key in its key policy or IAM
      policy. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/conditions-kms.html#conditions-kms-encryption-algorithm">kms:EncryptionAlgorithm</a>, <a href="https://docs.aws.amazon.com/kms/latest/developerguide/conditions-kms.html#conditions-kms-mac-algorithm">kms:MacAlgorithm</a>, <a href="https://docs.aws.amazon.com/kms/latest/developerguide/conditions-kms.html#conditions-kms-key-agreement-algorithm">kms:KeyAgreementAlgorithm</a>, or <a href="https://docs.aws.amazon.com/kms/latest/developerguide/conditions-kms.html#conditions-kms-signing-algorithm">kms:SigningAlgorithm</a> in the <i>
               <i>Key Management Service Developer Guide</i>
            </i>.</p>
         <important>
            <p>
               <a href="http://aws.amazon.com/kms/features/#AWS_Service_Integration">Amazon Web Services services that
          are integrated with KMS</a> use symmetric encryption KMS keys to protect your data.
        These services do not support asymmetric KMS keys or HMAC KMS keys.</p>
         </important>
         <p>KMS supports the following key specs for KMS keys:</p>
         <ul>
            <li>
               <p>Symmetric encryption key (default)</p>
               <ul>
                  <li>
                     <p>
                        <code>SYMMETRIC_DEFAULT</code>
                     </p>
                  </li>
               </ul>
            </li>
            <li>
               <p>HMAC keys (symmetric)</p>
               <ul>
                  <li>
                     <p>
                        <code>HMAC_224</code>
                     </p>
                  </li>
                  <li>
                     <p>
                        <code>HMAC_256</code>
                     </p>
                  </li>
                  <li>
                     <p>
                        <code>HMAC_384</code>
                     </p>
                  </li>
                  <li>
                     <p>
                        <code>HMAC_512</code>
                     </p>
                  </li>
               </ul>
            </li>
            <li>
               <p>Asymmetric RSA key pairs (encryption and decryption -or- signing and
          verification)</p>
               <ul>
                  <li>
                     <p>
                        <code>RSA_2048</code>
                     </p>
                  </li>
                  <li>
                     <p>
                        <code>RSA_3072</code>
                     </p>
                  </li>
                  <li>
                     <p>
                        <code>RSA_4096</code>
                     </p>
                  </li>
               </ul>
            </li>
            <li>
               <p>Asymmetric NIST-recommended elliptic curve key pairs (signing and verification -or-
          deriving shared secrets)</p>
               <ul>
                  <li>
                     <p>
                        <code>ECC_NIST_P256</code> (secp256r1)</p>
                  </li>
                  <li>
                     <p>
                        <code>ECC_NIST_P384</code> (secp384r1)</p>
                  </li>
                  <li>
                     <p>
                        <code>ECC_NIST_P521</code> (secp521r1)</p>
                  </li>
               </ul>
            </li>
            <li>
               <p>Other asymmetric elliptic curve key pairs (signing and verification)</p>
               <ul>
                  <li>
                     <p>
                        <code>ECC_SECG_P256K1</code> (secp256k1), commonly used for
              cryptocurrencies.</p>
                  </li>
               </ul>
            </li>
            <li>
               <p>Asymmetric ML-DSA key pairs (signing and verification)</p>
               <ul>
                  <li>
                     <p>
                        <code>ML_DSA_44</code>
                     </p>
                  </li>
                  <li>
                     <p>
                        <code>ML_DSA_65</code>
                     </p>
                  </li>
                  <li>
                     <p>
                        <code>ML_DSA_87</code>
                     </p>
                  </li>
               </ul>
            </li>
            <li>
               <p>SM2 key pairs (encryption and decryption -or- signing and verification -or- deriving
          shared secrets)</p>
               <ul>
                  <li>
                     <p>
                        <code>SM2</code> (China Regions only)</p>
                  </li>
               </ul>
            </li>
         </ul> |
| `policy` | String |  | <p>The key policy to attach to the KMS key.</p>
         <p>If you provide a key policy, it must meet the following criteria:</p>
         <ul>
            <li>
               <p>The key policy must allow the calling principal to make a
          subsequent <code>PutKeyPolicy</code> request on the KMS key.  This reduces the risk that
          the KMS key becomes unmanageable. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policy-default.html#prevent-unmanageable-key">Default key policy</a> in the <i>Key Management Service Developer Guide</i>. (To omit
          this condition, set <code>BypassPolicyLockoutSafetyCheck</code> to true.)</p>
            </li>
            <li>
               <p>Each statement in the key policy must contain one or more principals. The principals
          in the key policy must exist and be visible to KMS. When you create a new Amazon Web Services
          principal, you might need to enforce a delay before including the new principal in a key
          policy because the new principal might not be immediately visible to KMS. For more
          information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/troubleshoot_general.html#troubleshoot_general_eventual-consistency">Changes that I make are not always immediately visible</a> in the <i>Amazon Web Services
            Identity and Access Management User Guide</i>.</p>
            </li>
         </ul>
         <note>
            <p>If either of the required <code>Resource</code> or <code>Action</code> elements are
        missing from a key policy statement, the policy statement has no effect. When a key policy
        statement is missing one of these elements, the KMS console correctly reports an error,
        but the <code>CreateKey</code> and <code>PutKeyPolicy</code> API requests succeed, even
        though the policy statement is ineffective.</p>
            <p>For more information on required key policy elements, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policy-overview.html#key-policy-elements">Elements in a key
          policy</a> in the <i>Key Management Service Developer Guide</i>.</p>
         </note>
         <p>If you do not provide a key policy, KMS attaches a default key policy to the KMS key.
      For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policy-default.html">Default key policy</a> in the
      <i>Key Management Service Developer Guide</i>. </p>
         <note>
            <p>If the key policy exceeds the length constraint, KMS returns a
          <code>LimitExceededException</code>.</p>
         </note>
         <p>For help writing and formatting a JSON policy document, see the <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies.html">IAM JSON Policy Reference</a> in the <i>
               <i>Identity and Access Management User Guide</i>
            </i>.</p> |
| `bypass_policy_lockout_safety_check` | bool |  | <p>Skips ("bypasses") the key policy lockout safety check. The default value is false.</p>
         <important>
            <p>Setting this value to true increases the risk that the KMS key becomes unmanageable. Do
        not set this value to true indiscriminately.</p>
            <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policy-default.html#prevent-unmanageable-key">Default key policy</a> in the <i>Key Management Service Developer Guide</i>.</p>
         </important>
         <p>Use this parameter only when you intend to prevent the principal that is making the
      request from making a subsequent <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_PutKeyPolicy.html">PutKeyPolicy</a>
      request on the KMS key.</p> |
| `tags` | Vec<String> |  | <p>Assigns one or more tags to the KMS key. Use this parameter to tag the KMS key when it is
      created. To tag an existing KMS key, use the <a>TagResource</a> operation.</p>
         <important>
            <p>Do not include confidential or sensitive information in this field. This field may be displayed in plaintext in CloudTrail logs and other output.</p>
         </important>
         <note>
            <p>Tagging or untagging a KMS key can allow or deny permission to the KMS key. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/abac.html">ABAC for KMS</a> in the <i>Key Management Service Developer Guide</i>.</p>
         </note>
         <p>To use this parameter, you must have <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:TagResource</a> permission in an IAM policy.</p>
         <p>Each tag consists of a tag key and a tag value. Both the tag key and the tag value are
      required, but the tag value can be an empty (null) string. You cannot have more than one tag
      on a KMS key with the same tag key. If you specify an existing tag key with a different tag
      value, KMS replaces the current tag value with the specified one.</p>
         <p>When you add tags to an Amazon Web Services resource, Amazon Web Services generates a cost allocation
              report with usage and costs aggregated by tags. Tags can also be used to control access to a KMS key. For details,
              see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/tagging-keys.html">Tags in KMS</a>.</p> |
| `multi_region` | bool |  | <p>Creates a multi-Region primary key that you can replicate into other Amazon Web Services Regions. You
      cannot change this value after you create the KMS key. </p>
         <p>For a multi-Region key, set this parameter to <code>True</code>. For a single-Region KMS
      key, omit this parameter or set it to <code>False</code>. The default value is
        <code>False</code>.</p>
         <p>This operation supports <i>multi-Region keys</i>, an KMS feature that lets you create multiple
      interoperable KMS keys in different Amazon Web Services Regions. Because these KMS keys have the same key ID, key
      material, and other metadata, you can use them interchangeably to encrypt data in one Amazon Web Services Region and decrypt
      it in a different Amazon Web Services Region without re-encrypting the data or making a cross-Region call. For more information about multi-Region keys, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/multi-region-keys-overview.html">Multi-Region keys in KMS</a> in the <i>Key Management Service Developer Guide</i>.</p>
         <p>This value creates a <i>primary key</i>, not a replica. To create a
        <i>replica key</i>, use the <a>ReplicateKey</a> operation. </p>
         <p>You can create a symmetric or asymmetric multi-Region key, and you can create a
      multi-Region key with imported key material. However, you cannot create a multi-Region key in
      a custom key store.</p> |
| `description` | String |  | <p>A description of the KMS key. Use a description that helps you decide whether the KMS key
      is appropriate for a task. The default value is an empty string (no description).</p>
         <important>
            <p>Do not include confidential or sensitive information in this field. This field may be displayed in plaintext in CloudTrail logs and other output.</p>
         </important>
         <p>To set or change the description after the key is created, use <a>UpdateKeyDescription</a>.</p> |
| `origin` | String |  | <p>The source of the key material for the KMS key. You cannot change the origin after you
      create the KMS key. The default is <code>AWS_KMS</code>, which means that KMS creates the
      key material.</p>
         <p>To <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys-create-cmk.html">create a
        KMS key with no key material</a> (for imported key material), set this value to
        <code>EXTERNAL</code>. For more information about importing key material into KMS, see
        <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html">Importing Key
        Material</a> in the <i>Key Management Service Developer Guide</i>. The <code>EXTERNAL</code> origin value is valid
      only for symmetric KMS keys.</p>
         <p>To <a href="https://docs.aws.amazon.com/kms/latest/developerguide/create-cmk-keystore.html">create a KMS
        key in an CloudHSM key store</a> and create its key material in the associated CloudHSM
      cluster, set this value to <code>AWS_CLOUDHSM</code>. You must also use the
        <code>CustomKeyStoreId</code> parameter to identify the CloudHSM key store. The
        <code>KeySpec</code> value must be <code>SYMMETRIC_DEFAULT</code>.</p>
         <p>To <a href="https://docs.aws.amazon.com/kms/latest/developerguide/create-xks-keys.html">create a KMS key in
        an external key store</a>, set this value to <code>EXTERNAL_KEY_STORE</code>. You must
      also use the <code>CustomKeyStoreId</code> parameter to identify the external key store and
      the <code>XksKeyId</code> parameter to identify the associated external key. The
        <code>KeySpec</code> value must be <code>SYMMETRIC_DEFAULT</code>.</p> |
| `key_usage` | String |  | <p>Determines the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-cryptography.html#cryptographic-operations">cryptographic operations</a> for which you can use the KMS key. The default value is
        <code>ENCRYPT_DECRYPT</code>. This parameter is optional when you are creating a symmetric
      encryption KMS key; otherwise, it is required. You can't change the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/create-keys.html#key-usage">
               <code>KeyUsage</code>
            </a> value after the KMS key is created. Each KMS key can have
      only one key usage. This follows key usage best practices according to <a href="https://csrc.nist.gov/pubs/sp/800/57/pt1/r5/final">NIST SP 800-57 Recommendations for
        Key Management</a>, section 5.2, Key usage.</p>
         <p>Select only one valid value.</p>
         <ul>
            <li>
               <p>For symmetric encryption KMS keys, omit the parameter or specify
            <code>ENCRYPT_DECRYPT</code>.</p>
            </li>
            <li>
               <p>For HMAC KMS keys (symmetric), specify <code>GENERATE_VERIFY_MAC</code>.</p>
            </li>
            <li>
               <p>For asymmetric KMS keys with RSA key pairs, specify <code>ENCRYPT_DECRYPT</code> or
            <code>SIGN_VERIFY</code>.</p>
            </li>
            <li>
               <p>For asymmetric KMS keys with NIST-recommended elliptic curve key pairs, specify
            <code>SIGN_VERIFY</code> or <code>KEY_AGREEMENT</code>.</p>
            </li>
            <li>
               <p>For asymmetric KMS keys with <code>ECC_SECG_P256K1</code> key pairs, specify
            <code>SIGN_VERIFY</code>.</p>
            </li>
            <li>
               <p>For asymmetric KMS keys with ML-DSA key pairs, specify
          <code>SIGN_VERIFY</code>.</p>
            </li>
            <li>
               <p>For asymmetric KMS keys with SM2 key pairs (China Regions only), specify
            <code>ENCRYPT_DECRYPT</code>, <code>SIGN_VERIFY</code>, or
          <code>KEY_AGREEMENT</code>.</p>
            </li>
         </ul> |
| `xks_key_id` | String |  | <p>Identifies the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/keystore-external.html#concept-external-key">external key</a> that
      serves as key material for the KMS key in an <a href="https://docs.aws.amazon.com/kms/latest/developerguide/keystore-external.html">external key store</a>. Specify the ID that
      the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/keystore-external.html#concept-xks-proxy">external key store proxy</a> uses to refer to the external key. For help, see the
      documentation for your external key store proxy.</p>
         <p>This parameter is required for a KMS key with an <code>Origin</code> value of
        <code>EXTERNAL_KEY_STORE</code>. It is not valid for KMS keys with any other
        <code>Origin</code> value.</p>
         <p>The external key must be an existing 256-bit AES symmetric encryption key hosted outside
      of Amazon Web Services in an external key manager associated with the external key store specified by the
        <code>CustomKeyStoreId</code> parameter. This key must be enabled and configured to perform
      encryption and decryption. Each KMS key in an external key store must use a different external
      key. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/create-xks-keys.html#xks-key-requirements">Requirements for a KMS key in
        an external key store</a> in the <i>Key Management Service Developer Guide</i>.</p>
         <p>Each KMS key in an external key store is associated two backing keys. One is key material
      that KMS generates. The other is the external key specified by this parameter. When you use
      the KMS key in an external key store to encrypt data, the encryption operation is performed
      first by KMS using the KMS key material, and then by the external key manager using the
      specified external key, a process known as <i>double encryption</i>. For
      details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/keystore-external.html#concept-double-encryption">Double
        encryption</a> in the <i>Key Management Service Developer Guide</i>.</p> |
| `custom_key_store_id` | String |  | <p>Creates the KMS key in the specified <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-store-overview.html">custom key store</a>. The <code>ConnectionState</code> of
      the custom key store must be <code>CONNECTED</code>. To find the CustomKeyStoreID and
      ConnectionState use the <a>DescribeCustomKeyStores</a> operation.</p>
         <p>This parameter is valid only for symmetric encryption KMS keys in a single Region. You
      cannot create any other type of KMS key in a custom key store.</p>
         <p>When you create a KMS key in an CloudHSM key store, KMS generates a non-exportable 256-bit
      symmetric key in its associated CloudHSM cluster and associates it with the KMS key. When you
      create a KMS key in an external key store, you must use the <code>XksKeyId</code> parameter to
      specify an external key that serves as key material for the KMS key.</p> |
| `customer_master_key_spec` | String |  | <p>Instead, use the <code>KeySpec</code> parameter.</p>
         <p>The <code>KeySpec</code> and <code>CustomerMasterKeySpec</code> parameters work the same
      way. Only the names differ. We recommend that you use <code>KeySpec</code> parameter in your
      code. However, to avoid breaking changes, KMS supports both parameters.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `key_metadata` | String | <p>Metadata associated with the key.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create key
key = provider.kms.Key {
}

# Access key outputs
key_id = key.id
key_key_metadata = key.key_metadata
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
| `import_token` | String | <p>The import token to send in a subsequent <a>ImportKeyMaterial</a>
      request.</p> |
| `public_key` | String | <p>The public key to use to encrypt the key material before importing it with <a>ImportKeyMaterial</a>.</p> |
| `parameters_valid_to` | String | <p>The time at which the import token and public key are no longer valid. After this time,
      you cannot use them to make an <a>ImportKeyMaterial</a> request and you must send
      another <code>GetParametersForImport</code> request to get new ones.</p> |
| `key_id` | String | <p>The Amazon Resource Name (<a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#key-id-key-ARN">key ARN</a>) of the KMS key to use in a subsequent <a>ImportKeyMaterial</a> request. This is the same KMS key specified in the <code>GetParametersForImport</code>
      request.</p> |


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
parameters_for_import_public_key = parameters_for_import.public_key
parameters_for_import_parameters_valid_to = parameters_for_import.parameters_valid_to
parameters_for_import_key_id = parameters_for_import.key_id
```

---


### Primary_region

PrimaryRegion resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `key_id` | String | ✅ | <p>Identifies the current primary key. When the operation completes, this KMS key will be a
      replica key.</p>
         <p>Specify the key ID or key ARN of a multi-Region primary key.</p>
         <p>For example:</p>
         <ul>
            <li>
               <p>Key ID: <code>mrk-1234abcd12ab34cd56ef1234567890ab</code>
               </p>
            </li>
            <li>
               <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/mrk-1234abcd12ab34cd56ef1234567890ab</code>
               </p>
            </li>
         </ul>
         <p>To get the key ID and key ARN for a KMS key, use <a>ListKeys</a> or <a>DescribeKey</a>.</p> |
| `primary_region` | String | ✅ | <p>The Amazon Web Services Region of the new primary key. Enter the Region ID, such as
        <code>us-east-1</code> or <code>ap-southeast-2</code>. There must be an existing replica key
      in this Region. </p>
         <p>When the operation completes, the multi-Region key in this Region will be the primary
      key.</p> |



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


### Custom_key_store

CustomKeyStore resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `xks_proxy_uri_path` | String |  | <p>Specifies the base path to the proxy APIs for this external key store. To find this value,
      see the documentation for your external key store proxy. This parameter is required for all
      custom key stores with a <code>CustomKeyStoreType</code> of
      <code>EXTERNAL_KEY_STORE</code>.</p>
         <p>The value must start with <code>/</code> and must end with <code>/kms/xks/v1</code> where
        <code>v1</code> represents the version of the KMS external key store proxy API. This path
      can include an optional prefix between the required elements such as
          <code>/<i>prefix</i>/kms/xks/v1</code>.</p>
         <p>
            <b>Uniqueness requirements: </b>
         </p>
         <ul>
            <li>
               <p>The combined <code>XksProxyUriEndpoint</code> and <code>XksProxyUriPath</code> values
          must be unique in the Amazon Web Services account and Region.</p>
            </li>
         </ul> |
| `xks_proxy_vpc_endpoint_service_owner` | String |  | <p>Specifies the Amazon Web Services account ID that owns the Amazon VPC service endpoint for the interface that
      is used to communicate with your external key store proxy (XKS proxy). This parameter is
      optional. If not provided, the Amazon Web Services account ID calling the action will be used.</p> |
| `key_store_password` | String |  | <p>Specifies the <code>kmsuser</code> password for an CloudHSM key store. This parameter is
      required for custom key stores with a <code>CustomKeyStoreType</code> of
        <code>AWS_CLOUDHSM</code>.</p>
         <p>Enter the password of the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/keystore-cloudhsm.html#concept-kmsuser">
               <code>kmsuser</code> crypto user
        (CU) account</a> in the specified CloudHSM cluster. KMS logs into the cluster as this
      user to manage key material on your behalf.</p>
         <p>The password must be a string of 7 to 32 characters. Its value is case sensitive.</p>
         <p>This parameter tells KMS the <code>kmsuser</code> account password; it does not change
      the password in the CloudHSM cluster.</p> |
| `cloud_hsm_cluster_id` | String |  | <p>Identifies the CloudHSM cluster for an CloudHSM key store. This parameter is required for custom
      key stores with <code>CustomKeyStoreType</code> of <code>AWS_CLOUDHSM</code>.</p>
         <p>Enter the cluster ID of any active CloudHSM cluster that is not already associated with a
      custom key store. To find the cluster ID, use the <a href="https://docs.aws.amazon.com/cloudhsm/latest/APIReference/API_DescribeClusters.html">DescribeClusters</a> operation.</p> |
| `custom_key_store_type` | String |  | <p>Specifies the type of custom key store. The default value is
      <code>AWS_CLOUDHSM</code>.</p>
         <p>For a custom key store backed by an CloudHSM cluster, omit the parameter or enter
        <code>AWS_CLOUDHSM</code>. For a custom key store backed by an external key manager outside
      of Amazon Web Services, enter <code>EXTERNAL_KEY_STORE</code>. You cannot change this property after the key
      store is created.</p> |
| `trust_anchor_certificate` | String |  | <p>Specifies the certificate for an CloudHSM key store. This parameter is required for custom
      key stores with a <code>CustomKeyStoreType</code> of <code>AWS_CLOUDHSM</code>.</p>
         <p>Enter the content of the trust anchor certificate for the CloudHSM cluster. This is the
      content of the <code>customerCA.crt</code> file that you created when you <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/initialize-cluster.html">initialized the
        cluster</a>.</p> |
| `xks_proxy_uri_endpoint` | String |  | <p>Specifies the endpoint that KMS uses to send requests to the external key store proxy
      (XKS proxy). This parameter is required for custom key stores with a
        <code>CustomKeyStoreType</code> of <code>EXTERNAL_KEY_STORE</code>.</p>
         <p>The protocol must be HTTPS. KMS communicates on port 443. Do not specify the port in the
        <code>XksProxyUriEndpoint</code> value.</p>
         <p>For external key stores with <code>XksProxyConnectivity</code> value of
        <code>VPC_ENDPOINT_SERVICE</code>, specify <code>https://</code> followed by the private DNS
      name of the VPC endpoint service.</p>
         <p>For external key stores with <code>PUBLIC_ENDPOINT</code> connectivity, this endpoint must
      be reachable before you create the custom key store. KMS connects to the external key store
      proxy while creating the custom key store. For external key stores with
        <code>VPC_ENDPOINT_SERVICE</code> connectivity, KMS connects when you call the <a>ConnectCustomKeyStore</a> operation.</p>
         <p>The value of this parameter must begin with <code>https://</code>. The remainder can
      contain upper and lower case letters (A-Z and a-z), numbers (0-9), dots (<code>.</code>), and
      hyphens (<code>-</code>). Additional slashes (<code>/</code> and <code>\</code>) are not
      permitted.</p>
         <p>
            <b>Uniqueness requirements: </b>
         </p>
         <ul>
            <li>
               <p>The combined <code>XksProxyUriEndpoint</code> and <code>XksProxyUriPath</code> values
          must be unique in the Amazon Web Services account and Region.</p>
            </li>
            <li>
               <p>An external key store with <code>PUBLIC_ENDPOINT</code> connectivity cannot use the
          same <code>XksProxyUriEndpoint</code> value as an external key store with
            <code>VPC_ENDPOINT_SERVICE</code> connectivity in this Amazon Web Services Region.</p>
            </li>
            <li>
               <p>Each external key store with <code>VPC_ENDPOINT_SERVICE</code> connectivity must have
          its own private DNS name. The <code>XksProxyUriEndpoint</code> value for external key
          stores with <code>VPC_ENDPOINT_SERVICE</code> connectivity (private DNS name) must be
          unique in the Amazon Web Services account and Region.</p>
            </li>
         </ul> |
| `xks_proxy_connectivity` | String |  | <p>Indicates how KMS communicates with the external key store proxy. This parameter is
      required for custom key stores with a <code>CustomKeyStoreType</code> of
        <code>EXTERNAL_KEY_STORE</code>.</p>
         <p>If the external key store proxy uses a public endpoint, specify
        <code>PUBLIC_ENDPOINT</code>. If the external key store proxy uses a Amazon VPC
      endpoint service for communication with KMS, specify <code>VPC_ENDPOINT_SERVICE</code>. For
      help making this choice, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/choose-xks-connectivity.html">Choosing a connectivity option</a> in
      the <i>Key Management Service Developer Guide</i>.</p>
         <p>An Amazon VPC endpoint service keeps your communication with KMS in a private address space
      entirely within Amazon Web Services, but it requires more configuration, including establishing a Amazon VPC with multiple subnets, a VPC endpoint service, a network load balancer, and a
      verified private DNS name. A public endpoint is simpler to set up, but it might be slower and
      might not fulfill your security requirements. You might consider testing with a public
      endpoint, and then establishing a VPC endpoint service for production tasks. Note that this
      choice does not determine the location of the external key store proxy. Even if you choose a
      VPC endpoint service, the proxy can be hosted within the VPC or outside of Amazon Web Services such as in
      your corporate data center.</p> |
| `xks_proxy_authentication_credential` | String |  | <p>Specifies an authentication credential for the external key store proxy (XKS proxy). This
      parameter is required for all custom key stores with a <code>CustomKeyStoreType</code> of
        <code>EXTERNAL_KEY_STORE</code>.</p>
         <p>The <code>XksProxyAuthenticationCredential</code> has two required elements:
        <code>RawSecretAccessKey</code>, a secret key, and <code>AccessKeyId</code>, a unique
      identifier for the <code>RawSecretAccessKey</code>. For character requirements, see <a href="API_XksProxyAuthenticationCredentialType.html">XksProxyAuthenticationCredentialType</a>.</p>
         <p>KMS uses this authentication credential to sign requests to the external key store proxy
      on your behalf. This credential is unrelated to Identity and Access Management (IAM) and Amazon Web Services credentials.</p>
         <p>This parameter doesn't set or change the authentication credentials on the XKS proxy. It
      just tells KMS the credential that you established on your external key store proxy. If you
      rotate your proxy authentication credential, use the <a>UpdateCustomKeyStore</a>
      operation to provide the new credential to KMS.</p> |
| `xks_proxy_vpc_endpoint_service_name` | String |  | <p>Specifies the name of the Amazon VPC endpoint service for interface endpoints that is used to
      communicate with your external key store proxy (XKS proxy). This parameter is required when
      the value of <code>CustomKeyStoreType</code> is <code>EXTERNAL_KEY_STORE</code> and the value
      of <code>XksProxyConnectivity</code> is <code>VPC_ENDPOINT_SERVICE</code>.</p>
         <p>The Amazon VPC endpoint service must <a href="https://docs.aws.amazon.com/kms/latest/developerguide/create-xks-keystore.html#xks-requirements">fulfill all
        requirements</a> for use with an external key store. </p>
         <p>
            <b>Uniqueness requirements:</b>
         </p>
         <ul>
            <li>
               <p>External key stores with <code>VPC_ENDPOINT_SERVICE</code> connectivity can share an
          Amazon VPC, but each external key store must have its own VPC endpoint service and private DNS
          name.</p>
            </li>
         </ul> |
| `custom_key_store_name` | String | ✅ | <p>Specifies a friendly name for the custom key store. The name must be unique in your
      Amazon Web Services account and Region. This parameter is required for all custom key stores.</p>
         <important>
            <p>Do not include confidential or sensitive information in this field. This field may be displayed in plaintext in CloudTrail logs and other output.</p>
         </important> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create custom_key_store
custom_key_store = provider.kms.Custom_key_store {
    custom_key_store_name = "value"  # <p>Specifies a friendly name for the custom key store. The name must be unique in your
      Amazon Web Services account and Region. This parameter is required for all custom key stores.</p>
         <important>
            <p>Do not include confidential or sensitive information in this field. This field may be displayed in plaintext in CloudTrail logs and other output.</p>
         </important>
}

```

---


### Imported_key_material

ImportedKeyMaterial resource

**Operations**: ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|



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


### Grant

Grant resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `key_id` | String | ✅ | <p>Identifies the KMS key for the grant. The grant gives principals permission to use this
      KMS key.</p>
         <p>Specify the key ID or key ARN of the KMS key. To specify a KMS key in a
different Amazon Web Services account, you must use the key ARN.</p>
         <p>For example:</p>
         <ul>
            <li>
               <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code>
               </p>
            </li>
            <li>
               <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code>
               </p>
            </li>
         </ul>
         <p>To get the key ID and key ARN for a KMS key, use <a>ListKeys</a> or <a>DescribeKey</a>.</p> |
| `constraints` | String |  | <p>Specifies a grant constraint.</p>
         <important>
            <p>Do not include confidential or sensitive information in this field. This field may be displayed in plaintext in CloudTrail logs and other output.</p>
         </important>
         <p>KMS supports the <code>EncryptionContextEquals</code> and
        <code>EncryptionContextSubset</code> grant constraints, which allow the permissions in the
      grant only when the encryption context in the request matches
        (<code>EncryptionContextEquals</code>) or includes (<code>EncryptionContextSubset</code>)
      the encryption context specified in the constraint. </p>
         <p>The encryption context grant constraints are supported only on <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html#terms-grant-operations">grant operations</a> that include
      an <code>EncryptionContext</code> parameter, such as cryptographic operations on symmetric
      encryption KMS keys. Grants with grant constraints can include the <a>DescribeKey</a> and <a>RetireGrant</a> operations, but the constraint doesn't apply to these
      operations. If a grant with a grant constraint includes the <code>CreateGrant</code>
      operation, the constraint requires that any grants created with the <code>CreateGrant</code>
      permission have an equally strict or stricter encryption context constraint.</p>
         <p>You cannot use an encryption context grant constraint for cryptographic operations with
      asymmetric KMS keys or HMAC KMS keys. Operations with these keys don't support an encryption
      context.</p>
         <p>Each constraint value can include up to 8 encryption context pairs. The encryption context
      value in each constraint cannot exceed 384 characters. For information about grant
      constraints, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/create-grant-overview.html#grant-constraints">Using grant
        constraints</a> in the <i>Key Management Service Developer Guide</i>. For more information about encryption context,
      see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#encrypt_context">Encryption
        context</a> in the <i>
               <i>Key Management Service Developer Guide</i>
            </i>. </p> |
| `grant_tokens` | Vec<String> |  | <p>A list of grant tokens. </p>
         <p>Use a grant token when your permission to call this operation comes from a new grant that has not yet achieved <i>eventual consistency</i>. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html#grant_token">Grant token</a> and <a href="https://docs.aws.amazon.com/kms/latest/developerguide/using-grant-token.html">Using a grant token</a> in the
    <i>Key Management Service Developer Guide</i>.</p> |
| `grantee_principal` | String | ✅ | <p>The identity that gets the permissions specified in the grant.</p>
         <p>To specify the grantee principal, use the Amazon Resource Name (ARN) of an Amazon Web Services
      principal. Valid principals include Amazon Web Services accounts, IAM users, IAM roles,
      federated users, and assumed role users. For help with the ARN syntax for a principal, see
        <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_identifiers.html#identifiers-arns">IAM ARNs</a> in the <i>
               <i>Identity and Access Management User Guide</i>
            </i>.</p> |
| `retiring_principal` | String |  | <p>The principal that has permission to use the <a>RetireGrant</a> operation to
      retire the grant. </p>
         <p>To specify the principal, use the <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Name (ARN)</a> of an
      Amazon Web Services principal. Valid principals include Amazon Web Services accounts, IAM users, IAM roles,
      federated users, and assumed role users. For help with the ARN syntax for a principal, see
        <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_identifiers.html#identifiers-arns">IAM ARNs</a> in the <i>
               <i>Identity and Access Management User Guide</i>
            </i>.</p>
         <p>The grant determines the retiring principal. Other principals might have permission to
      retire the grant or revoke the grant. For details, see <a>RevokeGrant</a> and
        <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grant-delete.html">Retiring and revoking
        grants</a> in the <i>Key Management Service Developer Guide</i>. </p> |
| `dry_run` | bool |  | <p>Checks if your request will succeed. <code>DryRun</code> is an optional parameter. </p>
         <p>To learn more about how to use this parameter, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/testing-permissions.html">Testing your permissions</a> in the <i>Key Management Service Developer Guide</i>.</p> |
| `name` | String |  | <p>A friendly name for the grant. Use this value to prevent the unintended creation of
      duplicate grants when retrying this request.</p>
         <important>
            <p>Do not include confidential or sensitive information in this field. This field may be displayed in plaintext in CloudTrail logs and other output.</p>
         </important>
         <p>When this value is absent, all <code>CreateGrant</code> requests result in a new grant
      with a unique <code>GrantId</code> even if all the supplied parameters are identical. This can
      result in unintended duplicates when you retry the <code>CreateGrant</code> request.</p>
         <p>When this value is present, you can retry a <code>CreateGrant</code> request with
      identical parameters; if the grant already exists, the original <code>GrantId</code> is
      returned without creating a new grant. Note that the returned grant token is unique with every
        <code>CreateGrant</code> request, even when a duplicate <code>GrantId</code> is returned.
      All grant tokens for the same grant ID can be used interchangeably.</p> |
| `operations` | Vec<String> | ✅ | <p>A list of operations that the grant permits. </p>
         <p>This list must include only operations that are permitted in a grant. Also, the operation
      must be supported on the KMS key. For example, you cannot create a grant for a symmetric
      encryption KMS key that allows the <a>Sign</a> operation, or a grant for an
      asymmetric KMS key that allows the <a>GenerateDataKey</a> operation. If you try,
      KMS returns a <code>ValidationError</code> exception. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html#terms-grant-operations">Grant
        operations</a> in the <i>Key Management Service Developer Guide</i>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create grant
grant = provider.kms.Grant {
    key_id = "value"  # <p>Identifies the KMS key for the grant. The grant gives principals permission to use this
      KMS key.</p>
         <p>Specify the key ID or key ARN of the KMS key. To specify a KMS key in a
different Amazon Web Services account, you must use the key ARN.</p>
         <p>For example:</p>
         <ul>
            <li>
               <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code>
               </p>
            </li>
            <li>
               <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code>
               </p>
            </li>
         </ul>
         <p>To get the key ID and key ARN for a KMS key, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    grantee_principal = "value"  # <p>The identity that gets the permissions specified in the grant.</p>
         <p>To specify the grantee principal, use the Amazon Resource Name (ARN) of an Amazon Web Services
      principal. Valid principals include Amazon Web Services accounts, IAM users, IAM roles,
      federated users, and assumed role users. For help with the ARN syntax for a principal, see
        <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_identifiers.html#identifiers-arns">IAM ARNs</a> in the <i>
               <i>Identity and Access Management User Guide</i>
            </i>.</p>
    operations = "value"  # <p>A list of operations that the grant permits. </p>
         <p>This list must include only operations that are permitted in a grant. Also, the operation
      must be supported on the KMS key. For example, you cannot create a grant for a symmetric
      encryption KMS key that allows the <a>Sign</a> operation, or a grant for an
      asymmetric KMS key that allows the <a>GenerateDataKey</a> operation. If you try,
      KMS returns a <code>ValidationError</code> exception. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html#terms-grant-operations">Grant
        operations</a> in the <i>Key Management Service Developer Guide</i>.</p>
}

```

---


### Alias

Alias resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `alias_name` | String | ✅ | <p>Specifies the alias name. This value must begin with <code>alias/</code> followed by a
      name, such as <code>alias/ExampleAlias</code>. </p>
         <important>
            <p>Do not include confidential or sensitive information in this field. This field may be displayed in plaintext in CloudTrail logs and other output.</p>
         </important>
         <p>The <code>AliasName</code> value must be string of 1-256 characters. It can contain only
      alphanumeric characters, forward slashes (/), underscores (_), and dashes (-). The alias name
      cannot begin with <code>alias/aws/</code>. The <code>alias/aws/</code> prefix is reserved for
        <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#aws-managed-key">Amazon Web Services managed
        keys</a>.</p> |
| `target_key_id` | String | ✅ | <p>Associates the alias with the specified <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#customer-mgn-key">customer managed key</a>. The KMS key
      must be in the same Amazon Web Services Region. </p>
         <p>A valid key ID is required. If you supply a null or empty string value, this operation
      returns an error.</p>
         <p>For help finding the key ID and ARN, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/find-cmk-id-arn.html">Find the key ID and key ARN</a> in
      the <i>
               <i>Key Management Service Developer Guide</i>
            </i>.</p>
         <p>Specify the key ID or key ARN of the KMS key.</p>
         <p>For example:</p>
         <ul>
            <li>
               <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code>
               </p>
            </li>
            <li>
               <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code>
               </p>
            </li>
         </ul>
         <p>To get the key ID and key ARN for a KMS key, use <a>ListKeys</a> or <a>DescribeKey</a>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create alias
alias = provider.kms.Alias {
    alias_name = "value"  # <p>Specifies the alias name. This value must begin with <code>alias/</code> followed by a
      name, such as <code>alias/ExampleAlias</code>. </p>
         <important>
            <p>Do not include confidential or sensitive information in this field. This field may be displayed in plaintext in CloudTrail logs and other output.</p>
         </important>
         <p>The <code>AliasName</code> value must be string of 1-256 characters. It can contain only
      alphanumeric characters, forward slashes (/), underscores (_), and dashes (-). The alias name
      cannot begin with <code>alias/aws/</code>. The <code>alias/aws/</code> prefix is reserved for
        <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#aws-managed-key">Amazon Web Services managed
        keys</a>.</p>
    target_key_id = "value"  # <p>Associates the alias with the specified <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#customer-mgn-key">customer managed key</a>. The KMS key
      must be in the same Amazon Web Services Region. </p>
         <p>A valid key ID is required. If you supply a null or empty string value, this operation
      returns an error.</p>
         <p>For help finding the key ID and ARN, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/find-cmk-id-arn.html">Find the key ID and key ARN</a> in
      the <i>
               <i>Key Management Service Developer Guide</i>
            </i>.</p>
         <p>Specify the key ID or key ARN of the KMS key.</p>
         <p>For example:</p>
         <ul>
            <li>
               <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code>
               </p>
            </li>
            <li>
               <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code>
               </p>
            </li>
         </ul>
         <p>To get the key ID and key ARN for a KMS key, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
}

```

---


### Key_rotation_status

KeyRotationStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `key_id` | String | <p>Identifies the specified symmetric encryption KMS key.</p> |
| `on_demand_rotation_start_date` | String | <p>Identifies the date and time that an in progress on-demand rotation was initiated.</p>
         <p>KMS uses a background process to perform rotations. As a result, there might be a slight
      delay between initiating on-demand key rotation and the rotation's completion. Once the
      on-demand rotation is complete, KMS removes this field from the response. You can use <a>ListKeyRotations</a> to view the details of the completed on-demand rotation.</p> |
| `rotation_period_in_days` | i64 | <p>The number of days between each automatic rotation. The default value is 365 days.</p> |
| `next_rotation_date` | String | <p>The next date that KMS will automatically rotate the key material.</p> |
| `key_rotation_enabled` | bool | <p>A Boolean value that specifies whether key rotation is enabled.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access key_rotation_status outputs
key_rotation_status_id = key_rotation_status.id
key_rotation_status_key_id = key_rotation_status.key_id
key_rotation_status_on_demand_rotation_start_date = key_rotation_status.on_demand_rotation_start_date
key_rotation_status_rotation_period_in_days = key_rotation_status.rotation_period_in_days
key_rotation_status_next_rotation_date = key_rotation_status.next_rotation_date
key_rotation_status_key_rotation_enabled = key_rotation_status.key_rotation_enabled
```

---


### Key_policy

KeyPolicy resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `policy_name` | String |  | <p>The name of the key policy. If no policy name is specified, the default value is
        <code>default</code>. The only valid value is <code>default</code>.</p> |
| `bypass_policy_lockout_safety_check` | bool |  | <p>Skips ("bypasses") the key policy lockout safety check. The default value is false.</p>
         <important>
            <p>Setting this value to true increases the risk that the KMS key becomes unmanageable. Do
        not set this value to true indiscriminately.</p>
            <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policy-default.html#prevent-unmanageable-key">Default key policy</a> in the <i>Key Management Service Developer Guide</i>.</p>
         </important>
         <p>Use this parameter only when you intend to prevent the principal that is making the
      request from making a subsequent <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_PutKeyPolicy.html">PutKeyPolicy</a>
      request on the KMS key.</p> |
| `key_id` | String | ✅ | <p>Sets the key policy on the specified KMS key.</p>
         <p>Specify the key ID or key ARN of the KMS key.</p>
         <p>For example:</p>
         <ul>
            <li>
               <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code>
               </p>
            </li>
            <li>
               <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code>
               </p>
            </li>
         </ul>
         <p>To get the key ID and key ARN for a KMS key, use <a>ListKeys</a> or <a>DescribeKey</a>.</p> |
| `policy` | String | ✅ | <p>The key policy to attach to the KMS key.</p>
         <p>The key policy must meet the following criteria:</p>
         <ul>
            <li>
               <p>The key policy must allow the calling principal to make a
          subsequent <code>PutKeyPolicy</code> request on the KMS key.  This reduces the risk that
          the KMS key becomes unmanageable. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policy-default.html#prevent-unmanageable-key">Default key policy</a> in the <i>Key Management Service Developer Guide</i>. (To omit
          this condition, set <code>BypassPolicyLockoutSafetyCheck</code> to true.)</p>
            </li>
            <li>
               <p>Each statement in the key policy must contain one or more principals. The principals
          in the key policy must exist and be visible to KMS. When you create a new Amazon Web Services
          principal, you might need to enforce a delay before including the new principal in a key
          policy because the new principal might not be immediately visible to KMS. For more
          information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/troubleshoot_general.html#troubleshoot_general_eventual-consistency">Changes that I make are not always immediately visible</a> in the <i>Amazon Web Services
            Identity and Access Management User Guide</i>.</p>
            </li>
         </ul>
         <note>
            <p>If either of the required <code>Resource</code> or <code>Action</code> elements are
        missing from a key policy statement, the policy statement has no effect. When a key policy
        statement is missing one of these elements, the KMS console correctly reports an error,
        but the <code>PutKeyPolicy</code> API request succeeds, even though the policy statement is
        ineffective.</p>
            <p>For more information on required key policy elements, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policy-overview.html#key-policy-elements">Elements in a key
          policy</a> in the <i>Key Management Service Developer Guide</i>.</p>
         </note>
         <p>A key policy document can include only the following characters:</p>
         <ul>
            <li>
               <p>Printable ASCII characters from the space character (<code>\u0020</code>) through the end of the ASCII character range.</p>
            </li>
            <li>
               <p>Printable characters in the Basic Latin and Latin-1 Supplement character set (through <code>\u00FF</code>).</p>
            </li>
            <li>
               <p>The tab (<code>\u0009</code>), line feed (<code>\u000A</code>), and carriage return (<code>\u000D</code>) special characters</p>
            </li>
         </ul>
         <note>
            <p>If the key policy exceeds the length constraint, KMS returns a
          <code>LimitExceededException</code>.</p>
         </note>
         <p>For information about key policies, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html">Key policies in KMS</a> in the
      <i>Key Management Service Developer Guide</i>.For help writing and formatting a JSON policy document, see the <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies.html">IAM JSON Policy Reference</a> in the <i>
               <i>Identity and Access Management User Guide</i>
            </i>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `policy_name` | String | <p>The name of the key policy. The only valid value is <code>default</code>.</p> |
| `policy` | String | <p>A key policy document in JSON format.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create key_policy
key_policy = provider.kms.Key_policy {
    key_id = "value"  # <p>Sets the key policy on the specified KMS key.</p>
         <p>Specify the key ID or key ARN of the KMS key.</p>
         <p>For example:</p>
         <ul>
            <li>
               <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code>
               </p>
            </li>
            <li>
               <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code>
               </p>
            </li>
         </ul>
         <p>To get the key ID and key ARN for a KMS key, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    policy = "value"  # <p>The key policy to attach to the KMS key.</p>
         <p>The key policy must meet the following criteria:</p>
         <ul>
            <li>
               <p>The key policy must allow the calling principal to make a
          subsequent <code>PutKeyPolicy</code> request on the KMS key.  This reduces the risk that
          the KMS key becomes unmanageable. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policy-default.html#prevent-unmanageable-key">Default key policy</a> in the <i>Key Management Service Developer Guide</i>. (To omit
          this condition, set <code>BypassPolicyLockoutSafetyCheck</code> to true.)</p>
            </li>
            <li>
               <p>Each statement in the key policy must contain one or more principals. The principals
          in the key policy must exist and be visible to KMS. When you create a new Amazon Web Services
          principal, you might need to enforce a delay before including the new principal in a key
          policy because the new principal might not be immediately visible to KMS. For more
          information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/troubleshoot_general.html#troubleshoot_general_eventual-consistency">Changes that I make are not always immediately visible</a> in the <i>Amazon Web Services
            Identity and Access Management User Guide</i>.</p>
            </li>
         </ul>
         <note>
            <p>If either of the required <code>Resource</code> or <code>Action</code> elements are
        missing from a key policy statement, the policy statement has no effect. When a key policy
        statement is missing one of these elements, the KMS console correctly reports an error,
        but the <code>PutKeyPolicy</code> API request succeeds, even though the policy statement is
        ineffective.</p>
            <p>For more information on required key policy elements, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policy-overview.html#key-policy-elements">Elements in a key
          policy</a> in the <i>Key Management Service Developer Guide</i>.</p>
         </note>
         <p>A key policy document can include only the following characters:</p>
         <ul>
            <li>
               <p>Printable ASCII characters from the space character (<code>\u0020</code>) through the end of the ASCII character range.</p>
            </li>
            <li>
               <p>Printable characters in the Basic Latin and Latin-1 Supplement character set (through <code>\u00FF</code>).</p>
            </li>
            <li>
               <p>The tab (<code>\u0009</code>), line feed (<code>\u000A</code>), and carriage return (<code>\u000D</code>) special characters</p>
            </li>
         </ul>
         <note>
            <p>If the key policy exceeds the length constraint, KMS returns a
          <code>LimitExceededException</code>.</p>
         </note>
         <p>For information about key policies, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html">Key policies in KMS</a> in the
      <i>Key Management Service Developer Guide</i>.For help writing and formatting a JSON policy document, see the <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies.html">IAM JSON Policy Reference</a> in the <i>
               <i>Identity and Access Management User Guide</i>
            </i>.</p>
}

# Access key_policy outputs
key_policy_id = key_policy.id
key_policy_policy_name = key_policy.policy_name
key_policy_policy = key_policy.policy
```

---


### Public_key

PublicKey resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `key_spec` | String | <p>The type of the of the public key that was downloaded.</p> |
| `public_key` | String | <p>The exported public key. </p>
         <p>The value is a DER-encoded X.509 public key, also known as
        <code>SubjectPublicKeyInfo</code> (SPKI), as defined in <a href="https://tools.ietf.org/html/rfc5280">RFC 5280</a>. When you use the HTTP API or the Amazon Web Services CLI, the value is Base64-encoded. Otherwise, it is not Base64-encoded.</p>
         <p></p> |
| `key_agreement_algorithms` | Vec<String> | <p>The key agreement algorithm used to derive a shared secret. This field is present only
      when the KMS key has a <code>KeyUsage</code> value of <code>KEY_AGREEMENT</code>.</p> |
| `customer_master_key_spec` | String | <p>Instead, use the <code>KeySpec</code> field in the <code>GetPublicKey</code>
      response.</p>
         <p>The <code>KeySpec</code> and <code>CustomerMasterKeySpec</code> fields have the same
      value. We recommend that you use the <code>KeySpec</code> field in your code. However, to
      avoid breaking changes, KMS supports both fields.</p> |
| `key_usage` | String | <p>The permitted use of the public key. Valid values for asymmetric key pairs are
        <code>ENCRYPT_DECRYPT</code>, <code>SIGN_VERIFY</code>, and <code>KEY_AGREEMENT</code>. </p>
         <p>This information is critical. For example, if a public key with <code>SIGN_VERIFY</code>
      key usage encrypts data outside of KMS, the ciphertext cannot be decrypted. </p> |
| `signing_algorithms` | Vec<String> | <p>The signing algorithms that KMS supports for this key.</p>
         <p>This field appears in the response only when the <code>KeyUsage</code> of the public key
      is <code>SIGN_VERIFY</code>.</p> |
| `key_id` | String | <p>The Amazon Resource Name (<a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#key-id-key-ARN">key ARN</a>) of the asymmetric KMS key from which the public key was
      downloaded.</p> |
| `encryption_algorithms` | Vec<String> | <p>The encryption algorithms that KMS supports for this key. </p>
         <p>This information is critical. If a public key encrypts data outside of KMS by using an
      unsupported encryption algorithm, the ciphertext cannot be decrypted. </p>
         <p>This field appears in the response only when the <code>KeyUsage</code> of the public key
      is <code>ENCRYPT_DECRYPT</code>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access public_key outputs
public_key_id = public_key.id
public_key_key_spec = public_key.key_spec
public_key_public_key = public_key.public_key
public_key_key_agreement_algorithms = public_key.key_agreement_algorithms
public_key_customer_master_key_spec = public_key.customer_master_key_spec
public_key_key_usage = public_key.key_usage
public_key_signing_algorithms = public_key.signing_algorithms
public_key_key_id = public_key.key_id
public_key_encryption_algorithms = public_key.encryption_algorithms
```

---


### Key_description

KeyDescription resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `key_id` | String | ✅ | <p>Updates the description of the specified KMS key.</p>
         <p>Specify the key ID or key ARN of the KMS key.</p>
         <p>For example:</p>
         <ul>
            <li>
               <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code>
               </p>
            </li>
            <li>
               <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code>
               </p>
            </li>
         </ul>
         <p>To get the key ID and key ARN for a KMS key, use <a>ListKeys</a> or <a>DescribeKey</a>.</p> |
| `description` | String | ✅ | <p>New description for the KMS key.</p>
         <important>
            <p>Do not include confidential or sensitive information in this field. This field may be displayed in plaintext in CloudTrail logs and other output.</p>
         </important> |



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

# Create multiple custom_key_stores resources
custom_key_stores_0 = provider.kms.Custom_key_stores {
}
custom_key_stores_1 = provider.kms.Custom_key_stores {
}
custom_key_stores_2 = provider.kms.Custom_key_stores {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    custom_key_stores = provider.kms.Custom_key_stores {
    }
```

---

## Related Documentation

- [AWS Kms Documentation](https://docs.aws.amazon.com/kms/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
