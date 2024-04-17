pub mod models;

use std::collections::HashMap;
use rocket::futures::FutureExt;
use rocket::http::Method;
use rocket::serde::json::serde_json;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions};
use crate::models::{Component, Data, Request, Type};

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
    let client = reqwest::Client::new();
    
    let mut map: HashMap<String, Component> = components.into_iter()
        .map(|comp| (comp.id.clone(), comp))
        .collect();

    let mut response_cache = HashMap::new();

    for (_, value) in map.clone() {
        match value.type_c {
            Type::Request => {
                if let Data::Request(Request { url, .. }) = &value.data {
                    let response_future = response_cache
                        .entry(url.clone())
                        .or_insert_with(|| {
                            Box::pin(client.get(url).send().then(|result| async move {
                                result?.text().await
                            }))
                        });

                    let resp = response_future.await.unwrap();

                    for link in &value.links {
                        if let Some(entry) = map.get_mut(link) {
                            entry.data = Data::Value(serde_json::from_str(&resp).unwrap());
                        }
                    }
                }
            }
            _ => {}
        }
    }


    map.into_iter()
        .map(|comp| comp.1).collect()
}