use aws_sdk_dynamodb::{Client as DdbClient, types::AttributeValue};
//use aws_sdk_bedrock::{Client as BedrockClient}; // Hypothetical Bedrock clien
use lambda_runtime::{run, service_fn, Error};
use tracing::{info, error}; // import from the tracing crate
use serde::{Deserialize, Serialize};
use regex::Regex;
use lambda_runtime::LambdaEvent;
use aws_config::BehaviorVersion;
use aws_sdk_bedrockruntime::{
    error::ProvideErrorMetadata,
    operation::converse_stream::ConverseStreamError,
    types::{
        error::ConverseStreamOutputError,
        ConverseStreamOutput as ConverseStreamOutputType, Message,
    },
    
};
use aws_sdk_bedrockruntime::{
    operation::converse::{ConverseError, ConverseOutput},
    types::{ContentBlock, ConversationRole},
    Client as BedrockClient,
};

#[derive(Debug)]
struct BedrockConverseStreamError(String);
impl std::fmt::Display for BedrockConverseStreamError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Can't invoke '{}'. Reason: {}", MODEL_ID, self.0)
    }
}
impl std::error::Error for BedrockConverseStreamError {}
impl From<&str> for BedrockConverseStreamError {
    fn from(value: &str) -> Self {
        BedrockConverseStreamError(value.into())
    }
}

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



// Import additional necessary modules from aws_sdk_bedrockruntime...

const MODEL_ID: &str = "anthropic.claude-instant-v1";

/// This is a made-up example. Incoming messages come into the runtime as unicode
/// strings in json format, which can map to any structure that implements `serde::Deserialize`
/// The runtime pays no attention to the contents of the incoming message payload.
#[derive(Deserialize)]
pub(crate) struct IncomingMessage {
    command: String,
}

/// This is a made-up example of what an outgoing message structure may look like.
/// There is no restriction on what it can be. The runtime requires responses
/// to be serialized into json. The runtime pays no attention
/// to the contents of the outgoing message payload.
#[derive(Serialize)]
pub(crate) struct OutgoingMessage {
    req_id: String,
    msg: String,
}

async fn query_table_of_dynamodb(ddb_client: &DdbClient, table_name:String, person_name:String) -> Result<String, aws_sdk_dynamodb::Error> {
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

        let address = query_value
        .items
        .unwrap_or_default()
        .get(0)
        .and_then(|item| item.get("address"))
        .and_then(|attr| attr.as_s().ok())
        .map(|s| s.to_string())
        .unwrap_or_default();    

    Ok(address)
}

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


pub(crate) async fn bedrock_responses(bedrock_client: &BedrockClient, message:String) -> Result<String, Error> {
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

    let mut stream = match response {
    Ok(output) => Ok(output.stream),
    Err(e) => Err(BedrockConverseStreamError::from(
        e.as_service_error().unwrap(),
    )),
}?; 

    let mut response_text = String::new();

    loop {
    let token = stream.recv().await;
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
}

    Ok(response_text)
}

/// This is the main body for the function.
/// Write your code inside it.
pub(crate) async fn function_handler(event: LambdaEvent<IncomingMessage>) -> Result<OutgoingMessage, Error> {
    // Extract some useful info from the request
    let command = event.payload.command;

    //let check_command = command.clone();

    // How to retrieve name in the command 
    let re = Regex::new(r"\[(.*?)\]").unwrap();
    // retrieve name
    let name = re
    .captures(&command)
    .and_then(|caps| caps.get(1))
    .map(|m| m.as_str().to_string())
    .unwrap_or_default();

    let table_name = "customers".to_string();

    //client
    let ddb_config = aws_config::load_from_env().await;
    //let ddb_client = Client::new(&config);
    let ddb_client = aws_sdk_dynamodb::Client::new(&ddb_config);

    // query table of dynamodb
    let address = query_table_of_dynamodb(&ddb_client, table_name, name).await?;

    let message = format!("Question: {} \nAugmented context: {}.", command, address);

    // create a funtion to call bedrock and send message
    //let bedrock_config = aws_config::load_from_env().await;
    let bedrock_config = aws_config::from_env().region("eu-central-1").load().await;

    let bedrock_client = aws_sdk_bedrockruntime::Client::new(&bedrock_config);

    let resp_1= bedrock_responses(&bedrock_client, message).await?;

    // Prepare the outgoing message
    let resp = OutgoingMessage {
        req_id: event.context.request_id,
        msg: format!("{}.", resp_1),
    };

    // Return `OutgoingMessage` (it will be serialized to JSON automatically by the runtime)
    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;
    use lambda_runtime::{Context, LambdaEvent};

    #[tokio::test]
    async fn test_generic_handler() {
        let event = LambdaEvent::new(IncomingMessage { command: "test".to_string() }, Context::default());
        let response = function_handler(event).await.unwrap();
        assert_eq!(response.msg, "Command test.");
    }
}
