pub mod models;

use std::collections::HashMap;
use std::sync::{Arc};
use async_recursion::async_recursion;
use rocket::http::Method;
use rocket::futures::future;
use rocket::serde::json::Value;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions};
use tokio::sync::Mutex;
use crate::models::{Component};

pub fn make_cors() -> Cors {
    let allowed_origins = AllowedOrigins::some_exact(&[
        "http://localhost:3000",
        "http://localhost:8000"
    ]);

    CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Post, Method::Get, Method::Options].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }
        .to_cors()
        .expect("error while building CORS")
}

pub async fn execute_pipeline(components: Vec<Component>) -> Vec<Component> {
    let mut root_ids = Vec::new();

    let map: HashMap<String, Arc<Mutex<Component>>> = components.into_iter()
        .map(|comp| {
            if comp.is_root {
                root_ids.push(comp.id.clone())
            }
            (comp.id.clone(), Arc::new(Mutex::new(comp)))
        })
        .collect();

    let t_map = Arc::new(map);

    let root_futures: Vec<_> = root_ids.iter().map(|id| {
        let t_map_clone = Arc::clone(&t_map);
        async move {
            resolve_data(id, t_map_clone).await;
        }
    }).collect();

    future::join_all(root_futures).await;

    let results = Arc::try_unwrap(t_map)
        .unwrap_or_else(|_| panic!("Arc ainda possui referências pendentes"))
        .into_iter()
        .map(|(_, comp_arc)| Arc::try_unwrap(comp_arc)
            .unwrap_or_else(|_| panic!("Mutex ainda possui referências pendentes"))
            .into_inner())
        .collect::<Vec<Component>>();

    results
}

#[async_recursion]
async fn resolve_data(comp_id: &String, map: Arc<HashMap<String, Arc<Mutex<Component>>>>) -> Option<Value> {
    let comp_lock = map.get(comp_id)?;

    let mut comp = comp_lock.lock().await;

    let mut results = Vec::new();

    for child_id in &comp.child_nodes {
        let child_value = resolve_data(child_id, Arc::clone(&map)).await;
        results.push(child_value);
    }

    for result in results {
        if let Some(value) = result {
            comp.data.resolve_data(value)
        }
    }

    comp.data.get_data().await;

    comp.data.value().clone()
}
