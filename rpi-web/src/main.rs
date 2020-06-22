#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;

mod helper;

use rocket_contrib::serve::StaticFiles;
use rocket::response::NamedFile;
use rocket::response::Redirect;

use serde_cbor::to_vec;
use std::collections::HashMap;
use std::path::Path;

#[get("/pause")]
fn pause() -> Redirect {
    let socket = helper::connect();

    let mut data = HashMap::new();
    data.insert("type", "action");
    data.insert("value", "pause");
    let encoded = to_vec(&data);

    helper::send_message(socket, &encoded.unwrap());

    Redirect::to("/")
}

#[get("/unpause")]
fn unpause() {
    let socket = helper::connect();

    let mut data = HashMap::new();
    data.insert("type", "action");
    data.insert("value", "unpause");
    let encoded = to_vec(&data);

    helper::send_message(socket, &encoded.unwrap());
    
    Redirect::to("/")
}

#[get("/")]
fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/index.html")).ok()
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", StaticFiles::from("static")).mount("/", routes![index, pause, unpause])
}

fn main() {
    rocket().launch();
}