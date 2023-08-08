use std::collections::HashMap;
use aws_lambda_events::sqs::{SqsEvent, SqsMessage};
use aws_sdk_ssm::Client;
use aws_sdk_ssm::config::Region;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};

struct Props<'a> {
    channel: &'a String,
    environment: &'a String,
    text: &'a String,
}

impl Props<'_> {
    fn new(_msg: &SqsMessage) -> Props<'_> {
        let channel = _msg.message_attributes["Channel"].string_value.as_ref().expect("Channel name not present");
        let environment = _msg.message_attributes["EnvironmentId"].string_value.as_ref().expect("Environment id not present");
        let text = _msg.body.as_ref().expect("Text expected in the message");
        Props {
            channel,
            environment,
            text,
        }
    }
}

async fn function_handler(event: LambdaEvent<SqsEvent>) -> Result<(), Box<dyn std::error::Error>> {
    let region_provider = Region::new("us-east-1");

    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);

    //TODO process every message despite lambda being configured to trigger each msg
    let _msg = event.payload.records.get(0).expect("Message not present");

    let props = Props::new(_msg);

    let url = format!("/telegram-ids/{0}/token", props.environment);
    let token = get_parameter(&client, props.environment, &url).await.expect("Token not found");

    let url = format!("/telegram-ids/{0}/{1}", props.environment, props.channel);
    let channel_id = get_parameter(&client, props.environment, &url).await.expect("Channel id not found");

    let mut map = HashMap::new();
    map.insert("chat_id", channel_id.as_str());
    map.insert("text", &props.text);

    let client = reqwest::Client::new();
    client.post(format!("https://api.telegram.org/bot{token}/sendMessage"))
        .json(&map)
        .send().await?;
    Ok(())
}

async fn get_parameter(client: &Client, env: &str, url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let params = load_all_params(client, env).await;
    let opt = params.iter().find(|(x, _y)| x == url);

    let (_x, y) = opt.ok_or("No element found")?;

    return Ok(y.to_owned());
}

async fn load_all_params(client: &Client, env: &str) -> Vec<(String, String)> {
    let path = format!("/telegram-ids/{env}");
    let params_by_path = client.get_parameters_by_path().path(path).send().await.expect("Could not load parameters by path");

    let mut params: Vec<(String, String)> = Vec::new();

    for param in params_by_path.parameters().expect("Could not retrieve parameters").iter() {
        let name = param.name().expect("Could not load the name");
        let value = param.value().expect("Could not load the value");

        params.push((name.to_owned(), value.to_owned()));
    }
    return params;
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(function_handler)).await
}
