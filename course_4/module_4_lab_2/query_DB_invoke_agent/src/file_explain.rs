use aws_sdk_dynamodb::{Client as DdbClient, types::AttributeValue};
//explanation: 
// 1. Imports the DynamoDB client from the `aws_sdk_dynamodb` crate but aliases the Client to `DdbClient`.
// 2. Also imports `AttributeValue`, which represents different types of data DynamoDB can store.


//use aws_sdk_bedrock::{Client as BedrockClient}; // Hypothetical Bedrock clien
//explanation:
// 1. This line is commented out. 
// 2. It’s a placeholder for a hypothetical BedrockClient from an `aws_sdk_bedrock` crate (which may or may not exist).

use lambda_runtime::{run, service_fn, Error};
//explanation: 
// 1. Imports several items from the `lambda_runtime` crate:
//    - `run`: function that starts the AWS Lambda runtime loop.
//    - `service_fn`: utility that adapts an async function to be a Lambda handler.
//    - `Error`: a generic error type used for the function signature and error handling.

use tracing::{info, error}; // import from the tracing crate
//explanation:
// 1. Imports two macros (`info` and `error`) from the `tracing` crate.
// 2. These macros help log informational and error messages for debugging or observability purposes.

use serde::{Deserialize, Serialize};
//explanation: 
// 1. Imports traits from `serde`: `Deserialize` (to convert JSON into Rust structs) 
//    and `Serialize` (to convert Rust structs back into JSON).

use regex::Regex;
//explanation:
// 1. Imports the `Regex` type from the `regex` crate.
// 2. Allows us to create and work with regular expressions (useful for pattern matching within strings).

use lambda_runtime::LambdaEvent;
//explanation:
// 1. Imports the `LambdaEvent` type, which pairs the event payload with Lambda context details (e.g., request_id).

use aws_config::BehaviorVersion;
//explanation:
// 1. `BehaviorVersion` is an enum or type from `aws_config`. 
// 2. Typically indicates which behavior or version of AWS configuration we want to use (for example, if new/legacy behaviors exist).

use aws_sdk_bedrockruntime::{
    error::ProvideErrorMetadata,
    operation::converse_stream::ConverseStreamError,
    types::{
        error::ConverseStreamOutputError,
        ConverseStreamOutput as ConverseStreamOutputType, Message,
    },
};
//explanation:
// 1. Imports from `aws_sdk_bedrockruntime`:
//    - `ProvideErrorMetadata`: a trait offering additional metadata about an error.
//    - `ConverseStreamError`: an error type that can occur when streaming conversation with the Bedrock model.
//    - `ConverseStreamOutputError`: an error type specifically for output streaming issues.
//    - `ConverseStreamOutputType`: a variant type representing different output events in the streaming conversation.
//    - `Message`: a struct or builder used to create messages for the Bedrock conversation.

use aws_sdk_bedrockruntime::{
    operation::converse::{ConverseError, ConverseOutput},
    types::{ContentBlock, ConversationRole},
    Client as BedrockClient,
};
//explanation:
// 1. Another import block from the `aws_sdk_bedrockruntime` crate. 
// 2. `ConverseError` and `ConverseOutput` are used when doing a non-streaming conversation (standard “converse” calls).
// 3. `ContentBlock` and `ConversationRole` define the structure and role (user vs. system) for messages in the conversation.
// 4. `Client` is aliased as `BedrockClient` to represent the main Bedrock runtime client.

#[derive(Debug)]
struct BedrockConverseStreamError(String);
//explanation:
// 1. Declares a custom error struct named `BedrockConverseStreamError`.
// 2. It contains a single field of type `String`, and derives the `Debug` trait.

impl std::fmt::Display for BedrockConverseStreamError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Can't invoke '{}'. Reason: {}", MODEL_ID, self.0)
    }
}
//explanation:
// 1. Implements the `Display` trait, allowing `BedrockConverseStreamError` to be converted into a readable message (string form).
// 2. The message includes the constant `MODEL_ID` and the internal error reason.

