# Support Service



**Resources**: 12

---

## Overview

The support service provides access to 12 resource types:

- [Attachment](#attachment) [R]
- [Communications](#communications) [R]
- [Severity_levels](#severity_levels) [R]
- [Trusted_advisor_checks](#trusted_advisor_checks) [R]
- [Trusted_advisor_check_summaries](#trusted_advisor_check_summaries) [R]
- [Create_case_options](#create_case_options) [R]
- [Trusted_advisor_check_result](#trusted_advisor_check_result) [R]
- [Cases](#cases) [R]
- [Services](#services) [R]
- [Supported_languages](#supported_languages) [R]
- [Trusted_advisor_check_refresh_statuses](#trusted_advisor_check_refresh_statuses) [R]
- [Case](#case) [C]

---

## Resources


### Attachment

Attachment resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `attachment` | String | <p>This object includes the attachment content and file name.</p>
         <p>In the previous response syntax, the value for the <code>data</code> parameter appears
            as <code>blob</code>, which is represented as a base64-encoded string. The value for
                <code>fileName</code> is the name of the attachment, such as
                <code>troubleshoot-screenshot.png</code>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access attachment outputs
attachment_id = attachment.id
attachment_attachment = attachment.attachment
```

---


### Communications

Communications resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>A resumption point for pagination.</p> |
| `communications` | Vec<String> | <p>The communications for the case.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access communications outputs
communications_id = communications.id
communications_next_token = communications.next_token
communications_communications = communications.communications
```

---


### Severity_levels

SeverityLevels resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `severity_levels` | Vec<String> | <p>The available severity levels for the support case. Available severity levels are
            defined by your service level agreement with Amazon Web Services.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access severity_levels outputs
severity_levels_id = severity_levels.id
severity_levels_severity_levels = severity_levels.severity_levels
```

---


### Trusted_advisor_checks

TrustedAdvisorChecks resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `checks` | Vec<String> | <p>Information about all available Trusted Advisor checks.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access trusted_advisor_checks outputs
trusted_advisor_checks_id = trusted_advisor_checks.id
trusted_advisor_checks_checks = trusted_advisor_checks.checks
```

---


### Trusted_advisor_check_summaries

TrustedAdvisorCheckSummaries resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `summaries` | Vec<String> | <p>The summary information for the requested Trusted Advisor checks.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access trusted_advisor_check_summaries outputs
trusted_advisor_check_summaries_id = trusted_advisor_check_summaries.id
trusted_advisor_check_summaries_summaries = trusted_advisor_check_summaries.summaries
```

---


### Create_case_options

CreateCaseOptions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `communication_types` | Vec<String> | <p>
        A JSON-formatted array that contains the available communication type options, along with the available support 
        timeframes for the given inputs.
        </p> |
| `language_availability` | String | <p>Language availability can be any of the following:</p>
         <ul>
            <li>
               <p>
                    available
                </p>
            </li>
            <li>
               <p>
                    best_effort
                </p>
            </li>
            <li>
               <p>
                    unavailable
                </p>
            </li>
         </ul> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access create_case_options outputs
create_case_options_id = create_case_options.id
create_case_options_communication_types = create_case_options.communication_types
create_case_options_language_availability = create_case_options.language_availability
```

---


### Trusted_advisor_check_result

TrustedAdvisorCheckResult resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `result` | String | <p>The detailed results of the Trusted Advisor check.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access trusted_advisor_check_result outputs
trusted_advisor_check_result_id = trusted_advisor_check_result.id
trusted_advisor_check_result_result = trusted_advisor_check_result.result
```

---


### Cases

Cases resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `cases` | Vec<String> | <p>The details for the cases that match the request.</p> |
| `next_token` | String | <p>A resumption point for pagination.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access cases outputs
cases_id = cases.id
cases_cases = cases.cases
cases_next_token = cases.next_token
```

---


### Services

Services resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `services` | Vec<String> | <p>A JSON-formatted list of Amazon Web Services services.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access services outputs
services_id = services.id
services_services = services.services
```

---


### Supported_languages

SupportedLanguages resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `supported_languages` | Vec<String> | <p>
        A JSON-formatted array that contains the available ISO 639-1 language codes.
        </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access supported_languages outputs
supported_languages_id = supported_languages.id
supported_languages_supported_languages = supported_languages.supported_languages
```

---


### Trusted_advisor_check_refresh_statuses

TrustedAdvisorCheckRefreshStatuses resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `statuses` | Vec<String> | <p>The refresh status of the specified Trusted Advisor checks.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access trusted_advisor_check_refresh_statuses outputs
trusted_advisor_check_refresh_statuses_id = trusted_advisor_check_refresh_statuses.id
trusted_advisor_check_refresh_statuses_statuses = trusted_advisor_check_refresh_statuses.statuses
```

---


### Case

Case resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `attachment_set_id` | String |  | <p>The ID of a set of one or more attachments for the case. Create the set by using the
                <a>AddAttachmentsToSet</a> operation.</p> |
| `communication_body` | String | ✅ | <p>The communication body text that describes the issue. This text appears in the
                <b>Description</b> field on the Amazon Web Services Support Center <a href="https://console.aws.amazon.com/support/home#/case/create">Create Case</a> page.</p> |
| `issue_type` | String |  | <p>The type of issue for the case. You can specify <code>customer-service</code> or
                <code>technical</code>. If you don't specify a value, the default is
                <code>technical</code>.</p> |
| `category_code` | String |  | <p>The category of problem for the support case. You also use the <a>DescribeServices</a> operation to get the category code for a service. Each
            Amazon Web Services service defines its own set of category codes.</p> |
| `cc_email_addresses` | Vec<String> |  | <p>A list of email addresses that Amazon Web Services Support copies on case correspondence. Amazon Web Services Support
            identifies the account that creates the case when you specify your Amazon Web Services credentials in
            an HTTP POST method or use the <a href="http://aws.amazon.com/tools/">Amazon Web Services SDKs</a>.
        </p> |
| `severity_code` | String |  | <p>A value that indicates the urgency of the case. This value determines the response
            time according to your service level agreement with Amazon Web Services Support. You can use the <a>DescribeSeverityLevels</a> operation to get the possible values for
                <code>severityCode</code>. </p>
         <p>For more information, see <a>SeverityLevel</a> and <a href="https://docs.aws.amazon.com/awssupport/latest/user/getting-started.html#choosing-severity">Choosing a
                Severity</a> in the <i>Amazon Web Services Support User Guide</i>.</p>
         <note>
            <p>The availability of severity levels depends on the support plan for the
                Amazon Web Services account.</p>
         </note> |
| `language` | String |  | <p>The language in which Amazon Web Services Support handles the case. Amazon Web Services Support
currently supports Chinese (“zh”), English ("en"), Japanese ("ja") and Korean (“ko”). You must specify the ISO 639-1
code for the <code>language</code> parameter if you want support in that language.</p> |
| `service_code` | String |  | <p>The code for the Amazon Web Services service. You can use the <a>DescribeServices</a>
            operation to get the possible <code>serviceCode</code> values.</p> |
| `subject` | String | ✅ | <p>The title of the support case. The title appears in the <b>Subject</b> field on the Amazon Web Services Support Center <a href="https://console.aws.amazon.com/support/home#/case/create">Create Case</a> page.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create case
case = provider.support.Case {
    communication_body = "value"  # <p>The communication body text that describes the issue. This text appears in the
                <b>Description</b> field on the Amazon Web Services Support Center <a href="https://console.aws.amazon.com/support/home#/case/create">Create Case</a> page.</p>
    subject = "value"  # <p>The title of the support case. The title appears in the <b>Subject</b> field on the Amazon Web Services Support Center <a href="https://console.aws.amazon.com/support/home#/case/create">Create Case</a> page.</p>
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

# Create multiple attachment resources
attachment_0 = provider.support.Attachment {
}
attachment_1 = provider.support.Attachment {
}
attachment_2 = provider.support.Attachment {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    attachment = provider.support.Attachment {
    }
```

---

## Related Documentation

- [AWS Support Documentation](https://docs.aws.amazon.com/support/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
