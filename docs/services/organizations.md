# Organizations Service



**Resources**: 9

---

## Overview

The organizations service provides access to 9 resource types:

- [Create_account_status](#create_account_status) [R]
- [Handshake](#handshake) [R]
- [Gov_cloud_account](#gov_cloud_account) [C]
- [Organizational_unit](#organizational_unit) [CRUD]
- [Effective_policy](#effective_policy) [R]
- [Account](#account) [CR]
- [Organization](#organization) [CRD]
- [Resource_policy](#resource_policy) [CRD]
- [Policy](#policy) [CRUD]

---

## Resources


### Create_account_status

CreateAccountStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_account_status` | String | <p>A structure that contains the current status of an account creation request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access create_account_status outputs
create_account_status_id = create_account_status.id
create_account_status_create_account_status = create_account_status.create_account_status
```

---


### Handshake

Handshake resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `handshake` | String | <p>A structure that contains information about the specified handshake.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access handshake outputs
handshake_id = handshake.id
handshake_handshake = handshake.handshake
```

---


### Gov_cloud_account

GovCloudAccount resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `email` | String | ✅ | <p>Specifies the email address of the owner to assign to the new member account in the
            commercial Region. This email address must not already be associated with another
            Amazon Web Services account. You must use a valid email address to complete account creation.</p>
         <p>The rules for a valid email address:</p>
         <ul>
            <li>
               <p>The address must be a minimum of 6 and a maximum of 64 characters long.</p>
            </li>
            <li>
               <p>All characters must be 7-bit ASCII characters.</p>
            </li>
            <li>
               <p>There must be one and only one @ symbol, which separates the local name from
                    the domain name.</p>
            </li>
            <li>
               <p>The local name can't contain any of the following characters:</p>
               <p>whitespace, " ' ( ) < > [ ] : ; , \ | % &</p>
            </li>
            <li>
               <p>The local name can't begin with a dot (.)</p>
            </li>
            <li>
               <p>The domain name can consist of only the characters [a-z],[A-Z],[0-9], hyphen
                    (-), or dot (.)</p>
            </li>
            <li>
               <p>The domain name can't begin or end with a hyphen (-) or dot (.)</p>
            </li>
            <li>
               <p>The domain name must contain at least one dot</p>
            </li>
         </ul>
         <p>You can't access the root user of the account or remove an account that was created
            with an invalid email address. Like all request parameters for
                <code>CreateGovCloudAccount</code>, the request for the email address for the Amazon Web Services
            GovCloud (US) account originates from the commercial Region, not from the Amazon Web Services GovCloud
            (US) Region.</p> |
| `role_name` | String |  | <p>(Optional)</p>
         <p>The name of an IAM role that Organizations automatically preconfigures in the new member
            accounts in both the Amazon Web Services GovCloud (US) Region and in the commercial Region. This role
            trusts the management account, allowing users in the management account to assume the
            role, as permitted by the management account administrator. The role has administrator
            permissions in the new member account.</p>
         <p>If you don't specify this parameter, the role name defaults to
                <code>OrganizationAccountAccessRole</code>.</p>
         <p>For more information about how to use this role to access the member account, see the
            following links:</p>
         <ul>
            <li>
               <p>
                  <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_access.html#orgs_manage_accounts_create-cross-account-role">Creating the OrganizationAccountAccessRole in an invited member
                        account</a> in the <i>Organizations User Guide</i>
               </p>
            </li>
            <li>
               <p>Steps 2 and 3 in <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/tutorial_cross-account-with-roles.html">IAM Tutorial:
                        Delegate access across Amazon Web Services accounts using IAM roles</a> in the
                        <i>IAM User Guide</i>
               </p>
            </li>
         </ul>
         <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> that 
    is used to validate this parameter. The pattern can include uppercase 
    letters, lowercase letters, digits with no spaces, and any of the following characters: =,.@-</p> |
| `account_name` | String | ✅ | <p>The friendly name of the member account. </p>
         <p>The account name can consist of only the characters [a-z],[A-Z],[0-9], hyphen (-), or
            dot (.) You can't separate characters with a dash (–).</p> |
| `iam_user_access_to_billing` | String |  | <p>If set to <code>ALLOW</code>, the new linked account in the commercial Region enables
            IAM users to access account billing information <i>if</i> they have the
            required permissions. If set to <code>DENY</code>, only the root user of the new account
            can access account billing information. For more information, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/grantaccess.html#ControllingAccessWebsite-Activate">About IAM
                access to the Billing and Cost Management console</a> in the
            <i>Amazon Web Services Billing and Cost Management User Guide</i>.</p>
         <p>If you don't specify this parameter, the value defaults to <code>ALLOW</code>, and
            IAM users and roles with the required permissions can access billing information for
            the new account.</p> |
| `tags` | Vec<String> |  | <p>A list of tags that you want to attach to the newly created account. These tags are
            attached to the commercial account associated with the GovCloud account, and not to the
            GovCloud account itself. To add tags to the actual GovCloud account, call the <a>TagResource</a> operation in the GovCloud region after the new GovCloud
            account exists.</p>
         <p>For each tag in the list, you must specify both a tag key and a value. You can set the
            value to an empty string, but you can't set it to <code>null</code>. For more
            information about tagging, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_tagging.html">Tagging Organizations resources</a> in the
            Organizations User Guide.</p>
         <note>
            <p>If any one of the tags is not valid or if you exceed the maximum allowed number of
                tags for an account, then the entire request fails and the account is not
                created.</p>
         </note> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create gov_cloud_account
gov_cloud_account = provider.organizations.Gov_cloud_account {
    email = "value"  # <p>Specifies the email address of the owner to assign to the new member account in the
            commercial Region. This email address must not already be associated with another
            Amazon Web Services account. You must use a valid email address to complete account creation.</p>
         <p>The rules for a valid email address:</p>
         <ul>
            <li>
               <p>The address must be a minimum of 6 and a maximum of 64 characters long.</p>
            </li>
            <li>
               <p>All characters must be 7-bit ASCII characters.</p>
            </li>
            <li>
               <p>There must be one and only one @ symbol, which separates the local name from
                    the domain name.</p>
            </li>
            <li>
               <p>The local name can't contain any of the following characters:</p>
               <p>whitespace, " ' ( ) < > [ ] : ; , \ | % &</p>
            </li>
            <li>
               <p>The local name can't begin with a dot (.)</p>
            </li>
            <li>
               <p>The domain name can consist of only the characters [a-z],[A-Z],[0-9], hyphen
                    (-), or dot (.)</p>
            </li>
            <li>
               <p>The domain name can't begin or end with a hyphen (-) or dot (.)</p>
            </li>
            <li>
               <p>The domain name must contain at least one dot</p>
            </li>
         </ul>
         <p>You can't access the root user of the account or remove an account that was created
            with an invalid email address. Like all request parameters for
                <code>CreateGovCloudAccount</code>, the request for the email address for the Amazon Web Services
            GovCloud (US) account originates from the commercial Region, not from the Amazon Web Services GovCloud
            (US) Region.</p>
    account_name = "value"  # <p>The friendly name of the member account. </p>
         <p>The account name can consist of only the characters [a-z],[A-Z],[0-9], hyphen (-), or
            dot (.) You can't separate characters with a dash (–).</p>
}

```

---


### Organizational_unit

OrganizationalUnit resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>The friendly name to assign to the new OU.</p> |
| `tags` | Vec<String> |  | <p>A list of tags that you want to attach to the newly created OU. For each tag in the
            list, you must specify both a tag key and a value. You can set the value to an empty
            string, but you can't set it to <code>null</code>. For more information about tagging,
            see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_tagging.html">Tagging Organizations
                resources</a> in the Organizations User Guide.</p>
         <note>
            <p>If any one of the tags is not valid or if you exceed the allowed number of tags
                for an OU, then the entire request fails and the OU is not created.</p>
         </note> |
| `parent_id` | String | ✅ | <p>The unique identifier (ID) of the parent root or OU that you want to create the new OU
            in.</p>
         <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a parent ID string requires one of the 
    following:</p>
         <ul>
            <li>
               <p>
                  <b>Root</b> - A string that begins with "r-" followed by from 4 to 32 lowercase letters or 
          digits.</p>
            </li>
            <li>
               <p>
                  <b>Organizational unit (OU)</b> - A string that begins with "ou-" followed by from 4 to 32
          lowercase letters or digits (the ID of the root that the OU is in). This string is followed by a second 
          "-" dash and from 8 to 32 additional lowercase letters or digits.</p>
            </li>
         </ul> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `organizational_unit` | String | <p>A structure that contains details about the specified OU.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create organizational_unit
organizational_unit = provider.organizations.Organizational_unit {
    name = "value"  # <p>The friendly name to assign to the new OU.</p>
    parent_id = "value"  # <p>The unique identifier (ID) of the parent root or OU that you want to create the new OU
            in.</p>
         <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a parent ID string requires one of the 
    following:</p>
         <ul>
            <li>
               <p>
                  <b>Root</b> - A string that begins with "r-" followed by from 4 to 32 lowercase letters or 
          digits.</p>
            </li>
            <li>
               <p>
                  <b>Organizational unit (OU)</b> - A string that begins with "ou-" followed by from 4 to 32
          lowercase letters or digits (the ID of the root that the OU is in). This string is followed by a second 
          "-" dash and from 8 to 32 additional lowercase letters or digits.</p>
            </li>
         </ul>
}

# Access organizational_unit outputs
organizational_unit_id = organizational_unit.id
organizational_unit_organizational_unit = organizational_unit.organizational_unit
```

---


### Effective_policy

EffectivePolicy resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `effective_policy` | String | <p>The contents of the effective policy.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access effective_policy outputs
effective_policy_id = effective_policy.id
effective_policy_effective_policy = effective_policy.effective_policy
```

---


### Account

Account resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `role_name` | String |  | <p>The name of an IAM role that Organizations automatically preconfigures in the new member
            account. This role trusts the management account, allowing users in the management
            account to assume the role, as permitted by the management account administrator. The
            role has administrator permissions in the new member account.</p>
         <p>If you don't specify this parameter, the role name defaults to
                <code>OrganizationAccountAccessRole</code>.</p>
         <p>For more information about how to use this role to access the member account, see the
            following links:</p>
         <ul>
            <li>
               <p>
                  <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_access.html#orgs_manage_accounts_create-cross-account-role">Creating the OrganizationAccountAccessRole in an invited member
                        account</a> in the <i>Organizations User Guide</i>
               </p>
            </li>
            <li>
               <p>Steps 2 and 3 in <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/tutorial_cross-account-with-roles.html">IAM Tutorial:
                        Delegate access across Amazon Web Services accounts using IAM roles</a> in the
                        <i>IAM User Guide</i>
               </p>
            </li>
         </ul>
         <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> that 
    is used to validate this parameter. The pattern can include uppercase 
    letters, lowercase letters, digits with no spaces, and any of the following characters: =,.@-</p> |
| `tags` | Vec<String> |  | <p>A list of tags that you want to attach to the newly created account. For each tag in
            the list, you must specify both a tag key and a value. You can set the value to an empty
            string, but you can't set it to <code>null</code>. For more information about tagging,
            see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_tagging.html">Tagging Organizations
                resources</a> in the Organizations User Guide.</p>
         <note>
            <p>If any one of the tags is not valid or if you exceed the maximum allowed number of
                tags for an account, then the entire request fails and the account is not
                created.</p>
         </note> |
| `email` | String | ✅ | <p>The email address of the owner to assign to the new member account. This email address
            must not already be associated with another Amazon Web Services account. You must use a valid email
            address to complete account creation.</p>
         <p>The rules for a valid email address:</p>
         <ul>
            <li>
               <p>The address must be a minimum of 6 and a maximum of 64 characters long.</p>
            </li>
            <li>
               <p>All characters must be 7-bit ASCII characters.</p>
            </li>
            <li>
               <p>There must be one and only one @ symbol, which separates the local name from
                    the domain name.</p>
            </li>
            <li>
               <p>The local name can't contain any of the following characters:</p>
               <p>whitespace, " ' ( ) < > [ ] : ; , \ | % &</p>
            </li>
            <li>
               <p>The local name can't begin with a dot (.)</p>
            </li>
            <li>
               <p>The domain name can consist of only the characters [a-z],[A-Z],[0-9], hyphen
                    (-), or dot (.)</p>
            </li>
            <li>
               <p>The domain name can't begin or end with a hyphen (-) or dot (.)</p>
            </li>
            <li>
               <p>The domain name must contain at least one dot</p>
            </li>
         </ul>
         <p>You can't access the root user of the account or remove an account that was created
            with an invalid email address.</p> |
| `iam_user_access_to_billing` | String |  | <p>If set to <code>ALLOW</code>, the new account enables IAM users to access account
            billing information <i>if</i> they have the required permissions. If set
            to <code>DENY</code>, only the root user of the new account can access account billing
            information. For more information, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/grantaccess.html#ControllingAccessWebsite-Activate">About IAM
                access to the Billing and Cost Management console</a> in the
            <i>Amazon Web Services Billing and Cost Management User Guide</i>.</p>
         <p>If you don't specify this parameter, the value defaults to <code>ALLOW</code>, and
            IAM users and roles with the required permissions can access billing information for
            the new account.</p> |
| `account_name` | String | ✅ | <p>The friendly name of the member account.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `account` | String | <p>A structure that contains information about the requested account.</p>
         <important>
            <p>The <code>Status</code> parameter in the API response will be retired on September 9, 2026.
                Although both the account <code>State</code> and account <code>Status</code> parameters are currently
                available in the Organizations APIs (<code>DescribeAccount</code>, <code>ListAccounts</code>,
                <code>ListAccountsForParent</code>), we recommend that you update your scripts or other code to
                use the <code>State</code> parameter instead of <code>Status</code> before September 9, 2026.</p>
         </important> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create account
account = provider.organizations.Account {
    email = "value"  # <p>The email address of the owner to assign to the new member account. This email address
            must not already be associated with another Amazon Web Services account. You must use a valid email
            address to complete account creation.</p>
         <p>The rules for a valid email address:</p>
         <ul>
            <li>
               <p>The address must be a minimum of 6 and a maximum of 64 characters long.</p>
            </li>
            <li>
               <p>All characters must be 7-bit ASCII characters.</p>
            </li>
            <li>
               <p>There must be one and only one @ symbol, which separates the local name from
                    the domain name.</p>
            </li>
            <li>
               <p>The local name can't contain any of the following characters:</p>
               <p>whitespace, " ' ( ) < > [ ] : ; , \ | % &</p>
            </li>
            <li>
               <p>The local name can't begin with a dot (.)</p>
            </li>
            <li>
               <p>The domain name can consist of only the characters [a-z],[A-Z],[0-9], hyphen
                    (-), or dot (.)</p>
            </li>
            <li>
               <p>The domain name can't begin or end with a hyphen (-) or dot (.)</p>
            </li>
            <li>
               <p>The domain name must contain at least one dot</p>
            </li>
         </ul>
         <p>You can't access the root user of the account or remove an account that was created
            with an invalid email address.</p>
    account_name = "value"  # <p>The friendly name of the member account.</p>
}

# Access account outputs
account_id = account.id
account_account = account.account
```

---


### Organization

Organization resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `feature_set` | String |  | <p>Specifies the feature set supported by the new organization. Each feature set supports
            different levels of functionality.</p>
         <ul>
            <li>
               <p>
                  <code>CONSOLIDATED_BILLING</code>: All member accounts have their bills
                    consolidated to and paid by the management account. For more information, see
                        <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_getting-started_concepts.html#feature-set-cb-only">Consolidated billing</a> in the
                    <i>Organizations User Guide</i>.</p>
               <p> The consolidated billing feature subset isn't available for organizations in
                    the Amazon Web Services GovCloud (US) Region.</p>
            </li>
            <li>
               <p>
                  <code>ALL</code>: In addition to all the features supported by the
                    consolidated billing feature set, the management account can also apply any
                    policy type to any member account in the organization. For more information, see
                        <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_getting-started_concepts.html#feature-set-all">All
                        features</a> in the <i>Organizations User Guide</i>.</p>
            </li>
         </ul> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `organization` | String | <p>A structure that contains information about the organization.</p>
         <important>
            <p>The <code>AvailablePolicyTypes</code> part of the response is deprecated, and you
                shouldn't use it in your apps. It doesn't include any policy type supported by Organizations
                other than SCPs. In the China (Ningxia) Region, no policy type is included. To
                determine which policy types are enabled in your organization, use the <code>
                  <a>ListRoots</a>
               </code> operation.</p>
         </important> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create organization
organization = provider.organizations.Organization {
}

# Access organization outputs
organization_id = organization.id
organization_organization = organization.organization
```

---


### Resource_policy

ResourcePolicy resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `content` | String | ✅ | <p>If provided, the new content for the resource policy. The text must be correctly
            formatted JSON that complies with the syntax for the resource policy's type. For more
            information, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_scps_syntax.html">SCP syntax</a> in the
            <i>Organizations User Guide</i>.</p> |
| `tags` | Vec<String> |  | <p>A list of tags that you want to attach to the newly created resource policy. For each
            tag in the list, you must specify both a tag key and a value. You can set the value to
            an empty string, but you can't set it to <code>null</code>. For more information about
            tagging, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_tagging.html">Tagging
                Organizations resources</a> in the Organizations User Guide.</p>
         <note>
            <p>Calls with tags apply to the initial creation of the resource policy, otherwise an
                exception is thrown. If any one of the tags is not valid or if you exceed the
                allowed number of tags for the resource policy, then the entire request fails and
                the resource policy is not created. </p>
         </note> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `resource_policy` | String | <p>A structure that contains details about the resource policy.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create resource_policy
resource_policy = provider.organizations.Resource_policy {
    content = "value"  # <p>If provided, the new content for the resource policy. The text must be correctly
            formatted JSON that complies with the syntax for the resource policy's type. For more
            information, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_scps_syntax.html">SCP syntax</a> in the
            <i>Organizations User Guide</i>.</p>
}

# Access resource_policy outputs
resource_policy_id = resource_policy.id
resource_policy_resource_policy = resource_policy.resource_policy
```

---


### Policy

Policy resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>A list of tags that you want to attach to the newly created policy. For each tag in
            the list, you must specify both a tag key and a value. You can set the value to an empty
            string, but you can't set it to <code>null</code>. For more information about tagging,
            see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_tagging.html">Tagging Organizations
                resources</a> in the Organizations User Guide.</p>
         <note>
            <p>If any one of the tags is not valid or if you exceed the allowed number of tags
                for a policy, then the entire request fails and the policy is not created.</p>
         </note> |
| `content` | String | ✅ | <p>The policy text content to add to the new policy. The text that you supply must adhere
            to the rules of the policy type you specify in the <code>Type</code> parameter. </p>
         <p>The maximum size of a policy document depends on the policy's type. For more
            information, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html#min-max-values">Maximum and minimum values</a> in the
            <i>Organizations User Guide</i>.</p> |
| `name` | String | ✅ | <p>The friendly name to assign to the policy.</p>
         <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> 
    that is used to validate this parameter is a string of any of the characters in the ASCII 
    character range.</p> |
| `type` | String | ✅ | <p>The type of policy to create. You can specify one of the following values:</p>
         <ul>
            <li>
               <p>
                  <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_scp.html">SERVICE_CONTROL_POLICY</a>
               </p>
            </li>
            <li>
               <p>
                  <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_rcps.html">RESOURCE_CONTROL_POLICY</a>
               </p>
            </li>
            <li>
               <p>
                  <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_declarative.html">DECLARATIVE_POLICY_EC2</a>
               </p>
            </li>
            <li>
               <p>
                  <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_backup.html">BACKUP_POLICY</a>
               </p>
            </li>
            <li>
               <p>
                  <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_tag-policies.html">TAG_POLICY</a>
               </p>
            </li>
            <li>
               <p>
                  <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_chatbot.html">CHATBOT_POLICY</a>
               </p>
            </li>
            <li>
               <p>
                  <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_ai-opt-out.html">AISERVICES_OPT_OUT_POLICY</a>
               </p>
            </li>
            <li>
               <p>
                  <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_security_hub.html">SECURITYHUB_POLICY</a>
               </p>
            </li>
         </ul> |
| `description` | String | ✅ | <p>An optional description to assign to the policy.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `policy` | String | <p>A structure that contains details about the specified policy.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create policy
policy = provider.organizations.Policy {
    content = "value"  # <p>The policy text content to add to the new policy. The text that you supply must adhere
            to the rules of the policy type you specify in the <code>Type</code> parameter. </p>
         <p>The maximum size of a policy document depends on the policy's type. For more
            information, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html#min-max-values">Maximum and minimum values</a> in the
            <i>Organizations User Guide</i>.</p>
    name = "value"  # <p>The friendly name to assign to the policy.</p>
         <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> 
    that is used to validate this parameter is a string of any of the characters in the ASCII 
    character range.</p>
    type = "value"  # <p>The type of policy to create. You can specify one of the following values:</p>
         <ul>
            <li>
               <p>
                  <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_scp.html">SERVICE_CONTROL_POLICY</a>
               </p>
            </li>
            <li>
               <p>
                  <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_rcps.html">RESOURCE_CONTROL_POLICY</a>
               </p>
            </li>
            <li>
               <p>
                  <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_declarative.html">DECLARATIVE_POLICY_EC2</a>
               </p>
            </li>
            <li>
               <p>
                  <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_backup.html">BACKUP_POLICY</a>
               </p>
            </li>
            <li>
               <p>
                  <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_tag-policies.html">TAG_POLICY</a>
               </p>
            </li>
            <li>
               <p>
                  <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_chatbot.html">CHATBOT_POLICY</a>
               </p>
            </li>
            <li>
               <p>
                  <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_ai-opt-out.html">AISERVICES_OPT_OUT_POLICY</a>
               </p>
            </li>
            <li>
               <p>
                  <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_security_hub.html">SECURITYHUB_POLICY</a>
               </p>
            </li>
         </ul>
    description = "value"  # <p>An optional description to assign to the policy.</p>
}

# Access policy outputs
policy_id = policy.id
policy_policy = policy.policy
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple create_account_status resources
create_account_status_0 = provider.organizations.Create_account_status {
}
create_account_status_1 = provider.organizations.Create_account_status {
}
create_account_status_2 = provider.organizations.Create_account_status {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    create_account_status = provider.organizations.Create_account_status {
    }
```

---

## Related Documentation

- [AWS Organizations Documentation](https://docs.aws.amazon.com/organizations/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
