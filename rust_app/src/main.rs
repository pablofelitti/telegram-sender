use std::collections::HashMap;
use aws_lambda_events::sqs::{SqsEvent, SqsMessage};
use aws_sdk_ssm::Client;
use aws_sdk_ssm::config::Region;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};

async fn function_handler(event: LambdaEvent<SqsEvent>) -> Result<(), Error> {
    let region_provider = Region::new("us-east-1");

    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);

    //TODO process every message despite lambda being configured to trigger each msg
    let _msg = event.payload.records.get(0).expect("Message not present");

    let channel = _msg.message_attributes["Channel"].string_value.as_ref().expect("Channel name not present");
    let environment_id = _msg.message_attributes["EnvironmentId"].string_value.as_ref().expect("Environment id not present");

    let url = format!("/telegram-ids/{environment_id}/token");
    let token = get_parameter(&client, _msg, &url).await.unwrap();

    let url = format!("/telegram-ids/{environment_id}/{channel}");
    let channel_id = get_parameter(&client, _msg, &url).await.unwrap();

    let mut map = HashMap::new();
    map.insert("chat_id", channel_id.as_str());
    let text = match &_msg.body {
        None => { "" }
        Some(txt) => { &txt }
    };
    map.insert("text", text);

    let client = reqwest::Client::new();
    let res = client.post(format!("https://api.telegram.org/bot{token}/sendMessage"))
        .json(&map)
        .send().await?;

    println!("FINISHHHHHHHHHH {:?}", res);

    Ok(())
}

async fn get_parameter(client: &Client, msg: &SqsMessage, param_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let env_attr = msg.message_attributes.get("EnvironmentId").expect("No message attribute for the record");
    let env = env_attr.string_value.as_ref().expect("No environment value found");
    let params = load_all_params(client, env).await;
    let opt = params.iter().find(|(x, _y)| x == param_name);

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
