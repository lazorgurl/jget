use reqwest;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::io::stdin;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug)]
struct RequestIn {
    url: String,
    method: String,
    #[serde(default = "String::new")]
    body: String,
    #[serde(default = "HashMap::new")]
    headers: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ResponsetOut {
    status: u16,
    body: String,
    headers: HashMap<String, String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let spec: RequestIn = serde_json::from_reader(stdin())?;

    let mut header_map = HeaderMap::new();
    spec.headers.iter().for_each(|(k, v)| {
        header_map.append(
            HeaderName::from_str(k.as_str()).unwrap(),
            HeaderValue::from_str(v.as_str()).unwrap(),
        );
    });

    let client = reqwest::Client::new();

    let req = client
        .request(
            reqwest::Method::from_str(&spec.method)?,
            reqwest::Url::parse(&spec.url)?,
        )
        .body(spec.body)
        .headers(header_map)
        .build()?;

    let resp = client.execute(req).await?;
    let header_out = resp
        .headers()
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_str().unwrap().to_string()))
        .collect::<HashMap<String, String>>();

    let out: ResponsetOut = ResponsetOut {
        status: resp.status().as_u16(),
        headers: header_out,
        body: resp.text().await?,
    };

    println!("{}", serde_json::to_string(&out)?);
    Ok(())
}