impl std::error::Error for BedrockConverseStreamError {}
//explanation:
// 1. Implements the standard `Error` trait for `BedrockConverseStreamError`.
// 2. This means it can integrate seamlessly with Rust’s error-handling ecosystem.

impl From<&str> for BedrockConverseStreamError {
    fn from(value: &str) -> Self {
        BedrockConverseStreamError(value.into())
    }
}
//explanation:
// 1. Provides a conversion from a `&str` to a `BedrockConverseStreamError`.
// 2. This helps create an instance of `BedrockConverseStreamError` easily by passing a string slice.

impl From<&ConverseStreamError> for BedrockConverseStreamError {
    fn from(value: &ConverseStreamError) -> Self {
        BedrockConverseStreamError(
            match value {
                ConverseStreamError::ModelTimeoutException(_) => "Model took too long",
                ConverseStreamError::ModelNotReadyException(_) => "Model is not ready",
                _ => "Unknown",
            }
            .into(),
        )
    }
}
//explanation:
// 1. Implements conversion from `&ConverseStreamError` (Bedrock-specific streaming error) to `BedrockConverseStreamError`.
// 2. Uses pattern matching to handle different error kinds (model timeout, model not ready, or unknown).

impl From<&ConverseStreamOutputError> for BedrockConverseStreamError {
    fn from(value: &ConverseStreamOutputError) -> Self {
        match value {
            ConverseStreamOutputError::ValidationException(ve) => BedrockConverseStreamError(
                ve.message().unwrap_or("Unknown ValidationException").into(),
            ),
            ConverseStreamOutputError::ThrottlingException(te) => BedrockConverseStreamError(
                te.message().unwrap_or("Unknown ThrottlingException").into(),
            ),
            value => BedrockConverseStreamError(
                value
                    .message()
                    .unwrap_or("Unknown StreamOutput exception")
                    .into(),
            ),
        }
    }
}
//explanation:
// 1. Another `From` conversion, but this time from `ConverseStreamOutputError` (an error type for streaming outputs).
// 2. Different variants like `ValidationException` or `ThrottlingException` are matched and converted to a short string message.


// Import additional necessary modules from aws_sdk_bedrockruntime...
//explanation:
// 1. A simple placeholder comment indicating that more imports could be added here if necessary.

const MODEL_ID: &str = "anthropic.claude-instant-v1";
//explanation:
// 1. A global constant specifying the ID of the model that we want to call in the Bedrock environment.

/// This is a made-up example. Incoming messages come into the runtime as unicode
/// strings in json format, which can map to any structure that implements `serde::Deserialize`
/// The runtime pays no attention to the contents of the incoming message payload.
#[derive(Deserialize)]
pub(crate) struct IncomingMessage {
    command: String,
}
//explanation:
// 1. Defines a struct `IncomingMessage` with one field: `command` (of type String).
// 2. The `Deserialize` derive allows this struct to be automatically constructed from JSON input.

/// This is a made-up example of what an outgoing message structure may look like.
/// There is no restriction on what it can be. The runtime requires responses
/// to be serialized into json. The runtime pays no attention
/// to the contents of the outgoing message payload.
#[derive(Serialize)]
pub(crate) struct OutgoingMessage {
    req_id: String,
    msg: String,
}
//explanation:
// 1. Defines a struct `OutgoingMessage` with two fields: `req_id` and `msg`.
// 2. The `Serialize` derive allows us to convert this struct to JSON when returning from the Lambda.

