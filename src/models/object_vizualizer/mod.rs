use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Value;
use crate::models::Executable;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectVizualizer {
    #[serde(skip_deserializing)]
    value: Value,
}

#[typetag::serde(name = "json-vizualizer")]
impl Executable for ObjectVizualizer {
    fn resolve_data(&mut self, value: Value) {
        self.value = value;
    }
}