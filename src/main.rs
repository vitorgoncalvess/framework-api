use rocket::{get, launch, post, routes};
use rocket::serde::json::{Json};
use serde::Serialize;
use framework_api::{make_cors, execute_pipeline};
use framework_api::models::{Component, User};

#[post("/", data = "<component>")]
async fn index(component: Json<Vec<Component>>) -> Json<Vec<Component>> {
    let components = component.into_inner();

    let components = execute_pipeline(components).await;

    Json(components)
}


#[get("/test_req")]
fn test_req() -> Json<User> {
    Json(User::new(String::from("Vitor"), String::from("vitor.gsilva@sptech.school")))
}

#[get("/echo")]
fn echo_channel(ws: ws::WebSocket) -> ws::Channel<'static> {
    use rocket::futures::{SinkExt, StreamExt};

    ws.channel(move |mut stream| Box::pin(async move {
        while let Some(message) = stream.next().await {
            let _ = stream.send(message?).await;
        }

        Ok(())
    }))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![index, test_req, echo_channel])
        .mount("/api", rocket_cors::catch_all_options_routes())
        .attach(make_cors())
        .manage(make_cors())
}