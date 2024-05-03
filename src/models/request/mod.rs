use std::collections::HashMap;
use async_trait::async_trait;
use futures::FutureExt;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::{serde_json, Value};
use crate::models::Executable;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    pub value: Option<Value>,
    pub url: String,
    method: String,
    headers: HashMap<String, Value>,
}

#[typetag::serde(name = "req")]
#[async_trait]
impl Executable for Request {
    async fn get_data(&mut self) {
        if self.value.is_none() {
            let url = self.url.clone();

            let client = reqwest::Client::new();

            let response_future = Box::pin(client.get(&url).send().then(|result| async move {
                result?.text().await
            }));

            let resp = response_future.await.unwrap();

            let result: Value = serde_json::from_str(&resp).unwrap();

            self.value = Some(result);
        }
    }

    fn value(&self) -> &Option<Value> {
        &self.value
    }
}