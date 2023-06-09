use log::info;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use temporal_sdk::ActContext;

/// Make the http request
#[derive(Deserialize, Serialize, Debug, Clone)]
struct Response {
    args: HashMap<String, String>,
}

pub async fn make_http_request(
    _ctx: ActContext,
    _payload: Option<String>,
) -> Result<String, anyhow::Error> {
    let id = nanoid::nanoid!();
    info!("Starting http request activity: {}", id);
    let response = reqwest::get(format!("https://httpbin.org/get?answer={}", id))
        .await?
        .json::<Response>()
        .await?;

    info!("Response: {:?}", response);
    if let Some(answer) = response.args.get("answer") {
        return Ok(answer.to_string());
    }
    Err(anyhow::anyhow!("No answer found"))
}
