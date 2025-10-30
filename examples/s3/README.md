# S3 Bucket Examples

Example configurations for creating and managing S3 buckets with Hemmer.

## Prerequisites

1. **AWS Credentials**: Configure AWS credentials via:
   ```bash
   # Option 1: Environment variables
   export AWS_ACCESS_KEY_ID="your-access-key"
   export AWS_SECRET_ACCESS_KEY="your-secret-key"
   export AWS_REGION="us-east-1"

   # Option 2: AWS CLI profile
   aws configure
   ```

2. **Unique Bucket Names**: S3 bucket names must be globally unique across all AWS accounts. Update the bucket names in the examples to use your own unique names.

## Examples

### 1. Simple Single Bucket (`simple-bucket.k`)

Creates a single S3 bucket for testing.

```bash
# Initialize Hemmer project
hemmer init

# Preview changes
hemmer layout simple-bucket.k

# Apply changes (create bucket)
hemmer stitch simple-bucket.k

# Destroy resources
hemmer rip simple-bucket.k
```

**What it demonstrates:**
- Basic resource definition
- S3 bucket creation
- Unique naming with timestamps

### 2. Multiple Buckets (`multiple-buckets.k`)

Creates multiple S3 buckets for different purposes (data, logs, backups).

```bash
hemmer layout multiple-buckets.k
hemmer stitch multiple-buckets.k
```

**What it demonstrates:**
- Managing multiple resources of the same type
- Resource organization
- Naming conventions

## Testing Workflow

### Complete CRUD Test

```bash
# 1. CREATE: Create the bucket
hemmer stitch simple-bucket.k

# 2. READ: Verify bucket exists (implicit in layout)
hemmer layout simple-bucket.k
# Should show "No changes" if bucket exists

# 3. UPDATE: Modify bucket configuration (limited for S3)
# Edit the KCL file and run:
hemmer stitch simple-bucket.k

# 4. DELETE: Remove the bucket
hemmer rip simple-bucket.k

# 5. IDEMPOTENCY: Run again (should be no-op)
hemmer rip simple-bucket.k
# Should show "No resources to destroy"
```

### Verify with AWS CLI

```bash
# List your buckets
aws s3 ls

# Check specific bucket
aws s3api head-bucket --bucket your-bucket-name

# Delete bucket (alternative)
aws s3 rb s3://your-bucket-name
```

## Important Notes

### Bucket Naming Rules

S3 bucket names must:
- Be globally unique across all AWS accounts
- Be 3-63 characters long
- Use only lowercase letters, numbers, dots (.), and hyphens (-)
- Start and end with a letter or number
- Not be formatted as an IP address

### Common Issues

**BucketAlreadyExists**: The bucket name is taken. Choose a different name.

**AccessDenied**: Check your AWS credentials and IAM permissions.

**InvalidBucketName**: Bucket name doesn't follow naming rules.

### Costs

S3 buckets are free to create. You only pay for:
- Storage (first 50 TB: $0.023/GB/month)
- Requests (GET: $0.0004/1000, PUT: $0.005/1000)

Empty buckets cost nothing. Always delete test buckets when done!

## Provider Configuration

The AWS provider accepts optional configuration:

```kcl
provider_config = {
    aws = {
        region = "us-east-1"      # Optional: AWS region
        profile = "default"        # Optional: AWS CLI profile
    }
}
```

If not specified, uses environment variables or default AWS credentials chain.

## Next Steps

After testing S3:
1. Try EC2 instances (see `examples/ec2/`)
2. Combine resources (S3 + EC2)
3. Add dependencies between resources
4. Test state management

## Troubleshooting

### Debug Mode

```bash
export RUST_LOG=debug
hemmer stitch simple-bucket.k
```

### Check Hemmer State

```bash
hemmer state list
hemmer state show aws_bucket.data
```

### Manual Cleanup

If Hemmer fails to delete a bucket:

```bash
# Empty bucket first (if it has objects)
aws s3 rm s3://your-bucket-name --recursive

# Delete bucket
aws s3 rb s3://your-bucket-name
```
