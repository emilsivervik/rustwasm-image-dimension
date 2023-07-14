use serde_json::Value;

pub async fn send_to_slack(slack_url: &String, payload: &Value) {
    let client = reqwest_wasm::Client::new();
    match client.post(slack_url).json(&payload).send().await {
        Ok(_) => (),
        Err(err) => panic!("{}", err),
    };
}