async fn query_table_of_dynamodb(ddb_client: &DdbClient, table_name: String, person_name: String) 
    -> Result<String, aws_sdk_dynamodb::Error> 
{
    // query table of dynamodb
    let query_value = ddb_client
        .query()
        .table_name(table_name)
        .index_name("name_index")
        .key_condition_expression("#nm = :name")
        .expression_attribute_names("#nm","name")
        .expression_attribute_values(":name", AttributeValue::S(person_name))
        .projection_expression("address")
        .send()
        .await?;
    //explanation:
    // 1. We invoke the `.query()` method on the DynamoDB client.
    // 2. `.table_name(table_name)` sets the table we’re querying.
    // 3. `.index_name("name_index")` indicates which secondary index we want to use in this query (an index on the "name" attribute).
    // 4. `.key_condition_expression("#nm = :name")` is the condition for the query (i.e., the "name" attribute must match).
    // 5. `.expression_attribute_names("#nm","name")` binds the placeholder "#nm" in the query to the actual attribute "name".
    // 6. `.expression_attribute_values(":name", AttributeValue::S(person_name))` provides the value for ":name" as a string (S).
    // 7. `.projection_expression("address")` limits the returned fields to only "address".
    // 8. `.send().await?` sends the query to DynamoDB asynchronously; `?` propagates any error.

    let address = query_value
        .items
        .unwrap_or_default()
        .get(0)
        .and_then(|item| item.get("address"))
        .and_then(|attr| attr.as_s().ok())
        .map(|s| s.to_string())
        .unwrap_or_default();    
    //explanation:
    // 1. `.items` retrieves the list of matching items from the query result (if `items` is `None`, use an empty default).
    // 2. `.get(0)` tries to take the first item from that list.
    // 3. `.and_then(|item| item.get("address"))` looks up the "address" attribute in that item (a HashMap).
    // 4. `.and_then(|attr| attr.as_s().ok())` attempts to interpret the attribute as a string (AttributeValue::S).
    // 5. `.map(|s| s.to_string())` converts the borrowed `&str` into a `String`.
    // 6. `.unwrap_or_default()` falls back to an empty string if anything fails or is missing.

    Ok(address)
}
//explanation:
// 1. The function returns an Ok(...) containing the address string if found, 
//    or an empty string if not present, or an error if the DynamoDB query fails.

fn get_converse_output_text(
    output: ConverseStreamOutputType,
) -> Result<String, BedrockConverseStreamError> {
    Ok(match output {
        ConverseStreamOutputType::ContentBlockDelta(event) => match event.delta() {
            Some(delta) => delta.as_text().cloned().unwrap_or_else(|_| "".into()),
            None => "".into(),
        },
        _ => "".into(),
    })
}
//explanation:
// 1. A helper function to extract text from a streaming output event (`ConverseStreamOutputType`).
// 2. If `output` is `ContentBlockDelta`, we try to get `event.delta()` which may contain textual data. 
// 3. `.as_text().cloned()` tries to convert that delta to a string, and if it fails, defaults to "".
// 4. If `output` is any other variant, we return an empty string.

