use serde_json::{json, Value};

pub struct Webhook;

impl Webhook {
    pub fn send(&self, url: Option<String>, content: String) {
        match url {
            Some(uri) => {
                let client = reqwest::blocking::Client::new();

                println!("{:?}", uri);

                let response = client.post(uri).json(&self.body(content)).send().unwrap();

                if response.status() == 400 {
                    panic!("{:?}", response.text());
                }
            }
            None => {
                println!("{}", true);
            }
        }
    }

    pub fn body(&self, content: String) -> Value {
        json!({ "content": content })
    }
}
