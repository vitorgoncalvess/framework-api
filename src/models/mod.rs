use std::collections::HashMap;
use chrono::{DateTime, Utc};
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Data {
    Request(Request),
    Value(Value),
    Null,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "req")]
    Request,
    #[serde(rename = "json-vizualizer")]
    JSONVisualizer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    pub url: String,
    method: String,
    headers: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
    pub id: String,
    pub data: Data,
    #[serde(rename = "type")]
    pub type_c: Type,
    pub links: Vec<String>,
    pub is_linked_with: Option<String>,
}

#[derive(Serialize)]
pub struct User {
    id: u32,
    name: String,
    email: String,
    address: Address,
    phones: Vec<String>,
    transactions: Vec<Transaction>,
}

impl User {
    pub fn new(name: String, email: String) -> User {
        User {
            id: 1,
            name,
            email,
            address: Address { number: 7, street: String::from("Rua Pio XI"), complement: None },
            phones: vec![String::from("11976050778"), String::from("11976675445")],
            transactions: vec![Transaction { value: 25.42, type_t: String::from("CREDIT"), created_at: DateTime::default() }, Transaction { value: 30.52, type_t: String::from("DEBIT"), created_at: DateTime::default() }],
        }
    }
}

#[derive(Serialize)]
pub struct Transaction {
    value: f32,
    type_t: String,
    created_at: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct Address {
    number: u32,
    street: String,
    complement: Option<String>,
}