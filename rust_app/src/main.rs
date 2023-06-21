use aws_lambda_events::sqs::SqsEvent;
use aws_sdk_ssm::Client;
use aws_sdk_ssm::config::Region;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};

async fn function_handler(event: LambdaEvent<SqsEvent>) -> Result<(), Error> {
    println!("Hello world!! {:?}", event);

    let region_provider = Region::new("us-east-1");

    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);

    show_parameters(&client).await;

    Ok(())
}

async fn show_parameters(client: &Client) -> () {
    let resp = client.describe_parameters().send().await.unwrap();

    for param in resp.parameters().unwrap().iter() {
        println!("  {}", param.name().unwrap_or_default());
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(function_handler)).await
}
