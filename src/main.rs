#[macro_use]
extern crate rocket;

use std::net::Ipv4Addr;
use rocket::config::{Config};
use rocket::serde::json::{Json, Value, json};
use crate::baidu::{baidu_translate};
use crate::message::Message;

mod message;
mod baidu;


#[launch]
fn rocket() -> _ {
    let mut config = Config::debug_default();
    config.address = Ipv4Addr::new(0, 0, 0, 0).into();

    rocket::custom(config)
        .attach(stage())
}


#[post("/", format = "json", data = "<message>")]
async fn translate(message: Json<Message>) -> Json<Message> {
    let mut a = message.into_inner();
    let content = a.text.content.trim();
    println!("{content}");

    let pl = baidu_translate(&content.to_string()).await.unwrap();
    let mut tr = pl.trans_result;
    a.text.content = tr.remove(0).dst;
    Json(a)
}

#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("JSON", |rocket| async {
        rocket.mount("/api/v1/translate", routes![translate])
            .register("/api/v1/translate", catchers![not_found])
    })
}
