# Datasync Service



**Resources**: 15

---

## Overview

The datasync service provides access to 15 resource types:

- [Location_fsx_windows](#location_fsx_windows) [CRU]
- [Location_azure_blob](#location_azure_blob) [CRU]
- [Location_fsx_ontap](#location_fsx_ontap) [CRU]
- [Location_hdfs](#location_hdfs) [CRU]
- [Location_s3](#location_s3) [CRU]
- [Location_smb](#location_smb) [CRU]
- [Task_execution](#task_execution) [RU]
- [Location_efs](#location_efs) [CRU]
- [Location_fsx_lustre](#location_fsx_lustre) [CRU]
- [Agent](#agent) [CRUD]
- [Task](#task) [CRUD]
- [Location](#location) [D]
- [Location_nfs](#location_nfs) [CRU]
- [Location_object_storage](#location_object_storage) [CRU]
- [Location_fsx_open_zfs](#location_fsx_open_zfs) [CRU]

---

## Resources


### Location_fsx_windows

LocationFsxWindows resource

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `subdirectory` | String |  | <p>Specifies a mount path for your file system using forward slashes. This is where DataSync reads or writes data (depending on if this is a source or destination
      location).</p> |
| `fsx_filesystem_arn` | String | ✅ | <p>Specifies the Amazon Resource Name (ARN) for the FSx for Windows File Server file
      system.</p> |
| `tags` | Vec<String> |  | <p>Specifies labels that help you categorize, filter, and search for your Amazon Web Services
      resources. We recommend creating at least a name tag for your location.</p> |
| `domain` | String |  | <p>Specifies the name of the Windows domain that the FSx for Windows File Server file system
      belongs to.</p>
         <p>If you have multiple Active Directory domains in your environment, configuring this
      parameter makes sure that DataSync connects to the right file system.</p> |
| `security_group_arns` | Vec<String> | ✅ | <p>Specifies the ARNs of the Amazon EC2 security groups that provide access to your
      file system's preferred subnet.</p>
         <p>The security groups that you specify must be able to communicate with your file system's
      security groups. For information about configuring security groups for file system access, see
      the <a href="https://docs.aws.amazon.com/fsx/latest/WindowsGuide/limit-access-security-groups.html">
               <i>Amazon FSx for Windows File Server User Guide</i>
            </a>.</p>
         <note>
            <p>If you choose a security group that doesn't allow connections from within itself, do one
        of the following:</p>
            <ul>
               <li>
                  <p>Configure the security group to allow it to communicate within itself.</p>
               </li>
               <li>
                  <p>Choose a different security group that can communicate with the mount target's
            security group.</p>
               </li>
            </ul>
         </note> |
| `user` | String | ✅ | <p>Specifies the user with the permissions to mount and access the files, folders, and file
      metadata in your FSx for Windows File Server file system.</p>
         <p>For information about choosing a user with the right level of access for your transfer,
      see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/create-fsx-location.html#create-fsx-windows-location-permissions">required permissions</a> for FSx for Windows File Server locations.</p> |
| `password` | String | ✅ | <p>Specifies the password of the user with the permissions to mount and access the files,
      folders, and file metadata in your FSx for Windows File Server file system.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `user` | String | <p>The user with the permissions to mount and access the FSx for Windows File Server file
      system.</p> |
| `domain` | String | <p>The name of the Microsoft Active Directory domain that the FSx for Windows File Server file
      system belongs to.</p> |
| `security_group_arns` | Vec<String> | <p>The ARNs of the Amazon EC2 security groups that provide access to your file
      system's preferred subnet.</p>
         <p>For information about configuring security groups for file system access, see the <a href="https://docs.aws.amazon.com/fsx/latest/WindowsGuide/limit-access-security-groups.html">
               <i>Amazon FSx for Windows File Server User Guide</i>
            </a>.</p> |
| `creation_time` | String | <p>The time that the FSx for Windows File Server location was created.</p> |
| `location_arn` | String | <p>The ARN of the FSx for Windows File Server location.</p> |
| `location_uri` | String | <p>The uniform resource identifier (URI) of the FSx for Windows File Server location.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create location_fsx_windows
location_fsx_windows = provider.datasync.Location_fsx_windows {
    fsx_filesystem_arn = "value"  # <p>Specifies the Amazon Resource Name (ARN) for the FSx for Windows File Server file
      system.</p>
    security_group_arns = "value"  # <p>Specifies the ARNs of the Amazon EC2 security groups that provide access to your
      file system's preferred subnet.</p>
         <p>The security groups that you specify must be able to communicate with your file system's
      security groups. For information about configuring security groups for file system access, see
      the <a href="https://docs.aws.amazon.com/fsx/latest/WindowsGuide/limit-access-security-groups.html">
               <i>Amazon FSx for Windows File Server User Guide</i>
            </a>.</p>
         <note>
            <p>If you choose a security group that doesn't allow connections from within itself, do one
        of the following:</p>
            <ul>
               <li>
                  <p>Configure the security group to allow it to communicate within itself.</p>
               </li>
               <li>
                  <p>Choose a different security group that can communicate with the mount target's
            security group.</p>
               </li>
            </ul>
         </note>
    user = "value"  # <p>Specifies the user with the permissions to mount and access the files, folders, and file
      metadata in your FSx for Windows File Server file system.</p>
         <p>For information about choosing a user with the right level of access for your transfer,
      see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/create-fsx-location.html#create-fsx-windows-location-permissions">required permissions</a> for FSx for Windows File Server locations.</p>
    password = "value"  # <p>Specifies the password of the user with the permissions to mount and access the files,
      folders, and file metadata in your FSx for Windows File Server file system.</p>
}

# Access location_fsx_windows outputs
location_fsx_windows_id = location_fsx_windows.id
location_fsx_windows_user = location_fsx_windows.user
location_fsx_windows_domain = location_fsx_windows.domain
location_fsx_windows_security_group_arns = location_fsx_windows.security_group_arns
location_fsx_windows_creation_time = location_fsx_windows.creation_time
location_fsx_windows_location_arn = location_fsx_windows.location_arn
location_fsx_windows_location_uri = location_fsx_windows.location_uri
```

---


### Location_azure_blob

LocationAzureBlob resource

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `agent_arns` | Vec<String> |  | <p>(Optional) Specifies the Amazon Resource Name (ARN) of the DataSync agent that
      can connect with your Azure Blob Storage container. If you are setting up an agentless
      cross-cloud transfer, you do not need to specify a value for this parameter.</p>
         <p>You can specify more than one agent. For more information, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/multiple-agents.html">Using multiple
        agents for your transfer</a>.</p>
         <note>
            <p>Make sure you configure this parameter correctly when you first create your storage
        location. You cannot add or remove agents from a storage location after you create
        it.</p>
         </note> |
| `subdirectory` | String |  | <p>Specifies path segments if you want to limit your transfer to a virtual directory in your
      container (for example, <code>/my/images</code>).</p> |
| `custom_secret_config` | String |  | <p>Specifies configuration information for a customer-managed Secrets Manager secret where
      the authentication token for an AzureBlob storage location is stored in plain text. This
      configuration includes the secret ARN, and the ARN for an IAM role that
      provides access to the secret.</p>
         <note>
            <p>You can use either <code>CmkSecretConfig</code> (with <code>SasConfiguration</code>) or
          <code>CustomSecretConfig</code> (without <code>SasConfiguration</code>) to provide
        credentials for a <code>CreateLocationAzureBlob</code> request. Do not provide both
        parameters for the same request.</p>
         </note> |
| `cmk_secret_config` | String |  | <p>Specifies configuration information for a DataSync-managed secret, which
      includes the authentication token that DataSync uses to access a specific AzureBlob
      storage location, with a customer-managed KMS key.</p>
         <p>When you include this paramater as part of a <code>CreateLocationAzureBlob</code> request,
      you provide only the KMS key ARN. DataSync uses this KMS key together with the authentication token you specify for
        <code>SasConfiguration</code> to create a DataSync-managed secret to store the
      location access credentials.</p>
         <p>Make sure the DataSync has permission to access the KMS key that
      you specify.</p>
         <note>
            <p>You can use either <code>CmkSecretConfig</code> (with <code>SasConfiguration</code>) or
          <code>CustomSecretConfig</code> (without <code>SasConfiguration</code>) to provide
        credentials for a <code>CreateLocationAzureBlob</code> request. Do not provide both
        parameters for the same request.</p>
         </note> |
| `container_url` | String | ✅ | <p>Specifies the URL of the Azure Blob Storage container involved in your transfer.</p> |
| `authentication_type` | String | ✅ | <p>Specifies the authentication method DataSync uses to access your Azure Blob
      Storage. DataSync can access blob storage using a shared access signature
      (SAS).</p> |
| `access_tier` | String |  | <p>Specifies the access tier that you want your objects or files transferred into. This only
      applies when using the location as a transfer destination. For more information, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/creating-azure-blob-location.html#azure-blob-access-tiers">Access tiers</a>.</p> |
| `tags` | Vec<String> |  | <p>Specifies labels that help you categorize, filter, and search for your Amazon Web Services
      resources. We recommend creating at least a name tag for your transfer location.</p> |
| `sas_configuration` | String |  | <p>Specifies the SAS configuration that allows DataSync to access your Azure Blob
      Storage.</p>
         <note>
            <p>If you provide an authentication token using <code>SasConfiguration</code>, but do not
        provide secret configuration details using <code>CmkSecretConfig</code> or
          <code>CustomSecretConfig</code>, then DataSync stores the token using your
          Amazon Web Services account's secrets manager secret.</p>
         </note> |
| `blob_type` | String |  | <p>Specifies the type of blob that you want your objects or files to be when transferring
      them into Azure Blob Storage. Currently, DataSync only supports moving data into
      Azure Blob Storage as block blobs. For more information on blob types, see the <a href="https://learn.microsoft.com/en-us/rest/api/storageservices/understanding-block-blobs--append-blobs--and-page-blobs">Azure Blob Storage documentation</a>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `agent_arns` | Vec<String> | <p>The ARNs of the DataSync agents that can connect with your Azure Blob Storage
      container.</p> |
| `custom_secret_config` | String | <p>Describes configuration information for a customer-managed secret, such as an
      authentication token that DataSync uses to access a specific storage location, with
      a customer-managed KMS key.</p> |
| `access_tier` | String | <p>The access tier that you want your objects or files transferred into. This only applies
      when using the location as a transfer destination. For more information, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/creating-azure-blob-location.html#azure-blob-access-tiers">Access tiers</a>.</p> |
| `authentication_type` | String | <p>The authentication method DataSync uses to access your Azure Blob Storage.
        DataSync can access blob storage using a shared access signature (SAS).</p> |
| `location_arn` | String | <p>The ARN of your Azure Blob Storage transfer location.</p> |
| `creation_time` | String | <p>The time that your Azure Blob Storage transfer location was created.</p> |
| `managed_secret_config` | String | <p>Describes configuration information for a DataSync-managed secret, such as an
      authentication token that DataSync uses to access a specific storage location.
        DataSync uses the default Amazon Web Services-managed KMS key to
      encrypt this secret in Secrets Manager.</p> |
| `cmk_secret_config` | String | <p>Describes configuration information for a DataSync-managed secret, such as an
      authentication token that DataSync uses to access a specific storage location, with
      a customer-managed KMS key.</p> |
| `blob_type` | String | <p>The type of blob that you want your objects or files to be when transferring them into
      Azure Blob Storage. Currently, DataSync only supports moving data into Azure Blob
      Storage as block blobs. For more information on blob types, see the <a href="https://learn.microsoft.com/en-us/rest/api/storageservices/understanding-block-blobs--append-blobs--and-page-blobs">Azure Blob Storage documentation</a>.</p> |
| `location_uri` | String | <p>The URL of the Azure Blob Storage container involved in your transfer.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create location_azure_blob
location_azure_blob = provider.datasync.Location_azure_blob {
    container_url = "value"  # <p>Specifies the URL of the Azure Blob Storage container involved in your transfer.</p>
    authentication_type = "value"  # <p>Specifies the authentication method DataSync uses to access your Azure Blob
      Storage. DataSync can access blob storage using a shared access signature
      (SAS).</p>
}

# Access location_azure_blob outputs
location_azure_blob_id = location_azure_blob.id
location_azure_blob_agent_arns = location_azure_blob.agent_arns
location_azure_blob_custom_secret_config = location_azure_blob.custom_secret_config
location_azure_blob_access_tier = location_azure_blob.access_tier
location_azure_blob_authentication_type = location_azure_blob.authentication_type
location_azure_blob_location_arn = location_azure_blob.location_arn
location_azure_blob_creation_time = location_azure_blob.creation_time
location_azure_blob_managed_secret_config = location_azure_blob.managed_secret_config
location_azure_blob_cmk_secret_config = location_azure_blob.cmk_secret_config
location_azure_blob_blob_type = location_azure_blob.blob_type
location_azure_blob_location_uri = location_azure_blob.location_uri
```

---


### Location_fsx_ontap

LocationFsxOntap resource

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>Specifies labels that help you categorize, filter, and search for your Amazon Web Services
      resources. We recommend creating at least a name tag for your location.</p> |
| `protocol` | String | ✅ |  |
| `security_group_arns` | Vec<String> | ✅ | <p>Specifies the Amazon EC2 security groups that provide access to your file system's
      preferred subnet.</p>
         <p>The security groups must allow outbound traffic on the following ports (depending on the
      protocol you use):</p>
         <ul>
            <li>
               <p>
                  <b>Network File System (NFS)</b>: TCP ports 111, 635, and
          2049</p>
            </li>
            <li>
               <p>
                  <b>Server Message Block (SMB)</b>: TCP port 445</p>
            </li>
         </ul>
         <p>Your file system's security groups must also allow inbound traffic on the same
      ports.</p> |
| `subdirectory` | String |  | <p>Specifies a path to the file share in the SVM where you want to transfer data to or
      from.</p>
         <p>You can specify a junction path (also known as a mount point), qtree path (for NFS file
      shares), or share name (for SMB file shares). For example, your mount path might be
        <code>/vol1</code>, <code>/vol1/tree1</code>, or <code>/share1</code>.</p>
         <note>
            <p>Don't specify a junction path in the SVM's root volume. For more information, see <a href="https://docs.aws.amazon.com/fsx/latest/ONTAPGuide/managing-svms.html">Managing FSx for ONTAP storage virtual machines</a> in the <i>Amazon FSx for NetApp ONTAP User Guide</i>.</p>
         </note> |
| `storage_virtual_machine_arn` | String | ✅ | <p>Specifies the ARN of the storage virtual machine (SVM) in your file system where you want
      to copy data to or from.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `creation_time` | String | <p>The time that the location was created.</p> |
| `location_uri` | String | <p>The uniform resource identifier (URI) of the FSx for ONTAP file system
      location.</p> |
| `location_arn` | String | <p>The ARN of the FSx for ONTAP file system location.</p> |
| `storage_virtual_machine_arn` | String | <p>The ARN of the storage virtual machine (SVM) on your FSx for ONTAP file system
      where you're copying data to or from.</p> |
| `fsx_filesystem_arn` | String | <p>The ARN of the FSx for ONTAP file system.</p> |
| `protocol` | String |  |
| `security_group_arns` | Vec<String> | <p>The security groups that DataSync uses to access your FSx for ONTAP
      file system.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create location_fsx_ontap
location_fsx_ontap = provider.datasync.Location_fsx_ontap {
    protocol = "value"  # Required field
    security_group_arns = "value"  # <p>Specifies the Amazon EC2 security groups that provide access to your file system's
      preferred subnet.</p>
         <p>The security groups must allow outbound traffic on the following ports (depending on the
      protocol you use):</p>
         <ul>
            <li>
               <p>
                  <b>Network File System (NFS)</b>: TCP ports 111, 635, and
          2049</p>
            </li>
            <li>
               <p>
                  <b>Server Message Block (SMB)</b>: TCP port 445</p>
            </li>
         </ul>
         <p>Your file system's security groups must also allow inbound traffic on the same
      ports.</p>
    storage_virtual_machine_arn = "value"  # <p>Specifies the ARN of the storage virtual machine (SVM) in your file system where you want
      to copy data to or from.</p>
}

# Access location_fsx_ontap outputs
location_fsx_ontap_id = location_fsx_ontap.id
location_fsx_ontap_creation_time = location_fsx_ontap.creation_time
location_fsx_ontap_location_uri = location_fsx_ontap.location_uri
location_fsx_ontap_location_arn = location_fsx_ontap.location_arn
location_fsx_ontap_storage_virtual_machine_arn = location_fsx_ontap.storage_virtual_machine_arn
location_fsx_ontap_fsx_filesystem_arn = location_fsx_ontap.fsx_filesystem_arn
location_fsx_ontap_protocol = location_fsx_ontap.protocol
location_fsx_ontap_security_group_arns = location_fsx_ontap.security_group_arns
```

---


### Location_hdfs

LocationHdfs resource

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | Vec<String> |  | <p>The key-value pair that represents the tag that you want to add to the location. The value
      can be an empty string. We recommend using tags to name your resources. </p> |
| `replication_factor` | i64 |  | <p>The number of DataNodes to replicate the data to when writing to the HDFS cluster. By
      default, data is replicated to three DataNodes.</p> |
| `subdirectory` | String |  | <p>A subdirectory in the HDFS cluster. This subdirectory is used to read data from or write
      data to the HDFS cluster. If the subdirectory isn't specified, it will default to
        <code>/</code>.</p> |
| `kerberos_principal` | String |  | <p>The Kerberos principal with access to the files and folders on the HDFS cluster. </p>
         <note>
            <p>If <code>KERBEROS</code> is specified for <code>AuthenticationType</code>, this
        parameter is required.</p>
         </note> |
| `kerberos_krb5_conf` | String |  | <p>The <code>krb5.conf</code> file that contains the Kerberos configuration information. You
      can load the <code>krb5.conf</code> file by providing the file's address. If you're using the
        CLI, it performs the base64 encoding for you. Otherwise, provide the
      base64-encoded text. </p>
         <note>
            <p>If <code>KERBEROS</code> is specified for <code>AuthenticationType</code>, this
        parameter is required.</p>
         </note> |
| `agent_arns` | Vec<String> | ✅ | <p>The Amazon Resource Names (ARNs) of the DataSync agents that can connect to your
      HDFS cluster.</p> |
| `authentication_type` | String | ✅ | <p>The type of authentication used to determine the identity of the user. </p> |
| `simple_user` | String |  | <p>The user name used to identify the client on the host operating system. </p>
         <note>
            <p>If <code>SIMPLE</code> is specified for <code>AuthenticationType</code>, this parameter
        is required. </p>
         </note> |
| `kms_key_provider_uri` | String |  | <p>The URI of the HDFS cluster's Key Management Server (KMS). </p> |
| `kerberos_keytab` | String |  | <p>The Kerberos key table (keytab) that contains mappings between the defined Kerberos
      principal and the encrypted keys. You can load the keytab from a file by providing the file's
      address.</p>
         <note>
            <p>If <code>KERBEROS</code> is specified for <code>AuthenticationType</code>, this
        parameter is required. </p>
         </note> |
| `name_nodes` | Vec<String> | ✅ | <p>The NameNode that manages the HDFS namespace. The NameNode performs operations such as
      opening, closing, and renaming files and directories. The NameNode contains the information to
      map blocks of data to the DataNodes. You can use only one NameNode.</p> |
| `block_size` | i64 |  | <p>The size of data blocks to write into the HDFS cluster. The block size must be a multiple
      of 512 bytes. The default block size is 128 mebibytes (MiB).</p> |
| `qop_configuration` | String |  | <p>The Quality of Protection (QOP) configuration specifies the Remote Procedure Call (RPC)
      and data transfer protection settings configured on the Hadoop Distributed File System (HDFS)
      cluster. If <code>QopConfiguration</code> isn't specified, <code>RpcProtection</code> and
        <code>DataTransferProtection</code> default to <code>PRIVACY</code>. If you set
        <code>RpcProtection</code> or <code>DataTransferProtection</code>, the other parameter
      assumes the same value. </p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `location_arn` | String | <p>The ARN of the HDFS location.</p> |
| `replication_factor` | i64 | <p>The number of DataNodes to replicate the data to when writing to the HDFS cluster. </p> |
| `authentication_type` | String | <p>The type of authentication used to determine the identity of the user. </p> |
| `creation_time` | String | <p>The time that the HDFS location was created.</p> |
| `block_size` | i64 | <p>The size of the data blocks to write into the HDFS cluster. </p> |
| `simple_user` | String | <p>The user name to identify the client on the host operating system. This parameter is used
      if the <code>AuthenticationType</code> is defined as <code>SIMPLE</code>.</p> |
| `agent_arns` | Vec<String> | <p>The ARNs of the DataSync agents that can connect with your HDFS cluster.</p> |
| `location_uri` | String | <p>The URI of the HDFS location.</p> |
| `name_nodes` | Vec<String> | <p>The NameNode that manages the HDFS namespace. </p> |
| `kerberos_principal` | String | <p>The Kerberos principal with access to the files and folders on the HDFS cluster. This
      parameter is used if the <code>AuthenticationType</code> is defined as
      <code>KERBEROS</code>.</p> |
| `kms_key_provider_uri` | String | <p> The URI of the HDFS cluster's Key Management Server (KMS). </p> |
| `qop_configuration` | String | <p>The Quality of Protection (QOP) configuration, which specifies the Remote Procedure Call
      (RPC) and data transfer protection settings configured on the HDFS cluster. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create location_hdfs
location_hdfs = provider.datasync.Location_hdfs {
    agent_arns = "value"  # <p>The Amazon Resource Names (ARNs) of the DataSync agents that can connect to your
      HDFS cluster.</p>
    authentication_type = "value"  # <p>The type of authentication used to determine the identity of the user. </p>
    name_nodes = "value"  # <p>The NameNode that manages the HDFS namespace. The NameNode performs operations such as
      opening, closing, and renaming files and directories. The NameNode contains the information to
      map blocks of data to the DataNodes. You can use only one NameNode.</p>
}

# Access location_hdfs outputs
location_hdfs_id = location_hdfs.id
location_hdfs_location_arn = location_hdfs.location_arn
location_hdfs_replication_factor = location_hdfs.replication_factor
location_hdfs_authentication_type = location_hdfs.authentication_type
location_hdfs_creation_time = location_hdfs.creation_time
location_hdfs_block_size = location_hdfs.block_size
location_hdfs_simple_user = location_hdfs.simple_user
location_hdfs_agent_arns = location_hdfs.agent_arns
location_hdfs_location_uri = location_hdfs.location_uri
location_hdfs_name_nodes = location_hdfs.name_nodes
location_hdfs_kerberos_principal = location_hdfs.kerberos_principal
location_hdfs_kms_key_provider_uri = location_hdfs.kms_key_provider_uri
location_hdfs_qop_configuration = location_hdfs.qop_configuration
```

---


### Location_s3

LocationS3 resource

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `s3_config` | String | ✅ |  |
| `tags` | Vec<String> |  | <p>Specifies labels that help you categorize, filter, and search for your Amazon Web Services resources. We recommend creating at least a name tag for your transfer location.</p> |
| `subdirectory` | String |  | <p>Specifies a prefix in the S3 bucket that DataSync reads from or writes to
      (depending on whether the bucket is a source or destination location).</p>
         <note>
            <p>DataSync can't transfer objects with a prefix that begins with a slash (<code>/</code>)
        or includes <code>//</code>, <code>/./</code>, or <code>/../</code> patterns. For
        example:</p>
            <ul>
               <li>
                  <p>
                     <code>/photos</code>
                  </p>
               </li>
               <li>
                  <p>
                     <code>photos//2006/January</code>
                  </p>
               </li>
               <li>
                  <p>
                     <code>photos/./2006/February</code>
                  </p>
               </li>
               <li>
                  <p>
                     <code>photos/../2006/March</code>
                  </p>
               </li>
            </ul>
         </note> |
| `s3_bucket_arn` | String | ✅ | <p>Specifies the ARN of the S3 bucket that you want to use as a location. (When creating
      your DataSync task later, you specify whether this location is a transfer source or
      destination.) </p>
         <p>If your S3 bucket is located on an Outposts resource, you must specify an
        Amazon S3 access point. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/access-points.html">Managing data access
        with Amazon S3 access points</a> in the <i>Amazon S3 User
        Guide</i>.</p> |
| `agent_arns` | Vec<String> |  | <p>(Amazon S3 on Outposts only) Specifies the Amazon Resource Name (ARN) of the
        DataSync agent on your Outpost.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/deploy-agents.html#outposts-agent">Deploy your DataSync agent on Outposts</a>.</p> |
| `s3_storage_class` | String |  | <p>Specifies the storage class that you want your objects to use when Amazon S3 is a
      transfer destination.</p>
         <p>For buckets in Amazon Web Services Regions, the storage class defaults to
        <code>STANDARD</code>. For buckets on Outposts, the storage class defaults to
        <code>OUTPOSTS</code>.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/create-s3-location.html#using-storage-classes">Storage class
        considerations with Amazon S3 transfers</a>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `s3_storage_class` | String | <p>When Amazon S3 is a destination location, this is the storage class that you chose
      for your objects.</p>
         <p>Some storage classes have behaviors that can affect your Amazon S3 storage costs.
      For more information, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/create-s3-location.html#using-storage-classes">Storage class
        considerations with Amazon S3 transfers</a>.</p> |
| `s3_config` | String |  |
| `creation_time` | String | <p>The time that the Amazon S3 location was created.</p> |
| `location_uri` | String | <p>The URL of the Amazon S3 location that was described.</p> |
| `location_arn` | String | <p>The ARN of the Amazon S3 location.</p> |
| `agent_arns` | Vec<String> | <p>The ARNs of the DataSync agents deployed on your Outpost when using working with
        Amazon S3 on Outposts.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/deploy-agents.html#outposts-agent">Deploy your DataSync agent
        on Outposts</a>.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create location_s3
location_s3 = provider.datasync.Location_s3 {
    s3_config = "value"  # Required field
    s3_bucket_arn = "value"  # <p>Specifies the ARN of the S3 bucket that you want to use as a location. (When creating
      your DataSync task later, you specify whether this location is a transfer source or
      destination.) </p>
         <p>If your S3 bucket is located on an Outposts resource, you must specify an
        Amazon S3 access point. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/access-points.html">Managing data access
        with Amazon S3 access points</a> in the <i>Amazon S3 User
        Guide</i>.</p>
}

# Access location_s3 outputs
location_s3_id = location_s3.id
location_s3_s3_storage_class = location_s3.s3_storage_class
location_s3_s3_config = location_s3.s3_config
location_s3_creation_time = location_s3.creation_time
location_s3_location_uri = location_s3.location_uri
location_s3_location_arn = location_s3.location_arn
location_s3_agent_arns = location_s3.agent_arns
```

---


### Location_smb

LocationSmb resource

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `authentication_type` | String |  | <p>Specifies the authentication protocol that DataSync uses to connect to your SMB
      file server. DataSync supports <code>NTLM</code> (default) and <code>KERBEROS</code>
      authentication.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/create-smb-location.html#configuring-smb-permissions">Providing DataSync access to SMB file servers</a>.</p> |
| `kerberos_keytab` | String |  | <p>Specifies your Kerberos key table (keytab) file, which includes mappings between your
      Kerberos principal and encryption keys.</p>
         <p>To avoid task execution errors, make sure that the Kerberos principal that you use to
      create the keytab file matches exactly what you specify for <code>KerberosPrincipal</code>.
    </p> |
| `tags` | Vec<String> |  | <p>Specifies labels that help you categorize, filter, and search for your Amazon Web Services
      resources. We recommend creating at least a name tag for your location.</p> |
| `user` | String |  | <p>Specifies the user that can mount and access the files, folders, and file metadata in your
      SMB file server. This parameter applies only if <code>AuthenticationType</code> is set to
        <code>NTLM</code>.</p>
         <p>For information about choosing a user with the right level of access for your transfer,
      see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/create-smb-location.html#configuring-smb-permissions">Providing DataSync access to SMB file servers</a>.</p> |
| `server_hostname` | String | ✅ | <p>Specifies the domain name or IP address (IPv4 or IPv6) of the SMB file server that your DataSync agent connects to.</p>
         <note>
            <p>If you're using Kerberos authentication, you must specify a domain name.</p>
         </note> |
| `agent_arns` | Vec<String> | ✅ | <p>Specifies the DataSync agent (or agents) that can connect to your SMB file
      server. You specify an agent by using its Amazon Resource Name (ARN).</p> |
| `mount_options` | String |  | <p>Specifies the version of the SMB protocol that DataSync uses to access your SMB
      file server.</p> |
| `kerberos_krb5_conf` | String |  | <p>Specifies a Kerberos configuration file (<code>krb5.conf</code>) that defines your
      Kerberos realm configuration.</p>
         <p>The file must be base64 encoded. If you're using the CLI, the encoding is
      done for you.</p> |
| `password` | String |  | <p>Specifies the password of the user who can mount your SMB file server and has permission
      to access the files and folders involved in your transfer. This parameter applies only if
        <code>AuthenticationType</code> is set to <code>NTLM</code>.</p> |
| `dns_ip_addresses` | Vec<String> |  | <p>Specifies the IPv4 or IPv6 addresses for the DNS servers that your SMB file server belongs to.
      This parameter applies only if <code>AuthenticationType</code> is set to
      <code>KERBEROS</code>.</p>
         <p>If you have multiple domains in your environment, configuring this parameter makes sure
      that DataSync connects to the right SMB file server.</p> |
| `subdirectory` | String | ✅ | <p>Specifies the name of the share exported by your SMB file server where DataSync
      will read or write data. You can include a subdirectory in the share path (for example,
        <code>/path/to/subdirectory</code>). Make sure that other SMB clients in your network can
      also mount this path.</p>
         <p>To copy all data in the subdirectory, DataSync must be able to mount the SMB
      share and access all of its data. For more information, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/create-smb-location.html#configuring-smb-permissions">Providing DataSync access to SMB file servers</a>.</p> |
| `domain` | String |  | <p>Specifies the Windows domain name that your SMB file server belongs to. This parameter
      applies only if <code>AuthenticationType</code> is set to <code>NTLM</code>.</p>
         <p>If you have multiple domains in your environment, configuring this parameter makes sure
      that DataSync connects to the right file server.</p> |
| `kerberos_principal` | String |  | <p>Specifies a Kerberos principal, which is an identity in your Kerberos realm that has
      permission to access the files, folders, and file metadata in your SMB file server.</p>
         <p>A Kerberos principal might look like <code>HOST/kerberosuser@MYDOMAIN.ORG</code>.</p>
         <p>Principal names are case sensitive. Your DataSync task execution will fail if
      the principal that you specify for this parameter doesn’t exactly match the principal that you
      use to create the keytab file.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `location_uri` | String | <p>The URI of the SMB location.</p> |
| `dns_ip_addresses` | Vec<String> | <p>The IPv4 or IPv6 addresses for the DNS servers that your SMB file server belongs to. This element
      applies only if <code>AuthenticationType</code> is set to <code>KERBEROS</code>.</p> |
| `agent_arns` | Vec<String> | <p>The ARNs of the DataSync agents that can connect with your SMB file
      server.</p> |
| `mount_options` | String | <p>The SMB protocol version that DataSync uses to access your SMB file
      server.</p> |
| `user` | String | <p>The user that can mount and access the files, folders, and file metadata in your SMB file
      server. This element applies only if <code>AuthenticationType</code> is set to
        <code>NTLM</code>.</p> |
| `domain` | String | <p>The name of the Windows domain that the SMB file server belongs to. This element applies
      only if <code>AuthenticationType</code> is set to <code>NTLM</code>.</p> |
| `authentication_type` | String | <p>The authentication protocol that DataSync uses to connect to your SMB file
      server.</p> |
| `kerberos_principal` | String | <p>The Kerberos principal that has permission to access the files, folders, and file metadata
      in your SMB file server.</p> |
| `location_arn` | String | <p>The ARN of the SMB location.</p> |
| `creation_time` | String | <p>The time that the SMB location was created.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create location_smb
location_smb = provider.datasync.Location_smb {
    server_hostname = "value"  # <p>Specifies the domain name or IP address (IPv4 or IPv6) of the SMB file server that your DataSync agent connects to.</p>
         <note>
            <p>If you're using Kerberos authentication, you must specify a domain name.</p>
         </note>
    agent_arns = "value"  # <p>Specifies the DataSync agent (or agents) that can connect to your SMB file
      server. You specify an agent by using its Amazon Resource Name (ARN).</p>
    subdirectory = "value"  # <p>Specifies the name of the share exported by your SMB file server where DataSync
      will read or write data. You can include a subdirectory in the share path (for example,
        <code>/path/to/subdirectory</code>). Make sure that other SMB clients in your network can
      also mount this path.</p>
         <p>To copy all data in the subdirectory, DataSync must be able to mount the SMB
      share and access all of its data. For more information, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/create-smb-location.html#configuring-smb-permissions">Providing DataSync access to SMB file servers</a>.</p>
}

# Access location_smb outputs
location_smb_id = location_smb.id
location_smb_location_uri = location_smb.location_uri
location_smb_dns_ip_addresses = location_smb.dns_ip_addresses
location_smb_agent_arns = location_smb.agent_arns
location_smb_mount_options = location_smb.mount_options
location_smb_user = location_smb.user
location_smb_domain = location_smb.domain
location_smb_authentication_type = location_smb.authentication_type
location_smb_kerberos_principal = location_smb.kerberos_principal
location_smb_location_arn = location_smb.location_arn
location_smb_creation_time = location_smb.creation_time
```

---


### Task_execution

TaskExecution resource

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `options` | String | ✅ |  |
| `task_execution_arn` | String | ✅ | <p>Specifies the Amazon Resource Name (ARN) of the task execution that you're
      updating.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `task_mode` | String | <p>The task mode that you're using. For more information, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/choosing-task-mode.html">Choosing a task mode for your data
        transfer</a>.</p> |
| `files_prepared` | i64 | <p>The number of objects that DataSync will attempt to transfer after comparing
      your source and destination locations.</p>
         <note>
            <p>Applies only to <a href="https://docs.aws.amazon.com/datasync/latest/userguide/choosing-task-mode.html">Enhanced mode
        tasks</a>.</p>
         </note>
         <p>This counter isn't applicable if you configure your task to <a href="https://docs.aws.amazon.com/datasync/latest/userguide/configure-metadata.html#task-option-transfer-mode">transfer
        all data</a>. In that scenario, DataSync copies everything from the source to
      the destination without comparing differences between the locations.</p> |
| `bytes_written` | i64 | <p>The number of logical bytes that DataSync actually writes to the destination
      location.</p> |
| `task_report_config` | String | <p>The configuration of your task report, which provides detailed information about for your
        DataSync transfer. For more information, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/task-reports.html">Creating a task report</a>.</p> |
| `launch_time` | String | <p>The time that the task execution actually begins. For non-queued tasks,
        <code>LaunchTime</code> and <code>StartTime</code> are typically the same. For queued tasks,
        <code>LaunchTime</code> is typically later than <code>StartTime</code> because previously
      queued tasks must finish running before newer tasks can begin.</p> |
| `files_skipped` | i64 | <p>The number of files, objects, and directories that DataSync skips during your
      transfer.</p> |
| `files_verified` | i64 | <p>The number of files, objects, and directories that DataSync verifies during your
      transfer.</p>
         <note>
            <p>When you configure your task to <a href="https://docs.aws.amazon.com/datasync/latest/userguide/configure-data-verification-options.html">verify only the
          data that's transferred</a>, DataSync doesn't verify directories in some
        situations or files that fail to transfer.</p>
         </note> |
| `options` | String |  |
| `files_failed` | String | <p>The number of objects that DataSync fails to prepare, transfer, verify, and
      delete during your task execution.</p>
         <note>
            <p>Applies only to <a href="https://docs.aws.amazon.com/datasync/latest/userguide/choosing-task-mode.html">Enhanced mode
        tasks</a>.</p>
         </note> |
| `includes` | Vec<String> | <p>A list of filter rules that include specific data during your transfer. For more
      information and examples, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/filtering.html">Filtering data transferred by DataSync</a>.</p> |
| `estimated_files_to_transfer` | i64 | <p>The number of files, objects, and directories that DataSync expects to
      transfer over the network. This value is calculated while DataSync
      <a href="https://docs.aws.amazon.com/datasync/latest/userguide/run-task.html#understand-task-execution-statuses">prepares</a> the transfer.</p>
         <p>How this gets calculated depends primarily on your task’s <a href="https://docs.aws.amazon.com/datasync/latest/userguide/API_Options.html#DataSync-Type-Options-TransferMode">transfer
        mode</a> configuration:</p>
         <ul>
            <li>
               <p>If <code>TranserMode</code> is set to <code>CHANGED</code> - The calculation is based
          on comparing the content of the source and destination locations and determining the
          difference that needs to be transferred. The difference can include:</p>
               <ul>
                  <li>
                     <p>Anything that's added or modified at the source location.</p>
                  </li>
                  <li>
                     <p>Anything that's in both locations and modified at the destination after an initial
              transfer (unless <a href="https://docs.aws.amazon.com/datasync/latest/userguide/API_Options.html#DataSync-Type-Options-OverwriteMode">OverwriteMode</a> is set to <code>NEVER</code>).</p>
                  </li>
                  <li>
                     <p>
                        <b>(Basic task mode only)</b> The number of items that
                DataSync expects to delete (if <a href="https://docs.aws.amazon.com/datasync/latest/userguide/API_Options.html#DataSync-Type-Options-PreserveDeletedFiles">PreserveDeletedFiles</a> is set to
              <code>REMOVE</code>).</p>
                  </li>
               </ul>
            </li>
            <li>
               <p>If <code>TranserMode</code> is set to <code>ALL</code> - The calculation is based only
          on the items that DataSync finds at the source location.</p>
            </li>
         </ul> |
| `estimated_bytes_to_transfer` | i64 | <p>The number of logical bytes that DataSync expects to write to the destination
      location.</p> |
| `result` | String | <p>The result of the task execution.</p> |
| `files_deleted` | i64 | <p>The number of files, objects, and directories that DataSync actually deletes in
      your destination location. If you don't configure your task to <a href="https://docs.aws.amazon.com/datasync/latest/userguide/configure-metadata.html">delete data in the destination that
        isn't in the source</a>, the value is always <code>0</code>.</p> |
| `report_result` | String | <p>Indicates whether DataSync generated a complete <a href="https://docs.aws.amazon.com/datasync/latest/userguide/task-reports.html">task report</a> for your
      transfer.</p> |
| `end_time` | String | <p>The time that the transfer task ends.</p> |
| `start_time` | String | <p>The time that DataSync sends the request to start the task execution. For
      non-queued tasks, <code>LaunchTime</code> and <code>StartTime</code> are typically the same.
      For queued tasks, <code>LaunchTime</code> is typically later than <code>StartTime</code>
      because previously queued tasks must finish running before newer tasks can begin.</p> |
| `files_listed` | String | <p>The number of
      objects
      that DataSync
      finds
      at your locations.</p>
         <note>
            <p>Applies only to <a href="https://docs.aws.amazon.com/datasync/latest/userguide/choosing-task-mode.html">Enhanced mode
        tasks</a>.</p>
         </note> |
| `excludes` | Vec<String> | <p>A list of filter rules that exclude specific data during your transfer. For more
      information and examples, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/filtering.html">Filtering data transferred by DataSync</a>.</p> |
| `files_transferred` | i64 | <p>The number of files, objects, and directories that DataSync actually
      transfers over the network. This value is updated periodically during your task execution when
      something is read from the source and sent over the network.</p>
         <p>If DataSync fails to transfer something, this value can be less than
        <code>EstimatedFilesToTransfer</code>. In some cases, this value can also be greater than
        <code>EstimatedFilesToTransfer</code>. This element is implementation-specific for some
      location types, so don't use it as an exact indication of what's transferring or to monitor
      your task execution.</p> |
| `status` | String | <p>The status of the task execution. </p> |
| `estimated_files_to_delete` | i64 | <p>The number of files, objects, and directories that DataSync expects to delete in
      your destination location. If you don't configure your task to <a href="https://docs.aws.amazon.com/datasync/latest/userguide/configure-metadata.html">delete data in the destination that
        isn't in the source</a>, the value is always <code>0</code>.</p> |
| `manifest_config` | String | <p>The configuration of the manifest that lists the files or objects to transfer. For more
      information, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/transferring-with-manifest.html">Specifying what DataSync transfers by using a manifest</a>.</p> |
| `bytes_transferred` | i64 | <p>The number of bytes that DataSync sends to the network before compression (if
      compression is possible). For the number of bytes transferred over the network, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/API_DescribeTaskExecution.html#DataSync-DescribeTaskExecution-response-BytesCompressed">BytesCompressed</a>. </p> |
| `task_execution_arn` | String | <p>The ARN of the task execution that you wanted information about.
        <code>TaskExecutionArn</code> is hierarchical and includes <code>TaskArn</code> for the task
      that was executed. </p>
         <p>For example, a <code>TaskExecution</code> value with the ARN
        <code>arn:aws:datasync:us-east-1:111222333444:task/task-0208075f79cedf4a2/execution/exec-08ef1e88ec491019b</code>
      executed the task with the ARN
        <code>arn:aws:datasync:us-east-1:111222333444:task/task-0208075f79cedf4a2</code>. </p> |
| `bytes_compressed` | i64 | <p>The number of physical bytes that DataSync transfers over the network after
      compression (if compression is possible). This number is typically less than <a href="https://docs.aws.amazon.com/datasync/latest/userguide/API_DescribeTaskExecution.html#DataSync-DescribeTaskExecution-response-BytesTransferred">BytesTransferred</a> unless the data isn't compressible.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access task_execution outputs
task_execution_id = task_execution.id
task_execution_task_mode = task_execution.task_mode
task_execution_files_prepared = task_execution.files_prepared
task_execution_bytes_written = task_execution.bytes_written
task_execution_task_report_config = task_execution.task_report_config
task_execution_launch_time = task_execution.launch_time
task_execution_files_skipped = task_execution.files_skipped
task_execution_files_verified = task_execution.files_verified
task_execution_options = task_execution.options
task_execution_files_failed = task_execution.files_failed
task_execution_includes = task_execution.includes
task_execution_estimated_files_to_transfer = task_execution.estimated_files_to_transfer
task_execution_estimated_bytes_to_transfer = task_execution.estimated_bytes_to_transfer
task_execution_result = task_execution.result
task_execution_files_deleted = task_execution.files_deleted
task_execution_report_result = task_execution.report_result
task_execution_end_time = task_execution.end_time
task_execution_start_time = task_execution.start_time
task_execution_files_listed = task_execution.files_listed
task_execution_excludes = task_execution.excludes
task_execution_files_transferred = task_execution.files_transferred
task_execution_status = task_execution.status
task_execution_estimated_files_to_delete = task_execution.estimated_files_to_delete
task_execution_manifest_config = task_execution.manifest_config
task_execution_bytes_transferred = task_execution.bytes_transferred
task_execution_task_execution_arn = task_execution.task_execution_arn
task_execution_bytes_compressed = task_execution.bytes_compressed
```

---


### Location_efs

LocationEfs resource

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `ec2_config` | String | ✅ | <p>Specifies the subnet and security groups DataSync uses to connect to one of
      your Amazon EFS file system's <a href="https://docs.aws.amazon.com/efs/latest/ug/accessing-fs.html">mount targets</a>.</p> |
| `tags` | Vec<String> |  | <p>Specifies the key-value pair that represents a tag that you want to add to the
      resource. The value can be an empty string. This value helps you manage, filter, and search
      for your resources. We recommend that you create a name tag for your location.</p> |
| `access_point_arn` | String |  | <p>Specifies the Amazon Resource Name (ARN) of the access point that DataSync uses
      to mount your Amazon EFS file system.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/create-efs-location.html#create-efs-location-iam">Accessing
        restricted file systems</a>.</p> |
| `subdirectory` | String |  | <p>Specifies a mount path for your Amazon EFS file system. This is where DataSync reads or writes data on your file system (depending on if this is a source or
      destination location).</p>
         <p>By default, DataSync uses the root directory (or <a href="https://docs.aws.amazon.com/efs/latest/ug/efs-access-points.html">access point</a> if you provide one by using
        <code>AccessPointArn</code>). You can also include subdirectories using forward slashes (for
      example, <code>/path/to/folder</code>).</p> |
| `efs_filesystem_arn` | String | ✅ | <p>Specifies the ARN for your Amazon EFS file system.</p> |
| `file_system_access_role_arn` | String |  | <p>Specifies an Identity and Access Management (IAM) role that allows DataSync to access your Amazon EFS file system.</p>
         <p>For information on creating this role, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/create-efs-location.html#create-efs-location-iam-role">Creating a DataSync
        IAM role for file system access</a>.</p> |
| `in_transit_encryption` | String |  | <p>Specifies whether you want DataSync to use Transport Layer Security (TLS) 1.2
      encryption when it transfers data to or from your Amazon EFS file system.</p>
         <p>If you specify an access point using <code>AccessPointArn</code> or an IAM
      role using <code>FileSystemAccessRoleArn</code>, you must set this parameter to
        <code>TLS1_2</code>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `in_transit_encryption` | String | <p>Indicates whether DataSync uses Transport Layer Security (TLS) encryption when
      transferring data to or from the Amazon EFS file system.</p> |
| `access_point_arn` | String | <p>The ARN of the access point that DataSync uses to access the Amazon EFS
      file system.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/create-efs-location.html#create-efs-location-iam">Accessing
        restricted file systems</a>.</p> |
| `location_uri` | String | <p>The URL of the Amazon EFS file system location.</p> |
| `location_arn` | String | <p>The ARN of the Amazon EFS file system location.</p> |
| `creation_time` | String | <p>The time that the location was created.</p> |
| `file_system_access_role_arn` | String | <p>The Identity and Access Management (IAM) role that allows DataSync to
      access your Amazon EFS file system.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/create-efs-location.html#create-efs-location-iam-role">Creating a DataSync
        IAM role for file system access</a>.</p> |
| `ec2_config` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create location_efs
location_efs = provider.datasync.Location_efs {
    ec2_config = "value"  # <p>Specifies the subnet and security groups DataSync uses to connect to one of
      your Amazon EFS file system's <a href="https://docs.aws.amazon.com/efs/latest/ug/accessing-fs.html">mount targets</a>.</p>
    efs_filesystem_arn = "value"  # <p>Specifies the ARN for your Amazon EFS file system.</p>
}

# Access location_efs outputs
location_efs_id = location_efs.id
location_efs_in_transit_encryption = location_efs.in_transit_encryption
location_efs_access_point_arn = location_efs.access_point_arn
location_efs_location_uri = location_efs.location_uri
location_efs_location_arn = location_efs.location_arn
location_efs_creation_time = location_efs.creation_time
location_efs_file_system_access_role_arn = location_efs.file_system_access_role_arn
location_efs_ec2_config = location_efs.ec2_config
```

---


### Location_fsx_lustre

LocationFsxLustre resource

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `fsx_filesystem_arn` | String | ✅ | <p>Specifies the Amazon Resource Name (ARN) of the FSx for Lustre file
      system.</p> |
| `security_group_arns` | Vec<String> | ✅ | <p>Specifies the Amazon Resource Names (ARNs) of up to five security groups that provide
      access to your FSx for Lustre file system.</p>
         <p>The security groups must be able to access the file system's ports. The file system must
      also allow access from the security groups. For information about file system access, see the
        <a href="https://docs.aws.amazon.com/fsx/latest/LustreGuide/limit-access-security-groups.html">
               <i>Amazon FSx for Lustre User Guide</i>
            </a>.</p> |
| `subdirectory` | String |  | <p>Specifies a mount path for your FSx for Lustre file system. The path can include
      subdirectories.</p>
         <p>When the location is used as a source, DataSync reads data from the mount path.
      When the location is used as a destination, DataSync writes data to the mount path.
      If you don't include this parameter, DataSync uses the file system's root directory
        (<code>/</code>).</p> |
| `tags` | Vec<String> |  | <p>Specifies labels that help you categorize, filter, and search for your Amazon Web Services
      resources. We recommend creating at least a name tag for your location.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `creation_time` | String | <p>The time that the FSx for Lustre location was created.</p> |
| `security_group_arns` | Vec<String> | <p>The Amazon Resource Names (ARNs) of the security groups that are configured for the
        FSx for Lustre file system.</p> |
| `location_uri` | String | <p>The URI of the FSx for Lustre location that was described.</p> |
| `location_arn` | String | <p>The Amazon Resource Name (ARN) of the FSx for Lustre location that was
      described.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create location_fsx_lustre
location_fsx_lustre = provider.datasync.Location_fsx_lustre {
    fsx_filesystem_arn = "value"  # <p>Specifies the Amazon Resource Name (ARN) of the FSx for Lustre file
      system.</p>
    security_group_arns = "value"  # <p>Specifies the Amazon Resource Names (ARNs) of up to five security groups that provide
      access to your FSx for Lustre file system.</p>
         <p>The security groups must be able to access the file system's ports. The file system must
      also allow access from the security groups. For information about file system access, see the
        <a href="https://docs.aws.amazon.com/fsx/latest/LustreGuide/limit-access-security-groups.html">
               <i>Amazon FSx for Lustre User Guide</i>
            </a>.</p>
}

# Access location_fsx_lustre outputs
location_fsx_lustre_id = location_fsx_lustre.id
location_fsx_lustre_creation_time = location_fsx_lustre.creation_time
location_fsx_lustre_security_group_arns = location_fsx_lustre.security_group_arns
location_fsx_lustre_location_uri = location_fsx_lustre.location_uri
location_fsx_lustre_location_arn = location_fsx_lustre.location_arn
```

---


### Agent

Agent resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `activation_key` | String | ✅ | <p>Specifies your DataSync agent's activation key. If you don't have an
      activation key, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/activate-agent.html">Activating your agent</a>.</p> |
| `tags` | Vec<String> |  | <p>Specifies labels that help you categorize, filter, and search for your Amazon Web Services resources. We recommend creating at least one tag for your agent.</p> |
| `vpc_endpoint_id` | String |  | <p>Specifies the ID of the <a href="https://docs.aws.amazon.com/datasync/latest/userguide/choose-service-endpoint.html#datasync-in-vpc">VPC service
        endpoint</a> that you're using. For example, a VPC endpoint ID looks like
        <code>vpce-01234d5aff67890e1</code>.</p>
         <important>
            <p>The VPC service endpoint you use must include the DataSync service name (for
        example, <code>com.amazonaws.us-east-2.datasync</code>).</p>
         </important> |
| `agent_name` | String |  | <p>Specifies a name for your agent. We recommend specifying a name that you can
      remember.</p> |
| `security_group_arns` | Vec<String> |  | <p>Specifies the Amazon Resource Name (ARN) of the security group that allows traffic between
      your agent and VPC service endpoint. You can only specify one ARN.</p> |
| `subnet_arns` | Vec<String> |  | <p>Specifies the ARN of the subnet where your VPC service endpoint is located. You can only
      specify one ARN.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `agent_arn` | String | <p>The ARN of the agent.</p> |
| `last_connection_time` | String | <p>The last time that the agent was communicating with the DataSync
      service.</p> |
| `platform` | String | <p>The platform-related details about the agent, such as the version number.</p> |
| `creation_time` | String | <p>The time that the agent was <a href="https://docs.aws.amazon.com/datasync/latest/userguide/activate-agent.html">activated</a>.</p> |
| `name` | String | <p>The name of the agent.</p> |
| `private_link_config` | String | <p>The network configuration that the agent uses when connecting to a <a href="https://docs.aws.amazon.com/datasync/latest/userguide/choose-service-endpoint.html#choose-service-endpoint-vpc">VPC
        service endpoint</a>.</p> |
| `status` | String | <p>The status of the agent.</p>
         <ul>
            <li>
               <p>If the status is <code>ONLINE</code>, the agent is configured properly and ready to
          use.</p>
            </li>
            <li>
               <p>If the status is <code>OFFLINE</code>, the agent has been out of contact with
            DataSync for five minutes or longer. This can happen for a few reasons. For
          more information, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/troubleshooting-datasync-agents.html#troubleshoot-agent-offline">What do I do if my agent is offline?</a>
               </p>
            </li>
         </ul> |
| `endpoint_type` | String | <p>The type of <a href="https://docs.aws.amazon.com/datasync/latest/userguide/choose-service-endpoint.html">service endpoint</a> that your agent is connected to.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create agent
agent = provider.datasync.Agent {
    activation_key = "value"  # <p>Specifies your DataSync agent's activation key. If you don't have an
      activation key, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/activate-agent.html">Activating your agent</a>.</p>
}

# Access agent outputs
agent_id = agent.id
agent_agent_arn = agent.agent_arn
agent_last_connection_time = agent.last_connection_time
agent_platform = agent.platform
agent_creation_time = agent.creation_time
agent_name = agent.name
agent_private_link_config = agent.private_link_config
agent_status = agent.status
agent_endpoint_type = agent.endpoint_type
```

---


### Task

Task resource

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `excludes` | Vec<String> |  | <p>Specifies exclude filters that define the files, objects, and folders in your source
      location that you don't want DataSync to transfer. For more information and
      examples, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/filtering.html">Specifying what DataSync transfers by using filters</a>.</p> |
| `destination_location_arn` | String | ✅ | <p>Specifies the ARN of your transfer's destination location. </p> |
| `name` | String |  | <p>Specifies the name of your task.</p> |
| `tags` | Vec<String> |  | <p>Specifies the tags that you want to apply to your task.</p>
         <p>
            <i>Tags</i> are key-value pairs that help you manage, filter, and search
      for your DataSync resources.</p> |
| `schedule` | String |  | <p>Specifies a schedule for when you want your task to run. For more information, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/task-scheduling.html">Scheduling your
        task</a>.</p> |
| `manifest_config` | String |  | <p>Configures a manifest, which is a list of files or objects that you want DataSync to transfer. For more information and configuration examples, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/transferring-with-manifest.html">Specifying what DataSync transfers by using a manifest</a>.</p>
         <p>When using this parameter, your caller identity (the role that you're using DataSync with) must have the <code>iam:PassRole</code> permission. The <a href="https://docs.aws.amazon.com/datasync/latest/userguide/security-iam-awsmanpol.html#security-iam-awsmanpol-awsdatasyncfullaccess">AWSDataSyncFullAccess</a> policy includes this permission.</p> |
| `options` | String |  | <p>Specifies your task's settings, such as preserving file metadata, verifying data
      integrity, among other options.</p> |
| `includes` | Vec<String> |  | <p>Specifies include filters that define the files, objects, and folders in your source
      location that you want DataSync to transfer. For more information and examples, see
        <a href="https://docs.aws.amazon.com/datasync/latest/userguide/filtering.html">Specifying what
          DataSync transfers by using filters</a>.</p> |
| `task_mode` | String |  | <p>Specifies one of the following task modes for your data transfer:</p>
         <ul>
            <li>
               <p>
                  <code>ENHANCED</code> - Transfer virtually unlimited numbers of objects with higher
          performance than Basic mode. Enhanced mode tasks optimize the data transfer process by
          listing, preparing, transferring, and verifying data in parallel. Enhanced mode is
          currently available for transfers between Amazon S3 locations, transfers between
          Azure Blob and Amazon S3 without an agent, and transfers between other clouds and
            Amazon S3 without an agent.</p>
               <note>
                  <p>To create an Enhanced mode task, the IAM role that you use to call
            the <code>CreateTask</code> operation must have the
              <code>iam:CreateServiceLinkedRole</code> permission.</p>
               </note>
            </li>
            <li>
               <p>
                  <code>BASIC</code> (default) - Transfer files or objects between Amazon Web Services
          storage and all other supported DataSync locations. Basic mode tasks are subject
          to <a href="https://docs.aws.amazon.com/datasync/latest/userguide/datasync-limits.html">quotas</a> on the number of files, objects, and directories in a dataset. Basic
          mode sequentially prepares, transfers, and verifies data, making it slower than Enhanced
          mode for most workloads.</p>
            </li>
         </ul>
         <p>For more information, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/choosing-task-mode.html#task-mode-differences">Understanding
        task mode differences</a>.</p> |
| `source_location_arn` | String | ✅ | <p>Specifies the ARN of your transfer's source location.</p> |
| `cloud_watch_log_group_arn` | String |  | <p>Specifies the Amazon Resource Name (ARN) of an Amazon CloudWatch log group for
      monitoring your task.</p>
         <p>For Enhanced mode tasks, you don't need to specify anything. DataSync
      automatically sends logs to a CloudWatch log group named
      <code>/aws/datasync</code>.</p> |
| `task_report_config` | String |  | <p>Specifies how you want to configure a task report, which provides detailed information
      about your DataSync transfer. For more information, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/task-reports.html">Monitoring your DataSync
        transfers with task reports</a>.</p>
         <p>When using this parameter, your caller identity (the role that you're using DataSync with) must have the <code>iam:PassRole</code> permission. The <a href="https://docs.aws.amazon.com/datasync/latest/userguide/security-iam-awsmanpol.html#security-iam-awsmanpol-awsdatasyncfullaccess">AWSDataSyncFullAccess</a> policy includes this permission.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `excludes` | Vec<String> | <p>The exclude filters that define the files, objects, and folders in your source location
      that you don't want DataSync to transfer. For more information and examples, see
        <a href="https://docs.aws.amazon.com/datasync/latest/userguide/filtering.html">Specifying what
          DataSync transfers by using filters</a>.</p> |
| `source_location_arn` | String | <p>The ARN of your transfer's source location.</p> |
| `cloud_watch_log_group_arn` | String | <p>The Amazon Resource Name (ARN) of an Amazon CloudWatch log group for monitoring your
      task.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/configure-logging.html">Monitoring data transfers with
          CloudWatch Logs</a>.</p> |
| `error_detail` | String | <p>If there's an issue with your task, you can use the error details to help you
      troubleshoot the problem. For more information, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/troubleshooting-datasync-locations-tasks.html">Troubleshooting issues with DataSync transfers</a>.</p> |
| `destination_location_arn` | String | <p>The ARN of your transfer's destination location.</p> |
| `destination_network_interface_arns` | Vec<String> | <p>The ARNs of the <a href="https://docs.aws.amazon.com/datasync/latest/userguide/datasync-network.html#required-network-interfaces">network
        interfaces</a> that DataSync created for your destination location.</p> |
| `status` | String | <p>The status of your task. For information about what each status means, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/understand-task-statuses.html#understand-task-creation-statuses">Task statuses</a>.</p> |
| `current_task_execution_arn` | String | <p>The ARN of the most recent task execution.</p> |
| `options` | String | <p>The task's settings. For example, what file metadata gets preserved, how data integrity
      gets verified at the end of your transfer, bandwidth limits, among other options.</p> |
| `schedule` | String | <p>The schedule for when you want your task to run. For more information, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/task-scheduling.html">Scheduling your
        task</a>.</p> |
| `name` | String | <p>The name of your task.</p> |
| `manifest_config` | String | <p>The configuration of the manifest that lists the files or objects that you want DataSync to transfer. For more information, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/transferring-with-manifest.html">Specifying what DataSync transfers by using a manifest</a>.</p> |
| `task_report_config` | String | <p>The configuration of your task report, which provides detailed information about your
        DataSync transfer. For more information, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/task-reports.html">Monitoring your DataSync
        transfers with task reports</a>.</p> |
| `includes` | Vec<String> | <p>The include filters that define the files, objects, and folders in your source location
      that you want DataSync to transfer. For more information and examples, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/filtering.html">Specifying what DataSync transfers by using filters</a>.</p> |
| `error_code` | String | <p>If there's an issue with your task, you can use the error code to help you troubleshoot
      the problem. For more information, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/troubleshooting-datasync-locations-tasks.html">Troubleshooting issues with DataSync transfers</a>.</p> |
| `creation_time` | String | <p>The time that the task was created.</p> |
| `schedule_details` | String | <p>The details about your <a href="https://docs.aws.amazon.com/datasync/latest/userguide/task-scheduling.html">task schedule</a>.</p> |
| `task_mode` | String | <p>The task mode that you're using. For more information, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/choosing-task-mode.html">Choosing a task mode for your data
        transfer</a>.</p> |
| `source_network_interface_arns` | Vec<String> | <p>The ARNs of the <a href="https://docs.aws.amazon.com/datasync/latest/userguide/datasync-network.html#required-network-interfaces">network
        interfaces</a> that DataSync created for your source location.</p> |
| `task_arn` | String | <p>The ARN of your task.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create task
task = provider.datasync.Task {
    destination_location_arn = "value"  # <p>Specifies the ARN of your transfer's destination location. </p>
    source_location_arn = "value"  # <p>Specifies the ARN of your transfer's source location.</p>
}

# Access task outputs
task_id = task.id
task_excludes = task.excludes
task_source_location_arn = task.source_location_arn
task_cloud_watch_log_group_arn = task.cloud_watch_log_group_arn
task_error_detail = task.error_detail
task_destination_location_arn = task.destination_location_arn
task_destination_network_interface_arns = task.destination_network_interface_arns
task_status = task.status
task_current_task_execution_arn = task.current_task_execution_arn
task_options = task.options
task_schedule = task.schedule
task_name = task.name
task_manifest_config = task.manifest_config
task_task_report_config = task.task_report_config
task_includes = task.includes
task_error_code = task.error_code
task_creation_time = task.creation_time
task_schedule_details = task.schedule_details
task_task_mode = task.task_mode
task_source_network_interface_arns = task.source_network_interface_arns
task_task_arn = task.task_arn
```

---


### Location

Location resource

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


### Location_nfs

LocationNfs resource

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `server_hostname` | String | ✅ | <p>Specifies the DNS name or IP address (IPv4 or IPv6) of the NFS file server that your DataSync agent connects to.</p> |
| `subdirectory` | String | ✅ | <p>Specifies the export path in your NFS file server that you want DataSync to
      mount.</p>
         <p>This path (or a subdirectory of the path) is where DataSync transfers data to
      or from. For information on configuring an export for DataSync, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/create-nfs-location.html#accessing-nfs">Accessing NFS file servers</a>.</p> |
| `on_prem_config` | String | ✅ | <p>Specifies the Amazon Resource Name (ARN) of the DataSync agent that can
      connect to your NFS file server.</p>
         <p>You can specify more than one agent. For more information, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/do-i-need-datasync-agent.html#multiple-agents">Using multiple DataSync agents</a>.</p> |
| `mount_options` | String |  | <p>Specifies the options that DataSync can use to mount your NFS file
      server.</p> |
| `tags` | Vec<String> |  | <p>Specifies labels that help you categorize, filter, and search for your Amazon Web Services resources. We recommend creating at least a name tag for your location.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `location_uri` | String | <p>The URI of the NFS location.</p> |
| `location_arn` | String | <p>The ARN of the NFS location.</p> |
| `creation_time` | String | <p>The time when the NFS location was created.</p> |
| `mount_options` | String | <p>The mount options that DataSync uses to mount your NFS file server.</p> |
| `on_prem_config` | String |  |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create location_nfs
location_nfs = provider.datasync.Location_nfs {
    server_hostname = "value"  # <p>Specifies the DNS name or IP address (IPv4 or IPv6) of the NFS file server that your DataSync agent connects to.</p>
    subdirectory = "value"  # <p>Specifies the export path in your NFS file server that you want DataSync to
      mount.</p>
         <p>This path (or a subdirectory of the path) is where DataSync transfers data to
      or from. For information on configuring an export for DataSync, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/create-nfs-location.html#accessing-nfs">Accessing NFS file servers</a>.</p>
    on_prem_config = "value"  # <p>Specifies the Amazon Resource Name (ARN) of the DataSync agent that can
      connect to your NFS file server.</p>
         <p>You can specify more than one agent. For more information, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/do-i-need-datasync-agent.html#multiple-agents">Using multiple DataSync agents</a>.</p>
}

# Access location_nfs outputs
location_nfs_id = location_nfs.id
location_nfs_location_uri = location_nfs.location_uri
location_nfs_location_arn = location_nfs.location_arn
location_nfs_creation_time = location_nfs.creation_time
location_nfs_mount_options = location_nfs.mount_options
location_nfs_on_prem_config = location_nfs.on_prem_config
```

---


### Location_object_storage

LocationObjectStorage resource

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `server_protocol` | String |  | <p>Specifies the protocol that your object storage server uses to communicate. If not specified, the default
      value is <code>HTTPS</code>.</p> |
| `bucket_name` | String | ✅ | <p>Specifies the name of the object storage bucket involved in the transfer.</p> |
| `server_hostname` | String | ✅ | <p>Specifies the domain name or IP address (IPv4 or IPv6) of the object storage server that
      your DataSync agent connects to.</p> |
| `subdirectory` | String |  | <p>Specifies the object prefix for your object storage server. If this is a source location,
        DataSync only copies objects with this prefix. If this is a destination location,
        DataSync writes all objects with this prefix. </p> |
| `server_port` | i64 |  | <p>Specifies the port that your object storage server accepts inbound network traffic on (for
      example, port 443).</p> |
| `secret_key` | String |  | <p>Specifies the secret key (for example, a password) if credentials are required to
      authenticate with the object storage server.</p>
         <note>
            <p>If you provide a secret using <code>SecretKey</code>, but do not provide secret
        configuration details using <code>CmkSecretConfig</code> or <code>CustomSecretConfig</code>,
        then DataSync stores the token using your Amazon Web Services account's Secrets Manager secret.</p>
         </note> |
| `server_certificate` | String |  | <p>Specifies a certificate chain for DataSync to authenticate with your object
      storage system if the system uses a private or self-signed certificate authority (CA). You
      must specify a single <code>.pem</code> file with a full certificate chain (for example,
        <code>file:///home/user/.ssh/object_storage_certificates.pem</code>).</p>
         <p>The certificate chain might include:</p>
         <ul>
            <li>
               <p>The object storage system's certificate</p>
            </li>
            <li>
               <p>All intermediate certificates (if there are any)</p>
            </li>
            <li>
               <p>The root certificate of the signing CA</p>
            </li>
         </ul>
         <p>You can concatenate your certificates into a <code>.pem</code> file (which can be up to
      32768 bytes before base64 encoding). The following example <code>cat</code> command creates an
        <code>object_storage_certificates.pem</code> file that includes three certificates:</p>
         <p>
            <code>cat object_server_certificate.pem intermediate_certificate.pem
        ca_root_certificate.pem > object_storage_certificates.pem</code>
         </p>
         <p>To use this parameter, configure <code>ServerProtocol</code> to <code>HTTPS</code>.</p> |
| `cmk_secret_config` | String |  | <p>Specifies configuration information for a DataSync-managed secret, which
      includes the <code>SecretKey</code> that DataSync uses to access a specific object
      storage location, with a customer-managed KMS key.</p>
         <p>When you include this paramater as part of a <code>CreateLocationObjectStorage</code>
      request, you provide only the KMS key ARN. DataSync uses this KMS key together with the value you specify for the <code>SecretKey</code> parameter
      to create a DataSync-managed secret to store the location access credentials.</p>
         <p>Make sure the DataSync has permission to access the KMS key that
      you specify.</p>
         <note>
            <p>You can use either <code>CmkSecretConfig</code> (with <code>SecretKey</code>) or
          <code>CustomSecretConfig</code> (without <code>SecretKey</code>) to provide credentials
        for a <code>CreateLocationObjectStorage</code> request. Do not provide both parameters for
        the same request.</p>
         </note> |
| `access_key` | String |  | <p>Specifies the access key (for example, a user name) if credentials are required to
      authenticate with the object storage server.</p> |
| `custom_secret_config` | String |  | <p>Specifies configuration information for a customer-managed Secrets Manager secret where
      the secret key for a specific object storage location is stored in plain text. This
      configuration includes the secret ARN, and the ARN for an IAM role that
      provides access to the secret.</p>
         <note>
            <p>You can use either <code>CmkSecretConfig</code> (with <code>SecretKey</code>) or
          <code>CustomSecretConfig</code> (without <code>SecretKey</code>) to provide credentials
        for a <code>CreateLocationObjectStorage</code> request. Do not provide both parameters for
        the same request.</p>
         </note> |
| `agent_arns` | Vec<String> |  | <p>(Optional) Specifies the Amazon Resource Names (ARNs) of the DataSync agents
      that can connect with your object storage system. If you are setting up an agentless
      cross-cloud transfer, you do not need to specify a value for this parameter.</p>
         <note>
            <p>Make sure you configure this parameter correctly when you first create your storage
        location. You cannot add or remove agents from a storage location after you create
        it.</p>
         </note> |
| `tags` | Vec<String> |  | <p>Specifies the key-value pair that represents a tag that you want to add to the resource.
      Tags can help you manage, filter, and search for your resources. We recommend creating a name
      tag for your location.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `location_uri` | String | <p>The URI of the object storage system location.</p> |
| `server_port` | i64 | <p>The port that your object storage server accepts inbound network traffic on (for example,
      port 443).</p> |
| `agent_arns` | Vec<String> | <p>The ARNs of the DataSync agents that can connect with your object storage
      system.</p> |
| `cmk_secret_config` | String | <p>Describes configuration information for a DataSync-managed secret, such as an
      authentication token or set of credentials that DataSync uses to access a specific
      transfer location, and a customer-managed KMS key.</p> |
| `managed_secret_config` | String | <p>Describes configuration information for a DataSync-managed secret, such as an
      authentication token or set of credentials that DataSync uses to access a specific
      transfer location. DataSync uses the default Amazon Web Services-managed KMS key to encrypt this secret in Secrets Manager.</p> |
| `creation_time` | String | <p>The time that the location was created.</p> |
| `server_protocol` | String | <p>The protocol that your object storage system uses to communicate.</p> |
| `server_certificate` | String | <p>The certificate chain for DataSync to authenticate with your object storage
      system if the system uses a private or self-signed certificate authority (CA).</p> |
| `location_arn` | String | <p>The ARN of the object storage system location.</p> |
| `access_key` | String | <p>The access key (for example, a user name) required to authenticate with the object storage
      system.</p> |
| `custom_secret_config` | String | <p>Describes configuration information for a customer-managed secret, such as an
      authentication token or set of credentials that DataSync uses to access a specific
      transfer location, and a customer-managed KMS key.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create location_object_storage
location_object_storage = provider.datasync.Location_object_storage {
    bucket_name = "value"  # <p>Specifies the name of the object storage bucket involved in the transfer.</p>
    server_hostname = "value"  # <p>Specifies the domain name or IP address (IPv4 or IPv6) of the object storage server that
      your DataSync agent connects to.</p>
}

# Access location_object_storage outputs
location_object_storage_id = location_object_storage.id
location_object_storage_location_uri = location_object_storage.location_uri
location_object_storage_server_port = location_object_storage.server_port
location_object_storage_agent_arns = location_object_storage.agent_arns
location_object_storage_cmk_secret_config = location_object_storage.cmk_secret_config
location_object_storage_managed_secret_config = location_object_storage.managed_secret_config
location_object_storage_creation_time = location_object_storage.creation_time
location_object_storage_server_protocol = location_object_storage.server_protocol
location_object_storage_server_certificate = location_object_storage.server_certificate
location_object_storage_location_arn = location_object_storage.location_arn
location_object_storage_access_key = location_object_storage.access_key
location_object_storage_custom_secret_config = location_object_storage.custom_secret_config
```

---


### Location_fsx_open_zfs

LocationFsxOpenZfs resource

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `protocol` | String | ✅ | <p>The type of protocol that DataSync uses to access your file system.</p> |
| `subdirectory` | String |  | <p>A subdirectory in the location's path that must begin with <code>/fsx</code>. DataSync uses this subdirectory to read or write data (depending on whether the file
      system is a source or destination location).</p> |
| `tags` | Vec<String> |  | <p>The key-value pair that represents a tag that you want to add to the resource. The value
      can be an empty string. This value helps you manage, filter, and search for your resources. We
      recommend that you create a name tag for your location.</p> |
| `fsx_filesystem_arn` | String | ✅ | <p>The Amazon Resource Name (ARN) of the FSx for OpenZFS file system.</p> |
| `security_group_arns` | Vec<String> | ✅ | <p>The ARNs of the security groups that are used to configure the FSx for OpenZFS file
      system.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `location_arn` | String | <p>The ARN of the FSx for OpenZFS location that was described.</p> |
| `location_uri` | String | <p>The uniform resource identifier (URI) of the FSx for OpenZFS location that was
      described.</p>
         <p>Example: <code>fsxz://us-west-2.fs-1234567890abcdef02/fsx/folderA/folder</code>
         </p> |
| `security_group_arns` | Vec<String> | <p>The ARNs of the security groups that are configured for the FSx for OpenZFS file
      system.</p> |
| `protocol` | String | <p>The type of protocol that DataSync uses to access your file system.</p> |
| `creation_time` | String | <p>The time that the FSx for OpenZFS location was created.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create location_fsx_open_zfs
location_fsx_open_zfs = provider.datasync.Location_fsx_open_zfs {
    protocol = "value"  # <p>The type of protocol that DataSync uses to access your file system.</p>
    fsx_filesystem_arn = "value"  # <p>The Amazon Resource Name (ARN) of the FSx for OpenZFS file system.</p>
    security_group_arns = "value"  # <p>The ARNs of the security groups that are used to configure the FSx for OpenZFS file
      system.</p>
}

# Access location_fsx_open_zfs outputs
location_fsx_open_zfs_id = location_fsx_open_zfs.id
location_fsx_open_zfs_location_arn = location_fsx_open_zfs.location_arn
location_fsx_open_zfs_location_uri = location_fsx_open_zfs.location_uri
location_fsx_open_zfs_security_group_arns = location_fsx_open_zfs.security_group_arns
location_fsx_open_zfs_protocol = location_fsx_open_zfs.protocol
location_fsx_open_zfs_creation_time = location_fsx_open_zfs.creation_time
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple location_fsx_windows resources
location_fsx_windows_0 = provider.datasync.Location_fsx_windows {
    fsx_filesystem_arn = "value-0"
    security_group_arns = "value-0"
    user = "value-0"
    password = "value-0"
}
location_fsx_windows_1 = provider.datasync.Location_fsx_windows {
    fsx_filesystem_arn = "value-1"
    security_group_arns = "value-1"
    user = "value-1"
    password = "value-1"
}
location_fsx_windows_2 = provider.datasync.Location_fsx_windows {
    fsx_filesystem_arn = "value-2"
    security_group_arns = "value-2"
    user = "value-2"
    password = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    location_fsx_windows = provider.datasync.Location_fsx_windows {
        fsx_filesystem_arn = "production-value"
        security_group_arns = "production-value"
        user = "production-value"
        password = "production-value"
    }
```

---

## Related Documentation

- [AWS Datasync Documentation](https://docs.aws.amazon.com/datasync/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
