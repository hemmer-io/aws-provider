# Route_53_domains Service



**Resources**: 9

---

## Overview

The route_53_domains service provides access to 9 resource types:

- [Contact_reachability_status](#contact_reachability_status) [R]
- [Domain](#domain) [D]
- [Tags_for_domain](#tags_for_domain) [UD]
- [Domain_suggestions](#domain_suggestions) [R]
- [Domain_detail](#domain_detail) [R]
- [Domain_contact](#domain_contact) [U]
- [Operation_detail](#operation_detail) [R]
- [Domain_contact_privacy](#domain_contact_privacy) [U]
- [Domain_nameservers](#domain_nameservers) [U]

---

## Resources


### Contact_reachability_status

ContactReachabilityStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | String | <p>Whether the registrant contact has responded. Values include the following:</p>
         <dl>
            <dt>PENDING</dt>
            <dd>
               <p>We sent the confirmation email and haven't received a response yet.</p>
            </dd>
            <dt>DONE</dt>
            <dd>
               <p>We sent the email and got confirmation from the registrant contact.</p>
            </dd>
            <dt>EXPIRED</dt>
            <dd>
               <p>The time limit expired before the registrant contact responded.</p>
            </dd>
         </dl> |
| `domain_name` | String | <p>The domain name for which you requested the reachability status.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access contact_reachability_status outputs
contact_reachability_status_id = contact_reachability_status.id
contact_reachability_status_status = contact_reachability_status.status
contact_reachability_status_domain_name = contact_reachability_status.domain_name
```

---


### Domain

Domain resource

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


### Tags_for_domain

TagsForDomain resource

**Operations**: ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `domain_name` | String | ✅ | <p>The domain for which you want to add or update tags.</p> |
| `tags_to_update` | Vec<String> |  | <p>A list of the tag keys and values that you want to add or update. If you specify a key
			that already exists, the corresponding value will be replaced.</p> |



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


### Domain_suggestions

DomainSuggestions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `suggestions_list` | Vec<String> | <p>A list of possible domain names. If you specified <code>true</code> for
				<code>OnlyAvailable</code> in the request, the list contains only domains that are
			available for registration.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access domain_suggestions outputs
domain_suggestions_id = domain_suggestions.id
domain_suggestions_suggestions_list = domain_suggestions.suggestions_list
```

---


### Domain_detail

DomainDetail resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `abuse_contact_email` | String | <p>Email address to contact to report incorrect contact information for a domain, to
			report that the domain is being used to send spam, to report that someone is
			cybersquatting on a domain name, or report some other type of abuse.</p> |
| `auto_renew` | bool | <p>Specifies whether the domain registration is set to renew automatically.</p> |
| `registrant_contact` | String | <p>Provides details about the domain registrant.</p> |
| `dnssec_keys` | Vec<String> | <p>A complex type that contains information about the DNSSEC configuration.</p> |
| `registrar_url` | String | <p>Web address of the registrar.</p> |
| `billing_contact` | String | <p>Provides details about the domain billing contact.</p> |
| `dns_sec` | String | <p>Deprecated.</p> |
| `reseller` | String | <p>Reseller of the domain. Domains registered or transferred using Route 53 domains will
			have <code>"Amazon"</code> as the reseller. </p> |
| `status_list` | Vec<String> | <p>An array of domain name status codes, also known as Extensible Provisioning Protocol
			(EPP) status codes.</p>
         <p>ICANN, the organization that maintains a central database of domain names, has
			developed a set of domain name status codes that tell you the status of a variety of
			operations on a domain name, for example, registering a domain name, transferring a
			domain name to another registrar, renewing the registration for a domain name, and so
			on. All registrars use this same set of status codes.</p>
         <p>For a current list of domain name status codes and an explanation of what each code
			means, go to the <a href="https://www.icann.org/">ICANN website</a> and search
			for <code>epp status codes</code>. (Search on the ICANN website; web searches sometimes
			return an old version of the document.)</p> |
| `registrant_privacy` | bool | <p>Specifies whether contact information is concealed from WHOIS queries. If the value is
				<code>true</code>, WHOIS ("who is") queries return contact information either for
			Amazon Registrar or for our registrar associate,
			Gandi. If the value is <code>false</code>, WHOIS queries return the
			information that you entered for the registrant contact (domain owner).</p> |
| `nameservers` | Vec<String> | <p>The name servers of the domain.</p> |
| `registry_domain_id` | String | <p>Reserved for future use.</p> |
| `updated_date` | String | <p>The last updated date of the domain as found in the response to a WHOIS query. The
			date and time is in Unix time format and Coordinated Universal time (UTC).</p> |
| `billing_privacy` | bool | <p>Specifies whether contact information is concealed from WHOIS queries. If the value is
		<code>true</code>, WHOIS ("who is") queries return contact information either for
		Amazon Registrar or for our registrar associate,
		Gandi. If the value is <code>false</code>, WHOIS queries return the
		information that you entered for the billing contact.</p> |
| `registrar_name` | String | <p>Name of the registrar of the domain as identified in the registry. </p> |
| `expiration_date` | String | <p>The date when the registration for the domain is set to expire. The date and time is
			in Unix time format and Coordinated Universal time (UTC).</p> |
| `abuse_contact_phone` | String | <p>Phone number for reporting abuse.</p> |
| `domain_name` | String | <p>The name of a domain.</p> |
| `tech_contact` | String | <p>Provides details about the domain technical contact.</p> |
| `admin_contact` | String | <p>Provides details about the domain administrative contact.</p> |
| `admin_privacy` | bool | <p>Specifies whether contact information is concealed from WHOIS queries. If the value is
				<code>true</code>, WHOIS ("who is") queries return contact information either for
			Amazon Registrar or for our registrar associate,
			Gandi. If the value is <code>false</code>, WHOIS queries return the
			information that you entered for the admin contact.</p> |
| `tech_privacy` | bool | <p>Specifies whether contact information is concealed from WHOIS queries. If the value is
				<code>true</code>, WHOIS ("who is") queries return contact information either for
			Amazon Registrar or for our registrar associate,
			Gandi. If the value is <code>false</code>, WHOIS queries return the
			information that you entered for the technical contact.</p> |
| `who_is_server` | String | <p>The fully qualified name of the WHOIS server that can answer the WHOIS query for the
			domain.</p> |
| `creation_date` | String | <p>The date when the domain was created as found in the response to a WHOIS query. The
			date and time is in Unix time format and Coordinated Universal time (UTC).</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access domain_detail outputs
domain_detail_id = domain_detail.id
domain_detail_abuse_contact_email = domain_detail.abuse_contact_email
domain_detail_auto_renew = domain_detail.auto_renew
domain_detail_registrant_contact = domain_detail.registrant_contact
domain_detail_dnssec_keys = domain_detail.dnssec_keys
domain_detail_registrar_url = domain_detail.registrar_url
domain_detail_billing_contact = domain_detail.billing_contact
domain_detail_dns_sec = domain_detail.dns_sec
domain_detail_reseller = domain_detail.reseller
domain_detail_status_list = domain_detail.status_list
domain_detail_registrant_privacy = domain_detail.registrant_privacy
domain_detail_nameservers = domain_detail.nameservers
domain_detail_registry_domain_id = domain_detail.registry_domain_id
domain_detail_updated_date = domain_detail.updated_date
domain_detail_billing_privacy = domain_detail.billing_privacy
domain_detail_registrar_name = domain_detail.registrar_name
domain_detail_expiration_date = domain_detail.expiration_date
domain_detail_abuse_contact_phone = domain_detail.abuse_contact_phone
domain_detail_domain_name = domain_detail.domain_name
domain_detail_tech_contact = domain_detail.tech_contact
domain_detail_admin_contact = domain_detail.admin_contact
domain_detail_admin_privacy = domain_detail.admin_privacy
domain_detail_tech_privacy = domain_detail.tech_privacy
domain_detail_who_is_server = domain_detail.who_is_server
domain_detail_creation_date = domain_detail.creation_date
```

---


### Domain_contact

DomainContact resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `admin_contact` | String |  | <p>Provides detailed contact information.</p> |
| `tech_contact` | String |  | <p>Provides detailed contact information.</p> |
| `billing_contact` | String |  | <p>Provides detailed contact information.</p> |
| `registrant_contact` | String |  | <p>Provides detailed contact information.</p> |
| `consent` | String |  | <p> Customer's consent for the owner change request. Required if the domain is not free (consent price is more than $0.00).</p> |
| `domain_name` | String | ✅ | <p>The name of the domain that you want to update contact information for.</p> |



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


### Operation_detail

OperationDetail resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status_flag` | String | <p> Lists any outstanding operations that require customer action. Valid values
			are:</p>
         <ul>
            <li>
               <p>
                  <code>PENDING_ACCEPTANCE</code>: The operation is waiting for acceptance from
					the account that is receiving the domain.</p>
            </li>
            <li>
               <p>
                  <code>PENDING_CUSTOMER_ACTION</code>: The operation is waiting for customer
					action, for example, returning an email.</p>
            </li>
            <li>
               <p>
                  <code>PENDING_AUTHORIZATION</code>: The operation is waiting for the form of
					authorization. For more information, see <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_domains_ResendOperationAuthorization.html">ResendOperationAuthorization</a>.</p>
            </li>
            <li>
               <p>
                  <code>PENDING_PAYMENT_VERIFICATION</code>: The operation is waiting for the
					payment method to validate.</p>
            </li>
            <li>
               <p>
                  <code>PENDING_SUPPORT_CASE</code>: The operation includes a support case and
					is waiting for its resolution.</p>
            </li>
         </ul> |
| `domain_name` | String | <p>The name of a domain.</p> |
| `type` | String | <p>The type of operation that was requested.</p> |
| `last_updated_date` | String | <p> The date when the operation was last updated. </p> |
| `status` | String | <p>The current status of the requested operation in the system.</p> |
| `submitted_date` | String | <p>The date when the request was submitted.</p> |
| `operation_id` | String | <p>The identifier for the operation.</p> |
| `message` | String | <p>Detailed information on the status including possible errors.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access operation_detail outputs
operation_detail_id = operation_detail.id
operation_detail_status_flag = operation_detail.status_flag
operation_detail_domain_name = operation_detail.domain_name
operation_detail_type = operation_detail.type
operation_detail_last_updated_date = operation_detail.last_updated_date
operation_detail_status = operation_detail.status
operation_detail_submitted_date = operation_detail.submitted_date
operation_detail_operation_id = operation_detail.operation_id
operation_detail_message = operation_detail.message
```

---


### Domain_contact_privacy

DomainContactPrivacy resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `registrant_privacy` | bool |  | <p>Whether you want to conceal contact information from WHOIS queries. If you specify
			<code>true</code>, WHOIS ("who is") queries return contact information either for
			Amazon Registrar or for our registrar associate,
			Gandi. If you specify <code>false</code>, WHOIS queries return the
			information that you entered for the registrant contact (domain owner).</p>
         <note>
            <p>You must specify the same privacy setting for the administrative, billing, registrant, and
				technical contacts.</p>
         </note> |
| `tech_privacy` | bool |  | <p>Whether you want to conceal contact information from WHOIS queries. If you specify
				<code>true</code>, WHOIS ("who is") queries return contact information either for
			Amazon Registrar or for our registrar associate,
			Gandi. If you specify <code>false</code>, WHOIS queries return the
			information that you entered for the technical contact.</p>
         <note>
            <p>You must specify the same privacy setting for the administrative, billing, registrant, and
				technical contacts.</p>
         </note> |
| `admin_privacy` | bool |  | <p>Whether you want to conceal contact information from WHOIS queries. If you specify
			<code>true</code>, WHOIS ("who is") queries return contact information either for
			Amazon Registrar or for our registrar associate,
			Gandi. If you specify <code>false</code>, WHOIS queries return the
			information that you entered for the admin contact.</p>
         <note>
            <p>You must specify the same privacy setting for the administrative, billing, registrant, and
				technical contacts.</p>
         </note> |
| `domain_name` | String | ✅ | <p>The name of the domain that you want to update the privacy setting for.</p> |
| `billing_privacy` | bool |  | <p>
			Whether you want to conceal contact information from WHOIS queries. If you specify
			<code>true</code>, WHOIS ("who is") queries return contact information either for
			Amazon Registrar or for our registrar associate,
			Gandi. If you specify <code>false</code>, WHOIS queries return the
			information that you entered for the billing contact.
		</p>
         <note>
            <p>You must specify the same privacy setting for the administrative, billing, registrant, and
				technical contacts.</p>
         </note> |



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


### Domain_nameservers

DomainNameservers resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `nameservers` | Vec<String> | ✅ | <p>A list of new name servers for the domain.</p> |
| `fi_auth_key` | String |  | <p>The authorization key for .fi domains</p> |
| `domain_name` | String | ✅ | <p>The name of the domain that you want to change name servers for.</p> |



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

# Create multiple contact_reachability_status resources
contact_reachability_status_0 = provider.route_53_domains.Contact_reachability_status {
}
contact_reachability_status_1 = provider.route_53_domains.Contact_reachability_status {
}
contact_reachability_status_2 = provider.route_53_domains.Contact_reachability_status {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    contact_reachability_status = provider.route_53_domains.Contact_reachability_status {
    }
```

---

## Related Documentation

- [AWS Route_53_domains Documentation](https://docs.aws.amazon.com/route_53_domains/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
