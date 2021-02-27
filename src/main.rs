#![allow(unused_imports, unused_variables)] //remove this in end and clean up

extern crate reqwest;
extern crate serde;
extern crate openssl;
extern crate url;
extern crate base64;
extern crate tokio_tungstenite;

use serde::{Serialize, Deserialize};
use url::Url;
use std::collections::HashMap;
use tokio_tungstenite::WebSocketStream;
use tokio::io::{AsyncRead, AsyncWrite};
use reqwest::{Method, header::HeaderMap};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

struct Client {
    base_uri: String,
    token: String,
    http_client: reqwest::Client,
}

impl Client {
    fn new(base_uri: String, token: String) -> Self {
        Self {
            base_uri,
            token,
            http_client: reqwest::Client::new(),
        }
    }
}

impl Client {
    async fn process<S: AsyncRead + AsyncWrite + Unpin>(&self, msg: Message, websocket: WebSocketStream<S>) { //change frm generic to concrete type later
        let url = Url::parse(&self.base_uri).unwrap().join(&msg.url).unwrap(); 

        let resp = self.http_client.request(Method::from_bytes(&msg.method.as_bytes()).unwrap(), url) //do it more gracefully
            .json(&msg.body)
            .send()
            .await
            .unwrap();

        let status = resp.status().as_u16();
        let mut headers = HashMap::new();
        for (k, v) in resp.headers().iter() {
            headers.insert(k.clone().to_string(), v.clone().to_str().unwrap().to_owned());
        }
        let bytes = resp.bytes().await.unwrap(); //probably should do something more elelgent

        if status < 400 {
            let resp = ResponseObject::new(msg.id, self.token.clone(), status, headers, &bytes);
        } else {
            let resp = ResponseObject::new(msg.id, self.token.clone(), 500u16, HashMap::new(), &bytes);
        }
        //send response back with the socket stream
        todo!();

    }
}

struct Tunnel<S> { 
    ws_uri: String,
    http_uri: String,
    client: Option<Client>,
    ws_client: Option<WebSocketStream<S>>,
}

impl<S> Tunnel<S> {
    fn new(ws_uri: String, http_uri: String) -> Self {
        Self {
            ws_uri,
            http_uri,
            client: None,
            ws_client: None,
        }
    }
}

impl<S> Tunnel<S> {
    async fn open_tunnel(&mut self) {
        let token = "";
        self.client = Client::new(self.http_uri, token.into());
        loop {
            todo!();
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Message {
    id: String,
    method: String,
    url: String,
    body: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct ServerInfo {
    host: String,
    token: String,
}

#[derive(Debug)]
struct ResponseObject {
    request_id: String,
    token: String,
    status: u16,
    header: HashMap<String, String>,
    body: String,
}

impl ResponseObject {
    fn new<T: AsRef<[u8]>>(request_id: String, token: String, status: u16, header: HashMap<String, String>, body: T) -> Self {
        Self {
            request_id,
            token,
            status,
            header,
            body: base64::encode(body),
        }
    }
}
