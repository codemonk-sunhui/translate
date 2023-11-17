#[macro_use] extern crate rocket;

use std::net::Ipv4Addr;
use rocket::config::{Config};

mod json;



#[launch]
fn rocket() -> _ {
    let mut config = Config::release_default();
    config.address = Ipv4Addr::new(0, 0, 0, 0).into();

    rocket::custom(config)
        .attach(json::stage())
}