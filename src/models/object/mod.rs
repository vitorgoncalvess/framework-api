use rocket::serde::json::Value;
use rocket::serde::{Deserialize, Serialize};
use crate::models::Executable;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Object {
    value: Option<Value>,
}

#[typetag::serde(name = "object")]
impl Executable for Object {
    fn value(&self) -> &Option<Value> {
        &self.value
    }
}