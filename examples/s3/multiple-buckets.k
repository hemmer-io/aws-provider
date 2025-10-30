# Multiple S3 Buckets Example
#
# This example creates multiple S3 buckets for different purposes.
# Demonstrates managing multiple resources of the same type.

schema Environment:
    name: str
    stacks: [Stack]

schema Stack:
    name: str
    resources: [Resource]

schema Resource:
    type: str
    name: str
    attributes: {str:}

# Production environment with multiple buckets
prod = Environment {
    name = "prod"
    stacks = [{
        name = "storage"
        resources = [
            # Data bucket
            {
                type = "aws_bucket"
                name = "data"
                attributes = {
                    name = "hemmer-prod-data-bucket-12345"  # Replace with unique name
                }
            }
            # Logs bucket
            {
                type = "aws_bucket"
                name = "logs"
                attributes = {
                    name = "hemmer-prod-logs-bucket-12345"  # Replace with unique name
                }
            }
            # Backups bucket
            {
                type = "aws_bucket"
                name = "backups"
                attributes = {
                    name = "hemmer-prod-backups-bucket-12345"  # Replace with unique name
                }
            }
        ]
    }]
}
