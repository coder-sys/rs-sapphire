use std::{error::Error, str::FromStr};

use httpmock::MockServer;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub transcript: String,
    pub sent_tokens: Vec<String>,
}

pub async fn get_transcript(baseurl: String) -> String {
    let body = call_api(baseurl).await.unwrap();
    return body.transcript;
}
pub async fn get_sent_tokens(baseurl: String) -> Vec<String> {
    let body = call_api(baseurl).await.unwrap();
    return body.sent_tokens;
}
pub async fn call_api(baseurl: String) -> Result<Response, Box<dyn Error>> {
    let url = format!("{}/get_transcript/LwCRRUa8yTU", baseurl);
    let body = reqwest::get(url).await?.text().await?;
    let res = serialize_response(body).await?;
    Ok(res)
}

async fn serialize_response(body: String) -> Result<Response, serde_json::Error> {
    let response: Response = serde_json::from_str(&body)?;
    Ok(response)
}

#[cfg(test)]
#[tokio::test]
async fn test_call_api() {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(httpmock::Method::GET)
            .path("/get_transcript/8mAITcNt710");
        then.status(200)
            .header("Content-Type", "application/json")
            .body(r#"{"transcript":"Hello world"}"#);
    });
    let baseurl = server.base_url();
    let body = call_api(baseurl).await.unwrap();
}

#[tokio::test]
async fn test_get_transcript() {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(httpmock::Method::GET)
            .path("/get_transcript/8mAITcNt710");
        then.status(200)
            .header("Content-Type", "application/json")
            .body(r#"{"transcript":"Hello world"}"#);
    });
    let baseurl = server.base_url();
    let body = get_transcript(baseurl).await;
    assert_eq!(body, "Hello world");
}
