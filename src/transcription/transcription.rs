use std::error::Error;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]

pub struct Response {
    pub transcript: String,
}

pub async fn get_transcript()->String{
    let body = call_api().await.unwrap();
    return  body.transcript
}
pub async fn call_api() -> Result<Response, Box<dyn Error>> {
    let body = reqwest::get("http:localhost:8000/get_transcript/8mAITcNt710")
        .await?
        .text()
        .await?;
    let res = serialize_response(body).await?;
    Ok(res)
}


async fn serialize_response(body: String) -> Result<Response, serde_json::Error> {
    let response: Response = serde_json::from_str(&body)?;
    Ok(response)
}