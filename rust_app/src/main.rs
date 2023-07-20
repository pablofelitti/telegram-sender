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

async fn get_parameter(client: &Client, msg: &SqsMessage, param_name: &str) -> Option<String> {
    match msg.message_attributes.get("EnvironmentId") {
        None => { panic!("No message attribute for the record") }
        Some(e) => {
            let params = load_all_params(client, e.string_value.clone().unwrap().as_str()).await;
            println!("{} params have been found", params.len());
            let opt = params.iter().find(|(x, _y)| x == param_name);
            return if opt.is_some() {
                println!("Found element");
                let (_x, y) = opt?;
                Some(y.to_string())
            } else {
                println!("Did not find any element");
                None
            };
        }
    }
}

async fn load_all_params(client: &Client, env: &str) -> Vec<(String, String)> {
    let path = "/telegram-ids/".to_owned() + env;
    println!("PATH: {}", path);
    let resp = client.get_parameters_by_path().path(path).send().await.unwrap();

    let mut name;
    let mut value;

    let mut v: Vec<(String, String)> = Vec::new();

    for param in resp.parameters().unwrap().iter() {
        match param.name() {
            None => { panic!() }
            Some(_name) => name = _name
        }
        match param.value() {
            None => { panic!() }
            Some(_value) => value = _value
        }

        v.push((name.to_string(), value.to_string()));
    }
    return v;
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(function_handler)).await
}
