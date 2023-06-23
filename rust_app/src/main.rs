use aws_lambda_events::sqs::{SqsEvent, SqsMessage, SqsMessageAttribute};
use aws_sdk_ssm::Client;
use aws_sdk_ssm::config::Region;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};

async fn function_handler(event: LambdaEvent<SqsEvent>) -> Result<(), Error> {
    println!("Hello world!! {:?}", event);

    let region_provider = Region::new("us-east-1");

    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);

    show_parameters(&client, &event).await;

    Ok(())
}

async fn show_parameters(client: &Client, event: &LambdaEvent<SqsEvent>) -> () {
    let first_records = event.payload.records.get(0);
    match first_records {
        None => {println!("No records found for event")}
        Some(r) => {
            let env = r.message_attributes.get("EnvironmentId");
            match env {
                None => {println!("No message attribute for the record")}
                Some(e) => {
                    let path = "/telegram-ids/".to_owned() + e.string_value.clone().unwrap().as_str();
                    let resp = client.get_parameters_by_path().path(path).send().await.unwrap();

                    for param in resp.parameters().unwrap().iter() {
                        match param.name() {
                            None => {}
                            Some(name) => { println!("  {name}") }
                        }
                    }
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(function_handler)).await
}
