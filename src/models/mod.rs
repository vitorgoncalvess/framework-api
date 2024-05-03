mod request;
mod object_mapper;
mod plus;

mod object_vizualizer;
mod object;

use std::marker::Send;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::{Value};

#[typetag::serde(tag = "type")]
#[async_trait]
pub trait Executable: Send + Sync {
    async fn get_data(&mut self) {}

    fn value(&self) -> &Option<Value> {
        &None
    }

    fn resolve_data(&mut self, _value: Value) {}
}

#[derive(Serialize, Deserialize)]
pub struct Component {
    pub id: String,
    pub is_root: bool,
    pub data: Box<dyn Executable>,
    #[serde(rename = "childNodes")]
    pub child_nodes: Vec<String>,
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