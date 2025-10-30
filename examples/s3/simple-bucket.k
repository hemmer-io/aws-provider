# Simple S3 Bucket Example
#
# This example creates a single S3 bucket for testing.
# Make sure to use a globally unique bucket name!

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

# Development environment with a single S3 bucket
dev = Environment {
    name = "dev"
    stacks = [{
        name = "storage"
        resources = [
            {
                type = "aws_bucket"
                name = "data"
                attributes = {
                    # IMPORTANT: Change this to a unique name!
                    name = "hemmer-test-data-bucket-${timestamp()}"
                }
            }
        ]
    }]
}

# Helper function to generate timestamp for unique names
timestamp = lambda -> str {
    import datetime
    datetime.datetime.now().strftime("%Y%m%d-%H%M%S")
}
