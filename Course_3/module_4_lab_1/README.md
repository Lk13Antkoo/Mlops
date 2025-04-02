# This lab is on Course 3: advanced data engineering, Course 4 lab 1

Using bash command
For the task insert data from csv -> write pythonscript to convert to json format.

## Step:

1. Install AWS toolkit, create account and dev environment, please add Policies..

 For creating dev environment:

 https://codecatalyst.aws/spaces/Aws_VS_toolkit/projects/aws_3/settings
  
  For DynamoDB check table

  https://us-east-1.console.aws.amazon.com/dynamodbv2/home?region=us-east-1#tables

  for creating AWS service:

  https://us-east-1.console.aws.amazon.com/iam/home?region=us-east-1#/users/details/aws_tool_kit?section=permissions

2. Open AWS toolkit in VS and login

3. Make sure that you are using right region:
```bash
    echo $AWS_REGION
    echo $AWS_DEFAULT_REGION
```

4. Create table named "customers" with "customer_id" is the primary key.
```bash
aws dynamodb create-table \
    --table-name customers \
    --attribute-definitions \
        AttributeName=customer_id,AttributeType=N \
    --key-schema \
        AttributeName=customer_id,KeyType=HASH \
    --provisioned-throughput \
        ReadCapacityUnits=5,WriteCapacityUnits=5

```

5. Add data to "customers", file:// to make sure the code line can identify the file.

```bash
aws dynamodb batch-write-item \
--request-items file:///projects/aws_3/records.json
```

6. Query by primary key
```bash
aws dynamodb query \
    --table-name customers \
    --key-condition-expression "customer_id = :id" \
    --expression-attribute-values '{":id":{"N":"1"}}'
```

7. Add second index -> make "name" become the second index
Primary Key Limitation:

DynamoDB tables are designed to allow querying only by the primary key (partition key or partition key + sort key).
If you want to query by an attribute that is not part of the primary key, you must create a secondary index.
Secondary Index Solution:

A Global Secondary Index (GSI) allows you to query the table using any attribute as a partition key (and optionally, a sort key).
A Local Secondary Index (LSI) allows querying with the same partition key as the table but with a different sort key.
```bash
aws dynamodb update-table \
    --table-name customers \
    --attribute-definitions AttributeName=name,AttributeType=S \
    --global-secondary-index-updates \
        '[{
            "Create": {
                "IndexName": "name-index",
                "KeySchema": [
                    { "AttributeName": "name", "KeyType": "HASH" }
                ],
                "Projection": {
                    "ProjectionType": "ALL"
                },
                "ProvisionedThroughput": {
                    "ReadCapacityUnits": 5,
                    "WriteCapacityUnits": 5
                }
            }
        }]'
```

How to delete second index:
```bash
aws dynamodb update-table \
    --table-name customers \
    --global-secondary-index-updates '[{
        "Delete": {
            "IndexName": "name-index"
        }
    }]'

```
How to check information of table
```
aws dynamodb describe-table --table-name customers

```

8. Query: because "name" is a reserved keyword so add "#" before "name".

```bash
aws dynamodb query \
    --table-name customers \
    --index-name name-index \
    --key-condition-expression "#name = :nameValue" \
    --expression-attribute-names '{"#name": "name"}' \
    --expression-attribute-values '{":nameValue":{"S":"John Doe"}}'

```
Output:
```
{
    "Items": [
        {
            "customer_id": {
                "N": "1"
            },
            "name": {
                "S": "John Doe"
            },
            "address": {
                "S": "123 Main St"
            }
        }
    ],
    "Count": 1,
    "ScannedCount": 1,
    "ConsumedCapacity": null
}
```