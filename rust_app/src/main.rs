use std::error::Error;

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::Client;
use lambda_runtime::{run, service_fn, LambdaEvent};
use models::{InputMessage, Response};

mod models;
mod service;

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
/// - https://github.com/aws-samples/serverless-rust-demo/
async fn function_handler(client: &Client, event: LambdaEvent<InputMessage>) -> Result<Response, Box<dyn Error>> {
    
    let table_name = std::env::var("TABLE_NAME")?;

    let response = service::handle_message(client, event.payload, &table_name).await?;

    let resp = Response {
        statusCode: 200,
        body: response,
    };

    Ok(resp)
}


#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    run(service_fn(|event: LambdaEvent<InputMessage>| {
        function_handler(&client, event)
    })).await
}
