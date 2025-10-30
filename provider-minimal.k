# AWS Provider Manifest (Minimal for Testing)
# Simplified version for S3 and EC2 testing

schema ProviderManifest:
    name: str
    version: str
    source: str
    executable: str
    protocol: str
    resources: {str: ResourceDefinition}
    config_schema?: {str: FieldDefinition}

schema ResourceDefinition:
    fields: {str: FieldDefinition}
    capabilities: Capabilities
    outputs: {str: str}

schema FieldDefinition:
    type: str
    required: bool
    default?: any
    description?: str
    sensitive?: bool
    immutable?: bool

schema Capabilities:
    create: bool
    read: bool
    update: bool
    delete: bool

# AWS Provider Configuration
aws_provider = ProviderManifest {
    name = "aws"
    version = "0.1.0"
    source = "github.com/hemmer-io/aws-provider"
    executable = "libhemmer_aws_provider"
    protocol = "native"

    # Provider configuration schema
    config_schema = {
        region = {
            type = "String"
            required = False
            description = "AWS region (e.g., us-east-1). If not specified, uses AWS_REGION environment variable."
        }
        profile = {
            type = "String"
            required = False
            description = "AWS profile name from ~/.aws/credentials"
        }
    }

    # Resource definitions
    resources = {
        # S3 Bucket
        bucket = {
            fields = {
                name = {
                    type = "String"
                    required = True
                    immutable = True
                    description = "The name of the S3 bucket. Must be globally unique across all AWS accounts."
                }
            }
            capabilities = {
                create = True
                read = True
                update = True
                delete = True
            }
            outputs = {
                id = "String"
                name = "String"
                arn = "String"
            }
        }

        # EC2 Instance
        instance = {
            fields = {
                ami = {
                    type = "String"
                    required = True
                    immutable = True
                    description = "The AMI ID to use for the instance"
                }
                instance_type = {
                    type = "String"
                    required = True
                    description = "The instance type (e.g., t2.micro, t3.small)"
                }
            }
            capabilities = {
                create = True
                read = True
                update = True
                delete = True
            }
            outputs = {
                id = "String"
                instance_id = "String"
                state = "String"
            }
        }
    }
}
