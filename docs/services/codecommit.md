# Codecommit Service



**Resources**: 36

---

## Overview

The codecommit service provides access to 36 resource types:

- [Comments_for_compared_commit](#comments_for_compared_commit) [R]
- [Pull_request_description](#pull_request_description) [U]
- [Commit](#commit) [CR]
- [Repository](#repository) [CRD]
- [Comment_reaction](#comment_reaction) [C]
- [Folder](#folder) [R]
- [Pull_request_events](#pull_request_events) [R]
- [Approval_rule_template_description](#approval_rule_template_description) [U]
- [Comment_content](#comment_content) [D]
- [Pull_request_approval_states](#pull_request_approval_states) [R]
- [Pull_request_approval_rule](#pull_request_approval_rule) [CD]
- [Merge_conflicts](#merge_conflicts) [R]
- [Comment_reactions](#comment_reactions) [R]
- [Default_branch](#default_branch) [U]
- [Pull_request_approval_state](#pull_request_approval_state) [U]
- [Repository_description](#repository_description) [U]
- [Branch](#branch) [CRD]
- [Repository_triggers](#repository_triggers) [CR]
- [File](#file) [CRD]
- [Pull_request_status](#pull_request_status) [U]
- [Blob](#blob) [R]
- [Pull_request](#pull_request) [CR]
- [Comment](#comment) [RU]
- [Comments_for_pull_request](#comments_for_pull_request) [R]
- [Approval_rule_template_name](#approval_rule_template_name) [U]
- [Pull_request_approval_rule_content](#pull_request_approval_rule_content) [U]
- [Repository_name](#repository_name) [U]
- [Approval_rule_template](#approval_rule_template) [CRD]
- [Merge_commit](#merge_commit) [R]
- [Repository_encryption_key](#repository_encryption_key) [U]
- [Approval_rule_template_content](#approval_rule_template_content) [U]
- [Pull_request_override_state](#pull_request_override_state) [R]
- [Merge_options](#merge_options) [R]
- [Pull_request_title](#pull_request_title) [U]
- [Differences](#differences) [R]
- [Unreferenced_merge_commit](#unreferenced_merge_commit) [C]

---

## Resources


### Comments_for_compared_commit

CommentsForComparedCommit resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `comments_for_compared_commit_data` | Vec<String> | <p>A list of comment objects on the compared commit.</p> |
| `next_token` | String | <p>An enumeration token that can be used in a request to return the next batch of the results.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access comments_for_compared_commit outputs
comments_for_compared_commit_id = comments_for_compared_commit.id
comments_for_compared_commit_comments_for_compared_commit_data = comments_for_compared_commit.comments_for_compared_commit_data
comments_for_compared_commit_next_token = comments_for_compared_commit.next_token
```

---


### Pull_request_description

PullRequestDescription resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String | ✅ | <p>The updated content of the description for the pull request. This content replaces the
            existing description.</p> |
| `pull_request_id` | String | ✅ | <p>The system-generated ID of the pull request. To get this ID, use <a>ListPullRequests</a>.</p> |



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


### Commit

Commit resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `repository_name` | String | ✅ | <p>The name of the repository where you create the commit.</p> |
| `branch_name` | String | ✅ | <p>The name of the branch where you create the commit.</p> |
| `delete_files` | Vec<String> |  | <p>The files to delete in this commit. These files still exist in earlier commits.</p> |
| `author_name` | String |  | <p>The name of the author who created the commit. This information is used as both the
            author and committer for the commit.</p> |
| `parent_commit_id` | String |  | <p>The ID of the commit that is the parent of the commit you create. Not required if this
            is an empty repository.</p> |
| `commit_message` | String |  | <p>The commit message you want to include in the commit. Commit messages are limited to
            256 KB. If no message is specified, a default message is used.</p> |
| `set_file_modes` | Vec<String> |  | <p>The file modes to update for files in this commit.</p> |
| `email` | String |  | <p>The email address of the person who created the commit.</p> |
| `put_files` | Vec<String> |  | <p>The files to add or update in this commit.</p> |
| `keep_empty_folders` | bool |  | <p>If the commit contains deletions, whether to keep a folder or folder structure if the
            changes leave the folders empty. If true, a ..gitkeep file is created for empty folders.
            The default is false.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `commit` | String | <p>A commit data type object that contains information about the specified commit.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create commit
commit = provider.codecommit.Commit {
    repository_name = "value"  # <p>The name of the repository where you create the commit.</p>
    branch_name = "value"  # <p>The name of the branch where you create the commit.</p>
}

# Access commit outputs
commit_id = commit.id
commit_commit = commit.commit
```

---


### Repository

Repository resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | HashMap<String, String> |  | <p>One or more tag key-value pairs to use when tagging this repository.</p> |
| `repository_description` | String |  | <p>A comment or description about the new repository.</p>
         <note>
            <p>The description field for a repository accepts all HTML characters and all valid
                Unicode characters. Applications that do not HTML-encode the description and display
                it in a webpage can expose users to potentially malicious code. Make sure that you
                HTML-encode the description field in any application that uses this API to display
                the repository description on a webpage.</p>
         </note> |
| `repository_name` | String | ✅ | <p>The name of the new repository to be created.</p>
         <note>
            <p>The repository name must be unique across the calling Amazon Web Services account. Repository names
                are limited to 100 alphanumeric, dash, and underscore characters, and cannot include
                certain characters. For more information about the limits on repository names, see
                    <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/limits.html">Quotas</a> in the <i>CodeCommit User Guide</i>. The
                suffix .git is prohibited.</p>
         </note> |
| `kms_key_id` | String |  | <p>The ID of the encryption key. You can view the ID of an encryption key in the KMS console, or use the KMS APIs to
            programmatically retrieve a key ID. For more information about acceptable values for kmsKeyID, see 
            <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_Decrypt.html#KMS-Decrypt-request-KeyId">KeyId</a> in the Decrypt API description in 
            the <i>Key Management Service API Reference</i>.</p>
         <p>If no key is specified, the default <code>aws/codecommit</code> Amazon Web Services managed key is used.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `repository_metadata` | String | <p>Information about the repository.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create repository
repository = provider.codecommit.Repository {
    repository_name = "value"  # <p>The name of the new repository to be created.</p>
         <note>
            <p>The repository name must be unique across the calling Amazon Web Services account. Repository names
                are limited to 100 alphanumeric, dash, and underscore characters, and cannot include
                certain characters. For more information about the limits on repository names, see
                    <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/limits.html">Quotas</a> in the <i>CodeCommit User Guide</i>. The
                suffix .git is prohibited.</p>
         </note>
}

# Access repository outputs
repository_id = repository.id
repository_repository_metadata = repository.repository_metadata
```

---


### Comment_reaction

CommentReaction resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `comment_id` | String | ✅ | <p>The ID of the comment to which you want to add or update a reaction.</p> |
| `reaction_value` | String | ✅ | <p>The emoji reaction you want to add or update. To remove a reaction, provide a value of blank or null. You can also provide the value of none.
            For information about emoji reaction values supported in CodeCommit, see the <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/how-to-commit-comment.html#emoji-reaction-table">CodeCommit User Guide</a>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create comment_reaction
comment_reaction = provider.codecommit.Comment_reaction {
    comment_id = "value"  # <p>The ID of the comment to which you want to add or update a reaction.</p>
    reaction_value = "value"  # <p>The emoji reaction you want to add or update. To remove a reaction, provide a value of blank or null. You can also provide the value of none.
            For information about emoji reaction values supported in CodeCommit, see the <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/how-to-commit-comment.html#emoji-reaction-table">CodeCommit User Guide</a>.</p>
}

```

---


### Folder

Folder resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `symbolic_links` | Vec<String> | <p>The list of symbolic links to other files and folders in the specified folder, if
            any.</p> |
| `files` | Vec<String> | <p>The list of files in the specified folder, if any.</p> |
| `sub_modules` | Vec<String> | <p>The list of submodules in the specified folder, if any.</p> |
| `folder_path` | String | <p>The fully qualified path of the folder whose contents are returned.</p> |
| `commit_id` | String | <p>The full commit ID used as a reference for the returned version of the folder
            content.</p> |
| `tree_id` | String | <p>The full SHA-1 pointer of the tree information for the commit that contains the folder.</p> |
| `sub_folders` | Vec<String> | <p>The list of folders that exist under the specified folder, if any.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access folder outputs
folder_id = folder.id
folder_symbolic_links = folder.symbolic_links
folder_files = folder.files
folder_sub_modules = folder.sub_modules
folder_folder_path = folder.folder_path
folder_commit_id = folder.commit_id
folder_tree_id = folder.tree_id
folder_sub_folders = folder.sub_folders
```

---


### Pull_request_events

PullRequestEvents resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `pull_request_events` | Vec<String> | <p>Information about the pull request events.</p> |
| `next_token` | String | <p>An enumeration token that can be used in a request to return the next batch of the results.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access pull_request_events outputs
pull_request_events_id = pull_request_events.id
pull_request_events_pull_request_events = pull_request_events.pull_request_events
pull_request_events_next_token = pull_request_events.next_token
```

---


### Approval_rule_template_description

ApprovalRuleTemplateDescription resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `approval_rule_template_name` | String | ✅ | <p>The name of the template for which you want to update the description.</p> |
| `approval_rule_template_description` | String | ✅ | <p>The updated description of the approval rule template.</p> |



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


### Comment_content

CommentContent resource

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


### Pull_request_approval_states

PullRequestApprovalStates resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `approvals` | Vec<String> | <p>Information about users who have approved the pull request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access pull_request_approval_states outputs
pull_request_approval_states_id = pull_request_approval_states.id
pull_request_approval_states_approvals = pull_request_approval_states.approvals
```

---


### Pull_request_approval_rule

PullRequestApprovalRule resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `approval_rule_content` | String | ✅ | <p>The content of the approval rule, including the number of approvals needed and the structure of an approval pool defined for approvals, if any. For more information
        about approval pools, see the CodeCommit User Guide.</p>
         <note>
            <p>When you create the content of the approval rule, you can specify approvers in an
                approval pool in one of two ways:</p>
            <ul>
               <li>
                  <p>
                     <b>CodeCommitApprovers</b>: This option only
                        requires an Amazon Web Services account and a resource. It can be used for both IAM users
                        and federated access users whose name matches the provided resource name.
                        This is a very powerful option that offers a great deal of flexibility. For
                        example, if you specify the Amazon Web Services account <i>123456789012</i>
                        and <i>Mary_Major</i>, all of the following would be counted
                        as approvals coming from that user:</p>
                  <ul>
                     <li>
                        <p>An IAM user in the account
                                    (arn:aws:iam::<i>123456789012</i>:user/<i>Mary_Major</i>)</p>
                     </li>
                     <li>
                        <p>A federated user identified in IAM as Mary_Major
                                    (arn:aws:sts::<i>123456789012</i>:federated-user/<i>Mary_Major</i>)</p>
                     </li>
                  </ul>
                  <p>This option does not recognize an active session of someone assuming the
                        role of CodeCommitReview with a role session name of
                            <i>Mary_Major</i>
                            (arn:aws:sts::<i>123456789012</i>:assumed-role/CodeCommitReview/<i>Mary_Major</i>)
                        unless you include a wildcard (*Mary_Major).</p>
               </li>
               <li>
                  <p>
                     <b>Fully qualified ARN</b>: This option allows
                        you to specify the fully qualified Amazon Resource Name (ARN) of the IAM
                        user or role. </p>
               </li>
            </ul>
            <p>For more information about IAM ARNs, wildcards, and formats, see
               <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_identifiers.html">IAM
                Identifiers</a> in the <i>IAM User Guide</i>.</p>
         </note> |
| `approval_rule_name` | String | ✅ | <p>The name for the approval rule.</p> |
| `pull_request_id` | String | ✅ | <p>The system-generated ID of the pull request for which you want to create the approval rule.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create pull_request_approval_rule
pull_request_approval_rule = provider.codecommit.Pull_request_approval_rule {
    approval_rule_content = "value"  # <p>The content of the approval rule, including the number of approvals needed and the structure of an approval pool defined for approvals, if any. For more information
        about approval pools, see the CodeCommit User Guide.</p>
         <note>
            <p>When you create the content of the approval rule, you can specify approvers in an
                approval pool in one of two ways:</p>
            <ul>
               <li>
                  <p>
                     <b>CodeCommitApprovers</b>: This option only
                        requires an Amazon Web Services account and a resource. It can be used for both IAM users
                        and federated access users whose name matches the provided resource name.
                        This is a very powerful option that offers a great deal of flexibility. For
                        example, if you specify the Amazon Web Services account <i>123456789012</i>
                        and <i>Mary_Major</i>, all of the following would be counted
                        as approvals coming from that user:</p>
                  <ul>
                     <li>
                        <p>An IAM user in the account
                                    (arn:aws:iam::<i>123456789012</i>:user/<i>Mary_Major</i>)</p>
                     </li>
                     <li>
                        <p>A federated user identified in IAM as Mary_Major
                                    (arn:aws:sts::<i>123456789012</i>:federated-user/<i>Mary_Major</i>)</p>
                     </li>
                  </ul>
                  <p>This option does not recognize an active session of someone assuming the
                        role of CodeCommitReview with a role session name of
                            <i>Mary_Major</i>
                            (arn:aws:sts::<i>123456789012</i>:assumed-role/CodeCommitReview/<i>Mary_Major</i>)
                        unless you include a wildcard (*Mary_Major).</p>
               </li>
               <li>
                  <p>
                     <b>Fully qualified ARN</b>: This option allows
                        you to specify the fully qualified Amazon Resource Name (ARN) of the IAM
                        user or role. </p>
               </li>
            </ul>
            <p>For more information about IAM ARNs, wildcards, and formats, see
               <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_identifiers.html">IAM
                Identifiers</a> in the <i>IAM User Guide</i>.</p>
         </note>
    approval_rule_name = "value"  # <p>The name for the approval rule.</p>
    pull_request_id = "value"  # <p>The system-generated ID of the pull request for which you want to create the approval rule.</p>
}

```

---


### Merge_conflicts

MergeConflicts resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `conflict_metadata_list` | Vec<String> | <p>A list of metadata for any conflicting files. If the specified merge strategy is
            FAST_FORWARD_MERGE, this list is always empty.</p> |
| `mergeable` | bool | <p>A Boolean value that indicates whether the code is mergeable by the specified merge option.</p> |
| `source_commit_id` | String | <p>The commit ID of the source commit specifier that was used in the merge evaluation.</p> |
| `destination_commit_id` | String | <p>The commit ID of the destination commit specifier that was used in the merge evaluation.</p> |
| `next_token` | String | <p>An enumeration token that can be used in a request to return the next batch of the results.</p> |
| `base_commit_id` | String | <p>The commit ID of the merge base.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access merge_conflicts outputs
merge_conflicts_id = merge_conflicts.id
merge_conflicts_conflict_metadata_list = merge_conflicts.conflict_metadata_list
merge_conflicts_mergeable = merge_conflicts.mergeable
merge_conflicts_source_commit_id = merge_conflicts.source_commit_id
merge_conflicts_destination_commit_id = merge_conflicts.destination_commit_id
merge_conflicts_next_token = merge_conflicts.next_token
merge_conflicts_base_commit_id = merge_conflicts.base_commit_id
```

---


### Comment_reactions

CommentReactions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `reactions_for_comment` | Vec<String> | <p>An array of reactions to the specified comment.</p> |
| `next_token` | String | <p>An enumeration token that can be used in a request to return the next batch of the results.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access comment_reactions outputs
comment_reactions_id = comment_reactions.id
comment_reactions_reactions_for_comment = comment_reactions.reactions_for_comment
comment_reactions_next_token = comment_reactions.next_token
```

---


### Default_branch

DefaultBranch resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `default_branch_name` | String | ✅ | <p>The name of the branch to set as the default branch.</p> |
| `repository_name` | String | ✅ | <p>The name of the repository for which you want to set or change the default branch.</p> |



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


### Pull_request_approval_state

PullRequestApprovalState resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `pull_request_id` | String | ✅ | <p>The system-generated ID of the pull request.</p> |
| `revision_id` | String | ✅ | <p>The system-generated ID of the revision.</p> |
| `approval_state` | String | ✅ | <p>The approval state to associate with the user on the pull request.</p> |



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


### Repository_description

RepositoryDescription resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `repository_name` | String | ✅ | <p>The name of the repository to set or change the comment or description for.</p> |
| `repository_description` | String |  | <p>The new comment or description for the specified repository. Repository descriptions are limited to 1,000 characters.</p> |



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


### Branch

Branch resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `repository_name` | String | ✅ | <p>The name of the repository in which you want to create the new branch.</p> |
| `commit_id` | String | ✅ | <p>The ID of the commit to point the new branch to.</p> |
| `branch_name` | String | ✅ | <p>The name of the new branch to create.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `branch` | String | <p>The name of the branch.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create branch
branch = provider.codecommit.Branch {
    repository_name = "value"  # <p>The name of the repository in which you want to create the new branch.</p>
    commit_id = "value"  # <p>The ID of the commit to point the new branch to.</p>
    branch_name = "value"  # <p>The name of the new branch to create.</p>
}

# Access branch outputs
branch_id = branch.id
branch_branch = branch.branch
```

---


### Repository_triggers

RepositoryTriggers resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `repository_name` | String | ✅ | <p>The name of the repository where you want to create or update the trigger.</p> |
| `triggers` | Vec<String> | ✅ | <p>The JSON block of configuration information for each trigger.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `triggers` | Vec<String> | <p>The JSON block of configuration information for each trigger.</p> |
| `configuration_id` | String | <p>The system-generated unique ID for the trigger.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create repository_triggers
repository_triggers = provider.codecommit.Repository_triggers {
    repository_name = "value"  # <p>The name of the repository where you want to create or update the trigger.</p>
    triggers = "value"  # <p>The JSON block of configuration information for each trigger.</p>
}

# Access repository_triggers outputs
repository_triggers_id = repository_triggers.id
repository_triggers_triggers = repository_triggers.triggers
repository_triggers_configuration_id = repository_triggers.configuration_id
```

---


### File

File resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `file_mode` | String |  | <p>The file mode permissions of the blob. Valid file mode permissions are listed
            here.</p> |
| `branch_name` | String | ✅ | <p>The name of the branch where you want to add or update the file. If this is an empty
            repository, this branch is created.</p> |
| `parent_commit_id` | String |  | <p>The full commit ID of the head commit in the branch where you want to add or update the file. If this is an empty repository, 
            no commit ID is required. If this is not an empty repository, a commit ID is required. </p>
         <p>The commit ID must match the ID of the head commit at the time of the operation.
            Otherwise, an error occurs, and the file is not added or updated.</p> |
| `file_content` | String | ✅ | <p>The content of the file, in binary object format. </p> |
| `file_path` | String | ✅ | <p>The name of the file you want to add or update, including the relative path to the file in the repository.</p>
         <note>
            <p>If the path does not currently exist in the repository, the path is created as part of adding
                the file.</p>
         </note> |
| `repository_name` | String | ✅ | <p>The name of the repository where you want to add or update the file.</p> |
| `email` | String |  | <p>An email address for the person adding or updating the file.</p> |
| `commit_message` | String |  | <p>A message about why this file was added or updated. Although it is optional, a message
            makes the commit history for your repository more useful.</p> |
| `name` | String |  | <p>The name of the person adding or updating the file. Although it is optional, a name
            makes the commit history for your repository more useful.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `blob_id` | String | <p>The blob ID of the object that represents the file content.</p> |
| `file_content` | String | <p>The base-64 encoded binary data object that represents the content of the file.</p> |
| `file_mode` | String | <p>The extrapolated file mode permissions of the blob. Valid values include strings such as EXECUTABLE and not numeric values.</p>
         <note>
            <p>The file mode permissions returned by this API are not the standard file mode
                permission values, such as 100644, but rather extrapolated values. See the supported
                return values.</p>
         </note> |
| `commit_id` | String | <p>The full commit ID of the commit that contains the content returned by GetFile.</p> |
| `file_path` | String | <p>The fully qualified path to the specified file. Returns the name and extension of the
            file.</p> |
| `file_size` | i64 | <p>The size of the contents of the file, in bytes.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create file
file = provider.codecommit.File {
    branch_name = "value"  # <p>The name of the branch where you want to add or update the file. If this is an empty
            repository, this branch is created.</p>
    file_content = "value"  # <p>The content of the file, in binary object format. </p>
    file_path = "value"  # <p>The name of the file you want to add or update, including the relative path to the file in the repository.</p>
         <note>
            <p>If the path does not currently exist in the repository, the path is created as part of adding
                the file.</p>
         </note>
    repository_name = "value"  # <p>The name of the repository where you want to add or update the file.</p>
}

# Access file outputs
file_id = file.id
file_blob_id = file.blob_id
file_file_content = file.file_content
file_file_mode = file.file_mode
file_commit_id = file.commit_id
file_file_path = file.file_path
file_file_size = file.file_size
```

---


### Pull_request_status

PullRequestStatus resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `pull_request_status` | String | ✅ | <p>The status of the pull request. The only valid operations are to update the status
            from <code>OPEN</code> to <code>OPEN</code>, <code>OPEN</code> to <code>CLOSED</code> or
            from <code>CLOSED</code> to <code>CLOSED</code>.</p> |
| `pull_request_id` | String | ✅ | <p>The system-generated ID of the pull request. To get this ID, use <a>ListPullRequests</a>.</p> |



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


### Blob

Blob resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `content` | String | <p>The content of the blob, usually a file.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access blob outputs
blob_id = blob.id
blob_content = blob.content
```

---


### Pull_request

PullRequest resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `title` | String | ✅ | <p>The title of the pull request. This title is used to identify the pull request to
            other users in the repository.</p> |
| `description` | String |  | <p>A description of the pull request.</p> |
| `targets` | Vec<String> | ✅ | <p>The targets for the pull request, including the source of the code to be reviewed (the
            source branch) and the destination where the creator of the pull request intends the
            code to be merged after the pull request is closed (the destination branch).</p> |
| `client_request_token` | String |  | <p>A unique, client-generated idempotency token that, when provided in a request, ensures
            the request cannot be repeated with a changed parameter. If a request is received with
            the same parameters and a token is included, the request returns information about the
            initial request that used that token.</p>
         <note>
            <p>The Amazon Web ServicesSDKs prepopulate client request tokens. If you are using an Amazon Web ServicesSDK, an
                idempotency token is created for you.</p>
         </note> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `pull_request` | String | <p>Information about the specified pull request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create pull_request
pull_request = provider.codecommit.Pull_request {
    title = "value"  # <p>The title of the pull request. This title is used to identify the pull request to
            other users in the repository.</p>
    targets = "value"  # <p>The targets for the pull request, including the source of the code to be reviewed (the
            source branch) and the destination where the creator of the pull request intends the
            code to be merged after the pull request is closed (the destination branch).</p>
}

# Access pull_request outputs
pull_request_id = pull_request.id
pull_request_pull_request = pull_request.pull_request
```

---


### Comment

Comment resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `comment_id` | String | ✅ | <p>The system-generated ID of the comment you want to update. To get this ID, use <a>GetCommentsForComparedCommit</a> 
            or <a>GetCommentsForPullRequest</a>.</p> |
| `content` | String | ✅ | <p>The updated content to replace the existing content of the comment.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `comment` | String | <p>The contents of the comment.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access comment outputs
comment_id = comment.id
comment_comment = comment.comment
```

---


### Comments_for_pull_request

CommentsForPullRequest resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>An enumeration token that can be used in a request to return the next batch of the results.</p> |
| `comments_for_pull_request_data` | Vec<String> | <p>An array of comment objects on the pull request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access comments_for_pull_request outputs
comments_for_pull_request_id = comments_for_pull_request.id
comments_for_pull_request_next_token = comments_for_pull_request.next_token
comments_for_pull_request_comments_for_pull_request_data = comments_for_pull_request.comments_for_pull_request_data
```

---


### Approval_rule_template_name

ApprovalRuleTemplateName resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `new_approval_rule_template_name` | String | ✅ | <p>The new name you want to apply to the approval rule template.</p> |
| `old_approval_rule_template_name` | String | ✅ | <p>The current name of the approval rule template.</p> |



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


### Pull_request_approval_rule_content

PullRequestApprovalRuleContent resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `existing_rule_content_sha256` | String |  | <p>The SHA-256 hash signature for the content of the approval rule. You can retrieve this
            information by using
            <a>GetPullRequest</a>.</p> |
| `pull_request_id` | String | ✅ | <p>The system-generated ID of the pull request.</p> |
| `new_rule_content` | String | ✅ | <p>The updated content for the approval rule.</p>
         <note>
            <p>When you update the content of the approval rule, you can specify approvers in an
                approval pool in one of two ways:</p>
            <ul>
               <li>
                  <p>
                     <b>CodeCommitApprovers</b>: This option only
                        requires an Amazon Web Services account and a resource. It can be used for both IAM users
                        and federated access users whose name matches the provided resource name.
                        This is a very powerful option that offers a great deal of flexibility. For
                        example, if you specify the Amazon Web Services account <i>123456789012</i>
                        and <i>Mary_Major</i>, all of the following are counted as
                        approvals coming from that user:</p>
                  <ul>
                     <li>
                        <p>An IAM user in the account
                                (arn:aws:iam::<i>123456789012</i>:user/<i>Mary_Major</i>)</p>
                     </li>
                     <li>
                        <p>A federated user identified in IAM as Mary_Major
                                (arn:aws:sts::<i>123456789012</i>:federated-user/<i>Mary_Major</i>)</p>
                     </li>
                  </ul>
                  <p>This option does not recognize an active session of someone assuming the
                        role of CodeCommitReview with a role session name of
                            <i>Mary_Major</i>
                            (arn:aws:sts::<i>123456789012</i>:assumed-role/CodeCommitReview/<i>Mary_Major</i>)
                        unless you include a wildcard (*Mary_Major).</p>
               </li>
               <li>
                  <p>
                     <b>Fully qualified ARN</b>: This option allows
                        you to specify the fully qualified Amazon Resource Name (ARN) of the IAM
                        user or role. </p>
               </li>
            </ul>
            <p>For more information about IAM ARNs, wildcards, and formats, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_identifiers.html">IAM
                    Identifiers</a> in the <i>IAM User Guide</i>.</p>
         </note> |
| `approval_rule_name` | String | ✅ | <p>The name of the approval rule you want to update.</p> |



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


### Repository_name

RepositoryName resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `old_name` | String | ✅ | <p>The current name of the repository.</p> |
| `new_name` | String | ✅ | <p>The new name for the repository.</p> |



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


### Approval_rule_template

ApprovalRuleTemplate resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `approval_rule_template_name` | String | ✅ | <p>The name of the approval rule template. Provide descriptive names, because this name
            is applied to the approval rules created automatically in associated
            repositories.</p> |
| `approval_rule_template_content` | String | ✅ | <p>The content of the approval rule that is created on pull requests in associated
            repositories. If you specify one or more destination references (branches), approval
            rules are created in an associated repository only if their destination references
            (branches) match those specified in the template.</p>
         <note>
            <p>When you create the content of the approval rule template, you can specify
                approvers in an approval pool in one of two ways:</p>
            <ul>
               <li>
                  <p>
                     <b>CodeCommitApprovers</b>: This option only
                        requires an Amazon Web Services account and a resource. It can be used for both IAM users
                        and federated access users whose name matches the provided resource name.
                        This is a very powerful option that offers a great deal of flexibility. For
                        example, if you specify the Amazon Web Services account <i>123456789012</i>
                        and <i>Mary_Major</i>, all of the following are counted as
                        approvals coming from that user:</p>
                  <ul>
                     <li>
                        <p>An IAM user in the account
                                (arn:aws:iam::<i>123456789012</i>:user/<i>Mary_Major</i>)</p>
                     </li>
                     <li>
                        <p>A federated user identified in IAM as Mary_Major
                                (arn:aws:sts::<i>123456789012</i>:federated-user/<i>Mary_Major</i>)</p>
                     </li>
                  </ul>
                  <p>This option does not recognize an active session of someone assuming the
                        role of CodeCommitReview with a role session name of
                            <i>Mary_Major</i>
                            (arn:aws:sts::<i>123456789012</i>:assumed-role/CodeCommitReview/<i>Mary_Major</i>)
                        unless you include a wildcard (*Mary_Major).</p>
               </li>
               <li>
                  <p>
                     <b>Fully qualified ARN</b>: This option allows
                        you to specify the fully qualified Amazon Resource Name (ARN) of the IAM
                        user or role. </p>
               </li>
            </ul>
            <p>For more information about IAM ARNs, wildcards, and formats, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_identifiers.html">IAM
                    Identifiers</a> in the <i>IAM User Guide</i>.</p>
         </note> |
| `approval_rule_template_description` | String |  | <p>The description of the approval rule template. Consider providing a description that
            explains what this template does and when it might be appropriate to associate it with
            repositories.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `approval_rule_template` | String | <p>The content and structure of the approval rule template.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create approval_rule_template
approval_rule_template = provider.codecommit.Approval_rule_template {
    approval_rule_template_name = "value"  # <p>The name of the approval rule template. Provide descriptive names, because this name
            is applied to the approval rules created automatically in associated
            repositories.</p>
    approval_rule_template_content = "value"  # <p>The content of the approval rule that is created on pull requests in associated
            repositories. If you specify one or more destination references (branches), approval
            rules are created in an associated repository only if their destination references
            (branches) match those specified in the template.</p>
         <note>
            <p>When you create the content of the approval rule template, you can specify
                approvers in an approval pool in one of two ways:</p>
            <ul>
               <li>
                  <p>
                     <b>CodeCommitApprovers</b>: This option only
                        requires an Amazon Web Services account and a resource. It can be used for both IAM users
                        and federated access users whose name matches the provided resource name.
                        This is a very powerful option that offers a great deal of flexibility. For
                        example, if you specify the Amazon Web Services account <i>123456789012</i>
                        and <i>Mary_Major</i>, all of the following are counted as
                        approvals coming from that user:</p>
                  <ul>
                     <li>
                        <p>An IAM user in the account
                                (arn:aws:iam::<i>123456789012</i>:user/<i>Mary_Major</i>)</p>
                     </li>
                     <li>
                        <p>A federated user identified in IAM as Mary_Major
                                (arn:aws:sts::<i>123456789012</i>:federated-user/<i>Mary_Major</i>)</p>
                     </li>
                  </ul>
                  <p>This option does not recognize an active session of someone assuming the
                        role of CodeCommitReview with a role session name of
                            <i>Mary_Major</i>
                            (arn:aws:sts::<i>123456789012</i>:assumed-role/CodeCommitReview/<i>Mary_Major</i>)
                        unless you include a wildcard (*Mary_Major).</p>
               </li>
               <li>
                  <p>
                     <b>Fully qualified ARN</b>: This option allows
                        you to specify the fully qualified Amazon Resource Name (ARN) of the IAM
                        user or role. </p>
               </li>
            </ul>
            <p>For more information about IAM ARNs, wildcards, and formats, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_identifiers.html">IAM
                    Identifiers</a> in the <i>IAM User Guide</i>.</p>
         </note>
}

# Access approval_rule_template outputs
approval_rule_template_id = approval_rule_template.id
approval_rule_template_approval_rule_template = approval_rule_template.approval_rule_template
```

---


### Merge_commit

MergeCommit resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `merged_commit_id` | String | <p>The commit ID for the merge commit created when the source branch was merged into the
            destination branch. If the fast-forward merge strategy was used, there is no merge
            commit.</p> |
| `source_commit_id` | String | <p>The commit ID of the source commit specifier that was used in the merge evaluation.</p> |
| `destination_commit_id` | String | <p>The commit ID of the destination commit specifier that was used in the merge evaluation.</p> |
| `base_commit_id` | String | <p>The commit ID of the merge base.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access merge_commit outputs
merge_commit_id = merge_commit.id
merge_commit_merged_commit_id = merge_commit.merged_commit_id
merge_commit_source_commit_id = merge_commit.source_commit_id
merge_commit_destination_commit_id = merge_commit.destination_commit_id
merge_commit_base_commit_id = merge_commit.base_commit_id
```

---


### Repository_encryption_key

RepositoryEncryptionKey resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `repository_name` | String | ✅ | <p>The name of the repository for which you want to update the KMS encryption key used to encrypt and decrypt the repository.</p> |
| `kms_key_id` | String | ✅ | <p>The ID of the encryption key. You can view the ID of an encryption key in the KMS console, or use the KMS APIs to
            programmatically retrieve a key ID. For more information about acceptable values for keyID, see 
            <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_Decrypt.html#KMS-Decrypt-request-KeyId">KeyId</a> in the Decrypt API description in 
        the <i>Key Management Service API Reference</i>.</p> |



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


### Approval_rule_template_content

ApprovalRuleTemplateContent resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `existing_rule_content_sha256` | String |  | <p>The SHA-256 hash signature for the content of the approval rule. You can retrieve this
            information by using
            <a>GetPullRequest</a>.</p> |
| `approval_rule_template_name` | String | ✅ | <p>The name of the approval rule template where you want to update the content of the rule. </p> |
| `new_rule_content` | String | ✅ | <p>The content that replaces the existing content of the rule. Content statements must be
            complete. You cannot provide only the changes.</p> |



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


### Pull_request_override_state

PullRequestOverrideState resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `overridden` | bool | <p>A Boolean value that indicates whether a pull request has had its rules set aside (TRUE) or whether all approval rules still apply (FALSE).</p> |
| `overrider` | String | <p>The Amazon Resource Name (ARN) of the user or identity that overrode the rules and their requirements for the pull request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access pull_request_override_state outputs
pull_request_override_state_id = pull_request_override_state.id
pull_request_override_state_overridden = pull_request_override_state.overridden
pull_request_override_state_overrider = pull_request_override_state.overrider
```

---


### Merge_options

MergeOptions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `base_commit_id` | String | <p>The commit ID of the merge base.</p> |
| `destination_commit_id` | String | <p>The commit ID of the destination commit specifier that was used in the merge evaluation.</p> |
| `source_commit_id` | String | <p>The commit ID of the source commit specifier that was used in the merge evaluation.</p> |
| `merge_options` | Vec<String> | <p>The merge option or strategy used to merge the code.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access merge_options outputs
merge_options_id = merge_options.id
merge_options_base_commit_id = merge_options.base_commit_id
merge_options_destination_commit_id = merge_options.destination_commit_id
merge_options_source_commit_id = merge_options.source_commit_id
merge_options_merge_options = merge_options.merge_options
```

---


### Pull_request_title

PullRequestTitle resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `title` | String | ✅ | <p>The updated title of the pull request. This replaces the existing title.</p> |
| `pull_request_id` | String | ✅ | <p>The system-generated ID of the pull request. To get this ID, use <a>ListPullRequests</a>.</p> |



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


### Differences

Differences resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `differences` | Vec<String> | <p>A data type object that contains information about the differences, including whether
            the difference is added, modified, or deleted (A, D, M).</p> |
| `next_token` | String | <p>An enumeration token that can be used in a request to return the next batch of the results.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access differences outputs
differences_id = differences.id
differences_differences = differences.differences
differences_next_token = differences.next_token
```

---


### Unreferenced_merge_commit

UnreferencedMergeCommit resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `repository_name` | String | ✅ | <p>The name of the repository where you want to create the unreferenced merge commit.</p> |
| `email` | String |  | <p>The email address for the person who created the unreferenced commit.</p> |
| `commit_message` | String |  | <p>The commit message for the unreferenced commit.</p> |
| `source_commit_specifier` | String | ✅ | <p>The branch, tag, HEAD, or other fully qualified reference used to identify a commit
            (for example, a branch name or a full commit ID).</p> |
| `destination_commit_specifier` | String | ✅ | <p>The branch, tag, HEAD, or other fully qualified reference used to identify a commit
            (for example, a branch name or a full commit ID).</p> |
| `conflict_detail_level` | String |  | <p>The level of conflict detail to use. If unspecified, the default FILE_LEVEL is used,
            which returns a not-mergeable result if the same file has differences in both branches.
            If LINE_LEVEL is specified, a conflict is considered not mergeable if the same file in
            both branches has differences on the same line.</p> |
| `keep_empty_folders` | bool |  | <p>If the commit contains deletions, whether to keep a folder or folder structure if the
            changes leave the folders empty. If this is specified as true, a .gitkeep file is
            created for empty folders. The default is false.</p> |
| `conflict_resolution` | String |  | <p>If AUTOMERGE is the conflict resolution strategy, a list of inputs to use when
            resolving conflicts during a merge.</p> |
| `conflict_resolution_strategy` | String |  | <p>Specifies which branch to use when resolving conflicts, or whether to attempt
            automatically merging two versions of a file. The default is NONE, which requires any
            conflicts to be resolved manually before the merge operation is successful.</p> |
| `merge_option` | String | ✅ | <p>The merge option or strategy you want to use to merge the code.</p> |
| `author_name` | String |  | <p>The name of the author who created the unreferenced commit. This information is used
            as both the author and committer for the commit.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create unreferenced_merge_commit
unreferenced_merge_commit = provider.codecommit.Unreferenced_merge_commit {
    repository_name = "value"  # <p>The name of the repository where you want to create the unreferenced merge commit.</p>
    source_commit_specifier = "value"  # <p>The branch, tag, HEAD, or other fully qualified reference used to identify a commit
            (for example, a branch name or a full commit ID).</p>
    destination_commit_specifier = "value"  # <p>The branch, tag, HEAD, or other fully qualified reference used to identify a commit
            (for example, a branch name or a full commit ID).</p>
    merge_option = "value"  # <p>The merge option or strategy you want to use to merge the code.</p>
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

# Create multiple comments_for_compared_commit resources
comments_for_compared_commit_0 = provider.codecommit.Comments_for_compared_commit {
}
comments_for_compared_commit_1 = provider.codecommit.Comments_for_compared_commit {
}
comments_for_compared_commit_2 = provider.codecommit.Comments_for_compared_commit {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    comments_for_compared_commit = provider.codecommit.Comments_for_compared_commit {
    }
```

---

## Related Documentation

- [AWS Codecommit Documentation](https://docs.aws.amazon.com/codecommit/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
