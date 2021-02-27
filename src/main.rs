#![allow(unused_imports, unused_variables)] //remove this in end and clean up

extern crate reqwest;
extern crate serde;
extern crate websocket;
extern crate openssl;
extern crate url;

use serde::{Serialize, Deserialize};
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

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
    async fn process<T>(&self, msg: Message, websocket: T) { //change frm generic to concrete type later
        let url = Url::parse(&self.base_uri).join(&msg.url);

        let resp: self.http_client.request(&msg.method, url)
            .headers(&msg.header)
            .json(&msg.body)
            .send()
            .await
            .json::<HashMap<String, String>>()
            .await;

        if resp.status() < 400 {
            let resp = ResponseObject::new(msg.id, self.token, resp.status(), resp.headers().into(), resp);
            //send the response to server with the socket stream
        }
        
        todo!();

    }
}

struct Tunnel<T> { //replace generic with concrete type for websocket
    ws_uri: String,
    http_uri: String,
    client: Option<Client>,
    ws_client: Option<T>,
}

impl Tunnel {
    fn new(ws_uri: String, http_uri: String) -> Self {
        Self {
            ws_uri,
            http_uri,
            client: None,
            ws_client: None,
        }
    }
}

impl Tunnel {
    async fn open_tunnel(&mut self) {
        self.client = Client::new(self.http_uri, token);
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
    header: Option<HashMap<String, String>>,
    body: Option<HashMap<String, String>>,
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
    body: HashMap<String, String>,
}

impl ResponseObject {
    fn new(request_id: String, token: String, status: u16, header: HashMap<String, String>, body: HashMap<String, String>) -> Self {
        Self {
            request_id,
            token,
            status,
            header,
            body,
        }
    }
}
