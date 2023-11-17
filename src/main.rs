#[macro_use] extern crate rocket;
mod json;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(json::stage())
}