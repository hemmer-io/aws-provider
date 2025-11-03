# Amplify Service



**Resources**: 8

---

## Overview

The amplify service provides access to 8 resource types:

- [Deployment](#deployment) [C]
- [App](#app) [CRUD]
- [Job](#job) [RD]
- [Backend_environment](#backend_environment) [CRD]
- [Domain_association](#domain_association) [CRUD]
- [Webhook](#webhook) [CRUD]
- [Branch](#branch) [CRUD]
- [Artifact_url](#artifact_url) [R]

---

## Resources


### Deployment

Deployment resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `file_map` | HashMap<String, String> |  | <p> An optional file map that contains the file name as the key and the file content md5
            hash as the value. If this argument is provided, the service will generate a unique
            upload URL per file. Otherwise, the service will only generate a single upload URL for
            the zipped files. </p> |
| `app_id` | String | ✅ | <p> The unique ID for an Amplify app. </p> |
| `branch_name` | String | ✅ | <p> The name of the branch to use for the job. </p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create deployment
deployment = provider.amplify.Deployment {
    app_id = "value"  # <p> The unique ID for an Amplify app. </p>
    branch_name = "value"  # <p> The name of the branch to use for the job. </p>
}

```

---


### App

App resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `custom_rules` | Vec<String> |  | <p>The custom rewrite and redirect rules for an Amplify app. </p> |
| `build_spec` | String |  | <p>The build specification (build spec) for an Amplify app. </p> |
| `enable_auto_branch_creation` | bool |  | <p>Enables automated branch creation for an Amplify app. </p> |
| `access_token` | String |  | <p>The personal access token for a GitHub repository for an Amplify app. The personal
            access token is used to authorize access to a GitHub repository using the Amplify GitHub
            App. The token is not stored.</p>
         <p>Use <code>accessToken</code> for GitHub repositories only. To authorize access to a
            repository provider such as Bitbucket or CodeCommit, use <code>oauthToken</code>.</p>
         <p>You must specify either <code>accessToken</code> or <code>oauthToken</code> when you
            create a new app.</p>
         <p>Existing Amplify apps deployed from a GitHub repository using OAuth continue to work
            with CI/CD. However, we strongly recommend that you migrate these apps to use the GitHub
            App. For more information, see <a href="https://docs.aws.amazon.com/amplify/latest/userguide/setting-up-GitHub-access.html#migrating-to-github-app-auth">Migrating an existing OAuth app to the Amplify GitHub App</a> in the
                <i>Amplify User Guide</i> .</p> |
| `repository` | String |  | <p>The Git repository for the Amplify app. </p> |
| `iam_service_role_arn` | String |  | <p>The Amazon Resource Name (ARN) of the IAM service role for the Amplify app.</p> |
| `environment_variables` | HashMap<String, String> |  | <p>The environment variables map for an Amplify app. </p>
         <p>For a list of the environment variables that are accessible to Amplify by default, see
                <a href="https://docs.aws.amazon.com/amplify/latest/userguide/amplify-console-environment-variables.html">Amplify
                Environment variables</a> in the <i>Amplify Hosting User
            Guide</i>.</p> |
| `enable_branch_auto_build` | bool |  | <p>Enables the auto building of branches for an Amplify app. </p> |
| `cache_config` | String |  | <p>The cache configuration for the Amplify app.</p> |
| `oauth_token` | String |  | <p>The OAuth token for a third-party source control system for an Amplify app. The OAuth
            token is used to create a webhook and a read-only deploy key using SSH cloning. The
            OAuth token is not stored.</p>
         <p>Use <code>oauthToken</code> for repository providers other than GitHub, such as
            Bitbucket or CodeCommit. To authorize access to GitHub as your repository provider, use
                <code>accessToken</code>.</p>
         <p>You must specify either <code>oauthToken</code> or <code>accessToken</code> when you
            create a new app.</p>
         <p>Existing Amplify apps deployed from a GitHub repository using OAuth continue to work
            with CI/CD. However, we strongly recommend that you migrate these apps to use the GitHub
            App. For more information, see <a href="https://docs.aws.amazon.com/amplify/latest/userguide/setting-up-GitHub-access.html#migrating-to-github-app-auth">Migrating an existing OAuth app to the Amplify GitHub App</a> in the
                <i>Amplify User Guide</i> .</p> |
| `description` | String |  | <p>The description of the Amplify app. </p> |
| `auto_branch_creation_patterns` | Vec<String> |  | <p>The automated branch creation glob patterns for an Amplify app. </p> |
| `basic_auth_credentials` | String |  | <p>The credentials for basic authorization for an Amplify app. You must base64-encode the
            authorization credentials and provide them in the format
            <code>user:password</code>.</p> |
| `compute_role_arn` | String |  | <p>The Amazon Resource Name (ARN) of the IAM role to assign to an SSR app.
            The SSR Compute role allows the Amplify Hosting compute service to
            securely access specific Amazon Web Services resources based on the role's permissions.
            For more information about the SSR Compute role, see <a href="https://docs.aws.amazon.com/amplify/latest/userguide/amplify-SSR-compute-role.html">Adding an SSR Compute
                role</a> in the <i>Amplify User Guide</i>.</p> |
| `name` | String | ✅ | <p>The name of the Amplify app. </p> |
| `enable_basic_auth` | bool |  | <p>Enables basic authorization for an Amplify app. This will apply to all branches that
            are part of this app. </p> |
| `auto_branch_creation_config` | String |  | <p>The automated branch creation configuration for an Amplify app. </p> |
| `job_config` | String |  | <p>Describes the configuration details that apply to the jobs for an Amplify app.</p> |
| `tags` | HashMap<String, String> |  | <p>The tag for an Amplify app. </p> |
| `custom_headers` | String |  | <p>The custom HTTP headers for an Amplify app.</p> |
| `enable_branch_auto_deletion` | bool |  | <p>Automatically disconnects a branch in the Amplify console when you delete a branch
            from your Git repository. </p> |
| `platform` | String |  | <p>The platform for the Amplify app. For a static app, set the platform type to
                <code>WEB</code>. For a dynamic server-side rendered (SSR) app, set the platform
            type to <code>WEB_COMPUTE</code>. For an app requiring Amplify Hosting's original SSR
            support only, set the platform type to <code>WEB_DYNAMIC</code>.</p>
         <p>If you are deploying an SSG only app with Next.js version 14 or later, you must set
            the platform type to <code>WEB_COMPUTE</code> and set the artifacts
                <code>baseDirectory</code> to <code>.next</code> in the application's build
            settings. For an example of the build specification settings, see <a href="https://docs.aws.amazon.com/amplify/latest/userguide/deploy-nextjs-app.html#build-setting-detection-ssg-14">Amplify build settings for a Next.js 14 SSG application</a> in the
                <i>Amplify Hosting User Guide</i>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `app` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create app
app = provider.amplify.App {
    name = "value"  # <p>The name of the Amplify app. </p>
}

# Access app outputs
app_id = app.id
app_app = app.app
```

---


### Job

Job resource

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `job` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access job outputs
job_id = job.id
job_job = job.job
```

---


### Backend_environment

BackendEnvironment resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `environment_name` | String | ✅ | <p>The name for the backend environment. </p> |
| `app_id` | String | ✅ | <p>The unique ID for an Amplify app. </p> |
| `stack_name` | String |  | <p>The AWS CloudFormation stack name of a backend environment. </p> |
| `deployment_artifacts` | String |  | <p>The name of deployment artifacts. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `backend_environment` | String | <p>Describes the backend environment for an Amplify app. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create backend_environment
backend_environment = provider.amplify.Backend_environment {
    environment_name = "value"  # <p>The name for the backend environment. </p>
    app_id = "value"  # <p>The unique ID for an Amplify app. </p>
}

# Access backend_environment outputs
backend_environment_id = backend_environment.id
backend_environment_backend_environment = backend_environment.backend_environment
```

---


### Domain_association

DomainAssociation resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `app_id` | String | ✅ | <p> The unique ID for an Amplify app. </p> |
| `auto_sub_domain_iam_role` | String |  | <p> The required AWS Identity and Access Management (IAM) service role for the Amazon
            Resource Name (ARN) for automatically creating subdomains. </p> |
| `certificate_settings` | String |  | <p>The type of SSL/TLS certificate to use for your custom domain. If you don't specify a
            certificate type, Amplify uses the default certificate that it provisions and manages
            for you.</p> |
| `domain_name` | String | ✅ | <p> The domain name for the domain association. </p> |
| `enable_auto_sub_domain` | bool |  | <p> Enables the automated creation of subdomains for branches. </p> |
| `sub_domain_settings` | Vec<String> | ✅ | <p> The setting for the subdomain. </p> |
| `auto_sub_domain_creation_patterns` | Vec<String> |  | <p> Sets the branch patterns for automatic subdomain creation. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `domain_association` | String | <p> Describes the structure of a domain association, which associates a custom domain
            with an Amplify app. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create domain_association
domain_association = provider.amplify.Domain_association {
    app_id = "value"  # <p> The unique ID for an Amplify app. </p>
    domain_name = "value"  # <p> The domain name for the domain association. </p>
    sub_domain_settings = "value"  # <p> The setting for the subdomain. </p>
}

# Access domain_association outputs
domain_association_id = domain_association.id
domain_association_domain_association = domain_association.domain_association
```

---


### Webhook

Webhook resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `app_id` | String | ✅ | <p>The unique ID for an Amplify app. </p> |
| `description` | String |  | <p>The description for a webhook. </p> |
| `branch_name` | String | ✅ | <p>The name for a branch that is part of an Amplify app. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `webhook` | String | <p>Describes the structure of a webhook. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create webhook
webhook = provider.amplify.Webhook {
    app_id = "value"  # <p>The unique ID for an Amplify app. </p>
    branch_name = "value"  # <p>The name for a branch that is part of an Amplify app. </p>
}

# Access webhook outputs
webhook_id = webhook.id
webhook_webhook = webhook.webhook
```

---


### Branch

Branch resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | <p> The display name for a branch. This is used as the default domain prefix. </p> |
| `compute_role_arn` | String |  | <p>The Amazon Resource Name (ARN) of the IAM role to assign to a branch of
            an SSR app. The SSR Compute role allows the Amplify Hosting compute
            service to securely access specific Amazon Web Services resources based on the role's
            permissions. For more information about the SSR Compute role, see <a href="https://docs.aws.amazon.com/amplify/latest/userguide/amplify-SSR-compute-role.html">Adding an SSR Compute role</a> in the <i>Amplify User
                Guide</i>.</p> |
| `enable_pull_request_preview` | bool |  | <p> Enables pull request previews for this branch. </p> |
| `tags` | HashMap<String, String> |  | <p> The tag for the branch. </p> |
| `framework` | String |  | <p> The framework for the branch. </p> |
| `enable_performance_mode` | bool |  | <p>Enables performance mode for the branch.</p>
         <p>Performance mode optimizes for faster hosting performance by keeping content cached at
            the edge for a longer interval. When performance mode is enabled, hosting configuration
            or code changes can take up to 10 minutes to roll out. </p> |
| `build_spec` | String |  | <p> The build specification (build spec) for the branch. </p> |
| `ttl` | String |  | <p> The content Time To Live (TTL) for the website in seconds. </p> |
| `stage` | String |  | <p>Describes the current stage for the branch. </p> |
| `backend_environment_arn` | String |  | <p>The Amazon Resource Name (ARN) for a backend environment that is part of a Gen 1
            Amplify app. </p>
         <p>This field is available to Amplify Gen 1 apps only where the backend is
            created using Amplify Studio or the Amplify command line
            interface (CLI).</p> |
| `enable_skew_protection` | bool |  | <p>Specifies whether the skew protection feature is enabled for the branch.</p>
         <p>Deployment skew protection is available to Amplify applications to
            eliminate version skew issues between client and servers in web applications. When you
            apply skew protection to a branch, you can ensure that your clients always interact with
            the correct version of server-side assets, regardless of when a deployment occurs. For
            more information about skew protection, see <a href="https://docs.aws.amazon.com/amplify/latest/userguide/skew-protection.html">Skew protection for Amplify deployments</a> in the <i>Amplify User
                Guide</i>.</p> |
| `app_id` | String | ✅ | <p> The unique ID for an Amplify app. </p> |
| `branch_name` | String | ✅ | <p>The name for the branch. </p> |
| `enable_notification` | bool |  | <p> Enables notifications for the branch. </p> |
| `enable_auto_build` | bool |  | <p> Enables auto building for the branch. </p> |
| `pull_request_environment_name` | String |  | <p> The Amplify environment name for the pull request. </p> |
| `basic_auth_credentials` | String |  | <p> The basic authorization credentials for the branch. You must base64-encode the
            authorization credentials and provide them in the format
            <code>user:password</code>.</p> |
| `backend` | String |  | <p>The backend for a <code>Branch</code> of an Amplify app. Use for a
            backend created from an CloudFormation stack.</p>
         <p>This field is available to Amplify Gen 2 apps only. When you deploy an
            application with Amplify Gen 2, you provision the app's backend infrastructure using
            Typescript code.</p> |
| `description` | String |  | <p>The description for the branch. </p> |
| `environment_variables` | HashMap<String, String> |  | <p> The environment variables for the branch. </p> |
| `enable_basic_auth` | bool |  | <p> Enables basic authorization for the branch. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `branch` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create branch
branch = provider.amplify.Branch {
    app_id = "value"  # <p> The unique ID for an Amplify app. </p>
    branch_name = "value"  # <p>The name for the branch. </p>
}

# Access branch outputs
branch_id = branch.id
branch_branch = branch.branch
```

---


### Artifact_url

ArtifactUrl resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `artifact_id` | String | <p>The unique ID for an artifact. </p> |
| `artifact_url` | String | <p>The presigned URL for the artifact. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access artifact_url outputs
artifact_url_id = artifact_url.id
artifact_url_artifact_id = artifact_url.artifact_id
artifact_url_artifact_url = artifact_url.artifact_url
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple deployment resources
deployment_0 = provider.amplify.Deployment {
    app_id = "value-0"
    branch_name = "value-0"
}
deployment_1 = provider.amplify.Deployment {
    app_id = "value-1"
    branch_name = "value-1"
}
deployment_2 = provider.amplify.Deployment {
    app_id = "value-2"
    branch_name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    deployment = provider.amplify.Deployment {
        app_id = "production-value"
        branch_name = "production-value"
    }
```

---

## Related Documentation

- [AWS Amplify Documentation](https://docs.aws.amazon.com/amplify/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
