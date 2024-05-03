use std::collections::HashMap;
use async_trait::async_trait;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Value;
use crate::models::Executable;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectMapper {
    keys: Vec<String>,
    #[serde(rename = "selectedKeys")]
    selected_keys: Vec<String>,
    #[serde(skip_deserializing)]
    value: Option<Value>,
}

#[typetag::serde(name = "object-mapper")]
#[async_trait]
impl Executable for ObjectMapper {
    async fn get_data(&mut self) {
        if &self.keys.len() > &0 {
            let mut new_obj: HashMap<String, Value> = HashMap::new();
            if let Some(val) = &self.value {
                if let Value::Object(obj) = val {
                    for (key, value) in obj.into_iter() {
                        if self.selected_keys.contains(&key) {
                            new_obj.insert(key.clone(), value.clone());
                        }
                    }
                }
            }
            self.value = Some(Value::Object(new_obj
                .into_iter().map(|obj| obj).collect()))
        }
    }

    fn value(&self) -> &Option<Value> {
        &self.value
    }

    fn resolve_data(&mut self, _value: Value) {
        if let Value::Object(new_obj) = _value {
            if &self.keys.len() == &0 {
                let new_keys: Vec<String> = new_obj.iter().map(|(key, _)| key.clone()).collect();
                self.keys = new_keys.iter().cloned().collect();
                self.selected_keys = new_keys;
            }
            self.value = Some(Value::Object(new_obj))
        }
    }
}