pub(crate) async fn bedrock_responses(bedrock_client: &BedrockClient, message: String) 
    -> Result<String, Error> 
{
    // create a funtion to call bedrock and send message
    let response = bedrock_client.converse_stream()
        .model_id(MODEL_ID)
        .messages(
            Message::builder()
                .role(ConversationRole::User)
                .content(ContentBlock::Text(message.to_string()))
                .build()
                .map_err(|_| "failed to build message")?,
        )
        .send()
        .await;
    //explanation:
    // 1. We start a streaming conversation by calling `.converse_stream()` on the bedrock client.
    // 2. `.model_id(MODEL_ID)` tells the client which model to converse with (e.g. "anthropic.claude-instant-v1").
    // 3. We create a `Message` via the builder pattern:
    //    - `.role(ConversationRole::User)` indicates the message is from the user.
    //    - `.content(ContentBlock::Text(message.to_string()))` sets the text content of the user’s message.
    //    - `.build()` finalizes the message, and `map_err` converts any build error into a string-based error.
    // 4. `.send().await` initiates the streaming request and waits for the first server response.

    let mut stream = match response {
        Ok(output) => Ok(output.stream),
        Err(e) => Err(BedrockConverseStreamError::from(
            e.as_service_error().unwrap(),
        )),
    }?; 
    //explanation:
    // 1. If the response is `Ok`, extract its `.stream` field (the actual streaming object).
    // 2. If `Err(e)`, convert the error from bedrock (using `e.as_service_error()`) into our custom `BedrockConverseStreamError`.
    // 3. We then unwrap the `Result` with `?` to either continue with the stream or return an error.

    let mut response_text = String::new();
    //explanation:
    // 1. Initializes an empty string that will accumulate pieces of streamed text from the model.

    loop {
        let token = stream.recv().await;
        //explanation:
        // 1. `stream.recv()` waits for the next streaming token from the model. 
        // 2. `.await` because it’s an async operation.

        match token {
            Ok(Some(text)) => {
                let next = get_converse_output_text(text)?;
                print!("{}", next);
                response_text.push_str(&next);
                Ok(())
            }
            Ok(None) => break,
            Err(e) => Err(e
                .as_service_error()
                .map(BedrockConverseStreamError::from)
                .unwrap_or(BedrockConverseStreamError(
                    "Unknown error receiving stream".into(),
                ))),
        }?
        //explanation:
        // 1. `Ok(Some(text))`: means we got another chunk of streaming data:
        //    - Convert it to text using `get_converse_output_text`.
        //    - Print it to stdout for real-time visibility.
        //    - Append the partial text to `response_text`.
        // 2. `Ok(None)`: indicates the streaming has finished, so we `break` out of the loop.
        // 3. `Err(e)`: an error occurred in the streaming process:
        //    - Convert it to our custom error or a default “Unknown error receiving stream”.
        //    - We use `?` at the end to propagate the error if it’s an Err.

    }

    Ok(response_text)
}
//explanation:
// 1. Once no more tokens are coming in from the model, we exit the loop.
// 2. Return the accumulated `response_text` as the final answer from the model.

