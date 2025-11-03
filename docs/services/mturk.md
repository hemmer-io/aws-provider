# Mturk Service



**Resources**: 14

---

## Overview

The mturk service provides access to 14 resource types:

- [Additional_assignments_for_hit](#additional_assignments_for_hit) [C]
- [Qualification_score](#qualification_score) [R]
- [Hit_with_hit_type](#hit_with_hit_type) [C]
- [Expiration_for_hit](#expiration_for_hit) [U]
- [Account_balance](#account_balance) [R]
- [Worker_block](#worker_block) [CD]
- [File_upload_url](#file_upload_url) [R]
- [Qualification_type](#qualification_type) [CRUD]
- [Hit_type](#hit_type) [C]
- [Hit](#hit) [CRD]
- [Assignment](#assignment) [R]
- [Hit_review_status](#hit_review_status) [U]
- [Hit_type_of_hit](#hit_type_of_hit) [U]
- [Notification_settings](#notification_settings) [U]

---

## Resources


### Additional_assignments_for_hit

AdditionalAssignmentsForHIT resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `unique_request_token` | String |  | <p>
            A unique identifier for this request, which allows you to retry the call on error
            without extending the HIT multiple times.
            This is useful in cases such as network timeouts where it is unclear whether or not
            the call succeeded on the server. If the extend HIT already exists in the system
            from a previous call using the same <code>UniqueRequestToken</code>,
            subsequent calls will return an error with a message containing the request ID.
        </p> |
| `hit_id` | String | ✅ | <p>The ID of the HIT to extend.</p> |
| `number_of_additional_assignments` | i64 | ✅ | <p>The number of additional assignments to request for this HIT.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create additional_assignments_for_hit
additional_assignments_for_hit = provider.mturk.Additional_assignments_for_hit {
    hit_id = "value"  # <p>The ID of the HIT to extend.</p>
    number_of_additional_assignments = "value"  # <p>The number of additional assignments to request for this HIT.</p>
}

```

---


### Qualification_score

QualificationScore resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `qualification` | String | <p> The Qualification data structure of the Qualification
            assigned to a user, including the Qualification type and the value
            (score).
        </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access qualification_score outputs
qualification_score_id = qualification_score.id
qualification_score_qualification = qualification_score.qualification
```

---


### Hit_with_hit_type

HITWithHITType resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `unique_request_token` | String |  | <p>
            A unique identifier for this request which allows you to retry the call
            on error without creating duplicate HITs.
            This is useful in cases such as network timeouts where it is unclear whether or not
            the call succeeded on the server.
            If the HIT already exists in the system from a previous call using the same UniqueRequestToken,
            subsequent calls will return a AWS.MechanicalTurk.HitAlreadyExists error
            with a message containing the HITId.
        </p>
        <note>
            <p>
                Note: It is your responsibility to ensure uniqueness of the token.
                The unique token expires after 24 hours. Subsequent calls using the same
                UniqueRequestToken made after the 24 hour limit could create duplicate HITs.
            </p>
        </note> |
| `question` | String |  | <p>
            The data the person completing the HIT uses to produce the results.
        </p>
        <p>
            Constraints: Must be a QuestionForm data structure, an ExternalQuestion data structure,
            or an HTMLQuestion data structure. The XML question data must not be larger than
            64 kilobytes (65,535 bytes) in size, including whitespace.
        </p>
        <p>Either a Question parameter or a HITLayoutId parameter must be provided.</p> |
| `hit_review_policy` | String |  | <p>
            The HIT-level Review Policy applies to the HIT.
            You can specify for Mechanical Turk to take various actions based on the policy.
        </p> |
| `lifetime_in_seconds` | i64 | ✅ | <p>
            An amount of time, in seconds, after which the HIT is no longer available for users to accept.
            After the lifetime of the HIT elapses, the HIT no longer appears in HIT searches,
            even if not all of the assignments for the HIT have been accepted.
        </p> |
| `max_assignments` | i64 |  | <p>
            The number of times the HIT can be accepted and completed before the HIT becomes unavailable.
        </p> |
| `hit_layout_id` | String |  | <p>
            The HITLayoutId allows you to use a pre-existing HIT design with placeholder values
            and create an additional HIT by providing those values as HITLayoutParameters.
        </p>
        <p>
            Constraints: Either a Question parameter or a HITLayoutId parameter must be provided.
        </p> |
| `hit_layout_parameters` | Vec<String> |  | <p>
            If the HITLayoutId is provided, any placeholder values must be filled in with values
            using the HITLayoutParameter structure. For more information, see HITLayout.
        </p> |
| `assignment_review_policy` | String |  | <p>
            The Assignment-level Review Policy applies to the assignments under the HIT.
            You can specify for Mechanical Turk to take various actions based on the policy.
        </p> |
| `hit_type_id` | String | ✅ | <p>The HIT type ID you want to create this HIT with.</p> |
| `requester_annotation` | String |  | <p>
            An arbitrary data field.
            The RequesterAnnotation parameter lets your application attach arbitrary data
            to the HIT for tracking purposes.
            For example, this parameter could be an identifier internal to the Requester's application
            that corresponds with the HIT.
        </p>
        <p>
            The RequesterAnnotation parameter for a HIT is only visible to the Requester who created the HIT.
            It is not shown to the Worker, or any other Requester.
        </p>
        <p>
            The RequesterAnnotation parameter may be different for each HIT you submit.
            It does not affect how your HITs are grouped.
        </p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create hit_with_hit_type
hit_with_hit_type = provider.mturk.Hit_with_hit_type {
    lifetime_in_seconds = "value"  # <p>
            An amount of time, in seconds, after which the HIT is no longer available for users to accept.
            After the lifetime of the HIT elapses, the HIT no longer appears in HIT searches,
            even if not all of the assignments for the HIT have been accepted.
        </p>
    hit_type_id = "value"  # <p>The HIT type ID you want to create this HIT with.</p>
}

```

---


### Expiration_for_hit

ExpirationForHIT resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `expire_at` | String | ✅ | <p>
            The date and time at which you want the HIT to expire
        </p> |
| `hit_id` | String | ✅ | <p>
            The HIT to update.
        </p> |



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


### Account_balance

AccountBalance resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `available_balance` | String |  |
| `on_hold_balance` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access account_balance outputs
account_balance_id = account_balance.id
account_balance_available_balance = account_balance.available_balance
account_balance_on_hold_balance = account_balance.on_hold_balance
```

---


### Worker_block

WorkerBlock resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `worker_id` | String | ✅ | <p>The ID of the Worker to block.</p> |
| `reason` | String | ✅ | <p>A message explaining the reason for blocking the Worker. This parameter enables you to keep track of your Workers. The Worker does not see this message.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create worker_block
worker_block = provider.mturk.Worker_block {
    worker_id = "value"  # <p>The ID of the Worker to block.</p>
    reason = "value"  # <p>A message explaining the reason for blocking the Worker. This parameter enables you to keep track of your Workers. The Worker does not see this message.</p>
}

```

---


### File_upload_url

FileUploadURL resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `file_upload_url` | String | <p> A temporary URL for the file that the Worker uploaded for
            the answer.
        </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access file_upload_url outputs
file_upload_url_id = file_upload_url.id
file_upload_url_file_upload_url = file_upload_url.file_upload_url
```

---


### Qualification_type

QualificationType resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `test` | String |  | <p>
            The questions for the Qualification test a Worker must answer
            correctly to obtain a Qualification of this type. If this parameter
            is specified,
            <code>TestDurationInSeconds</code>
            must also be specified.
        </p>
        <p>Constraints: Must not be longer than 65535 bytes. Must be a
            QuestionForm data structure. This parameter cannot be specified if
            AutoGranted is true.</p>
        <p>Constraints: None. If not specified, the Worker may request
            the Qualification without answering any questions.</p> |
| `answer_key` | String |  | <p>The answers to the Qualification test specified in the Test
            parameter, in the form of an AnswerKey data structure.</p>
        <p>Constraints: Must not be longer than 65535 bytes.</p>
        <p>Constraints: None. If not specified, you must process
            Qualification requests manually.</p> |
| `description` | String | ✅ | <p>A long description for the Qualification type. On the Amazon
            Mechanical Turk website, the long description is displayed when a
            Worker examines a Qualification type.</p> |
| `keywords` | String |  | <p>One or more words or phrases that describe the Qualification
            type, separated by commas. The keywords of a type make the type
            easier to find during a search.</p> |
| `test_duration_in_seconds` | i64 |  | <p>The number of seconds the Worker has to complete the
            Qualification test, starting from the time the Worker requests the
            Qualification.</p> |
| `auto_granted` | bool |  | <p>Specifies whether requests for the Qualification type are
            granted immediately, without prompting the Worker with a
            Qualification test.</p>
        <p>Constraints: If the Test parameter is specified, this
            parameter cannot be true.</p> |
| `retry_delay_in_seconds` | i64 |  | <p>The number of seconds that a Worker must wait after
            requesting a Qualification of the Qualification type before the
            worker can retry the Qualification request.</p>
        <p>Constraints: None. If not specified, retries are disabled and
            Workers can request a Qualification of this type only once, even if
            the Worker has not been granted the Qualification. It is not possible
            to disable retries for a Qualification type after it has been created
            with retries enabled. If you want to disable retries, you must delete
            existing retry-enabled Qualification type and then create a new
            Qualification type with retries disabled.</p> |
| `auto_granted_value` | i64 |  | <p>The Qualification value to use for automatically granted
            Qualifications. This parameter is used only if the AutoGranted
            parameter is true.</p> |
| `qualification_type_status` | String | ✅ | <p>The initial status of the Qualification type.</p>
        <p>Constraints: Valid values are: Active | Inactive</p> |
| `name` | String | ✅ | <p> The name you give to the Qualification type. The type name
            is used to represent the Qualification to Workers, and to find the
            type using a Qualification type search. It must be unique across all
            of your Qualification types.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `qualification_type` | String | <p> The returned Qualification Type</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create qualification_type
qualification_type = provider.mturk.Qualification_type {
    description = "value"  # <p>A long description for the Qualification type. On the Amazon
            Mechanical Turk website, the long description is displayed when a
            Worker examines a Qualification type.</p>
    qualification_type_status = "value"  # <p>The initial status of the Qualification type.</p>
        <p>Constraints: Valid values are: Active | Inactive</p>
    name = "value"  # <p> The name you give to the Qualification type. The type name
            is used to represent the Qualification to Workers, and to find the
            type using a Qualification type search. It must be unique across all
            of your Qualification types.</p>
}

# Access qualification_type outputs
qualification_type_id = qualification_type.id
qualification_type_qualification_type = qualification_type.qualification_type
```

---


### Hit_type

HITType resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `keywords` | String |  | <p>
            One or more words or phrases that describe the HIT, separated by commas.
            These words are used in searches to find HITs.
        </p> |
| `description` | String | ✅ | <p>
            A general description of the HIT. A description includes detailed information about the kind of task
            the HIT contains. On the Amazon Mechanical Turk web site, the HIT description appears in the expanded
            view of search results, and in the HIT and assignment screens. A good description gives the user enough
            information to evaluate the HIT before accepting it.
        </p> |
| `auto_approval_delay_in_seconds` | i64 |  | <p>
            The number of seconds after an assignment for the HIT has been submitted,
            after which the assignment is considered Approved automatically
            unless the Requester explicitly rejects it.
        </p> |
| `assignment_duration_in_seconds` | i64 | ✅ | <p>
            The amount of time, in seconds, that a Worker has to complete the HIT after accepting it.
            If a Worker does not complete the assignment within the specified duration,
            the assignment is considered abandoned. If the HIT is still active
            (that is, its lifetime has not elapsed), the assignment becomes available
            for other users to find and accept.
        </p> |
| `reward` | String | ✅ | <p>
            The amount of money the Requester will pay a Worker for successfully completing the HIT.
        </p> |
| `qualification_requirements` | Vec<String> |  | <p>
            Conditions that a Worker's Qualifications must meet in order
            to accept the HIT. A HIT can have between zero and ten
            Qualification requirements. All requirements must be met in
            order for a Worker to accept the HIT. Additionally, other
            actions can be restricted using the <code>ActionsGuarded</code>
            field on each <code>QualificationRequirement</code> structure.
        </p> |
| `title` | String | ✅ | <p>
            The title of the HIT. A title should be short and descriptive about the kind of task the HIT contains.
            On the Amazon Mechanical Turk web site, the HIT title appears in search results,
            and everywhere the HIT is mentioned.
        </p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create hit_type
hit_type = provider.mturk.Hit_type {
    description = "value"  # <p>
            A general description of the HIT. A description includes detailed information about the kind of task
            the HIT contains. On the Amazon Mechanical Turk web site, the HIT description appears in the expanded
            view of search results, and in the HIT and assignment screens. A good description gives the user enough
            information to evaluate the HIT before accepting it.
        </p>
    assignment_duration_in_seconds = "value"  # <p>
            The amount of time, in seconds, that a Worker has to complete the HIT after accepting it.
            If a Worker does not complete the assignment within the specified duration,
            the assignment is considered abandoned. If the HIT is still active
            (that is, its lifetime has not elapsed), the assignment becomes available
            for other users to find and accept.
        </p>
    reward = "value"  # <p>
            The amount of money the Requester will pay a Worker for successfully completing the HIT.
        </p>
    title = "value"  # <p>
            The title of the HIT. A title should be short and descriptive about the kind of task the HIT contains.
            On the Amazon Mechanical Turk web site, the HIT title appears in search results,
            and everywhere the HIT is mentioned.
        </p>
}

```

---


### Hit

HIT resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `qualification_requirements` | Vec<String> |  | <p>
            Conditions that a Worker's Qualifications must meet in order
            to accept the HIT. A HIT can have between zero and ten
            Qualification requirements. All requirements must be met in
            order for a Worker to accept the HIT. Additionally, other
            actions can be restricted using the <code>ActionsGuarded</code>
            field on each <code>QualificationRequirement</code> structure.
        </p> |
| `unique_request_token` | String |  | <p>
            A unique identifier for this request which allows you to retry the call
            on error without creating duplicate HITs.
            This is useful in cases such as network timeouts where it is unclear whether or not
            the call succeeded on the server.
            If the HIT already exists in the system from a previous call using the same UniqueRequestToken,
            subsequent calls will return a AWS.MechanicalTurk.HitAlreadyExists error
            with a message containing the HITId.
        </p>
        <note>
            <p>
                Note: It is your responsibility to ensure uniqueness of the token.
                The unique token expires after 24 hours. Subsequent calls using the same
                UniqueRequestToken made after the 24 hour limit could create duplicate HITs.
            </p>
        </note> |
| `keywords` | String |  | <p>
            One or more words or phrases that describe the HIT, separated by commas.
            These words are used in searches to find HITs.
        </p> |
| `hit_review_policy` | String |  | <p>
            The HIT-level Review Policy applies to the HIT.
            You can specify for Mechanical Turk to take various actions based on the policy.
        </p> |
| `auto_approval_delay_in_seconds` | i64 |  | <p>
            The number of seconds after an assignment for the HIT has been submitted,
            after which the assignment is considered Approved automatically
            unless the Requester explicitly rejects it.
        </p> |
| `description` | String | ✅ | <p>
            A general description of the HIT. A description includes detailed information about the kind of task
            the HIT contains. On the Amazon Mechanical Turk web site, the HIT description appears in the expanded
            view of search results, and in the HIT and assignment screens. A good description gives the user enough
            information to evaluate the HIT before accepting it.
        </p> |
| `reward` | String | ✅ | <p>
            The amount of money the Requester will pay a Worker for successfully completing the HIT.
        </p> |
| `hit_layout_id` | String |  | <p>
            The HITLayoutId allows you to use a pre-existing HIT design with placeholder values
            and create an additional HIT by providing those values as HITLayoutParameters.
        </p>
        <p>
            Constraints: Either a Question parameter or a HITLayoutId parameter must be provided.
        </p> |
| `hit_layout_parameters` | Vec<String> |  | <p>
            If the HITLayoutId is provided, any placeholder values must be filled in with values
            using the HITLayoutParameter structure. For more information, see HITLayout.
        </p> |
| `max_assignments` | i64 |  | <p>
            The number of times the HIT can be accepted and completed before the HIT becomes unavailable.
        </p> |
| `lifetime_in_seconds` | i64 | ✅ | <p>
            An amount of time, in seconds, after which the HIT is no longer available for users to accept.
            After the lifetime of the HIT elapses, the HIT no longer appears in HIT searches,
            even if not all of the assignments for the HIT have been accepted.
        </p> |
| `assignment_review_policy` | String |  | <p>
            The Assignment-level Review Policy applies to the assignments under the HIT.
            You can specify for Mechanical Turk to take various actions based on the policy.
        </p> |
| `assignment_duration_in_seconds` | i64 | ✅ | <p>
            The amount of time, in seconds, that a Worker has to complete the HIT after accepting it.
            If a Worker does not complete the assignment within the specified duration,
            the assignment is considered abandoned. If the HIT is still active
            (that is, its lifetime has not elapsed), the assignment becomes available
            for other users to find and accept.
        </p> |
| `requester_annotation` | String |  | <p>
            An arbitrary data field.
            The RequesterAnnotation parameter lets your application attach arbitrary data
            to the HIT for tracking purposes.
            For example, this parameter could be an identifier internal to the Requester's application
            that corresponds with the HIT.
        </p>
        <p>
            The RequesterAnnotation parameter for a HIT is only visible to the Requester who created the HIT.
            It is not shown to the Worker, or any other Requester.
        </p>
        <p>
            The RequesterAnnotation parameter may be different for each HIT you submit.
            It does not affect how your HITs are grouped.
        </p> |
| `title` | String | ✅ | <p>
            The title of the HIT. A title should be short and descriptive about the kind of task the HIT contains.
            On the Amazon Mechanical Turk web site, the HIT title appears in search results,
            and everywhere the HIT is mentioned.
        </p> |
| `question` | String |  | <p>
            The data the person completing the HIT uses to produce the results.
        </p>
        <p>
            Constraints: Must be a QuestionForm data structure, an ExternalQuestion data structure,
            or an HTMLQuestion data structure. The XML question data must not be larger than
            64 kilobytes (65,535 bytes) in size, including whitespace.
        </p>
        <p>Either a Question parameter or a HITLayoutId parameter must be provided.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `hit` | String | <p> Contains the requested HIT data.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create hit
hit = provider.mturk.Hit {
    description = "value"  # <p>
            A general description of the HIT. A description includes detailed information about the kind of task
            the HIT contains. On the Amazon Mechanical Turk web site, the HIT description appears in the expanded
            view of search results, and in the HIT and assignment screens. A good description gives the user enough
            information to evaluate the HIT before accepting it.
        </p>
    reward = "value"  # <p>
            The amount of money the Requester will pay a Worker for successfully completing the HIT.
        </p>
    lifetime_in_seconds = "value"  # <p>
            An amount of time, in seconds, after which the HIT is no longer available for users to accept.
            After the lifetime of the HIT elapses, the HIT no longer appears in HIT searches,
            even if not all of the assignments for the HIT have been accepted.
        </p>
    assignment_duration_in_seconds = "value"  # <p>
            The amount of time, in seconds, that a Worker has to complete the HIT after accepting it.
            If a Worker does not complete the assignment within the specified duration,
            the assignment is considered abandoned. If the HIT is still active
            (that is, its lifetime has not elapsed), the assignment becomes available
            for other users to find and accept.
        </p>
    title = "value"  # <p>
            The title of the HIT. A title should be short and descriptive about the kind of task the HIT contains.
            On the Amazon Mechanical Turk web site, the HIT title appears in search results,
            and everywhere the HIT is mentioned.
        </p>
}

# Access hit outputs
hit_id = hit.id
hit_hit = hit.hit
```

---


### Assignment

Assignment resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `assignment` | String | <p> The assignment. The response includes one Assignment
            element.
        </p> |
| `hit` | String | <p> The HIT associated with this assignment. The response
            includes one HIT element.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access assignment outputs
assignment_id = assignment.id
assignment_assignment = assignment.assignment
assignment_hit = assignment.hit
```

---


### Hit_review_status

HITReviewStatus resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `hit_id` | String | ✅ | <p>
            The ID of the HIT to update.
        </p> |
| `revert` | bool |  | <p>
            Specifies how to update the HIT status. Default is <code>False</code>.
        </p>
        <ul>
            <li>
                <p>
                    Setting this to false will only transition a HIT from <code>Reviewable</code> to <code>Reviewing</code>
                </p>
            </li>
            <li>
                <p>
                    Setting this to true will only transition a HIT from <code>Reviewing</code> to <code>Reviewable</code>
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

```

---


### Hit_type_of_hit

HITTypeOfHIT resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `hit_id` | String | ✅ | <p>The HIT to update.</p> |
| `hit_type_id` | String | ✅ | <p>The ID of the new HIT type.</p> |



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


### Notification_settings

NotificationSettings resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `active` | bool |  | <p>
            Specifies whether notifications are sent for HITs of this HIT type,
            according to the notification specification.
            You must specify either the Notification parameter or the Active parameter
            for the call to UpdateNotificationSettings to succeed.
        </p> |
| `hit_type_id` | String | ✅ | <p>
            The ID of the HIT type whose notification specification is being updated.
        </p> |
| `notification` | String |  | <p>
            The notification specification for the HIT type.
        </p> |



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

# Create multiple additional_assignments_for_hit resources
additional_assignments_for_hit_0 = provider.mturk.Additional_assignments_for_hit {
    hit_id = "value-0"
    number_of_additional_assignments = "value-0"
}
additional_assignments_for_hit_1 = provider.mturk.Additional_assignments_for_hit {
    hit_id = "value-1"
    number_of_additional_assignments = "value-1"
}
additional_assignments_for_hit_2 = provider.mturk.Additional_assignments_for_hit {
    hit_id = "value-2"
    number_of_additional_assignments = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    additional_assignments_for_hit = provider.mturk.Additional_assignments_for_hit {
        hit_id = "production-value"
        number_of_additional_assignments = "production-value"
    }
```

---

## Related Documentation

- [AWS Mturk Documentation](https://docs.aws.amazon.com/mturk/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
