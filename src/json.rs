use std::collections::HashMap;
use rocket::serde::json::{Json, Value, json, serde_json};
use rocket::yansi::Paint;

#[post("/", format = "json", data = "<hm>")]
async fn translate(hm: Json<HashMap<String, Value>>) -> Json<HashMap<String, Value>> {
    let a = hm.into_inner();
    let str = serde_json::to_string(&a).unwrap();
    println!("{str}");
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

fn print_hash_map(m: &HashMap<String, Value>) {
    for (k, v) in m {
        println!("{}:{}", k, v);
    }
}