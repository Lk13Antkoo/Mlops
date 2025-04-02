"""List claude models in my account in us-west-2"""

import boto3

client = boto3.client("bedrock", region_name="us-east-1")
response = client.list_foundation_models()

claude_models = []
for model in response["modelSummaries"]:
    if "claude" in model["modelName"].lower():  # Case-insensitive search for "claude"
        claude_models.append(
            {
                "modelArn": model["modelArn"],
                "modelName": model["modelName"],
                "providerName": model["providerName"],
            }
        )

if claude_models:
    print("Found the following Claude models:")
    for model in claude_models:
        print(f"- Name: {model['modelName']}")
        print(f"- Provider: {model['providerName']}")
        print(f"- ARN: {model['modelArn']}\n")
else:
    print("No Claude models found in your account.")