/// This is the main body for the function.
/// Write your code inside it.
pub(crate) async fn function_handler(event: LambdaEvent<IncomingMessage>) -> Result<OutgoingMessage, Error> {
    // Extract some useful info from the request
    let command = event.payload.command;
    //explanation:
    // 1. Extracts the `command` string from the `IncomingMessage` stored in `event.payload`.

    //let check_command = command.clone();
    //explanation:
    // 1. This line is commented out. 
    // 2. Could be used for debugging or comparing the command with something else.

    // How to retrieve name in the command 
    let re = Regex::new(r"\[(.*?)\]").unwrap();
    //explanation:
    // 1. Creates a `Regex` object to match text inside square brackets: `[ ... ]`.
    // 2. `r"\[(.*?)\]"` is a raw string literal for a pattern capturing whatever is between the brackets.
    // 3. `.unwrap()` panics if the regex pattern is invalid, which usually shouldn't happen here.

    // retrieve name
    let name = re
        .captures(&command)
        //explanation:
        // 1. `.captures(&command)` attempts to match the regex against the `command` string.
        // 2. If it succeeds, it returns a `Captures` object containing all matched groups.

        .and_then(|caps| caps.get(1))
        //explanation:
        // 1. `.and_then(|caps| caps.get(1))` tries to get the first capture group (the text inside brackets).
        // 2. If no match is found, this becomes None.

        .map(|m| m.as_str().to_string())
        //explanation:
        // 1. If a capture is found, `.as_str()` yields the matched substring slice.
        // 2. `.to_string()` converts that slice to an owned String.

        .unwrap_or_default();
    //explanation:
    // 1. If the entire chain returns None, `.unwrap_or_default()` provides an empty string (“”).
    // 2. This ensures `name` is never None, just possibly an empty string if no brackets were found in `command`.

    let table_name = "customers".to_string();
    //explanation:
    // 1. Assigns the string "customers" to `table_name`. 
    // 2. This is the DynamoDB table that we’re going to query.

    //client
    let ddb_config = aws_config::load_from_env().await;
    //explanation:
    // 1. Loads AWS configuration from environment variables or default sources.
    // 2. This typically sets up the region, credentials, etc.

    //let ddb_client = Client::new(&config);
    //explanation:
    // 1. This line is commented out. 
    // 2. Shows an alternative way to create a client from a config object (not used here).

    let ddb_client = aws_sdk_dynamodb::Client::new(&ddb_config);
    //explanation:
    // 1. Initializes a new DynamoDB client using the loaded configuration.

    // query table of dynamodb
    let address = query_table_of_dynamodb(&ddb_client, table_name, name).await?;
    //explanation:
    // 1. Calls our `query_table_of_dynamodb` function, passing in the client, table name, and the name from brackets.
    // 2. Waits for the result (async). If there's an error, `?` propagates it immediately.

    let message = format!("Question: {} \nAugmented context: {}.", command, address);
    //explanation:
    // 1. We create a string that includes the user’s original command and the “augmented context” (address) from DynamoDB.
    // 2. This combined context is what we’ll feed into the Bedrock model.

    // create a funtion to call bedrock and send message
    //let bedrock_config = aws_config::load_from_env().await;
    //explanation:
    // 1. Another commented-out line demonstrating an alternate approach to building configuration for Bedrock.

    let bedrock_config = aws_config::from_env().region("eu-central-1").load().await;
    //explanation:
    // 1. Builds an AWS config specifically for the region "eu-central-1" from environment sources.
    // 2. `.load().await` finalizes loading the config.

    let bedrock_client = aws_sdk_bedrockruntime::Client::new(&bedrock_config);
    //explanation:
    // 1. Instantiates a `BedrockClient` using our region-specific config.

    let resp_1 = bedrock_responses(&bedrock_client, message).await?;
    //explanation:
    // 1. Calls our `bedrock_responses` function to send the message to Bedrock.
    // 2. Captures the final text response from the streaming conversation in `resp_1`.

    // Prepare the outgoing message
    let resp = OutgoingMessage {
        req_id: event.context.request_id,
        msg: format!("{}.", resp_1),
    };
    //explanation:
    // 1. Constructs an `OutgoingMessage` struct:
    //    - `req_id`: taken from the AWS Lambda `event.context`.
    //    - `msg`: includes the streaming result plus a period at the end.

    // Return `OutgoingMessage` (it will be serialized to JSON automatically by the runtime)
    Ok(resp)
}
//explanation:
// 1. The `function_handler` function ends by returning `Ok(resp)`, 
//    which the Lambda runtime serializes to JSON for the response.

#[cfg(test)]
mod tests {
    use super::*;
    use lambda_runtime::{Context, LambdaEvent};

    #[tokio::test]
    async fn test_generic_handler() {
        let event = LambdaEvent::new(
            IncomingMessage { command: "test".to_string() }, 
            Context::default()
        );
        //explanation:
        // 1. Creates a test Lambda event with a payload of `{ command: "test" }`.
        // 2. `Context::default()` just gives a default, mostly-empty Lambda context.

        let response = function_handler(event).await.unwrap();
        //explanation:
        // 1. Calls `function_handler` with the test event.
        // 2. `.await` because `function_handler` is async.
        // 3. `.unwrap()` will panic if an error occurs, failing the test.

        assert_eq!(response.msg, "Command test.");
        //explanation:
        // 1. Checks that the returned `OutgoingMessage` has its `msg` field set to "Command test.".
        // 2. If the message doesn’t match, the test fails.
    }
}
//explanation:
// 1. The `tests` module is only included when running tests (i.e., `cargo test`).
// 2. It verifies the behavior of `function_handler`.
