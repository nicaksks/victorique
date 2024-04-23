use serde_json::{json, Value};

use crate::logger::terminal::{Constructor, Logger};

pub struct Webhook;

impl Webhook {
    pub fn send(&self, url: Option<String>, content: String) {
        let log = Logger::default();
        match url {
            Some(uri) => {
                let client = reqwest::blocking::Client::new();
                let response = client.post(uri).json(&self.body(content)).send();

                match response {
                    Ok(r) => {
                        if r.status() == 400 {
                            panic!("{:?}", r.text());
                        }
                    }
                    Err(_) => {
                        log.error("Invalid URL");
                    }
                }
            }
            None => {}
        }
    }

    pub fn body(&self, content: String) -> Value {
        json!({ "content": content })
    }
}
