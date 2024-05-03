use rocket::serde::{Serialize, Deserialize};
use rocket::serde::json::Value;
use crate::models::Executable;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plus {
    value: f64,
}

#[typetag::serde(name = "plus")]
impl Executable for Plus {
    fn resolve_data(&mut self, _value: Value) {
        if let Value::Number(num) = _value {
            self.value = self.value + num.as_f64().unwrap();
        } else {
            println!("{:?}", _value);
            panic!("Valor deve ser numerico")
        }
    }
}