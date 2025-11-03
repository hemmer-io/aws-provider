# Codebuild Service



**Resources**: 12

---

## Overview

The codebuild service provides access to 12 resource types:

- [Webhook](#webhook) [CUD]
- [Build_batch](#build_batch) [D]
- [Resource_policy](#resource_policy) [CRD]
- [Test_cases](#test_cases) [R]
- [Fleet](#fleet) [CUD]
- [Report](#report) [D]
- [Report_group](#report_group) [CUD]
- [Source_credentials](#source_credentials) [D]
- [Report_group_trend](#report_group_trend) [R]
- [Project](#project) [CUD]
- [Project_visibility](#project_visibility) [U]
- [Code_coverages](#code_coverages) [R]

---

## Resources


### Webhook

Webhook resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `branch_filter` | String |  | <p>A regular expression used to determine which repository branches are built when a
      webhook is triggered. If the name of a branch matches the regular expression, then it is
      built. If <code>branchFilter</code> is empty, then all branches are built.</p>
         <note>
            <p>It is recommended that you use <code>filterGroups</code> instead of
        <code>branchFilter</code>. </p>
         </note> |
| `filter_groups` | Vec<Vec<String>> |  | <p>An array of arrays of <code>WebhookFilter</code> objects used to determine which
      webhooks are triggered. At least one <code>WebhookFilter</code> in the array must
      specify <code>EVENT</code> as its <code>type</code>. </p>
         <p>For a build to be triggered, at least one filter group in the
      <code>filterGroups</code> array must pass. For a filter group to pass, each of its
      filters must pass. </p> |
| `build_type` | String |  | <p>Specifies the type of build this webhook will trigger.</p>
         <note>
            <p>
               <code>RUNNER_BUILDKITE_BUILD</code> is only available for <code>NO_SOURCE</code> source type projects 
        configured for Buildkite runner builds. For more information about CodeBuild-hosted Buildkite runner builds, see <a href="https://docs.aws.amazon.com/codebuild/latest/userguide/sample-runner-buildkite.html">Tutorial: Configure a CodeBuild-hosted Buildkite runner</a> in the <i>CodeBuild
        user guide</i>.</p>
         </note> |
| `pull_request_build_policy` | String |  | <p>A PullRequestBuildPolicy object that defines comment-based approval requirements for triggering builds on pull requests. This policy helps control when automated builds are executed based on contributor permissions and approval workflows.</p> |
| `project_name` | String | ✅ | <p>The name of the CodeBuild project.</p> |
| `manual_creation` | bool |  | <p>If manualCreation is true, CodeBuild doesn't create a webhook in GitHub and instead returns <code>payloadUrl</code> and 
      <code>secret</code> values for the webhook. The <code>payloadUrl</code> and <code>secret</code> values in the output can be 
      used to manually create a webhook within GitHub.</p>
         <note>
            <p>
               <code>manualCreation</code> is only available for GitHub webhooks.</p>
         </note> |
| `scope_configuration` | String |  | <p>The scope configuration for global or organization webhooks.</p>
         <note>
            <p>Global or organization webhooks are only available for GitHub and Github Enterprise webhooks.</p>
         </note> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create webhook
webhook = provider.codebuild.Webhook {
    project_name = "value"  # <p>The name of the CodeBuild project.</p>
}

```

---


### Build_batch

BuildBatch resource

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


### Resource_policy

ResourcePolicy resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resource_arn` | String | ✅ | <p> The ARN of the <code>Project</code> or <code>ReportGroup</code> resource you want to
            associate with a resource policy. </p> |
| `policy` | String | ✅ | <p> A JSON-formatted resource policy. For more information, see <a href="https://docs.aws.amazon.com/codebuild/latest/userguide/project-sharing.html#project-sharing-share">Sharing
                a Project</a> and <a href="https://docs.aws.amazon.com/codebuild/latest/userguide/report-groups-sharing.html#report-groups-sharing-share">Sharing a Report Group</a> in the <i>CodeBuild User Guide</i>.
        </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `policy` | String | <p> The resource policy for the resource identified by the input ARN parameter. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create resource_policy
resource_policy = provider.codebuild.Resource_policy {
    resource_arn = "value"  # <p> The ARN of the <code>Project</code> or <code>ReportGroup</code> resource you want to
            associate with a resource policy. </p>
    policy = "value"  # <p> A JSON-formatted resource policy. For more information, see <a href="https://docs.aws.amazon.com/codebuild/latest/userguide/project-sharing.html#project-sharing-share">Sharing
                a Project</a> and <a href="https://docs.aws.amazon.com/codebuild/latest/userguide/report-groups-sharing.html#report-groups-sharing-share">Sharing a Report Group</a> in the <i>CodeBuild User Guide</i>.
        </p>
}

# Access resource_policy outputs
resource_policy_id = resource_policy.id
resource_policy_policy = resource_policy.policy
```

---


### Test_cases

TestCases resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `test_cases` | Vec<String> | <p>
      The returned list of test cases.
    </p> |
| `next_token` | String | <p>
      During a previous call, the maximum number of items that can be returned is the value specified in
      <code>maxResults</code>. If there more items in the list, then a unique string called a <i>nextToken</i>
      is returned. To get the next batch of items in the list, call this operation again, adding the next token
      to the call. To get all of the items in the list, keep calling this operation with each
      subsequent next token that is returned, until no more next tokens are returned.
    </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access test_cases outputs
test_cases_id = test_cases.id
test_cases_test_cases = test_cases.test_cases
test_cases_next_token = test_cases.next_token
```

---


### Fleet

Fleet resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>A list of tag key and value pairs associated with this compute fleet.</p>
         <p>These tags are available for use by Amazon Web Services services that support CodeBuild build project
      tags.</p> |
| `name` | String | ✅ | <p>The name of the compute fleet.</p> |
| `compute_type` | String | ✅ | <p>Information about the compute resources the compute fleet uses. Available values
            include:</p>
         <ul>
            <li>
               <p>
                  <code>ATTRIBUTE_BASED_COMPUTE</code>: Specify the amount of vCPUs, memory, disk space, and the type of machine.</p>
               <note>
                  <p> If you use <code>ATTRIBUTE_BASED_COMPUTE</code>, you must define your attributes by using <code>computeConfiguration</code>. CodeBuild 
                        will select the cheapest instance that satisfies your specified attributes. For more information, see <a href="https://docs.aws.amazon.com/codebuild/latest/userguide/build-env-ref-compute-types.html#environment-reserved-capacity.types">Reserved capacity environment 
                        types</a> in the <i>CodeBuild User Guide</i>.</p>
               </note>
            </li>
            <li>
               <p>
                  <code>CUSTOM_INSTANCE_TYPE</code>: Specify the instance type for your compute fleet. For a list of supported instance types, see <a href="https://docs.aws.amazon.com/codebuild/latest/userguide/build-env-ref-compute-types.html#environment-reserved-capacity.instance-types">Supported instance families
                        </a> in the <i>CodeBuild User Guide</i>.</p>
            </li>
            <li>
               <p>
                  <code>BUILD_GENERAL1_SMALL</code>: Use up to 4 GiB memory and 2 vCPUs for
                    builds.</p>
            </li>
            <li>
               <p>
                  <code>BUILD_GENERAL1_MEDIUM</code>: Use up to 8 GiB memory and 4 vCPUs for
                    builds.</p>
            </li>
            <li>
               <p>
                  <code>BUILD_GENERAL1_LARGE</code>: Use up to 16 GiB memory and 8 vCPUs for
                    builds, depending on your environment type.</p>
            </li>
            <li>
               <p>
                  <code>BUILD_GENERAL1_XLARGE</code>: Use up to 72 GiB memory and 36 vCPUs for
                    builds, depending on your environment type.</p>
            </li>
            <li>
               <p>
                  <code>BUILD_GENERAL1_2XLARGE</code>: Use up to 144 GiB memory, 72 vCPUs, and
                    824 GB of SSD storage for builds. This compute type supports Docker images up to
                    100 GB uncompressed.</p>
            </li>
            <li>
               <p>
                  <code>BUILD_LAMBDA_1GB</code>: Use up to 1 GiB memory for
                    builds. Only available for environment type <code>LINUX_LAMBDA_CONTAINER</code> and <code>ARM_LAMBDA_CONTAINER</code>.</p>
            </li>
            <li>
               <p>
                  <code>BUILD_LAMBDA_2GB</code>: Use up to 2 GiB memory for
                    builds. Only available for environment type <code>LINUX_LAMBDA_CONTAINER</code> and <code>ARM_LAMBDA_CONTAINER</code>.</p>
            </li>
            <li>
               <p>
                  <code>BUILD_LAMBDA_4GB</code>: Use up to 4 GiB memory for
                    builds. Only available for environment type <code>LINUX_LAMBDA_CONTAINER</code> and <code>ARM_LAMBDA_CONTAINER</code>.</p>
            </li>
            <li>
               <p>
                  <code>BUILD_LAMBDA_8GB</code>: Use up to 8 GiB memory for
                    builds. Only available for environment type <code>LINUX_LAMBDA_CONTAINER</code> and <code>ARM_LAMBDA_CONTAINER</code>.</p>
            </li>
            <li>
               <p>
                  <code>BUILD_LAMBDA_10GB</code>: Use up to 10 GiB memory for
                    builds. Only available for environment type <code>LINUX_LAMBDA_CONTAINER</code> and <code>ARM_LAMBDA_CONTAINER</code>.</p>
            </li>
         </ul>
         <p> If you use <code>BUILD_GENERAL1_SMALL</code>: </p>
         <ul>
            <li>
               <p> For environment type <code>LINUX_CONTAINER</code>, you can use up to 4 GiB
                    memory and 2 vCPUs for builds. </p>
            </li>
            <li>
               <p> For environment type <code>LINUX_GPU_CONTAINER</code>, you can use up to 16
                    GiB memory, 4 vCPUs, and 1 NVIDIA A10G Tensor Core GPU for builds.</p>
            </li>
            <li>
               <p> For environment type <code>ARM_CONTAINER</code>, you can use up to 4 GiB
                    memory and 2 vCPUs on ARM-based processors for builds.</p>
            </li>
         </ul>
         <p> If you use <code>BUILD_GENERAL1_LARGE</code>: </p>
         <ul>
            <li>
               <p> For environment type <code>LINUX_CONTAINER</code>, you can use up to 16 GiB
                    memory and 8 vCPUs for builds. </p>
            </li>
            <li>
               <p> For environment type <code>LINUX_GPU_CONTAINER</code>, you can use up to 255
                    GiB memory, 32 vCPUs, and 4 NVIDIA Tesla V100 GPUs for builds.</p>
            </li>
            <li>
               <p> For environment type <code>ARM_CONTAINER</code>, you can use up to 16 GiB
                    memory and 8 vCPUs on ARM-based processors for builds.</p>
            </li>
         </ul>
         <p>For more information, see <a href="https://docs.aws.amazon.com/codebuild/latest/userguide/build-env-ref-compute-types.html#environment.types">On-demand environment types</a> 
                in the <i>CodeBuild User Guide.</i>
         </p> |
| `image_id` | String |  | <p>The Amazon Machine Image (AMI) of the compute fleet.</p> |
| `fleet_service_role` | String |  | <p>The service role associated with the compute fleet. For more information, see <a href="https://docs.aws.amazon.com/codebuild/latest/userguide/auth-and-access-control-iam-identity-based-access-control.html#customer-managed-policies-example-permission-policy-fleet-service-role.html">
            Allow a user to add a permission policy for a fleet service role</a> in the <i>CodeBuild User Guide</i>.</p> |
| `scaling_configuration` | String |  | <p>The scaling configuration of the compute fleet.</p> |
| `proxy_configuration` | String |  | <p>The proxy configuration of the compute fleet.</p> |
| `vpc_config` | String |  |  |
| `base_capacity` | i64 | ✅ | <p>The initial number of machines allocated to the ﬂeet, which deﬁnes the number of builds that can run in parallel.</p> |
| `overflow_behavior` | String |  | <p>The compute fleet overflow behavior.</p>
         <ul>
            <li>
               <p>For overflow behavior <code>QUEUE</code>, your overflow builds need to wait on 
                    the existing fleet instance to become available.</p>
            </li>
            <li>
               <p>For overflow behavior <code>ON_DEMAND</code>, your overflow builds run on CodeBuild on-demand.</p>
               <note>
                  <p>If you choose to set your overflow behavior to on-demand while creating a VPC-connected 
                    fleet, make sure that you add the required VPC permissions to your project service role. For more 
                    information, see <a href="https://docs.aws.amazon.com/codebuild/latest/userguide/auth-and-access-control-iam-identity-based-access-control.html#customer-managed-policies-example-create-vpc-network-interface">Example 
                    policy statement to allow CodeBuild access to Amazon Web Services services required to create a VPC network interface</a>.</p>
               </note>
            </li>
         </ul> |
| `compute_configuration` | String |  | <p>The compute configuration of the compute fleet. This is only required if <code>computeType</code> is set to <code>ATTRIBUTE_BASED_COMPUTE</code> or <code>CUSTOM_INSTANCE_TYPE</code>.</p> |
| `environment_type` | String | ✅ | <p>The environment type of the compute fleet.</p>
         <ul>
            <li>
               <p>The environment type <code>ARM_CONTAINER</code> is available only in regions
                    US East (N. Virginia), US East (Ohio), US West (Oregon), EU (Ireland),
                    Asia Pacific (Mumbai), Asia Pacific (Tokyo), Asia Pacific (Singapore), Asia Pacific (Sydney), 
                    EU (Frankfurt), and South America (São Paulo).</p>
            </li>
            <li>
               <p>The environment type <code>ARM_EC2</code> is available only in regions
                    US East (N. Virginia), US East (Ohio), US West (Oregon), EU (Ireland), 
                    EU (Frankfurt), Asia Pacific (Tokyo),
                    Asia Pacific (Singapore), Asia Pacific (Sydney), South America (São Paulo), and
                    Asia Pacific (Mumbai).</p>
            </li>
            <li>
               <p>The environment type <code>LINUX_CONTAINER</code> is available only in regions
                    US East (N. Virginia), US East (Ohio), US West (Oregon), EU (Ireland), 
                    EU (Frankfurt), Asia Pacific (Tokyo),
                    Asia Pacific (Singapore), Asia Pacific (Sydney), South America (São Paulo), and
                    Asia Pacific (Mumbai).</p>
            </li>
            <li>
               <p>The environment type <code>LINUX_EC2</code> is available only in regions
                    US East (N. Virginia), US East (Ohio), US West (Oregon), EU (Ireland), 
                    EU (Frankfurt), Asia Pacific (Tokyo),
                    Asia Pacific (Singapore), Asia Pacific (Sydney), South America (São Paulo), and
                    Asia Pacific (Mumbai).</p>
            </li>
            <li>
               <p>The environment type <code>LINUX_GPU_CONTAINER</code> is available only in
                    regions US East (N. Virginia), US East (Ohio), US West (Oregon), EU (Ireland), 
                    EU (Frankfurt), Asia Pacific (Tokyo), and Asia Pacific (Sydney).</p>
            </li>
            <li>
               <p>The environment type <code>MAC_ARM</code> is available for Medium fleets only in
                    regions US East (N. Virginia), US East (Ohio), US West (Oregon), Asia Pacific (Sydney), 
                    and EU (Frankfurt)</p>
            </li>
            <li>
               <p>The environment type <code>MAC_ARM</code> is available for Large fleets only in
                    regions US East (N. Virginia), US East (Ohio), US West (Oregon), and Asia Pacific (Sydney).</p>
            </li>
            <li>
               <p>The environment type <code>WINDOWS_EC2</code> is available only in regions
                    US East (N. Virginia), US East (Ohio), US West (Oregon), EU (Ireland), 
                    EU (Frankfurt), Asia Pacific (Tokyo),
                    Asia Pacific (Singapore), Asia Pacific (Sydney), South America (São Paulo), and
                    Asia Pacific (Mumbai).</p>
            </li>
            <li>
               <p>The environment type <code>WINDOWS_SERVER_2019_CONTAINER</code> is available only in regions
                    US East (N. Virginia), US East (Ohio), US West (Oregon), Asia Pacific (Sydney), 
                    Asia Pacific (Tokyo), Asia Pacific (Mumbai) and
                    EU (Ireland).</p>
            </li>
            <li>
               <p>The environment type <code>WINDOWS_SERVER_2022_CONTAINER</code> is available only in regions
                    US East (N. Virginia), US East (Ohio), US West (Oregon), EU (Ireland), EU (Frankfurt), 
                    Asia Pacific (Sydney), Asia Pacific (Singapore), Asia Pacific (Tokyo), South America (São Paulo) and
                    Asia Pacific (Mumbai).</p>
            </li>
         </ul>
         <p>For more information, see <a href="https://docs.aws.amazon.com/codebuild/latest/userguide/build-env-ref-compute-types.html">Build environment compute types</a> in the <i>CodeBuild
                user guide</i>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create fleet
fleet = provider.codebuild.Fleet {
    name = "value"  # <p>The name of the compute fleet.</p>
    compute_type = "value"  # <p>Information about the compute resources the compute fleet uses. Available values
            include:</p>
         <ul>
            <li>
               <p>
                  <code>ATTRIBUTE_BASED_COMPUTE</code>: Specify the amount of vCPUs, memory, disk space, and the type of machine.</p>
               <note>
                  <p> If you use <code>ATTRIBUTE_BASED_COMPUTE</code>, you must define your attributes by using <code>computeConfiguration</code>. CodeBuild 
                        will select the cheapest instance that satisfies your specified attributes. For more information, see <a href="https://docs.aws.amazon.com/codebuild/latest/userguide/build-env-ref-compute-types.html#environment-reserved-capacity.types">Reserved capacity environment 
                        types</a> in the <i>CodeBuild User Guide</i>.</p>
               </note>
            </li>
            <li>
               <p>
                  <code>CUSTOM_INSTANCE_TYPE</code>: Specify the instance type for your compute fleet. For a list of supported instance types, see <a href="https://docs.aws.amazon.com/codebuild/latest/userguide/build-env-ref-compute-types.html#environment-reserved-capacity.instance-types">Supported instance families
                        </a> in the <i>CodeBuild User Guide</i>.</p>
            </li>
            <li>
               <p>
                  <code>BUILD_GENERAL1_SMALL</code>: Use up to 4 GiB memory and 2 vCPUs for
                    builds.</p>
            </li>
            <li>
               <p>
                  <code>BUILD_GENERAL1_MEDIUM</code>: Use up to 8 GiB memory and 4 vCPUs for
                    builds.</p>
            </li>
            <li>
               <p>
                  <code>BUILD_GENERAL1_LARGE</code>: Use up to 16 GiB memory and 8 vCPUs for
                    builds, depending on your environment type.</p>
            </li>
            <li>
               <p>
                  <code>BUILD_GENERAL1_XLARGE</code>: Use up to 72 GiB memory and 36 vCPUs for
                    builds, depending on your environment type.</p>
            </li>
            <li>
               <p>
                  <code>BUILD_GENERAL1_2XLARGE</code>: Use up to 144 GiB memory, 72 vCPUs, and
                    824 GB of SSD storage for builds. This compute type supports Docker images up to
                    100 GB uncompressed.</p>
            </li>
            <li>
               <p>
                  <code>BUILD_LAMBDA_1GB</code>: Use up to 1 GiB memory for
                    builds. Only available for environment type <code>LINUX_LAMBDA_CONTAINER</code> and <code>ARM_LAMBDA_CONTAINER</code>.</p>
            </li>
            <li>
               <p>
                  <code>BUILD_LAMBDA_2GB</code>: Use up to 2 GiB memory for
                    builds. Only available for environment type <code>LINUX_LAMBDA_CONTAINER</code> and <code>ARM_LAMBDA_CONTAINER</code>.</p>
            </li>
            <li>
               <p>
                  <code>BUILD_LAMBDA_4GB</code>: Use up to 4 GiB memory for
                    builds. Only available for environment type <code>LINUX_LAMBDA_CONTAINER</code> and <code>ARM_LAMBDA_CONTAINER</code>.</p>
            </li>
            <li>
               <p>
                  <code>BUILD_LAMBDA_8GB</code>: Use up to 8 GiB memory for
                    builds. Only available for environment type <code>LINUX_LAMBDA_CONTAINER</code> and <code>ARM_LAMBDA_CONTAINER</code>.</p>
            </li>
            <li>
               <p>
                  <code>BUILD_LAMBDA_10GB</code>: Use up to 10 GiB memory for
                    builds. Only available for environment type <code>LINUX_LAMBDA_CONTAINER</code> and <code>ARM_LAMBDA_CONTAINER</code>.</p>
            </li>
         </ul>
         <p> If you use <code>BUILD_GENERAL1_SMALL</code>: </p>
         <ul>
            <li>
               <p> For environment type <code>LINUX_CONTAINER</code>, you can use up to 4 GiB
                    memory and 2 vCPUs for builds. </p>
            </li>
            <li>
               <p> For environment type <code>LINUX_GPU_CONTAINER</code>, you can use up to 16
                    GiB memory, 4 vCPUs, and 1 NVIDIA A10G Tensor Core GPU for builds.</p>
            </li>
            <li>
               <p> For environment type <code>ARM_CONTAINER</code>, you can use up to 4 GiB
                    memory and 2 vCPUs on ARM-based processors for builds.</p>
            </li>
         </ul>
         <p> If you use <code>BUILD_GENERAL1_LARGE</code>: </p>
         <ul>
            <li>
               <p> For environment type <code>LINUX_CONTAINER</code>, you can use up to 16 GiB
                    memory and 8 vCPUs for builds. </p>
            </li>
            <li>
               <p> For environment type <code>LINUX_GPU_CONTAINER</code>, you can use up to 255
                    GiB memory, 32 vCPUs, and 4 NVIDIA Tesla V100 GPUs for builds.</p>
            </li>
            <li>
               <p> For environment type <code>ARM_CONTAINER</code>, you can use up to 16 GiB
                    memory and 8 vCPUs on ARM-based processors for builds.</p>
            </li>
         </ul>
         <p>For more information, see <a href="https://docs.aws.amazon.com/codebuild/latest/userguide/build-env-ref-compute-types.html#environment.types">On-demand environment types</a> 
                in the <i>CodeBuild User Guide.</i>
         </p>
    base_capacity = "value"  # <p>The initial number of machines allocated to the ﬂeet, which deﬁnes the number of builds that can run in parallel.</p>
    environment_type = "value"  # <p>The environment type of the compute fleet.</p>
         <ul>
            <li>
               <p>The environment type <code>ARM_CONTAINER</code> is available only in regions
                    US East (N. Virginia), US East (Ohio), US West (Oregon), EU (Ireland),
                    Asia Pacific (Mumbai), Asia Pacific (Tokyo), Asia Pacific (Singapore), Asia Pacific (Sydney), 
                    EU (Frankfurt), and South America (São Paulo).</p>
            </li>
            <li>
               <p>The environment type <code>ARM_EC2</code> is available only in regions
                    US East (N. Virginia), US East (Ohio), US West (Oregon), EU (Ireland), 
                    EU (Frankfurt), Asia Pacific (Tokyo),
                    Asia Pacific (Singapore), Asia Pacific (Sydney), South America (São Paulo), and
                    Asia Pacific (Mumbai).</p>
            </li>
            <li>
               <p>The environment type <code>LINUX_CONTAINER</code> is available only in regions
                    US East (N. Virginia), US East (Ohio), US West (Oregon), EU (Ireland), 
                    EU (Frankfurt), Asia Pacific (Tokyo),
                    Asia Pacific (Singapore), Asia Pacific (Sydney), South America (São Paulo), and
                    Asia Pacific (Mumbai).</p>
            </li>
            <li>
               <p>The environment type <code>LINUX_EC2</code> is available only in regions
                    US East (N. Virginia), US East (Ohio), US West (Oregon), EU (Ireland), 
                    EU (Frankfurt), Asia Pacific (Tokyo),
                    Asia Pacific (Singapore), Asia Pacific (Sydney), South America (São Paulo), and
                    Asia Pacific (Mumbai).</p>
            </li>
            <li>
               <p>The environment type <code>LINUX_GPU_CONTAINER</code> is available only in
                    regions US East (N. Virginia), US East (Ohio), US West (Oregon), EU (Ireland), 
                    EU (Frankfurt), Asia Pacific (Tokyo), and Asia Pacific (Sydney).</p>
            </li>
            <li>
               <p>The environment type <code>MAC_ARM</code> is available for Medium fleets only in
                    regions US East (N. Virginia), US East (Ohio), US West (Oregon), Asia Pacific (Sydney), 
                    and EU (Frankfurt)</p>
            </li>
            <li>
               <p>The environment type <code>MAC_ARM</code> is available for Large fleets only in
                    regions US East (N. Virginia), US East (Ohio), US West (Oregon), and Asia Pacific (Sydney).</p>
            </li>
            <li>
               <p>The environment type <code>WINDOWS_EC2</code> is available only in regions
                    US East (N. Virginia), US East (Ohio), US West (Oregon), EU (Ireland), 
                    EU (Frankfurt), Asia Pacific (Tokyo),
                    Asia Pacific (Singapore), Asia Pacific (Sydney), South America (São Paulo), and
                    Asia Pacific (Mumbai).</p>
            </li>
            <li>
               <p>The environment type <code>WINDOWS_SERVER_2019_CONTAINER</code> is available only in regions
                    US East (N. Virginia), US East (Ohio), US West (Oregon), Asia Pacific (Sydney), 
                    Asia Pacific (Tokyo), Asia Pacific (Mumbai) and
                    EU (Ireland).</p>
            </li>
            <li>
               <p>The environment type <code>WINDOWS_SERVER_2022_CONTAINER</code> is available only in regions
                    US East (N. Virginia), US East (Ohio), US West (Oregon), EU (Ireland), EU (Frankfurt), 
                    Asia Pacific (Sydney), Asia Pacific (Singapore), Asia Pacific (Tokyo), South America (São Paulo) and
                    Asia Pacific (Mumbai).</p>
            </li>
         </ul>
         <p>For more information, see <a href="https://docs.aws.amazon.com/codebuild/latest/userguide/build-env-ref-compute-types.html">Build environment compute types</a> in the <i>CodeBuild
                user guide</i>.</p>
}

```

---


### Report

Report resource

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


### Report_group

ReportGroup resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | <p>
      The name of the report group.
    </p> |
| `export_config` | String | ✅ | <p>
      A <code>ReportExportConfig</code> object that contains information about where the report group test results are exported.
    </p> |
| `type` | String | ✅ | <p>
      The type of report group.
    </p> |
| `tags` | Vec<String> |  | <p>
      A list of tag key and value pairs associated with this report group.
    </p>
         <p>These tags are available for use by Amazon Web Services services that support CodeBuild report group
      tags.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create report_group
report_group = provider.codebuild.Report_group {
    name = "value"  # <p>
      The name of the report group.
    </p>
    export_config = "value"  # <p>
      A <code>ReportExportConfig</code> object that contains information about where the report group test results are exported.
    </p>
    type = "value"  # <p>
      The type of report group.
    </p>
}

```

---


### Source_credentials

SourceCredentials resource

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


### Report_group_trend

ReportGroupTrend resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `raw_data` | Vec<String> | <p>An array that contains the raw data for each report.</p> |
| `stats` | String | <p>Contains the accumulated trend data.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access report_group_trend outputs
report_group_trend_id = report_group_trend.id
report_group_trend_raw_data = report_group_trend.raw_data
report_group_trend_stats = report_group_trend.stats
```

---


### Project

Project resource

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `secondary_sources` | Vec<String> |  | <p>An array of <code>ProjectSource</code> objects. </p> |
| `description` | String |  | <p>A description that makes the build project easy to identify.</p> |
| `vpc_config` | String |  | <p>VpcConfig enables CodeBuild to access resources in an Amazon VPC.</p>
         <note>
            <p>If you're using compute fleets during project creation, do not provide vpcConfig.</p>
         </note> |
| `secondary_source_versions` | Vec<String> |  | <p>An array of <code>ProjectSourceVersion</code> objects. If
      <code>secondarySourceVersions</code> is specified at the build level, then they take
      precedence over these <code>secondarySourceVersions</code> (at the project level).
    </p> |
| `logs_config` | String |  | <p>Information about logs for the build project. These can be logs in CloudWatch Logs, logs
      uploaded to a specified S3 bucket, or both. </p> |
| `name` | String | ✅ | <p>The name of the build project.</p> |
| `tags` | Vec<String> |  | <p>A list of tag key and value pairs associated with this build project.</p>
         <p>These tags are available for use by Amazon Web Services services that support CodeBuild build project
      tags.</p> |
| `source` | String | ✅ | <p>Information about the build input source code for the build project.</p> |
| `cache` | String |  | <p>Stores recently used information so that it can be quickly accessed at a later
        time.</p> |
| `concurrent_build_limit` | i64 |  | <p>The maximum number of concurrent builds that are allowed for this project.</p>
         <p>New builds are only started if the current number of builds is less than or equal to this limit. 
  If the current build count meets this limit, new builds are throttled and are not run.</p> |
| `build_batch_config` | String |  | <p>A <a>ProjectBuildBatchConfig</a>
 object that defines the batch build options
            for the project.</p> |
| `source_version` | String |  | <p>A version of the build input to be built for this project. If not specified, the latest
            version is used. If specified, it must be one of: </p>
         <ul>
            <li>
               <p>For CodeCommit: the commit ID, branch, or Git tag to use.</p>
            </li>
            <li>
               <p>For GitHub: the commit ID, pull request ID, branch name, or tag name that
          corresponds to the version of the source code you want to build. If a pull
          request ID is specified, it must use the format <code>pr/pull-request-ID</code>
          (for example <code>pr/25</code>). If a branch name is specified, the branch's
          HEAD commit ID is used. If not specified, the default branch's HEAD commit ID is
          used.</p>
            </li>
            <li>
               <p>For GitLab: the commit ID, branch, or Git tag to use.</p>
            </li>
            <li>
               <p>For Bitbucket: the commit ID, branch name, or tag name that corresponds to the
          version of the source code you want to build. If a branch name is specified, the
          branch's HEAD commit ID is used. If not specified, the default branch's HEAD
          commit ID is used.</p>
            </li>
            <li>
               <p>For Amazon S3: the version ID of the object that represents the build input ZIP
          file to use.</p>
            </li>
         </ul>
         <p>If <code>sourceVersion</code> is specified at the build level, then that version takes
            precedence over this <code>sourceVersion</code> (at the project level). </p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/codebuild/latest/userguide/sample-source-version.html">Source Version Sample
                with CodeBuild</a> in the <i>CodeBuild User Guide</i>. 
    </p> |
| `environment` | String | ✅ | <p>Information about the build environment for the build project.</p> |
| `artifacts` | String | ✅ | <p>Information about the build output artifacts for the build project.</p> |
| `secondary_artifacts` | Vec<String> |  | <p>An array of <code>ProjectArtifacts</code> objects. </p> |
| `service_role` | String | ✅ | <p>The ARN of the IAM role that enables CodeBuild to interact with dependent Amazon Web Services services
      on behalf of the Amazon Web Services account.</p> |
| `timeout_in_minutes` | i64 |  | <p>How long, in minutes, from 5 to 2160 (36 hours), for CodeBuild to wait before it times out
      any build that has not been marked as completed. The default is 60 minutes.</p> |
| `encryption_key` | String |  | <p>The Key Management Service customer master key (CMK) to be used for encrypting the build output
      artifacts.</p>
         <note>
            <p>You can use a cross-account KMS key to encrypt the build output artifacts if your
        service role has permission to that key. </p>
         </note>
         <p>You can specify either the Amazon Resource Name (ARN) of the CMK or, if available, the CMK's alias (using
        the format <code>alias/<alias-name></code>).
    </p> |
| `badge_enabled` | bool |  | <p>Set this to true to generate a publicly accessible URL for your project's build
        badge.</p> |
| `queued_timeout_in_minutes` | i64 |  | <p>The number of minutes a build is allowed to be queued before it times out. </p> |
| `file_system_locations` | Vec<String> |  | <p>
      An array of <code>ProjectFileSystemLocation</code> objects for a CodeBuild build project. A <code>ProjectFileSystemLocation</code> object 
      specifies the <code>identifier</code>, <code>location</code>, <code>mountOptions</code>, 
      <code>mountPoint</code>, and <code>type</code> of a file system created using Amazon Elastic File System.
  </p> |
| `auto_retry_limit` | i64 |  | <p>The maximum number of additional automatic retries after a failed build. For example, if the 
      auto-retry limit is set to 2, CodeBuild will call the <code>RetryBuild</code> API to automatically 
      retry your build for up to 2 additional times.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create project
project = provider.codebuild.Project {
    name = "value"  # <p>The name of the build project.</p>
    source = "value"  # <p>Information about the build input source code for the build project.</p>
    environment = "value"  # <p>Information about the build environment for the build project.</p>
    artifacts = "value"  # <p>Information about the build output artifacts for the build project.</p>
    service_role = "value"  # <p>The ARN of the IAM role that enables CodeBuild to interact with dependent Amazon Web Services services
      on behalf of the Amazon Web Services account.</p>
}

```

---


### Project_visibility

ProjectVisibility resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `project_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the build project.</p> |
| `project_visibility` | String | ✅ |  |
| `resource_access_role` | String |  | <p>The ARN of the IAM role that enables CodeBuild to access the CloudWatch Logs and Amazon S3 artifacts for
      the project's builds.</p> |



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


### Code_coverages

CodeCoverages resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>If there are more items to return, this contains a token that is passed to a subsequent
            call to <code>DescribeCodeCoverages</code> to retrieve the next set of items.</p> |
| `code_coverages` | Vec<String> | <p>An array of <code>CodeCoverage</code> objects that contain the results.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access code_coverages outputs
code_coverages_id = code_coverages.id
code_coverages_next_token = code_coverages.next_token
code_coverages_code_coverages = code_coverages.code_coverages
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple webhook resources
webhook_0 = provider.codebuild.Webhook {
    project_name = "value-0"
}
webhook_1 = provider.codebuild.Webhook {
    project_name = "value-1"
}
webhook_2 = provider.codebuild.Webhook {
    project_name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    webhook = provider.codebuild.Webhook {
        project_name = "production-value"
    }
```

---

## Related Documentation

- [AWS Codebuild Documentation](https://docs.aws.amazon.com/codebuild/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
