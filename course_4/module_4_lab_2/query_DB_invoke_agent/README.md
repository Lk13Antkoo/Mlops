flowchart TB
    A([LambdaEvent<br>IncomingMessage]) --> B{function_handler}

    B --> C[Extract command<br> via event.payload]

    C --> D[Parse name<br> using Regex]

    D --> E[query_table_of_dynamodb<br>(DynamoDB)]

    E --> F[Build message with<br> augmented context]

    F --> G[Create BedrockClient<br> (Region, Config)]

    G --> H[bedrock_responses<br> (Streaming)]

    H --> I[Loop through streaming<br> tokens and accumulate text]

    I --> J[Construct OutgoingMessage]

    J --> K([Return JSON])

A → B: Lambda receives the event (IncomingMessage) and enters function_handler.

B → C: We pull the command field from the incoming payload.
C → D: A Regex checks if there is a name enclosed in brackets.

D → E: The function queries DynamoDB for the address associated with that name (using query_table_of_dynamodb).

E → F: The retrieved address is appended to the user’s command to form a new message (augmented context).

F → G: We configure Bedrock by creating a BedrockClient (with region settings).
G → H: Invoke bedrock_responses to start a streaming conversation with the model.

H → I: In a loop, we receive tokens from the model. Each token is appended to response_text.

I → J: Once streaming completes, we create an OutgoingMessage with the final model output.

J → K: The function returns the JSON-serialized OutgoingMessage as the Lambda response.

let address = query_value.items.unwrap_or_default().get(0).and_then(|item| item.get("address")).and_then(|attr| attr.as_s().ok()).map(|s| s.to_string()).unwrap_or_default();    

./llama-cli -m ../../gemma-1.1-7b-it.Q4_K_M.gguf -cnv --chat-template gemma