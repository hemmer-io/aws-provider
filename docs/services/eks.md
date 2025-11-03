# Eks Service



**Resources**: 18

---

## Overview

The eks service provides access to 18 resource types:

- [Nodegroup_config](#nodegroup_config) [U]
- [Nodegroup_version](#nodegroup_version) [U]
- [Fargate_profile](#fargate_profile) [CRD]
- [Nodegroup](#nodegroup) [CRD]
- [Addon_versions](#addon_versions) [R]
- [Insights_refresh](#insights_refresh) [R]
- [Cluster](#cluster) [CRD]
- [Addon](#addon) [CRUD]
- [Identity_provider_config](#identity_provider_config) [R]
- [Insight](#insight) [R]
- [Addon_configuration](#addon_configuration) [R]
- [Pod_identity_association](#pod_identity_association) [CRUD]
- [Access_entry](#access_entry) [CRUD]
- [Cluster_versions](#cluster_versions) [R]
- [Cluster_version](#cluster_version) [U]
- [Cluster_config](#cluster_config) [U]
- [Update](#update) [R]
- [Eks_anywhere_subscription](#eks_anywhere_subscription) [CRUD]

---

## Resources


### Nodegroup_config

NodegroupConfig resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `cluster_name` | String | ✅ | <p>The name of your cluster.</p> |
| `labels` | String |  | <p>The Kubernetes <code>labels</code> to apply to the nodes in the node group after the
            update.</p> |
| `nodegroup_name` | String | ✅ | <p>The name of the managed node group to update.</p> |
| `scaling_config` | String |  | <p>The scaling configuration details for the Auto Scaling group after the update.</p> |
| `node_repair_config` | String |  | <p>The node auto repair configuration for the node group.</p> |
| `taints` | String |  | <p>The Kubernetes taints to be applied to the nodes in the node group after the update. For
            more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/node-taints-managed-node-groups.html">Node taints on
                managed node groups</a>.</p> |
| `update_config` | String |  | <p>The node group update configuration.</p> |
| `client_request_token` | String |  | <p>A unique, case-sensitive identifier that you provide to ensure
the idempotency of the request.</p> |



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


### Nodegroup_version

NodegroupVersion resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `nodegroup_name` | String | ✅ | <p>The name of the managed node group to update.</p> |
| `version` | String |  | <p>The Kubernetes version to update to. If no version is specified, then the Kubernetes version of
            the node group does not change. You can specify the Kubernetes version of the cluster to
            update the node group to the latest AMI version of the cluster's Kubernetes version.
            If you specify <code>launchTemplate</code>, and your launch template uses a custom AMI, then don't specify  <code>version</code>,
            or the node group  update will fail. For more information about using launch templates with Amazon EKS, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/launch-templates.html">Customizing managed nodes with launch templates</a> in the <i>Amazon EKS User Guide</i>.</p> |
| `release_version` | String |  | <p>The AMI version of the Amazon EKS optimized AMI to use for the update. By default, the
            latest available AMI version for the node group's Kubernetes version is used. For information
            about Linux versions, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/eks-linux-ami-versions.html">Amazon EKS optimized Amazon Linux AMI versions</a> in the
            <i>Amazon EKS User Guide</i>. Amazon EKS managed node groups support the November 2022 and later releases
            of the Windows AMIs. For information about Windows versions, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/eks-ami-versions-windows.html">Amazon EKS
                optimized Windows AMI versions</a> in the <i>Amazon EKS User Guide</i>.</p>
         <p>If you specify <code>launchTemplate</code>, and your launch template uses a custom AMI, then don't specify 
                <code>releaseVersion</code>, or the node group  update will fail.
            For more information about using launch templates with Amazon EKS, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/launch-templates.html">Customizing managed nodes with launch templates</a> in the <i>Amazon EKS User Guide</i>.</p> |
| `launch_template` | String |  | <p>An object representing a node group's launch template specification. You can only
            update a node group using a launch template if the node group was originally deployed
            with a launch template. When updating, you must specify the same launch template ID or
            name that was used to create the node group.</p> |
| `cluster_name` | String | ✅ | <p>The name of your cluster.</p> |
| `force` | bool |  | <p>Force the update if any <code>Pod</code> on the existing node group can't be drained
            due to a <code>Pod</code> disruption budget issue. If an update fails because all Pods
            can't be drained, you can force the update after it fails to terminate the old node
            whether or not any <code>Pod</code> is running on the node.</p> |
| `client_request_token` | String |  | <p>A unique, case-sensitive identifier that you provide to ensure
the idempotency of the request.</p> |



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


### Fargate_profile

FargateProfile resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_request_token` | String |  | <p>A unique, case-sensitive identifier that you provide to ensure
the idempotency of the request.</p> |
| `pod_execution_role_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the <code>Pod</code> execution role to use for a <code>Pod</code>
            that matches the selectors in the Fargate profile. The <code>Pod</code> execution role
            allows Fargate infrastructure to register with your cluster as a node, and it provides
            read access to Amazon ECR image repositories. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/pod-execution-role.html">
               <code>Pod</code> execution role</a> in the <i>Amazon EKS User Guide</i>.</p> |
| `cluster_name` | String | ✅ | <p>The name of your cluster.</p> |
| `selectors` | Vec<String> |  | <p>The selectors to match for a <code>Pod</code> to use this Fargate profile. Each
            selector must have an associated Kubernetes <code>namespace</code>. Optionally, you can also
            specify <code>labels</code> for a <code>namespace</code>. You may specify up to five
            selectors in a Fargate profile.</p> |
| `subnets` | String |  | <p>The IDs of subnets to launch a <code>Pod</code> into. A <code>Pod</code> running on
            Fargate isn't assigned a public IP address, so only private subnets (with no direct
            route to an Internet Gateway) are accepted for this parameter.</p> |
| `fargate_profile_name` | String | ✅ | <p>The name of the Fargate profile.</p> |
| `tags` | HashMap<String, String> |  | <p>Metadata that assists with categorization and organization.
            Each tag consists of a key and an optional value. You define both. Tags don't
            propagate to any other cluster or Amazon Web Services resources.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `fargate_profile` | String | <p>The full description of your Fargate profile.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create fargate_profile
fargate_profile = provider.eks.Fargate_profile {
    pod_execution_role_arn = "value"  # <p>The Amazon Resource Name (ARN) of the <code>Pod</code> execution role to use for a <code>Pod</code>
            that matches the selectors in the Fargate profile. The <code>Pod</code> execution role
            allows Fargate infrastructure to register with your cluster as a node, and it provides
            read access to Amazon ECR image repositories. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/pod-execution-role.html">
               <code>Pod</code> execution role</a> in the <i>Amazon EKS User Guide</i>.</p>
    cluster_name = "value"  # <p>The name of your cluster.</p>
    fargate_profile_name = "value"  # <p>The name of the Fargate profile.</p>
}

# Access fargate_profile outputs
fargate_profile_id = fargate_profile.id
fargate_profile_fargate_profile = fargate_profile.fargate_profile
```

---


### Nodegroup

Nodegroup resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `taints` | Vec<String> |  | <p>The Kubernetes taints to be applied to the nodes in the node group. For more information,
            see <a href="https://docs.aws.amazon.com/eks/latest/userguide/node-taints-managed-node-groups.html">Node taints on
                managed node groups</a>.</p> |
| `cluster_name` | String | ✅ | <p>The name of your cluster.</p> |
| `instance_types` | String |  | <p>Specify the instance types for a node group. If you specify a GPU instance type, make
            sure to also specify an applicable GPU AMI type with the <code>amiType</code> parameter.
            If you specify <code>launchTemplate</code>, then you can specify zero or one instance
            type in your launch template <i>or</i> you can specify 0-20 instance types
            for <code>instanceTypes</code>. If however, you specify an instance type in your launch
            template <i>and</i> specify any <code>instanceTypes</code>, the node group
            deployment will fail. If you don't specify an instance type in a launch template or for
                <code>instanceTypes</code>, then <code>t3.medium</code> is used, by default. If you
            specify <code>Spot</code> for <code>capacityType</code>, then we recommend specifying
            multiple values for <code>instanceTypes</code>. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/managed-node-groups.html#managed-node-group-capacity-types">Managed node group capacity types</a> and <a href="https://docs.aws.amazon.com/eks/latest/userguide/launch-templates.html">Customizing managed nodes with
                launch templates</a> in the <i>Amazon EKS User Guide</i>.</p> |
| `capacity_type` | String |  | <p>The capacity type for your node group.</p> |
| `version` | String |  | <p>The Kubernetes version to use for your managed nodes. By default, the Kubernetes version of the
            cluster is used, and this is the only accepted specified value. If you specify <code>launchTemplate</code>,
            and your launch template uses a custom AMI, then don't specify  <code>version</code>, or the node group 
            deployment will fail. For more information about using launch templates with Amazon EKS, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/launch-templates.html">Customizing managed nodes with launch templates</a> in the <i>Amazon EKS User Guide</i>.</p> |
| `labels` | HashMap<String, String> |  | <p>The Kubernetes <code>labels</code> to apply to the nodes in the node group when they are
            created.</p> |
| `node_role` | String | ✅ | <p>The Amazon Resource Name (ARN) of the IAM role to associate with your node group. The Amazon EKS worker
            node <code>kubelet</code> daemon makes calls to Amazon Web Services APIs on your behalf. Nodes receive
            permissions for these API calls through an IAM instance profile and associated
            policies. Before you can launch nodes and register them into a cluster, you must create
            an IAM role for those nodes to use when they are launched. For more information, see
                <a href="https://docs.aws.amazon.com/eks/latest/userguide/create-node-role.html">Amazon EKS
                node IAM role</a> in the <i>
               <i>Amazon EKS User Guide</i>
            </i>.
            If you specify <code>launchTemplate</code>, then don't specify  <code>
               <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_IamInstanceProfile.html">IamInstanceProfile</a>
            </code> in your launch template, or the node group 
            deployment will fail. For more information about using launch templates with Amazon EKS, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/launch-templates.html">Customizing managed nodes with launch templates</a> in the <i>Amazon EKS User Guide</i>.</p> |
| `update_config` | String |  | <p>The node group update configuration.</p> |
| `scaling_config` | String |  | <p>The scaling configuration details for the Auto Scaling group that is created for your
            node group.</p> |
| `launch_template` | String |  | <p>An object representing a node group's launch template specification. When using this
            object, don't directly specify <code>instanceTypes</code>, <code>diskSize</code>, or
                <code>remoteAccess</code>. You cannot later specify a different launch template ID
            or name than what was used to create the node group.</p>
         <p>Make sure that the launch template meets the requirements in
                <code>launchTemplateSpecification</code>. Also refer to <a href="https://docs.aws.amazon.com/eks/latest/userguide/launch-templates.html">Customizing managed nodes with
                launch templates</a> in the <i>Amazon EKS User Guide</i>.</p> |
| `remote_access` | String |  | <p>The remote access configuration to use with your node group. For Linux, the protocol
            is SSH. For Windows, the protocol is RDP. If you specify <code>launchTemplate</code>, then don't specify 
                <code>remoteAccess</code>, or the node group  deployment will fail.
            For more information about using launch templates with Amazon EKS, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/launch-templates.html">Customizing managed nodes with launch templates</a> in the <i>Amazon EKS User Guide</i>.</p> |
| `node_repair_config` | String |  | <p>The node auto repair configuration for the node group.</p> |
| `release_version` | String |  | <p>The AMI version of the Amazon EKS optimized AMI to use with your node group. By default,
            the latest available AMI version for the node group's current Kubernetes version is used. For
            information about Linux versions, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/eks-linux-ami-versions.html">Amazon EKS
                optimized Amazon Linux AMI versions</a> in the <i>Amazon EKS User Guide</i>. Amazon EKS managed node
            groups support the November 2022 and later releases of the Windows AMIs. For information
            about Windows versions, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/eks-ami-versions-windows.html">Amazon EKS
                optimized Windows AMI versions</a> in the <i>Amazon EKS User Guide</i>.</p>
         <p>If you specify <code>launchTemplate</code>, and your launch template uses a custom AMI, then don't specify 
                <code>releaseVersion</code>, or the node group  deployment will fail.
            For more information about using launch templates with Amazon EKS, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/launch-templates.html">Customizing managed nodes with launch templates</a> in the <i>Amazon EKS User Guide</i>.</p> |
| `subnets` | String | ✅ | <p>The subnets to use for the Auto Scaling group that is created for your node group.
            If you specify <code>launchTemplate</code>, then don't specify  <code>
               <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_CreateNetworkInterface.html">SubnetId</a>
            </code> in your launch template, or the node group  deployment
            will fail. For more information about using launch templates with Amazon EKS, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/launch-templates.html">Customizing managed nodes with launch templates</a> in the <i>Amazon EKS User Guide</i>.</p> |
| `nodegroup_name` | String | ✅ | <p>The unique name to give your node group.</p> |
| `ami_type` | String |  | <p>The AMI type for your node group. If you specify <code>launchTemplate</code>, and your launch template uses a custom AMI,
                then don't specify <code>amiType</code>, or the node group  deployment
            will fail. If your launch template uses a Windows custom AMI, then add
                <code>eks:kube-proxy-windows</code> to your Windows nodes <code>rolearn</code> in
            the <code>aws-auth</code>
            <code>ConfigMap</code>. For more information about using launch templates with Amazon EKS, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/launch-templates.html">Customizing managed nodes with launch templates</a> in the <i>Amazon EKS User Guide</i>.</p> |
| `client_request_token` | String |  | <p>A unique, case-sensitive identifier that you provide to ensure
the idempotency of the request.</p> |
| `tags` | HashMap<String, String> |  | <p>Metadata that assists with categorization and organization.
            Each tag consists of a key and an optional value. You define both. Tags don't
            propagate to any other cluster or Amazon Web Services resources.</p> |
| `disk_size` | i64 |  | <p>The root device disk size (in GiB) for your node group instances. The default disk
            size is 20 GiB for Linux and Bottlerocket. The default disk size is 50 GiB for Windows.
            If you specify <code>launchTemplate</code>, then don't specify  <code>diskSize</code>, or the node group 
            deployment will fail. For more information about using launch templates with Amazon EKS, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/launch-templates.html">Customizing managed nodes with launch templates</a> in the <i>Amazon EKS User Guide</i>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `nodegroup` | String | <p>The full description of your node group.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create nodegroup
nodegroup = provider.eks.Nodegroup {
    cluster_name = "value"  # <p>The name of your cluster.</p>
    node_role = "value"  # <p>The Amazon Resource Name (ARN) of the IAM role to associate with your node group. The Amazon EKS worker
            node <code>kubelet</code> daemon makes calls to Amazon Web Services APIs on your behalf. Nodes receive
            permissions for these API calls through an IAM instance profile and associated
            policies. Before you can launch nodes and register them into a cluster, you must create
            an IAM role for those nodes to use when they are launched. For more information, see
                <a href="https://docs.aws.amazon.com/eks/latest/userguide/create-node-role.html">Amazon EKS
                node IAM role</a> in the <i>
               <i>Amazon EKS User Guide</i>
            </i>.
            If you specify <code>launchTemplate</code>, then don't specify  <code>
               <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_IamInstanceProfile.html">IamInstanceProfile</a>
            </code> in your launch template, or the node group 
            deployment will fail. For more information about using launch templates with Amazon EKS, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/launch-templates.html">Customizing managed nodes with launch templates</a> in the <i>Amazon EKS User Guide</i>.</p>
    subnets = "value"  # <p>The subnets to use for the Auto Scaling group that is created for your node group.
            If you specify <code>launchTemplate</code>, then don't specify  <code>
               <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_CreateNetworkInterface.html">SubnetId</a>
            </code> in your launch template, or the node group  deployment
            will fail. For more information about using launch templates with Amazon EKS, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/launch-templates.html">Customizing managed nodes with launch templates</a> in the <i>Amazon EKS User Guide</i>.</p>
    nodegroup_name = "value"  # <p>The unique name to give your node group.</p>
}

# Access nodegroup outputs
nodegroup_id = nodegroup.id
nodegroup_nodegroup = nodegroup.nodegroup
```

---


### Addon_versions

AddonVersions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `addons` | Vec<String> | <p>The list of available versions with Kubernetes version compatibility and other
            properties.</p> |
| `next_token` | String | <p>The <code>nextToken</code> value to include in a future
                <code>DescribeAddonVersions</code> request. When the results of a
                <code>DescribeAddonVersions</code> request exceed <code>maxResults</code>, you can
            use this value to retrieve the next page of results. This value is <code>null</code>
            when there are no more results to return.</p>
         <note>
            <p>This token should be treated as an opaque identifier that is used only to
                retrieve the next items in a list and not for other programmatic purposes.</p>
         </note> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access addon_versions outputs
addon_versions_id = addon_versions.id
addon_versions_addons = addon_versions.addons
addon_versions_next_token = addon_versions.next_token
```

---


### Insights_refresh

InsightsRefresh resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | String | <p>The current status of the insights refresh operation.</p> |
| `started_at` | String | <p>The date and time when the insights refresh operation started.</p> |
| `message` | String | <p>The message associated with the insights refresh operation.</p> |
| `ended_at` | String | <p>The date and time when the insights refresh operation ended.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access insights_refresh outputs
insights_refresh_id = insights_refresh.id
insights_refresh_status = insights_refresh.status
insights_refresh_started_at = insights_refresh.started_at
insights_refresh_message = insights_refresh.message
insights_refresh_ended_at = insights_refresh.ended_at
```

---


### Cluster

Cluster resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_request_token` | String |  | <p>A unique, case-sensitive identifier that you provide to ensure
the idempotency of the request.</p> |
| `resources_vpc_config` | String | ✅ | <p>The VPC configuration that's used by the cluster control plane. Amazon EKS VPC resources
            have specific requirements to work properly with Kubernetes. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/network_reqs.html">Cluster VPC
                Considerations</a> and <a href="https://docs.aws.amazon.com/eks/latest/userguide/sec-group-reqs.html">Cluster Security Group Considerations</a> in the
            <i>Amazon EKS User Guide</i>. You must specify at least two subnets. You can specify up to five
            security groups. However, we recommend that you use a dedicated security group for your
            cluster control plane.</p> |
| `version` | String |  | <p>The desired Kubernetes version for your cluster. If you don't specify a value here, the
            default version available in Amazon EKS is used.</p>
         <note>
            <p>The default version might not be the latest version available.</p>
         </note> |
| `role_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the IAM role that provides permissions for the Kubernetes control plane
            to make calls to Amazon Web Services API operations on your behalf. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/service_IAM_role.html">Amazon EKS Service
                IAM Role</a> in the <i>
               <i>Amazon EKS User Guide</i>
            </i>.</p> |
| `outpost_config` | String |  | <p>An object representing the configuration of your local Amazon EKS cluster on an Amazon Web Services
            Outpost. Before creating a local cluster on an Outpost, review <a href="https://docs.aws.amazon.com/eks/latest/userguide/eks-outposts-local-cluster-overview.html">Local clusters
                for Amazon EKS on Amazon Web Services Outposts</a> in the <i>Amazon EKS User Guide</i>. This object isn't
            available for creating Amazon EKS clusters on the Amazon Web Services cloud.</p> |
| `zonal_shift_config` | String |  | <p>Enable or disable ARC zonal shift for the cluster. If zonal shift is enabled, Amazon Web Services
            configures zonal autoshift for the cluster.</p>
         <p>Zonal shift is a feature of Amazon Application Recovery Controller (ARC). ARC zonal shift is designed to be a
            temporary measure that allows you to move traffic for a resource away from an impaired
            AZ until the zonal shift expires or you cancel it. You can extend the zonal shift if
            necessary.</p>
         <p>You can start a zonal shift for an Amazon EKS cluster, or you can allow Amazon Web Services to do it for
            you by enabling <i>zonal autoshift</i>. This shift updates the flow of
            east-to-west network traffic in your cluster to only consider network endpoints for Pods
            running on worker nodes in healthy AZs. Additionally, any ALB or NLB handling ingress
            traffic for applications in your Amazon EKS cluster will automatically route traffic to
            targets in the healthy AZs. For more information about zonal shift in EKS, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/zone-shift.html">Learn about
                Amazon Application Recovery Controller (ARC) Zonal Shift in Amazon EKS</a> in the
                <i>
               <i>Amazon EKS User Guide</i>
            </i>.</p> |
| `bootstrap_self_managed_addons` | bool |  | <p>If you set this value to <code>False</code> when creating a cluster, the default
            networking add-ons will not be installed.</p>
         <p>The default networking add-ons include <code>vpc-cni</code>, <code>coredns</code>, and
                <code>kube-proxy</code>.</p>
         <p>Use this option when you plan to install third-party alternative add-ons or
            self-manage the default networking add-ons.</p> |
| `logging` | String |  | <p>Enable or disable exporting the Kubernetes control plane logs for your cluster to CloudWatch Logs .
            By default, cluster control plane logs aren't exported to CloudWatch Logs . For more information,
            see <a href="https://docs.aws.amazon.com/eks/latest/userguide/control-plane-logs.html">Amazon EKS
                Cluster control plane logs</a> in the
            <i>
               <i>Amazon EKS User Guide</i>
            </i>.</p>
         <note>
            <p>CloudWatch Logs ingestion, archive storage, and data scanning rates apply to exported
                control plane logs. For more information, see <a href="http://aws.amazon.com/cloudwatch/pricing/">CloudWatch Pricing</a>.</p>
         </note> |
| `access_config` | String |  | <p>The access configuration for the cluster.</p> |
| `remote_network_config` | String |  | <p>The configuration in the cluster for EKS Hybrid Nodes. You can add, change, or remove this
            configuration after the cluster is created.</p> |
| `compute_config` | String |  | <p>Enable or disable the compute capability of EKS Auto Mode when creating your EKS Auto
            Mode cluster. If the compute capability is enabled, EKS Auto Mode will create and delete
            EC2 Managed Instances in your Amazon Web Services account</p> |
| `deletion_protection` | bool |  | <p>Indicates whether to enable deletion protection for the cluster. When enabled, the cluster 
            cannot be deleted unless deletion protection is first disabled. This helps prevent 
            accidental cluster deletion. Default value is <code>false</code>.</p> |
| `tags` | HashMap<String, String> |  | <p>Metadata that assists with categorization and organization.
            Each tag consists of a key and an optional value. You define both. Tags don't
            propagate to any other cluster or Amazon Web Services resources.</p> |
| `kubernetes_network_config` | String |  | <p>The Kubernetes network configuration for the cluster.</p> |
| `name` | String | ✅ | <p>The unique name to give to your cluster. The name can contain only alphanumeric characters (case-sensitive),
hyphens, and underscores. It must start with an alphanumeric character and can't be longer than
100 characters. The name must be unique within the Amazon Web Services Region and Amazon Web Services account that you're 
creating the cluster in.</p> |
| `upgrade_policy` | String |  | <p>New clusters, by default, have extended support enabled. You can disable extended
            support when creating a cluster by setting this value to <code>STANDARD</code>.</p> |
| `encryption_config` | Vec<String> |  | <p>The encryption configuration for the cluster.</p> |
| `storage_config` | String |  | <p>Enable or disable the block storage capability of EKS Auto Mode when creating your EKS
            Auto Mode cluster. If the block storage capability is enabled, EKS Auto Mode will create
            and delete EBS volumes in your Amazon Web Services account.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `cluster` | String | <p>The full description of your specified cluster.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create cluster
cluster = provider.eks.Cluster {
    resources_vpc_config = "value"  # <p>The VPC configuration that's used by the cluster control plane. Amazon EKS VPC resources
            have specific requirements to work properly with Kubernetes. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/network_reqs.html">Cluster VPC
                Considerations</a> and <a href="https://docs.aws.amazon.com/eks/latest/userguide/sec-group-reqs.html">Cluster Security Group Considerations</a> in the
            <i>Amazon EKS User Guide</i>. You must specify at least two subnets. You can specify up to five
            security groups. However, we recommend that you use a dedicated security group for your
            cluster control plane.</p>
    role_arn = "value"  # <p>The Amazon Resource Name (ARN) of the IAM role that provides permissions for the Kubernetes control plane
            to make calls to Amazon Web Services API operations on your behalf. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/service_IAM_role.html">Amazon EKS Service
                IAM Role</a> in the <i>
               <i>Amazon EKS User Guide</i>
            </i>.</p>
    name = "value"  # <p>The unique name to give to your cluster. The name can contain only alphanumeric characters (case-sensitive),
hyphens, and underscores. It must start with an alphanumeric character and can't be longer than
100 characters. The name must be unique within the Amazon Web Services Region and Amazon Web Services account that you're 
creating the cluster in.</p>
}

# Access cluster outputs
cluster_id = cluster.id
cluster_cluster = cluster.cluster
```

---


### Addon

Addon resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `addon_name` | String | ✅ | <p>The name of the add-on. The name must match one of the names returned by
                <code>DescribeAddonVersions</code>.</p> |
| `tags` | HashMap<String, String> |  | <p>Metadata that assists with categorization and organization.
            Each tag consists of a key and an optional value. You define both. Tags don't
            propagate to any other cluster or Amazon Web Services resources.</p> |
| `pod_identity_associations` | Vec<String> |  | <p>An array of EKS Pod Identity associations to be created. Each association maps a Kubernetes service account to
            an IAM role.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/add-ons-iam.html">Attach an IAM Role to an Amazon EKS add-on
                using EKS Pod Identity</a> in the <i>Amazon EKS User Guide</i>.</p> |
| `configuration_values` | String |  | <p>The set of configuration values for the add-on that's created. The values that you
            provide are validated against the schema returned by
                <code>DescribeAddonConfiguration</code>.</p> |
| `cluster_name` | String | ✅ | <p>The name of your cluster.</p> |
| `client_request_token` | String |  | <p>A unique, case-sensitive identifier that you provide to ensure
the idempotency of the request.</p> |
| `namespace_config` | String |  | <p>The namespace configuration for the addon. If specified, this will override the default namespace for the addon.</p> |
| `service_account_role_arn` | String |  | <p>The Amazon Resource Name (ARN) of an existing IAM role to bind to the add-on's service account. The role must be assigned the IAM permissions required by the add-on. If you don't specify an existing IAM role, then the add-on uses the
     permissions assigned to the node IAM role. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/create-node-role.html">Amazon EKS node IAM role</a> in the <i>Amazon EKS User Guide</i>.</p>
         <note>
            <p>To specify an existing IAM role, you must have an IAM OpenID Connect (OIDC) provider created for
                your cluster. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/enable-iam-roles-for-service-accounts.html">Enabling
                    IAM roles for service accounts on your cluster</a> in the
                <i>Amazon EKS User Guide</i>.</p>
         </note> |
| `resolve_conflicts` | String |  | <p>How to resolve field value conflicts for an Amazon EKS add-on. Conflicts are handled based
            on the value you choose:</p>
         <ul>
            <li>
               <p>
                  <b>None</b> – If the self-managed version of
                    the add-on is installed on your cluster, Amazon EKS doesn't change the value.
                    Creation of the add-on might fail.</p>
            </li>
            <li>
               <p>
                  <b>Overwrite</b> – If the self-managed
                    version of the add-on is installed on your cluster and the Amazon EKS default value
                    is different than the existing value, Amazon EKS changes the value to the Amazon EKS
                    default value.</p>
            </li>
            <li>
               <p>
                  <b>Preserve</b> – This is similar to the NONE
                    option. If the self-managed version of the add-on is installed on your cluster
                    Amazon EKS doesn't change the add-on resource properties. Creation of the add-on
                    might fail if conflicts are detected. This option works differently during the
                    update operation. For more information, see <a href="https://docs.aws.amazon.com/eks/latest/APIReference/API_UpdateAddon.html">
                     <code>UpdateAddon</code>
                  </a>.</p>
            </li>
         </ul>
         <p>If you don't currently have the self-managed version of the add-on installed on your
            cluster, the Amazon EKS add-on is installed. Amazon EKS sets all values to default values,
            regardless of the option that you specify.</p> |
| `addon_version` | String |  | <p>The version of the add-on. The version must match one of the versions returned by <a href="https://docs.aws.amazon.com/eks/latest/APIReference/API_DescribeAddonVersions.html">
               <code>DescribeAddonVersions</code>
            </a>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `addon` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create addon
addon = provider.eks.Addon {
    addon_name = "value"  # <p>The name of the add-on. The name must match one of the names returned by
                <code>DescribeAddonVersions</code>.</p>
    cluster_name = "value"  # <p>The name of your cluster.</p>
}

# Access addon outputs
addon_id = addon.id
addon_addon = addon.addon
```

---


### Identity_provider_config

IdentityProviderConfig resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `identity_provider_config` | String | <p>The object that represents an OpenID Connect (OIDC) identity provider
            configuration.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access identity_provider_config outputs
identity_provider_config_id = identity_provider_config.id
identity_provider_config_identity_provider_config = identity_provider_config.identity_provider_config
```

---


### Insight

Insight resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `insight` | String | <p>The full description of the insight.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access insight outputs
insight_id = insight.id
insight_insight = insight.insight
```

---


### Addon_configuration

AddonConfiguration resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `configuration_schema` | String | <p>A JSON schema that's used to validate the configuration values you provide when an
            add-on is created or updated.</p> |
| `pod_identity_configuration` | Vec<String> | <p>The Kubernetes service account name used by the add-on, and any suggested IAM policies.
            Use this information to create an IAM Role for the add-on.</p> |
| `addon_version` | String | <p>The version of the add-on. The version must match one of the versions returned by <a href="https://docs.aws.amazon.com/eks/latest/APIReference/API_DescribeAddonVersions.html">
               <code>DescribeAddonVersions</code>
            </a>.</p> |
| `addon_name` | String | <p>The name of the add-on.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access addon_configuration outputs
addon_configuration_id = addon_configuration.id
addon_configuration_configuration_schema = addon_configuration.configuration_schema
addon_configuration_pod_identity_configuration = addon_configuration.pod_identity_configuration
addon_configuration_addon_version = addon_configuration.addon_version
addon_configuration_addon_name = addon_configuration.addon_name
```

---


### Pod_identity_association

PodIdentityAssociation resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `cluster_name` | String | ✅ | <p>The name of the cluster to create the EKS Pod Identity association in.</p> |
| `service_account` | String | ✅ | <p>The name of the Kubernetes service account inside the cluster to associate the IAM
            credentials with.</p> |
| `role_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the IAM role to associate with the service account. The EKS Pod Identity
            agent manages credentials to assume this role for applications in the containers in the
            Pods that use this service account.</p> |
| `disable_session_tags` | bool |  | <p>Disable the automatic sessions tags that are appended by EKS Pod Identity.</p>
         <p>EKS Pod Identity adds a pre-defined set of session tags when it assumes the role. You
            can use these tags to author a single role that can work across resources by allowing
            access to Amazon Web Services resources based on matching tags. By default, EKS Pod Identity attaches
            six tags, including tags for cluster name, namespace, and service account name. For the
            list of tags added by EKS Pod Identity, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/pod-id-abac.html#pod-id-abac-tags">List of session tags
                added by EKS Pod Identity</a> in the <i>Amazon EKS User Guide</i>.</p>
         <p>Amazon Web Services compresses inline session policies, managed policy ARNs, and session tags into a
            packed binary format that has a separate limit. If you receive a
                <code>PackedPolicyTooLarge</code> error indicating the packed binary format has
            exceeded the size limit, you can attempt to reduce the size by disabling the session
            tags added by EKS Pod Identity.</p> |
| `target_role_arn` | String |  | <p>The Amazon Resource Name (ARN) of the target IAM role to associate with the service account. This role
            is assumed by using the EKS Pod Identity association role, then the credentials for this
            role are injected into the Pod.</p>
         <p>When you run applications on Amazon EKS, your application might need to access Amazon Web Services
            resources from a different role that exists in the same or different Amazon Web Services account. For
            example, your application running in “Account A” might need to access resources, such as
            Amazon S3 buckets in “Account B” or within “Account A” itself. You can create a association
            to access Amazon Web Services resources in “Account B” by creating two IAM roles: a role in “Account
            A” and a role in “Account B” (which can be the same or different account), each with the
            necessary trust and permission policies. After you provide these roles in the
                <i>IAM role</i> and <i>Target IAM role</i> fields, EKS
            will perform role chaining to ensure your application gets the required permissions.
            This means Role A will assume Role B, allowing your Pods to securely access resources
            like S3 buckets in the target account.</p> |
| `client_request_token` | String |  | <p>A unique, case-sensitive identifier that you provide to ensure
the idempotency of the request.</p> |
| `tags` | HashMap<String, String> |  | <p>Metadata that assists with categorization and organization.
            Each tag consists of a key and an optional value. You define both. Tags don't
            propagate to any other cluster or Amazon Web Services resources.</p>
         <p>The following basic restrictions apply to tags:</p>
         <ul>
            <li>
               <p>Maximum number of tags per resource – 50</p>
            </li>
            <li>
               <p>For each resource, each tag key must be unique, and each tag key can have only
                    one value.</p>
            </li>
            <li>
               <p>Maximum key length – 128 Unicode characters in UTF-8</p>
            </li>
            <li>
               <p>Maximum value length – 256 Unicode characters in UTF-8</p>
            </li>
            <li>
               <p>If your tagging schema is used across multiple services and resources,
                    remember that other services may have restrictions on allowed characters.
                    Generally allowed characters are: letters, numbers, and spaces representable in
                    UTF-8, and the following characters: + - = . _ : / @.</p>
            </li>
            <li>
               <p>Tag keys and values are case-sensitive.</p>
            </li>
            <li>
               <p>Do not use <code>aws:</code>, <code>AWS:</code>, or any upper or lowercase
                    combination of such as a prefix for either keys or values as it is reserved for
                    Amazon Web Services use. You cannot edit or delete tag keys or values with this prefix. Tags with
                    this prefix do not count against your tags per resource limit.</p>
            </li>
         </ul> |
| `namespace` | String | ✅ | <p>The name of the Kubernetes namespace inside the cluster to create the EKS Pod Identity association in. The
            service account and the Pods that use the service account must be in this
            namespace.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `association` | String | <p>The full description of the EKS Pod Identity association.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create pod_identity_association
pod_identity_association = provider.eks.Pod_identity_association {
    cluster_name = "value"  # <p>The name of the cluster to create the EKS Pod Identity association in.</p>
    service_account = "value"  # <p>The name of the Kubernetes service account inside the cluster to associate the IAM
            credentials with.</p>
    role_arn = "value"  # <p>The Amazon Resource Name (ARN) of the IAM role to associate with the service account. The EKS Pod Identity
            agent manages credentials to assume this role for applications in the containers in the
            Pods that use this service account.</p>
    namespace = "value"  # <p>The name of the Kubernetes namespace inside the cluster to create the EKS Pod Identity association in. The
            service account and the Pods that use the service account must be in this
            namespace.</p>
}

# Access pod_identity_association outputs
pod_identity_association_id = pod_identity_association.id
pod_identity_association_association = pod_identity_association.association
```

---


### Access_entry

AccessEntry resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | HashMap<String, String> |  | <p>Metadata that assists with categorization and organization.
            Each tag consists of a key and an optional value. You define both. Tags don't
            propagate to any other cluster or Amazon Web Services resources.</p> |
| `type` | String |  | <p>The type of the new access entry. Valid values are <code>STANDARD</code>,
                <code>FARGATE_LINUX</code>, <code>EC2_LINUX</code>, <code>EC2_WINDOWS</code>,
                <code>EC2</code> (for EKS Auto Mode), <code>HYBRID_LINUX</code>, and
                <code>HYPERPOD_LINUX</code>. </p>
         <p>If the <code>principalArn</code> is for an IAM role that's used for self-managed
            Amazon EC2 nodes, specify <code>EC2_LINUX</code> or <code>EC2_WINDOWS</code>. Amazon EKS grants
            the necessary permissions to the node for you. If the <code>principalArn</code> is for
            any other purpose, specify <code>STANDARD</code>. If you don't specify a value, Amazon EKS
            sets the value to <code>STANDARD</code>. If you have the access mode of the cluster set
            to <code>API_AND_CONFIG_MAP</code>, it's unnecessary to create access entries for IAM
            roles used with Fargate profiles or managed Amazon EC2 nodes, because Amazon EKS creates entries
            in the <code>aws-auth</code>
            <code>ConfigMap</code> for the roles. You can't change this value once you've created
            the access entry.</p>
         <p>If you set the value to <code>EC2_LINUX</code> or <code>EC2_WINDOWS</code>, you can't
            specify values for <code>kubernetesGroups</code>, or associate an
                <code>AccessPolicy</code> to the access entry.</p> |
| `username` | String |  | <p>The username to authenticate to Kubernetes with. We recommend not specifying a username and
            letting Amazon EKS specify it for you. For more information about the value Amazon EKS specifies
            for you, or constraints before specifying your own username, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/access-entries.html#creating-access-entries">Creating
                access entries</a> in the <i>Amazon EKS User Guide</i>.</p> |
| `cluster_name` | String | ✅ | <p>The name of your cluster.</p> |
| `principal_arn` | String | ✅ | <p>The ARN of the IAM principal for the <code>AccessEntry</code>. You can specify one ARN for each access entry. You can't specify the
            same ARN in more than one access entry. This value can't be changed after access entry
            creation.</p>
         <p>The valid principals differ depending on the type of the access entry in the
                <code>type</code> field. For <code>STANDARD</code> access entries, you can use every
            IAM principal type. For nodes (<code>EC2</code> (for EKS Auto Mode),
                <code>EC2_LINUX</code>, <code>EC2_WINDOWS</code>, <code>FARGATE_LINUX</code>, and
                <code>HYBRID_LINUX</code>), the only valid ARN is IAM roles.
            
            You can't use the STS session principal type with access entries because this is a
            temporary principal for each session and not a permanent identity that can be assigned
            permissions.</p>
         <p>
            <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/best-practices.html#bp-users-federation-idp">IAM best
                practices</a> recommend using IAM roles with temporary credentials, rather
            than IAM users with long-term credentials. </p> |
| `client_request_token` | String |  | <p>A unique, case-sensitive identifier that you provide to ensure
the idempotency of the request.</p> |
| `kubernetes_groups` | String |  | <p>The value for <code>name</code> that you've specified for <code>kind: Group</code> as
            a <code>subject</code> in a Kubernetes <code>RoleBinding</code> or
                <code>ClusterRoleBinding</code> object. Amazon EKS doesn't confirm that the value for
                <code>name</code> exists in any bindings on your cluster. You can specify one or
            more names.</p>
         <p>Kubernetes authorizes the <code>principalArn</code> of the access entry to access any
            cluster objects that you've specified in a Kubernetes <code>Role</code> or
                <code>ClusterRole</code> object that is also specified in a binding's
                <code>roleRef</code>. For more information about creating Kubernetes
                <code>RoleBinding</code>, <code>ClusterRoleBinding</code>, <code>Role</code>, or
                <code>ClusterRole</code> objects, see <a href="https://kubernetes.io/docs/reference/access-authn-authz/rbac/">Using RBAC
                Authorization in the Kubernetes documentation</a>.</p>
         <p>If you want Amazon EKS to authorize the <code>principalArn</code> (instead of, or in
            addition to Kubernetes authorizing the <code>principalArn</code>), you can associate one or
            more access policies to the access entry using <code>AssociateAccessPolicy</code>. If
            you associate any access policies, the <code>principalARN</code> has all permissions
            assigned in the associated access policies and all permissions in any Kubernetes
                <code>Role</code> or <code>ClusterRole</code> objects that the group names are bound
            to.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `access_entry` | String | <p>Information about the access entry.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create access_entry
access_entry = provider.eks.Access_entry {
    cluster_name = "value"  # <p>The name of your cluster.</p>
    principal_arn = "value"  # <p>The ARN of the IAM principal for the <code>AccessEntry</code>. You can specify one ARN for each access entry. You can't specify the
            same ARN in more than one access entry. This value can't be changed after access entry
            creation.</p>
         <p>The valid principals differ depending on the type of the access entry in the
                <code>type</code> field. For <code>STANDARD</code> access entries, you can use every
            IAM principal type. For nodes (<code>EC2</code> (for EKS Auto Mode),
                <code>EC2_LINUX</code>, <code>EC2_WINDOWS</code>, <code>FARGATE_LINUX</code>, and
                <code>HYBRID_LINUX</code>), the only valid ARN is IAM roles.
            
            You can't use the STS session principal type with access entries because this is a
            temporary principal for each session and not a permanent identity that can be assigned
            permissions.</p>
         <p>
            <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/best-practices.html#bp-users-federation-idp">IAM best
                practices</a> recommend using IAM roles with temporary credentials, rather
            than IAM users with long-term credentials. </p>
}

# Access access_entry outputs
access_entry_id = access_entry.id
access_entry_access_entry = access_entry.access_entry
```

---


### Cluster_versions

ClusterVersions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>Pagination token for the next set of results.</p> |
| `cluster_versions` | Vec<String> | <p>List of cluster version information objects.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access cluster_versions outputs
cluster_versions_id = cluster_versions.id
cluster_versions_next_token = cluster_versions.next_token
cluster_versions_cluster_versions = cluster_versions.cluster_versions
```

---


### Cluster_version

ClusterVersion resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `force` | bool |  | <p>Set this value to <code>true</code> to override upgrade-blocking readiness checks when
            updating a cluster.</p> |
| `client_request_token` | String |  | <p>A unique, case-sensitive identifier that you provide to ensure
the idempotency of the request.</p> |
| `name` | String | ✅ | <p>The name of the Amazon EKS cluster to update.</p> |
| `version` | String | ✅ | <p>The desired Kubernetes version following a successful update.</p> |



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


### Cluster_config

ClusterConfig resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `logging` | String |  | <p>Enable or disable exporting the Kubernetes control plane logs for your cluster to CloudWatch Logs .
            By default, cluster control plane logs aren't exported to CloudWatch Logs . For more information,
            see <a href="https://docs.aws.amazon.com/eks/latest/userguide/control-plane-logs.html">Amazon EKS
                cluster control plane logs</a> in the
            <i>
               <i>Amazon EKS User Guide</i>
            </i>.</p>
         <note>
            <p>CloudWatch Logs ingestion, archive storage, and data scanning rates apply to exported
                control plane logs. For more information, see <a href="http://aws.amazon.com/cloudwatch/pricing/">CloudWatch Pricing</a>.</p>
         </note> |
| `resources_vpc_config` | String |  |  |
| `access_config` | String |  | <p>The access configuration for the cluster.</p> |
| `compute_config` | String |  | <p>Update the configuration of the compute capability of your EKS Auto Mode cluster. For
            example, enable the capability.</p> |
| `client_request_token` | String |  | <p>A unique, case-sensitive identifier that you provide to ensure
the idempotency of the request.</p> |
| `upgrade_policy` | String |  | <p>You can enable or disable extended support for clusters currently on standard support.
            You cannot disable extended support once it starts. You must enable extended support
            before your cluster exits standard support.</p> |
| `kubernetes_network_config` | String |  |  |
| `name` | String | ✅ | <p>The name of the Amazon EKS cluster to update.</p> |
| `storage_config` | String |  | <p>Update the configuration of the block storage capability of your EKS Auto Mode
            cluster. For example, enable the capability.</p> |
| `zonal_shift_config` | String |  | <p>Enable or disable ARC zonal shift for the cluster. If zonal shift is enabled, Amazon Web Services
            configures zonal autoshift for the cluster.</p>
         <p>Zonal shift is a feature of Amazon Application Recovery Controller (ARC). ARC zonal shift is designed to be a
            temporary measure that allows you to move traffic for a resource away from an impaired
            AZ until the zonal shift expires or you cancel it. You can extend the zonal shift if
            necessary.</p>
         <p>You can start a zonal shift for an EKS cluster, or you can allow Amazon Web Services to do it for
            you by enabling <i>zonal autoshift</i>. This shift updates the flow of
            east-to-west network traffic in your cluster to only consider network endpoints for Pods
            running on worker nodes in healthy AZs. Additionally, any ALB or NLB handling ingress
            traffic for applications in your EKS cluster will automatically route traffic to targets
            in the healthy AZs. For more information about zonal shift in EKS, see <a href="https://docs.aws.amazon.com/eks/latest/userguide/zone-shift.html">Learn about
                Amazon Application Recovery Controller (ARC) Zonal Shift in Amazon EKS</a> in the
                <i>
               <i>Amazon EKS User Guide</i>
            </i>.</p> |
| `remote_network_config` | String |  |  |
| `deletion_protection` | bool |  | <p>Specifies whether to enable or disable deletion protection for the cluster. When 
            enabled (<code>true</code>), the cluster cannot be deleted until deletion protection is 
            explicitly disabled. When disabled (<code>false</code>), the cluster can be deleted 
            normally.</p> |



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


### Update

Update resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update` | String | <p>The full description of the specified update.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access update outputs
update_id = update.id
update_update = update.update
```

---


### Eks_anywhere_subscription

EksAnywhereSubscription resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `license_quantity` | i64 |  | <p>The number of licenses to purchase with the subscription. Valid values are between 1
            and 100. This value can't be changed after creating the subscription.</p> |
| `client_request_token` | String |  | <p>A unique, case-sensitive identifier that you provide to ensure
the idempotency of the request.</p> |
| `tags` | HashMap<String, String> |  | <p>The metadata for a subscription to assist with categorization and organization. Each
            tag consists of a key and an optional value. Subscription tags don't propagate to any
            other resources associated with the subscription.</p> |
| `license_type` | String |  | <p>The license type for all licenses in the subscription. Valid value is CLUSTER. With
            the CLUSTER license type, each license covers support for a single EKS Anywhere
            cluster.</p> |
| `term` | String | ✅ | <p>An object representing the term duration and term unit type of your subscription. This
            determines the term length of your subscription. Valid values are MONTHS for term unit
            and 12 or 36 for term duration, indicating a 12 month or 36 month subscription. This
            value cannot be changed after creating the subscription.</p> |
| `auto_renew` | bool |  | <p>A boolean indicating whether the subscription auto renews at the end of the
            term.</p> |
| `name` | String | ✅ | <p>The unique name for your subscription. It must be unique in your Amazon Web Services account in the
            Amazon Web Services Region you're creating the subscription in. The name can contain only alphanumeric
            characters (case-sensitive), hyphens, and underscores. It must start with an alphabetic
            character and can't be longer than 100 characters.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `subscription` | String | <p>The full description of the subscription.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create eks_anywhere_subscription
eks_anywhere_subscription = provider.eks.Eks_anywhere_subscription {
    term = "value"  # <p>An object representing the term duration and term unit type of your subscription. This
            determines the term length of your subscription. Valid values are MONTHS for term unit
            and 12 or 36 for term duration, indicating a 12 month or 36 month subscription. This
            value cannot be changed after creating the subscription.</p>
    name = "value"  # <p>The unique name for your subscription. It must be unique in your Amazon Web Services account in the
            Amazon Web Services Region you're creating the subscription in. The name can contain only alphanumeric
            characters (case-sensitive), hyphens, and underscores. It must start with an alphabetic
            character and can't be longer than 100 characters.</p>
}

# Access eks_anywhere_subscription outputs
eks_anywhere_subscription_id = eks_anywhere_subscription.id
eks_anywhere_subscription_subscription = eks_anywhere_subscription.subscription
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple nodegroup_config resources
nodegroup_config_0 = provider.eks.Nodegroup_config {
    cluster_name = "value-0"
    nodegroup_name = "value-0"
}
nodegroup_config_1 = provider.eks.Nodegroup_config {
    cluster_name = "value-1"
    nodegroup_name = "value-1"
}
nodegroup_config_2 = provider.eks.Nodegroup_config {
    cluster_name = "value-2"
    nodegroup_name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    nodegroup_config = provider.eks.Nodegroup_config {
        cluster_name = "production-value"
        nodegroup_name = "production-value"
    }
```

---

## Related Documentation

- [AWS Eks Documentation](https://docs.aws.amazon.com/eks/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
